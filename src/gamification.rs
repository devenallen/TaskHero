#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum PriorityLevel {
    Low = 1,
    Medium = 2,
    High = 3,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub due_date: String,
    pub priority: PriorityLevel,
    pub completed: bool,
}

impl Task {
    // Helper function to calculate points based on priority level
    pub fn points(&self) -> u32 {
        match self.priority {
            PriorityLevel::Low => 10,
            PriorityLevel::Medium => 20,
            PriorityLevel::High => 30,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Gamification {
    pub points: u32,
    pub bronze_goal: u32,
    pub silver_goal: u32,
    pub gold_goal: u32,
    pub achievement_message: String,
}

impl Gamification {
    pub fn new() -> Self {
        Gamification {
            points: 0,
            bronze_goal: 5,
            silver_goal: 10,
            gold_goal: 20,
            achievement_message: String::new(),
        }
    }

    // Check for challenge achievements
    pub fn check_challenges(&mut self, tasks: &[Task]) {
        let bronze_points = 50;
        let silver_points = 100;
        let gold_points = 500;

        let completed_tasks = tasks.iter().filter(|task| task.completed).count();
        self.points = tasks.iter().filter(|task| task.completed).map(Task::points).sum();

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

    fn display_achievement(&mut self, message: &str) {
        self.achievement_message = message.to_string();
    }
}