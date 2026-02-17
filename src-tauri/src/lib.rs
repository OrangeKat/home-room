mod commands;
mod models;
mod utils;

use crate::commands::habits::*;
use crate::commands::tasks::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_tasks,
            add_task,
            delete_task,
            toggle_task_completion,
            fetch_habits,
            add_habit,
            delete_habit,
            fetch_habit_completions,
            toggle_habit_completion
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
