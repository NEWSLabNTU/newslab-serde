//! Data ser/deserialization library for [nalgebra] crate.


/// Serialize [Isometry3](nalgebra::Isometry3) as a (x, y, z) position
/// and a triple of (roll, pitch, yaw) angles.
///
/// ```rust
/// # use serde::{Deserialize, Serialize};
/// # use nalgebra::Isometry3;
/// # use newslab_serde_nalgebra::isometry3_as_euler_angles;
/// #[derive(Serialize, Deserialize)]
/// struct MyRotation {
///     #[serde(with = "isometry3_as_euler_angles")]
///     pose: Isometry3<f32>,
/// }
///
/// let json = r#"{
///     "pose": {
///         "translation": [-1, 2.0, 0.0],
///         "rotation": {
///             "roll": "0.0deg",
///             "pitch":  "1.57079632679rad",
///             "yaw": "360.0deg"
///         }
///     }
/// }"#;
///
/// let _: MyRotation = serde_json::from_str(json).unwrap();
/// ```
pub mod isometry3_as_euler_angles {
    use nalgebra::{
        coordinates::XYZ, Isometry3, RealField, SimdRealField, Translation3, UnitQuaternion,
    };
    use newslab_serde_measurements::EulerAngles;
    use num::NumCast;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    struct EulerIsometry3<T> {
        pub translation: [T; 3],
        pub rotation: EulerAngles,
    }

    pub fn serialize<S, T>(rot: &Isometry3<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: SimdRealField + RealField + Serialize + NumCast,
        T::Element: SimdRealField,
        S: Serializer,
    {
        let Isometry3 {
            translation,
            rotation,
        } = rot;
        let XYZ { x, y, z } = (**translation).clone();
        let (roll, pitch, yaw) = rotation.euler_angles();

        EulerIsometry3 {
            translation: [x, y, z],
            rotation: EulerAngles::from_radians(roll, pitch, yaw),
        }
        .serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Isometry3<T>, D::Error>
    where
        T: SimdRealField + RealField + Deserialize<'de> + NumCast,
        D: Deserializer<'de>,
    {
        let EulerIsometry3 {
            translation: [x, y, z],
            rotation: angles,
        } = EulerIsometry3::deserialize(deserializer)?;
        let [roll, pitch, yaw] = angles.to_radians::<T>();

        let translation = Translation3::new(x, y, z);
        let rotation = UnitQuaternion::from_euler_angles(roll, pitch, yaw);
        let isometry = Isometry3 {
            translation,
            rotation,
        };
        Ok(isometry)
    }
}

/// Serialize [UnitQuaternion](nalgebra::UnitQuaternion) as the triple
/// of (roll, pitch, yaw) angles.
///
/// ```rust
/// # use serde::{Deserialize, Serialize};
/// # use nalgebra::UnitQuaternion;
/// # use newslab_serde_nalgebra::unit_quaternion_as_euler_angles;
/// #[derive(Serialize, Deserialize)]
/// struct MyRotation {
///     #[serde(with = "unit_quaternion_as_euler_angles")]
///     rotation: UnitQuaternion<f32>,
/// }
///
/// let json = r#"{
///     "rotation": {
///         "roll": "0.0deg",
///         "pitch":  "1.57079632679rad",
///         "yaw": "360.0deg"
///     }
/// }"#;
///
/// let _: MyRotation = serde_json::from_str(json).unwrap();
/// ```
pub mod unit_quaternion_as_euler_angles {
    use nalgebra::{RealField, SimdRealField, UnitQuaternion};
    use newslab_serde_measurements::EulerAngles;
    use num::NumCast;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, T>(rot: &UnitQuaternion<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: SimdRealField + RealField + Serialize + NumCast,
        S: Serializer,
    {
        let (roll, pitch, yaw) = rot.euler_angles();
        EulerAngles::from_radians(roll, pitch, yaw).serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<UnitQuaternion<T>, D::Error>
    where
        T: SimdRealField + RealField + Deserialize<'de> + NumCast,
        D: Deserializer<'de>,
    {
        let angles = EulerAngles::deserialize(deserializer)?;
        let [roll, pitch, yaw] = angles.to_radians();
        let rot = UnitQuaternion::from_euler_angles(roll, pitch, yaw);
        Ok(rot)
    }
}
