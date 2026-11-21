use dioxus::prelude::*;

use crate::data::{CERTIFICATIONS, EDUCATION};

#[component]
pub fn CredentialsSection() -> Element {
    rsx! {
        section { id: "credentials",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "eyebrow", "Background" }
                    h2 { "School & certifications" }
                }

                div { class: "cred-grid",
                    div { class: "cred-col",
                        for edu in EDUCATION {
                            if let Some((verify_label, href)) = edu.url {
                                a {
                                    key: "{edu.school}",
                                    class: "cred-card cert-link",
                                    href: "{href}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    div {
                                        div { class: "cred-type", "{edu.kind} · {verify_label}" }
                                        h3 { "{edu.school}" }
                                        p { class: "cred-sub", "{edu.degree} · {edu.location}" }
                                    }
                                    span { class: "cert-arrow", aria_hidden: "true", "↗" }
                                }
                            } else {
                                div { class: "cred-card", key: "{edu.school}",
                                    div { class: "cred-type", "{edu.kind}" }
                                    h3 { "{edu.school}" }
                                    p { class: "cred-sub", "{edu.degree} · {edu.location}" }
                                }
                            }
                        }
                    }

                    div { class: "cred-col",
                        for cert in CERTIFICATIONS {
                            a {
                                key: "{cert.title}",
                                class: "cred-card cert-link",
                                href: "{cert.url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                div {
                                    div { class: "cred-type", "{cert.issuer}" }
                                    h3 { "{cert.title}" }
                                    p { class: "cred-sub", "{cert.subtitle}" }
                                }
                                span { class: "cert-arrow", aria_hidden: "true", "↗" }
                            }
                        }
                    }
                }
            }
        }
    }
}
