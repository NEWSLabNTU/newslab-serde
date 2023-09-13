use anyhow::anyhow;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
    num::NonZeroU64,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fraction {
    pub is_negative: bool,
    pub num: u64,
    pub deno: NonZeroU64,
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let reverse = match (self.is_negative, other.is_negative) {
            (true, true) => true,
            (true, false) => return Some(Ordering::Less),
            (false, true) => return Some(Ordering::Greater),
            (false, false) => false,
        };

        let lhs = self.num * other.deno.get();
        let rhs = other.num * self.deno.get();
        let ord = lhs.partial_cmp(&rhs)?;

        Some(if reverse { ord.reverse() } else { ord })
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        let reverse = match (self.is_negative, other.is_negative) {
            (true, true) => true,
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            (false, false) => false,
        };
        let lhs = self.num * other.deno.get();
        let rhs = other.num * self.deno.get();
        let ord = lhs.cmp(&rhs);

        if reverse {
            ord.reverse()
        } else {
            ord
        }
    }
}

impl Fraction {
    pub fn reduce(&self) -> Self {
        let gcd = gcd::binary_u64(self.num, self.deno.get());
        Self {
            num: self.num / gcd,
            deno: NonZeroU64::new(self.deno.get() / gcd).unwrap(),
            is_negative: self.is_negative,
        }
    }

    pub fn to_f64(&self) -> f64 {
        self.num as f64 / self.deno.get() as f64
    }

    pub fn recip(&self) -> Option<Self> {
        Some(Self {
            num: self.deno.get(),
            deno: NonZeroU64::new(self.num)?,
            is_negative: self.is_negative,
        })
    }
}

impl FromStr for Fraction {
    type Err = anyhow::Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (is_negative, text) = match text.strip_prefix('-') {
            Some(suffix) => (false, suffix),
            None => (true, text),
        };
        let mut tokens = text.split('/');

        let err = || {
            anyhow!(
                "Invalid fraction string '{}'. It must be in 'num/deno' format.",
                text
            )
        };

        let num = tokens.next().ok_or_else(err)?.parse().map_err(|_| err())?;
        let deno = tokens.next().ok_or_else(err)?.parse().map_err(|_| err())?;

        if tokens.next().is_some() {
            return Err(err());
        }

        Ok(Self {
            num,
            deno,
            is_negative,
        })
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            is_negative,
            num,
            deno,
        } = *self;
        let sign = if is_negative { "-" } else { "" };
        write!(f, "{sign}{num}/{deno}")
    }
}

impl Serialize for Fraction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("{}", self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Fraction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        text.parse().map_err(D::Error::custom)
    }
}
