use dioxus::prelude::*;

use crate::components::systems::{NotebookFieldNotes, RichTitle};
use crate::data::{ArticleBlock, BLOG_POSTS, PLANNED_BLOG_TOPICS};

#[derive(Clone, Copy, PartialEq, Eq)]
enum BlogLayoutMode {
    Timeline,
    ByTopic,
}

/// Handcrafted ruled-notebook SVG sketch of the hexagonal Rust monolith & bulkhead pools.
#[component]
fn HexagonalNotebookSketch() -> Element {
    rsx! {
        div { class: "nb-sheet",
            div { class: "nb-sheet-head",
                span { class: "nb-stamp", "Hexagonal Architecture Sketch" }
            }
            svg {
                class: "nb-sketch-svg",
                view_box: "0 0 640 296",
                fill: "none",
                role: "img",
                "aria-label": "Notebook sketch showing Incoming Request flowing into Axum HTTP api layer, calling the hexagonal domain core, with four isolated infra bulkhead adapter pools pointing inward",

                // Outer dashed boundary: Single Rust Binary
                rect {
                    x: "8",
                    y: "14",
                    width: "624",
                    height: "248",
                    rx: "18",
                    fill: "rgba(250, 248, 243, 0.72)",
                    stroke: "#848292",
                    stroke_width: "1.4",
                    stroke_dasharray: "6 5",
                }
                rect {
                    x: "24",
                    y: "4",
                    width: "212",
                    height: "20",
                    rx: "10",
                    fill: "#f2e8cd",
                    stroke: "rgba(50, 50, 61, 0.12)",
                }
                text {
                    x: "130",
                    y: "17.5",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-mono)",
                    font_size: "9.2",
                    font_weight: "700",
                    "SINGLE RUST BINARY (IN-PROCESS)"
                }

                // 1. Incoming Request pill
                rect {
                    x: "24",
                    y: "108",
                    width: "96",
                    height: "54",
                    rx: "14",
                    fill: "#f3e7e1",
                    stroke: "#d97c60",
                    stroke_width: "1.6",
                }
                text {
                    x: "72",
                    y: "131",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "11.5",
                    font_weight: "700",
                    "Incoming"
                }
                text {
                    x: "72",
                    y: "147",
                    text_anchor: "middle",
                    fill: "#5e5d6b",
                    font_family: "var(--font-mono)",
                    font_size: "10",
                    font_weight: "600",
                    "Request"
                }

                // Arrow: Request -> api
                path {
                    d: "M 120 135 C 132 134, 136 136, 146 135",
                    stroke: "#d97c60",
                    stroke_width: "2",
                    stroke_linecap: "round",
                }
                path {
                    d: "M 141 130 L 148 135 L 141 140",
                    stroke: "#d97c60",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }

                // 2. API Layer box
                rect {
                    x: "150",
                    y: "86",
                    width: "118",
                    height: "98",
                    rx: "15",
                    fill: "#faf8f3",
                    stroke: "#32323d",
                    stroke_width: "1.6",
                }
                rect { x: "162", y: "96", width: "44", height: "18", rx: "9", fill: "#f3e3db" }
                text {
                    x: "184",
                    y: "108",
                    text_anchor: "middle",
                    fill: "#b44a2f",
                    font_family: "var(--font-mono)",
                    font_size: "9.5",
                    font_weight: "700",
                    "api"
                }
                text {
                    x: "162",
                    y: "132",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "12",
                    font_weight: "700",
                    "Axum HTTP"
                }
                text { x: "162", y: "150", fill: "#5e5d6b", font_family: "var(--font-sans)", font_size: "10", "• Validate DTOs" }
                text { x: "162", y: "165", fill: "#5e5d6b", font_family: "var(--font-sans)", font_size: "10", "• Map response" }

                // Arrow: api -> domain
                path {
                    d: "M 268 135 C 282 134, 288 136, 302 135",
                    stroke: "#3f7c66",
                    stroke_width: "2.1",
                    stroke_linecap: "round",
                }
                path {
                    d: "M 296 130 L 303 135 L 296 140",
                    stroke: "#3f7c66",
                    stroke_width: "2.1",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                text {
                    x: "285",
                    y: "125",
                    text_anchor: "middle",
                    fill: "#3f7c66",
                    font_family: "var(--font-mono)",
                    font_size: "8.5",
                    font_weight: "700",
                    "calls"
                }

                // 3. DOMAIN Core Hexagon
                polygon {
                    points: "324,58 430,58 454,135 430,212 324,212 304,135",
                    fill: "#e1ede7",
                    stroke: "#3f7c66",
                    stroke_width: "2",
                }
                rect {
                    x: "344",
                    y: "72",
                    width: "66",
                    height: "19",
                    rx: "9.5",
                    fill: "#faf8f3",
                    stroke: "rgba(63, 124, 102, 0.35)",
                }
                text {
                    x: "377",
                    y: "84.5",
                    text_anchor: "middle",
                    fill: "#2c6350",
                    font_family: "var(--font-mono)",
                    font_size: "9.5",
                    font_weight: "700",
                    "domain"
                }
                text {
                    x: "377",
                    y: "111",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "12",
                    font_weight: "700",
                    "Search & Rank"
                }
                text {
                    x: "377",
                    y: "127",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "11.5",
                    font_weight: "700",
                    "Orchestration"
                }
                line {
                    x1: "330",
                    y1: "140",
                    x2: "424",
                    y2: "140",
                    stroke: "rgba(63, 124, 102, 0.3)",
                    stroke_dasharray: "3 3",
                }
                text {
                    x: "377",
                    y: "158",
                    text_anchor: "middle",
                    fill: "#2c6350",
                    font_family: "var(--font-mono)",
                    font_size: "9",
                    font_weight: "600",
                    "Ports & Traits"
                }
                text { x: "377", y: "174", text_anchor: "middle", fill: "#5e5d6b", font_family: "var(--font-sans)", font_size: "9.5", "Typed structs only" }
                text { x: "377", y: "189", text_anchor: "middle", fill: "#5e5d6b", font_family: "var(--font-sans)", font_size: "9.5", "Zero infra imports" }

                // Inward dashed arrows: infra -> domain
                path { d: "M 488 56 C 472 58, 464 78, 449 98", stroke: "#7b7296", stroke_width: "1.8", stroke_dasharray: "4 3", stroke_linecap: "round" }
                path { d: "M 455 97 L 448 99 L 449 92", stroke: "#7b7296", stroke_width: "1.8", stroke_linecap: "round", stroke_linejoin: "round" }

                path { d: "M 488 108 C 474 110, 466 118, 455 126", stroke: "#7b7296", stroke_width: "1.8", stroke_dasharray: "4 3", stroke_linecap: "round" }
                path { d: "M 462 126 L 454 127 L 457 120", stroke: "#7b7296", stroke_width: "1.8", stroke_linecap: "round", stroke_linejoin: "round" }

                path { d: "M 488 162 C 474 160, 466 152, 455 144", stroke: "#7b7296", stroke_width: "1.8", stroke_dasharray: "4 3", stroke_linecap: "round" }
                path { d: "M 457 150 L 454 143 L 462 144", stroke: "#7b7296", stroke_width: "1.8", stroke_linecap: "round", stroke_linejoin: "round" }

                path { d: "M 488 214 C 472 212, 464 192, 449 172", stroke: "#7b7296", stroke_width: "1.8", stroke_dasharray: "4 3", stroke_linecap: "round" }
                path { d: "M 449 179 L 448 171 L 455 173", stroke: "#7b7296", stroke_width: "1.8", stroke_linecap: "round", stroke_linejoin: "round" }

                // 4. INFRA Bulkhead Pools
                rect { x: "490", y: "32", width: "126", height: "44", rx: "11", fill: "#eae6f2", stroke: "#7b7296", stroke_width: "1.4" }
                text { x: "553", y: "51", text_anchor: "middle", fill: "#32323d", font_family: "var(--font-display)", font_size: "10.5", font_weight: "700", "Recall Fan-Out" }
                text { x: "553", y: "66", text_anchor: "middle", fill: "#5e5d6b", font_family: "var(--font-mono)", font_size: "8.5", "Order-preserving" }

                rect { x: "490", y: "85", width: "126", height: "44", rx: "11", fill: "#f2e8cd", stroke: "#b99c55", stroke_width: "1.4" }
                text { x: "553", y: "104", text_anchor: "middle", fill: "#32323d", font_family: "var(--font-display)", font_size: "10.5", font_weight: "700", "Metadata Pool" }
                text { x: "553", y: "119", text_anchor: "middle", fill: "#5e5d6b", font_family: "var(--font-mono)", font_size: "8.5", "Bulk HMGET lane" }

                rect { x: "490", y: "138", width: "126", height: "44", rx: "11", fill: "#e1ede7", stroke: "#3f7c66", stroke_width: "1.4" }
                text { x: "553", y: "157", text_anchor: "middle", fill: "#32323d", font_family: "var(--font-display)", font_size: "10.5", font_weight: "700", "LTR + ONNX Pool" }
                text { x: "553", y: "172", text_anchor: "middle", fill: "#5e5d6b", font_family: "var(--font-mono)", font_size: "8.5", "spawn_blocking" }

                rect { x: "490", y: "191", width: "126", height: "44", rx: "11", fill: "#f3e7e1", stroke: "#d97c60", stroke_width: "1.4" }
                text { x: "553", y: "210", text_anchor: "middle", fill: "#32323d", font_family: "var(--font-display)", font_size: "10.5", font_weight: "700", "ESCI Cache Pool" }
                text { x: "553", y: "225", text_anchor: "middle", fill: "#5e5d6b", font_family: "var(--font-mono)", font_size: "8.5", "Async write-back" }

                text {
                    x: "553",
                    y: "252",
                    text_anchor: "middle",
                    fill: "#7b7296",
                    font_family: "var(--font-mono)",
                    font_size: "8.6",
                    font_weight: "700",
                    "infra · domain::ports"
                }

                // Bottom handwritten-style margin caption
                text {
                    x: "320",
                    y: "284",
                    text_anchor: "middle",
                    fill: "#5e5d6b",
                    font_family: "var(--font-serif)",
                    font_style: "italic",
                    font_size: "11.5",
                    "Dependencies point strictly inward (api → domain ← infra) · Distinct OpenTelemetry spans per stage"
                }
            }
        }
    }
}

/// Dedicated standalone Blog / Writing page with:
/// 1) Chronological Timeline (timestamp on the left, newest first)
/// 2) By-Topic Indented Tree Index (grouped by subject with vertical guides & year)
/// 3) Rich serif Article Reader with inline Architecture Field Notes
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
                    // Rich serif Article Reader with inline Architecture Field Notes
                    div { class: "blog-top-bar",
                        button {
                            class: "text-btn",
                            r#type: "button",
                            onclick: move |_| {
                                active_slug.set(None);
                                document::eval("window.scrollTo({ top: 0, behavior: 'instant' });");
                            },
                            "← All posts"
                        }
                        span { class: "blog-meta-mono", "{post.date} · {post.read_time}" }
                    }

                    article { class: "article-shell",
                        header { class: "article-head",
                            div { class: "article-topic", "{post.topic}" }
                            h1 { class: "article-title",
                                RichTitle { title: post.title }
                            }
                            p { class: "article-subtitle", "{post.subtitle}" }
                            div { class: "chip-row",
                                for tag in post.tags {
                                    span { class: "chip", key: "{tag}", "{tag}" }
                                }
                            }
                        }

                        div { class: "article-prose",
                            for (idx, block) in post.blocks.iter().enumerate() {
                                match block {
                                    ArticleBlock::Heading(h) => rsx! {
                                        h2 { key: "h-{idx}", class: "article-h2", "{h}" }
                                    },
                                    ArticleBlock::Paragraph(p) => rsx! {
                                        p { key: "p-{idx}", "{p}" }
                                    },
                                    ArticleBlock::ArchitectureFlow(flow) => rsx! {
                                        div { key: "f-{idx}", class: "article-flow-wrap",
                                            div { class: "drawer-block-label", "Architecture Flow" }
                                            div { class: "drawer-flow", "{flow}" }
                                        }
                                    },
                                    ArticleBlock::HexagonalSketch => rsx! {
                                        div { key: "s-{idx}", class: "article-nb-wrap",
                                            HexagonalNotebookSketch {}
                                        }
                                    },
                                    ArticleBlock::Bullets(items) => rsx! {
                                        ul { key: "b-{idx}", class: "article-bullets",
                                            for item in *items {
                                                li { "{item}" }
                                            }
                                        }
                                    },
                                    ArticleBlock::FieldNotes {
                                        stamp,
                                        comparisons,
                                        callouts,
                                        sticky_notes,
                                    } => rsx! {
                                        div { key: "n-{idx}", class: "article-nb-wrap",
                                            NotebookFieldNotes {
                                                stamp: *stamp,
                                                comparisons: *comparisons,
                                                callouts: *callouts,
                                                sticky_notes: *sticky_notes,
                                            }
                                        }
                                    },
                                }
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
                            span { class: "hl-mark", "Notes" }
                            " & Essays"
                        }
                        p { class: "blog-intro",
                            "A quiet place to write down what I'm learning — mostly about production search, Rust for ML, latency mysteries, and evaluating AI systems. Newer posts sit at the top with their date on the left."
                        }
                    }

                    if layout_mode() == BlogLayoutMode::Timeline {
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
                                                h2 { class: "blog-tl-title",
                                                    RichTitle { title: post.title }
                                                }
                                                p { class: "blog-tl-sub", "{post.subtitle}" }
                                                span { class: "blog-read-cue", "Read essay →" }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if !PLANNED_BLOG_TOPICS.is_empty() {
                            div { class: "blog-drafts-divider", "Upcoming Drafts" }
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
                        }
                    } else {
                        div { class: "topic-tree",
                            div { class: "tree-root-label", "Published Articles" }
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

                            if !PLANNED_BLOG_TOPICS.is_empty() {
                                div { class: "tree-root-label", style: "margin-top: 26px;", "Upcoming Drafts" }
                                div { class: "tree-branch",
                                    for group in planned_topics {
                                        div { class: "tree-group", key: "draft-{group}",
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
                    }
                }
            }
        }
    }
}
