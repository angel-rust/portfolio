use dioxus::{events::MouseEvent, prelude::*};

#[derive(Props, PartialEq, Clone)]
pub struct HeaderProps {
    pub is_dark: bool,
    pub on_toggle: EventHandler<MouseEvent>,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    rsx! {
        header { class: "max-w-5xl mx-auto px-4 py-6 flex items-center justify-between",
            a { href: "#home", class: "font-bold text-xl tracking-tight", "Angel Medina" }

            nav { class: "hidden md:flex gap-6 text-sm opacity-80",
                a { href: "#about",      class: "hover:opacity-100", "About" }
                a { href: "#experience", class: "hover:opacity-100", "Experience" }
                a { href: "#projects",   class: "hover:opacity-100", "Projects" }
                a { href: "#contact",    class: "hover:opacity-100", "Contact" }
            }

            button {
                class: "ml-4 inline-flex items-center gap-2 rounded-md border px-3 py-1.5 text-sm
                        hover:bg-black/5 dark:hover:bg-white/10 outline-none
                        focus-visible:ring-2 focus-visible:ring-blue-500",
                onclick: move |ev| props.on_toggle.call(ev),
                span { if props.is_dark { "🌙 Dark" } else { "☀️ Light" } }
            }
        }
    }
}
