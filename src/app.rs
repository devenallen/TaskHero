use crate::gamification::{Gamification, PriorityLevel, Task}; // Use Task and PriorityLevel from gamification.rs

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    tasks: Vec<Task>,           // List of tasks
    selected_task: Option<usize>, // Index of selected task
    new_task_name: String,      // Fields for creating a new task
    new_task_description: String,
    new_task_due_date: String,
    new_task_priority: PriorityLevel,
    new_task_completed: bool,   
    is_editing: bool,           // Flag for editing task mode
    gamification: Gamification, // Gamification system
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            selected_task: None,
            new_task_name: String::new(),
            new_task_description: String::new(),
            new_task_due_date: String::new(),
            new_task_priority: PriorityLevel::Low,
            new_task_completed: false,
            is_editing: false,
            gamification: Gamification::new(), // Initialize gamification
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }

    /// Function to add a new task
    fn add_task(&mut self) {
        if !self.new_task_name.is_empty()
            && !self.new_task_description.is_empty()
            && !self.new_task_due_date.is_empty()
        {
            self.tasks.push(Task {
                name: self.new_task_name.clone(),
                description: self.new_task_description.clone(),
                due_date: self.new_task_due_date.clone(),
                priority: self.new_task_priority,
                completed: false,
            });

            // Clear inputs after adding task
            self.new_task_name.clear();
            self.new_task_description.clear();
            self.new_task_due_date.clear();
            self.new_task_priority = PriorityLevel::Low;
            self.new_task_completed = false;
        }
    }

    /// Function to update achievements and points
    fn update_achievements(&mut self) {
        self.gamification.check_challenges(&self.tasks); // Use gamification system to check challenges
    }
}

impl eframe::App for TemplateApp {
    /// Save app state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Update the UI
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
            });
        });

        // Task creation and display UI
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Add a Task");

            // Task input fields
            ui.horizontal(|ui| {
                ui.label("Task Name: ");
                ui.text_edit_singleline(&mut self.new_task_name);
            });
            ui.horizontal(|ui| {
                ui.label("Description: ");
                ui.text_edit_singleline(&mut self.new_task_description);
            });
            ui.horizontal(|ui| {
                ui.label("Due Date: ");
                ui.text_edit_singleline(&mut self.new_task_due_date);
            });
            ui.horizontal(|ui| {
                ui.label("Priority: ");
                let mut priority_val = self.new_task_priority as u8;
                ui.add(egui::Slider::new(&mut priority_val, 1..=3).text("Level"));
                self.new_task_priority = match priority_val {
                    1 => PriorityLevel::Low,
                    2 => PriorityLevel::Medium,
                    3 => PriorityLevel::High,
                    _ => PriorityLevel::Low,
                };
            });

            // Add task button
            if ui.button("Add Task").clicked() {
                self.add_task();
            }

            ui.separator();

            // Display tasks and achievements
            ui.heading("Tasks");
            for (i, task) in self.tasks.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut task.completed, "");
                    ui.label(&task.name);
                    if ui.button("Details").clicked() {
                        self.selected_task = Some(i);
                    }
                });
            }

            // Check for achievements
            self.update_achievements();
        });

        // Achievements UI
        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Achievements");
            ui.separator();
            ui.label(&self.gamification.achievement_message); // Display the achievement message
            ui.separator();

            // Display task progress toward goals
            let completed_tasks = self.tasks.iter().filter(|task| task.completed).count();
            ui.label(format!(
                "Current Tasks: {} / Bronze Goal: {}",
                completed_tasks, self.gamification.bronze_goal
            ));
            ui.label(format!(
                "Current Tasks: {} / Silver Goal: {}",
                completed_tasks, self.gamification.silver_goal
            ));
            ui.label(format!(
                "Current Tasks: {} / Gold Goal: {}",
                completed_tasks, self.gamification.gold_goal
            ));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading(format!("Challenges (Points: {})", self.gamification.points));

            });
        });
    }
}
