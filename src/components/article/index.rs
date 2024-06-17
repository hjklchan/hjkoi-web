use dioxus::prelude::*;

pub fn Index() -> Element {
    rsx! {
        section { class: "pb-3",
            h1 { class: "text-3xl",
                "Articles"
            }
        }

        section {
            div { class: "container mx-auto",
                div { class: "hover:bg-gray-200 flex flex-wrap md:flex-nowrap p-1",
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
                        // p { "这是一条很长很长很长很长很长很长很长很长的描述..." }
                        // a { class: "text-indigo-500 inline-flex items-center mt-4",
                        //     href: "/",
                        //     "Read more ->"
                        // }
                    }
                }
                div { class: "hover:bg-gray-200 flex flex-wrap md:flex-nowrap p-1",
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
                        // p { "这是一条很长很长很长很长很长很长很长很长的描述..." }
                        // a { class: "text-indigo-500 inline-flex items-center mt-4",
                        //     href: "/",
                        //     "Read more ->"
                        // }
                    }
                }
            }
        }
    }
}