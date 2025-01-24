use serde::{Deserialize, Serialize};
use strum::{AsRefStr, VariantArray};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, VariantArray, AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum SamplingMethod {
    #[default]
    Euler,
    EulerA,
    Heun,
    Dpm2,
    // Pp means ++
    #[strum(serialize = "dpm++2s_a")]
    DpmPp2sA,
    #[strum(serialize = "dpm++2m")]
    DpmPp2m,
    #[strum(serialize = "dpm++2mv2")]
    DpmPp2mv2,
    Ipndm,
    IpndmV,
    Lcm,
}
#[cfg(test)]
mod test {
    use super::SamplingMethod;
    use strum::VariantArray;
    #[test]
    fn as_str() {
        let expect = [
            "euler",
            "euler_a",
            "heun",
            "dpm2",
            "dpm++2s_a",
            "dpm++2m",
            "dpm++2mv2",
            "ipndm",
            "ipndm_v",
            "lcm",
        ]
        .into_iter();
        let actual = SamplingMethod::VARIANTS.iter().map(|v| v.as_ref());
        for (expect, actual) in expect.zip(actual) {
            assert_eq!(expect, actual)
        }
    }
}
