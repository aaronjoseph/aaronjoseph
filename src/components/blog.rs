use dioxus::prelude::*;

use crate::data::{BLOG_POSTS, GITHUB_URL, LINKEDIN_URL, PLANNED_BLOG_TOPICS, X_URL};

#[component]
pub fn BlogSection() -> Element {
    let mut active_slug = use_signal(|| Option::<&'static str>::None);

    rsx! {
        section { id: "blog",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "eyebrow", "Writing & Notes" }
                    h2 { "Engineering blog" }
                    p { class: "sec-sub",
                        "Deep dives on production search, Rust for ML serving, learning-to-rank performance, and agentic AI observability."
                    }
                }

                if BLOG_POSTS.is_empty() {
                    div { class: "blog-empty-card",
                        div { class: "blog-empty-top",
                            span { class: "blog-status-pill",
                                span { class: "pulse-dot", aria_hidden: "true" }
                                span { "FIRST POSTS IN PROGRESS" }
                            }
                            span { class: "blog-count-meta", "0 published · drafts queued" }
                        }

                        div { class: "blog-empty-grid",
                            div {
                                h3 { "Notes from production systems — coming soon." }
                                p {
                                    "I'm starting this space to document architecture lessons, latency benchmarks, and practical trade-offs from running real-world search, ranking, and LLM systems. No fluff — just concrete write-ups."
                                }
                                div { class: "blog-follow-row",
                                    span { class: "blog-follow-lbl", "Follow updates on:" }
                                    a { href: "{LINKEDIN_URL}", target: "_blank", rel: "noopener noreferrer", class: "blog-follow-link", "LinkedIn ↗" }
                                    a { href: "{GITHUB_URL}", target: "_blank", rel: "noopener noreferrer", class: "blog-follow-link", "GitHub ↗" }
                                    a { href: "{X_URL}", target: "_blank", rel: "noopener noreferrer", class: "blog-follow-link", "X ↗" }
                                }
                            }

                            div { class: "blog-topics-box",
                                div { class: "blog-topics-head", "PLANNED WRITE-UPS" }
                                ul { class: "blog-topics-list",
                                    for (category, topic) in PLANNED_BLOG_TOPICS {
                                        li { key: "{topic}",
                                            span { class: "blog-topic-cat", "{category}" }
                                            span { class: "blog-topic-title", "{topic}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "blog-grid",
                        for post in BLOG_POSTS {
                            {
                                let slug = post.slug;
                                let is_open = active_slug() == Some(slug);
                                rsx! {
                                    article { class: "blog-card", key: "{slug}",
                                        div { class: "blog-card-meta",
                                            span { "{post.date}" }
                                            span { "·" }
                                            span { "{post.read_time}" }
                                        }
                                        h3 { "{post.title}" }
                                        p { "{post.summary}" }
                                        div { class: "chip-row",
                                            for tag in post.tags {
                                                span { class: "chip", key: "{tag}", "{tag}" }
                                            }
                                        }
                                        button {
                                            class: "text-btn blog-read-btn",
                                            r#type: "button",
                                            onclick: move |_| {
                                                if is_open {
                                                    active_slug.set(None);
                                                } else {
                                                    active_slug.set(Some(slug));
                                                }
                                            },
                                            if is_open { "Close article ↑" } else { "Read article →" }
                                        }
                                        if is_open {
                                            div { class: "blog-reader",
                                                for para in post.paragraphs {
                                                    p { "{para}" }
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
