use dioxus::prelude::*;

use crate::components::Route;

pub fn List() -> Element {
    rsx! {
        section { class: "pb-2",
            h1 { class: "text-3xl",
                "Articles"
            }
        }

        section {
            div { class: "container mx-auto",
                // Simple article list items
                // TODO: Show each articles.
                Link {
                    to: Route::ArticleDetail { article_id: 1 },
                    div { class: "hover:bg-gray-200 flex flex-wrap md:flex-nowrap p-1 cursor-pointer",
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
                }
            }
        }
    }
}