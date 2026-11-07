#!/usr/bin/env bash
# Comprueba el laboratorio «Una CLI que resume un CSV». Uso: ./tool/comprobar_laboratorio.sh
# No necesita red (si ya bajaste las dependencias con cargo build): corre tus pruebas, clippy y rustfmt, y pone a
# prueba tus pruebas rompiendo tu código a propósito en una copia. No toca tus archivos.
set -uo pipefail
cd "$(dirname "$0")/.."
RAIZ=$(pwd)
# Las copias rotas compilan en una carpeta aparte de target/ para no ensuciar tu compilación.
DESTINO="$RAIZ/target/comprobador"
bien=0
mal=0

check() { # check <0 si salió bien> <texto si bien> <texto si mal>
  if [ "$1" -eq 0 ]; then
    echo "  ✓ $2"
    bien=$((bien + 1))
  else
    echo "  ✗ $3"
    mal=$((mal + 1))
  fi
}

# (archivo | línea original | línea rota | qué rompe): las roturas de los retos de la lección.
ROTURAS=(
  "src/lib.rs|        self.suma as f64 / self.conteo as f64|        (self.suma / self.conteo as u64) as f64|un promedio que trunca (reto 1)"
  "src/lib.rs|        e.conteo += 1;|        e.conteo = 1;|un conteo que no suma (reto 2)"
  "src/lib.rs|    if partidas.is_empty() {|    if partidas.is_empty() && false {|un CSV vacío que pasa como bueno (reto 3)"
)

echo "1. Tus pruebas (cargo test)"
salida=$(cargo test --no-fail-fast 2>&1)
codigo=$?
resumen=$(echo "$salida" | grep -E '^test result' | awk '{p+=$4; f+=$6} END {print p " pasan, " f " fallan"}')
check $codigo "cargo test en verde ($resumen)" "cargo test en rojo ($resumen)"
if [ $codigo -ne 0 ]; then
  echo "$salida" | grep -E '^test .* FAILED$' | sed 's/^test /      falla: /; s/ \.\.\. FAILED$//'
  if echo "$salida" | grep -q 'not yet implemented'; then
    echo "      Todavía hay funciones con todo!(): escríbelas siguiendo la lección."
  fi
  if echo "$salida" | grep -q 'could not compile'; then
    echo "      No compila. El primer error:"
    echo "$salida" | grep -m1 -A4 -E '^error(\[E[0-9]+\])?: ' | sed 's/^/        /'
  fi
fi

echo "2. Clippy sin advertencias (cargo clippy --all-targets -- -D warnings)"
salida=$(cargo clippy --all-targets --quiet -- -D warnings 2>&1)
codigo=$?
check $codigo "clippy limpio" "clippy encontró algo:"
[ $codigo -ne 0 ] && echo "$salida" | grep -m3 -E '^(error|warning)' | sed 's/^/      /'

echo "3. Formato (cargo fmt --check)"
cargo fmt --check >/dev/null 2>&1
check $? "el código tiene el formato de rustfmt" "falta formato: corre cargo fmt"

echo "4. Tus pruebas detectan los errores de los retos (en una copia)"
for rotura in "${ROTURAS[@]}"; do
  IFS='|' read -r archivo original roto que <<<"$rotura"
  contenido=$(cat "$archivo" 2>/dev/null)
  if [[ "$contenido" != *"$original"* ]]; then
    check 1 "" "no encuentro en $archivo la línea «${original#"${original%%[![:space:]]*}"}»: ¿ya la escribiste como en la lección?"
    continue
  fi
  copia=$(mktemp -d)
  cp -r Cargo.toml Cargo.lock src tests datos "$copia"/ 2>/dev/null
  printf '%s\n' "${contenido/"$original"/"$roto"}" >"$copia/$archivo"
  CARGO_TARGET_DIR="$DESTINO" cargo test --quiet --manifest-path "$copia/Cargo.toml" >/dev/null 2>&1
  codigo=$?
  rm -rf "$copia"
  if [ $codigo -ne 0 ]; then r=0; else r=1; fi
  check $r "detectan $que" "NO detectan $que: falta una prueba"
done

echo "5. target/ fuera de git"
grep -qxE '/?target/?' .gitignore 2>/dev/null
check $? ".gitignore excluye target/" "agrega target/ a .gitignore"

echo
if [ $mal -eq 0 ]; then
  echo "Laboratorio completo: $bien de $bien."
  exit 0
fi
if [ $mal -eq 1 ]; then echo "Falta 1 punto ($bien bien)."; else echo "Faltan $mal puntos ($bien bien)."; fi
exit 1
