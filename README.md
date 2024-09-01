# Example

## WASM - Node.js module - written in Rust

### Building



- Install and setup prerequisites `rustup`.

- Install `wasm-pack` as a global Node.js module with npm.

```npm i -g wasm-pack```

- Use `wasm-pack` to generate the module in `/pkg` folder.

```wasm-pack build --target web```

### Usage

- Add this repo as a submodule or copy the contents into a folder.

- Build the package.

- Import the generated `/pkg` folder as a dependancy in your Node.js projects.

```
dependancies: {
    ...
    "eg-wasm-js-rust": "<modulePath>/pkg";
    ...
}
```
