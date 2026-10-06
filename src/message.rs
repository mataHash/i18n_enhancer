pub fn parse_message(
    greeting: &str,
    name: &str,
    verb: &str,
    quantity: &str,
    order: &str,
) -> String {
    let quantity: u64 = quantity.parse().expect("failed to parse");
    let order = if quantity > 1 {
        format!("{}s", order)
    } else {
        format!("{}", order)
    };
    let message = format!("{}, {}. {} {} {}.", greeting, name, verb, quantity, order,);
    message
}
