use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            id: "home",
            class: "flex flex-col items-center justify-center min-h-[60vh] text-center p-6",

            h1 { class: "text-4xl sm:text-6xl font-extrabold tracking-tight",
                "Angel Medina"
            }
            p { class: "mt-3 text-lg opacity-80 max-w-xl",
                "Native Rust Engineer"
            }

            div { class: "mt-6 flex flex-wrap gap-4 justify-center",
                a {
                    href: "https://github.com/angel-rust",
                    class: "px-4 py-2 rounded-md bg-blue-600 text-white font-semibold hover:bg-blue-700 transition",
                    "GitHub"
                }
                a {
                    href: "mailto:hello@angelmedina.io",
                    class: "px-4 py-2 rounded-md border font-semibold hover:bg-black/5 dark:hover:bg-white/10 transition",
                    "Email"
                }
            }
        }
    }
}
