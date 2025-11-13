use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

/// Example user struct with serde rename_all and validations
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[validate(range(min = 1, message = "User ID must be positive"))]
    pub user_id: i32,
    #[validate(length(min = 3, max = 50, message = "Username must be 3-50 characters"))]
    pub user_name: String,
    #[validate(email(message = "Must be a valid email address"))]
    pub email: String,
    pub is_active: bool,
    #[serde(skip)]
    pub internal_field: String,
}

/// Example with field-level rename and validations
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Product {
    #[serde(rename = "productId")]
    #[validate(range(min = 1, message = "Product ID must be positive"))]
    pub id: i32,
    #[validate(length(min = 1, max = 100, message = "Product name must be 1-100 characters"))]
    pub name: String,
    #[validate(range(min = 0.01, message = "Price must be greater than 0"))]
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

/// Example with nested structs and validations
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[validate(length(min = 1, message = "Order ID cannot be empty"))]
    pub order_id: String,
    pub user: User,
    #[validate(length(min = 1, message = "Order must contain at least one product"))]
    pub products: Vec<Product>,
    pub status: OrderStatus,
    pub payment_method: PaymentMethod,
    #[validate(range(min = 0.01, message = "Total amount must be positive"))]
    pub total_amount: f64,
}

/// Example with complex types and validations
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[validate(length(min = 1, max = 100, message = "App name must be 1-100 characters"))]
    pub app_name: String,
    pub version: String,
    pub features: HashMap<String, bool>,
    pub theme: Option<String>,
}

lazy_static::lazy_static! {
    static ref VERSION_REGEX: regex::Regex = regex::Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
}

/// Example for event payload with validations
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProgressUpdate {
    #[validate(length(min = 1, message = "Task ID cannot be empty"))]
    pub task_id: String,
    #[validate(range(min = 0.0, max = 100.0, message = "Progress must be between 0 and 100"))]
    pub progress: f64,
    pub message: String,
}

/// Example for streaming data with validations
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub timestamp: i64,
    #[validate(length(min = 1, message = "Log level cannot be empty"))]
    pub level: String,
    #[validate(length(min = 1, max = 500, message = "Log message must be 1-500 characters"))]
    pub message: String,
}
