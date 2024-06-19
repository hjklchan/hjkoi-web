use crate::components::Route;
use dioxus::prelude::*;

#[derive(Debug, serde::Deserialize, Clone, PartialEq)]
pub struct ArticleItem {
    id: u64,
    title: String,
    description: Option<String>,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ListRep {
    items: Vec<ArticleItem>,
}

pub fn List() -> Element {
    let range = 1..30;
    let total = range.len();

    let resource = use_resource(move || get_list(10));

    rsx! {
        section { class: "pb-5 px-1.5",
            div { class: "flex justify-between items-center",
                h1 { class: "text-3xl",
                    span { class: "font-medium", "Articles "}
                    "({total})"
                }
                Link { class: "text-lg cursor-pointer block text-gray-400 hover:text-gray-500",
                    to: Route::CreateArticle { },
                    "Create"
                }
            }
        }

        section {
            div { class: "container mx-auto",
                // Simple article list items
                match &*resource.read_unchecked() {
                    Some(Ok(list)) => {
                        rsx! {
                            for item in list {
                                ListItem { item: item.clone() }
                            }
                        }
                    }
                    Some(Err(_err)) => {
                        rsx! { h1 { class: "text-2xl font-medium", "Occurred error!" } }
                    }
                    None => {
                        rsx! { h1 { class: "text-2xl font-medium", "Loading..." } }
                    }
                }
            }
        }
    }
}

#[component]
fn ListItem(item: ArticleItem) -> Element {
    rsx! {
        div { class: "py-0.5",
            Link {
                to: Route::ArticleDetail { article_id: item.id },
                div { class: "hover:bg-[#E7ECEE] flex flex-wrap cursor-pointer py-1 px-1.5",
                    div { class: "w-full flex flex-row justify-between gap-4",
                        h2 { class: "text-base font-medium text-gray-900 title-font",
                            "{item.title}"
                        }
                        span { class: "text-end text-gray-500 text-md",
                            match item.created_at {
                                Some(t) => {
                                    t.format("%D").to_string()
                                },
                                None => {
                                    String::from("Unknown")
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn get_list(count: u32) -> Result<Vec<ArticleItem>, reqwest::Error> {
    let url = "http://localhost:8888/articles";
    let dat = &reqwest::get(url).await?.json::<ListRep>().await?;

    Ok(dat.items.clone())
}
