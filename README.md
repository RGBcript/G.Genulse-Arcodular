# 🧠 GENESIS
> **AGI Local. Biológica. Eficiente.**

```text
   ______ ______ _   __ ______ _____ ____ _____
  / ____// ____// | / // ____// ___//  _// ___/
 / / __ / __/  /  |/ // __/   \__ \ / /  \__ \ 
/ /_/ // /___ / /|  // /___  ___/ // /  ___/ / 
\____//_____//_/ |_//_____/ /____//___//____/  
```

![Status](https://img.shields.io/badge/status-alpha-orange) ![Rust](https://img.shields.io/badge/rust-1.75%2B-blue) ![WASM](https://img.shields.io/badge/WASM-ready-green) ![Arch](https://img.shields.io/badge/Architecture-Arcodular-blue)

> **⚠️ Demo Visual:** *[Inserta aquí tu GIF/Screenshot de `docs/demo.gif`]*

**Genesis no es otro chatbot.** Es un organismo digital que **siente, duerme y aprende** en tiempo real.
Olvídate de las matrices estáticas. Genesis utiliza **Neurobiología Simulada** (Spiking Neural Networks + Homeostasis) para correr en tu CPU/GPU con una fracción de la energía de un LLM tradicional.

---

## ⚡ Quick Start (En 30 segundos)

### 1. El Cerebro (Backend)
Clona y corre la simulación de conciencia:
```bash
git clone https://github.com/RGBcript/G.Genulse-Arcodular.git
cd G.Genulse-Arcodular
cargo run -p arcodular
```
*Verás los logs del ciclo cognitivo: Percibir -> Pensar -> Dormir.*

### 2. Los Sentidos (Neuro-Link Frontend)
Para ver el **Tablero Visual** y el **Grafo Neuronal**:
```bash
cd togen/www
npm install
npm run dev
```
*Abre `http://localhost:5173` y selecciona la pestaña "Neuro-Link".*

---

## 🧬 ¿Por qué Genesis?

### 1. Biología, no estadística
**Mientras ChatGPT predice la siguiente palabra, Genesis construye modelos internos del contexto.**
Funciona como una sinapsis que se refuerza, no como un diccionario gigante.
*   **Memoria a Corto Plazo:** Recuerda el contexto sin re-procesar todo el historial.
*   **Sueño (Sleep Protocol):** Consolida aprendizajes (`a_fast` -> `w_slow`) y limpia toxinas cognitivas.
*   **Eficiencia:** Usa cuantización BitNet (-1, 0, 1) y Spiking Networks.

### 2. Arquitectura Arcodular
El sistema no es un bloque monolítico. Es un grafo de **Arcos Cognitivos**:
*   **Genulse (Física):** El motor de ecuaciones diferenciales.
*   **Arcodular (Mente):** El orquestador de módulos.
*   **Togen (Sentidos):** Protocolo de hashing multimodal (128-bit).

---

## 🏗️ Arquitectura del Sistema (v1.2)

El proyecto implementa la arquitectura **Arcodular** (Arc-Modular), un diseño propio que rechaza las capas monolíticas en favor de estructuras funcionales interconectadas.

| Módulo (Crate) | Definición y Propósito | Estado |
| :--- | :--- | :--- |
| **`genulse`** | **Motor de Física Neuromórfica (The Physics).**<br>Implementa las leyes fundamentales del universo cognitivo: Neural ODEs (tiempo continuo), Células Genulse con plasticidad Hebbiana y cuantización BitNet (-1, 0, 1). Es el sustrato biológico digital. | 🟢 Estable |
| **`arcodular`** | **Sistema Operativo de la Conciencia (The Mind).**<br>Orquestador de alto nivel que gestiona los Arcos Cognitivos (Visual, Emocional, Racional). Controla el ciclo de vida (Percibir → Pensar → Dormir) y la integración con herramientas externas (Oracle Proxy). | 🟡 v1.2 |
| **`togen`** | **Traductor de Realidad (The Senses).**<br>Convierte el caos del mundo físico (texto, imagen, audio) en **Togens**: átomos de significado matemático de 128-bits. Es la interfaz determinista entre la realidad y la cognición. | 🟢 Funcional |

---

## 🚀 Características Clave

### v1.3: The Neuro-Link (In Development)
*   **Genesis Neuro-Link:** Dashboard visual interactivo (React + WebAssembly) para visualizar el flujo de Togens y la actividad de los Arcos en tiempo real.
*   **Visual Node Editor:** Interfaz basada en nodos (Flow-Based Programming) para inspeccionar la arquitectura mental.

### v1.2: The Awakening (Actual)
*   **Persistencia Conversacional:** Implementación de memoria a corto plazo (`ShortTermMemory`) que permite al agente recordar el contexto de la charla (N turnos anteriores).
*   **Proxy de Maestros (Oracle):** Conexión asíncrona (`reqwest`) a APIs de inferencia (por defecto compatible con **Ollama** en `localhost:11434`).
*   **Tokio Async Runtime:** El núcleo ahora es no-bloqueante, permitiendo "pensar" y "sentir" simultáneamente.

### v1.1: Core Foundations (Legacy)
*   **Célula Genulse:** Unidad básica de procesamiento de información.
*   **Visual Cortex:** Matrices de percepción para interpretar inputs de datos crudos.

---

## 🛠️ Instalación y Uso

### Prerrequisitos
1.  **Rust & Cargo:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2.  **Node.js & NPM:** Para el frontend Neuro-Link.
3.  *(Opcional)* **Ollama** corriendo localmente para el módulo de lenguaje.

### Ejecución Completa

```bash
# Terminal 1: El Cerebro
cargo run -p arcodular

# Terminal 2: La Interfaz
cd togen/www
npm run dev
```

---

## 🧠 Roadmap Técnico

- [x] **Fase 0:** Portar lógica de Python a Rust (`genulse`).
- [x] **Fase 1:** Estructura de Workspace y compilación limpia.
- [x] **Fase 2:** Integración de API Externa y Memoria (v1.2).
- [x] **Fase 3:** Input Sensorial Real (Frontend Neuro-Link Implementado).
- [ ] **Fase 4:** **Broca's Arc (The Red Pill).** Implementación del *Symbol Grounding* biológico. Aprendizaje de lenguaje por asociación Hebbiana (Visual <-> Auditivo) sin LLMs.
- [ ] **Fase 5:** Persistencia a Largo Plazo (Base de datos vectorial / Archivos).

### 🧪 Experimental: Aceleración por Hardware (WGPU)
Se ha iniciado la integración preliminar de **WebGPU** en el núcleo `genulse`.
- **Objetivo:** Procesamiento paralelo masivo de matrices sensoriales (Visión) usando Compute Shaders.
- **Estado:** Motor `GpuVisualCortex` implementado pero no activo por defecto.
- **Tecnología:** WGSL (WebGPU Shading Language) + wgpu-rs.

---

## 📡 Protocolo Togen (Especificación v1.1)

El **Togen** es un hash de 128-bits que representa un "átomo de significado". Su estructura permite identificar el tipo de dato y sus propiedades sin necesidad de decodificar el contenido original.

### Estructura de Bits
`[Header: 8] [Meta: 8] [Semántico: 32] [Estructural: 32] [Exacto: 48]`

### Tipos de Datos (Header)
| Hex | Tipo | Descripción |
| :--- | :--- | :--- |
| `0x01` | **Texto** | Lenguaje natural. |
| `0x02` | **Código** | Lenguaje de programación (detectado por heurística). |
| `0x03` | **Imagen** | Datos visuales (JPG, PNG). |
| `0x04` | **Audio** | Datos sonoros (WAV). |
| `0x05` | **Acción** | Comandos de control (Teclado, Mouse, UI). |

### Formato de Acciones (Recomendado)
Aunque el sistema acepta texto libre, se recomienda el formato `Verbo:Parámetro` para mayor claridad semántica.

| Comando | Meta (Hex) | Tipo |
| :--- | :--- | :--- |
| `Click:Left`, `Move:100,200` | `0x02` | **Mouse** |
| `Key:Enter`, `Type:Hello` | `0x01` | **Teclado** |
| `Wait:1000`, `Scroll:Down` | `0x00` | **Otro** |

---

## 🤝 Contribución

Este es un proyecto de investigación personal para la creación de **AGI Local**.
Si eres estudiante o investigador, siéntete libre de hacer fork y experimentar con los kernels sensoriales en `genulse`.

**Autor:** [RGBcript](https://github.com/RGBcript)
**Licencia:** MIT
