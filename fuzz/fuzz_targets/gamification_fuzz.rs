#![no_main]
use libfuzzer_sys::fuzz_target;

extern crate task_hero;

use task_hero::gamification::{PriorityLevel, Task, Gamification};

// Generate random Task and Gamification instances and test methods
fuzz_target!(|data: &[u8]| {
    // Convert `data` to a string if possible
    if let Ok(random_string) = std::str::from_utf8(data) {
        // Create a list of tasks using random strings
        let tasks = vec![
            Task {
                name: random_string.to_string(),
                description: random_string.to_string(),
                due_date: random_string.to_string(),
                priority: PriorityLevel::Low, // Randomize later for more thorough fuzzing
                completed: false,
                completed_date: None,
            }
        ];

        // Initialize a Gamification instance
        let mut gamification = Gamification::new();

        // Call methods with fuzzed data
        gamification.check_challenges(&tasks);
        gamification.daily_reward(&tasks);
        gamification.weekly_challenge(&tasks);
    }
});
