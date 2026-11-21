use dioxus::prelude::*;

use crate::data::{
    CERTIFICATIONS, EDUCATION, EMAIL, GITHUB_URL, LINKEDIN_URL, PHONE, ROLES, SKILL_GROUPS,
    SUMMARY,
};
use crate::ResumeStyle;

#[component]
pub fn ResumeView(style_mode: Signal<ResumeStyle>, on_back: EventHandler<()>) -> Element {
    let mut compact = use_signal(|| false);
    let is_ats = style_mode() == ResumeStyle::Ats;

    rsx! {
        div { class: "resume-page",
            div { class: "resume-toolbar",
                button {
                    class: "btn btn-ghost",
                    r#type: "button",
                    onclick: move |_| on_back.call(()),
                    "← Back to Portfolio"
                }

                div { class: "resume-mode-switch", role: "tablist", aria_label: "Résumé format",
                    button {
                        class: if !is_ats { "mode-pill active" } else { "mode-pill" },
                        r#type: "button",
                        role: "tab",
                        aria_selected: "{!is_ats}",
                        onclick: move |_| style_mode.set(ResumeStyle::Styled),
                        "Styled Résumé"
                    }
                    button {
                        class: if is_ats { "mode-pill active" } else { "mode-pill" },
                        r#type: "button",
                        role: "tab",
                        aria_selected: "{is_ats}",
                        onclick: move |_| style_mode.set(ResumeStyle::Ats),
                        "Plain / ATS-Compliant"
                    }
                }

                div { class: "resume-toolbar-actions",
                    button {
                        class: "btn btn-ghost",
                        r#type: "button",
                        onclick: move |_| compact.toggle(),
                        if compact() { "Standard spacing" } else { "Compact density" }
                    }
                    button {
                        class: "btn btn-primary",
                        r#type: "button",
                        onclick: move |_| {
                            document::eval("window.print();");
                        },
                        if is_ats { "Print / Save ATS PDF" } else { "Print / Save Styled PDF" }
                    }
                }
            }

            if is_ats {
                // Strict single-column, monochrome, parser-friendly ATS layout
                article {
                    class: if compact() { "resume-sheet ats compact" } else { "resume-sheet ats" },
                    header { class: "ats-header",
                        h1 { class: "ats-name", "Aaron Joseph Mathew" }
                        div { class: "ats-contact",
                            a { href: "mailto:{EMAIL}", "{EMAIL}" }
                            span { " | " }
                            span { "{PHONE}" }
                            span { " | " }
                            a { href: "{LINKEDIN_URL}", target: "_blank", rel: "noopener noreferrer", "LinkedIn" }
                            span { " | " }
                            a { href: "{GITHUB_URL}", target: "_blank", rel: "noopener noreferrer", "GitHub" }
                            span { " | Bengaluru, India" }
                        }
                        div { class: "ats-headline",
                            "Senior AI Engineer | Production Search, Learning to Rank, LLM Evaluation, Python & Rust"
                        }
                    }

                    section { class: "ats-section",
                        h2 { "Summary" }
                        p { "{SUMMARY}" }
                    }

                    section { class: "ats-section",
                        h2 { "Skills" }
                        div { class: "ats-skills",
                            for group in SKILL_GROUPS {
                                {
                                    let joined = group
                                        .items
                                        .iter()
                                        .map(|(name, _)| *name)
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    rsx! {
                                        p { class: "ats-skill-line", key: "{group.id}",
                                            strong { "{group.title}: " }
                                            "{joined}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    section { class: "ats-section",
                        h2 { "Experience" }
                        for role in ROLES {
                            div { class: "ats-job", key: "{role.id}",
                                div { class: "ats-job-row",
                                    strong { "{role.title}" }
                                    span { class: "ats-job-right", "{role.company} | {role.location} | {role.period}" }
                                }
                                ul {
                                    for bullet in role.bullets {
                                        li { "{bullet}" }
                                    }
                                }
                            }
                        }
                    }

                    section { class: "ats-section",
                        h2 { "Education" }
                        for edu in EDUCATION {
                            div { class: "ats-edu-row", key: "{edu.school}",
                                strong { "{edu.school}" }
                                span { " — {edu.degree}" }
                                span { class: "ats-job-right", "{edu.location}" }
                            }
                        }
                    }

                    section { class: "ats-section", style: "margin-bottom:0;",
                        h2 { "Certifications" }
                        for cert in CERTIFICATIONS {
                            p { class: "ats-skill-line", key: "{cert.title}",
                                strong { "{cert.title}" }
                                " — {cert.issuer}"
                            }
                        }
                    }
                }
            } else {
                // Styled presentation résumé
                article {
                    class: if compact() { "resume-sheet compact" } else { "resume-sheet" },
                    header { class: "rs-header",
                        div {
                            h1 { class: "rs-name", "Aaron Joseph Mathew" }
                            div { class: "rs-role", "Senior AI Engineer · Search, Ranking & Production ML Systems" }
                        }
                        div { class: "rs-contact",
                            a { href: "mailto:{EMAIL}", "{EMAIL}" }
                            span { "·" }
                            span { "{PHONE}" }
                            span { "·" }
                            a { href: "{LINKEDIN_URL}", target: "_blank", rel: "noopener noreferrer", "LinkedIn" }
                            span { "·" }
                            a { href: "{GITHUB_URL}", target: "_blank", rel: "noopener noreferrer", "GitHub" }
                        }
                    }

                    div { class: "rs-section",
                        h2 { "Summary" }
                        p { class: "rs-summary", "{SUMMARY}" }
                    }

                    div { class: "rs-section",
                        h2 { "Skills" }
                        div { class: "rs-skills",
                            for group in SKILL_GROUPS {
                                {
                                    let joined = group
                                        .items
                                        .iter()
                                        .map(|(name, _)| *name)
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    rsx! {
                                        div { class: "rs-skill-line", key: "{group.id}",
                                            strong { "{group.title}: " }
                                            span { "{joined}" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "rs-section",
                        h2 { "Experience" }
                        for role in ROLES {
                            div { class: "rs-job", key: "{role.id}",
                                div { class: "rs-job-head",
                                    div {
                                        span { class: "rs-job-title", "{role.company}" }
                                        span { " — {role.title}" }
                                    }
                                    span { class: "rs-job-dates", "{role.period} · {role.location}" }
                                }
                                ul {
                                    for bullet in role.bullets {
                                        li { "{bullet}" }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "rs-section",
                        h2 { "Education" }
                        div { class: "rs-edu-grid",
                            for edu in EDUCATION {
                                div { class: "rs-edu-item", key: "{edu.school}",
                                    strong { "{edu.school}" }
                                    span { "{edu.degree} · {edu.location}" }
                                }
                            }
                        }
                    }

                    div { class: "rs-section", style: "margin-bottom:0;",
                        h2 { "Certifications" }
                        div { class: "rs-skills",
                            for cert in CERTIFICATIONS {
                                div { class: "rs-skill-line", key: "{cert.title}",
                                    strong { "{cert.title}" }
                                    span { " — {cert.issuer}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
