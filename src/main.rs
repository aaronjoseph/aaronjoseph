mod components;
mod data;

use dioxus::prelude::*;

use components::{
    blog::{BlogView, WritingPreviewSection},
    cmd_palette::CommandPalette,
    contact::ContactSection,
    credentials::CredentialsSection,
    hero::Hero,
    journey::JourneySection,
    navbar::Navbar,
    resume::ResumeView,
    stack::StackSection,
    systems::SystemsSection,
};
use data::EMAIL;

const FAVICON: Asset = asset!("/assets/favicon.svg");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    Portfolio,
    Blog,
    Resume,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ResumeStyle {
    Styled,
    Ats,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut view = use_signal(|| AppView::Portfolio);
    let resume_style = use_signal(|| ResumeStyle::Styled);
    let mut selected_project = use_signal(|| Option::<&'static str>::None);
    let mut cmd_open = use_signal(|| false);
    let mut toast = use_signal(|| Option::<String>::None);

    // Cross-fade out the AJM splash loader only after main.css is genuinely applied
    // and webfonts are ready (with bounded timeouts), preventing any FOUC or font reflow jump.
    use_effect(move || {
        document::eval(
            r#"
            (function() {
                const splash = document.getElementById('aj-splash');
                if (!splash) {
                    document.body.classList.add('aj-ready');
                    return;
                }
                if (splash.dataset.waiting) return;
                splash.dataset.waiting = '1';

                let done = false;
                const reveal = () => {
                    if (done) return;
                    done = true;
                    requestAnimationFrame(() => {
                        requestAnimationFrame(() => {
                            document.body.classList.add('aj-ready');
                            splash.classList.add('aj-hidden');
                            setTimeout(() => splash.remove(), 460);
                        });
                    });
                };

                const failsafe = setTimeout(reveal, 1600);

                const isCssApplied = () => {
                    const bg = getComputedStyle(document.documentElement)
                        .getPropertyValue('--bg')
                        .trim();
                    return bg.length > 0;
                };

                const waitForCss = () => new Promise((resolve) => {
                    if (isCssApplied()) return resolve();
                    let tries = 0;
                    const check = () => {
                        if (isCssApplied() || tries++ > 90) return resolve();
                        requestAnimationFrame(check);
                    };
                    requestAnimationFrame(check);
                });

                const waitForFonts = () => {
                    if (!document.fonts || !document.fonts.ready) return Promise.resolve();
                    return Promise.race([
                        document.fonts.ready,
                        new Promise((r) => setTimeout(r, 700)),
                    ]);
                };

                Promise.all([waitForCss(), waitForFonts()]).then(() => {
                    clearTimeout(failsafe);
                    reveal();
                });
            })();
            "#,
        );
    });

    // Global keyboard shortcut for Cmd/Ctrl+K and Escape (closes Command Palette or Deep-Dive Drawer)
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(
                r#"
                if (!window.__ajmKeyBound) {
                    window.__ajmKeyBound = true;
                    window.addEventListener('keydown', (e) => {
                        if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
                            e.preventDefault();
                            dioxus.send('toggle_cmd');
                        } else if (e.key === 'Escape') {
                            dioxus.send('close_overlay');
                        }
                    });
                }
                "#,
            );
            while let Ok(msg) = eval.recv::<String>().await {
                match msg.as_str() {
                    "toggle_cmd" => cmd_open.toggle(),
                    "close_overlay" => {
                        if cmd_open() {
                            cmd_open.set(false);
                        } else if selected_project().is_some() {
                            selected_project.set(None);
                        }
                    }
                    _ => {}
                }
            }
        });
    });

    // Prevent background body scroll while side drawer is open
    use_effect(move || {
        let locked = selected_project().is_some();
        let js = if locked {
            "document.body.style.overflow = 'hidden';"
        } else {
            "document.body.style.overflow = '';"
        };
        document::eval(js);
    });

    let copy_email = move |_| {
        let js = format!(
            "navigator.clipboard && navigator.clipboard.writeText('{EMAIL}').catch(() => {{}});"
        );
        document::eval(&js);
        toast.set(Some(format!("Copied {EMAIL}")));
        spawn(async move {
            let mut timer = document::eval("setTimeout(() => dioxus.send(true), 2400);");
            let _ = timer.recv::<bool>().await;
            toast.set(None);
        });
    };

    let open_resume = move |_| {
        selected_project.set(None);
        view.set(AppView::Resume);
        document::eval("window.scrollTo({ top: 0, behavior: 'instant' });");
    };

    let open_blog = move |_| {
        selected_project.set(None);
        view.set(AppView::Blog);
        document::eval("window.scrollTo({ top: 0, behavior: 'instant' });");
    };

    let go_home = move |_| {
        view.set(AppView::Portfolio);
        document::eval("window.scrollTo({ top: 0, behavior: 'instant' });");
    };

    rsx! {
        document::Link { rel: "icon", r#type: "image/svg+xml", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }

        div { class: "app-shell",
            div { class: "bg-canvas", aria_hidden: "true" }

            Navbar {
                view,
                cmd_open,
            }

            match view() {
                AppView::Portfolio => rsx! {
                    main {
                        Hero {
                            on_open_resume: open_resume,
                            on_open_blog: open_blog,
                            on_copy_email: copy_email,
                        }
                        SystemsSection { selected_project }
                        StackSection {}
                        JourneySection {}
                        WritingPreviewSection { on_open_blog: open_blog }
                        CredentialsSection {}
                        ContactSection {
                            on_copy_email: copy_email,
                            on_open_resume: open_resume,
                        }
                    }
                },
                AppView::Blog => rsx! {
                    main {
                        BlogView { on_back: go_home }
                    }
                },
                AppView::Resume => rsx! {
                    main {
                        ResumeView {
                            style_mode: resume_style,
                            on_back: go_home,
                        }
                    }
                },
            }

            CommandPalette {
                open: cmd_open,
                view,
                resume_style,
                selected_project,
                on_copy_email: copy_email,
            }

            if let Some(msg) = toast() {
                div { class: "toast", role: "status", aria_live: "polite", "{msg}" }
            }
        }
    }
}
