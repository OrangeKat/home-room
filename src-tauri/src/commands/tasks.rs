use crate::models::{Task, NewTask};
use crate::utils::get_supabase_config;

#[tauri::command]
pub async fn fetch_tasks() -> Result<Vec<Task>, String> {
    let client = reqwest::Client::new();
    let config = get_supabase_config()?;

    println!("Fetching tasks from: {}", config.url);

    let resp = client
        .get(format!("{}/rest/v1/tasks?select=*", config.url))
        .header("apikey", &config.key)
        .header("Authorization", format!("Bearer {}", config.key))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        println!("Supabase Error (Fetch): {} - {}", status, text);
        return Err(format!("Supabase error {}: {}", status, text));
    }

    let tasks = resp.json::<Vec<Task>>().await.map_err(|e| format!("JSON parse error: {}", e))?;
    println!("Fetched {} tasks", tasks.len());
    Ok(tasks)
}

#[tauri::command]
pub async fn add_task(title: String, date: String, time: String, is_urgent: bool) -> Result<(), String> {
    let client = reqwest::Client::new();
    let config = get_supabase_config()?;

    println!("Adding task: {} at {} {}", title, date, time);

    let new_task = NewTask {
        title,
        date,
        time, 
        is_urgent,
        completed: false,
    };

    let resp = client
        .post(format!("{}/rest/v1/tasks", config.url))
        .header("apikey", &config.key)
        .header("Authorization", format!("Bearer {}", config.key))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal")
        .json(&new_task)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        println!("Supabase Error (Add): {} - {}", status, text);
        return Err(format!("Supabase error {}: {}", status, text));
    }

    println!("Task added successfully");
    Ok(())
}

#[tauri::command]
pub async fn delete_task(id: i64) -> Result<(), String> {
    let client = reqwest::Client::new();
    let config = get_supabase_config()?;

    let resp = client
        .delete(format!("{}/rest/v1/tasks?id=eq.{}", config.url, id))
        .header("apikey", &config.key)
        .header("Authorization", format!("Bearer {}", config.key))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Supabase error: {}", resp.status()));
    }

    Ok(())
}

#[tauri::command]
pub async fn toggle_task_completion(id: i64, completed: bool) -> Result<(), String> {
    let client = reqwest::Client::new();
    let config = get_supabase_config()?;

    let body = serde_json::json!({ "is_completed": completed });

    let resp = client
        .patch(format!("{}/rest/v1/tasks?id=eq.{}", config.url, id))
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
