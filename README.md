# Solucion para CaribeExport

Demo en Rust de internacionalización (i18n) y localización (l10n). Carga idioma, moneda y formato de fecha desde JSON en `.formats/` según lengua/región, y genera por consola: saludo con pluralización, total monetario convertido y fecha actual formateada.

## Estructura de `src/`

```text
src/
├── main.rs           # Punto de entrada: crea la Persona, carga/deserializa y orquesta la salida
├── types.rs          # Structs Language, Currency, DateFormat y Persona
├── file_io.rs        # Carga de JSONs desde .formats/<lang>/ y .formats/<lang>/region/<reg>/
├── serialization.rs  # Deserialización con serde_json a los tipos de types.rs
├── message.rs        # Arma el saludo con pluralización simple (order/orders)
├── money.rs          # Convierte y formatea el total (quantity / value * price)
└── dates.rs          # Formatea la fecha/hora local con chrono
```

## Estructura de `.formats/`

```text
.formats/
├── en/
│   ├── language.json            # Textos en inglés: saludo, have, order, send, cancel, total, exit
│   └── region/
│       └── en-us/
│           ├── currency.json    # Moneda: símbolo USD$, tasa 58.60
│           └── date.json        # Fecha: formato %m-%d-%Y
├── es/
│   ├── language.json            # Textos en español: saludo, have, order, send, cancel, total, exit
│   └── region/
│       ├── es-do/
│       │   ├── currency.json    # Moneda: símbolo RD$, tasa 1
│       │   └── date.json        # Fecha: formato %d-%m-%Y
│       ├── es-es/
│       │   ├── currency.json    # Moneda: símbolo €, tasa 62.70
│       │   └── date.json        # Fecha: formato %d-%m-%Y
│       └── es-mx/
│           ├── currency.json    # Moneda: símbolo MXN, tasa 3.29
│           └── date.json        # Fecha: formato %d-%m-%Y
└── fr/
    ├── language.json            # Textos en francés: saludo, have, order, send, cancel, total, exit
    └── region/
        └── fr-idf/
            ├── currency.json    # Moneda: símbolo €, tasa 62.70
            └── date.json        # Fecha: formato %d-%m-%Y
```

| Archivo | Qué hace |
|---|---|
| `src/main.rs:8` | Flujo principal: `load_*` → `ser_*` → `parse_*` → `println!`. Ejemplo fijo con `Persona { daury, 32, 100.0 }` y locale `en` / región `en-us`. |
| `src/types.rs:4` | `Language` (greeting, have, order, send, cancel, total, exit), `Currency` (symbol, value), `DateFormat` (format), `Persona` (name, quantity_of_orders, price_per_order). |
| `src/file_io.rs:8` | `load_language`, `load_currency`, `load_date_format` leen `.formats/<lang>/language.json` y `.formats/<lang>/region/<reg>/{currency,date}.json`. |
| `src/serialization.rs:2` | `ser_lang`, `ser_curr`, `ser_format` convierten el JSON en texto a structs tipados. |
| `src/message.rs:2` | `parse_message` compone `"greeting, name. have N order(s)."` (agrega `s` si `N > 1`). |
| `src/money.rs:3` | `parse_money` calcula `quantity / value * price_per_order` y lo formatea como `{symbol}{precio}` con 2 decimales. |
| `src/dates.rs:2` | `parse_dates` devuelve `Local::now()` con el patrón `chrono` del JSON (ej. `%m-%d-%Y`). |

Locales disponibles en `.formats/`: `en` (`en-us`), `es` (`es-do`, `es-es`, `es-mx`), `fr` (`fr-idf`).

Salida esperada de ejemplo:

```text
Saludo, NombreCliente. Catidad de pedidos (Ej: hello, daury. you have 32 orders).
Moneda (Ej:USD$54.61)
Fecha (10-06-2026)
```

## Cómo correrlo con cargo

Requisito: tener [Rust + Cargo](https://rustup.rs/) instalado.

```bash
cargo run
```

Otros comandos útiles:

```bash
cargo build   # compila el binario en target/debug/
cargo check   # verifica que compila sin generar binario
```

---

# Solution for CaribeExport (English version)

Rust demo of internationalization (i18n) and localization (l10n). It loads language, currency, and date format from JSON in `.formats/` according to language/region, and prints to console: greeting with pluralization, converted monetary total, and formatted current date.

## `src/` Structure

```text
src/
├── main.rs           # Entry point: creates the Persona, loads/deserializes and orchestrates the output
├── types.rs          # Structs Language, Currency, DateFormat and Persona
├── file_io.rs        # Loads JSONs from .formats/<lang>/ and .formats/<lang>/region/<reg>/
├── serialization.rs  # Deserialization with serde_json into the types from types.rs
├── message.rs        # Builds the greeting with simple pluralization (order/orders)
├── money.rs          # Converts and formats the total (quantity / value * price)
└── dates.rs          # Formats the local date/time with chrono
```

## `.formats/` Structure

```text
.formats/
├── en/
│   ├── language.json            # English texts: greeting, have, order, send, cancel, total, exit
│   └── region/
│       └── en-us/
│           ├── currency.json    # Currency: USD$ symbol, 58.60 rate
│           └── date.json        # Date: %m-%d-%Y format
├── es/
│   ├── language.json            # Spanish texts: greeting, have, order, send, cancel, total, exit
│   └── region/
│       ├── es-do/
│       │   ├── currency.json    # Currency: RD$ symbol, 1 rate
│       │   └── date.json        # Date: %d-%m-%Y format
│       ├── es-es/
│       │   ├── currency.json    # Currency: € symbol, 62.70 rate
│       │   └── date.json        # Date: %d-%m-%Y format
│       └── es-mx/
│           ├── currency.json    # Currency: MXN symbol, 3.29 rate
│           └── date.json        # Date: %d-%m-%Y format
└── fr/
    ├── language.json            # French texts: greeting, have, order, send, cancel, total, exit
    └── region/
        └── fr-idf/
            ├── currency.json    # Currency: € symbol, 62.70 rate
            └── date.json        # Date: %d-%m-%Y format
```

| File | What it does |
|---|---|
| `src/main.rs:8` | Main flow: `load_*` → `ser_*` → `parse_*` → `println!`. Fixed example with `Persona { daury, 32, 100.0 }` and locale `en` / region `en-us`. |
| `src/types.rs:4` | `Language` (greeting, have, order, send, cancel, total, exit), `Currency` (symbol, value), `DateFormat` (format), `Persona` (name, quantity_of_orders, price_per_order). |
| `src/file_io.rs:8` | `load_language`, `load_currency`, `load_date_format` read `.formats/<lang>/language.json` and `.formats/<lang>/region/<reg>/{currency,date}.json`. |
| `src/serialization.rs:2` | `ser_lang`, `ser_curr`, `ser_format` convert the JSON text into typed structs. |
| `src/message.rs:2` | `parse_message` builds `"greeting, name. have N order(s)."` (adds `s` if `N > 1`). |
| `src/money.rs:3` | `parse_money` calculates `quantity / value * price_per_order` and formats it as `{symbol}{price}` with 2 decimals. |
| `src/dates.rs:2` | `parse_dates` returns `Local::now()` with the `chrono` pattern from the JSON (e.g. `%m-%d-%Y`). |

Available locales in `.formats/`: `en` (`en-us`), `es` (`es-do`, `es-es`, `es-mx`), `fr` (`fr-idf`).

Example expected output:

```text
Greeting, ClientName. Order count (E.g.: hello, daury. you have 32 orders).
Currency (E.g.: USD$54.61)
Date (10-06-2026)
```

## How to run it with cargo

Requirement: have [Rust + Cargo](https://rustup.rs/) installed.

```bash
cargo run
```

Other useful commands:

```bash
cargo build   # compiles the binary into target/debug/
cargo check   # checks that it compiles without generating a binary
```
