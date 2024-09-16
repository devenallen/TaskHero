pub struct Task {
    pub title: String,
    pub completed: bool,
}

impl Task {
    pub fn new(title: &str) -> Task {
        Task {
            title: title.to_string(),
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

pub fn create_task(title: &str) -> Task {
    Task::new(title)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        let task = create_task("Learn Rust");
        assert_eq!(task.title, "Learn Rust");
        assert!(!task.completed);
    }

    #[test]
    fn test_complete_task() {
        let mut task = create_task("Learn Rust");
        task.complete();
        assert!(task.completed);
    }
}

