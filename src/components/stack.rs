use dioxus::prelude::*;

use crate::data::SKILL_GROUPS;

#[component]
pub fn StackSection() -> Element {
    let mut active_filter = use_signal(|| "all");

    rsx! {
        section { id: "stack",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "eyebrow", "Toolbox" }
                    h2 { "What I reach for day to day" }
                    p { class: "sec-sub",
                        "The languages, frameworks, and cloud tools I spend the most time with — filter by area if you're looking for something specific."
                    }
                }

                div { class: "filter-bar", role: "group", aria_label: "Filter stack categories",
                    button {
                        class: if active_filter() == "all" { "filter-pill active" } else { "filter-pill" },
                        r#type: "button",
                        onclick: move |_| active_filter.set("all"),
                        "All Layers"
                    }
                    for group in SKILL_GROUPS {
                        {
                            let gid = group.id;
                            rsx! {
                                button {
                                    key: "{gid}",
                                    class: if active_filter() == gid { "filter-pill active" } else { "filter-pill" },
                                    r#type: "button",
                                    onclick: move |_| active_filter.set(gid),
                                    "{group.filter_label}"
                                }
                            }
                        }
                    }
                }

                div { class: "stack-grid",
                    for group in SKILL_GROUPS {
                        {
                            let dimmed = active_filter() != "all" && active_filter() != group.id;
                            rsx! {
                                div {
                                    key: "{group.id}",
                                    class: if dimmed { "stack-card dimmed" } else { "stack-card" },
                                    div { class: "stack-head",
                                        h3 { "{group.title}" }
                                        span { class: "stack-count", "{group.items.len():02}" }
                                    }
                                    div { class: "chip-row",
                                        for (skill, is_core) in group.items {
                                            span {
                                                key: "{skill}",
                                                class: if *is_core { "chip core" } else { "chip" },
                                                "{skill}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "stack-legend",
                    span {
                        span { class: "legend-dot" }
                        "Highlighted = daily core production stack"
                    }
                }
            }
        }
    }
}
