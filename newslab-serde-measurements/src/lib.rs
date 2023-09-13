//! Data ser/deserialization library for [measurements](measurements) crate.

use measurements::Angle;
use num::{Float, NumCast};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EulerAngles {
    #[serde(with = "angle")]
    pub roll: Angle,
    #[serde(with = "angle")]
    pub pitch: Angle,
    #[serde(with = "angle")]
    pub yaw: Angle,
}

impl EulerAngles {
    pub fn from_degrees<T>(roll: T, pitch: T, yaw: T) -> Self
    where
        T: NumCast,
    {
        let roll = num::cast(roll).unwrap();
        let pitch = num::cast(pitch).unwrap();
        let yaw = num::cast(yaw).unwrap();

        EulerAngles {
            roll: Angle::from_degrees(roll),
            pitch: Angle::from_degrees(pitch),
            yaw: Angle::from_degrees(yaw),
        }
    }

    pub fn from_radians<T>(roll: T, pitch: T, yaw: T) -> Self
    where
        T: NumCast,
    {
        let roll = num::cast(roll).unwrap();
        let pitch = num::cast(pitch).unwrap();
        let yaw = num::cast(yaw).unwrap();

        EulerAngles {
            roll: Angle::from_radians(roll),
            pitch: Angle::from_radians(pitch),
            yaw: Angle::from_radians(yaw),
        }
    }

    pub fn to_degrees<T>(&self) -> [T; 3]
    where
        T: NumCast,
    {
        let Self { roll, pitch, yaw } = *self;

        let roll = num::cast(roll.as_degrees()).unwrap();
        let pitch = num::cast(pitch.as_degrees()).unwrap();
        let yaw = num::cast(yaw.as_degrees()).unwrap();
        [roll, pitch, yaw]
    }

    pub fn to_radians<T>(&self) -> [T; 3]
    where
        T: NumCast,
    {
        let Self { roll, pitch, yaw } = *self;

        let roll = num::cast(roll.as_radians()).unwrap();
        let pitch = num::cast(pitch.as_radians()).unwrap();
        let yaw = num::cast(yaw.as_radians()).unwrap();
        [roll, pitch, yaw]
    }
}

/// Serialization helper to en/decode an angle value with units.
///
/// ```rust
/// # use newslab_serde_measurements::angle;
/// # use serde::{Serialize, Deserialize};
/// # use measurements::Angle;
/// #[derive(Serialize, Deserialize)]
/// struct MyAngle {
///     #[serde(with = "angle")]
///     angle1: Angle,
///     #[serde(with = "angle")]
///     angle2: Angle,
/// }
///
/// let json = r#"{ "angle1": "3.0deg", "angle2": "-1.0rad" }"#;
/// let MyAngle { angle1, angle2 } = serde_json::from_str(json).unwrap();
///
/// assert_eq!(angle1.as_degrees(), 3.0);
/// assert_eq!(angle2.as_radians(), -1.0);
/// ```
pub mod angle {
    use measurements::Angle;
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(angle: &Angle, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let degs = angle.as_degrees();

        if (1e-3..=1e3).contains(&degs) {
            format!("{degs}deg")
        } else {
            format!("{degs:e}deg")
        }
        .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Angle, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;

        let parse_number = |text: &str| -> Result<f64, D::Error> {
            let value: f64 = text
                .parse()
                .map_err(|_| D::Error::custom(format!("{} is not a valid number", text)))?;
            Ok(value)
        };

        let angle = if let Some(prefix) = text.strip_suffix("deg") {
            let value = parse_number(prefix)?;
            Angle::from_degrees(value)
        } else if let Some(prefix) = text.strip_suffix("rad") {
            let value = parse_number(prefix)?;
            Angle::from_radians(value)
        } else {
            return Err(D::Error::custom(
                "Unable to parse '{}' as a angle measure.
It must be a floating number plus a length unit, for example, '10.0deg' or '10.0rad'.",
            ));
        };

        Ok(angle)
    }
}

/// Serialization helper to en/decode an length value with units.
///
/// ```rust
/// # use newslab_serde_measurements::length;
/// # use serde::{Serialize, Deserialize};
/// # use measurements::Length;
/// #[derive(Serialize, Deserialize)]
/// struct MyLength {
///     #[serde(with = "length")]
///     len1: Length,
///     #[serde(with = "length")]
///     len2: Length,
/// }
///
/// let json = r#"{ "len1": "2m", "len2": "-0.4mm" }"#;
/// let MyLength { len1, len2 } = serde_json::from_str(json).unwrap();
///
/// assert_eq!(len1.as_meters(), 2.0);
/// assert_eq!(len2.as_millimeters(), -0.4);
/// ```
pub mod length {
    use crate::ScientificNotation;
    use measurements::Length;
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(len: &Length, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ScientificNotation {
            significand,
            exponent,
        } = ScientificNotation::from_float(len.as_meters());

        let significand: f64 = num::cast(significand).unwrap();

        let text = if exponent >= 6 {
            let significand = significand * 10f64.powi(exponent - 3);
            format!("{:e}km", significand)
        } else if exponent >= 3 {
            let significand = significand * 10f64.powi(exponent - 3);
            format!("{}km", significand)
        } else if exponent >= 0 {
            format!("{}m", significand)
        } else if exponent >= -3 {
            let significand = significand * 10f64.powi(exponent + 3);
            format!("{}mm", significand)
        } else if exponent >= -6 {
            let significand = significand * 10f64.powi(exponent + 6);
            format!("{}µm", significand)
        } else if exponent >= -9 {
            let significand = significand * 10f64.powi(exponent + 9);
            format!("{}nm", significand)
        } else {
            let significand = significand * 10f64.powi(exponent + 9);
            format!("{:e}nm", significand)
        };

        text.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Length, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;

        let parse_number = |text: &str| -> Result<f64, D::Error> {
            let value: f64 = text
                .parse()
                .map_err(|_| D::Error::custom(format!("{} is not a valid number", text)))?;
            Ok(value)
        };

        let length = if let Some(prefix) = text.strip_suffix("nm") {
            let value = parse_number(prefix)?;
            Length::from_nanometers(value)
        } else if let Some(prefix) = text.strip_suffix("um") {
            let value = parse_number(prefix)?;
            Length::from_micrometers(value)
        } else if let Some(prefix) = text.strip_suffix("µm") {
            let value = parse_number(prefix)?;
            Length::from_micrometers(value)
        } else if let Some(prefix) = text.strip_suffix("um") {
            let value = parse_number(prefix)?;
            Length::from_micrometers(value)
        } else if let Some(prefix) = text.strip_suffix("mm") {
            let value = parse_number(prefix)?;
            Length::from_millimeters(value)
        } else if let Some(prefix) = text.strip_suffix("cm") {
            let value = parse_number(prefix)?;
            Length::from_centimeters(value)
        } else if let Some(prefix) = text.strip_suffix("dm") {
            let value = parse_number(prefix)?;
            Length::from_decimeters(value)
        } else if let Some(prefix) = text.strip_suffix("hm") {
            let value = parse_number(prefix)?;
            Length::from_hectometers(value)
        } else if let Some(prefix) = text.strip_suffix("km") {
            let value = parse_number(prefix)?;
            Length::from_kilometers(value)
        } else if let Some(prefix) = text.strip_suffix('m') {
            let value = parse_number(prefix)?;
            Length::from_meters(value)
        } else if let Some(prefix) = text.strip_suffix("in") {
            let value = parse_number(prefix)?;
            Length::from_inches(value)
        } else if let Some(prefix) = text.strip_suffix("yd") {
            let value = parse_number(prefix)?;
            Length::from_yards(value)
        } else if let Some(prefix) = text.strip_suffix("mi") {
            let value = parse_number(prefix)?;
            Length::from_miles(value)
        } else if let Some(prefix) = text.strip_suffix("furlong") {
            let value = parse_number(prefix)?;
            Length::from_furlongs(value)
        } else if let Some(prefix) = text.strip_suffix("ft") {
            let value = parse_number(prefix)?;
            Length::from_feet(value)
        } else {
            return Err(D::Error::custom(
                "Unable to parse '{}' as a length measure.
It must be a floating number plus a length unit, for example, '10.0m'.",
            ));
        };

        Ok(length)
    }
}

struct ScientificNotation<T> {
    pub significand: T,
    pub exponent: i32,
}

impl<T> ScientificNotation<T>
where
    T: Float,
{
    // pub fn to_float(&self) -> T
    // where
    //     T: NumCast,
    // {
    //     let ten = num::cast::<_, T>(10f32).unwrap();
    //     self.significand * ten.powi(self.exponent)
    // }

    pub fn from_float(value: T) -> ScientificNotation<T>
    where
        T: NumCast,
    {
        if !value.is_finite() || value == T::zero() {
            return ScientificNotation {
                significand: value,
                exponent: 0,
            };
        }

        let ten = num::cast::<_, T>(10f32).unwrap();
        let exponent: i32 = num::cast(value.abs().log10().floor()).unwrap();
        let significand = value / ten.powi(exponent);

        ScientificNotation {
            significand,
            exponent,
        }
    }
}
