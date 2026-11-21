use dioxus::prelude::*;

use crate::data::{BLOG_POSTS, GITHUB_URL, LINKEDIN_URL, PLANNED_BLOG_TOPICS, X_URL};

#[derive(Clone, Copy, PartialEq, Eq)]
enum BlogLayoutMode {
    Timeline,
    ByTopic,
}

/// Compact Eugene Yan-style "Writing" preview strip on the homepage that opens the dedicated Blog page.
#[component]
pub fn WritingPreviewSection(on_open_blog: EventHandler<()>) -> Element {
    rsx! {
        section { id: "writing-preview",
            div { class: "wrap",
                div { class: "latest-header",
                    div {
                        div { class: "eyebrow", "Writing & Notes" }
                        h2 { "Latest from the blog" }
                    }
                    button {
                        class: "latest-all-btn",
                        r#type: "button",
                        onclick: move |_| on_open_blog.call(()),
                        if BLOG_POSTS.is_empty() {
                            "Open Writing page →"
                        } else {
                            "All {BLOG_POSTS.len()} posts →"
                        }
                    }
                }

                if BLOG_POSTS.is_empty() {
                    div { class: "latest-list",
                        for item in PLANNED_BLOG_TOPICS.iter().take(3) {
                            button {
                                key: "{item.title}",
                                class: "latest-row",
                                r#type: "button",
                                onclick: move |_| on_open_blog.call(()),
                                span { class: "latest-date", "{item.date_label}" }
                                div { class: "latest-content",
                                    div { class: "latest-title", "{item.title}" }
                                    p { class: "latest-sub", "{item.subtitle}" }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "latest-list",
                        for post in BLOG_POSTS.iter().take(4) {
                            button {
                                key: "{post.slug}",
                                class: "latest-row",
                                r#type: "button",
                                onclick: move |_| on_open_blog.call(()),
                                span { class: "latest-date", "{post.date}" }
                                div { class: "latest-content",
                                    div { class: "latest-title", "{post.title}" }
                                    p { class: "latest-sub", "{post.subtitle}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Dedicated standalone Blog / Writing page with:
/// 1) Chronological Timeline (timestamp on the left, newest first — inspired by iamtrask / eugeneyan / mccormickml)
/// 2) By-Topic Indented Tree Index (grouped by subject with vertical guides & year)
/// 3) Full distraction-free serif Article Reader
#[component]
pub fn BlogView(on_back: EventHandler<()>) -> Element {
    let mut layout_mode = use_signal(|| BlogLayoutMode::Timeline);
    let mut active_slug = use_signal(|| Option::<&'static str>::None);

    let active_post = active_slug().and_then(|slug| BLOG_POSTS.iter().find(|p| p.slug == slug));

    // Unique topic groups in insertion order
    let mut published_topics: Vec<&'static str> = Vec::new();
    for post in BLOG_POSTS {
        if !published_topics.contains(&post.topic) {
            published_topics.push(post.topic);
        }
    }

    let mut planned_topics: Vec<&'static str> = Vec::new();
    for item in PLANNED_BLOG_TOPICS {
        if !planned_topics.contains(&item.topic) {
            planned_topics.push(item.topic);
        }
    }

    rsx! {
        div { class: "blog-page",
            div { class: "blog-wrap",
                if let Some(post) = active_post {
                    // Distraction-free serif Article Reader (Eugene Yan-inspired typography)
                    div { class: "blog-top-bar",
                        button {
                            class: "text-btn",
                            r#type: "button",
                            onclick: move |_| active_slug.set(None),
                            "← All posts"
                        }
                        span { class: "blog-meta-mono", "{post.date} · {post.read_time}" }
                    }

                    article { class: "article-shell",
                        header { class: "article-head",
                            div { class: "article-topic", "{post.topic}" }
                            h1 { class: "article-title", "{post.title}" }
                            p { class: "article-subtitle", "{post.subtitle}" }
                            div { class: "chip-row",
                                for tag in post.tags {
                                    span { class: "chip", key: "{tag}", "{tag}" }
                                }
                            }
                        }

                        div { class: "article-prose",
                            for para in post.paragraphs {
                                p { "{para}" }
                            }
                        }
                    }
                } else {
                    // Blog Index Header
                    div { class: "blog-top-bar",
                        button {
                            class: "text-btn",
                            r#type: "button",
                            onclick: move |_| on_back.call(()),
                            "← Back home"
                        }

                        div { class: "blog-mode-switch", role: "tablist", aria_label: "Blog layout",
                            button {
                                class: if layout_mode() == BlogLayoutMode::Timeline { "mode-pill active" } else { "mode-pill" },
                                r#type: "button",
                                role: "tab",
                                aria_selected: "{layout_mode() == BlogLayoutMode::Timeline}",
                                onclick: move |_| layout_mode.set(BlogLayoutMode::Timeline),
                                "Latest (Timeline)"
                            }
                            button {
                                class: if layout_mode() == BlogLayoutMode::ByTopic { "mode-pill active" } else { "mode-pill" },
                                r#type: "button",
                                role: "tab",
                                aria_selected: "{layout_mode() == BlogLayoutMode::ByTopic}",
                                onclick: move |_| layout_mode.set(BlogLayoutMode::ByTopic),
                                "By Topic"
                            }
                        }
                    }

                    header { class: "blog-header",
                        h1 {
                            "Writing & "
                            span { class: "hl-mark", "Engineering Notes" }
                        }
                        p { class: "blog-intro",
                            "A quiet notebook on production search, Rust for ML serving, learning-to-rank performance, and evaluating agentic AI systems. New posts land at the top with their timestamp on the left."
                        }
                    }

                    if BLOG_POSTS.is_empty() {
                        div { class: "blog-draft-banner",
                            div { class: "blog-draft-badge",
                                span { class: "pulse-dot", aria_hidden: "true" }
                                span { "Just getting started · 0 published posts yet" }
                            }
                            p {
                                "First write-ups are in progress. Below is the exact layout where new posts will appear as they go live — feel free to connect on "
                                a { href: "{LINKEDIN_URL}", target: "_blank", rel: "noopener noreferrer", class: "inline-hl", "LinkedIn" }
                                ", "
                                a { href: "{GITHUB_URL}", target: "_blank", rel: "noopener noreferrer", class: "inline-hl", "GitHub" }
                                ", or "
                                a { href: "{X_URL}", target: "_blank", rel: "noopener noreferrer", class: "inline-hl", "X" }
                                " in the meantime."
                            }
                        }

                        if layout_mode() == BlogLayoutMode::Timeline {
                            div { class: "blog-timeline-list",
                                for item in PLANNED_BLOG_TOPICS {
                                    div { class: "blog-tl-row is-draft", key: "{item.title}",
                                        div { class: "blog-tl-date", "{item.date_label}" }
                                        div { class: "blog-tl-body",
                                            div { class: "blog-tl-topic", "{item.topic}" }
                                            h2 { class: "blog-tl-title", "{item.title}" }
                                            p { class: "blog-tl-sub", "{item.subtitle}" }
                                        }
                                    }
                                }
                            }
                        } else {
                            // Hierarchical indented Topic Tree (inspired by classic scholarly CS blogs)
                            div { class: "topic-tree",
                                div { class: "tree-root-label", "Engineering Notes (Drafts)" }
                                div { class: "tree-branch",
                                    for group in planned_topics {
                                        div { class: "tree-group", key: "{group}",
                                            div { class: "tree-group-title", "{group}" }
                                            div { class: "tree-items",
                                                for item in PLANNED_BLOG_TOPICS.iter().filter(|t| t.topic == group) {
                                                    div { class: "tree-item is-draft", key: "{item.title}",
                                                        span { class: "tree-item-title", "{item.title}" }
                                                        span { class: "tree-item-year", " ({item.year})" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else if layout_mode() == BlogLayoutMode::Timeline {
                        div { class: "blog-timeline-list",
                            for post in BLOG_POSTS {
                                {
                                    let slug = post.slug;
                                    rsx! {
                                        button {
                                            key: "{slug}",
                                            class: "blog-tl-row clickable",
                                            r#type: "button",
                                            onclick: move |_| {
                                                active_slug.set(Some(slug));
                                                document::eval("window.scrollTo({ top: 0, behavior: 'instant' });");
                                            },
                                            div { class: "blog-tl-date", "{post.date}" }
                                            div { class: "blog-tl-body",
                                                div { class: "blog-tl-topic", "{post.topic} · {post.read_time}" }
                                                h2 { class: "blog-tl-title", "{post.title}" }
                                                p { class: "blog-tl-sub", "{post.subtitle}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div { class: "topic-tree",
                            div { class: "tree-root-label", "All Articles" }
                            div { class: "tree-branch",
                                for group in published_topics {
                                    div { class: "tree-group", key: "{group}",
                                        div { class: "tree-group-title", "{group}" }
                                        div { class: "tree-items",
                                            for post in BLOG_POSTS.iter().filter(|p| p.topic == group) {
                                                {
                                                    let slug = post.slug;
                                                    rsx! {
                                                        button {
                                                            key: "{slug}",
                                                            class: "tree-item clickable",
                                                            r#type: "button",
                                                            onclick: move |_| {
                                                                active_slug.set(Some(slug));
                                                                document::eval("window.scrollTo({ top: 0, behavior: 'instant' });");
                                                            },
                                                            span { class: "tree-item-title", "{post.title}" }
                                                            span { class: "tree-item-year", " ({post.year})" }
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
