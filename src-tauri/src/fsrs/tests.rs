#[cfg(test)]
mod fsrs_sanity_checks {
    use crate::fsrs::{process_review, FsrsParameters};

    fn params() -> FsrsParameters {
        FsrsParameters::default()
    }

    #[test]
    fn test_1_new_card_scheduling() {
        let result = process_review("new", 0.0, 5.0, 5, &params());
        assert_eq!(result.new_interval, 1);
        assert_eq!(result.new_state, "learning");
        assert!((result.new_difficulty - 5.145).abs() < 0.01);
        assert!((result.new_stability - 0.40).abs() < 0.01);
    }

    #[test]
    fn test_2_learning_graduation() {
        let result = process_review("learning", 1.0, 5.0, 7, &params());
        assert_eq!(result.new_interval, 3);
        assert_eq!(result.new_state, "review");
        assert!(result.new_stability > 1.0);
        assert!(result.new_difficulty < 5.0);
    }

    #[test]
    fn test_3_learning_stays_learning() {
        let result = process_review("learning", 1.0, 5.0, 3, &params());
        assert_eq!(result.new_interval, 1);
        assert_eq!(result.new_state, "learning");
        assert!(result.new_difficulty > 5.0);
        assert!((result.new_stability - 0.1).abs() < 0.01);
    }

    #[test]
    fn test_4_review_hard() {
        let result = process_review("review", 30.0, 5.0, 4, &params());
        assert_eq!(result.new_state, "review");
        assert!(result.new_difficulty > 5.0);
        assert!(result.new_stability > 30.0);
    }

    #[test]
    fn test_5_review_easy() {
        let result = process_review("review", 30.0, 5.0, 8, &params());
        assert_eq!(result.new_state, "review");
        assert!(result.new_difficulty < 5.0);
        assert!(result.new_stability > 30.0);
    }

    #[test]
    fn test_6_lapse() {
        let result = process_review("review", 30.0, 5.0, 1, &params());
        assert_eq!(result.new_interval, 1);
        assert_eq!(result.new_state, "relearning");
        assert!(result.is_lapse);
        assert!(result.new_difficulty > 5.0);
        assert!((result.new_stability - 10.8).abs() < 0.5);
    }

    #[test]
    fn test_7_relearning_recovery() {
        let result = process_review("relearning", 10.8, 6.3, 7, &params());
        assert_eq!(result.new_interval, 3);
        assert_eq!(result.new_state, "review");
        assert!(result.new_stability < 10.8);
    }

    #[test]
    fn test_8_difficulty_calibration() {
        let mut difficulty = 5.0;

        // High rating decreases difficulty
        let r1 = process_review("review", 15.0, difficulty, 9, &params());
        assert!(r1.new_difficulty < difficulty);

        difficulty = r1.new_difficulty;

        // Low rating increases difficulty
        let r2 = process_review("review", 15.0, difficulty, 2, &params());
        assert!(r2.new_difficulty > difficulty);

        // Mid rating minimal change
        let r3 = process_review("review", 15.0, 5.0, 5, &params());
        assert!((r3.new_difficulty - 5.0).abs() < 0.2);
    }

    #[test]
    fn test_9_full_lifecycle() {
        let test_params = params();

        // Step 1: New → learning
        let r1 = process_review("new", 0.0, 5.0, 7, &test_params);
        assert_eq!(r1.new_state, "learning");
        assert_eq!(r1.new_interval, 1);

        // Step 2: Learning → review (graduation)
        let r2 = process_review(
            "learning",
            r1.new_stability,
            r1.new_difficulty,
            8,
            &test_params,
        );
        assert_eq!(r2.new_state, "review");
        assert_eq!(r2.new_interval, 3);

        // Step 3: First review after graduation
        let r3 = process_review(
            "review",
            r2.new_stability,
            r2.new_difficulty,
            7,
            &test_params,
        );
        assert_eq!(r3.new_state, "review");
        assert!(
            r3.new_stability > r2.new_stability,
            "Stability should increase"
        );

        // Step 4: Continue building stability
        let r4 = process_review(
            "review",
            r3.new_stability,
            r3.new_difficulty,
            8,
            &test_params,
        );
        assert_eq!(r4.new_state, "review");
        assert!(
            r4.new_stability > r3.new_stability,
            "Stability should keep increasing"
        );

        // Step 5: More reviews with high ratings
        let r5 = process_review(
            "review",
            r4.new_stability,
            r4.new_difficulty,
            9,
            &test_params,
        );
        assert_eq!(r5.new_state, "review");
        assert!(
            r5.new_stability > r4.new_stability,
            "Stability should keep increasing"
        );

        // Verify monotonic stability growth over many reviews
        let mut current_stability = r5.new_stability;
        let mut current_difficulty = r5.new_difficulty;
        let mut max_interval = r5.new_interval;

        for i in 0..10 {
            let result = process_review(
                "review",
                current_stability,
                current_difficulty,
                8,
                &test_params,
            );
            assert_eq!(result.new_state, "review", "Should stay in review state");
            assert!(
                result.new_stability > current_stability,
                "Iteration {}: Stability should increase from {} to {}",
                i,
                current_stability,
                result.new_stability
            );

            // Track maximum interval seen
            if result.new_interval > max_interval {
                max_interval = result.new_interval;
            }

            current_stability = result.new_stability;
            current_difficulty = result.new_difficulty;
        }

        // After many reviews, stability should reach mastery threshold
        assert!(
            current_stability > 20.0,
            "After 15 reviews, stability should reach mastery level (21 days)"
        );

        // Intervals should have increased at some point
        assert!(
            max_interval > 3,
            "Maximum interval should have increased beyond initial 3 days"
        );
    }

    #[test]
    fn test_10_rating_bounds() {
        let test_params = params(); // Changed variable name

        // Test all ratings 1-10
        for rating in 1..=10 {
            let result = process_review("review", 15.0, 5.0, rating, &test_params);
            assert!(result.new_difficulty >= 1.0 && result.new_difficulty <= 10.0);
            assert!(result.new_stability > 0.0);
            assert!(result.new_interval >= 1 && result.new_interval <= 180);
        }
    }
}
