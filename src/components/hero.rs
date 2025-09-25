use dioxus::prelude::*;

// use the preloaded header.svg
const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            id: "hero",
            class: "flex flex-col items-center justify-center min-h-screen bg-slate-900 text-white text-center p-6",
            
            // hero image
            img {
                src: HEADER_SVG,
                alt: "Header graphic",
                class: "w-32 h-32 mb-6"
            }

            // headline
            h1 { class: "text-4xl sm:text-6xl font-extrabold tracking-tight",
                "Angel Medina"
            }
            p { class: "mt-3 text-lg text-slate-300 max-w-xl",
                "Native Rust Engineer | Systems | Ops | Hospitality"
            }

            // links / buttons
            div { class: "mt-6 flex flex-wrap gap-4 justify-center",
                a { 
                    href: "https://github.com/angel-rust",
                    class: "px-4 py-2 rounded-md bg-blue-600 hover:bg-blue-700 transition",
                    "GitHub"
                }
                a { 
                    href: "https://www.linkedin.com/in/angelmedina",
                    class: "px-4 py-2 rounded-md bg-slate-700 hover:bg-slate-600 transition",
                    "LinkedIn"
                }
                a { 
                    href: "mailto:hello@angelmedina.io",
                    class: "px-4 py-2 rounded-md border border-slate-500 hover:bg-slate-800 transition",
                    "Email"
                }
            }
        }
    }
}
