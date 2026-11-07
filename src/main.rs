use std::error::Error;
use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(ruta) = std::env::args().nth(1) else {
        eprintln!("uso: resumen-csv <archivo.csv>");
        return ExitCode::from(2);
    };

    match resumen_csv::leer_archivo(&ruta) {
        Ok(partidas) => {
            let resumen = resumen_csv::resumir(&partidas);
            print!("{}", resumen_csv::formatear(&resumen));
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            // Recorre la cadena de causas: el error de CSV dice la línea exacta.
            let mut causa = error.source();
            while let Some(c) = causa {
                eprintln!("  causa: {c}");
                causa = c.source();
            }
            ExitCode::FAILURE
        }
    }
}
