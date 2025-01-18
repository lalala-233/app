use serde::{Deserialize, Serialize};
use strum::{AsRefStr, VariantArray};
#[derive(Serialize, Deserialize, Clone, Debug, Default, AsRefStr)]
pub enum WeightType {
    #[default]
    None,
    F32,
    F16,
    Q4_0,
    Q4_1,
    Q5_0,
    Q5_1,
    Q8_0,
    Q2K,
    Q3K,
    Q4Ks,
}
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, AsRefStr, PartialEq, VariantArray,
)]
#[strum(serialize_all = "lowercase")]
pub enum Scheduler {
    #[default]
    Discrete,
    Karras,
    Exponential,
    Ays,
    Gits,
}
