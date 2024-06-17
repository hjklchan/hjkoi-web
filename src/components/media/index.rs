use dioxus::prelude::*;


pub fn Index() -> Element {
    rsx! {
        div { class: "space-y-2",
            h1 { class: "text-2xl font-base",
                "Media Area"
            }
            hr {}
        }

        div {
            span { "MP4 or MP3" }
        }
    }
}