# Módulo de Efecto

```rust
use crate::{modules::particles, render_effects::{self, RenderEffect}};

struct {{EFFECT_NAME}} {
    id: String
}

impl {{EFFECT_NAME}} {

    fn new() -> Self {
        {{EFFECT_NAME}} { id: String::from("{{effect_id}}") }
    }
}

impl RenderEffect for {{EFFECT_NAME}} {

    fn apply(&mut self, _app: &nannou::App, grid: &[u8], buffer: &mut [u8]) {
        // Lógica principal del efecto
    }

    fn get_id(&self) -> &str {
        &self.id
    }

    fn render(&self, _app: &nannou::App) {
        // Renderizado si es necesario
    }

    fn update(&mut self, _app: &nannou::App, grid: &[u8], buffer: &mut [u8]) {
        // Actualización del buffer de pixels
    }
}
```

## Para crear un nuevo efecto

1. Copiá este archivo como `src/modules/<NOMBRE>.rs`
2. Reemplazá `{{EFFECT_NAME}}` con el nombre del efecto (ej: `Glow`, `Blur`)
3. Reemplazá `{{effect_id}}` con un id único en string (ej: `"glow"`, `"blur"`)
4. Agregá `pub mod <NOMBRE>;` en `src/modules/mod.rs`
5. Agregá `pub use <NOMBRE>::*;` en `src/modules/mod.rs` si querés re-exportarlo
6. Implementá la lógica de `apply` / `update`

## Estructura del trait `RenderEffect`

| Método | Descripción |
|--------|-------------|
| `apply` | Aplica el efecto al buffer |
| `get_id` | Retorna el ID único del efecto |
| `render` | Renderizado (opcional) |
| `update` | Actualización por frame |

## Registro en `find_module`

Si usás un match para elegir efecto por string:

```rust
fn find_module(module: &str) -> Box<dyn RenderEffect> {
    match module {
        "{{effect_id}}" => Box::new({{EFFECT_NAME}}::new()),
        _ => Box::new({{EFFECT_NAME}}::new()),
    }
}
```