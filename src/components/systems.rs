use dioxus::prelude::*;

use crate::components::company_logo::CompanyLogo;
use crate::data::{
    CalloutStyle, DEEP_DIVE_PROJECTS, MetricTone, StickyTone, TechKind,
};

#[component]
pub fn TechEvolutionStrip(steps: &'static [TechKind]) -> Element {
    if steps.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "tech-evo-strip", aria_label: "Technology progression",
            for (i, step) in steps.iter().enumerate() {
                if i > 0 {
                    span { class: "tech-evo-arrow", "→" }
                }
                match step {
                    TechKind::Flask => rsx! {
                        span { class: "tech-pill tp-flask",
                            span { class: "tp-dot" }
                            "Flask"
                        }
                    },
                    TechKind::FastApi => rsx! {
                        span { class: "tech-pill tp-fastapi",
                            svg {
                                width: "12",
                                height: "12",
                                view_box: "0 0 24 24",
                                fill: "currentColor",
                                "aria-hidden": "true",
                                path { d: "M13 2 3 14h7l-1 8 10-12h-7l1-8z" }
                            }
                            "FastAPI"
                        }
                    },
                    TechKind::RustAxum => rsx! {
                        span { class: "tech-pill tp-rust",
                            // Recognizable Rust cog/gear emblem
                            svg {
                                width: "14",
                                height: "14",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                "aria-hidden": "true",
                                circle { cx: "12", cy: "12", r: "4.2" }
                                circle { cx: "12", cy: "12", r: "8.2", stroke_dasharray: "2.4 2.4" }
                                path { d: "M12 1.8v2.4M12 19.8v2.4M1.8 12h2.4M19.8 12h2.4M4.8 4.8l1.7 1.7M17.5 17.5l1.7 1.7M19.2 4.8l-1.7 1.7M6.5 17.5l-1.7 1.7" }
                            }
                            "Rust (Axum)"
                        }
                    },
                    TechKind::GkeArm => rsx! {
                        span { class: "tech-pill tp-arm",
                            "GKE ARM C4A"
                        }
                    },
                }
            }
        }
    }
}

#[component]
pub fn SystemsSection(selected_project: Signal<Option<&'static str>>) -> Element {
    let mut company_filter = use_signal(|| "all");

    let mut companies: Vec<(&'static str, &'static str)> = vec![("all", "All Projects")];
    for p in DEEP_DIVE_PROJECTS {
        if !companies.iter().any(|(id, _)| *id == p.company_id) {
            companies.push((p.company_id, p.company));
        }
    }

    let active_idx = selected_project().and_then(|id| {
        DEEP_DIVE_PROJECTS.iter().position(|p| p.id == id)
    });

    let active_project = active_idx.map(|i| &DEEP_DIVE_PROJECTS[i]);

    rsx! {
        section { id: "systems",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "eyebrow", "Key Work" }
                    h2 { "Things I've built (and why they mattered)" }
                    p { class: "sec-sub",
                        "A quiet engineering log of systems I've built over the years. Pick any project to open the full breakdown on the side."
                    }
                }

                div { class: "filter-bar", role: "group", aria_label: "Filter projects by company",
                    for (cid, label) in companies {
                        {
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
                                    if cid != "all" {
                                        CompanyLogo { company_id: cid, size: 16 }
                                    }
                                    span { "{label} ({count})" }
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
                            let row_class = if is_active {
                                format!("dd-row co-{} active", project.company_id)
                            } else {
                                format!("dd-row co-{}", project.company_id)
                            };
                            rsx! {
                                button {
                                    key: "{pid}",
                                    class: "{row_class}",
                                    r#type: "button",
                                    onclick: move |_| selected_project.set(Some(pid)),
                                    div { class: "dd-row-main",
                                        div { class: "dd-meta",
                                            span { class: "co-inline",
                                                CompanyLogo { company_id: project.company_id, size: 20 }
                                                span { class: "dd-company", "{project.company}" }
                                            }
                                            span { class: "dd-sep", "·" }
                                            span { "{project.category}" }
                                            span { class: "dd-sep", "·" }
                                            span { "{project.period}" }
                                        }
                                        if !project.tech_steps.is_empty() {
                                            TechEvolutionStrip { steps: project.tech_steps }
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
                                span { class: "co-inline",
                                    CompanyLogo { company_id: proj.company_id, size: 20 }
                                    span { class: "dd-company", "{proj.company}" }
                                }
                                span { "· {proj.category} · {proj.period}" }
                            }
                            if !proj.tech_steps.is_empty() {
                                TechEvolutionStrip { steps: proj.tech_steps }
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
                        // Handcrafted Ruled-Paper Engineering Notebook Sheet
                        if !proj.comparisons.is_empty() || !proj.callouts.is_empty() || !proj.sticky_notes.is_empty() {
                            div { class: "nb-sheet",
                                div { class: "nb-sheet-head",
                                    span { class: "nb-stamp", "Architecture Field Notes" }
                                    span { class: "nb-pg", "pg {idx + 1}" }
                                }

                                if !proj.comparisons.is_empty() {
                                    div { class: "nb-comp-list",
                                        for c in proj.comparisons {
                                            div { class: "nb-comp-row", key: "{c.label}",
                                                div { class: "nb-comp-label", "{c.label}" }
                                                div { class: "nb-comp-flow",
                                                    span { class: "nb-before", "{c.before}" }
                                                    span { class: "nb-arrow", "→" }
                                                    span { class: "nb-after", "{c.after}" }
                                                    span {
                                                        class: match c.tone {
                                                            MetricTone::Signal => "nb-delta green",
                                                            MetricTone::Accent => "nb-delta orange",
                                                            MetricTone::Default => "nb-delta yellow",
                                                        },
                                                        "{c.delta}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                for callout in proj.callouts {
                                    div {
                                        key: "{callout.title}",
                                        class: match callout.style {
                                            CalloutStyle::PurpleDashed => "nb-callout nb-purple",
                                            CalloutStyle::GreenCheck => "nb-callout nb-green",
                                        },
                                        div { class: "nb-callout-title",
                                            if callout.style == CalloutStyle::GreenCheck {
                                                span { class: "nb-check", "✓ " }
                                            }
                                            "{callout.title}"
                                        }
                                        p { class: "nb-callout-body", "{callout.body}" }
                                        if let Some((lbl, url)) = callout.link {
                                            a {
                                                class: "nb-callout-link",
                                                href: "{url}",
                                                target: "_blank",
                                                rel: "noopener noreferrer",
                                                "{lbl}"
                                            }
                                        }
                                    }
                                }

                                if !proj.sticky_notes.is_empty() {
                                    div { class: "nb-sticky-grid",
                                        for note in proj.sticky_notes {
                                            div {
                                                key: "{note.title}",
                                                class: match note.tone {
                                                    StickyTone::Yellow => "nb-sticky sticky-yellow",
                                                    StickyTone::Pink => "nb-sticky sticky-pink",
                                                },
                                                div { class: "nb-sticky-tape", aria_hidden: "true" }
                                                div { class: "nb-sticky-title", "{note.title}" }
                                                ul { class: "nb-sticky-list",
                                                    for item in note.items {
                                                        li { "{item}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            // Compact KPI strip for projects without a notebook sheet yet
                            div { class: "drawer-block",
                                div { class: "drawer-block-label", "KEY OUTCOMES" }
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
