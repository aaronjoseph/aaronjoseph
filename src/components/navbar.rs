use dioxus::prelude::*;

use crate::AppView;

#[component]
pub fn Navbar(view: Signal<AppView>, cmd_open: Signal<bool>) -> Element {
    let mut mobile_open = use_signal(|| false);
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
                    button { class: "nav-link-btn", r#type: "button", onclick: move |_| jump_to("#systems"), "Work" }
                    button { class: "nav-link-btn", r#type: "button", onclick: move |_| jump_to("#journey"), "Story" }
                    button { class: "nav-link-btn", r#type: "button", onclick: move |_| jump_to("#stack"), "Toolbox" }
                    button {
                        class: if is_blog { "nav-link-btn active" } else { "nav-link-btn" },
                        r#type: "button",
                        onclick: move |_| open_page(AppView::Blog),
                        "Blog"
                    }
                    button { class: "nav-link-btn", r#type: "button", onclick: move |_| jump_to("#credentials"), "Background" }
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
                        title: "Quick search (Cmd/Ctrl+K)",
                        onclick: move |_| cmd_open.set(true),
                        span { "Search" }
                        span { "⌘K" }
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
                button { r#type: "button", onclick: move |_| jump_to("#systems"), "Selected Work" }
                button { r#type: "button", onclick: move |_| jump_to("#journey"), "The Story So Far" }
                button { r#type: "button", onclick: move |_| jump_to("#stack"), "Toolbox" }
                button { r#type: "button", onclick: move |_| open_page(AppView::Blog), "Blog & Notes" }
                button { r#type: "button", onclick: move |_| jump_to("#credentials"), "Background" }
                button { r#type: "button", onclick: move |_| open_page(AppView::Resume), "Résumé (Styled / ATS)" }
                button { r#type: "button", onclick: move |_| jump_to("#contact"), "Say hello" }
            }
        }
    }
}
