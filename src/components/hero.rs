use dioxus::prelude::*;

use crate::data::{MetricTone, TELEMETRY_PROJECTS};

#[component]
pub fn Hero(
    on_open_resume: EventHandler<()>,
    on_copy_email: EventHandler<()>,
) -> Element {
    let mut active_project = use_signal(|| 0usize);
    let current = &TELEMETRY_PROJECTS[active_project()];

    rsx! {
        section { class: "hero", id: "top",
            div { class: "wrap hero-grid",
                div {
                    div { class: "status-pill",
                        span { class: "pulse-dot", aria_hidden: "true" }
                        span { "Senior AI Engineer @ Best Buy · Bengaluru" }
                    }

                    h1 {
                        "Search & AI systems built for "
                        span { class: "grad-text", "high throughput & low latency." }
                    }

                    p { class: "hero-lead",
                        "I'm "
                        strong { "Aaron Joseph Mathew" }
                        " — a Senior AI Engineer with "
                        strong { "9+ years" }
                        " across analytics, machine learning, and production AI. I architect and harden production search, ranking, and agentic platforms handling up to "
                        strong { "12,000 RPS" }
                        " — where throughput, tail latency, and cloud spend all matter."
                    }

                    ul { class: "hero-pillars",
                        li {
                            span { class: "pillar-marker", "01/" }
                            span {
                                strong { "High-Throughput, Low-Latency Search" }
                                " — semantic search, Learning to Rank (ONNX/XGBoost), NER & ESCI scaled up to 12,000 RPS with RPS-aware HPA."
                            }
                        }
                        li {
                            span { class: "pillar-marker", "02/" }
                            span {
                                strong { "Python to Rust Systems Engineering" }
                                " — migrating bottlenecked ML services to Rust (Axum, Dioxus) and hexagonal architectures."
                            }
                        }
                        li {
                            span { class: "pillar-marker", "03/" }
                            span {
                                strong { "Agentic Observability & Eval" }
                                " — LLM-as-a-judge regression suites, Cloud Trace diagnostics, and Gemma vs. Gemini benchmarking."
                            }
                        }
                    }

                    div { class: "hero-cta-row",
                        a { class: "btn btn-primary", href: "#contact",
                            span { "Let's talk" }
                            svg {
                                width: "16",
                                height: "16",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.2",
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
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                                path { d: "M14 2v6h6M16 13H8M16 17H8M10 9H8" }
                            }
                            span { "Résumé (Styled / ATS)" }
                        }
                        button {
                            class: "btn btn-ghost",
                            r#type: "button",
                            onclick: move |_| on_copy_email.call(()),
                            svg {
                                width: "15",
                                height: "15",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                rect { width: "14", height: "14", x: "8", y: "8", rx: "2" }
                                path { d: "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" }
                            }
                            span { "Copy email" }
                        }
                    }

                    div { class: "hero-quick-stats",
                        div {
                            div { class: "qstat-val", "9+ yrs" }
                            div { class: "qstat-lbl", "ML & AI Eng" }
                        }
                        div {
                            div { class: "qstat-val", "12,000" }
                            div { class: "qstat-lbl", "Peak RPS" }
                        }
                        div {
                            div { class: "qstat-val", "2 ms" }
                            div { class: "qstat-lbl", "LTR ONNX P50" }
                        }
                        div {
                            div { class: "qstat-val", "MS" }
                            div { class: "qstat-lbl", "Georgia Tech" }
                        }
                    }
                }

                aside { class: "telemetry-card", aria_label: "Production systems impact breakdown",
                    div { class: "telemetry-head",
                        div { class: "telemetry-title",
                            svg {
                                width: "14",
                                height: "14",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M3 12h4l3-9 4 18 3-9h4" }
                            }
                            span { "production-telemetry.log" }
                        }
                        span { class: "telemetry-badge", "VERIFIED IMPACT" }
                    }

                    div { class: "telemetry-tabs", role: "tablist", aria_label: "Production projects",
                        for (idx, project) in TELEMETRY_PROJECTS.iter().enumerate() {
                            button {
                                key: "{project.id}",
                                class: if active_project() == idx { "t-tab active" } else { "t-tab" },
                                r#type: "button",
                                role: "tab",
                                aria_selected: "{active_project() == idx}",
                                onclick: move |_| active_project.set(idx),
                                "{project.label}"
                            }
                        }
                    }

                    div { class: "telemetry-body", key: "{current.id}",
                        div { class: "t-context", "{current.context}" }

                        div { class: "t-streams",
                            for ws in current.workstreams {
                                div { class: "t-stream", key: "{ws.index}-{ws.title}",
                                    div { class: "t-stream-head",
                                        span { class: "t-stream-idx", "{ws.index}" }
                                        div { class: "t-stream-titles",
                                            div { class: "t-stream-title", "{ws.title}" }
                                            div { class: "t-stream-pipe", "{ws.pipeline}" }
                                        }
                                    }
                                    p { class: "t-stream-why", "{ws.advantage}" }
                                    div { class: "t-gains",
                                        for (label, tone) in ws.gains {
                                            span {
                                                key: "{label}",
                                                class: match tone {
                                                    MetricTone::Signal => "t-gain signal",
                                                    MetricTone::Accent => "t-gain accent",
                                                    MetricTone::Default => "t-gain",
                                                },
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "telemetry-foot",
                        span { "{current.stack_summary}" }
                        span { "Rust + Dioxus WASM" }
                    }
                }
            }
        }
    }
}
