// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    dotenv::dotenv().ok(); // Load environment variables from .env if present
    home_room_lib::run()
}
