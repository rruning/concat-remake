# Lote 2: sonidos integrados + efectos de movimiento estilo CapCut

## 1. Shelf "Integrados" en la página de Audio (sin internet)

16 sonidos sintetizados en la máquina (originales, sin copyright), escritos
como WAV en `<library>/audio/builtin/` la primera vez que se abren:

    whoosh, explosion, boom (golpe grave), riser, click (tipo MC),
    pop, ding, coin, sweep-down, laser, punch, sparkle, scratch,
    shutter, rewind, error

Archivos nuevos:

- `src/crates/concat-host/src/sfx.rs` — sintetizador + pruebas (5 tests)
- `src/crates/concat/src/panes/sounds.rs` — shelf `fx-…` (preview, +, drag)
- `src/crates/concat/ui/workspace/audio-page.slint` — propiedad `builtin`
- `src/crates/concat/ui/workspace/media-pane.slint` — categoría "Integrados"
- `src/crates/concat/ui/phone/editor.slint` — chip "Integrados" primero
- `src/crates/concat/{src/studio.rs,src/lib.rs}` — cableado del modelo
- Locales: claves `sounds.builtin`, `sounds.builtinHint`, `sounds.catUi`

## 2. Cinco efectos de movimiento (WGSL, build.rs los registra solo)

Ids: concat.shake, concat.jitter, concat.bounce, concat.heartbeat,
concat.strobe — carpeta nueva cada uno en
`src/crates/concat-effects/packages/` (effect.toml + effect.wgsl).
Parámetros: Amount %, Speed. Categoría "Motion", orders 300-304.
Sin fixtures.toml todavía (igual que el lote 1).

## Verificar

    cd src
    cargo test -p concat-effects
    cargo test -p concat-host sfx
    cargo run --profile quick -p concat

`cargo test -p concat-effects` queda en rojo por las 20 transiciones del
lote 1, que siguen sin `fixtures.toml` (deuda ya anotada en
NUEVAS_TRANSICIONES.md): el test `every_scene_linear_package_probes...`
pide probes default/min/max por paquete. Los 5 efectos de movimiento de
este lote SÍ llevan fixtures y pasan; `every_built_in_package_loads` pasa
con los 157 paquetes.

Los 5 efectos de movimiento no están probados en GPU.
