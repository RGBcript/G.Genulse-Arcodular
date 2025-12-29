# 🧠 GENESIS — Neuromorphic AI System
> *Code that lives.*

![Status](https://img.shields.io/badge/Status-Stable_v1.1-green) ![Lang](https://img.shields.io/badge/Language-Rust-orange) ![Arch](https://img.shields.io/badge/Architecture-Arcodular-blue)

**Genesis** es un framework de Inteligencia Artificial Neuromórfica escrito en **Rust**. A diferencia de las redes neuronales profundas tradicionales (Deep Learning), Genesis simula procesos biológicos fundamentales como la **plasticidad hebbiana rápida**, la **homeostasis química** y la **consolidación de memoria durante el sueño** (Sleep Protocol).

Su arquitectura **Arcodular** (Arc-Modular) divide la cognición en estructuras funcionales interconectadas ("Arcos"), permitiendo un comportamiento emergente complejo con una eficiencia computacional extrema.

---

## 🧬 Conceptos Clave

### 1. Vida Artificial vs. Estadística
Genesis no es solo un multiplicador de matrices. Cada **GenulseCell** (neurona) tiene:
*   **Estado interno persistente (Memoria a corto plazo)**.
*   **Pesos rápidos (`a_fast`)**: Aprendizaje instantáneo (One-shot learning).
*   **Pesos lentos (`w_slow`)**: Conocimiento consolidado (optimizados con BitNet: -1, 0, 1).
*   **Homeostasis**: Decaimiento natural de la energía para evitar la saturación ("Leaky Integrate-and-Fire").

### 2. El Ciclo Cognitivo
El sistema opera en un bucle continuo de tiempo discreto:
1.  **Perceive (Percibir):** Entrada de datos sensoriales crudos.
2.  **Think (Pensar):** Procesamiento a través de los arcos visuales, emocionales y racionales.
3.  **Panic Override:** Si la `Amygdala` detecta amenaza extrema, bloquea el `PrefrontalCortex`.
4.  **Sleep (Dormir):** Periódicamente, el sistema entra en estado de sueño para transferir lo aprendido en el día (`a_fast`) a la memoria permanente (`w_slow`) y limpiar toxinas cognitivas.

---

## 🏗 Arquitectura del Workspace

El proyecto está organizado en un Workspace de Cargo con tres componentes principales:

### `🧩 genulse_core`
El motor físico y matemático.
*   Implementación de **Neural ODEs** discretas.
*   Tipos de datos optimizados para inferencia rápida.
*   Lógica de la célula `GenulseCell` (Hebbian Plasticity + Homeostatic Decay).

### `👁️ genesis_togenizer`
La interfaz sensorial.
*   Convierte datos crudos (texto, imagen, señales) en **Togens**.
*   Generación de embeddings deterministas.

### `🧠 genesis_brain`
El orquestador central (La "Mente").
*   Conecta los arcos:
    *   **VisualCortex:** Procesamiento espacial (Convolución).
    *   **Amygdala:** Sistema de alerta temprana y emocional.
    *   **Hippocampus:** Context y memoria episódica.
    *   **PrefrontalCortex:** Toma de decisiones y control.

---

## 🚀 Instalación y Uso

### Requisitos
*   Rust (Latest Stable)
*   Cargo

### Ejecución
Clona el repositorio y ejecuta el cerebro principal:

```bash
git clone https://github.com/RGBcript/G.Genulse-Arcodular.git
cd G.Genulse-Arcodular/MAINI

# Ejecutar en modo release (recomendado para performance)
cargo run --release -p genesis_brain
```

### Configuración de Logs
Para ver los detalles del pensamiento interno:

```bash
# Windows (PowerShell)
$env:RUST_LOG="info"
cargo run -p genesis_brain
```

---

## 📊 Estado Actual (v1.1 Stable)

Esta versión marca la recuperación total tras el incidente de hardware y la estabilización del núcleo.

*   ✅ **Stable Core:** Reparado el bug de "Infinite Feedback Loop" mediante correcciones de homeostasis.
*   ✅ **Sleep Protocol:** Funcional. La consolidación reduce la entropía del sistema correctamente.
*   ✅ **Steady State:** La actividad neuronal se mantiene en un rango saludable (~33.75).

## 🗺️ Roadmap (Hacia v1.2)

- [ ] **Entrada Sensorial Real:** Soporte para streams de video y audio en tiempo real.
- [ ] **GPU Acceleration:** Portar operaciones matriciales pesadas a `wgpu`.
- [ ] **Vector Quantization (VQ):** Mejora en el togenizer para compresión conceptual.

---
