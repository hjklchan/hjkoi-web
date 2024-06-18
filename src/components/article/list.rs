use crate::components::Route;
use dioxus::prelude::*;

#[derive(Debug)]
#[allow(unused)]
pub struct ArticleItem {
    id: u64,
    title: String,
    description: String,
    created_at: String,
}

pub fn List() -> Element {
    let range = 1..30;
    let total = range.len();

    rsx! {
        section { class: "pb-5 px-1.5",
            div { class: "flex justify-between items-center",
                h1 { class: "text-3xl",
                    span { class: "font-medium", "Articles "}
                    "({total})"
                }
                span { class: "text-lg cursor-pointer block text-gray-400 hover:text-gray-500",
                    onclick: move |_evt| tracing::info!("Create a new article"),
                    "Create"
                }
            }
        }

        section {
            div { class: "container mx-auto",
                // Simple article list items
                // TODO: Show each articles.
                for i in range {
                    div { class: "py-0.5",
                        Link {
                            to: Route::ArticleDetail { article_id: 1 },
                            div { class: "hover:bg-[#E7ECEE] flex flex-wrap cursor-pointer py-1 px-1.5",
                                div { class: "w-full flex flex-row justify-between gap-4",
                                    h2 { class: "text-base font-medium text-gray-900 title-font",
                                        "Rust 中智能指针的学习记录（{i}）Rust 中智能指针的学习记录（{i}）"
                                    }
                                    span { class: "w-1/2 text-end text-gray-500 text-md",
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
}
