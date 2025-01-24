use serde::{Deserialize, Serialize};
use strum::{AsRefStr, VariantArray};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, VariantArray, AsRefStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum Schedule {
    #[default]
    Discrete,
    Karras,
    Exponential,
    Ays,
    Gits,
}

#[cfg(test)]
mod test {
    use super::Schedule;
    use strum::VariantArray;
    #[test]
    fn as_str() {
        let expect = ["discrete", "karras", "exponential", "ays", "gits"].into_iter();
        let actual = Schedule::VARIANTS.iter().map(|v| v.as_ref());
        for (expect, actual) in expect.zip(actual) {
            assert_eq!(expect, actual)
        }
    }
}
