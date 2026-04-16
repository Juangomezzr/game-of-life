# Arquitectura MVC - Game of Life

## Flujo de datos

```
┌─────────────────────────────────────────────────────────────────┐
│                        main.rs                                  │
│  Entry point que inicializa nannou y el ciclo de vida          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Controller (controller.rs)                 │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  simulation: Simulation  │  renderer: Renderer            │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  update()  → llama a simulation.step() + renderer.update()      │
│  render()  → llama a renderer.render()                          │
└─────────────────────────────────────────────────────────────────┘
           │                                    │
           ▼                                    ▼
┌─────────────────────┐              ┌─────────────────────┐
│   Simulation        │              │     Renderer        │
│   (simulation.rs)   │              │   (renderer.rs)     │
│                     │              │                     │
│  - grid: Grid       │              │  - texture: wgpu    │
│  - step()           │              │  - pixel_buffer     │
│  - get_grid()       │              │  - update_texture() │
│                     │              │  - send_to_gpu()    │
│  SOLO LÓGICA        │              │  SOLO RENDER         │
└─────────────────────┘              └─────────────────────┘
```

---

## Cada capa y su responsabilidad

### 1. `main.rs` — Entry Point

**Qué hace:** Configura nannou, crea la ventana, define el ciclo de vida.

**Qué NO hace:** No sabe nada de simulación, ni de render, ni de estados.

```rust
fn main() {
    nannou::app(model).update(update).run()
}
```

**Señal de que está bien:** Si cambias el motor gráfico (nannou → egui), esta es la única capa que tocas.

---

### 2. `controller.rs` — Orquestador

**Qué hace:** Coordina CUÁNDO actualizar y renderizar. Mantiene el estado de la app (paused, velocidad, etc.).

**Qué NO hace:** No sabe cómo se calcula un paso de simulación, ni cómo se dibuja una textura.

```rust
pub fn update(&mut self) {
    self.simulation.step();           // Delega
    self.renderer.update_texture(...); // Delega
}
```

**Señal de que está bien:** Si añades un botón de "pausa", solo tocas esta capa.

---

### 3. `simulation.rs` — Modelo

**Qué hace:** Reglas del Juego de la Vida, estado de la cuadrícula, patrones.

**Qué NO hace:** No sabe que existe una GPU, ni colores, ni ventanas.

```rust
pub fn step(&mut self) {
    self.grid.step();  // Lógica pura
}
```

**Señal de que está bien:** Puedes testear `Simulation::new()` y `step()` en un test unitario sin inicializar nannou.

---

### 4. `renderer.rs` — Vista

**Qué hace:** Convierte datos en píxeles, maneja texturas GPU, colores.

**Qué NO hace:** No decide cuándo actualizar, ni conoce las reglas del juego.

```rust
pub fn update_texture(&mut self, grid: &[u8]) {
    // Solo transforma [u8] → píxeles RGBA
}
```

**Señal de que está bien:** Si cambias el color de las células, solo tocas esta capa.

---

## ¿Cómo identificar cuándo usar este patrón?

### ✅ Úsalo cuando veas...

| Señal en tu código | Solución |
|-------------------|----------|
| Una función que hace lógica Y dibuja | Separa en `simulation` + `renderer` |
| No puedes testear sin inicializar todo | Saca la lógica a un módulo sin dependencias externas |
| Cambiar un color requiere tocar la lógica | Mueve colores al `renderer` |
| El `update()` hace demasiadas cosas | Crea un `controller` que orchestre |

### ❌ No lo uses cuando...

- Es un script de 50 líneas
- Todo el código cabe en una función
- No vas a añadir más features

---

## Patrón general para cualquier app

```
main.rs       → "Quién soy"
controller.rs → "Qué hago y cuándo"
model.rs      → "Cómo funciono"
view.rs       → "Cómo me veo"
```

**Ejemplos:**

| App | Model | View | Controller |
|-----|-------|------|------------|
| Juego | Física, entidades | Render 2D/3D | Input, estados del juego |
| Chat | Mensajes, usuarios | UI components | Send/receive, conexiones |
| Dashboard | Datos, cálculos | Gráficas, tablas | Fetch, refresh rate |

---

## ¿Qué ganaste con esto?

1. **Puedes testear la simulación** sin abrir ventana
2. **Puedes cambiar el render** (terminal, web, GUI) sin tocar la lógica
3. **Sabes dónde mirar** cuando algo falla
4. **Puedes añadir features** (pausa, velocidad, guardar/cargar) sin romper nada