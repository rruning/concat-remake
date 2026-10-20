# Lote 1: 20 transiciones nuevas

Carpetas nuevas en src/crates/concat-effects/packages/ (no hace falta tocar Rust:
build.rs las registra solas).

Ids: concat.flash-cut, concat.pixelate-cut, concat.venetian-blinds, concat.ink-bleed,
concat.page-turn, concat.ripple-wave, concat.diagonal-split, concat.star-wipe,
concat.heart-reveal, concat.bar-slide, concat.checker-flip, concat.spiral-in,
concat.blur-cross, concat.mosaic-tiles, concat.door-open, concat.bar-grow,
concat.drop-wipe, concat.zoom-punch, concat.luma-wipe, concat.spin-swap

Sin fixtures.toml todavía. Para verificar, compilar y correr:

    cd src && cargo test -p concat-effects
    cargo run -p concat-cli -- check ../src/crates/concat-effects/packages/concat.page-turn

Y después agregar fixtures.toml con [[probe]] por cada una, para fijar colores de salida.
Las transiciones no están probadas en GPU.
