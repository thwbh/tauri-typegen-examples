use crate::models::*;
use tauri::ipc::Channel;
use tauri::{AppHandle, Emitter};

/// Simple command with no parameters
#[tauri::command]
pub fn greet() -> String {
    "Hello from Tauri!".to_string()
}

/// Command with simple parameters
#[tauri::command]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

/// Command returning a custom struct
#[tauri::command]
pub fn get_user(user_id: i32) -> User {
    User {
        user_id,
        user_name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        is_active: true,
        internal_field: "This field is skipped in serialization".to_string(),
    }
}

/// Command accepting and returning custom structs
#[tauri::command]
pub fn create_user(user: User) -> Result<User, String> {
    if user.user_name.is_empty() {
        return Err("Username cannot be empty".to_string());
    }
    Ok(user)
}

/// Command with optional parameters
#[tauri::command]
pub fn search_products(query: String, limit: Option<i32>) -> Vec<Product> {
    let limit = limit.unwrap_or(10);
    vec![
        Product {
            id: 1,
            name: format!("Product matching '{}'", query),
            price: 29.99,
            in_stock: Some(true),
        },
        Product {
            id: 2,
            name: format!("Another product for '{}'", query),
            price: 49.99,
            in_stock: Some(false),
        },
    ]
    .into_iter()
    .take(limit as usize)
    .collect()
}

/// Command with enum parameters
#[tauri::command]
pub fn update_order_status(order_id: String, status: OrderStatus) -> Result<String, String> {
    Ok(format!("Order {} updated to {:?}", order_id, status))
}

/// Command with complex nested types
#[tauri::command]
pub fn create_order(order: Order) -> Result<Order, String> {
    if order.products.is_empty() {
        return Err("Order must contain at least one product".to_string());
    }
    Ok(order)
}

/// Command with HashMap
#[tauri::command]
pub fn update_settings(settings: Settings) -> Settings {
    settings
}

/// Command that emits events
#[tauri::command]
pub async fn process_task(app: AppHandle, task_id: String) -> Result<String, String> {
    // Emit progress updates
    for i in 1..=5 {
        let progress = (i as f64 / 5.0) * 100.0;
        app.emit(
            "progress-update",
            ProgressUpdate {
                task_id: task_id.clone(),
                progress,
                message: format!("Processing step {}/5", i),
            },
        )
        .map_err(|e| e.to_string())?;

        // Simulate some work
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(format!("Task {} completed", task_id))
}

/// Command with Channel for streaming data
#[tauri::command]
pub async fn stream_logs(channel: Channel<LogEntry>) -> Result<(), String> {
    for i in 1..=10 {
        let log = LogEntry {
            timestamp: chrono::Utc::now().timestamp(),
            level: if i % 3 == 0 {
                "ERROR".to_string()
            } else if i % 2 == 0 {
                "WARN".to_string()
            } else {
                "INFO".to_string()
            },
            message: format!("Log message #{}", i),
        };

        channel.send(log).map_err(|e| e.to_string())?;

        // Simulate log generation
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    Ok(())
}

/// Command with multiple channels
#[tauri::command]
pub async fn monitor_system(
    cpu_channel: Channel<f64>,
    memory_channel: Channel<f64>,
) -> Result<(), String> {
    for i in 1..=5 {
        // Simulate CPU usage
        let cpu = 20.0 + (i as f64 * 10.0);
        cpu_channel.send(cpu).map_err(|e| e.to_string())?;

        // Simulate memory usage
        let memory = 50.0 + (i as f64 * 5.0);
        memory_channel.send(memory).map_err(|e| e.to_string())?;

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    Ok(())
}

/// Command demonstrating various Result types
#[tauri::command]
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
