use dioxus::prelude::*;

use crate::data::IMPACT_CARDS;

#[component]
pub fn ImpactSection() -> Element {
    rsx! {
        section { id: "impact",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "eyebrow", "Measured Outcomes" }
                    h2 { "Before & after, in production" }
                    p { class: "sec-sub",
                        "Engineering gains measured in milliseconds shaved, infrastructure retired, and validation cycles compressed."
                    }
                }

                div { class: "bento-grid",
                    for card in IMPACT_CARDS {
                        article {
                            key: "{card.title}",
                            class: if card.wide { "bento-card wide" } else { "bento-card" },
                            div {
                                div { class: "bento-tag",
                                    span { "{card.tag}" }
                                    span { class: "bento-org", "{card.org}" }
                                }
                                div { class: "bento-num", "{card.headline}" }
                                h3 { class: "bento-title", "{card.title}" }
                                p { class: "bento-desc", "{card.desc}" }
                            }

                            if let Some(ref meter) = card.meter {
                                div { class: "delta-meter",
                                    div { class: "meter-row",
                                        span { class: "meter-lbl", "Before" }
                                        div { class: "meter-track",
                                            div {
                                                class: "meter-fill before",
                                                style: "width: {meter.before_pct}%;",
                                            }
                                        }
                                        span { class: "meter-val", "{meter.before_label}" }
                                    }
                                    div { class: "meter-row",
                                        span { class: "meter-lbl", "After" }
                                        div { class: "meter-track",
                                            div {
                                                class: if meter.accent_after { "meter-fill after-accent" } else { "meter-fill after" },
                                                style: "width: {meter.after_pct}%;",
                                            }
                                        }
                                        span { class: "meter-val good", "{meter.after_label}" }
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
