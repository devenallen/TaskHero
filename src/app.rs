// In gamification.rs
use crate::gamification::{Gamification, PriorityLevel, Task};
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]

/// This struct represents the main application state.
/// It contains the list of tasks, the selected task index, and the fields for creating a new task.
pub struct TemplateApp {
    tasks: Vec<Task>,                 // List of tasks
    selected_task: Option<usize>,     // Index of selected task
    new_task_name: String,            // Fields for creating a new task
    new_task_description: String,     // Description of the new task
    new_task_due_date: String,        // Due date of the new task
    new_task_priority: PriorityLevel, // Priority level of the new task
    new_task_completed: bool,         // Flag for new task completion
    is_editing: bool,                 // Flag for editing task mode
    gamification: Gamification,       // Gamification system
    details_report_viewable: bool,    // Flag for viewing the details report
}

/// Implement the Default trait for TemplateApp to provide a default state.
impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),                     // Initialize tasks list
            selected_task: None,                   // Initialize selected task index
            new_task_name: String::new(),          // Initialize new task fields
            new_task_description: String::new(),   // Initialize new task fields
            new_task_due_date: String::new(),      // Initialize new task fields
            new_task_priority: PriorityLevel::Low, // Initialize new task fields
            new_task_completed: false,             // Initialize new task fields
            is_editing: false,                     // Initialize editing mode
            gamification: Gamification::new(),     // Initialize gamification
            details_report_viewable: false,        // Initialize details report viewable flag
        }
    }
}

/// Implement the TemplateApp struct.
/// This struct contains the main application logic and UI layout.
impl TemplateApp {

    /// This function creates a new instance of the TemplateApp struct.
    /// It initializes the app state and loads the state from storage if available.
    /// 
    /// # Arguments
    /// 
    /// * `cc` - The eframe::CreationContext containing the storage for the app state.
    /// 
    /// # Returns
    /// 
    /// A new instance of the TemplateApp struct.
    /// 
    /// # Example
    /// 
    /// ```
    /// let app = TemplateApp::new(&cc);
    /// ```
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }
        // Default::default()
        if let Some(storage) = cc.storage {
            app = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        };

        // Trigger achievements check when app is first loaded
        app.update_achievements();

        app
    }

    /// This function adds a new task to the task list.
    /// It checks if the task name, description, and due date are not empty before adding the task.
    /// 
    /// # Example
    /// 
    /// ```
    /// app.add_task();
    /// ```
    /// 
    /// # Notes
    /// 
    /// This function modifies the app state by adding a new task to the task list.
    /// It also clears the input fields after adding the task.
    /// 
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
                completed_date: None,
            });

            // Clear inputs after adding task
            self.new_task_name.clear();
            self.new_task_description.clear();
            self.new_task_due_date.clear();
            self.new_task_priority = PriorityLevel::Low;
            self.new_task_completed = false;
        }
    }

    /// This function updates the achievements based on the task list.
    /// It uses the gamification system to check challenges and daily rewards.
    /// 
    /// # Example
    /// 
    /// ```
    /// app.update_achievements();
    /// ```
    /// 
    /// # Notes
    /// 
    /// This function modifies the app state by updating the achievement messages and points.
    /// 
    fn update_achievements(&mut self) {
        self.gamification.check_challenges(&self.tasks); // Use gamification system to check challenges
        // check the daily goal
        self.gamification.daily_reward(&self.tasks);
    }

    /// This function handles the logic for the left panel of the UI.
    /// It contains the task creation form, task list, and task details.
    /// 
    /// # Arguments
    /// 
    /// * `ctx` - The egui::Context for the UI.
    /// 
    /// # Example
    /// 
    /// ```
    /// app.left_panel_logic(&ctx);
    /// ```
    fn left_panel_logic(&mut self, ctx: &egui::Context) {
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
                    // if the task checkbox is checked, update the completed date, otherwise set it to None
                    if task.completed {
                        task.completed_date = Some(chrono::Local::now().format("%Y-%m-%d").to_string().to_string());
                    } else {
                        task.completed_date = None;
                    }
                    ui.label(&task.name);
                    if ui.button("Details").clicked() {
                        self.selected_task = Some(i);
                    }
                });

            }

            // Check if a task is selected and display its details
            if let Some(selected_index) = self.selected_task {
                let selected_task = &mut self.tasks[selected_index]; // Get the selected task

                ui.separator();
                ui.heading("Task Details");

                // Editable fields for the selected task
                if self.is_editing {
                    ui.label("Editing Task:");
                    ui.horizontal(|ui| {
                        ui.label("Name: ");
                        ui.text_edit_singleline(&mut selected_task.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Description: ");
                        ui.text_edit_multiline(&mut selected_task.description);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Due Date: ");
                        ui.text_edit_singleline(&mut selected_task.due_date); // Assuming due_date is a String
                    });
                    ui.horizontal(|ui| {
                        ui.label("Priority: ");
                        let mut priority_val = selected_task.priority as u8;
                        ui.add(egui::Slider::new(&mut (priority_val), 1..=3).text("Level"));
                        selected_task.priority = match priority_val {
                            1 => PriorityLevel::Low,
                            2 => PriorityLevel::Medium,
                            3 => PriorityLevel::High,
                            _ => PriorityLevel::Low,
                        };
                    });

                    // Save Changes button
                    if ui.button("Save Changes").clicked() {
                        self.is_editing = false; // Exit editing mode
                    }

                    // Cancel Edits button
                    if ui.button("Cancel Edits").clicked() {
                        self.is_editing = false; // Exit editing mode
                    }
                } else {
                    // Display read-only fields for the selected task
                    ui.label(format!("Name: {}", selected_task.name));
                    ui.label(format!("Description: {}", selected_task.description));
                    ui.label(format!("Due Date: {}", selected_task.due_date));
                    ui.label(format!("Priority: {:?}", selected_task.priority));
                    ui.label(format!("Completed: {}", selected_task.completed));

                    // Edit Task button
                    if ui.button("Edit Task").clicked() {
                        self.is_editing = true; // Enter editing mode
                    }
                }

                ui.separator();

                // Clear all tasks button
                if ui.button("Clear All Tasks").clicked() {
                    self.tasks.clear();
                }
            }
            // Check for achievements
            self.update_achievements();
        });
    }

    /// This function handles the logic for the right panel of the UI.
    /// It contains the achievements, task progress, and goal setting UI.
    /// 
    /// # Arguments
    /// 
    /// * `ctx` - The egui::Context for the UI.
    /// 
    /// # Example
    /// 
    /// ```
    /// app.right_panel_logic(&ctx);
    /// ```
    fn right_panel_logic(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Achievements");
            ui.separator();
            ui.label(&self.gamification.achievement_message); // Display the achievement message
            ui.separator();

            // Display task progress toward goals
            let completed_tasks = self.tasks.iter().filter(|task| task.completed).count();
            ui.separator();
            // add a progress bar for the bronze goal 
            ui.add(egui::ProgressBar::new(completed_tasks as f32 / self.gamification.bronze_goal as f32).text(format!("Bronze Goal: {}/{}", if completed_tasks <= self.gamification.bronze_goal as usize { completed_tasks } else {self.gamification.bronze_goal as usize}, self.gamification.bronze_goal)));
            ui.separator();
            // add a progress bar for the silver goal
            ui.add(egui::ProgressBar::new(completed_tasks as f32 / self.gamification.silver_goal as f32).text(format!("Silver Goal: {}/{}", if completed_tasks <= self.gamification.silver_goal as usize { completed_tasks } else {self.gamification.silver_goal as usize}, self.gamification.silver_goal)));
            ui.separator();
            // add a progress bar for the gold goal
            ui.add(egui::ProgressBar::new(completed_tasks as f32 / self.gamification.gold_goal as f32).text(format!("Gold Goal: {}/{}", if completed_tasks <= self.gamification.gold_goal as usize { completed_tasks } else {self.gamification.gold_goal as usize}, self.gamification.gold_goal)));

            ui.separator();

            // Add goal setting UI panel
            ui.heading("Customize Your Goals");

            // Input fields to set bronze, silver, and gold task goals
            ui.horizontal(|ui| {
                ui.label("Bronze Goal: ");
                ui.add(egui::DragValue::new(&mut self.gamification.bronze_goal).speed(1).range(1..=self.gamification.silver_goal-1));
            });

            ui.horizontal(|ui| {
                ui.label("Silver Goal: ");
                ui.add(egui::DragValue::new(&mut self.gamification.silver_goal).speed(1).range(self.gamification.bronze_goal+1..=self.gamification.gold_goal-1));
            });

            ui.horizontal(|ui| {
                ui.label("Gold Goal: ");
                ui.add(egui::DragValue::new(&mut self.gamification.gold_goal).speed(1).range(self.gamification.silver_goal+1..=100));
            });

            ui.separator();
        });
    }

    /// This function handles the logic for the central panel of the UI.
    /// It contains the daily rewards, weekly challenges, and task tracking UI.
    /// 
    /// # Arguments
    /// 
    /// * `ctx` - The egui::Context for the UI.
    /// 
    /// # Example
    /// 
    /// ```
    /// app.central_panel_logic(&ctx);
    /// ```
    fn central_panel_logic(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading(format!("Challenges (Points: {})", self.gamification.points));

            });
            ui.separator();
            // Display the status on daily rewards
            ui.heading("Daily Rewards");
            ui.separator();
            // Show a message explaining the daily reward system
            ui.label("Complete 5, 10, or 15 tasks in a day to earn rewards!");
            ui.separator();
            // Display the daily reward message
            ui.label(&self.gamification.daily_reward_message);
            // Display the daily reward points
            ui.label(format!("Daily Reward Points: {}", self.gamification.daily_reward));
            // Display a message saying that daily points reset at the end of the day
            ui.label("Daily points reset at the end of the day.");

            ui.separator();
            // Display the weekly challenge message
            ui.heading("Weekly Challenge");
            ui.separator();
            ui.label(&self.gamification.weekly_challenge_message);

        });
    }

    /// This function handles the logic for the bottom panel of the UI.
    /// It contains the tasks report and metrics display.
    /// 
    /// # Arguments
    /// 
    /// * `ctx` - The egui::Context for the UI.
    /// 
    /// # Example
    /// 
    /// ```
    /// app.bottom_panel_logic(&ctx);
    /// ```
    fn bottom_panel_logic(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            // if the button is clicked, display a report of metrics of tasks
            if ui.button("Tasks Report").clicked() {self.details_report_viewable = true;}
            if self.details_report_viewable {
                // put a report of metrics of tasks, including total number, % completed, and % of each priority level on the screen
                let total_tasks = self.tasks.len();
                let completed_tasks = self.tasks.iter().filter(|task| task.completed).count();
                let low_priority_tasks = self.tasks.iter().filter(|task| task.priority == PriorityLevel::Low).count();
                let medium_priority_tasks = self.tasks.iter().filter(|task| task.priority == PriorityLevel::Medium).count();
                let high_priority_tasks = self.tasks.iter().filter(|task| task.priority == PriorityLevel::High).count();
                let completed_percentage = (completed_tasks as f32 / total_tasks as f32) * 100.0;
                let low_priority_percentage = (low_priority_tasks as f32 / total_tasks as f32) * 100.0;
                let medium_priority_percentage = (medium_priority_tasks as f32 / total_tasks as f32) * 100.0;
                let high_priority_percentage = (high_priority_tasks as f32 / total_tasks as f32) * 100.0;
                // put the above metrics on the screen
                //To add new details about the tasks like Total Points Earned,Incomplete Tasks,Average Task Priority,Most Common Task Priority,Upcoming Tasks
                 // 1. Total Points Earned
                let total_points: u32 = self.tasks.iter()
                .filter(|task| task.completed)
                .map(|task| task.points())
                .sum();
                // 2. Incomplete Tasks
                let incomplete_tasks = total_tasks - completed_tasks;
                let incomplete_percentage = if total_tasks > 0 {
                    (incomplete_tasks as f32 / total_tasks as f32) * 100.0
                } else {
                    0.0
                };
                // 3. Average Task Priority
                let total_priority: u32 = self.tasks.iter()
                .map(|task| task.priority as u32)
                .sum();
                let avg_priority = if total_tasks > 0 {
                    total_priority as f32 / total_tasks as f32
                } else {
                    0.0
                };
                 // 4. Most Common Task Priority
                let mut priority_counts = [0; 3]; // For Low, Medium, High
                for task in &self.tasks {
                    match task.priority {
                        PriorityLevel::Low => priority_counts[0] += 1,
                        PriorityLevel::Medium => priority_counts[1] += 1,
                        PriorityLevel::High => priority_counts[2] += 1,
                    }
                }
                let most_common_priority = if priority_counts[0] >= priority_counts[1] && priority_counts[0] >= priority_counts[2] {
                    "Low"
                } else if priority_counts[1] >= priority_counts[0] && priority_counts[1] >= priority_counts[2] {
                    "Medium"
                } else {
                    "High"
                };
                 // 5. Upcoming Tasks (with future due dates)
                let upcoming_tasks = self.tasks.iter()
                 .filter(|task| {
                     // Parse the due date in the format MM.DD.YYYY
                     if let Ok(due_date) = chrono::NaiveDate::parse_from_str(&task.due_date, "%m.%d.%Y") {
                         due_date > chrono::Local::now().naive_local().date() // Compare the due date with the current date
                     } else {
                         false // If the date parsing fails, it's not an upcoming task
                     }
                })
                 .count();
                ui.label(format!("Total Tasks: {}", total_tasks));
                ui.label(format!("Completed Tasks: {} ({:.2}%)", completed_tasks, completed_percentage));
                ui.label(format!("Low Priority Tasks: {} ({:.2}%)", low_priority_tasks, low_priority_percentage));
                ui.label(format!("Medium Priority Tasks: {} ({:.2}%)", medium_priority_tasks, medium_priority_percentage));
                ui.label(format!("High Priority Tasks: {} ({:.2}%)", high_priority_tasks, high_priority_percentage));
                //New metric
                ui.label(format!("Total Points Earned: {}", total_points));
                ui.label(format!("Incomplete Tasks: {} ({:.2}%)", incomplete_tasks, incomplete_percentage));
                ui.label(format!("Average Task Priority: {:.2}", avg_priority));
                ui.label(format!("Most Common Task Priority: {}", most_common_priority));
                ui.label(format!("Upcoming Tasks: {}", upcoming_tasks));
                 //add a button to close the report
                if ui.button("Close Report").clicked() {
                    self.details_report_viewable = false;
                }
                
            } else { // print out a message saying that the report is available
                ui.label("Tasks Report available.");

            }
        });
    }
}

/// Implement the eframe::App trait for the TemplateApp struct.
/// This trait provides the necessary methods to run the app and update the UI.
impl eframe::App for TemplateApp {
    /// Save the app state to storage.
    /// 
    /// # Arguments
    /// 
    /// * `storage` - The eframe::Storage to save the app state.
    /// 
    /// # Example
    /// 
    /// ```
    /// app.save(&mut storage);
    /// ```
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Update the app state and UI.
    /// 
    /// # Arguments
    /// 
    /// * `ctx` - The egui::Context for the UI.
    /// * `frame` - The eframe::Frame for the UI.
    /// 
    /// # Example
    /// 
    /// ```
    /// app.update(&ctx, &mut frame);
    /// ```
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

        // task creation/ info UI
        self.left_panel_logic(ctx);

        // achievement UI
        self.right_panel_logic(ctx);

        // central tracking UI
        self.central_panel_logic(ctx);

        // task report UI
        self.bottom_panel_logic(ctx);

    }
}
