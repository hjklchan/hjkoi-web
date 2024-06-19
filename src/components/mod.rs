#![allow(non_snake_case)]

pub mod article;
pub mod media;
pub mod not_found;

use crate::components::article::{
    Create as CreateArticle, Detail as ArticleDetail, List as ArticleList,
};
use crate::components::media::Index as Media;
use crate::components::not_found::NotFound;
use dioxus::prelude::*;

#[derive(Routable, PartialEq, Debug, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
        #[route("/")]
        Root {},
        // Articles
        // List
        #[route("/articles")]
        ArticleList {},
        // Create
        #[route("/articles/create")]
        CreateArticle {},
        // Detail
        #[route("/articles/:article_id")]
        ArticleDetail { article_id: u64 },

        // Media
        #[route("/media")]
        Media {},
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

pub fn Root() -> Element {
    rsx! {}
}

pub fn Layout() -> Element {
    rsx! {
        div { class: "bg-white pb-8",
            header { class: "sticky top-0 z-10 bg-[#E7ECEE] bg-opacity-80 shadow-md backdrop-blur-lg backdrop-filter firefox:bg-opacity-90",
                div { class: "py-1.5 px-2 max-w-screen-2xl mx-auto flex items-center justify-between leading-6",
                    div { class: "flex z-50 md:flex-1",
                        nav { class: "flex space-x-3 text-md items-center font-light text-slate-700 whitespace-nowrap",
                            Link { class: "flex text-lg title-font font-medium items-center text-gray-900 pr-1.5",
                                to: Route::Root {},
                                span { class: "hover:skew-y-6", "hjkl1:)" }
                            }
                            Link {
                                to: Route::Media {},
                                "媒体"
                            }
                            Link { to: Route::ArticleList {}, "文章" }
                            a { class: "", href: "/", "游戏" }
                            a { class: "", href: "/", "关于我" }
                        }
                    }
                    div { class: "hidden md:flex h-full justify-end ml-2 flex-1",
                        div { class: "hidden md:flex items-center",
                            div { class: "hidden lg:flex items-center border-l border-gray-300 ml-4 pl-2",
                                a { class: "ml-2 block text-gray-400 hover:text-gray-500",
                                    target: "_blank",
                                    href: "https://github.com/hjklchan",
                                    "Github"
                                }
                            }
                        }
                    }
                }
            }

            div { class: "w-full text-sm",
                style: "min-height: 100vh;",
                div { class: "flex flex-row justify-center font-light",
                    div { class: "text-gray-600 body-font overflow-hidden container pb-12 max-w-screen-xl mx-2 lg:mx-24 pt-7 grow",
                        section { Outlet::<Route> {} }
                    }
                }
            }
        }

        footer { class: "sticky z-30 text-gray-400 border-t bg-[#E7ECEE] bg-opacity-80 backdrop-blur-lg backdrop-filter firefox:bg-opacity-90",
            div { class: "container mx-auto py-1.5 px-5 flex flex-wrap flex-col sm:flex-row",
                p { class: "text-gray-400 text-sm text-center sm:text-left",
                    "© 2024 Dioxus Labs —"
                    a { class: "text-gray-500 ml-1",
                        href: "/",
                        "@hjkl1"
                    }
                }
            }
        }
    }
}
