use super::AddArgs;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, VariantArray};
#[derive(
    Serialize, Deserialize, Clone, Debug, Copy, Default, AsRefStr, VariantArray, PartialEq,
)]
#[strum(serialize_all = "snake_case")]
pub enum WeightType {
    #[default]
    #[strum(serialize = "")]
    Default,
    F32,
    F16,
    Q4_0,
    Q4_1,
    Q5_0,
    Q5_1,
    Q8_0,
    #[strum(serialize = "q2_K")]
    Q2k,
    #[strum(serialize = "q3_K")]
    Q3K,
    #[strum(serialize = "q4_K")]
    Q4K,
    #[strum(serialize = "q5_K")]
    Q5K,
    #[strum(serialize = "q6_K")]
    Q6K,
    Iq2Xxs,
    Iq2Xs,
    Iq3Xxs,
    Iq1S,
    Iq4Nl,
    Iq3S,
    Iq2S,
    Iq4Xs,
    Iq1M,
    Bf16,
    TQ1_0,
    TQ2_0,
}
impl AddArgs for WeightType {
    fn add_args(&self, command: &mut std::process::Command) {
        match self {
            Self::Default => (),
            _ => {
                command.args(["--type", self.as_ref()]);
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::WeightType;
    use strum::VariantArray;
    #[test]
    fn as_str() {
        let expect = [
            "", "f32", "f16", "q4_0", "q4_1", "q5_0", "q5_1", "q8_0", "q2_K", "q3_K", "q4_K",
            "q5_K", "q6_K", "iq2_xxs", "iq2_xs", "iq3_xxs", "iq1_s", "iq4_nl", "iq3_s", "iq2_s",
            "iq4_xs", "iq1_m", "bf16", "tq1_0", "tq2_0",
        ]
        .into_iter();
        let actual = WeightType::VARIANTS.iter().map(|v| v.as_ref());
        for (expect, actual) in expect.zip(actual) {
            assert_eq!(expect, actual)
        }
    }
}
