use dioxus::prelude::*;

use crate::components::company_logo::CompanyLogo;
use crate::data::{
    CalloutStyle, DEEP_DIVE_PROJECTS, MetricTone, StickyTone,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum FrameworkKind {
    Flask,
    FastApi,
    Rust,
    Gcp,
}

#[component]
fn FrameworkLogo(kind: FrameworkKind) -> Element {
    match kind {
        FrameworkKind::Flask => rsx! {
            // Official Flask curved powder-horn logo
            svg {
                class: "fw-logo-svg",
                view_box: "0 0 24 24",
                fill: "none",
                "aria-hidden": "true",
                path {
                    d: "M16.4 2.4 C16.8 2.1 17.3 2.1 17.7 2.4 L21.4 5.1 C21.8 5.4 21.8 6.0 21.5 6.4 L20.4 7.8 C20.1 8.1 19.6 8.2 19.2 8.0 C18.6 11.4 16.8 15.0 13.8 17.6 C10.7 20.3 6.6 21.6 2.8 21.3 C2.2 21.2 2.0 20.5 2.5 20.1 C5.6 17.9 8.4 15.2 10.5 12.1 C12.4 9.2 13.7 6.5 14.4 4.3 C14.1 3.9 14.2 3.4 14.5 3.1 L16.4 2.4 Z",
                    fill: "#2c2a35",
                }
                path {
                    d: "M16.9 6.6 C16.0 9.8 14.2 13.2 11.4 15.8 C9.6 17.4 7.5 18.6 5.5 19.2",
                    stroke: "#ffffff",
                    stroke_width: "1.35",
                    stroke_linecap: "round",
                    opacity: "0.9",
                }
                path {
                    d: "M15.4 3.8 L19.8 7.0",
                    stroke: "#ffffff",
                    stroke_width: "1.15",
                    stroke_linecap: "round",
                }
            }
        },
        FrameworkKind::FastApi => rsx! {
            // Official FastAPI #009688 teal circle + white lightning bolt
            svg {
                class: "fw-logo-svg",
                view_box: "0 0 24 24",
                fill: "none",
                "aria-hidden": "true",
                circle { cx: "12", cy: "12", r: "10.8", fill: "#009688" }
                path {
                    d: "M12.9 4.5 L6.7 13.4 H11.5 L10.7 19.5 L17.3 10.6 H12.4 Z",
                    fill: "#ffffff",
                }
            }
        },
        FrameworkKind::Rust => rsx! {
            // Official Rust toothed gear + 5 rim holes + bold serif 'R'
            svg {
                class: "fw-logo-svg",
                view_box: "0 0 24 24",
                fill: "none",
                "aria-hidden": "true",
                g { fill: "#ce422b",
                    rect { x: "10.85", y: "0.9", width: "2.3", height: "3.2", rx: "0.6", transform: "rotate(0 12 12)" }
                    rect { x: "10.85", y: "0.9", width: "2.3", height: "3.2", rx: "0.6", transform: "rotate(30 12 12)" }
                    rect { x: "10.85", y: "0.9", width: "2.3", height: "3.2", rx: "0.6", transform: "rotate(60 12 12)" }
                    rect { x: "10.85", y: "0.9", width: "2.3", height: "3.2", rx: "0.6", transform: "rotate(90 12 12)" }
                    rect { x: "10.85", y: "0.9", width: "2.3", height: "3.2", rx: "0.6", transform: "rotate(120 12 12)" }
                    rect { x: "10.85", y: "0.9", width: "2.3", height: "3.2", rx: "0.6", transform: "rotate(150 12 12)" }
                    rect { x: "10.85", y: "0.9", width: "2.3", height: "3.2", rx: "0.6", transform: "rotate(180 12 12)" }
                    rect { x: "10.85", y: "0.9", width: "2.3", height: "3.2", rx: "0.6", transform: "rotate(210 12 12)" }
                    rect { x: "10.85", y: "0.9", width: "2.3", height: "3.2", rx: "0.6", transform: "rotate(240 12 12)" }
                    rect { x: "10.85", y: "0.9", width: "2.3", height: "3.2", rx: "0.6", transform: "rotate(270 12 12)" }
                    rect { x: "10.85", y: "0.9", width: "2.3", height: "3.2", rx: "0.6", transform: "rotate(300 12 12)" }
                    rect { x: "10.85", y: "0.9", width: "2.3", height: "3.2", rx: "0.6", transform: "rotate(330 12 12)" }
                    circle { cx: "12", cy: "12", r: "9.4" }
                }
                circle { cx: "12", cy: "12", r: "6.7", fill: "#ffffff" }
                circle { cx: "12", cy: "3.9", r: "0.72", fill: "#ffffff" }
                circle { cx: "19.7", cy: "9.5", r: "0.72", fill: "#ffffff" }
                circle { cx: "16.8", cy: "18.6", r: "0.72", fill: "#ffffff" }
                circle { cx: "7.2", cy: "18.6", r: "0.72", fill: "#ffffff" }
                circle { cx: "4.3", cy: "9.5", r: "0.72", fill: "#ffffff" }
                text {
                    x: "12.2",
                    y: "16.2",
                    text_anchor: "middle",
                    fill: "#ce422b",
                    font_family: "Georgia, 'Times New Roman', serif",
                    font_weight: "900",
                    font_size: "12.4",
                    "R"
                }
            }
        },
        FrameworkKind::Gcp => rsx! {
            // Official 4-color Google Cloud mark
            svg {
                class: "fw-logo-svg",
                view_box: "0 0 24 24",
                fill: "none",
                "aria-hidden": "true",
                path {
                    d: "M15.4 8.2 L17.3 6.3 C15.8 4.9 13.8 4.1 11.6 4.1 C7.6 4.1 4.4 6.9 3.8 10.9 L6.6 11.3 C7.1 8.7 9.2 6.9 11.6 6.9 C13.1 6.9 14.4 7.4 15.4 8.2 Z",
                    fill: "#ea4335",
                }
                path {
                    d: "M19.8 10.1 C19.4 8.5 18.5 7.2 17.3 6.3 L15.4 8.2 C16.3 8.9 16.9 10.0 17.1 11.2 H17.5 C19.0 11.2 20.2 12.4 20.2 13.9 C20.2 15.4 19.0 16.6 17.5 16.6 H12.0 V19.4 H17.5 C20.5 19.4 23.0 17.0 23.0 13.9 C23.0 11.6 21.6 10.4 19.8 10.1 Z",
                    fill: "#4285f4",
                }
                path {
                    d: "M6.7 19.4 H12.0 V16.6 H6.7 C6.2 16.6 5.7 16.4 5.3 16.1 L3.3 18.1 C4.3 18.9 5.5 19.4 6.7 19.4 Z",
                    fill: "#34a853",
                }
                path {
                    d: "M6.7 10.5 C3.8 10.5 1.4 12.5 1.4 15.0 C1.4 16.3 2.1 17.4 3.3 18.1 L5.3 16.1 C4.6 15.7 4.2 15.0 4.2 14.3 C4.2 13.1 5.3 12.1 6.7 12.1 C7.1 12.1 7.5 12.2 7.8 12.4 L9.5 10.6 C8.7 10.5 7.7 10.5 6.7 10.5 Z",
                    fill: "#fbbc05",
                }
            }
        },
    }
}

enum TitleSegment<'a> {
    Text(&'a str),
    Arrow,
    Framework(FrameworkKind, &'a str),
}

fn parse_title_segments(mut input: &str) -> Vec<TitleSegment<'_>> {
    const PATTERNS: &[(&str, Option<FrameworkKind>)] = &[
        ("Rust (Axum)", Some(FrameworkKind::Rust)),
        ("GKE ARM C4A", Some(FrameworkKind::Gcp)),
        ("FastAPI", Some(FrameworkKind::FastApi)),
        ("Flask", Some(FrameworkKind::Flask)),
        ("→", None),
    ];

    let mut out = Vec::new();
    while !input.is_empty() {
        let mut best: Option<(usize, &str, Option<FrameworkKind>)> = None;
        for &(pat, kind) in PATTERNS {
            if let Some(pos) = input.find(pat) {
                let replace = match best {
                    None => true,
                    Some((best_pos, best_pat, _)) => {
                        pos < best_pos || (pos == best_pos && pat.len() > best_pat.len())
                    }
                };
                if replace {
                    best = Some((pos, pat, kind));
                }
            }
        }

        match best {
            Some((pos, pat, kind)) => {
                if pos > 0 {
                    out.push(TitleSegment::Text(&input[..pos]));
                }
                match kind {
                    Some(fw) => out.push(TitleSegment::Framework(fw, pat)),
                    None => out.push(TitleSegment::Arrow),
                }
                input = &input[pos + pat.len()..];
            }
            None => {
                out.push(TitleSegment::Text(input));
                break;
            }
        }
    }
    out
}

/// Renders a project/workstream heading with inline official framework logos & brand colors.
#[component]
pub fn RichTitle(title: &'static str) -> Element {
    let segments = parse_title_segments(title);
    rsx! {
        for (idx, seg) in segments.into_iter().enumerate() {
            match seg {
                TitleSegment::Text(t) => rsx! {
                    span { key: "t-{idx}", "{t}" }
                },
                TitleSegment::Arrow => rsx! {
                    span { key: "a-{idx}", class: "rt-arrow", "→" }
                },
                TitleSegment::Framework(kind, label) => {
                    let cls = match kind {
                        FrameworkKind::Flask => "rt-token rt-flask",
                        FrameworkKind::FastApi => "rt-token rt-fastapi",
                        FrameworkKind::Rust => "rt-token rt-rust",
                        FrameworkKind::Gcp => "rt-token rt-gke",
                    };
                    rsx! {
                        span { key: "f-{idx}", class: "{cls}",
                            FrameworkLogo { kind }
                            span { "{label}" }
                        }
                    }
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
                                        CompanyLogo { company_id: cid, size: 18 }
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
                                        }
                                        h3 { class: "dd-title",
                                            RichTitle { title: project.title }
                                        }
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
                                    div { class: "dd-card-foot",
                                        span { class: "dd-period", "{project.period}" }
                                        span { class: "dd-open-cue", aria_hidden: "true",
                                            span { "Deep dive" }
                                            svg {
                                                width: "13",
                                                height: "13",
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
                                    CompanyLogo { company_id: proj.company_id, size: 22 }
                                    span { class: "dd-company", "{proj.company}" }
                                }
                                span { "· {proj.category} · {proj.period}" }
                            }
                            h2 { class: "drawer-title",
                                RichTitle { title: proj.title }
                            }
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
