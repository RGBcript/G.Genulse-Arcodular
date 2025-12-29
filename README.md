#  GENESIS  Neuromorphic AI System
> *Code that lives.*

![Status](https://img.shields.io/badge/Status-Active_v1.2-green) ![Lang](https://img.shields.io/badge/Language-Rust-orange) ![Arch](https://img.shields.io/badge/Architecture-Arcodular-blue)

**Genesis** es un framework de Inteligencia Artificial Neuromórfica escrito en **Rust**. A diferencia de las redes neuronales profundas tradicionales (Deep Learning), Genesis simula procesos biológicos fundamentales como la **plasticidad hebbiana rápida**, la **homeostasis química** y la **consolidación de memoria durante el sueño** (Sleep Protocol).

Su arquitectura **Arcodular** (Arc-Modular) divide la cognición en estructuras funcionales interconectadas ("Arcos"), permitiendo un comportamiento emergente complejo con una eficiencia computacional extrema.

---

##  Conceptos Clave

### 1. Vida Artificial vs. Estadística
Genesis no es solo un multiplicador de matrices. Cada **GenulseCell** (neurona) tiene:
*   **Estado interno persistente (Memoria a corto plazo)**.
*   **Pesos rápidos (w_fast)**: Aprendizaje instantáneo (One-shot learning).
*   **Pesos lentos (w_slow)**: Conocimiento consolidado (optimizados con BitNet: -1, 0, 1).
*   **Homeostasis**: Decaimiento natural de la energía para evitar la saturación ("Leaky Integrate-and-Fire").

### 2. El Ciclo Cognitivo
El sistema opera en un bucle continuo de tiempo discreto:
1.  **Perceive (Percibir):** Entrada de datos sensoriales crudos.
2.  **Think (Pensar):** Procesamiento a través de los arcos visuales, emocionales y racionales.
3.  **Panic Override:** Si la Amygdala detecta amenaza extrema, bloquea el PrefrontalCortex.
4.  **Sleep (Dormir):** Periódicamente, el sistema entra en estado de sueño para transferir lo aprendido en el día (_fast) a la memoria permanente (w_slow) y limpiar toxinas cognitivas.

---

##  Arquitectura del Sistema (v1.2)

El proyecto implementa la arquitectura **Arcodular** (Arc-Modular), un diseño propio que rechaza las capas monolíticas en favor de estructuras funcionales interconectadas.

| Módulo (Crate) | Definición y Propósito | Estado |
| :--- | :--- | :--- |
| **genulse** | **Motor de Física Neuromórfica (The Physics).**<br>Implementa las leyes fundamentales del universo cognitivo: Neural ODEs (tiempo continuo), Células Genulse con plasticidad Hebbiana y cuantización BitNet (-1, 0, 1). Es el sustrato biológico digital. |  Estable |
| **Arcodular** | **Sistema Operativo de la Conciencia (The Mind).**<br>Orquestador de alto nivel que gestiona los Arcos Cognitivos (Visual, Emocional, Racional). Controla el ciclo de vida (Percibir  Pensar  Dormir) y la integración con herramientas externas (Oracle Proxy). |  v1.2 |
| **Togen** | **Traductor de Realidad (The Senses).**<br>Convierte el caos del mundo físico (texto, imagen, audio) en **Togens**: átomos de significado matemático de 128-bits. Es la interfaz determinista entre la realidad y la cognición. |  Funcional |

---

##  Características Clave

### v1.2: The Awakening (Actual)
*   **Persistencia Conversacional:** Implementación de memoria a corto plazo (ShortTermMemory) que permite al agente recordar el contexto de la charla (N turnos anteriores).
*   **Proxy de Maestros (Oracle):** Conexión asíncrona (
eqwest) a APIs de inferencia (por defecto compatible con **Ollama** en localhost:11434).
*   **Tokio Async Runtime:** El núcleo ahora es no-bloqueante, permitiendo "pensar" y "sentir" simultáneamente.

### v1.1: Core Foundations (Legacy)
*   **Célula Genulse:** Unidad básica de procesamiento de información.
*   **Visual Cortex:** Matrices de percepción para interpretar inputs de datos crudos.

---

##  Instalación y Uso

### Prerrequisitos
1.  **Rust & Cargo:** curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2.  *(Opcional pero recomendado)* **Ollama** corriendo localmente con un modelo (ej. llama3) para que el cerebro tenga "voz".

### Ejecución

Clona el repositorio y lanza el cuerpo principal:

`ash
git clone https://github.com/RGBcript/G.Genulse-Arcodular.git
cd G.Genulse-Arcodular

# Ejecutar el sistema (asegúrate de estar en la raíz del workspace)
cargo run
`

*Si usas VS Code, puedes usar el **Task Runner** integrado.*

---

##  Roadmap Técnico

- [x] **Fase 0:** Portar lógica de Python a Rust (genulse).
- [x] **Fase 1:** Estructura de Workspace y compilación limpia.
- [x] **Fase 2:** Integración de API Externa y Memoria (v1.2).
- [ ] **Fase 3:** Input Sensorial Real (Conectar genulse al flujo de chat).
- [ ] **Fase 4:** Persistencia a Largo Plazo (Base de datos vectorial / Archivos).

###  Experimental: Aceleración por Hardware (WGPU)
Se ha iniciado la integración preliminar de **WebGPU** en el núcleo genulse.
- **Objetivo:** Procesamiento paralelo masivo de matrices sensoriales (Visión) usando Compute Shaders.
- **Estado:** Motor GpuVisualCortex implementado pero no activo por defecto.
- **Tecnología:** WGSL (WebGPU Shading Language) + wgpu-rs.

---

##  Contribución

Este es un proyecto de investigación personal para la creación de **AGI Local**.
Si eres estudiante o investigador, siéntete libre de hacer fork y experimentar con los kernels sensoriales en genulse.

**Autor:** [RGBcript](https://github.com/RGBcript)
**Licencia:** MIT


