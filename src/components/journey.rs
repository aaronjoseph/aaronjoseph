use std::collections::HashSet;

use dioxus::prelude::*;

use crate::data::ROLES;

#[component]
pub fn JourneySection() -> Element {
    let mut open_roles = use_signal(|| {
        ROLES
            .iter()
            .filter(|r| r.default_open)
            .map(|r| r.id)
            .collect::<HashSet<&'static str>>()
    });

    let all_open = open_roles.read().len() == ROLES.len();

    rsx! {
        section { id: "journey",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "eyebrow", "The Story So Far" }
                    h2 { "Nine years of figuring things out" }
                    p { class: "sec-sub",
                        "From early-stage rural fintech and pricing analytics to Rust-powered search and agentic AI."
                    }
                }

                div { class: "career-layout",
                    aside { class: "about-panel",
                        h3 { "A bit about how I work" }
                        p {
                            "My path into AI engineering wasn't a straight line. I started in decision science and fintech — building pricing models for Pfizer, conversion funnels at FundsIndia, and joining Seed-stage Kaleidofin as employee #70 to build risk analysis for rural India's underbanked savers."
                        }
                        p {
                            "Somewhere along the way, I fell in love with the unglamorous side of ML: why a ranking model takes 24 hours to retrain, why a search endpoint spikes to 5 seconds under load, or why a cluster is quietly burning budget on idle Python workers."
                        }
                        p {
                            "These days at Best Buy, I spend my time where machine learning meets systems engineering — rewriting hot search and UI paths in Rust (Axum & Dioxus), moving rankers to ONNX, and making agentic AI easy to trace and trust."
                        }
                    }

                    div {
                        div { class: "timeline-controls",
                            button {
                                class: "text-btn",
                                r#type: "button",
                                onclick: move |_| {
                                    if all_open {
                                        open_roles.write().clear();
                                    } else {
                                        let all: HashSet<&'static str> = ROLES.iter().map(|r| r.id).collect();
                                        open_roles.set(all);
                                    }
                                },
                                if all_open { "Collapse all roles" } else { "Expand all roles" }
                            }
                        }

                        div { class: "timeline",
                            for role in ROLES {
                                {
                                    let id = role.id;
                                    let is_open = open_roles.read().contains(id);
                                    rsx! {
                                        article {
                                            key: "{id}",
                                            class: if role.current { "role-card current" } else { "role-card" },
                                            button {
                                                class: "role-trigger",
                                                r#type: "button",
                                                aria_expanded: "{is_open}",
                                                onclick: move |_| {
                                                    let mut set = open_roles.write();
                                                    if set.contains(id) {
                                                        set.remove(id);
                                                    } else {
                                                        set.insert(id);
                                                    }
                                                },
                                                div {
                                                    div { class: "role-meta",
                                                        if let Some(badge) = role.badge {
                                                            span { class: "role-now", "{badge}" }
                                                        }
                                                        span { "{role.period} · {role.location}" }
                                                    }
                                                    h3 { class: "role-title", "{role.title}" }
                                                    div { class: "role-company", "{role.company}" }
                                                }
                                                span {
                                                    class: if is_open { "role-chevron open" } else { "role-chevron" },
                                                    aria_hidden: "true",
                                                    svg {
                                                        width: "15",
                                                        height: "15",
                                                        view_box: "0 0 24 24",
                                                        fill: "none",
                                                        stroke: "currentColor",
                                                        stroke_width: "2.2",
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        path { d: "m6 9 6 6 6-6" }
                                                    }
                                                }
                                            }

                                            if is_open {
                                                div { class: "role-body",
                                                    ul { class: "role-list",
                                                        for bullet in role.bullets {
                                                            li { "{bullet}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
