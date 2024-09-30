# Overview of TaskHero project

TaskHero is a gamified task management application built in Rust. It helps users track and manage tasks with an achievement-based system that rewards productivity. The project is developed by the following group:

1. Deven Allen
2. Mitchell Casleton
3. Jagadish Kumar Hiremath

# Project architecture

This project is based on the eframe template provided by the following github repo: https://github.com/emilk/eframe_template.

# Crate dependencies

This project inherits the same dependencies from the eframe template (egui and eframe).

# Directory structure

In the TaskHero directory, we have several subdirectories. The one where we do the most work in under src. In src/app.rs, we configure the GUI and collect and store data.

The 'assets' directory stores various images and icons that are available for us to use.

# Data structures, modules, and functions

The eframe natively stores data for us between sessions using the 'save' method. We had to add the ability to derive both the TemplateApp and Task structs to allow this to happen. Through this, the user's local data is automatically stored when the app is closed and reloaded when the app is oppened.

Our TemplateApp struct stores many different things, the main one being a vector of Tasks. The Task struct has 5 values within it: name (String), description (String), due date (String), priority (PriorityLevel), and completed (bool). PriorityLevel is a struct we defined consisting of low, meduim, and high levels to describe the importance of that task.

# Usage Examples

To run, clone the repository and go into the TaskHero directory. Run 'cargo build' and then 'cargo run' to launch the app.

The left section focuses on the management of tasks. Create Tasks in the upper left corner that will be shown beneath. Tasks are editable by clicking on the "View More Info" button and then clicking the "Edit" button. To complete a task, check the box next to it. To clear all tasks, click the "Clear all tasks" button (this action is irreversible).

The middle section shows the challenges that you are close to or have completed.

The right section shows your acheivements and allows you to set goals for yourself.

# eframe template

[![dependency status](https://deps.rs/repo/github/emilk/eframe_template/status.svg)](https://deps.rs/repo/github/emilk/eframe_template)
[![Build Status](https://github.com/emilk/eframe_template/workflows/CI/badge.svg)](https://github.com/emilk/eframe_template/actions?workflow=CI)

This is a template repo for [eframe](https://github.com/emilk/egui/tree/master/crates/eframe), a framework for writing apps using [egui](https://github.com/emilk/egui/).



### Testing locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

## Info about Updating egui

As of 2023, egui is in active development with frequent releases with breaking changes. [eframe_template](https://github.com/emilk/eframe_template/) will be updated in lock-step to always use the latest version of egui.

When updating `egui` and `eframe` it is recommended you do so one version at the time, and read about the changes in [the egui changelog](https://github.com/emilk/egui/blob/master/CHANGELOG.md) and [eframe changelog](https://github.com/emilk/egui/blob/master/crates/eframe/CHANGELOG.md).
