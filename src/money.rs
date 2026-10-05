use format_num::NumberFormat;

pub fn parse_money(quantity: u64, value: &str, price_per_order: f64, symbol: &str)-> String{
    let quantity_of_orders: f64 = quantity as f64;

    let value: f64 = value
        .parse()
        .expect("failed to parse value");

    let price = quantity_of_orders / value * price_per_order;

    let  num = NumberFormat::new();
    let format = format!("{}{}", symbol,  num.format(",.2f", price));
    format
}
