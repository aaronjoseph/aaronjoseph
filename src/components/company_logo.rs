use dioxus::prelude::*;

/// Crisp, self-contained vector brand marks for each company (zero external hotlinks).
#[component]
pub fn CompanyLogo(company_id: &'static str, #[props(default = 22)] size: u32) -> Element {
    let s = format!("{size}");

    match company_id {
        "bestbuy" => rsx! {
            // Official Best Buy stacked BEST / BUY wordmark + horizontal yellow hang-tag on Best Buy blue
            svg {
                class: "co-logo-svg",
                width: "{s}",
                height: "{s}",
                view_box: "0 0 28 28",
                fill: "none",
                "aria-hidden": "true",
                rect { width: "28", height: "28", rx: "6.5", fill: "#0046be" }
                text {
                    x: "2.9",
                    y: "12.5",
                    fill: "#ffffff",
                    font_family: "'Arial Black', 'Helvetica Neue', Arial, sans-serif",
                    font_weight: "900",
                    font_size: "7.7",
                    letter_spacing: "-0.35",
                    "BEST"
                }
                text {
                    x: "2.9",
                    y: "20.8",
                    fill: "#ffffff",
                    font_family: "'Arial Black', 'Helvetica Neue', Arial, sans-serif",
                    font_weight: "900",
                    font_size: "7.7",
                    letter_spacing: "-0.35",
                    "BUY"
                }
                path {
                    d: "M 20.8 16.0 H 25.6 V 20.8 H 20.8 L 19.3 19.3 V 17.5 Z",
                    fill: "#fff200",
                }
                circle { cx: "20.6", cy: "18.4", r: "0.6", fill: "#1d252c" }
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
                rect { width: "28", height: "28", rx: "6.5", fill: "#aad500" }
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
                rect { width: "28", height: "28", rx: "6.5", fill: "#1e162b" }
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
            // Official Kaleidofin two-tone left-pointing '<' chevron (deep ocean teal + turquoise)
            svg {
                class: "co-logo-svg",
                width: "{s}",
                height: "{s}",
                view_box: "0 0 28 28",
                fill: "none",
                "aria-hidden": "true",
                rect {
                    width: "28",
                    height: "28",
                    rx: "6.5",
                    fill: "#ffffff",
                    stroke: "rgba(44,42,53,0.12)",
                    stroke_width: "1",
                }
                path {
                    d: "M 6.5 12.1 L 21.3 5.4 L 21.3 10.2 L 12.4 14.0 L 6.5 14.0 Z",
                    fill: "#185c7c",
                }
                path {
                    d: "M 6.5 14.0 L 12.4 14.0 L 21.3 17.8 L 21.3 22.6 L 6.5 15.9 Z",
                    fill: "#35b3ac",
                }
            }
        },
        "fundsindia" => rsx! {
            // Official FundsIndia split green/blue chevron tree-diamond emblem
            svg {
                class: "co-logo-svg",
                width: "{s}",
                height: "{s}",
                view_box: "0 0 28 28",
                fill: "none",
                "aria-hidden": "true",
                rect {
                    width: "28",
                    height: "28",
                    rx: "6.5",
                    fill: "#ffffff",
                    stroke: "rgba(44,42,53,0.12)",
                    stroke_width: "1",
                }
                path {
                    d: "M 14 3.8 C 13.3 3.8 12.8 4.1 12.2 4.7 L 5.1 11.8 C 4.1 12.8 4.1 14.0 5.1 15.0 L 11.4 21.3 V 23.0 C 11.4 23.8 12.1 24.4 12.9 24.4 H 14 Z",
                    fill: "#34c769",
                }
                path {
                    d: "M 14 3.8 C 14.7 3.8 15.2 4.1 15.8 4.7 L 22.9 11.8 C 23.9 12.8 23.9 14.0 22.9 15.0 L 16.6 21.3 V 23.0 C 16.6 23.8 15.9 24.4 15.1 24.4 H 14 Z",
                    fill: "#2563ff",
                }
                path {
                    d: "M 8.4 8.2 L 14 13.8 L 19.6 8.2",
                    fill: "none",
                    stroke: "#ffffff",
                    stroke_width: "1.65",
                    stroke_linejoin: "miter",
                }
                path {
                    d: "M 5.6 11.0 L 14 19.4 L 22.4 11.0",
                    fill: "none",
                    stroke: "#ffffff",
                    stroke_width: "1.65",
                    stroke_linejoin: "miter",
                }
            }
        },
        "musigma" => rsx! {
            // Official Mu Sigma glossy crimson-maroon circular orb with lowercase white serif 'μσ'
            svg {
                class: "co-logo-svg co-logo-circle",
                width: "{s}",
                height: "{s}",
                view_box: "0 0 28 28",
                fill: "none",
                "aria-hidden": "true",
                defs {
                    radialGradient {
                        id: "musigmaOrbGrad",
                        cx: "65%",
                        cy: "30%",
                        r: "70%",
                        fx: "68%",
                        fy: "26%",
                        stop { offset: "0%", stop_color: "#e45c1e" }
                        stop { offset: "48%", stop_color: "#a21c14" }
                        stop { offset: "100%", stop_color: "#690c0c" }
                    }
                    clipPath {
                        id: "musigmaOrbClip",
                        circle { cx: "14", cy: "14", r: "13.2" }
                    }
                }
                circle { cx: "14", cy: "14", r: "13.2", fill: "url(#musigmaOrbGrad)" }
                g {
                    clip_path: "url(#musigmaOrbClip)",
                    text {
                        x: "14.7",
                        y: "19.8",
                        text_anchor: "middle",
                        fill: "#ffffff",
                        font_family: "Georgia, 'Times New Roman', serif",
                        font_weight: "400",
                        font_size: "19.2",
                        letter_spacing: "-0.8",
                        "μσ"
                    }
                }
            }
        },
        _ => rsx! {},
    }
}
