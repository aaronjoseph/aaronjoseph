use dioxus::prelude::*;

use crate::data::{DEEP_DIVE_COMPANIES, DEEP_DIVE_PROJECTS, MetricTone};

#[component]
pub fn SystemsSection(selected_project: Signal<Option<&'static str>>) -> Element {
    let mut company_filter = use_signal(|| "all");

    let active_idx = selected_project().and_then(|id| {
        DEEP_DIVE_PROJECTS.iter().position(|p| p.id == id)
    });

    let active_project = active_idx.map(|i| &DEEP_DIVE_PROJECTS[i]);

    rsx! {
        section { id: "systems",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "eyebrow", "Architecture & Outcomes" }
                    h2 { "Systems I've designed & shipped" }
                    p { class: "sec-sub",
                        "Click any project from Best Buy, Falabella, or Accenture AI to inspect its measured before/after deltas, architecture flow, and technical deep-dive notes."
                    }
                }

                div { class: "filter-bar", role: "group", aria_label: "Filter projects by company",
                    for (cid, label) in DEEP_DIVE_COMPANIES {
                        {
                            let cid = *cid;
                            let count = if cid == "all" {
                                DEEP_DIVE_PROJECTS.len()
                            } else {
                                DEEP_DIVE_PROJECTS.iter().filter(|p| p.company_id == cid).count()
                            };
                            rsx! {
                                button {
                                    key: "{cid}",
                                    class: if company_filter() == cid { "filter-pill active" } else { "filter-pill" },
                                    r#type: "button",
                                    onclick: move |_| company_filter.set(cid),
                                    "{label} ({count})"
                                }
                            }
                        }
                    }
                }

                div { class: "dd-list",
                    for project in DEEP_DIVE_PROJECTS.iter().filter(|p| company_filter() == "all" || p.company_id == company_filter()) {
                        {
                            let pid = project.id;
                            let is_active = selected_project() == Some(pid);
                            rsx! {
                                button {
                                    key: "{pid}",
                                    class: if is_active { "dd-row active" } else { "dd-row" },
                                    r#type: "button",
                                    onclick: move |_| selected_project.set(Some(pid)),
                                    div { class: "dd-row-left",
                                        span { class: "dd-idx", "{project.index}" }
                                        div { class: "dd-row-main",
                                            div { class: "dd-meta",
                                                span { class: "dd-company", "{project.company}" }
                                                span { class: "dd-sep", "·" }
                                                span { "{project.category}" }
                                                span { class: "dd-sep", "·" }
                                                span { "{project.period}" }
                                            }
                                            h3 { class: "dd-title", "{project.title}" }
                                            p { class: "dd-oneliner", "{project.one_liner}" }
                                            div { class: "t-gains",
                                                for (label, tone) in project.headline_badges {
                                                    span {
                                                        key: "{label}",
                                                        class: match tone {
                                                            MetricTone::Signal => "t-gain signal",
                                                            MetricTone::Accent => "t-gain accent",
                                                            MetricTone::Default => "t-gain",
                                                        },
                                                        "{label}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    span { class: "dd-open-cue", aria_hidden: "true",
                                        span { "Deep dive" }
                                        svg {
                                            width: "15",
                                            height: "15",
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_width: "2.2",
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            path { d: "M5 12h14m-6-6 6 6-6 6" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Slide-over Architecture Deep-Dive Drawer
        if let (Some(idx), Some(proj)) = (active_idx, active_project) {
            div {
                class: "drawer-backdrop",
                onclick: move |_| selected_project.set(None),
                aside {
                    class: "drawer-panel",
                    role: "dialog",
                    aria_modal: "true",
                    aria_label: "{proj.title}",
                    onclick: move |evt| evt.stop_propagation(),

                    div { class: "drawer-header",
                        div {
                            div { class: "drawer-kicker",
                                span { class: "dd-idx", "{proj.index}" }
                                span { class: "dd-company", "{proj.company}" }
                                span { "· {proj.category} · {proj.period}" }
                            }
                            h2 { class: "drawer-title", "{proj.title}" }
                            p { class: "drawer-sub", "{proj.one_liner}" }
                        }
                        button {
                            class: "icon-btn drawer-close",
                            r#type: "button",
                            aria_label: "Close deep dive",
                            title: "Close (Esc)",
                            onclick: move |_| selected_project.set(None),
                            svg {
                                width: "17",
                                height: "17",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M18 6 6 18M6 6l12 12" }
                            }
                        }
                    }

                    div { class: "drawer-body",
                        // Measured Outcomes (Key KPIs + Before/After Bars)
                        div { class: "drawer-block",
                            div { class: "drawer-block-label", "MEASURED OUTCOMES" }

                            div { class: "drawer-kpi-grid",
                                for (val, lbl, tone) in proj.key_outcomes {
                                    div { class: "drawer-kpi", key: "{lbl}",
                                        div {
                                            class: match tone {
                                                MetricTone::Signal => "drawer-kpi-val signal",
                                                MetricTone::Accent => "drawer-kpi-val accent",
                                                MetricTone::Default => "drawer-kpi-val",
                                            },
                                            "{val}"
                                        }
                                        div { class: "drawer-kpi-lbl", "{lbl}" }
                                    }
                                }
                            }

                            if !proj.meters.is_empty() {
                                div { class: "drawer-meters",
                                    for m in proj.meters {
                                        div { class: "drawer-meter-card", key: "{m.label}",
                                            div { class: "drawer-meter-title", "{m.label}" }
                                            div { class: "meter-row",
                                                span { class: "meter-lbl", "Before" }
                                                div { class: "meter-track",
                                                    div {
                                                        class: "meter-fill before",
                                                        style: "width: {m.before_pct}%;",
                                                    }
                                                }
                                                span { class: "meter-val", "{m.before_label}" }
                                            }
                                            div { class: "meter-row",
                                                span { class: "meter-lbl", "After" }
                                                div { class: "meter-track",
                                                    div {
                                                        class: if m.accent_after { "meter-fill after-accent" } else { "meter-fill after" },
                                                        style: "width: {m.after_pct}%;",
                                                    }
                                                }
                                                span { class: "meter-val good", "{m.after_label}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Architecture Flow
                        div { class: "drawer-block",
                            div { class: "drawer-block-label", "ARCHITECTURE FLOW" }
                            div { class: "case-flow", "{proj.flow}" }
                        }

                        // Deep-Dive Sections
                        for sec in proj.sections {
                            div { class: "drawer-block", key: "{sec.heading}",
                                div { class: "drawer-block-label", "{sec.heading}" }
                                ul { class: "drawer-list",
                                    for pt in sec.points {
                                        li { "{pt}" }
                                    }
                                }
                            }
                        }

                        // Tech Stack
                        div { class: "drawer-block",
                            div { class: "drawer-block-label", "STACK & COMPONENTS" }
                            div { class: "chip-row",
                                for (name, is_core) in proj.stack {
                                    span {
                                        key: "{name}",
                                        class: if *is_core { "chip core" } else { "chip" },
                                        "{name}"
                                    }
                                }
                            }
                        }
                    }

                    div { class: "drawer-footer",
                        button {
                            class: "text-btn",
                            r#type: "button",
                            disabled: idx == 0,
                            onclick: move |_| {
                                if idx > 0 {
                                    selected_project.set(Some(DEEP_DIVE_PROJECTS[idx - 1].id));
                                }
                            },
                            "← Previous project"
                        }
                        span { class: "drawer-pager", "{idx + 1} / {DEEP_DIVE_PROJECTS.len()}" }
                        button {
                            class: "text-btn",
                            r#type: "button",
                            disabled: idx + 1 >= DEEP_DIVE_PROJECTS.len(),
                            onclick: move |_| {
                                if idx + 1 < DEEP_DIVE_PROJECTS.len() {
                                    selected_project.set(Some(DEEP_DIVE_PROJECTS[idx + 1].id));
                                }
                            },
                            "Next project →"
                        }
                    }
                }
            }
        }
    }
}
