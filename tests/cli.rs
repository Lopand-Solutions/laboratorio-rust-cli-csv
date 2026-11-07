// Pruebas de la CLI de punta a punta: corren el binario como lo haría una persona.
use std::process::Command;

fn correr(args: &[&str]) -> (i32, String, String) {
    let salida = Command::new(env!("CARGO_BIN_EXE_resumen-csv"))
        .args(args)
        .output()
        .expect("no pude correr el binario");
    (
        salida.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&salida.stdout).into_owned(),
        String::from_utf8_lossy(&salida.stderr).into_owned(),
    )
}

#[test]
fn resume_el_csv_de_ejemplo() {
    let (codigo, salida, _) = correr(&["datos/partidas.csv"]);
    assert_eq!(codigo, 0);
    assert!(
        salida.contains("arcade                3      1250      416.7"),
        "{salida}"
    );
    assert!(
        salida.contains("historia              3      4600     1533.3"),
        "{salida}"
    );
}

#[test]
fn sin_argumentos_explica_el_uso() {
    let (codigo, _, error) = correr(&[]);
    assert_eq!(codigo, 2);
    assert!(error.contains("uso: resumen-csv"));
}

#[test]
fn una_fila_rota_sale_con_1_y_dice_la_linea() {
    let (codigo, _, error) = correr(&["datos/rota.csv"]);
    assert_eq!(codigo, 1);
    assert!(error.contains("line: 3"), "{error}");
}
