use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn create_element(tag: String, container: String) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let node = document.create_element(tag.as_str()).unwrap();
    document
        .query_selector(container.as_str())
        .unwrap()
        .unwrap()
        .append_child(&node)?;
    Ok(())
}

#[wasm_bindgen]
pub fn del_element(query_selector: String) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let current_node = document
        .query_selector(query_selector.as_str())
        .unwrap()
        .unwrap();
    let parent_node = current_node.parent_node().unwrap();
    parent_node.remove_child(&current_node)?;
    Ok(())
}

#[wasm_bindgen]
pub fn test_dom() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    element.set_attribute("id", "app")?;
    let body = document.body().unwrap();
    body.append_child(&element)?;
    let span = document.create_element("span").unwrap();
    span.set_inner_html("hello");
    element.append_child(&span)?;

    let button_elem = document.create_element("button").unwrap();
    button_elem.set_inner_html("change title");
    let click_closure = Closure::wrap(Box::new(move || {
        span.set_inner_html("rom");
    }) as Box<dyn FnMut()>);
    button_elem
        .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())?;
    click_closure.forget();
    element.append_child(&button_elem)?;
    Ok(())
}

#[wasm_bindgen(start)]
pub fn start() {
    // let env = env_logger::Env::default();
    // env_logger::init_from_env(env);
    // env_logger::init();
    console_log!("starting!");
}
