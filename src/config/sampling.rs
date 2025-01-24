use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SamplingConfig {
    pub seed: i64,
    // 不会为 0
    pub cfg_scale: f32,
    pub slg_scale: f32,
    // 大于 0
    pub steps: u32,
    pub width: u32,
    pub height: u32,
}
impl Default for SamplingConfig {
    fn default() -> Self {
        Self {
            steps: 20,
            cfg_scale: 7.0,
            slg_scale: 0.0,
            width: 512,
            height: 512,
            seed: -1,
        }
    }
}
