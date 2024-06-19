use dioxus::prelude::*;

#[component]
pub fn Detail(article_id: u64) -> Element {
    tracing::info!("Article detail");

    let markdown_parser = pulldown_cmark::Parser::new(include_str!("../../../README.md"));
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, markdown_parser);

    rsx! {
        section { class: "max-w-screen-xl",
            div {
                div { class: "flex justify-between py-3 lg:py-10",
                    h1 { class: "text-3xl font-semibold",
                        "Rust 中智能指针的学习记录（一）"
                    }
                    span { class: "hidden md:flex text-gray-500 text-lg",
                        "March 21, 2024"
                    }
                }

                div {
                    article { class: "max-w-screen-xl py-3 prose font-serif",
                        dangerous_inner_html: html_output
                    }
                }
            }
        }
    }
}
