use sycamore::prelude::*;

#[component]
pub fn App() -> View {
    view! {
        main(class="container") {
            h1 {
                "Welcome to Tauri + Sycamore"
            }

            div(class="row") {
                a(href="https://tauri.app", target="_blank") {
                    img(src="public/tauri.svg", class="logo tauri", alt="Tauri logo")
                }
                a(href="https://sycamore.dev", target="_blank") {
                    img(src="public/sycamore.svg", class="logo sycamore", alt="Sycamore logo")
                }
            }
            p {
                "Click on the Tauri and Sycamore logos to learn more."
            }
        }
    }
}
