use dioxus::prelude::*;

use crate::components::company_logo::CompanyLogo;

#[component]
pub fn Hero(
    on_open_resume: EventHandler<()>,
    on_open_blog: EventHandler<()>,
    on_copy_email: EventHandler<()>,
) -> Element {
    rsx! {
        section { class: "hero", id: "top",
            div { class: "wrap",
                // Playful Editorial Top Intro
                div { class: "hero-editorial",
                    h1 { class: "hero-title",
                        "Hi, I'm "
                        span { class: "squiggle-name",
                            "Aaron Joseph Mathew."
                            svg {
                                class: "squiggle-svg",
                                view_box: "0 0 320 14",
                                fill: "none",
                                preserve_aspect_ratio: "none",
                                "aria-hidden": "true",
                                path {
                                    d: "M3 9.5C34 2.5 64 13 96 7.5C128 2 162 12.5 196 7C230 1.5 264 12 317 6",
                                    stroke: "#f08b6d",
                                    stroke_width: "4.5",
                                    stroke_linecap: "round",
                                }
                            }
                        }
                    }

                    div { class: "hero-badge-row",
                        div { class: "status-pill",
                            span { class: "pulse-dot", aria_hidden: "true" }
                            span { "Currently at" }
                            CompanyLogo { company_id: "bestbuy", size: 20 }
                            span { "Best Buy · Bengaluru, India" }
                        }
                    }

                    p { class: "hero-serif-lead",
                        "I'm an AI & search engineer who enjoys taking slow, expensive ML systems and making them feel effortless — whether that means rewriting a search path in Rust, tracking down a stubborn 5-second GPU spike, or keeping things steady at "
                        strong { "12,000 requests a second" }
                        "."
                    }

                    p { class: "hero-inline-nav",
                        "Over the last nine years, I've gone from rural fintech and risk analytics to high-scale search, ranking, and agentic AI. Feel free to browse "
                        a { href: "#systems", class: "inline-hl hl-yellow", "what I've built" }
                        ", read "
                        button {
                            class: "inline-hl-btn hl-lime",
                            r#type: "button",
                            onclick: move |_| on_open_blog.call(()),
                            "my notebook"
                        }
                        ", grab "
                        button {
                            class: "inline-hl-btn hl-lavender",
                            r#type: "button",
                            onclick: move |_| on_open_resume.call(()),
                            "my résumé"
                        }
                        ", or "
                        a { href: "#contact", class: "inline-hl hl-peach", "write to me" }
                        " anytime."
                    }

                    div { class: "hero-cta-row",
                        a { class: "btn btn-primary", href: "#contact",
                            span { "Say hello" }
                            svg {
                                width: "16",
                                height: "16",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.4",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M5 12h14m-6-6 6 6-6 6" }
                            }
                        }
                        button {
                            class: "btn btn-ghost",
                            r#type: "button",
                            onclick: move |_| on_open_resume.call(()),
                            svg {
                                width: "15",
                                height: "15",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.1",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                                path { d: "M14 2v6h6M16 13H8M16 17H8M10 9H8" }
                            }
                            span { "Résumé (Styled / ATS)" }
                        }
                        button {
                            class: "btn btn-peach",
                            r#type: "button",
                            onclick: move |_| on_copy_email.call(()),
                            svg {
                                width: "15",
                                height: "15",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.1",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                rect { width: "14", height: "14", x: "8", y: "8", rx: "2" }
                                path { d: "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" }
                            }
                            span { "Copy email" }
                        }
                    }
                }

                // Playful Spring Sticker Stats Strip
                div { class: "sticker-stats",
                    div { class: "sticker-stat s-yellow",
                        div { class: "qstat-val", "9+ yrs" }
                        div { class: "qstat-lbl", "ML & AI Engineering" }
                    }
                    div { class: "sticker-stat s-mint",
                        div { class: "qstat-val", "12,000" }
                        div { class: "qstat-lbl", "Peak Search RPS" }
                    }
                    div { class: "sticker-stat s-peach",
                        div { class: "qstat-val", "95%" }
                        div { class: "qstat-lbl", "Compute cost cut via Rust migration" }
                    }
                    div { class: "sticker-stat s-lavender",
                        div { class: "qstat-val", "50%" }
                        div { class: "qstat-lbl", "Search P50 latency cut (700→350 ms)" }
                    }
                }
            }
        }
    }
}
