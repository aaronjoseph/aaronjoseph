use dioxus::prelude::*;

use crate::AppView;

#[component]
pub fn Navbar(
    theme: Signal<String>,
    view: Signal<AppView>,
    cmd_open: Signal<bool>,
    on_toggle_theme: EventHandler<()>,
) -> Element {
    let mut mobile_open = use_signal(|| false);
    let is_dark = theme() == "dark";
    let is_blog = view() == AppView::Blog;
    let is_resume = view() == AppView::Resume;

    let mut jump_to = move |anchor: &'static str| {
        mobile_open.set(false);
        view.set(AppView::Portfolio);
        let js = format!(
            "setTimeout(() => {{ const el = document.querySelector('{anchor}'); if (el) el.scrollIntoView({{ behavior: 'smooth' }}); }}, 20);"
        );
        document::eval(&js);
    };

    let mut open_page = move |target: AppView| {
        mobile_open.set(false);
        view.set(target);
        document::eval("window.scrollTo({ top: 0, behavior: 'instant' });");
    };

    rsx! {
        header { class: "navbar",
            div { class: "wrap nav-inner",
                button {
                    class: "brand",
                    r#type: "button",
                    aria_label: "Aaron Joseph Mathew Home",
                    onclick: move |_| {
                        mobile_open.set(false);
                        view.set(AppView::Portfolio);
                        document::eval("window.scrollTo({ top: 0, behavior: 'smooth' });");
                    },
                    span { class: "brand-mark", "AJM" }
                    span {
                        "Aaron "
                        span { class: "brand-sub", "Mathew" }
                    }
                }

                nav { class: "nav-links", aria_label: "Primary",
                    button { class: "nav-link-btn", r#type: "button", onclick: move |_| jump_to("#systems"), "Systems" }
                    button { class: "nav-link-btn", r#type: "button", onclick: move |_| jump_to("#journey"), "Experience" }
                    button { class: "nav-link-btn", r#type: "button", onclick: move |_| jump_to("#stack"), "Stack" }
                    button {
                        class: if is_blog { "nav-link-btn active" } else { "nav-link-btn" },
                        r#type: "button",
                        onclick: move |_| open_page(AppView::Blog),
                        "Blog"
                    }
                    button { class: "nav-link-btn", r#type: "button", onclick: move |_| jump_to("#credentials"), "Credentials" }
                    button {
                        class: if is_resume { "nav-link-btn active" } else { "nav-link-btn" },
                        r#type: "button",
                        onclick: move |_| open_page(AppView::Resume),
                        "Résumé"
                    }
                }

                div { class: "nav-actions",
                    button {
                        class: "kbd-btn",
                        r#type: "button",
                        title: "Quick jump (Cmd/Ctrl+K)",
                        onclick: move |_| cmd_open.set(true),
                        span { "Search" }
                        span { "⌘K" }
                    }

                    button {
                        class: "icon-btn",
                        r#type: "button",
                        aria_label: "Toggle color theme",
                        title: if is_dark { "Switch to warm light mode" } else { "Switch to dark mode" },
                        onclick: move |_| on_toggle_theme.call(()),
                        if is_dark {
                            svg {
                                width: "17",
                                height: "17",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                circle { cx: "12", cy: "12", r: "4" }
                                path { d: "M12 2v2m0 16v2M4.93 4.93l1.41 1.41m11.32 11.32 1.41 1.41M2 12h2m16 0h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41" }
                            }
                        } else {
                            svg {
                                width: "17",
                                height: "17",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                            }
                        }
                    }

                    button {
                        class: "nav-cta",
                        r#type: "button",
                        onclick: move |_| jump_to("#contact"),
                        "Say hello"
                    }

                    button {
                        class: "icon-btn mobile-menu-btn",
                        r#type: "button",
                        aria_label: "Toggle navigation menu",
                        aria_expanded: "{mobile_open()}",
                        onclick: move |_| mobile_open.toggle(),
                        svg {
                            width: "18",
                            height: "18",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            path { d: "M4 7h16M4 12h16M4 17h16" }
                        }
                    }
                }
            }
        }

        if mobile_open() {
            nav { class: "mobile-drawer", aria_label: "Mobile Navigation",
                button { r#type: "button", onclick: move |_| jump_to("#systems"), "Architecture & Outcomes" }
                button { r#type: "button", onclick: move |_| jump_to("#journey"), "Experience" }
                button { r#type: "button", onclick: move |_| jump_to("#stack"), "Technical Stack" }
                button { r#type: "button", onclick: move |_| open_page(AppView::Blog), "Blog & Writing" }
                button { r#type: "button", onclick: move |_| jump_to("#credentials"), "Education & Certs" }
                button { r#type: "button", onclick: move |_| open_page(AppView::Resume), "Résumé (Styled / ATS)" }
                button { r#type: "button", onclick: move |_| jump_to("#contact"), "Say hello" }
            }
        }
    }
}
