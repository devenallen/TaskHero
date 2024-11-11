#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
/// Priority levels for tasks
pub enum PriorityLevel {
    Low = 1,
    Medium = 2,
    High = 3,
}

#[derive(serde::Deserialize, serde::Serialize)]
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
        // get the current date
        let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();

        // get the date 7 days ago
        let seven_days_ago = chrono::Local::now().checked_sub_signed(chrono::Duration::days(7)).unwrap().format("%Y-%m-%d").to_string();

        // get the number of tasks completed each day in the last 7 days
        let mut tasks_completed_each_day = vec![0; 7];
        for task in tasks {
            if let Some(completed_date) = &task.completed_date {
                // check if the task was completed within the last 7 days
                if completed_date >= &seven_days_ago && completed_date <= &current_date {
                    // calculate the index of the day in the last 7 days
                    let days_diff = chrono::NaiveDate::parse_from_str(&current_date, "%Y-%m-%d").unwrap()
                        .signed_duration_since(chrono::NaiveDate::parse_from_str(completed_date, "%Y-%m-%d").unwrap())
                        .num_days() as usize;
                    tasks_completed_each_day[6 - days_diff] += 1;
                }
            }
        }

        // if the user has completed a task every day for the last 7 days, give them 100 points
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