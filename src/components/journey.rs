use dioxus::prelude::*;

use crate::components::company_logo::CompanyLogo;
use crate::data::ROLES;

/// Handcrafted warm-editorial animated SVG illustration for each career chapter.
#[component]
fn StoryGraphic(role_id: &'static str) -> Element {
    match role_id {
        "bestbuy" => rsx! {
            // Chapter 6: Rust/ARM C4A Performance & Cost Reduction Engine
            svg {
                class: "story-svg",
                view_box: "0 0 320 148",
                fill: "none",
                "aria-hidden": "true",
                rect { x: "6", y: "6", width: "308", height: "136", rx: "16", fill: "#faf8f3", stroke: "rgba(50,50,61,0.08)" }

                // Rotating Rust-inspired performance gear on left
                g { class: "sg-spin-slow",
                    g { fill: "#ce422b",
                        rect { x: "55.5", y: "27", width: "7", height: "9", rx: "2", transform: "rotate(0 59 64)" }
                        rect { x: "55.5", y: "27", width: "7", height: "9", rx: "2", transform: "rotate(45 59 64)" }
                        rect { x: "55.5", y: "27", width: "7", height: "9", rx: "2", transform: "rotate(90 59 64)" }
                        rect { x: "55.5", y: "27", width: "7", height: "9", rx: "2", transform: "rotate(135 59 64)" }
                        rect { x: "55.5", y: "27", width: "7", height: "9", rx: "2", transform: "rotate(180 59 64)" }
                        rect { x: "55.5", y: "27", width: "7", height: "9", rx: "2", transform: "rotate(225 59 64)" }
                        rect { x: "55.5", y: "27", width: "7", height: "9", rx: "2", transform: "rotate(270 59 64)" }
                        rect { x: "55.5", y: "27", width: "7", height: "9", rx: "2", transform: "rotate(315 59 64)" }
                        circle { cx: "59", cy: "64", r: "29" }
                    }
                    circle { cx: "59", cy: "64", r: "20", fill: "#fff7f4" }
                }
                text {
                    x: "59",
                    y: "71",
                    text_anchor: "middle",
                    fill: "#ce422b",
                    font_family: "Georgia, serif",
                    font_weight: "800",
                    font_size: "22",
                    "R"
                }
                text {
                    x: "59",
                    y: "118",
                    text_anchor: "middle",
                    fill: "#87808f",
                    font_family: "ui-monospace, monospace",
                    font_size: "8.5",
                    font_weight: "600",
                    "RUST · ARM C4A"
                }

                // Animated dashed connector line
                path {
                    class: "sg-dash-flow",
                    d: "M 96 64 H 130",
                    stroke: "#d97c60",
                    stroke_width: "2.2",
                    stroke_dasharray: "5 4",
                    stroke_linecap: "round",
                }

                // Cost & latency win bars on right
                // Row 1: CPU −95%
                rect { x: "136", y: "24", width: "160", height: "28", rx: "9", fill: "#fcebe6" }
                text { x: "147", y: "41.5", fill: "#b44a2f", font_family: "ui-monospace, monospace", font_size: "9.5", font_weight: "700", "−95% CPU · −98% RAM" }

                // Row 2: Cost / turn 21c -> 18c
                rect { x: "136", y: "60", width: "160", height: "28", rx: "9", fill: "#e4f2ec" }
                text { x: "147", y: "77.5", fill: "#3f7c66", font_family: "ui-monospace, monospace", font_size: "9.5", font_weight: "700", "21¢ → 18¢ / agent turn" }

                // Row 3: Annual savings + TTFT
                rect { x: "136", y: "96", width: "160", height: "28", rx: "9", fill: "#f1edf5" }
                text { x: "147", y: "113.5", fill: "#69587a", font_family: "ui-monospace, monospace", font_size: "9.5", font_weight: "700", "~$180K/yr · 15→12s TTFT" }
            }
        },
        "falabella" => rsx! {
            // Chapter 5: High-RPS Search Wave (15K RPS) + 15-Min Polars LTR
            svg {
                class: "story-svg",
                view_box: "0 0 320 148",
                fill: "none",
                "aria-hidden": "true",
                rect { x: "6", y: "6", width: "308", height: "136", rx: "16", fill: "#faf8f3", stroke: "rgba(50,50,61,0.08)" }

                // Subtle grid lines
                line { x1: "24", y1: "42", x2: "296", y2: "42", stroke: "rgba(44,42,53,0.06)", stroke_dasharray: "3 4" }
                line { x1: "24", y1: "72", x2: "296", y2: "72", stroke: "rgba(44,42,53,0.06)", stroke_dasharray: "3 4" }
                line { x1: "24", y1: "102", x2: "296", y2: "102", stroke: "rgba(44,42,53,0.06)", stroke_dasharray: "3 4" }

                // Animated high-RPS throughput waveform
                path {
                    d: "M 24 98 C 56 98, 70 80, 96 66 C 122 52, 146 34, 178 36 C 210 38, 224 62, 252 48 C 272 38, 284 32, 296 30 L 296 106 L 24 106 Z",
                    fill: "rgba(46, 125, 99, 0.11)",
                }
                path {
                    class: "sg-draw-line",
                    d: "M 24 98 C 56 98, 70 80, 96 66 C 122 52, 146 34, 178 36 C 210 38, 224 62, 252 48 C 272 38, 284 32, 296 30",
                    stroke: "#3f7c66",
                    stroke_width: "2.8",
                    stroke_linecap: "round",
                }
                circle { class: "sg-pulse-node", cx: "178", cy: "36", r: "5", fill: "#d97c60" }
                circle { class: "sg-pulse-node", cx: "296", cy: "30", r: "4.5", fill: "#3f7c66" }

                // Floating callout pills
                rect { x: "24", y: "18", width: "108", height: "24", rx: "12", fill: "#fcebe6" }
                text { x: "78", y: "33.5", text_anchor: "middle", fill: "#b44a2f", font_family: "ui-monospace, monospace", font_size: "9.5", font_weight: "700", "UP TO 15K RPS" }

                rect { x: "142", y: "108", width: "154", height: "24", rx: "12", fill: "#e4f2ec" }
                text { x: "219", y: "123.5", text_anchor: "middle", fill: "#3f7c66", font_family: "ui-monospace, monospace", font_size: "9.2", font_weight: "700", "LTR: 24h → 15m (Polars)" }

                text { x: "24", y: "123.5", fill: "#87808f", font_family: "ui-monospace, monospace", font_size: "8.8", font_weight: "600", "Async FastAPI" }
            }
        },
        "accenture" => rsx! {
            // Chapter 4: Terraform IaC -> dev / stage / prod MLOps pipeline
            svg {
                class: "story-svg",
                view_box: "0 0 320 148",
                fill: "none",
                "aria-hidden": "true",
                rect { x: "6", y: "6", width: "308", height: "136", rx: "16", fill: "#faf8f3", stroke: "rgba(50,50,61,0.08)" }

                // Terraform IaC source block on left
                rect { x: "22", y: "42", width: "84", height: "64", rx: "12", fill: "#f1edf5", stroke: "rgba(135,119,150,0.35)" }
                text { x: "64", y: "69", text_anchor: "middle", fill: "#5f4c73", font_family: "ui-monospace, monospace", font_size: "10", font_weight: "700", "Terraform" }
                text { x: "64", y: "85", text_anchor: "middle", fill: "#877796", font_family: "ui-monospace, monospace", font_size: "8.5", "GCP IaC" }

                // Animated branching provisioning lines
                path { class: "sg-dash-flow", d: "M 106 74 H 136 L 154 36 H 184", stroke: "#877796", stroke_width: "2", stroke_dasharray: "5 4", stroke_linecap: "round" }
                path { class: "sg-dash-flow", d: "M 106 74 H 184", stroke: "#d97c60", stroke_width: "2", stroke_dasharray: "5 4", stroke_linecap: "round" }
                path { class: "sg-dash-flow", d: "M 106 74 H 136 L 154 112 H 184", stroke: "#3f7c66", stroke_width: "2", stroke_dasharray: "5 4", stroke_linecap: "round" }

                // 3 provisioned GCP env cards
                rect { x: "184", y: "21", width: "112", height: "28", rx: "9", fill: "#f2eee6", stroke: "rgba(44,42,53,0.1)" }
                circle { cx: "199", cy: "35", r: "4", fill: "#877796" }
                text { x: "211", y: "38.5", fill: "#2c2a35", font_family: "ui-monospace, monospace", font_size: "9.5", font_weight: "600", "dev · GCP" }

                rect { x: "184", y: "60", width: "112", height: "28", rx: "9", fill: "#fcebe6", stroke: "rgba(240,139,109,0.28)" }
                circle { cx: "199", cy: "74", r: "4", fill: "#d97c60" }
                text { x: "211", y: "77.5", fill: "#2c2a35", font_family: "ui-monospace, monospace", font_size: "9.5", font_weight: "600", "stage · GCP" }

                rect { x: "184", y: "99", width: "112", height: "28", rx: "9", fill: "#e4f2ec", stroke: "rgba(46,125,99,0.28)" }
                circle { cx: "199", cy: "113", r: "4", fill: "#3f7c66" }
                text { x: "211", y: "116.5", fill: "#3f7c66", font_family: "ui-monospace, monospace", font_size: "9.5", font_weight: "700", "prod · −50%" }
            }
        },
        "kaleidofin" => rsx! {
            // Chapter 3: AMFI Sharpe Ratio Risk Radar + Shield blocking Franklin outlier
            svg {
                class: "story-svg",
                view_box: "0 0 320 148",
                fill: "none",
                "aria-hidden": "true",
                rect { x: "6", y: "6", width: "308", height: "136", rx: "16", fill: "#faf8f3", stroke: "rgba(50,50,61,0.08)" }

                // Axes (Risk vs Return)
                line { x1: "32", y1: "114", x2: "196", y2: "114", stroke: "rgba(44,42,53,0.18)", stroke_width: "1.4" }
                line { x1: "32", y1: "26", x2: "32", y2: "114", stroke: "rgba(44,42,53,0.18)", stroke_width: "1.4" }
                text { x: "114", y: "130", text_anchor: "middle", fill: "#87808f", font_family: "ui-monospace, monospace", font_size: "8", "CREDIT RISK →" }

                // Efficient Sharpe frontier curve
                path {
                    class: "sg-draw-line",
                    d: "M 42 102 Q 86 64, 152 54",
                    stroke: "#35b3ac",
                    stroke_width: "2.2",
                    stroke_linecap: "round",
                }
                // Safe mutual fund dots
                circle { cx: "56", cy: "92", r: "4.5", fill: "#3f7c66" }
                circle { cx: "82", cy: "75", r: "4.5", fill: "#3f7c66" }
                circle { cx: "112", cy: "63", r: "4.5", fill: "#35b3ac" }
                circle { cx: "140", cy: "56", r: "4.5", fill: "#35b3ac" }

                // High-risk Franklin outlier dot (+1-2% XIRR trap)
                circle { class: "sg-pulse-node", cx: "178", cy: "36", r: "6.5", fill: "rgba(217, 72, 59, 0.2)" }
                circle { cx: "178", cy: "36", r: "4", fill: "#d9483b" }
                text { x: "178", y: "24", text_anchor: "middle", fill: "#b44a2f", font_family: "ui-monospace, monospace", font_size: "8", font_weight: "700", "TRAP (+2%)" }

                // Protective Shield on right
                g { class: "sg-float",
                    path {
                        d: "M 254 26 L 286 38 V 68 C 286 89 271 104 254 112 C 237 104 222 89 222 68 V 38 Z",
                        fill: "#e4f2ec",
                        stroke: "#3f7c66",
                        stroke_width: "2.2",
                    }
                    path {
                        d: "M 242 68 L 250 76 L 267 58",
                        stroke: "#3f7c66",
                        stroke_width: "3",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                    }
                }
                text { x: "254", y: "130", text_anchor: "middle", fill: "#3f7c66", font_family: "ui-monospace, monospace", font_size: "8.5", font_weight: "700", "$3.4B AVOIDED" }
            }
        },
        "fundsindia" => rsx! {
            // Chapter 2: Mutual Fund Conversion Funnel & Product Analytics
            svg {
                class: "story-svg",
                view_box: "0 0 320 148",
                fill: "none",
                "aria-hidden": "true",
                rect { x: "6", y: "6", width: "308", height: "136", rx: "16", fill: "#faf8f3", stroke: "rgba(50,50,61,0.08)" }

                // Stepped product conversion funnel bars
                rect { class: "sg-bar-grow", x: "24", y: "24", width: "164", height: "20", rx: "7", fill: "#e6eefc" }
                text { x: "34", y: "37.5", fill: "#2563ff", font_family: "ui-monospace, monospace", font_size: "8.8", font_weight: "700", "DISCOVERY · PRODUCT UI" }

                rect { class: "sg-bar-grow", x: "38", y: "52", width: "136", height: "20", rx: "7", fill: "#d9f4e6" }
                text { x: "48", y: "65.5", fill: "#1f7a54", font_family: "ui-monospace, monospace", font_size: "8.8", font_weight: "700", "FUND COMPARISON" }

                rect { class: "sg-bar-grow", x: "54", y: "80", width: "104", height: "20", rx: "7", fill: "#fcebe6" }
                text { x: "64", y: "93.5", fill: "#b44a2f", font_family: "ui-monospace, monospace", font_size: "8.8", font_weight: "700", "INVESTMENT" }

                rect { class: "sg-bar-grow", x: "68", y: "108", width: "76", height: "18", rx: "7", fill: "#3f7c66" }
                text { x: "106", y: "120", text_anchor: "middle", fill: "#ffffff", font_family: "ui-monospace, monospace", font_size: "8.5", font_weight: "700", "+15% CONV" }

                // Product & Design metric card on right
                rect { x: "206", y: "28", width: "92", height: "94", rx: "12", fill: "#f2eee6", stroke: "rgba(44,42,53,0.09)" }
                text { x: "252", y: "54", text_anchor: "middle", fill: "#3f7c66", font_family: "Georgia, serif", font_size: "20", font_weight: "700", "+15%" }
                text { x: "252", y: "70", text_anchor: "middle", fill: "#5b5663", font_family: "ui-monospace, monospace", font_size: "8", "FUNNEL LIFT" }
                line { x1: "222", y1: "82", x2: "282", y2: "82", stroke: "rgba(44,42,53,0.09)" }
                text { x: "252", y: "100", text_anchor: "middle", fill: "#2563ff", font_family: "ui-monospace, monospace", font_size: "8.8", font_weight: "700", "−10% DROPS" }
                text { x: "252", y: "113", text_anchor: "middle", fill: "#87808f", font_family: "ui-monospace, monospace", font_size: "7.5", "PRODUCT & UX" }
            }
        },
        _ => rsx! {
            // Chapter 1 (MuSigma): Decision Science & Executive Dashboards
            svg {
                class: "story-svg",
                view_box: "0 0 320 148",
                fill: "none",
                "aria-hidden": "true",
                rect { x: "6", y: "6", width: "308", height: "136", rx: "16", fill: "#faf8f3", stroke: "rgba(50,50,61,0.08)" }

                // Mini window dots
                circle { cx: "24", cy: "22", r: "3", fill: "#f7cec5" }
                circle { cx: "34", cy: "22", r: "3", fill: "#fbf0cf" }
                circle { cx: "44", cy: "22", r: "3", fill: "#e4f2ec" }
                text { x: "294", y: "25", text_anchor: "end", fill: "#87808f", font_family: "ui-monospace, monospace", font_size: "8", "ANALYTICS & DASHBOARDS" }
                line { x1: "18", y1: "33", x2: "302", y2: "33", stroke: "rgba(44,42,53,0.07)" }

                // Animated rising bar chart
                rect { class: "sg-bar-rise", x: "30", y: "88", width: "22", height: "34", rx: "5", fill: "#f1eae0" }
                rect { class: "sg-bar-rise", x: "62", y: "74", width: "22", height: "48", rx: "5", fill: "#f7cec5" }
                rect { class: "sg-bar-rise", x: "94", y: "60", width: "22", height: "62", rx: "5", fill: "#d97c60" }
                rect { class: "sg-bar-rise", x: "126", y: "48", width: "22", height: "74", rx: "5", fill: "#a21c14" }

                // Rising trend sparkline
                path {
                    class: "sg-draw-line",
                    d: "M 41 80 L 73 66 L 105 52 L 137 40",
                    stroke: "#2c2a35",
                    stroke_width: "2",
                    stroke_linecap: "round",
                }
                circle { cx: "137", cy: "40", r: "3.5", fill: "#2c2a35" }

                // KPI side tile
                rect { x: "172", y: "46", width: "126", height: "76", rx: "12", fill: "#f2eee6", stroke: "rgba(44,42,53,0.08)" }
                text { x: "186", y: "70", fill: "#a21c14", font_family: "Georgia, serif", font_size: "17", font_weight: "700", "~$1.2M/yr" }
                text { x: "186", y: "86", fill: "#5b5663", font_family: "ui-monospace, monospace", font_size: "8.2", "PRICING INSIGHTS" }
                rect { x: "186", y: "96", width: "96", height: "17", rx: "8.5", fill: "#e4f2ec" }
                text { x: "234", y: "107.5", text_anchor: "middle", fill: "#3f7c66", font_family: "ui-monospace, monospace", font_size: "8", font_weight: "700", "NEXT → FINANCE" }
            }
        },
    }
}

#[component]
pub fn JourneySection() -> Element {
    let mut active_idx = use_signal(|| 0usize);

    // Lightweight passive scroll-spy: never traps or hijacks scrolling, and never mutates
    // card heights on scroll — simply updates the sticky left animated story panel as you
    // scroll naturally past each company.
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(
                r#"
                if (window.__ajmStoryScrollHandler) {
                    window.removeEventListener('scroll', window.__ajmStoryScrollHandler);
                }
                window.__ajmStoryLockUntil = 0;

                let lastSent = 0;
                let ticking = false;

                const evaluate = () => {
                    ticking = false;
                    if (Date.now() < (window.__ajmStoryLockUntil || 0)) return;
                    const sec = document.getElementById('journey');
                    if (!sec) return;
                    const secRect = sec.getBoundingClientRect();
                    if (secRect.top > window.innerHeight * 0.8 || secRect.bottom < 100) return;

                    const cards = Array.from(sec.querySelectorAll('.role-card[data-role-idx]'));
                    if (!cards.length) return;

                    const focalY = window.innerHeight * 0.36;
                    let bestIdx = 0;
                    let bestDist = Infinity;

                    cards.forEach((el, i) => {
                        const r = el.getBoundingClientRect();
                        // If the focal line falls inside this card, prefer it directly
                        if (r.top <= focalY && r.bottom >= focalY) {
                            bestIdx = i;
                            bestDist = -1;
                        } else if (bestDist >= 0) {
                            const mid = (r.top + r.bottom) * 0.5;
                            const d = Math.abs(mid - focalY);
                            if (d < bestDist) {
                                bestDist = d;
                                bestIdx = i;
                            }
                        }
                    });

                    if (bestIdx !== lastSent) {
                        lastSent = bestIdx;
                        dioxus.send(bestIdx);
                    }
                };

                const onScroll = () => {
                    if (!ticking) {
                        ticking = true;
                        requestAnimationFrame(evaluate);
                    }
                };

                window.__ajmStoryScrollHandler = onScroll;
                window.__ajmStorySetLast = (idx) => { lastSent = idx; };
                window.addEventListener('scroll', onScroll, { passive: true });
                "#,
            );

            while let Ok(idx) = eval.recv::<usize>().await {
                if idx < ROLES.len() {
                    active_idx.set(idx);
                }
            }
        });
    });

    let current_role = &ROLES[active_idx()];
    let chrono_step = ROLES.len().saturating_sub(active_idx());
    let progress_pct = ((chrono_step as f32) / (ROLES.len() as f32) * 100.0).round() as u32;

    fn jump_to_role(mut active_idx: Signal<usize>, idx: usize, scroll_to_card: bool) {
        active_idx.set(idx);
        let js = if scroll_to_card {
            format!(
                r#"
                window.__ajmStoryLockUntil = Date.now() + 700;
                if (window.__ajmStorySetLast) window.__ajmStorySetLast({idx});
                const card = document.querySelector('.role-card[data-role-idx="{idx}"]');
                if (card) card.scrollIntoView({{ behavior: 'smooth', block: 'center' }});
                "#
            )
        } else {
            format!(
                r#"
                window.__ajmStoryLockUntil = Date.now() + 450;
                if (window.__ajmStorySetLast) window.__ajmStorySetLast({idx});
                "#
            )
        };
        document::eval(&js);
    }

    rsx! {
        section { id: "journey",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "eyebrow", "The Story So Far" }
                    h2 { "Nine years of figuring things out" }
                    p { class: "sec-sub",
                        "From early-stage analytics and rural fintech to Rust-powered search and agentic AI."
                    }
                }

                div { class: "career-layout",
                    // Sticky animated story card on the left — passively updates as you scroll
                    aside {
                        class: format!("about-panel story-panel story-tone-{}", current_role.id),
                        div { class: "story-top-bar",
                            span { class: "story-kicker", "{current_role.story_kicker}" }
                            span { class: "story-period-chip", "{current_role.period}" }
                        }

                        div { class: "story-progress-track", aria_hidden: "true",
                            div {
                                class: "story-progress-fill",
                                style: "width: {progress_pct}%;",
                            }
                        }

                        div { class: "story-stage", key: "{current_role.id}",
                            div { class: "story-art",
                                StoryGraphic { role_id: current_role.id }
                            }

                            div { class: "story-co-head",
                                CompanyLogo { company_id: current_role.id, size: 24 }
                                h3 { "{current_role.story_headline}" }
                            }

                            p { class: "story-body", "{current_role.story_body}" }

                            div { class: "story-takeaway",
                                span { class: "story-takeaway-dot", aria_hidden: "true" }
                                span { "{current_role.story_takeaway}" }
                            }
                        }

                        // Chronological switcher (MuSigma -> Best Buy)
                        div { class: "story-chapters", role: "tablist", aria_label: "Jump to company",
                            for (rev_i, role) in ROLES.iter().enumerate().rev() {
                                {
                                    let is_cur = active_idx() == rev_i;
                                    rsx! {
                                        button {
                                            key: "ch-{role.id}",
                                            class: if is_cur { "story-ch-btn active" } else { "story-ch-btn" },
                                            r#type: "button",
                                            role: "tab",
                                            aria_selected: "{is_cur}",
                                            title: "{role.company} ({role.period})",
                                            onclick: move |_| jump_to_role(active_idx, rev_i, true),
                                            CompanyLogo { company_id: role.id, size: 18 }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "career-right",
                        div { class: "timeline",
                            for (idx, role) in ROLES.iter().enumerate() {
                                {
                                    let id = role.id;
                                    let is_active = active_idx() == idx;
                                    let card_cls = match (role.current, is_active) {
                                        (true, true) => "role-card current active-story",
                                        (true, false) => "role-card current",
                                        (false, true) => "role-card active-story",
                                        (false, false) => "role-card",
                                    };
                                    rsx! {
                                        article {
                                            key: "{id}",
                                            class: "{card_cls}",
                                            "data-role-idx": "{idx}",
                                            onclick: move |_| jump_to_role(active_idx, idx, false),
                                            div { class: "role-trigger",
                                                div {
                                                    div { class: "role-meta",
                                                        if let Some(badge) = role.badge {
                                                            span { class: "role-now", "{badge}" }
                                                        }
                                                        span { "{role.period} · {role.location}" }
                                                    }
                                                    h3 { class: "role-title", "{role.title}" }
                                                    div { class: "role-company co-inline",
                                                        CompanyLogo { company_id: role.id, size: 22 }
                                                        span { "{role.company}" }
                                                    }
                                                }
                                            }

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
