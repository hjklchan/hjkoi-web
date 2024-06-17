use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { class: "py-24 flex justify-center",
            h1 { class: "text-center text-5xl font-base",
                "Not found"
            }
        }
    }
}