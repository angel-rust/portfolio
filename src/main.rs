#![allow(non_snake_case)]
use dioxus::document;
use dioxus::prelude::*;

mod components;
use components::{Footer, Header, Hero};

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    #[derive(Clone, Copy, PartialEq)]
    enum UiTheme {
        Light,
        Dark,
    }

    // Signal must be mutable if you call .set(...)
    let mut theme = use_signal(|| UiTheme::Dark);

    // Read current theme and provide a toggle handler
    let is_dark = matches!(*theme.read(), UiTheme::Dark);
    let on_toggle = move |_| {
        let cur = *theme.read();
        theme.set(match cur {
            UiTheme::Dark => UiTheme::Light,
            UiTheme::Light => UiTheme::Dark,
        });
    };

    // Compute wrapper + border classes based on theme (no html.dark needed)
    let shell = if is_dark {
        "min-h-dvh bg-slate-900 text-slate-100"
    } else {
        "min-h-dvh bg-white text-slate-900"
    };
    let border_color = if is_dark { "border-slate-800" } else { "border-slate-200" };

    rsx! {
        // Only Tailwind (ensure assets/tailwind.css exists and is large)
        document::Stylesheet { href: "assets/tailwind.css" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }

        // Page wrapper
        div { class: "{shell}",

            // Header with theme toggle
            Header { is_dark, on_toggle }

            main { id: "main", class: "max-w-5xl mx-auto px-4 pb-20",
                Hero {}

                section { id: "about", class: "pt-12 border-t {border_color}",
                    h2 { class: "text-xl font-semibold", "About" }
                    p  { class: "mt-2 opacity-80",
                        "Native Rust Engineer | Hospitality Professional | System & Operations Specialist"
                    }
                }

                section { id: "experience", class: "pt-12 border-t {border_color}",
                    h2 { class: "text-xl font-semibold", "Experience" }
                    ul { class: "mt-4 grid gap-4",
                        li { class: "rounded-lg border {border_color} p-4",
                            h3 { class: "font-semibold", "Cuvee — General Manager" }
                            p  { class: "text-sm opacity-70", "2022–2025" }
                            ul { class: "list-disc ml-5 mt-1",
                                li { "Grew retail to $1M in 3 years" }
                                li { "Led ecommerce + B2B; pipeline generation" }
                            }
                        }
                        li { class: "rounded-lg border {border_color} p-4",
                            h3 { class: "font-semibold", "Try Hard Coffee — Executive Chef" }
                            p  { class: "text-sm opacity-70", "Prior" }
                            ul { class: "list-disc ml-5 mt-1",
                                li { "Systems & Supplier Relationships" }
                                li { "Creating Systems and operations" }
                            }
                        }
                    }
                }

                section { id: "projects", class: "pt-12 border-t {border_color}",
                    h2 { class: "text-xl font-semibold", "Projects" }
                    div { class: "mt-4 grid grid-cols-1 sm:grid-cols-2 gap-4",
                        div { class: "rounded-lg border {border_color} p-4",
                            h3 { class: "font-semibold", "TREZZA, POS & Blockchain" }
                            p  { class: "text-sm opacity-80",
                                "Progressive Web App POS, Payment Transactions Router, Crypto token on Solana project."
                            }
                            a  { href: "#", class: "mt-2 inline-block text-blue-600 hover:underline", "View" }
                        }
                        div { class: "rounded-lg border {border_color} p-4",
                            h3 { class: "font-semibold", "Hologram Brick (MVP)" }
                            p  { class: "text-sm opacity-80",
                                "Tiny blockchain brick in Rust exploring lanes/throughput using embedded philosophy with a canonical, deterministic mindset."
                            }
                            a  { href: "#", class: "mt-2 inline-block text-blue-600 hover:underline", "View" }
                        }
                    }
                }

                section { id: "contact", class: "pt-12 border-t {border_color}",
                    h2 { class: "text-xl font-semibold", "Contact" }
                    p  { class: "mt-2 opacity-80", "Open to Rust roles" }
                    a  { href: "mailto:hello@angelmedina.io",
                        class: "mt-3 inline-block rounded-md bg-blue-600 text-white px-4 py-2
                                font-semibold hover:bg-blue-700 focus-visible:ring-2
                                focus-visible:ring-blue-500 outline-none",
                        "Email me"
                    }
                }
            }

            Footer {}
        }
    }
}
