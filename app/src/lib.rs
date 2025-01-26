pub mod components;
pub mod config;
pub mod models;
pub mod server_fn;
pub mod telemetry;
pub mod validation;

pub use components::app::App;

use leptos::prelude::*;
use leptos_meta::MetaTags;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}
