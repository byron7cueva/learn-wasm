use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);

    type HTMLDocument;
    type Element;

    static document: HTMLDocument;

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

    // Para obtener el root del documento
    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    #[wasm_bindgen(method, js_name = appendChild)]
    fn append(this: &Element, item: Element);

    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner(this: &Element, html: &str);
}

#[wasm_bindgen]
pub fn run_log(item: &str) {
    log(&format!("Log from wasm: {}", item));
}

#[wasm_bindgen]
pub fn run_alert(item: &str) {
    alert(&format!("From wasm: {}", item));
}

#[wasm_bindgen]
pub fn create_stuff() {
    let div = document.createElement("div");
    let p = document.createElement("p");

    p.set_inner("Hola from WASM in Rust");
    div.append(p);

    document.body().append(div);
}