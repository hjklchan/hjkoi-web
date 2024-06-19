use dioxus::prelude::*;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ArticleDetail {
    id: u64,
    title: String,
    body: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>
}

#[component]
pub fn Detail(article_id: u64) -> Element {
    let resource = use_resource(move || get_article(article_id));

    rsx! {
        section { class: "max-w-screen-xl",
            match &*resource.read_unchecked() {
                Some(Ok(article)) => {
                    let markdown_parser = pulldown_cmark::Parser::new(&article.body);
                    let mut html_output = String::new();
                    pulldown_cmark::html::push_html(&mut html_output, markdown_parser);
                    
                    rsx! {
                        div {
                            div { class: "flex justify-between py-3 lg:py-10",
                                h1 { class: "text-3xl font-semibold",
                                    "{article.title}"
                                }
                                span { class: "hidden md:flex text-gray-500 text-lg",
                                    match article.created_at {
                                        Some(t) => {
                                            t.format("%D").to_string()
                                        },
                                        None => {
                                            String::from("Unknown")
                                        },
                                    }
                                }
                            }
            
                            div {
                                if article.body.len() == 0 {
                                    span { "No content..." }
                                } else {
                                    article { class: "max-w-screen-xl py-3 prose font-serif",
                                        dangerous_inner_html: html_output
                                    }
                                }
                            }
                        }
                    }
                }
                Some(Err(_err)) => {
                    tracing::info!("{:#?}", _err);
                    rsx! { h1 { class: "text-2xl font-medium", "Occurred error!" } }
                }
                None => {
                    rsx! { h1 { class: "text-2xl font-medium", "Loading..." } }
                }
            }
        }
    }
}

async fn get_article(article_id: u64) -> Result<ArticleDetail, reqwest::Error> {
    let url = format!("http://localhost:8888/articles/{}", article_id);
    Ok(reqwest::get(url).await?.json::<ArticleDetail>().await?)
}
