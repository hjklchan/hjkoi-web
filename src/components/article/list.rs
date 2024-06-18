use dioxus::prelude::*;

pub fn List() -> Element {
    rsx! {
        section { class: "pb-2",
            h1 { class: "text-3xl",
                "Articles"
            }
        }

        section {
            div { class: "container mx-auto",
                // TODO: Show each articles.
                div { class: "hover:bg-gray-200 flex flex-wrap md:flex-nowrap p-1 cursor-pointer",
                    onclick: move |_| {}, // TODO: On-Click to open the article detail.
                    div { class: "md:flex-grow",
                        div { class: "flex flex-row justify-between gap-4",
                            h2 { class: "text-md font-medium text-gray-900 title-font dark:text-white",
                                "Rust 中智能指针的学习记录（一）"
                            }
                            span { class: "text-gray-500 text-md",
                                "March 21, 2024"
                            }
                        }
                    }
                }
                div { class: "hover:bg-gray-200 flex flex-wrap md:flex-nowrap p-1 cursor-pointer",
                    div { class: "md:flex-grow",
                        div { class: "flex flex-row justify-between gap-4",
                            h2 { class: "text-md font-medium text-gray-900 title-font dark:text-white",
                                a { href: "",
                                    "Rust 中智能指针的学习记录（一）"
                                }
                            }
                            span { class: " text-gray-500 text-md",
                                "March 21, 2024"
                            }
                        }
                    }
                }
            }
        }
    }
}