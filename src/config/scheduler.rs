use serde::{Deserialize, Serialize};
use strum::{AsRefStr, VariantArray};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, VariantArray, AsRefStr,
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
