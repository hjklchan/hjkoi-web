use dioxus::prelude::*;

#[component]
pub fn Detail(article_id: u64) -> Element {
    tracing::info!("Article detail");

    let markdown_body = include_str!("../../../README.md");
    
    rsx! {
        section { class: "pb-5",
            div { class: "flex justify-between",
                h1 { class: "text-xl font-semibold",
                    "Rust 中智能指针的学习记录（一）"
                }
                span { class: "text-gray-500 text-lg",
                    "March 21, 2024"
                }
            }

            // FIXME: Fix markdown html showing.
            article { class: "py-3 markdown-body dioxus-blog-post",
                dangerous_inner_html: format_args!("{}", markdown_body)
            }
        }
    }
}