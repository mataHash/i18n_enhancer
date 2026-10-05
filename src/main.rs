mod types;
mod file_io;
mod serialization;
mod money;
mod dates;
mod message;

fn main() {
    let daury = types::Persona{
        name: String::from("daury"),
        quantity_of_orders: 32,
        price_per_order: 100f64
    };
    let language = file_io::load_language("en");
    let currency = file_io::load_currency("en-us");
    let format = file_io::load_date_format("en-us");

    let language = serialization::ser_lang(&language);
    let currency = serialization::ser_curr(&currency);
    let format = serialization::ser_format(&format);
    
    let message = message::parse_message(
        &language.greeting,
        &daury.name,
        &language.have,
        &daury.quantity_of_orders.to_string(),
        &language.order
    );
    let money = money::parse_money(
        daury.quantity_of_orders,
        &currency.value,
        daury.price_per_order,
        &currency.symbol
    );
    let format = dates::parse_dates(&format.format);
    println!("{message}");
    println!("{money}");
    println!("{format}");


}
