use dioxus::prelude::*;

use crate::data::{MetricTone, TELEMETRY_TABS};

#[component]
pub fn Hero(
    on_open_resume: EventHandler<()>,
    on_copy_email: EventHandler<()>,
) -> Element {
    let mut active_tab = use_signal(|| 0usize);
    let current = &TELEMETRY_TABS[active_tab()];

    rsx! {
        section { class: "hero", id: "top",
            div { class: "wrap hero-grid",
                div {
                    div { class: "status-pill",
                        span { class: "pulse-dot", aria_hidden: "true" }
                        span { "Senior AI Engineer @ Best Buy · Bengaluru" }
                    }

                    h1 {
                        "Search & AI systems engineered for "
                        span { class: "grad-text", "real-world latency." }
                    }

                    p { class: "hero-lead",
                        "I'm "
                        strong { "Aaron Joseph Mathew" }
                        " — a Senior AI Engineer with "
                        strong { "9+ years" }
                        " across analytics, machine learning, and production AI. I build and harden high-throughput search, ranking, and agentic platforms where milliseconds and cloud spend actually matter."
                    }

                    ul { class: "hero-pillars",
                        li {
                            span { class: "pillar-marker", "01/" }
                            span {
                                strong { "Low-Latency Search & Ranking" }
                                " — semantic search, Learning to Rank (ONNX/XGBoost), NER & ESCI serving up to 12,000 RPS."
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
                            span { "Printable Résumé" }
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

                aside { class: "telemetry-card", aria_label: "Production systems benchmarks",
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

                    div { class: "telemetry-tabs", role: "tablist", aria_label: "System benchmarks",
                        for (idx, tab) in TELEMETRY_TABS.iter().enumerate() {
                            button {
                                key: "{tab.id}",
                                class: if active_tab() == idx { "t-tab active" } else { "t-tab" },
                                r#type: "button",
                                role: "tab",
                                aria_selected: "{active_tab() == idx}",
                                onclick: move |_| active_tab.set(idx),
                                "{tab.label}"
                            }
                        }
                    }

                    div { class: "telemetry-body", key: "{current.id}",
                        div { class: "t-arch",
                            span { "{current.arch_prefix}" }
                            span { class: "hl", "{current.arch_highlight}" }
                            span { "{current.arch_mid}" }
                            span { class: "ok", "{current.arch_signal}" }
                        }

                        div { class: "t-metrics",
                            for m in current.metrics {
                                div { class: "t-metric", key: "{m.label}",
                                    div { class: "t-metric-k", "{m.label}" }
                                    div {
                                        class: match m.tone {
                                            MetricTone::Signal => "t-metric-v signal",
                                            MetricTone::Accent => "t-metric-v accent",
                                            MetricTone::Default => "t-metric-v",
                                        },
                                        "{m.value}"
                                    }
                                    div { class: "t-metric-d", "{m.detail}" }
                                }
                            }
                        }

                        p { class: "t-note", "{current.note}" }
                    }

                    div { class: "telemetry-foot",
                        span { "Interactive telemetry · built in Rust/Dioxus" }
                        span { "GCP · GKE · Rust · Python" }
                    }
                }
            }
        }
    }
}
