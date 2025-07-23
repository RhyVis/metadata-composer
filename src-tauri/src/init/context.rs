use tauri::{Builder, Wry, generate_context};

pub fn run(builder: Builder<Wry>) {
    builder
        .run(generate_context!())
        .expect("Error while running application");
}
