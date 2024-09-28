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
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

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

                //egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        // Create a left side panel for the tasks and other controls.
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .min_width(ctx.available_rect().width() / 2.0) // Ensures the panel fills half the window.
            .max_width(ctx.available_rect().width() / 2.0) // Keeps the width fixed.
            .show(ctx, |ui| {
                ui.heading("Stats");
                ui.label(format!("Points: {}", self.points));
                ui.separator();

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

                /*
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.new_task_completed, "Completed");
                });
                */

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
                            //completed: self.new_task_completed,
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
                                self.selected_task = Some(i);
                            }
                        });
                    }
                });

                ui.separator();

                // Clear all tasks button
                if ui.button("Clear All Tasks").clicked() {
                    self.tasks.clear();
                }
            });

        // Create a right side panel for the "Achievements" section.
        egui::SidePanel::right("right_panel")
            .resizable(false)
            .min_width(ctx.available_rect().width() / 2.0) // Ensures the panel fills half the window.
            .max_width(ctx.available_rect().width() / 2.0) // Keeps the width fixed.
            .show(ctx, |ui| {
                ui.heading("Achievements");
                // Add content related to achievements here, e.g., list of achievements.
                ui.label("Achievement 1: Task completion streak!");
                ui.label("Achievement 2: High priority tasks completed!");
                // Add more achievements as needed
            });

        
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            egui::ScrollArea::vertical().show(ui, |ui| {

                ui.heading("Stats");
                ui.label(format!("Points: {}", self.points));
                ui.separator();

                ui.heading("Add a Task");

                /*
                ui.horizontal(|ui| {
                    ui.label("Write a task: ");
                    ui.text_edit_singleline(&mut self.new_task);
                });
                */

                /*
                ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
                if ui.button("Increment").clicked() {
                    self.value += 1.0;
                }
                */

                /*
                * Add a new task.
                * Fill in information needed for a task (Task name, description, due date, priority, and completed). Add the task to the list of tasks.
                * Clear the fields for the next task.
                * Use a button click to add the task.
                */
                // update the task with input from the user
                ui.horizontal(|ui| {
                    ui.label("Task Name: ");
                    ui.text_edit_singleline(&mut self.new_task_name);
                });
                ui.separator();

                // Display the list of tasks
                ui.heading("Tasks");
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

                ui.separator();
                //clear all tasks
                if ui.button("Clear All Tasks").clicked() {
                    self.tasks.clear();
                }


                ui.add(egui::github_link_file!(
                    "https://github.com/devenallen/TaskHero/eframe_template/",
                    "Source code."
                ));

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    egui::warn_if_debug_build(ui);
                });
            });
        });

    }
}

/*
fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
    */
