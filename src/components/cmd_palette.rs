use dioxus::prelude::*;

use crate::data::DEEP_DIVE_PROJECTS;
use crate::{AppView, ResumeStyle};

#[derive(Clone, Copy)]
enum CmdAction {
    Section(&'static str),
    Project(&'static str),
    Blog,
    ResumeStyled,
    ResumeAts,
    CopyEmail,
}

struct CmdItem {
    label: String,
    hint: &'static str,
    action: CmdAction,
}

#[component]
pub fn CommandPalette(
    open: Signal<bool>,
    view: Signal<AppView>,
    resume_style: Signal<ResumeStyle>,
    selected_project: Signal<Option<&'static str>>,
    on_copy_email: EventHandler<()>,
) -> Element {
    let mut query = use_signal(String::new);

    if !open() {
        return rsx! {};
    }

    let mut items = vec![
        CmdItem { label: "Open Blog & Notes Page".into(), hint: "Page", action: CmdAction::Blog },
        CmdItem { label: "Jump to Selected Work".into(), hint: "#systems", action: CmdAction::Section("#systems") },
        CmdItem { label: "Jump to The Story So Far".into(), hint: "#journey", action: CmdAction::Section("#journey") },
        CmdItem { label: "Jump to Toolbox".into(), hint: "#stack", action: CmdAction::Section("#stack") },
        CmdItem { label: "Jump to School & Certifications".into(), hint: "#credentials", action: CmdAction::Section("#credentials") },
        CmdItem { label: "Jump to Say Hello".into(), hint: "#contact", action: CmdAction::Section("#contact") },
        CmdItem { label: "Open Styled Résumé".into(), hint: "Styled", action: CmdAction::ResumeStyled },
        CmdItem { label: "Open Plain / ATS-Compliant Résumé".into(), hint: "ATS", action: CmdAction::ResumeAts },
        CmdItem { label: "Copy Email Address".into(), hint: "Clipboard", action: CmdAction::CopyEmail },
    ];

    for p in DEEP_DIVE_PROJECTS {
        items.push(CmdItem {
            label: format!("Read deep dive: {} ({})", p.title, p.company),
            hint: "Project",
            action: CmdAction::Project(p.id),
        });
    }

    let q = query().to_lowercase();
    let filtered: Vec<CmdItem> = items
        .into_iter()
        .filter(|it| {
            q.is_empty()
                || it.label.to_lowercase().contains(&q)
                || it.hint.to_lowercase().contains(&q)
        })
        .collect();

    let first_action = filtered.first().map(|it| it.action);

    let mut run_action = move |action: CmdAction| {
        open.set(false);
        query.set(String::new());
        match action {
            CmdAction::Section(sel) => {
                view.set(AppView::Portfolio);
                let js = format!(
                    "setTimeout(() => {{ const el = document.querySelector('{sel}'); if (el) el.scrollIntoView({{ behavior: 'smooth' }}); }}, 20);"
                );
                document::eval(&js);
            }
            CmdAction::Project(pid) => {
                view.set(AppView::Portfolio);
                selected_project.set(Some(pid));
                document::eval(
                    "setTimeout(() => { const el = document.querySelector('#systems'); if (el) el.scrollIntoView({ behavior: 'smooth' }); }, 20);",
                );
            }
            CmdAction::Blog => {
                selected_project.set(None);
                view.set(AppView::Blog);
                document::eval("window.scrollTo({ top: 0, behavior: 'instant' });");
            }
            CmdAction::ResumeStyled => {
                selected_project.set(None);
                resume_style.set(ResumeStyle::Styled);
                view.set(AppView::Resume);
                document::eval("window.scrollTo({ top: 0, behavior: 'instant' });");
            }
            CmdAction::ResumeAts => {
                selected_project.set(None);
                resume_style.set(ResumeStyle::Ats);
                view.set(AppView::Resume);
                document::eval("window.scrollTo({ top: 0, behavior: 'instant' });");
            }
            CmdAction::CopyEmail => on_copy_email.call(()),
        }
    };

    rsx! {
        div {
            class: "cmd-backdrop",
            onclick: move |_| {
                open.set(false);
                query.set(String::new());
            },
            div {
                class: "cmd-dialog",
                role: "dialog",
                aria_modal: "true",
                aria_label: "Quick search",
                onclick: move |evt| evt.stop_propagation(),
                div { class: "cmd-input-wrap",
                    svg {
                        width: "16",
                        height: "16",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        style: "color: var(--text-3);",
                        circle { cx: "11", cy: "11", r: "8" }
                        path { d: "m21 21-4.3-4.3" }
                    }
                    input {
                        class: "cmd-input",
                        r#type: "text",
                        autofocus: true,
                        placeholder: "Jump to a section, project deep dive, blog, or résumé...",
                        value: "{query}",
                        oninput: move |evt| query.set(evt.value()),
                        onkeydown: move |evt| {
                            if evt.key() == Key::Escape {
                                open.set(false);
                                query.set(String::new());
                            } else if evt.key() == Key::Enter {
                                if let Some(act) = first_action {
                                    run_action(act);
                                }
                            }
                        }
                    }
                    span {
                        style: "font-family: var(--font-mono); font-size: 0.72rem; color: var(--text-3);",
                        "ESC"
                    }
                }

                div { class: "cmd-list",
                    if filtered.is_empty() {
                        div {
                            style: "padding: 14px 12px; color: var(--text-3); font-size: 0.88rem;",
                            "Nothing matching that — try 'blog', 'resume', or 'Rust'."
                        }
                    } else {
                        for item in filtered {
                            {
                                let act = item.action;
                                let label = item.label.clone();
                                let hint = item.hint;
                                rsx! {
                                    button {
                                        key: "{label}",
                                        class: "cmd-item",
                                        r#type: "button",
                                        onclick: move |_| run_action(act),
                                        span { "{label}" }
                                        span { "{hint}" }
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
