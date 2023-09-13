//! Data ser/deserialization library for well-known concepts.

pub use serde_bound::{optional_range, range};
mod serde_bound {
    use serde::{Deserialize, Serialize};
    use std::ops::{Bound, Bound::*};

    #[derive(Serialize, Deserialize)]
    struct SerializedBound<T> {
        #[serde(rename = ">")]
        pub min: Option<T>,
        #[serde(rename = ">=")]
        pub imin: Option<T>,
        #[serde(rename = "<")]
        pub max: Option<T>,
        #[serde(rename = "<=")]
        pub imax: Option<T>,
    }

    fn unpack<T>(bound: &Bound<T>) -> (Option<&T>, Option<&T>) {
        match bound {
            Unbounded => (None, None),
            Included(val) => (None, Some(val)),
            Excluded(val) => (Some(val), None),
        }
    }

    fn pack<T>(bound: Option<T>, ibound: Option<T>) -> Option<Bound<T>> {
        let output = match (bound, ibound) {
            (None, None) => Unbounded,
            (Some(val), None) => Excluded(val),
            (None, Some(val)) => Included(val),
            (Some(_), Some(_)) => return None,
        };
        Some(output)
    }

    impl<'a, T> SerializedBound<&'a T> {
        pub fn from_bound((lower, upper): &'a (Bound<T>, Bound<T>)) -> Self {
            let (min, imin) = unpack(lower);
            let (max, imax) = unpack(upper);

            SerializedBound {
                min,
                imin,
                max,
                imax,
            }
        }
    }

    impl<T> SerializedBound<T> {
        pub fn into_bound(self) -> Result<(Bound<T>, Bound<T>), &'static str> {
            let SerializedBound {
                min,
                imin,
                max,
                imax,
            } = self;

            let lower = pack(min, imin).ok_or("min and imin must not be both specified")?;
            let upper = pack(max, imax).ok_or("max and imax must not be both specified")?;

            Ok((lower, upper))
        }
    }

    /// Serialize or deserialize arbitrary ranges.
    ///
    /// ```rust
    /// # use std::ops::Bound;
    /// # use serde::{Serialize, Deserialize};
    /// # use newslab_serde_common::range;
    /// #[derive(Serialize, Deserialize)]
    /// struct MyRange {
    ///     #[serde(with = "range")]
    ///     range: (Bound<f32>, Bound<f32>),
    /// }
    ///
    /// let json = r#"
    ///     {
    ///         "range": {
    ///             ">": -10.0,
    ///             "<=": 5.0
    ///         }
    ///     }
    /// "#;
    ///
    /// let my_range: MyRange = serde_json::from_str(json).unwrap();
    /// assert_eq!(
    ///     my_range.range,
    ///     (Bound::Excluded(-10.0), Bound::Included(5.0))
    /// );
    /// ```
    pub mod range {
        use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
        use std::ops::Bound;

        use super::SerializedBound;

        pub fn serialize<S, T>(
            bound: &(Bound<T>, Bound<T>),
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            T: Serialize,
            S: Serializer,
        {
            SerializedBound::from_bound(bound).serialize(serializer)
        }

        pub fn deserialize<'de, D, T>(deserializer: D) -> Result<(Bound<T>, Bound<T>), D::Error>
        where
            T: Deserialize<'de>,
            D: Deserializer<'de>,
        {
            let raw = SerializedBound::<T>::deserialize(deserializer)?;
            raw.into_bound().map_err(D::Error::custom)
        }
    }

    /// Optionally serialize or deserialize arbitrary ranges.
    ///
    /// ```rust
    /// # use std::ops::Bound;
    /// # use serde::{Serialize, Deserialize};
    /// # use newslab_serde_common::optional_range;
    /// #[derive(Serialize, Deserialize)]
    /// struct MyRange {
    ///     #[serde(with = "optional_range")]
    ///     range: Option<(Bound<f32>, Bound<f32>)>,
    /// }
    ///
    /// let json = r#"
    ///     {
    ///         "range": {
    ///             ">": -10.0,
    ///             "<=": 5.0
    ///         }
    ///     }
    /// "#;
    ///
    /// let my_range: MyRange = serde_json::from_str(json).unwrap();
    /// assert_eq!(
    ///     my_range.range,
    ///     Some((Bound::Excluded(-10.0), Bound::Included(5.0)))
    /// );
    /// ```
    pub mod optional_range {
        use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
        use std::ops::Bound;

        use super::SerializedBound;

        type Range<T> = (Bound<T>, Bound<T>);

        pub fn serialize<S, T>(
            bound: &Option<(Bound<T>, Bound<T>)>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            T: Serialize,
            S: Serializer,
        {
            bound
                .as_ref()
                .map(|bound| SerializedBound::from_bound(bound))
                .serialize(serializer)
        }

        pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<Range<T>>, D::Error>
        where
            T: Deserialize<'de>,
            D: Deserializer<'de>,
        {
            let bound = Option::<SerializedBound<T>>::deserialize(deserializer)?
                .map(|raw| raw.into_bound())
                .transpose()
                .map_err(D::Error::custom)?;
            Ok(bound)
        }
    }
}

/// Serialize or deserialize a non-empty string.
///
/// ```rust
/// # use std::ops::Bound;
/// # use serde::{Serialize, Deserialize};
/// # use newslab_serde_common::non_empty_string;
/// #[derive(Serialize, Deserialize)]
/// struct MyString {
///     #[serde(with = "non_empty_string")]
///     text: String,
/// }
///
/// // Case 1
/// let json = r#"
///     {
///         "text": "hello"
///     }
/// "#;
///
/// let my_string: MyString = serde_json::from_str(json).unwrap();
/// assert_eq!(my_string.text, "hello");
///
/// // Case 2
/// let json = r#"
///     {
///         "text": ""
///     }
/// "#;
///
/// let result: Result<MyString, _> = serde_json::from_str(json);
/// assert!(result.is_err());
/// ```
pub mod non_empty_string {
    use serde::{
        de::Error as _, ser::Error as _, Deserialize, Deserializer, Serialize, Serializer,
    };

    pub fn serialize<S>(text: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if text.is_empty() {
            return Err(S::Error::custom("string must not be empty"));
        }
        text.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        if text.is_empty() {
            return Err(D::Error::custom("string must not be empty"));
        }
        Ok(text)
    }
}
