#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
});
extern crate task_hero;

use task_hero::gamification::{PriorityLevel, Task};

fn fuzz_target(data: &[u8]) {
    // Try to construct a Task with fuzzed data
    if let Ok(name) = std::str::from_utf8(data) {
        let task = Task {
            name: name.to_string(),
            description: "Fuzz description".to_string(),
            due_date: "2024-11-22".to_string(),
            priority: PriorityLevel::High, // Fixed priority for testing
            completed: true,
            completed_date: Some("2024-11-22".to_string()),
        };

        // Calculate points and ensure no panics
        let _ = task.points();
    }
}
