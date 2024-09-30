#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)] // Ensure you derive both Deserialize and Serialize
pub enum PriorityLevel {
    Low = 1,
    Medium = 2,
    High = 3
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    tasks: Vec<Task>, // List of tasks
    selected_task: Option<usize>, // Index of selected task
    points: u32, // Points earned by completing tasks

    // Fields for the new task
    new_task_name: String, // Name of the new task being created
    new_task_description: String, // Description of the new task
    new_task_due_date: String, // Due date of the new task
    new_task_priority: PriorityLevel, // Priority of the new task
    new_task_completed: bool, // Whether the new task is completed
    is_editing: bool, // Flag to indicate if the task is being edited
    //Challenge achievment message
    achievement_message: String,
    //New fields for goal setting
    bronze_goal:u32,
    silver_goal:u32,
    gold_goal:u32,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Task {
    name: String, // Name of the task
    description: String, // Description of the task
    due_date: String, // Due date of the task
    priority: PriorityLevel, // Priority of the task
    completed: bool, // Whether the task is completed
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            tasks: Vec::new(),
            selected_task: None,
            points: 0,
            new_task_name: String::new(),
            new_task_description: String::new(),
            new_task_due_date: String::new(),
            new_task_priority: PriorityLevel::Low,
            new_task_completed: false,
            is_editing: false,
            //Default goal values
            bronze_goal:5,
            silver_goal:10,
            gold_goal:20,
            //Dispay the challenge message
            achievement_message: String::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
    //add fields to track challenge progress
    fn check_challenges(&mut self) {
        let bronze_points = 50;
        let silver_points = 100;
        let gold_points = 500;

        // Count completed tasks
        let completed_tasks = self.tasks.iter().filter(|task| task.completed).count();

        // If user has completed the Gold level
        if self.points >= gold_points && completed_tasks >= self.gold_goal as usize {
            self.display_achievement("Congrats! You have reached the Gold level!");
            return;  // Stop further checking once Gold is achieved
        }

        // If user has completed the Silver level but not Gold
        if self.points >= silver_points && completed_tasks >= self.silver_goal as usize {
            self.display_achievement("Congrats! You have reached the Silver level!");
            return;  // Stop further checking once Silver is achieved
        }

        // If user has completed the Bronze level but not Silver or Gold
        if self.points >= bronze_points && completed_tasks >= self.bronze_goal as usize {
            self.display_achievement("Congrats! You have reached the Bronze level!");
            return;  // Stop further checking once Bronze is achieved
        }

        // Generic fallback message if no level is achieved yet
        self.display_achievement("Keep going! You're progressing toward the next level!");
    }

    fn display_achievement(&mut self, message: &str) {
        self.achievement_message = message.to_string(); // Store the achievement message
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
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

        // Create a left side panel for the tasks and other controls.
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .min_width(ctx.available_rect().width() / 2.0) // Ensures the panel fills half the window.
            .max_width(ctx.available_rect().width() / 2.0) // Keeps the width fixed.
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {

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
                        //ui.add(egui::Slider::new(&mut self.new_task_priority, 1..=3));
                        let mut priority_val = self.new_task_priority as u8;
                        ui.add(egui::Slider::new(&mut (priority_val), 1..=3).text("Level"));
                        self.new_task_priority = match priority_val {
                            1 => PriorityLevel::Low,
                            2 => PriorityLevel::Medium,
                            3 => PriorityLevel::High,
                            _ => PriorityLevel::Low,
                        };
                    });

                    // Button to add a task
                    if ui.button("Add Task").clicked() {
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
                    ui.separator();

                    // Display the list of tasks with scrolling
                    ui.heading("Tasks");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (i, task) in self.tasks.iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut task.completed, "");
                                ui.label(task.name.clone());
                                if ui.button("View More info").clicked() {
                                    // display more info about the task
                                    self.selected_task = Some(i);
                                }
                            });
                        }
                    });
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

                        // Close button to hide the details view
                        if ui.button("Close").clicked() {
                            self.selected_task = None; // Clear the selection to hide details
                        }
                    }

                    ui.separator();

                    // Clear all tasks button
                    if ui.button("Clear All Tasks").clicked() {
                        self.tasks.clear();
                    }
                });
            });
        // level 1 is 10 points, level 2 is 20 points, level 3 is 30 points
        let points_earned = self
            .tasks
            .iter()
            .filter(|task| task.completed)
            .map(|task| match task.priority {
                PriorityLevel::Low => 10,
                PriorityLevel::Medium => 20,
                PriorityLevel::High => 30,
                })
                .sum::<u32>();
        self.points = points_earned;
        let completed_tasks = self.tasks.iter().filter(|task| task.completed).count();
        //check for challenges
        self.check_challenges(); 

        // Create a right side panel for the "Achievements" section.
        egui::SidePanel::right("right_panel")
            .resizable(false)
            .min_width(ctx.available_rect().width() / 2.0) // Ensures the panel fills half the window.
            .max_width(ctx.available_rect().width() / 2.0) // Keeps the width fixed.
            .show(ctx, |ui| {

                // Display the achievements section
                ui.heading("Achievements");
                ui.separator();

                //Display the challenge achievement message
                ui.label(&self.achievement_message);
                ui.separator();
                // Display user-defined goal progress
                ui.label(format!(
                    "Current Tasks: {} / Bronze Goal: {}",
                    completed_tasks, self.bronze_goal
                ));
                ui.label(format!(
                    "Current Tasks: {} / Silver Goal: {}",
                    completed_tasks, self.silver_goal
                ));
                ui.label(format!(
                    "Current Tasks: {} / Gold Goal: {}",
                    completed_tasks, self.gold_goal
                ));

                ui.separator();

                // Add goal setting UI panel
                ui.heading("Set Your Goals");

                // Input fields to set bronze, silver, and gold task goals
                ui.horizontal(|ui| {
                    ui.label("Bronze Goal: ");
                    ui.add(egui::DragValue::new(&mut self.bronze_goal).speed(1).range(1..=100));
                });

                ui.horizontal(|ui| {
                    ui.label("Silver Goal: ");
                    ui.add(egui::DragValue::new(&mut self.silver_goal).speed(1).range(1..=100));
                });

                ui.horizontal(|ui| {
                    ui.label("Gold Goal: ");
                    ui.add(egui::DragValue::new(&mut self.gold_goal).speed(1).range(1..=100));
                });

                ui.separator();

            });

        
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            egui::ScrollArea::vertical().show(ui, |ui| {
                let bronze_points = 50;
                let silver_points = 100;
                let gold_points = 500;

                // this if statement helps to set self.points to u32 for the purpose of saturating_sub functions below
                if self.points >= gold_points {
                } 
                if self.points >= silver_points {
                }
                if self.points >= bronze_points {
                }

                //To display challenge info and message,
                let points_message = format!(
                    "Bronze: {} points required | {} points to Bronze level\n\n\
                     Silver: {} points required | {} points to Silver level\n\n\
                     Gold: {} points required | {} points to Gold level",
                    bronze_points, bronze_points.saturating_sub(self.points),
                    silver_points, silver_points.saturating_sub(self.points),
                    gold_points, gold_points.saturating_sub(self.points)
                );

                // Challenge section
                //ui.heading("Challenges | Points: {}\n", self.points);
                // make a heading that is like this: "Challenges | Points: 100"
                ui.heading(format!("Challenges (Points: {})", self.points));

                //ui.label(format!("Points: {}\n", self.points));

                // Display points message here
                ui.label(&points_message);

                ui.separator();

                ui.heading("Completed Challenges");

                if self.points >= gold_points {
                    //ui.label("Gold level challenge completed!\n");
                    // create a label with a gold color
                    ui.colored_label(egui::Color32::from_rgb(255, 215, 0), "Gold level challenge completed!\n");
                } 
                if self.points >= silver_points {
                    //ui.label("Silver level challenge completed!\n");
                    // create a label with a silver color
                    ui.colored_label(egui::Color32::from_rgb(192, 192, 192), "Silver level challenge completed!\n");
                }
                if self.points >= bronze_points {
                    //ui.label("Bronze level challenge completed!");
                    // create a label with a bronze color
                    ui.colored_label(egui::Color32::from_rgb(205, 127, 50), "Bronze level challenge completed!");
                }

                ui.separator();

            });
        });

    }
}
