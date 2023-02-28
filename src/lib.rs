wit_bindgen_rust::export!("extension.wit");

use extension::State;
use std::cell::RefCell;

struct Extension;

thread_local! {
    static WASM_STATE: RefCell<State> = RefCell::new(State{ buffer: vec![], idx: 0 });
}

impl extension::Extension for Extension {
    /// Greets the invoker with a warm welcome
    /// returns: The welcome message
    fn greet(name: String) -> String {
        format!("Hello {name}")
    }

    /// Provides the answer to life, the universe & everything
    /// returns: 42
    fn answer_to_life() -> i32 {
        42
    }

    /// Updates the current WASM state
    fn set_state(s: State) -> i32 {
        WASM_STATE.with(|state| {
            state.replace(s);
            state.borrow().idx
        })
    }

    /// Returns the current WASM state
    fn get_state() -> State {
        WASM_STATE.with(|state| state.borrow().clone())
    }
}
