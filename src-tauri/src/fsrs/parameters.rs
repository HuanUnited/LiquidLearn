use serde::{Deserialize, Serialize};

/// FSRS v3 parameters (w_1 through w_19) [file:13]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsrsParameters {
    pub w_1: f64,  // Initial stability
    pub w_2: f64,  // Learning stability
    pub w_3: f64,  // Review stability
    pub w_4: f64,  // Relearning stability
    pub w_5: f64,  // Ease factor for new
    pub w_6: f64,  // Ease factor adjustment
    pub w_7: f64,  // Lapse impact
    pub w_8: f64,  // Difficulty factor
    pub w_9: f64,  // Difficulty update
    pub w_10: f64, // Difficulty min
    pub w_11: f64, // Decay rate (CRITICAL)
    pub w_12: f64, // Decay power
    pub w_13: f64, // Weight threshold
    pub w_14: f64, // Stability jitter
    pub w_15: f64, // Interval ceiling
    pub w_16: f64, // Threshold for relearning
    pub w_17: f64, // Difficulty mod (CRITICAL)
    pub w_18: f64, // Difficulty modifier
    pub w_19: f64, // Lapse multiplier
    pub desired_retention: f64,
}

impl Default for FsrsParameters {
    /// Default FSRS parameters (validated by community) [file:13]
    fn default() -> Self {
        Self {
            w_1: 0.40,
            w_2: 1.86,
            w_3: 4.93,
            w_4: 0.94,
            w_5: 0.86,
            w_6: 0.01,
            w_7: 1.49,
            w_8: 0.04,
            w_9: 0.36,
            w_10: 0.86,
            w_11: 0.20,
            w_12: 2.50,
            w_13: 0.14,
            w_14: 0.94,
            w_15: 0.16,
            w_16: 0.10,
            w_17: 0.29,
            w_18: 0.34,
            w_19: 3.73,
            desired_retention: 0.95,
        }
    }
}
