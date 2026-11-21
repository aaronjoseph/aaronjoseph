mod components;
mod data;

use dioxus::prelude::*;

use components::{
    cmd_palette::CommandPalette, contact::ContactSection, credentials::CredentialsSection,
    hero::Hero, impact::ImpactSection, journey::JourneySection, navbar::Navbar,
    resume::ResumeView, stack::StackSection, systems::SystemsSection,
};
use data::EMAIL;

const FAVICON: Asset = asset!("/assets/favicon.svg");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    Portfolio,
    Resume,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut theme = use_signal(|| "dark".to_string());
    let mut view = use_signal(|| AppView::Portfolio);
    let mut cmd_open = use_signal(|| false);
    let mut toast = use_signal(|| Option::<String>::None);

    // Load saved theme from localStorage on mount & dismiss the AJ splash loader
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(
                r#"
                const saved = localStorage.getItem('ajm-theme');
                const activeTheme = (saved === 'light' || saved === 'dark') ? saved : 'dark';
                document.documentElement.setAttribute('data-theme', activeTheme);
                const splash = document.getElementById('aj-splash');
                if (splash) {
                    const dismiss = () => {
                        if (splash.dataset.dismissed) return;
                        splash.dataset.dismissed = '1';
                        requestAnimationFrame(() => {
                            splash.classList.add('aj-hidden');
                            setTimeout(() => splash.remove(), 320);
                        });
                    };
                    const css = document.querySelector('link[rel="stylesheet"][href*="main"]');
                    if (!css || css.sheet) {
                        dismiss();
                    } else {
                        css.addEventListener('load', dismiss, { once: true });
                        css.addEventListener('error', dismiss, { once: true });
                        setTimeout(dismiss, 800);
                    }
                }
                dioxus.send(activeTheme);
                "#,
            );
            if let Ok(val) = eval.recv::<String>().await {
                if val == "light" || val == "dark" {
                    theme.set(val);
                }
            }
        });
    });

    // Global keyboard shortcut for Cmd/Ctrl+K and Escape
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
                            dioxus.send('close_cmd');
                        }
                    });
                }
                "#,
            );
            while let Ok(msg) = eval.recv::<String>().await {
                match msg.as_str() {
                    "toggle_cmd" => cmd_open.toggle(),
                    "close_cmd" => cmd_open.set(false),
                    _ => {}
                }
            }
        });
    });

    let toggle_theme = move |_| {
        let next = if theme() == "dark" { "light" } else { "dark" };
        theme.set(next.to_string());
        let js = format!(
            "document.documentElement.setAttribute('data-theme', '{next}'); try {{ localStorage.setItem('ajm-theme', '{next}'); }} catch(e) {{}}"
        );
        document::eval(&js);
    };

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
        view.set(AppView::Resume);
        document::eval("window.scrollTo({ top: 0, behavior: 'instant' });");
    };

    rsx! {
        document::Link { rel: "icon", r#type: "image/svg+xml", href: FAVICON }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500;600&family=Plus+Jakarta+Sans:wght@400;500;600;700&family=Space+Grotesk:wght@500;600;700&display=swap",
        }
        document::Stylesheet { href: MAIN_CSS }

        div { class: "app-shell", "data-theme": "{theme}",
            div { class: "bg-canvas", aria_hidden: "true" }

            Navbar {
                theme,
                view,
                cmd_open,
                on_toggle_theme: toggle_theme,
            }

            if view() == AppView::Portfolio {
                main {
                    Hero {
                        on_open_resume: open_resume,
                        on_copy_email: copy_email,
                    }
                    ImpactSection {}
                    SystemsSection {}
                    JourneySection {}
                    StackSection {}
                    CredentialsSection {}
                    ContactSection {
                        on_copy_email: copy_email,
                        on_open_resume: open_resume,
                    }
                }
            } else {
                ResumeView {
                    on_back: move |_| {
                        view.set(AppView::Portfolio);
                        document::eval("window.scrollTo({ top: 0, behavior: 'instant' });");
                    },
                }
            }

            CommandPalette {
                open: cmd_open,
                view,
                on_copy_email: copy_email,
                on_toggle_theme: toggle_theme,
            }

            if let Some(msg) = toast() {
                div { class: "toast", role: "status", aria_live: "polite", "{msg}" }
            }
        }
    }
}
