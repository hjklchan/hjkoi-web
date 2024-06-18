use dioxus::prelude::*;

#[component]
pub fn Detail(article_id: u64) -> Element {
    tracing::info!("Article detail");

    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    let parser = pulldown_cmark::Parser::new_ext(include_str!("../../../README.md"), options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    rsx! {
        section { class: "max-w-screen-xl",
            div {
                div { class: "flex justify-between py-10",
                    h1 { class: "text-xl font-semibold",
                        "Rust 中智能指针的学习记录（一）"
                    }
                    span { class: "text-gray-500 text-lg",
                        "March 21, 2024"
                    }
                }

                div {
                    article { class: "w-full py-3 prose",
                        dangerous_inner_html: "{html_output}"
                    }
                }
            }
        }
    }
}
