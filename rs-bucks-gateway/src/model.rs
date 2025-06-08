use serde::Deserialize;

/// Represents a Stripe customer entity.
#[derive(Debug, Deserialize)]
pub struct Customer {
    pub id: String,
    pub email: Option<String>,
    pub description: Option<String>,
}

/// Represents the response format from Stripe's `/v1/customers` endpoint.
#[derive(Debug, Deserialize)]
pub struct CustomerList {
    pub data: Vec<Customer>,
}
