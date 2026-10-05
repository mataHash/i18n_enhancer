use serde::{Deserialize,Serialize};
use std::fs;
use chrono::{DateTime,Local};
use format_num::NumberFormat;

#[derive(Serialize, Deserialize, Debug)]
struct Language {
	saludo: String,
    tienes: String,
	pedidos: String,
	enviar: String,
	cancelar: String,
	total: String,
	salir: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Currency {
    moneda: String,
    valor: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DateFormat{
    format: String,
}
struct Persona {
    nombre: String,
    cantidad_pedidos: u64,
    precio: f64,
}
fn main() {
// fecha = dia + "/" + mes + "/" + anio
// boton.texto = "Enviar" 
    let daury = Persona {
        nombre: String::from("Daury"),
        cantidad_pedidos: 98,
        precio: 450f64
    };
    let language =
        fs::read_to_string("formats/en/language.json")
        .expect("espected a file on language");
    let currency = fs::read_to_string("formats/en/region/en-US/currency.json")
        .expect("espected a file on currency");
    let format = fs::read_to_string("formats/en/region/en-US/date.json")
        .expect("espected a file on format");

    let language: Language =
        serde_json::from_str(&language)
        .expect("failed to deserialize");
    let currency: Currency =
        serde_json::from_str(&currency)
        .expect("failed to deserialize");

    let format: DateFormat =
        serde_json::from_str(&format)
        .expect("failed to deserialize");

    let cantidad_pedidos: f64 =
        daury
        .cantidad_pedidos
        as f64;

    let valor: f64 = currency
        .valor
        .parse()
        .expect("couldn't parse int");

    let message = format!("{} {}, {} {} {}",
        language.saludo,
        daury.nombre,
        language.tienes,
        daury.cantidad_pedidos,
        language.pedidos);

    println!("{message}");

    let precio = cantidad_pedidos * valor * daury.precio;

    let  num = NumberFormat::new();
    println!("{}{}",currency.moneda,  num.format(",.2f", precio));

    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format(&format.format);
    println!("{custom_format}");
}
