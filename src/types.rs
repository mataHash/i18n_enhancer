use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    pub greeting: String,
    pub have: String,
    pub order: String,
    pub send: String,
    pub cancel: String,
    pub total: String,
    pub exit: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub symbol: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateFormat {
    pub format: String,
}
pub struct Persona {
    pub name: String,
    pub quantity_of_orders: u64,
    pub price_per_order: f64,
}
