use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event};
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

struct Global {
    pub document: Rc<web_sys::Document>,
    pub window: Rc<web_sys::Window>,
    pub count: i32,
}

impl Global {
    fn new() -> Self {
        let window = Rc::new(web_sys::window().unwrap());
        let document = Rc::new(window.document().unwrap());
        Global {
            window,
            document,
            count: 0,
        }
    }

    pub fn get_instance() -> Rc<RefCell<Global>> {
        static mut GLOBAL: Option<Rc<RefCell<Global>>> = None;
        unsafe {
            GLOBAL
                .get_or_insert_with(|| Rc::new(RefCell::new(Global::new())))
                .clone()
        }
    }
}

struct Store {
    count: i32,
    input_value: String,
    input_value_elem: Option<Vec<Rc<Element>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            count: 0,
            input_value: "".to_string(),
            input_value_elem: None,
        }
    }

    pub fn get_instance() -> Rc<RefCell<Store>> {
        static mut STORE: Option<Rc<RefCell<Store>>> = None;
        unsafe {
            STORE
                .get_or_insert_with(|| Rc::new(RefCell::new(Store::new())))
                .clone()
        }
    }

    pub fn push_input_value_elem(&mut self, elem: Rc<Element>) {
        if self.input_value_elem.is_none() {
            self.input_value_elem = Some(vec![elem]);
        } else {
            self.input_value_elem.as_mut().unwrap().push(elem);
        }
    }
}

#[wasm_bindgen]
pub fn test_dom() -> Result<(), JsValue> {
    let global = Global::get_instance();
    // let window = global.borrow_mut().window.clone();
    let document = global.borrow_mut().document.clone();
    let element = document.create_element("div").unwrap();
    element.set_attribute("id", "app")?;
    let body = document.body().unwrap();
    body.append_child(&element)?;
    {
        let span = document.create_element("span").unwrap();
        let span = Rc::new(span);
        let store = Store::get_instance();
        span.set_inner_html(store.borrow().input_value.as_str());
        if store.borrow_mut().input_value_elem.is_none() {
            store.borrow_mut().input_value_elem = Some(vec![]);
        }
        store
            .borrow_mut()
            .input_value_elem
            .as_mut()
            .unwrap()
            .push(Rc::clone(&span));
        element.append_child(&span)?;
    }

    let span = document.create_element("span").unwrap();
    let span = Rc::new(span);
    let store = Store::get_instance();
    span.set_inner_html(store.borrow().input_value.as_str());
    store.borrow_mut().push_input_value_elem(Rc::clone(&span));
    element.append_child(&span)?;
    {
        let input_elem = document.create_element("input").unwrap();
        let input_closure = Closure::wrap(Box::new(move |event: Event| {
            if let Some(target) = event.target() {
                if let Some(input_el) =
                    wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target)
                {
                    let v = input_el.value();
                    let title = v.trim();
                    let store = Store::get_instance();
                    store.borrow_mut().input_value = title.to_string();
                    console_log!("here");
                    update_text();
                }
            }
        }) as Box<dyn FnMut(_)>);
        input_elem
            .add_event_listener_with_callback("input", input_closure.as_ref().unchecked_ref())?;
        input_closure.forget();
        element.append_child(&input_elem)?;
    }
    {
        let button_elem = document.create_element("button").unwrap();
        button_elem.set_inner_html("change title");
        let click_closure = Closure::wrap(Box::new(move || {
            span.set_inner_html("rom");
        }) as Box<dyn FnMut()>);
        button_elem
            .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())?;
        click_closure.forget();
        element.append_child(&button_elem)?;
    }
    {
        let add_btn_elem = document.create_element("button").unwrap();
        add_btn_elem.set_inner_html("+1");
        let click_closure = Closure::wrap(Box::new(move || {
            add_count(1);
            get_count();
        }) as Box<dyn FnMut()>);
        add_btn_elem
            .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())?;
        click_closure.forget();
        element.append_child(&add_btn_elem)?;
    }
    {
        let add_btn_elem = document.create_element("button").unwrap();
        add_btn_elem.set_inner_html("+2");
        let click_closure = Closure::wrap(Box::new(move || {
            add_count(2);
            get_count();
        }) as Box<dyn FnMut()>);
        add_btn_elem
            .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())?;
        click_closure.forget();
        element.append_child(&add_btn_elem)?;
    }
    get_count();
    Ok(())
}

pub fn update_text() {
    let store = Store::get_instance();
    let value = &store.borrow().input_value.clone();
    store
        .borrow_mut()
        .input_value_elem
        .as_mut()
        .unwrap()
        .iter()
        .for_each(|x| {
            x.set_inner_html(value);
        });
}

pub fn get_count() {
    let store = Store::get_instance();
    let count = store.borrow().count;
    console_log!("{}", count);
}

pub fn add_count(x: i32) {
    let store = Store::get_instance();
    store.borrow_mut().count += x;
}

#[wasm_bindgen(start)]
pub fn start() {
    // let env = env_logger::Env::default();
    // env_logger::init_from_env(env);
    // env_logger::init();
    console_log!("starting!");
}
