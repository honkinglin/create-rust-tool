# Create Rust Tool Template 🦀

A robust **Rust Project Template** designed for building modern tools that need to run everywhere: **CLI**, **Browser (WASM)**, and **NPM**.

## Features

-   **Monorepo Structure**: Managed via Cargo Workspace.
-   **Core Logic**: Pure Rust "Core" crate, shared across all targets.
-   **CLI**: Ready-to-use Command Line Interface (using `clap`).
-   **WASM**: Pre-configured `wasm-bindgen` setup for browser compatibility.
-   **NPM Integration**: Scripts and scaffolds for publishing to NPM and integrating with Vite/Webpack.

## 🚀 Quick Start

To use this template, you need [cargo-generate](https://github.com/cargo-generate/cargo-generate).

1.  **Install cargo-generate**:
    ```bash
    cargo install cargo-generate
    ```

2.  **Generate a new project**:
    ```bash
    cargo generate --git https://github.com/honkinglin/create-rust-tool --name my-tool
    ```

3.  **Start Developing**:
    ```bash
    cd my-tool
    # Run the CLI
    cargo run -p my-tool-cli -- add 1 2
    ```

## 📂 Project Structure

Once generated, your project will look like this:

```
my-tool/
├── crates/
│   ├── core/           # 🧠 The brain. All shared logic goes here.
│   ├── cli/            # 🖥️ The terminal. CLI wrapper around core.
│   └── wasm/           # 🌐 The web. WASM bindings for core.
├── packages/
│   ├── wasm-pkg/       # 📦 Generated NPM package (after build).
│   ├── vite-plugin/    # ⚡ Example Vite plugin.
│   └── webpack-plugin/ # 📦 Example Webpack plugin.
└── scripts/
    └── build-wasm.sh   # 🛠️ Script to build WASM package.
```

## 🛠️ Development Workflow

### 1. Develop Core Logic
Edit `crates/core/src/lib.rs`. This is where you implement your tool's actual functionality (e.g., image compression, data processing).

Run tests:
```bash
cargo test -p my-tool-core
```

### 2. Verify CLI
The CLI (`crates/cli`) imports your core crate. Run it locally:
```bash
cargo run -p my-tool-cli -- --help
```

### 3. Build for Web (WASM)
To compile your Rust code for the browser:

```bash
./scripts/build-wasm.sh
```

This will run `wasm-pack` and output the NPM package to `packages/wasm-pkg`.

## 📦 Publishing

### 1. Publish to Crates.io (Rust)
Publish your crates in this order:
1.  `crates/core`
2.  `crates/cli` (depends on core)

```bash
cd crates/core && cargo publish
cd ../cli && cargo publish
```

### 2. Publish to NPM (JavaScript)
1.  Build the WASM package: `./scripts/build-wasm.sh`
2.  Publish the generic WASM wrapper:
    ```bash
    cd packages/wasm-pkg
    npm publish --access public
    ```
3.  (Optional) Publish plugins if you have them (`packages/vite-plugin`).

