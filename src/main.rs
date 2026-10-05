use serde::{Deserialize,Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Recursos {
	saludo: String,
    tienes: String,
	pedidos: String,
	enviar: String,
	cancelar: String,
    moneda: String,
    valor: String,
	total: String,
	salir: String
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
        precio: 450.00
    };
    let content =
        fs::read_to_string("es.json")
        .expect("espected a file");

    let content: Recursos =
        serde_json::from_str(&content)
        .expect("failed to deserialize");

    let cantidad_pedidos: f64 =
        daury
        .cantidad_pedidos
        as f64;

    let valor: f64 = content
        .valor
        .parse()
        .expect("couldn't parse int");

    let message = format!("{} {}, {} {} {}",
        content.saludo,
        daury.nombre,
        content.tienes,
        daury.cantidad_pedidos,
        content.pedidos);

    println!("{message}");

    let precio = format!("{}{}",
        content.moneda,
        cantidad_pedidos * valor * daury.precio);
    println!("{precio}");
}
