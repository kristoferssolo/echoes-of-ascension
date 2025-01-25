use app::telemetry::{get_subscriber, init_subscriber};

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    // initializes logging using the `log` crate
    let subscriber = get_subscriber("echoes-of-ascension-frontend", "info", std::io::stdout);
    init_subscriber(subscriber);

    leptos::mount::hydrate_body(App);
}
