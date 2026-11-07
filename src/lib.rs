//! Lee un CSV de partidas y resume los puntos por modo de juego.
use std::collections::BTreeMap;
use std::fmt::Write;

use serde::Deserialize;
use thiserror::Error;

/// Una fila del CSV. Los nombres de los campos son los encabezados del archivo.
#[derive(Debug, Deserialize)]
pub struct Partida {
    pub jugador: String,
    pub modo: String,
    pub puntos: u32,
    pub segundos: u32,
}

/// Conteo y suma de un modo; el promedio se calcula al pedirlo.
#[derive(Debug, Default, PartialEq)]
pub struct Estadistica {
    pub conteo: u32,
    pub suma: u64,
}

impl Estadistica {
    pub fn promedio(&self) -> f64 {
        if self.conteo == 0 {
            return 0.0;
        }
        self.suma as f64 / self.conteo as f64
    }
}

#[derive(Debug, Error)]
pub enum ErrorResumen {
    #[error("no pude leer {ruta}")]
    Lectura {
        ruta: String,
        #[source]
        causa: std::io::Error,
    },

    #[error("el CSV tiene una fila inválida")]
    Csv(#[from] csv::Error),

    #[error("el CSV no tiene partidas")]
    SinPartidas,
}

/// Interpreta el texto de un CSV de partidas.
pub fn leer_texto(texto: &str) -> Result<Vec<Partida>, ErrorResumen> {
    let mut lector = csv::Reader::from_reader(texto.as_bytes());
    let mut partidas = Vec::new();
    for fila in lector.deserialize() {
        let partida: Partida = fila?;
        partidas.push(partida);
    }
    if partidas.is_empty() {
        return Err(ErrorResumen::SinPartidas);
    }
    Ok(partidas)
}

/// Lee un archivo CSV de partidas.
pub fn leer_archivo(ruta: &str) -> Result<Vec<Partida>, ErrorResumen> {
    let texto = std::fs::read_to_string(ruta).map_err(|causa| ErrorResumen::Lectura {
        ruta: ruta.to_string(),
        causa,
    })?;
    leer_texto(&texto)
}

/// Agrupa por modo. BTreeMap deja los modos en orden alfabético, así la salida no cambia entre corridas.
pub fn resumir(partidas: &[Partida]) -> BTreeMap<String, Estadistica> {
    let mut resumen: BTreeMap<String, Estadistica> = BTreeMap::new();
    for p in partidas {
        let e = resumen.entry(p.modo.clone()).or_default();
        e.conteo += 1;
        e.suma += u64::from(p.puntos);
    }
    resumen
}

/// La tabla que imprime la CLI.
pub fn formatear(resumen: &BTreeMap<String, Estadistica>) -> String {
    let mut salida = String::new();
    writeln!(
        salida,
        "{:<14} {:>8} {:>9} {:>10}",
        "modo", "partidas", "puntos", "promedio"
    )
    .unwrap();
    for (modo, e) in resumen {
        writeln!(
            salida,
            "{:<14} {:>8} {:>9} {:>10.1}",
            modo,
            e.conteo,
            e.suma,
            e.promedio()
        )
        .unwrap();
    }
    salida
}

#[cfg(test)]
mod pruebas {
    use super::*;

    const CSV: &str = "jugador,modo,puntos,segundos\n\
                       Kira,arcade,3,10\n\
                       Ale,arcade,0,12\n\
                       Kira,historia,1200,300\n";

    #[test]
    fn resume_por_modo() {
        let partidas = leer_texto(CSV).unwrap();
        let resumen = resumir(&partidas);
        assert_eq!(resumen.len(), 2);
        assert_eq!(resumen["arcade"], Estadistica { conteo: 2, suma: 3 });
        assert_eq!(
            resumen["historia"],
            Estadistica {
                conteo: 1,
                suma: 1200
            }
        );
    }

    #[test]
    fn el_promedio_no_trunca() {
        let e = Estadistica { conteo: 2, suma: 3 };
        assert_eq!(e.promedio(), 1.5);
        assert_eq!(Estadistica::default().promedio(), 0.0);
    }

    #[test]
    fn una_fila_mala_es_un_error_con_su_linea() {
        let texto = "jugador,modo,puntos,segundos\nKira,arcade,diez,10\n";
        let error = leer_texto(texto).unwrap_err();
        let ErrorResumen::Csv(causa) = &error else {
            panic!("se esperaba un error de CSV y llegó {error:?}");
        };
        let posicion = causa.position().expect("el error trae su posición");
        assert_eq!(posicion.line(), 2);
    }

    #[test]
    fn sin_filas_es_un_error() {
        let error = leer_texto("jugador,modo,puntos,segundos\n").unwrap_err();
        assert!(matches!(error, ErrorResumen::SinPartidas));
    }

    #[test]
    fn un_archivo_que_no_existe_dice_cual() {
        let error = leer_archivo("no-existe.csv").unwrap_err();
        assert_eq!(error.to_string(), "no pude leer no-existe.csv");
    }
}
