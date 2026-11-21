use dioxus::prelude::*;

/// Crisp, self-contained vector brand marks for each company (zero external hotlinks).
#[component]
pub fn CompanyLogo(company_id: &'static str, #[props(default = 20)] size: u32) -> Element {
    let s = format!("{size}");

    match company_id {
        "bestbuy" => rsx! {
            // Iconic Best Buy yellow hang-tag mark on deep Best Buy blue tile
            svg {
                class: "co-logo-svg",
                width: "{s}",
                height: "{s}",
                view_box: "0 0 28 28",
                fill: "none",
                "aria-hidden": "true",
                rect { width: "28", height: "28", rx: "7", fill: "#0046be" }
                // Yellow hang tag rotated slightly
                path {
                    d: "M6.5 9.2 15.8 5.8a1.5 1.5 0 0 1 1.6.35l4.4 4.4a1.5 1.5 0 0 1 .05 2.08l-7.2 7.8a1.5 1.5 0 0 1-2.12.06L6.1 14.1a1.5 1.5 0 0 1-.4-1.4l.4-2.5a1.5 1.5 0 0 1 .4-.99Z",
                    fill: "#fff200",
                }
                circle { cx: "16.8", cy: "10.3", r: "1.6", fill: "#0046be" }
            }
        },
        "falabella" => rsx! {
            // Iconic Falabella vibrant lime-green tile with clean lowercase 'f'
            svg {
                class: "co-logo-svg",
                width: "{s}",
                height: "{s}",
                view_box: "0 0 28 28",
                fill: "none",
                "aria-hidden": "true",
                rect { width: "28", height: "28", rx: "7", fill: "#aad500" }
                path {
                    d: "M16.9 7.2c-2.6 0-4.1 1.5-4.1 4.1v1.4h-2.1v2.8h2.1v6.4h3.2v-6.4h2.5l.4-2.8H16v-1.2c0-.9.4-1.4 1.4-1.4h1.5V7.3c-.5-.1-1.2-.1-2-.1Z",
                    fill: "#1b3a10",
                }
            }
        },
        "accenture" => rsx! {
            // Iconic Accenture purple '>' chevron mark
            svg {
                class: "co-logo-svg",
                width: "{s}",
                height: "{s}",
                view_box: "0 0 28 28",
                fill: "none",
                "aria-hidden": "true",
                rect { width: "28", height: "28", rx: "7", fill: "#1e162b" }
                path {
                    d: "M8.8 8.2 20.2 14 8.8 19.8",
                    stroke: "#a100ff",
                    stroke_width: "3.6",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        },
        "kaleidofin" => rsx! {
            // Kaleidofin geometric kaleidoscope petals mark
            svg {
                class: "co-logo-svg",
                width: "{s}",
                height: "{s}",
                view_box: "0 0 28 28",
                fill: "none",
                "aria-hidden": "true",
                rect { width: "28", height: "28", rx: "7", fill: "#ffffff", stroke: "rgba(44,42,53,0.12)", stroke_width: "1" }
                path { d: "M14 5.5 18.2 12 14 14 9.8 12Z", fill: "#00a3a1" }
                path { d: "M22.5 14 16 18.2 14 14 16 9.8Z", fill: "#f15a29" }
                path { d: "M14 22.5 9.8 16 14 14 18.2 16Z", fill: "#f7941d" }
                path { d: "M5.5 14 12 9.8 14 14 12 18.2Z", fill: "#662d91" }
            }
        },
        "fundsindia" => rsx! {
            // FundsIndia blue/green upward investment growth bars
            svg {
                class: "co-logo-svg",
                width: "{s}",
                height: "{s}",
                view_box: "0 0 28 28",
                fill: "none",
                "aria-hidden": "true",
                rect { width: "28", height: "28", rx: "7", fill: "#0b4f8a" }
                rect { x: "6.5", y: "16", width: "3.4", height: "5.5", rx: "1", fill: "#7bd389" }
                rect { x: "12.2", y: "12.5", width: "3.4", height: "9", rx: "1", fill: "#4eb848" }
                rect { x: "17.9", y: "8.2", width: "3.4", height: "13.3", rx: "1", fill: "#ffffff" }
            }
        },
        "musigma" => rsx! {
            // MuSigma crimson μΣ emblem
            svg {
                class: "co-logo-svg",
                width: "{s}",
                height: "{s}",
                view_box: "0 0 28 28",
                fill: "none",
                "aria-hidden": "true",
                rect { width: "28", height: "28", rx: "7", fill: "#b22234" }
                text {
                    x: "50%",
                    y: "56%",
                    dominant_baseline: "middle",
                    text_anchor: "middle",
                    fill: "#ffffff",
                    font_family: "Georgia, serif",
                    font_weight: "700",
                    font_size: "12.5",
                    "μΣ"
                }
            }
        },
        _ => rsx! {},
    }
}
