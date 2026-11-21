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

/// Handcrafted ruled-notebook SVG sketch of GPU reranker async write-back caching.
#[component]
fn AsyncCacheNotebookSketch() -> Element {
    rsx! {
        div { class: "nb-sheet",
            div { class: "nb-sheet-head",
                span { class: "nb-stamp", "Async Write-Back Cache Sketch" }
            }
            svg {
                class: "nb-sketch-svg",
                view_box: "0 0 640 296",
                fill: "none",
                role: "img",
                "aria-label": "Notebook sketch showing an early cache read overlapping recall, XXH3 hash freshness check routing hits directly to ordered response and misses to the GPU reranker, and a bounded async background batch queue with a memory watermark gate flushing to Redis",

                // Top Fast Response Lane
                rect {
                    x: "12",
                    y: "14",
                    width: "616",
                    height: "128",
                    rx: "16",
                    fill: "rgba(225, 237, 231, 0.45)",
                    stroke: "#3f7c66",
                    stroke_width: "1.4",
                    stroke_dasharray: "6 4",
                }
                rect {
                    x: "26",
                    y: "4",
                    width: "218",
                    height: "20",
                    rx: "10",
                    fill: "#e1ede7",
                    stroke: "rgba(63, 124, 102, 0.3)",
                }
                text {
                    x: "135",
                    y: "17.5",
                    text_anchor: "middle",
                    fill: "#2c6350",
                    font_family: "var(--font-mono)",
                    font_size: "9",
                    font_weight: "700",
                    "CRITICAL PATH · FAST RESPONSE"
                }

                // Box 1: Early Cache Read
                rect {
                    x: "26",
                    y: "40",
                    width: "114",
                    height: "78",
                    rx: "13",
                    fill: "#faf8f3",
                    stroke: "#32323d",
                    stroke_width: "1.5",
                }
                text {
                    x: "83",
                    y: "64",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "11",
                    font_weight: "700",
                    "Early Cache"
                }
                text {
                    x: "83",
                    y: "80",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "11",
                    font_weight: "700",
                    "Read"
                }
                text {
                    x: "83",
                    y: "100",
                    text_anchor: "middle",
                    fill: "#5e5d6b",
                    font_family: "var(--font-mono)",
                    font_size: "8.5",
                    "Overlaps recall"
                }

                // Arrow 1 -> 2
                path {
                    d: "M 140 79 H 164",
                    stroke: "#3f7c66",
                    stroke_width: "2",
                    stroke_linecap: "round",
                }
                path {
                    d: "M 159 74 L 166 79 L 159 84",
                    stroke: "#3f7c66",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }

                // Box 2: Hash Freshness Check
                rect {
                    x: "168",
                    y: "40",
                    width: "122",
                    height: "78",
                    rx: "13",
                    fill: "#f2e8cd",
                    stroke: "#b99c55",
                    stroke_width: "1.5",
                }
                text {
                    x: "229",
                    y: "64",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "11",
                    font_weight: "700",
                    "Hash Freshness"
                }
                text {
                    x: "229",
                    y: "80",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "11",
                    font_weight: "700",
                    "Check"
                }
                text {
                    x: "229",
                    y: "100",
                    text_anchor: "middle",
                    fill: "#7c6222",
                    font_family: "var(--font-mono)",
                    font_size: "8.5",
                    "Override → XXH3"
                }

                // Hit bypass curved path
                path {
                    d: "M 290 58 C 343 32, 433 32, 494 58",
                    stroke: "#3f7c66",
                    stroke_width: "2",
                    stroke_linecap: "round",
                }
                path {
                    d: "M 487 52 L 495 58 L 487 62",
                    stroke: "#3f7c66",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                rect { x: "340", y: "28", width: "104", height: "18", rx: "9", fill: "#e1ede7" }
                text {
                    x: "392",
                    y: "40",
                    text_anchor: "middle",
                    fill: "#2c6350",
                    font_family: "var(--font-mono)",
                    font_size: "8.5",
                    font_weight: "700",
                    "HITS BYPASS GPU"
                }

                // Misses arrow -> GPU Reranker
                path {
                    d: "M 290 93 H 326",
                    stroke: "#d97c60",
                    stroke_width: "2",
                    stroke_linecap: "round",
                }
                path {
                    d: "M 321 88 L 328 93 L 321 98",
                    stroke: "#d97c60",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                text {
                    x: "308",
                    y: "85",
                    text_anchor: "middle",
                    fill: "#b44a2f",
                    font_family: "var(--font-mono)",
                    font_size: "8",
                    font_weight: "700",
                    "miss"
                }

                // Box 3: GPU Reranker
                rect {
                    x: "330",
                    y: "62",
                    width: "128",
                    height: "62",
                    rx: "13",
                    fill: "#f3e7e1",
                    stroke: "#d97c60",
                    stroke_width: "1.6",
                }
                text {
                    x: "394",
                    y: "86",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "11",
                    font_weight: "700",
                    "GPU Reranker"
                }
                text {
                    x: "394",
                    y: "104",
                    text_anchor: "middle",
                    fill: "#a95138",
                    font_family: "var(--font-mono)",
                    font_size: "8.3",
                    "Misses only · lean logs"
                }

                // Arrow GPU -> Ordered Response
                path {
                    d: "M 458 93 H 494",
                    stroke: "#3f7c66",
                    stroke_width: "2",
                    stroke_linecap: "round",
                }
                path {
                    d: "M 489 88 L 496 93 L 489 98",
                    stroke: "#3f7c66",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }

                // Box 4: Ordered Response
                rect {
                    x: "498",
                    y: "42",
                    width: "114",
                    height: "76",
                    rx: "13",
                    fill: "#e1ede7",
                    stroke: "#3f7c66",
                    stroke_width: "1.8",
                }
                text {
                    x: "555",
                    y: "70",
                    text_anchor: "middle",
                    fill: "#2c6350",
                    font_family: "var(--font-display)",
                    font_size: "11",
                    font_weight: "700",
                    "Ordered"
                }
                text {
                    x: "555",
                    y: "86",
                    text_anchor: "middle",
                    fill: "#2c6350",
                    font_family: "var(--font-display)",
                    font_size: "11",
                    font_weight: "700",
                    "Response"
                }
                text {
                    x: "555",
                    y: "104",
                    text_anchor: "middle",
                    fill: "#3f7c66",
                    font_family: "var(--font-mono)",
                    font_size: "8.5",
                    "Zero write wait"
                }

                // Fire-and-forget dashed arrow down to Background Writer
                path {
                    d: "M 394 124 V 166",
                    stroke: "#7b7296",
                    stroke_width: "2",
                    stroke_dasharray: "4 3",
                    stroke_linecap: "round",
                }
                path {
                    d: "M 389 160 L 394 168 L 399 160",
                    stroke: "#7b7296",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                rect { x: "402", y: "138", width: "114", height: "18", rx: "9", fill: "#eae6f2" }
                text {
                    x: "459",
                    y: "150",
                    text_anchor: "middle",
                    fill: "#5b5276",
                    font_family: "var(--font-mono)",
                    font_size: "8.2",
                    font_weight: "700",
                    "async fire & forget"
                }

                // Bottom Background Lane
                rect {
                    x: "12",
                    y: "170",
                    width: "616",
                    height: "84",
                    rx: "16",
                    fill: "rgba(234, 230, 242, 0.45)",
                    stroke: "#7b7296",
                    stroke_width: "1.4",
                    stroke_dasharray: "6 4",
                }
                rect {
                    x: "26",
                    y: "160",
                    width: "232",
                    height: "20",
                    rx: "10",
                    fill: "#eae6f2",
                    stroke: "rgba(123, 114, 150, 0.3)",
                }
                text {
                    x: "142",
                    y: "173.5",
                    text_anchor: "middle",
                    fill: "#5b5276",
                    font_family: "var(--font-mono)",
                    font_size: "9",
                    font_weight: "700",
                    "BACKGROUND · ASYNC WRITE-BACK"
                }

                rect {
                    x: "32",
                    y: "190",
                    width: "164",
                    height: "48",
                    rx: "11",
                    fill: "#faf8f3",
                    stroke: "#7b7296",
                    stroke_width: "1.4",
                }
                text {
                    x: "114",
                    y: "211",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "10.5",
                    font_weight: "700",
                    "Bounded Batch Queue"
                }
                text {
                    x: "114",
                    y: "227",
                    text_anchor: "middle",
                    fill: "#5e5d6b",
                    font_family: "var(--font-mono)",
                    font_size: "8.3",
                    "Coalesced pipelined writes"
                }

                path {
                    d: "M 196 214 H 230",
                    stroke: "#7b7296",
                    stroke_width: "1.8",
                    stroke_dasharray: "4 3",
                    stroke_linecap: "round",
                }
                path {
                    d: "M 225 209 L 232 214 L 225 219",
                    stroke: "#7b7296",
                    stroke_width: "1.8",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }

                rect {
                    x: "234",
                    y: "190",
                    width: "174",
                    height: "48",
                    rx: "11",
                    fill: "#f2e8cd",
                    stroke: "#b99c55",
                    stroke_width: "1.4",
                }
                text {
                    x: "321",
                    y: "211",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "10.5",
                    font_weight: "700",
                    "Memory Watermark Gate"
                }
                text {
                    x: "321",
                    y: "227",
                    text_anchor: "middle",
                    fill: "#7c6222",
                    font_family: "var(--font-mono)",
                    font_size: "8.3",
                    "Pauses under pressure"
                }

                path {
                    d: "M 408 214 H 442",
                    stroke: "#7b7296",
                    stroke_width: "1.8",
                    stroke_dasharray: "4 3",
                    stroke_linecap: "round",
                }
                path {
                    d: "M 437 209 L 444 214 L 437 219",
                    stroke: "#7b7296",
                    stroke_width: "1.8",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }

                rect {
                    x: "446",
                    y: "190",
                    width: "164",
                    height: "48",
                    rx: "11",
                    fill: "#faf8f3",
                    stroke: "#3f7c66",
                    stroke_width: "1.4",
                }
                text {
                    x: "528",
                    y: "211",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "10.5",
                    font_weight: "700",
                    "Redis Cache Store"
                }
                text {
                    x: "528",
                    y: "227",
                    text_anchor: "middle",
                    fill: "#2c6350",
                    font_family: "var(--font-mono)",
                    font_size: "8.3",
                    "HSET + jittered EXPIRE NX"
                }

                // Bottom caption
                text {
                    x: "320",
                    y: "282",
                    text_anchor: "middle",
                    fill: "#5e5d6b",
                    font_family: "var(--font-serif)",
                    font_style: "italic",
                    font_size: "11.5",
                    "Cache failures fail open to the model · Background writes never block or fail a live request"
                }
            }
        }
    }
}

/// Handcrafted ruled-notebook SVG sketch of ONNX Runtime session pooling & non-blocking Tokio scheduling.
#[component]
fn OnnxPoolNotebookSketch() -> Element {
    rsx! {
        div { class: "nb-sheet",
            div { class: "nb-sheet-head",
                span { class: "nb-stamp", "ONNX Session Pool Sketch" }
            }
            svg {
                class: "nb-sketch-svg",
                view_box: "0 0 640 296",
                fill: "none",
                role: "img",
                "aria-label": "Notebook sketch separating Tokio async I/O workers assembling flat Vec<f32> features and acquiring an admission semaphore from a dedicated CPU blocking pool of single-threaded ONNX Runtime sessions with per-request A/B branch routing",

                // Left Zone: Async I/O Workers (Tokio)
                rect {
                    x: "12",
                    y: "18",
                    width: "234",
                    height: "236",
                    rx: "16",
                    fill: "rgba(225, 237, 231, 0.42)",
                    stroke: "#3f7c66",
                    stroke_width: "1.4",
                    stroke_dasharray: "6 4",
                }
                rect {
                    x: "26",
                    y: "8",
                    width: "174",
                    height: "20",
                    rx: "10",
                    fill: "#e1ede7",
                    stroke: "rgba(63, 124, 102, 0.3)",
                }
                text {
                    x: "113",
                    y: "21.5",
                    text_anchor: "middle",
                    fill: "#2c6350",
                    font_family: "var(--font-mono)",
                    font_size: "9",
                    font_weight: "700",
                    "ASYNC I/O WORKERS (TOKIO)"
                }

                rect {
                    x: "30",
                    y: "46",
                    width: "198",
                    height: "58",
                    rx: "12",
                    fill: "#faf8f3",
                    stroke: "#32323d",
                    stroke_width: "1.5",
                }
                text {
                    x: "129",
                    y: "69",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "11",
                    font_weight: "700",
                    "Feature Assembly"
                }
                text {
                    x: "129",
                    y: "88",
                    text_anchor: "middle",
                    fill: "#5e5d6b",
                    font_family: "var(--font-mono)",
                    font_size: "8.5",
                    "Flat Vec<f32> · zero DataFrames"
                }

                path {
                    d: "M 129 104 V 130",
                    stroke: "#3f7c66",
                    stroke_width: "2",
                    stroke_linecap: "round",
                }
                path {
                    d: "M 124 125 L 129 132 L 134 125",
                    stroke: "#3f7c66",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }

                rect {
                    x: "30",
                    y: "134",
                    width: "198",
                    height: "56",
                    rx: "12",
                    fill: "#f2e8cd",
                    stroke: "#b99c55",
                    stroke_width: "1.5",
                }
                text {
                    x: "129",
                    y: "157",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "11",
                    font_weight: "700",
                    "Admission Semaphore"
                }
                text {
                    x: "129",
                    y: "175",
                    text_anchor: "middle",
                    fill: "#7c6222",
                    font_family: "var(--font-mono)",
                    font_size: "8.5",
                    "Bounded permits · fair queueing"
                }

                text {
                    x: "129",
                    y: "218",
                    text_anchor: "middle",
                    fill: "#2c6350",
                    font_family: "var(--font-mono)",
                    font_size: "8.3",
                    font_weight: "700",
                    "Network & Redis I/O stay unblocked"
                }
                text {
                    x: "129",
                    y: "234",
                    text_anchor: "middle",
                    fill: "#5e5d6b",
                    font_family: "var(--font-sans)",
                    font_size: "9.5",
                    "No Mutex<Session> on async threads"
                }

                // Bridge arrow: spawn_blocking
                path {
                    d: "M 228 162 H 316",
                    stroke: "#d97c60",
                    stroke_width: "2.2",
                    stroke_linecap: "round",
                }
                path {
                    d: "M 310 157 L 318 162 L 310 167",
                    stroke: "#d97c60",
                    stroke_width: "2.2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                rect {
                    x: "236",
                    y: "136",
                    width: "90",
                    height: "20",
                    rx: "10",
                    fill: "#f3e7e1",
                    stroke: "#d97c60",
                    stroke_width: "1",
                }
                text {
                    x: "281",
                    y: "149",
                    text_anchor: "middle",
                    fill: "#a95138",
                    font_family: "var(--font-mono)",
                    font_size: "8.2",
                    font_weight: "700",
                    "spawn_blocking"
                }

                // Right Zone: Dedicated CPU Pool · ONNX Runtime
                rect {
                    x: "322",
                    y: "18",
                    width: "306",
                    height: "236",
                    rx: "16",
                    fill: "rgba(234, 230, 242, 0.42)",
                    stroke: "#7b7296",
                    stroke_width: "1.4",
                    stroke_dasharray: "6 4",
                }
                rect {
                    x: "336",
                    y: "8",
                    width: "244",
                    height: "20",
                    rx: "10",
                    fill: "#eae6f2",
                    stroke: "rgba(123, 114, 150, 0.3)",
                }
                text {
                    x: "458",
                    y: "21.5",
                    text_anchor: "middle",
                    fill: "#5b5276",
                    font_family: "var(--font-mono)",
                    font_size: "9",
                    font_weight: "700",
                    "DEDICATED CPU POOL · ONNX RUNTIME"
                }

                // Branch selector bar (generous side padding)
                rect {
                    x: "336",
                    y: "40",
                    width: "278",
                    height: "28",
                    rx: "10",
                    fill: "#faf8f3",
                    stroke: "rgba(50, 50, 61, 0.12)",
                }
                text {
                    x: "475",
                    y: "57.5",
                    text_anchor: "middle",
                    fill: "#5b5276",
                    font_family: "var(--font-mono)",
                    font_size: "8.2",
                    font_weight: "700",
                    "A/B Branch Selector · [Model A | Model B]"
                }

                // 3 Pooled ONNX Sessions
                rect {
                    x: "340",
                    y: "82",
                    width: "128",
                    height: "44",
                    rx: "11",
                    fill: "#faf8f3",
                    stroke: "#7b7296",
                    stroke_width: "1.4",
                }
                text {
                    x: "404",
                    y: "101",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "10.5",
                    font_weight: "700",
                    "ONNX Session #1"
                }
                text {
                    x: "404",
                    y: "116",
                    text_anchor: "middle",
                    fill: "#5e5d6b",
                    font_family: "var(--font-mono)",
                    font_size: "8.1",
                    "1 intra · 1 inter thread"
                }

                rect {
                    x: "340",
                    y: "136",
                    width: "128",
                    height: "44",
                    rx: "11",
                    fill: "#faf8f3",
                    stroke: "#7b7296",
                    stroke_width: "1.4",
                }
                text {
                    x: "404",
                    y: "155",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "10.5",
                    font_weight: "700",
                    "ONNX Session #2"
                }
                text {
                    x: "404",
                    y: "170",
                    text_anchor: "middle",
                    fill: "#5e5d6b",
                    font_family: "var(--font-mono)",
                    font_size: "8.1",
                    "1 intra · 1 inter thread"
                }

                rect {
                    x: "340",
                    y: "190",
                    width: "128",
                    height: "44",
                    rx: "11",
                    fill: "#faf8f3",
                    stroke: "#7b7296",
                    stroke_width: "1.4",
                }
                text {
                    x: "404",
                    y: "209",
                    text_anchor: "middle",
                    fill: "#32323d",
                    font_family: "var(--font-display)",
                    font_size: "10.5",
                    font_weight: "700",
                    "ONNX Session #N"
                }
                text {
                    x: "404",
                    y: "224",
                    text_anchor: "middle",
                    fill: "#5e5d6b",
                    font_family: "var(--font-mono)",
                    font_size: "8.1",
                    "Sized to CPU cores"
                }

                // Curves converging into Ranked Scores
                path {
                    d: "M 468 104 C 482 106, 488 128, 502 144",
                    stroke: "#3f7c66",
                    stroke_width: "1.7",
                    stroke_linecap: "round",
                }
                path {
                    d: "M 468 158 H 502",
                    stroke: "#3f7c66",
                    stroke_width: "1.7",
                    stroke_linecap: "round",
                }
                path {
                    d: "M 468 212 C 482 210, 488 188, 502 172",
                    stroke: "#3f7c66",
                    stroke_width: "1.7",
                    stroke_linecap: "round",
                }

                rect {
                    x: "504",
                    y: "122",
                    width: "108",
                    height: "72",
                    rx: "13",
                    fill: "#e1ede7",
                    stroke: "#3f7c66",
                    stroke_width: "1.8",
                }
                text {
                    x: "558",
                    y: "150",
                    text_anchor: "middle",
                    fill: "#2c6350",
                    font_family: "var(--font-display)",
                    font_size: "11",
                    font_weight: "700",
                    "Ranked"
                }
                text {
                    x: "558",
                    y: "166",
                    text_anchor: "middle",
                    fill: "#2c6350",
                    font_family: "var(--font-display)",
                    font_size: "11",
                    font_weight: "700",
                    "Scores"
                }
                text {
                    x: "558",
                    y: "183",
                    text_anchor: "middle",
                    fill: "#3f7c66",
                    font_family: "var(--font-mono)",
                    font_size: "8.2",
                    "Predictable tail"
                }

                // Bottom caption
                text {
                    x: "320",
                    y: "282",
                    text_anchor: "middle",
                    fill: "#5e5d6b",
                    font_family: "var(--font-serif)",
                    font_style: "italic",
                    font_size: "11.5",
                    "Clean separation of feature assembly, concurrency admission, and CPU-pinned ONNX execution"
                }
            }
        }
    }
}

/// Interactive Article Reader with a moving sticky Left Reading Rail:
/// highlights the active subsection, marks completed ones with sage checks,
/// shows remaining sections left to read, and animates a continuous progress spine.
#[component]
fn ArticleReader(slug: &'static str, on_back: EventHandler<()>) -> Element {
    let Some(post) = BLOG_POSTS.iter().find(|p| p.slug == slug) else {
        return rsx! {};
    };

    let mut active_sec = use_signal(|| 0usize);
    let mut read_pct = use_signal(|| 0u32);

    // Collect Overview + every Heading block in order
    let mut sections: Vec<(usize, &'static str)> = vec![(0, "Overview")];
    let mut heading_counter = 0usize;
    let mut block_heading_ids: Vec<Option<usize>> = Vec::with_capacity(post.blocks.len());
    for block in post.blocks {
        if let ArticleBlock::Heading(h) = block {
            heading_counter += 1;
            sections.push((heading_counter, *h));
            block_heading_ids.push(Some(heading_counter));
        } else {
            block_heading_ids.push(None);
        }
    }

    let total_sections = sections.len();

    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(
                r#"
                if (window.__ajmArticleScrollHandler) {
                    window.removeEventListener('scroll', window.__ajmArticleScrollHandler);
                }

                let lastSec = -1;
                let lastPct = -1;
                let ticking = false;

                const evaluate = () => {
                    ticking = false;
                    const art = document.getElementById('article-shell');
                    if (!art) return;

                    const artRect = art.getBoundingClientRect();
                    const scrollable = Math.max(1, art.offsetHeight - window.innerHeight * 0.55);
                    const scrolled = Math.max(0, 120 - artRect.top);
                    const atBottom = (window.innerHeight + window.scrollY) >= (document.documentElement.scrollHeight - 24);
                    const pct = atBottom ? 100 : Math.min(100, Math.max(0, Math.round((scrolled / scrollable) * 100)));

                    const anchors = Array.from(art.querySelectorAll('[data-art-sec]'));
                    let curSec = 0;
                    if (atBottom && anchors.length > 0) {
                        curSec = anchors.length - 1;
                    } else {
                        const threshold = 165;
                        anchors.forEach((el, i) => {
                            if (el.getBoundingClientRect().top <= threshold) {
                                curSec = i;
                            }
                        });
                    }

                    if (curSec !== lastSec || pct !== lastPct) {
                        lastSec = curSec;
                        lastPct = pct;
                        dioxus.send([curSec, pct]);
                    }
                };

                const onScroll = () => {
                    if (!ticking) {
                        ticking = true;
                        requestAnimationFrame(evaluate);
                    }
                };

                window.__ajmArticleScrollHandler = onScroll;
                window.addEventListener('scroll', onScroll, { passive: true });
                requestAnimationFrame(evaluate);
                "#,
            );

            while let Ok((sec, pct)) = eval.recv::<(usize, u32)>().await {
                if sec < total_sections {
                    active_sec.set(sec);
                }
                read_pct.set(pct);
            }
        });
    });

    let cur = active_sec();
    let pct = read_pct();
    let remaining = total_sections.saturating_sub(cur + 1);
    let status_text = if pct >= 96 {
        "Finished reading ✓".to_string()
    } else if remaining == 0 {
        "Final section".to_string()
    } else if remaining == 1 {
        "1 section left to read".to_string()
    } else {
        format!("{remaining} sections left to read")
    };

    rsx! {
        div { class: "blog-top-bar",
            button {
                class: "text-btn",
                r#type: "button",
                onclick: move |_| {
                    document::eval(
                        "if (window.__ajmArticleScrollHandler) { window.removeEventListener('scroll', window.__ajmArticleScrollHandler); } window.scrollTo({ top: 0, behavior: 'instant' });",
                    );
                    on_back.call(());
                },
                "← All posts"
            }
            span { class: "blog-meta-mono", "{post.date} · {post.read_time}" }
        }

        div { class: "article-layout",
            // Sticky moving Left Reading Rail
            aside { class: "article-toc", aria_label: "Article sections",
                div { class: "toc-head",
                    span { class: "toc-kicker", "Reading Guide" }
                    span { class: "toc-pct", "{pct}%" }
                }

                div { class: "toc-bar-track", aria_hidden: "true",
                    div { class: "toc-bar-fill", style: "width: {pct}%;" }
                }

                nav { class: "toc-list",
                    div { class: "toc-spine", aria_hidden: "true",
                        div { class: "toc-spine-fill", style: "height: {pct}%;" }
                    }
                    for (sec_idx, title) in sections {
                        {
                            let state_cls = if sec_idx < cur {
                                "toc-item is-done"
                            } else if sec_idx == cur {
                                "toc-item is-active"
                            } else {
                                "toc-item is-upcoming"
                            };
                            let target_id = format!("art-sec-{sec_idx}");
                            rsx! {
                                button {
                                    key: "toc-{sec_idx}",
                                    class: "{state_cls}",
                                    r#type: "button",
                                    aria_current: if sec_idx == cur { "location" } else { "false" },
                                    onclick: move |_| {
                                        active_sec.set(sec_idx);
                                        let js = format!(
                                            "const el = document.getElementById('{target_id}'); if (el) el.scrollIntoView({{ behavior: 'smooth', block: 'start' }});"
                                        );
                                        document::eval(&js);
                                    },
                                    span { class: "toc-node", aria_hidden: "true",
                                        if sec_idx < cur { "✓" } else { "" }
                                    }
                                    span { class: "toc-label", "{title}" }
                                }
                            }
                        }
                    }
                }

                div {
                    class: if pct >= 96 { "toc-foot is-complete" } else { "toc-foot" },
                    "{status_text}"
                }
            }

            article { id: "article-shell", class: "article-shell",
                header { id: "art-sec-0", class: "article-head", "data-art-sec": "0",
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
                            ArticleBlock::Heading(h) => {
                                let hid = block_heading_ids[idx].unwrap_or(0);
                                let el_id = format!("art-sec-{hid}");
                                rsx! {
                                    h2 {
                                        key: "h-{idx}",
                                        id: "{el_id}",
                                        class: "article-h2",
                                        "data-art-sec": "{hid}",
                                        "{h}"
                                    }
                                }
                            }
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
                            ArticleBlock::AsyncCacheSketch => rsx! {
                                div { key: "ac-{idx}", class: "article-nb-wrap",
                                    AsyncCacheNotebookSketch {}
                                }
                            },
                            ArticleBlock::OnnxPoolSketch => rsx! {
                                div { key: "ox-{idx}", class: "article-nb-wrap",
                                    OnnxPoolNotebookSketch {}
                                }
                            },
                            ArticleBlock::Bullets(items) => rsx! {
                                div { key: "b-{idx}", class: "article-point-grid",
                                    for (pi, (label, body)) in items.iter().enumerate() {
                                        {
                                            let tone_cls = match pi % 4 {
                                                0 => "article-point-card pt-sage",
                                                1 => "article-point-card pt-lavender",
                                                2 => "article-point-card pt-peach",
                                                _ => "article-point-card pt-butter",
                                            };
                                            rsx! {
                                                div { key: "{label}", class: "{tone_cls}",
                                                    div { class: "article-point-head",
                                                        span { class: "article-point-badge", "{label}" }
                                                    }
                                                    p { class: "article-point-body", "{body}" }
                                                }
                                            }
                                        }
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
        }
    }
}

/// Dedicated standalone Blog / Writing page with:
/// 1) Chronological Timeline (timestamp on the left, newest first)
/// 2) By-Topic Indented Tree Index (grouped by subject with vertical guides & year)
/// 3) Rich serif Article Reader with moving Left Reading Rail & Architecture Field Notes
#[component]
pub fn BlogView(on_back: EventHandler<()>) -> Element {
    let mut layout_mode = use_signal(|| BlogLayoutMode::Timeline);
    let mut active_slug = use_signal(|| Option::<&'static str>::None);

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

    let wrap_cls = if active_slug().is_some() {
        "blog-wrap reading-mode"
    } else {
        "blog-wrap"
    };

    rsx! {
        div { class: "blog-page",
            div { class: "{wrap_cls}",
                if let Some(slug) = active_slug() {
                    ArticleReader {
                        key: "{slug}",
                        slug,
                        on_back: move |_| active_slug.set(None),
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
