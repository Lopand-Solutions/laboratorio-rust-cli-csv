# Laboratorio: una CLI que resume un CSV

Repositorio del laboratorio de [Lopand Academy](https://academy.lopandsolutions.com/tutoriales/laboratorio-rust-cli-csv).
Una herramienta de línea de comandos en Rust que lee un CSV de partidas de un juego y resume, por modo de juego,
cuántas partidas hubo, la suma de puntos y el promedio. Los errores (archivo que no existe, fila inválida, CSV vacío)
salen con un mensaje claro y un código de salida distinto de cero. Con pruebas y el reto «rómpelo y arréglalo».

## Ramas

- **`inicio`**: las dependencias, los datos de ejemplo, las pruebas y el comprobador. Las funciones tienen `todo!()`:
  desde aquí escribes el código de la lección.
- **`main`**: la solución, tal como está en la lección.

```bash
git clone -b inicio https://github.com/Lopand-Solutions/laboratorio-rust-cli-csv.git resumen-csv
cd resumen-csv
cargo build
```

## Córrelo

```bash
cargo test
cargo run -- datos/partidas.csv
```

## Cómo saber si quedó

```bash
./tool/comprobar_laboratorio.sh
```

Corre tus pruebas, `cargo clippy -- -D warnings` y `cargo fmt --check`, y rompe tu código a propósito en una copia
(los tres retos de la lección) para ver que tus pruebas lo detectan. No toca tus archivos.

## Requisitos

Rust 1.85 o superior (edición 2024), con `clippy` y `rustfmt`. Las versiones de las dependencias están fijas en
`Cargo.lock`: son las que probamos.

Código libre para usar y adaptar (licencia MIT).
