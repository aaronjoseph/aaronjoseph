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
                    div { class: "eyebrow", "Career Trajectory" }
                    h2 { "From decision science to Rust-powered AI" }
                    p { class: "sec-sub",
                        "Nine years progressing from analytics and financial risk modeling to high-scale search and agentic AI systems."
                    }
                }

                div { class: "career-layout",
                    aside { class: "about-panel",
                        h3 { "How I approach engineering" }
                        p {
                            "I started in decision science and fintech analytics — building competitive pricing dashboards for Pfizer, conversion funnels at FundsIndia, and joining Seed-stage Kaleidofin as employee #70 to protect rural India's underbanked savers."
                        }
                        p {
                            "Along the way, I saw that the hardest problems in applied ML rarely end at offline accuracy. They live in tail latency, noisy retries, runaway cloud bills, slow retraining loops, and how quickly a team can trust a model change."
                        }
                        p {
                            "Today at Best Buy, I combine ML engineering with systems programming — moving critical search and UI paths to Rust (Axum & Dioxus), compiling rankers to ONNX, and making agentic AI observable and testable by default."
                        }
                        div { class: "coffee-note",
                            "\"Started my DS career at MuSigma — don't ask me how I circumvented the 3-year bond, or maybe do ask, over coffee.\""
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
