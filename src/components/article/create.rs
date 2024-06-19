use dioxus::prelude::*;

pub fn Create() -> Element {
    rsx! {
        section { class: "pb-5",
            div { class: "items-center",
                h1 { class: "text-3xl",
                    span { class: "font-medium", "Create Article"}
                }
            }
        }

        section {
            "TODO..."
        }
    }
}