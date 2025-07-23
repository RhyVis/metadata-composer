use tauri::{Builder, Wry};

use crate::init::context::run;

mod context;
mod invoke_handler;

pub trait BuilderExt {
    fn register_invoke_handler(self) -> Self;
    fn run_with_context(self);
}

impl BuilderExt for Builder<Wry> {
    fn register_invoke_handler(self) -> Self {
        invoke_handler::register_invoke_handler(self)
    }

    fn run_with_context(self) {
        run(self)
    }
}
