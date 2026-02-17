use serde::Deserialize;
use crate::models::{Habit, NewHabit};
use crate::utils::get_supabase_config;

#[tauri::command]
pub async fn fetch_habits() -> Result<Vec<Habit>, String> {
    let client = reqwest::Client::new();
    let config = get_supabase_config()?;

    let resp = client
        .get(format!("{}/rest/v1/habits?select=*&is_deleted=eq.false", config.url))
        .header("apikey", &config.key)
        .header("Authorization", format!("Bearer {}", config.key))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Supabase error: {}", resp.status()));
    }

    let habits = resp.json::<Vec<Habit>>().await.map_err(|e| format!("JSON parse error: {}", e))?;
    Ok(habits)
}

#[tauri::command]
pub async fn add_habit(title: String) -> Result<(), String> {
    let client = reqwest::Client::new();
    let config = get_supabase_config()?;

    let new_habit = NewHabit { title };

    let resp = client
        .post(format!("{}/rest/v1/habits", config.url))
        .header("apikey", &config.key)
        .header("Authorization", format!("Bearer {}", config.key))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal")
        .json(&new_habit)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Supabase error: {}", resp.status()));
    }

    Ok(())
}

#[tauri::command]
pub async fn delete_habit(id: i64) -> Result<(), String> {
    let client = reqwest::Client::new();
    let config = get_supabase_config()?;

    let body = serde_json::json!({ "is_deleted": true });

    let resp = client
        .patch(format!("{}/rest/v1/habits?id=eq.{}", config.url, id))
        .header("apikey", &config.key)
        .header("Authorization", format!("Bearer {}", config.key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Supabase error: {}", resp.status()));
    }

    Ok(())
}

#[tauri::command]
pub async fn fetch_habit_completions(date: String) -> Result<Vec<i64>, String> {
    let client = reqwest::Client::new();
    let config = get_supabase_config()?;

    let resp = client
        .get(format!("{}/rest/v1/habit_completions?select=habit_id&completed_date=eq.{}", config.url, date))
        .header("apikey", &config.key)
        .header("Authorization", format!("Bearer {}", config.key))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Supabase error: {}", resp.status()));
    }

    #[derive(Deserialize)]
    struct Completion {
        habit_id: i64,
    }

    let completions = resp.json::<Vec<Completion>>().await.map_err(|e| format!("JSON parse error: {}", e))?;
    let ids = completions.into_iter().map(|c| c.habit_id).collect();
    Ok(ids)
}

#[tauri::command]
pub async fn toggle_habit_completion(habit_id: i64, date: String, completed: bool) -> Result<(), String> {
    let client = reqwest::Client::new();
    let config = get_supabase_config()?;

    let resp = if completed {
        let body = serde_json::json!({
            "habit_id": habit_id,
            "completed_date": date
        });
        
        client
            .post(format!("{}/rest/v1/habit_completions", config.url))
            .header("apikey", &config.key)
            .header("Authorization", format!("Bearer {}", config.key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=minimal")
            .json(&body)
            .send()
            .await
    } else {
        client
            .delete(format!("{}/rest/v1/habit_completions?habit_id=eq.{}&completed_date=eq.{}", config.url, habit_id, date))
            .header("apikey", &config.key)
            .header("Authorization", format!("Bearer {}", config.key))
            .send()
            .await
    };

    let resp = resp.map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Supabase error: {}", resp.status()));
    }

    Ok(())
}
