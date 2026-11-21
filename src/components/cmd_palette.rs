use dioxus::prelude::*;

use crate::AppView;

#[derive(Clone, Copy)]
enum CmdAction {
    Section(&'static str),
    Resume,
    CopyEmail,
    ToggleTheme,
}

struct CmdItem {
    label: &'static str,
    hint: &'static str,
    action: CmdAction,
}

const ITEMS: &[CmdItem] = &[
    CmdItem { label: "Jump to Impact & Benchmarks", hint: "#impact", action: CmdAction::Section("#impact") },
    CmdItem { label: "Jump to Selected Systems", hint: "#systems", action: CmdAction::Section("#systems") },
    CmdItem { label: "Jump to Experience Timeline", hint: "#journey", action: CmdAction::Section("#journey") },
    CmdItem { label: "Jump to Technical Stack", hint: "#stack", action: CmdAction::Section("#stack") },
    CmdItem { label: "Jump to Education & Certifications", hint: "#credentials", action: CmdAction::Section("#credentials") },
    CmdItem { label: "Jump to Contact", hint: "#contact", action: CmdAction::Section("#contact") },
    CmdItem { label: "Open Printable / ATS Résumé", hint: "View", action: CmdAction::Resume },
    CmdItem { label: "Copy Email Address", hint: "Clipboard", action: CmdAction::CopyEmail },
    CmdItem { label: "Toggle Dark / Light Theme", hint: "Theme", action: CmdAction::ToggleTheme },
];

#[component]
pub fn CommandPalette(
    open: Signal<bool>,
    view: Signal<AppView>,
    on_copy_email: EventHandler<()>,
    on_toggle_theme: EventHandler<()>,
) -> Element {
    let mut query = use_signal(String::new);

    if !open() {
        return rsx! {};
    }

    let q = query().to_lowercase();
    let filtered: Vec<&CmdItem> = ITEMS
        .iter()
        .filter(|it| {
            q.is_empty()
                || it.label.to_lowercase().contains(&q)
                || it.hint.to_lowercase().contains(&q)
        })
        .collect();

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
            CmdAction::Resume => {
                view.set(AppView::Resume);
                document::eval("window.scrollTo({ top: 0, behavior: 'instant' });");
            }
            CmdAction::CopyEmail => on_copy_email.call(()),
            CmdAction::ToggleTheme => on_toggle_theme.call(()),
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
                aria_label: "Quick navigation",
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
                        placeholder: "Jump to section, open résumé, copy email...",
                        value: "{query}",
                        oninput: move |evt| query.set(evt.value()),
                        onkeydown: move |evt| {
                            if evt.key() == Key::Escape {
                                open.set(false);
                                query.set(String::new());
                            } else if evt.key() == Key::Enter {
                                if let Some(first) = ITEMS.iter().find(|it| {
                                    let q = query().to_lowercase();
                                    q.is_empty()
                                        || it.label.to_lowercase().contains(&q)
                                        || it.hint.to_lowercase().contains(&q)
                                }) {
                                    run_action(first.action);
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
                            "No matching actions."
                        }
                    } else {
                        for item in filtered {
                            {
                                let act = item.action;
                                rsx! {
                                    button {
                                        key: "{item.label}",
                                        class: "cmd-item",
                                        r#type: "button",
                                        onclick: move |_| run_action(act),
                                        span { "{item.label}" }
                                        span { "{item.hint}" }
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
