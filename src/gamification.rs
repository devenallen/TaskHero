#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
/// Priority levels for tasks
pub enum PriorityLevel {
    Low = 1,
    Medium = 2,
    High = 3,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
/// Task struct with fields for name, description, due date, priority level, and completion status
pub struct Task {
    pub name: String,
    pub description: String,
    pub due_date: String,
    pub priority: PriorityLevel,
    pub completed: bool,
    pub completed_date: Option<String>, // add when the task was completed
}

/// Implementation of Task struct with a method to calculate points based on priority level
impl Task {
    /// Helper function to calculate points based on priority level
    pub fn points(&self) -> u32 {
        match self.priority {
            PriorityLevel::Low => 10,
            PriorityLevel::Medium => 20,
            PriorityLevel::High => 30,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
/// Gamification struct with fields for points, goals, and achievement messages
pub struct Gamification {
    pub points: u32,
    pub bronze_goal: u32,
    pub silver_goal: u32,
    pub gold_goal: u32,
    pub achievement_message: String,
    pub daily_reward: u32,
    pub daily_reward_message: String,
    pub weekly_challenge_message: String,
}

/// Implementation of Gamification struct with methods to check challenges, daily rewards, and weekly challenges
impl Gamification {
    /// Helper function to create a new instance of Gamification
    pub fn new() -> Self {
        Gamification {
            points: 0,
            bronze_goal: 5,
            silver_goal: 10,
            gold_goal: 20,
            achievement_message: String::new(),
            daily_reward: 0,
            daily_reward_message: String::new(),
            weekly_challenge_message: String::from("Complete a task every day for a week to earn 100 points!"),
        }
    }

    /// Helper function to check the user's progress and display achievement messages
    /// based on the number of completed tasks and total points
    /// 
    /// # Arguments
    /// 
    /// * `tasks` - A slice of Task structs representing the user's tasks
    pub fn check_challenges(&mut self, tasks: &[Task]) {
        let bronze_points = 50;
        let silver_points = 100;
        let gold_points = 500;

        let completed_tasks = tasks.iter().filter(|task| task.completed).count(); // count the number of completed tasks
        self.points = tasks.iter().filter(|task| task.completed).map(Task::points).sum(); // calculate the total points

        if self.points >= gold_points && completed_tasks >= self.gold_goal as usize {
            self.display_achievement("Congrats! You have reached the Gold level!");
        } else if self.points >= silver_points && completed_tasks >= self.silver_goal as usize {
            self.display_achievement("Congrats! You have reached the Silver level!");
        } else if self.points >= bronze_points && completed_tasks >= self.bronze_goal as usize {
            self.display_achievement("Congrats! You have reached the Bronze level!");
        } else {
            self.display_achievement("Keep going! You're progressing toward the next level!");
        }
    }

    /// Helper function to calculate the daily reward based on the number of tasks completed in a day
    /// and display a message to the user
    /// 
    /// # Arguments
    /// 
    /// * `tasks` - A slice of Task structs representing the user's tasks
    pub fn daily_reward(&mut self, tasks: &[Task]) {
        // calculate the number of tasks completed within the last day using the completed_date field
        let daily_tasks: usize = tasks.iter().filter(|task| {
            if let Some(completed_date) = &task.completed_date {
                // check if the task was completed within the last day
                // (for simplicity, assume the date format is "YYYY-MM-DD")
                completed_date == &chrono::Local::now().format("%Y-%m-%d").to_string()
            } else {
                false
            }
        }).count();

        // if the user completed 15, 10, or 5 tasks in a day, give them 100, 50, or 25 points, respectively
        if daily_tasks >= 15 {
            self.display_daily_reward("Congrats! You completed 15 tasks today!");
            self.daily_reward = 100;
        } else if daily_tasks >= 10 {
            self.display_daily_reward("Congrats! You completed 10 tasks today!");
            self.daily_reward = 50;
        } else if daily_tasks >= 5 {
            self.display_daily_reward("Congrats! You completed 5 tasks today!");
            self.daily_reward = 25;
        } else {
            self.display_daily_reward("Keep going! You're making progress!");
            self.daily_reward = 0;
        }
    }

    /// Helper function to calculate the weekly challenge based on the number of tasks completed each day
    /// in the last 7 days and display a message to the user
    /// 
    /// # Arguments
    /// 
    /// * `tasks` - A slice of Task structs representing the user's tasks
    pub fn weekly_challenge(&mut self, tasks: &[Task]) {
        // Get the current date
        let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    
        // Get the date 7 days ago
        let seven_days_ago = chrono::Local::now().checked_sub_signed(chrono::Duration::days(7))
            .map(|date| date.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "1970-01-01".to_string()); // Default to a valid date if subtraction fails
    
        // Get the number of tasks completed each day in the last 7 days
        let mut tasks_completed_each_day = vec![0; 7];
    
        for task in tasks {
            if let Some(completed_date) = &task.completed_date {
                // Check if the task was completed within the last 7 days
                if completed_date >= &seven_days_ago && completed_date <= &current_date {
                    // Try parsing the current date and completed date safely
                    let current_naive_date = chrono::NaiveDate::parse_from_str(&current_date, "%Y-%m-%d");
                    let completed_naive_date = chrono::NaiveDate::parse_from_str(completed_date, "%Y-%m-%d");
    
                    // If both dates are valid, calculate the difference
                    if let (Ok(current), Ok(completed)) = (current_naive_date, completed_naive_date) {
                        let days_diff = current.signed_duration_since(completed).num_days() as usize;
                        if days_diff < 7 {
                            tasks_completed_each_day[6 - days_diff] += 1;
                        }
                    }
                }
            }
        }
    
        // If the user has completed a task every day for the last 7 days, give them 100 points
        if tasks_completed_each_day.iter().all(|&count| count > 0) {
            self.points = 100;
            self.display_weekly_challenge("Congrats! You completed a task every day for the last week!");
        }
    }
    
    

    /// Helper function to display an achievement message to the user
    fn display_achievement(&mut self, message: &str) {
        self.achievement_message = message.to_string();
    }

    /// Helper function to display a daily reward message to the user
    fn display_daily_reward(&mut self, message: &str) {
        self.daily_reward_message = message.to_string();
    }

    /// Helper function to display a weekly challenge message to the user
    fn display_weekly_challenge(&mut self, message: &str) {
        self.weekly_challenge_message = message.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local};

    #[test]
    fn test_task_points() {
        let low_priority_task = Task {
            name: "Task 1".to_string(),
            description: "A low priority task".to_string(),
            due_date: "2024-11-23".to_string(),
            priority: PriorityLevel::Low,
            completed: true,
            completed_date: None,
        };
        assert_eq!(low_priority_task.points(), 10);

        let medium_priority_task = Task {
            name: "Task 2".to_string(),
            description: "A medium priority task".to_string(),
            due_date: "2024-11-23".to_string(),
            priority: PriorityLevel::Medium,
            completed: true,
            completed_date: None,
        };
        assert_eq!(medium_priority_task.points(), 20);

        let high_priority_task = Task {
            name: "Task 3".to_string(),
            description: "A high priority task".to_string(),
            due_date: "2024-11-23".to_string(),
            priority: PriorityLevel::High,
            completed: true,
            completed_date: None,
        };
        assert_eq!(high_priority_task.points(), 30);
    }

    #[test]
    fn test_gamification_check_challenges() {
        let tasks = vec![
            Task {
                name: "Task 1".to_string(),
                description: "A completed task".to_string(),
                due_date: "2024-11-23".to_string(),
                priority: PriorityLevel::Low,
                completed: true,
                completed_date: None,
            },
            Task {
                name: "Task 2".to_string(),
                description: "Another completed task".to_string(),
                due_date: "2024-11-23".to_string(),
                priority: PriorityLevel::Medium,
                completed: true,
                completed_date: None,
            },
        ];

        let mut gamification = Gamification::new();
        gamification.check_challenges(&tasks);

        assert_eq!(gamification.points, 30); // 10 (Low) + 20 (Medium)
        assert_eq!(
            gamification.achievement_message,
            "Keep going! You're progressing toward the next level!"
        );
    }

    #[test]
    fn test_gamification_daily_reward() {
        let today = Local::now().format("%Y-%m-%d").to_string();
        let tasks = vec![
            Task {
                name: "Task 1".to_string(),
                description: "Completed today".to_string(),
                due_date: today.clone(),
                priority: PriorityLevel::High,
                completed: true,
                completed_date: Some(today.clone()),
            },
            Task {
                name: "Task 2".to_string(),
                description: "Another completed today".to_string(),
                due_date: today.clone(),
                priority: PriorityLevel::Medium,
                completed: true,
                completed_date: Some(today.clone()),
            },
        ];

        let mut gamification = Gamification::new();
        gamification.daily_reward(&tasks);

        assert_eq!(gamification.daily_reward, 0); // Less than 5 tasks
        assert_eq!(
            gamification.daily_reward_message,
            "Keep going! You're making progress!"
        );
    }

    #[test]
    fn test_gamification_weekly_challenge() {
        let today = Local::now().format("%Y-%m-%d").to_string();

        // Generate tasks completed for each of the last 7 days
        let tasks: Vec<Task> = (0..7)
            .map(|day_offset| {
                let date = Local::now()
                    .checked_sub_signed(Duration::days(day_offset))
                    .unwrap()
                    .format("%Y-%m-%d")
                    .to_string();

                Task {
                    name: format!("Task {}", day_offset + 1),
                    description: "Completed task".to_string(),
                    due_date: date.clone(),
                    priority: PriorityLevel::Medium,
                    completed: true,
                    completed_date: Some(date),
                }
            })
            .collect();

        let mut gamification = Gamification::new();
        gamification.weekly_challenge(&tasks);

        // Expecting 100 points because a task was completed each day for the last 7 days
        assert_eq!(gamification.points, 100);
        assert_eq!(
            gamification.weekly_challenge_message,
            "Congrats! You completed a task every day for the last week!"
        );
    }


    #[test]
    fn test_gamification_achievements() {
        let tasks = vec![
            Task {
                name: "Task 1".to_string(),
                description: "A completed high priority task".to_string(),
                due_date: "2024-11-23".to_string(),
                priority: PriorityLevel::High,
                completed: true,
                completed_date: None,
            },
            Task {
                name: "Task 2".to_string(),
                description: "Another completed high priority task".to_string(),
                due_date: "2024-11-23".to_string(),
                priority: PriorityLevel::High,
                completed: true,
                completed_date: None,
            },
        ];

        let mut gamification = Gamification::new();
        gamification.check_challenges(&tasks);

        assert_eq!(gamification.points, 60); // 30 (High) + 30 (High)
        assert_eq!(
            gamification.achievement_message,
            "Keep going! You're progressing toward the next level!"
        );
    }

    #[test]
    fn test_daily_reward_exact_thresholds() {
        let today = Local::now().format("%Y-%m-%d").to_string();

        let tasks: Vec<Task> = (0..15)
            .map(|i| Task {
                name: format!("Task {}", i + 1),
                description: "Completed today".to_string(),
                due_date: today.clone(),
                priority: PriorityLevel::Low,
                completed: true,
                completed_date: Some(today.clone()),
            })
            .collect();

        let mut gamification = Gamification::new();

        // Test for 5 tasks
        gamification.daily_reward(&tasks[0..5]);
        assert_eq!(gamification.daily_reward, 25);

        // Test for 10 tasks
        gamification.daily_reward(&tasks[0..10]);
        assert_eq!(gamification.daily_reward, 50);

        // Test for 15 tasks
        gamification.daily_reward(&tasks[0..15]);
        assert_eq!(gamification.daily_reward, 100);
    }
}
