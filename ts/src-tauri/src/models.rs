use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Example user struct with serde rename_all
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub email: String,
    pub is_active: bool,
    #[serde(skip)]
    pub internal_field: String,
}

/// Example with field-level rename
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "productId")]
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub in_stock: Option<bool>,
}

/// Example enum with rename_all
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

/// Example enum with variant-level rename
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    #[serde(rename = "credit_card")]
    CreditCard,
    #[serde(rename = "paypal")]
    PayPal,
    #[serde(rename = "bank_transfer")]
    BankTransfer,
}

/// Example with nested structs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub order_id: String,
    pub user: User,
    pub products: Vec<Product>,
    pub status: OrderStatus,
    pub payment_method: PaymentMethod,
    pub total_amount: f64,
}

/// Example with complex types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub app_name: String,
    pub version: String,
    pub features: HashMap<String, bool>,
    pub theme: Option<String>,
}

/// Example for event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressUpdate {
    pub task_id: String,
    pub progress: f64,
    pub message: String,
}

/// Example for streaming data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub timestamp: i64,
    pub level: String,
    pub message: String,
}
