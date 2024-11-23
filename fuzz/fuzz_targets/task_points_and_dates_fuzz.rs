#![no_main]
use libfuzzer_sys::fuzz_target;
use task_hero::gamification::{Task, Gamification, PriorityLevel};
use chrono::NaiveDate;

fuzz_target!(|data: &[u8]| {
    // Try converting input to a UTF-8 string for date testing
    if let Ok(random_string) = std::str::from_utf8(data) {
        // Test points calculation with different priorities
        let task = Task {
            name: "Test Task".to_string(),
            description: "Description".to_string(),
            due_date: "2024-11-23".to_string(),
            priority: match random_string.chars().next() {
                Some('H') => PriorityLevel::High,
                Some('M') => PriorityLevel::Medium,
                _ => PriorityLevel::Low,
            },
            completed: true,
            completed_date: Some(random_string.to_string()),
        };

        // Call Task::points() and ensure no panics
        let _ = task.points();

        // Initialize Gamification and test date-dependent logic
        let mut gamification = Gamification::new();

        // Fuzz daily_reward with potentially invalid dates
        gamification.daily_reward(&[task.clone()]);

        // Fuzz weekly_challenge with extreme or invalid date ranges
        let malformed_task = Task {
            name: "Malformed Task".to_string(),
            description: "Description".to_string(),
            due_date: "invalid-date".to_string(), // Inject invalid date
            priority: PriorityLevel::Low,
            completed: true,
            completed_date: Some(random_string.to_string()),
        };
        gamification.weekly_challenge(&[malformed_task]);

        // Check how NaiveDate parsing handles edge cases
        let _ = NaiveDate::parse_from_str(random_string, "%Y-%m-%d").ok();
    }
});
