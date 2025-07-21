use tauri::{Builder, Wry};

mod invoke_handler;

pub trait BuilderExt {
    fn register_invoke_handler(self) -> Self;
}

impl BuilderExt for Builder<Wry> {
    fn register_invoke_handler(self) -> Self {
        invoke_handler::register_invoke_handler(self)
    }
}
