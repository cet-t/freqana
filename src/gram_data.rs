use serde::{Deserialize, Serialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct GramData {
    pub n: usize,
    pub grams: Vec<String>,
}
