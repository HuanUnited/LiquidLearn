use super::parameters::FsrsParameters;

const MEAN_RATING: f64 = 5.5; // Midpoint of 1-10 scale
const MAX_INTERVAL_DAYS: i32 = 180; // MVP limit

/// Result of processing a review [file:13]
#[derive(Debug, Clone)]
pub struct ReviewResult {
    pub new_interval: i32,
    pub new_difficulty: f64,
    pub new_stability: f64,
    pub new_state: String,
    pub is_lapse: bool,
}

/// Schedule next review interval [file:13]
pub fn schedule_next_review(
    state: &str,
    stability: f64,
    rating: i32,
    params: &FsrsParameters,
) -> i32 {
    let new_interval = match state {
        "new" => 1,

        "learning" => {
            if rating >= 5 {
                3 // Graduate to review
            } else {
                1 // Stay in learning
            }
        }

        "review" => {
            // Core FSRS formula [file:13]
            let interval_modifier = match rating {
                1..=2 => 0.0,  // Lapse
                3..=4 => 0.5,  // Hard
                5..=6 => 1.0,  // Good
                7..=8 => 1.5,  // Easy
                9..=10 => 2.0, // Very easy
                _ => 1.0,
            };

            if interval_modifier == 0.0 {
                1 // Lapse → restart
            } else {
                let decay_rate = params.w_11;
                let retention = params.desired_retention;

                // FSRS formula: interval = S * ln(R) / ln(decay_rate) * modifier
                let calculated = stability * retention.ln() / decay_rate.ln() * interval_modifier;

                calculated.round() as i32
            }
        }

        "relearning" => {
            if rating >= 6 {
                3 // Recover to review
            } else {
                1 // Stay in relearning
            }
        }

        _ => 1,
    };

    // Bound interval [file:13]
    new_interval.max(1).min(MAX_INTERVAL_DAYS)
}

/// Update card difficulty [file:13]
pub fn update_difficulty(current_difficulty: f64, rating: i32, params: &FsrsParameters) -> f64 {
    let adjustment = params.w_17 * (MEAN_RATING - rating as f64);
    let new_difficulty = current_difficulty + adjustment;

    // Clamp to 1.0-10.0 [file:13]
    new_difficulty.max(1.0).min(10.0)
}

/// Update card stability [file:13]
pub fn update_stability(
    state: &str,
    current_stability: f64,
    rating: i32,
    new_interval: i32,
    params: &FsrsParameters,
) -> f64 {
    let new_stability = match state {
        "new" => {
            if rating >= 5 {
                params.w_1 // Initialize stability
            } else {
                0.1 // Reset
            }
        }

        "learning" => {
            if rating >= 5 {
                current_stability + params.w_2 * (new_interval as f64) / 10.0
            } else {
                0.1 // Reset
            }
        }

        "review" => {
            if rating >= 3 {
                // Success: increase stability [file:13]
                let factor = match rating {
                    3..=4 => params.w_3 * 0.6,  // Hard
                    5..=6 => params.w_3 * 1.0,  // Good
                    7..=8 => params.w_3 * 1.2,  // Easy
                    9..=10 => params.w_3 * 1.5, // Very easy
                    _ => params.w_3,
                };

                current_stability + factor * (new_interval as f64)
            } else {
                // Lapse: drop stability [file:13]
                current_stability * 0.36 // Drop to ~1/3
            }
        }

        "relearning" => {
            if rating >= 5 {
                current_stability * params.w_4
            } else {
                0.1 // Reset
            }
        }

        _ => 0.1,
    };

    new_stability.max(0.1)
}

/// Update card state (state machine) [file:13]
pub fn update_state(current_state: &str, rating: i32) -> (String, bool) {
    let (new_state, is_lapse) = match (current_state, rating) {
        // New → always go to learning
        ("new", _) => ("learning".to_string(), false),

        // Learning transitions
        ("learning", 1..=4) => ("learning".to_string(), false),
        ("learning", 5..=10) => ("review".to_string(), false),

        // Review transitions
        ("review", 1..=2) => ("relearning".to_string(), true), // Lapse!
        ("review", 3..=10) => ("review".to_string(), false),

        // Relearning transitions
        ("relearning", 1..=4) => ("relearning".to_string(), false),
        ("relearning", 5..=10) => ("review".to_string(), false),

        // Default
        _ => ("learning".to_string(), false),
    };

    (new_state, is_lapse)
}

/// Process a review (main entry point) [file:13]
pub fn process_review(
    state: &str,
    stability: f64,
    difficulty: f64,
    rating: i32,
    params: &FsrsParameters,
) -> ReviewResult {
    // Validate rating
    if !(1..=10).contains(&rating) {
        panic!("Rating must be between 1 and 10");
    }

    // 1. Update state
    let (new_state, is_lapse) = update_state(state, rating);

    // 2. Schedule next review
    let new_interval = schedule_next_review(state, stability, rating, params);

    // 3. Update difficulty
    let new_difficulty = update_difficulty(difficulty, rating, params);

    // 4. Update stability
    let new_stability = update_stability(state, stability, rating, new_interval, params);

    ReviewResult {
        new_interval,
        new_difficulty,
        new_stability,
        new_state,
        is_lapse,
    }
}
