use dioxus::prelude::*;

use crate::data::CASE_STUDIES;

#[component]
pub fn SystemsSection() -> Element {
    rsx! {
        section { id: "systems",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "eyebrow", "Architecture Deep Dives" }
                    h2 { "Systems I've designed & shipped" }
                    p { class: "sec-sub",
                        "A closer look at the architectural decisions behind my production search, ranking, and AI reliability work."
                    }
                }

                div { class: "case-grid",
                    for cs in CASE_STUDIES {
                        article { class: "case-card", key: "{cs.title}",
                            div { class: "case-top",
                                span { class: "case-kicker", "{cs.kicker}" }
                                span { class: "case-place", "{cs.place}" }
                            }
                            h3 { "{cs.title}" }
                            p { "{cs.body}" }
                            div { class: "case-flow", "{cs.flow}" }
                            div { class: "chip-row",
                                for (label, variant) in cs.chips {
                                    span {
                                        key: "{label}",
                                        class: if variant.is_empty() { "chip".to_string() } else { format!("chip {variant}") },
                                        "{label}"
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
