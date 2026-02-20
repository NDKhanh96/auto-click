use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};

use crate::api::mouse::click_fixed;

#[component]
pub fn App() -> impl IntoView {
    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let _ = click_fixed(1486, 1074).await;
        });
    };

    view! {
        <main class="">
            <h1>"Welcome to Tauri + Leptos"</h1>
            <p>"Click on the Tauri and Leptos logos to learn more."</p>
            <button type="" class="bg-blue-500">
                "Greet"
            </button>
        </main>
    }
}
