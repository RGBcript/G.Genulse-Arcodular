# 🧬 Genesis Togenizer v1.1

> **Multimodal Hashing Protocol for Embodied AGI**
> *Part of Project Genesis*

Genesis Togenizer is a unified tokenization system that converts any input (Text, Image, Audio, Code, Action) into a deterministic **128-bit Hash (Togen)**. Unlike traditional LLM tokenizers, Togenizer is designed for **Genulse** (Neuromorphic/Analog) architectures, enabling extreme efficiency and native physical grounding.

## 🚀 Features

*   **Unified Hashing:** 128-bit output for all modalities.
*   **Structure:** `[Exact 48b] [Structural 32b] [Semantic 32b] [Meta 8b] [Header 8b]`
*   **Performance:** Written in **Rust** 🦀 with **WebAssembly (WASM)** support.
*   **Web Interface:** Includes a Cyberpunk React UI for testing and visualization.
*   **Deterministic:** Same input = Same Togen (Always).

## 🛠️ Project Structure

This is a Monorepo containing both the Core Logic (Rust) and the Web Interface.

genesis_togenizer/
├── src/ # 🦀 Rust Core Logic (Hashing Algorithms)
├── pkg/ # 📦 Compiled WASM Package
└── www/ # ⚛️ Web Interface (React + Vite + Tailwind)

## 📦 Installation & Usage

### 1. Core (Rust CLI)
cargo run -- "Hello World"

Output: Togen Hash (e.g., 0x01...)
text

### 2. Web Interface (Local)
cd www
npm install
npm run dev

Open http://localhost:5173 to use the Drag & Drop UI
text

## 🧠 The Genulse Vision
This tokenizer is the input layer for the upcoming **Genulse Architecture** (Genesis Impulse), a continuous-time neural network (Neural ODEs) designed to run on analog neuromorphic hardware.

---
*Built by Bryn & The Genesis Team* 🧪