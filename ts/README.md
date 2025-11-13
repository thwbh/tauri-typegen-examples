# Tauri-TypeGen Example Project

This example project demonstrates all features of tauri-typegen including:

## Features Demonstrated

### 1. **Serde Attributes**

#### Struct-level `rename_all`
```rust
// models.rs:6-14
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: i32,      // becomes "userId" in TypeScript
    pub user_name: String, // becomes "userName" in TypeScript
    pub is_active: bool,   // becomes "isActive" in TypeScript
}
```

#### Field-level `rename`
```rust
// models.rs:16-22
#[derive(Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "productId")]
    pub id: i32,
    pub name: String,
}
```

#### Enum `rename_all`
```rust
// models.rs:24-32
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Pending,    // becomes "PENDING"
    Processing, // becomes "PROCESSING"
}
```

#### Enum variant-level `rename`
```rust
// models.rs:34-42
#[derive(Serialize, Deserialize)]
pub enum PaymentMethod {
    #[serde(rename = "credit_card")]
    CreditCard,
}
```

#### Skip fields with `#[serde(skip)]`
```rust
// models.rs:7-14
pub struct User {
    pub user_id: i32,
    #[serde(skip)]  // This field won't appear in TypeScript
    pub internal_field: String,
}
```

### 2. **Command Types**

#### Simple Commands
```rust
// commands.rs:5-8
#[tauri::command]
pub fn greet() -> String
```

#### Commands with Parameters
```rust
// commands.rs:10-13
#[tauri::command]
pub fn add_numbers(a: i32, b: i32) -> i32
```

#### Commands with Custom Structs
```rust
// commands.rs:15-26
#[tauri::command]
pub fn get_user(user_id: i32) -> User

#[tauri::command]
pub fn create_user(user: User) -> Result<User, String>
```

#### Commands with Optional Parameters
```rust
// commands.rs:28-44
#[tauri::command]
pub fn search_products(query: String, limit: Option<i32>) -> Vec<Product>
```

#### Commands with Enums
```rust
// commands.rs:46-49
#[tauri::command]
pub fn update_order_status(order_id: String, status: OrderStatus) -> Result<String, String>
```

#### Commands with Nested Types
```rust
// commands.rs:51-57
#[tauri::command]
pub fn create_order(order: Order) -> Result<Order, String>
```

#### Commands with HashMap
```rust
// commands.rs:59-62
#[tauri::command]
pub fn update_settings(settings: Settings) -> Settings
```

### 3. **Events**

Commands that emit events using `app.emit()`:

```rust
// commands.rs:64-82
#[tauri::command]
pub async fn process_task(app: AppHandle, task_id: String) -> Result<String, String> {
    app.emit("progress-update", ProgressUpdate {
        task_id: task_id.clone(),
        progress: 50.0,
        message: "Processing...".to_string(),
    })?;
    Ok(format!("Task {} completed", task_id))
}
```

### 4. **Channels (Streaming)**

#### Single Channel
```rust
// commands.rs:84-100
#[tauri::command]
pub async fn stream_logs(channel: Channel<LogEntry>) -> Result<(), String> {
    channel.send(log_entry)?;
    Ok(())
}
```

#### Multiple Channels
```rust
// commands.rs:102-120
#[tauri::command]
pub async fn monitor_system(
    cpu_channel: Channel<f64>,
    memory_channel: Channel<f64>,
) -> Result<(), String>
```

### 5. **Error Handling**

```rust
// commands.rs:122-129
#[tauri::command]
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

## Running the Example

1. **Generate TypeScript bindings:**
   ```bash
   cd src-tauri
   cargo tauri-typegen tauri-typegen generate
   ```

2. **Start the development server:**
   ```bash
   cd ..
   npm run dev
   ```

3. **Run Tauri:**
   ```bash
   cd src-tauri
   cargo tauri dev
   ```

   Or from the root:
   ```bash
   cargo tauri dev
   ```

## Generated Files

After running tauri-typegen, you'll find these files in `src/generated/`:

- **`types.ts`** - TypeScript interfaces for all Rust structs
- **`commands.ts`** - Typed command functions
- **`events.ts`** - Typed event listeners
- **`index.ts`** - Barrel export

## Using Generated Bindings

```typescript
import { createUser, streamLogs, processTask } from './generated';

// Strongly-typed command calls
const user = await createUser({
  userId: 1,
  userName: "John",
  email: "john@example.com",
  isActive: true
});

// Event listeners with typed payloads
import { onProgressUpdate } from './generated';
await onProgressUpdate((payload) => {
  console.log(`Task ${payload.taskId}: ${payload.progress}%`);
});

// Channel/streaming with typed data
import { Channel } from '@tauri-apps/api/core';
const logChannel = new Channel();
logChannel.onmessage = (log) => {
  console.log(`[${log.level}] ${log.message}`);
};
await streamLogs(logChannel);
```

## Complex Types Coverage

The example includes:
- ✅ Primitive types (i32, f64, String, bool)
- ✅ Optional types (Option<T>)
- ✅ Collections (Vec<T>, HashMap<K, V>)
- ✅ Custom structs
- ✅ Nested structs
- ✅ Enums (unit variants)
- ✅ Result types
- ✅ Async commands
- ✅ Events
- ✅ Channels
- ✅ Serde rename attributes
- ✅ Serde skip attributes
