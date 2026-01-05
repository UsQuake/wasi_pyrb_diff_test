# Native vs WASM Python/Ruby Differential Testing

## 🔍 Context & Motivation

> **This project originates from a *failed WASM fuzzing experiment*.**

### 💥 Why WASM Fuzzing Failed

- 🧩 **No reliable oracle** — semantic bugs rarely crash.
- 🌫️ **Plausible but divergent results** across runtimes.
- 🔄 **Cross-engine non-determinism** (V8 / JSC / SpiderMonkey).
- ❌ Coverage-driven fuzzing produced little insight.

👉 The failure revealed a *semantic robustness problem*, not a tooling issue.

---

## 🧠 Reframing the Question

Instead of asking:

> *“Can we make WASM crash?”*

We ask:

> **“When and how does program meaning diverge across execution environments?”**

This project directly studies **semantic equivalence** under identical inputs.

---

## 🔬 What This Project Does

- 🧪 Differential testing between:
  - Native Python/Ruby
  - WASM (WASI-based) Python/Ruby
- 🌍 Execution across multiple JS engines:
  - V8
  - JavaScriptCore
  - SpiderMonkey
- ⚖️ Native execution as a behavioral reference
- 🧠 Interpreters as stress-inducing, high-level workloads

✨ Focuses on *semantic robustness*, not performance or crashes.

---

## 🔗 From Failed Fuzzing to Structured Exploration

| Failed WASM Fuzzing | This Project |
|---|---|
| Random input mutation | Differential execution |
| Missing oracle | Native vs WASM reference |
| Engine-specific behavior | Explicit cross-engine comparison |
| Low insight | Observable semantic boundaries |

👉 **Failure became signal.**

---

## 🧩 Background: WASI

WASI (WebAssembly System Interface) enables WASM to run outside browsers by providing:
- POSIX-like APIs
- `main`-based execution
- libc compatibility

WASM bytecode alone requires a runtime; with WASI it can execute in VMs such as:
- https://github.com/bytecodealliance/wasmtime

---

## 🛠️ Implementation Overview

### WASI-Polyfilled Python/Ruby in JS Engines

Execution without browser Web APIs is achieved via:

1. Async threading from `dart2wasm`
2. JavaScript-based WASI polyfills
3. Custom UTF encoding utilities
4. Integrated runtime glue logic
5. In-memory WASI filesystem mapping

Related tooling:
- https://github.com/bjorn3/browser_wasi_shim
- https://github.com/UsQuake/wasi_sandbox_generator

---

## ⚙️ Requirements

- Rust 1.71.1 (2021)
- Docker (API 1.4.0)

---

## 🚀 Setup

### Clone
```bash
git clone https://github.com/UsQuake/wasi_pyrb_diff_test.git
cd wasi_pyrb_diff_test
```

### Build Docker Images
```bash
sudo docker image build -t d8_py  ./sandbox_imgs/d8_python_wasi
sudo docker image build -t js_py  ./sandbox_imgs/js_python_wasi
sudo docker image build -t jsc_py ./sandbox_imgs/jsc_python_wasi
sudo docker image build -t na_py  ./sandbox_imgs/native_python

sudo docker image build -t d8_rb  ./sandbox_imgs/d8_ruby_wasi
sudo docker image build -t js_rb  ./sandbox_imgs/js_ruby_wasi
sudo docker image build -t jsc_rb ./sandbox_imgs/jsc_ruby_wasi
sudo docker image build -t na_rb  ./sandbox_imgs/native_ruby
```

### Build & Run
```bash
cargo build
sudo target/debug/main
```

---

## ✨ Takeaway

> **If semantics are too subtle to crash,  
> then semantics must be studied directly.**
