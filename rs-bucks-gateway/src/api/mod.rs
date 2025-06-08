pub mod stripe;

use crate::model::Customer;

/// Public wrapper to fetch customers via Stripe
pub async fn fetch_customers() -> Result<Vec<Customer>, Box<dyn std::error::Error>> {
    stripe::fetch_customers().await
}