# Hello World

## Tools

Install wasm-pack

```bash
cargo install wasm-pack
```

Inicializar el proyecto

```bash
cargo init
```

Compilando el crate usando wasm-pack, dentro de un wasm modulo.
wasm-pack tiene soporte para bundlers como webpack y rollup.

En este caso se genera un ES6 modulo por ello se utiliza el web target

```bash
wasm-pack build --target web
```

Se genera un directorio pkg el cual contiene el wasm modulo, contenido en un objeto
javascipt.
