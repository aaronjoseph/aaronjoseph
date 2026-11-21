use dioxus::prelude::*;

use crate::data::{
    CERTIFICATIONS, EDUCATION, EMAIL, GITHUB_URL, LINKEDIN_URL, PHONE, ROLES, SKILL_GROUPS,
    SUMMARY,
};

#[component]
pub fn ResumeView(on_back: EventHandler<()>) -> Element {
    let mut compact = use_signal(|| false);

    rsx! {
        div { class: "resume-page",
            div { class: "resume-toolbar",
                button {
                    class: "btn btn-ghost",
                    r#type: "button",
                    onclick: move |_| on_back.call(()),
                    "← Back to Portfolio"
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
                        "Print / Save PDF"
                    }
                }
            }

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
