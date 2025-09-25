use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "border-t mt-12",
            div { class: "max-w-5xl mx-auto px-4 py-6 text-sm opacity-80",
                "© 2025 — Built with Rust (Dioxus) + Tailwind"
            }
        }
    }
}
