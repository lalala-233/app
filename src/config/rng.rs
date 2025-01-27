use super::AddArgs;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, VariantArray};
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, VariantArray, AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum RngType {
    #[default]
    Cuda,
    StdDefault,
}
impl AddArgs for RngType {
    fn add_args(&self, command: &mut std::process::Command) {
        command.args(["--rng", self.as_ref()]);
    }
}
#[cfg(test)]
mod test {
    use super::RngType;
    use strum::VariantArray;
    #[test]
    fn as_str() {
        let expect = ["cuda", "std_default"].into_iter();
        let actual = RngType::VARIANTS.iter().map(|v| v.as_ref());
        for (expect, actual) in expect.zip(actual) {
            assert_eq!(expect, actual)
        }
    }
}
