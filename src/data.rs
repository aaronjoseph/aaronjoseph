#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MetricTone {
    Default,
    Accent,
    Signal,
}

pub struct TelemetryMetric {
    pub label: &'static str,
    pub value: &'static str,
    pub detail: &'static str,
    pub tone: MetricTone,
}

pub struct TelemetryTab {
    pub id: &'static str,
    pub label: &'static str,
    pub arch_prefix: &'static str,
    pub arch_highlight: &'static str,
    pub arch_mid: &'static str,
    pub arch_signal: &'static str,
    pub metrics: &'static [TelemetryMetric],
    pub note: &'static str,
}

pub struct ImpactMeter {
    pub before_label: &'static str,
    pub before_pct: u8,
    pub after_label: &'static str,
    pub after_pct: u8,
    pub accent_after: bool,
}

pub struct ImpactCard {
    pub tag: &'static str,
    pub org: &'static str,
    pub headline: &'static str,
    pub title: &'static str,
    pub desc: &'static str,
    pub wide: bool,
    pub meter: Option<ImpactMeter>,
}

pub struct CaseStudy {
    pub kicker: &'static str,
    pub place: &'static str,
    pub title: &'static str,
    pub body: &'static str,
    pub flow: &'static str,
    pub chips: &'static [(&'static str, &'static str)], // (label, class: "core" | "signal" | "")
}

pub struct Role {
    pub id: &'static str,
    pub company: &'static str,
    pub title: &'static str,
    pub period: &'static str,
    pub location: &'static str,
    pub badge: Option<&'static str>,
    pub current: bool,
    pub default_open: bool,
    pub bullets: &'static [&'static str],
}

pub struct SkillGroup {
    pub id: &'static str,
    pub filter_label: &'static str,
    pub title: &'static str,
    pub items: &'static [(&'static str, bool)], // (name, is_core)
}

pub struct Education {
    pub kind: &'static str,
    pub school: &'static str,
    pub degree: &'static str,
    pub location: &'static str,
}

pub struct Certification {
    pub issuer: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub url: &'static str,
}

pub const EMAIL: &str = "aaronjosephmathew@gmail.com";
pub const PHONE: &str = "+91 9844348064";
pub const LINKEDIN_URL: &str = "https://www.linkedin.com/in/aaron-joseph-mathew-4b425b71/";
pub const GITHUB_URL: &str = "https://github.com/aaronjoseph";
pub const X_URL: &str = "https://twitter.com/AaronJosephMath";

pub const SUMMARY: &str = "Senior AI Engineer with 9+ years across analytics, machine learning and AI engineering, focused on production search, ranking and reliable ML systems. Worked on search systems handling up to 12,000 requests per second, with reliability work across data pipelines and APIs. Halved Search 2.0 P50 latency from 700 ms to 350 ms. Experience includes learning-to-rank pipelines, LLM evaluation, observability for agentic workflows, and Python and Rust services.";

pub const TELEMETRY_TABS: &[TelemetryTab] = &[
    TelemetryTab {
        id: "search",
        label: "search-2.0",
        arch_prefix: "Flask → FastAPI → ",
        arch_highlight: "Rust + Axum",
        arch_mid: " · ",
        arch_signal: "Hexagonal Arch",
        metrics: &[
            TelemetryMetric {
                label: "Search P50 Latency",
                value: "350 ms",
                detail: "down from 700 ms (−50%)",
                tone: MetricTone::Signal,
            },
            TelemetryMetric {
                label: "Memory Footprint",
                value: "190× less",
                detail: "28× lower compute cost",
                tone: MetricTone::Accent,
            },
            TelemetryMetric {
                label: "Non-Peak GKE Spend",
                value: "−83%",
                detail: "78% lower min node-pool",
                tone: MetricTone::Signal,
            },
            TelemetryMetric {
                label: "Projected Savings",
                value: "~$100K/yr",
                detail: "annualised at scale",
                tone: MetricTone::Default,
            },
        ],
        note: "Re-architected Best Buy's semantic search serving layer to decouple core ranking logic from infrastructure and right-size C4A GKE node pools.",
    },
    TelemetryTab {
        id: "ltr",
        label: "ltr-onnx",
        arch_prefix: "Python LTR → ",
        arch_highlight: "ONNX Runtime",
        arch_mid: " · GPU ESCI → ",
        arch_signal: "Async Write-Back",
        metrics: &[
            TelemetryMetric {
                label: "LTR Inference P50",
                value: "2 ms",
                detail: "4 ms P95 (3× faster)",
                tone: MetricTone::Signal,
            },
            TelemetryMetric {
                label: "ESCI Spike Fix",
                value: "<200 ms",
                detail: "down from 5,000 ms",
                tone: MetricTone::Accent,
            },
            TelemetryMetric {
                label: "LTR Retraining",
                value: "<15 min",
                detail: "down from 24 hours",
                tone: MetricTone::Signal,
            },
            TelemetryMetric {
                label: "Cloud ML Cost",
                value: "−20%+",
                detail: "across 5 business units",
                tone: MetricTone::Default,
            },
        ],
        note: "End-to-end ranking speedups across training and serving — optimizing NDCG, RPV, and CTR while stabilizing GPU inference under peak load.",
    },
    TelemetryTab {
        id: "askblue",
        label: "askblue-eval",
        arch_prefix: "",
        arch_highlight: "LLM-as-a-Judge",
        arch_mid: " · Cloud Trace · ",
        arch_signal: "Streamlit → Dioxus",
        metrics: &[
            TelemetryMetric {
                label: "Product Validation",
                value: "<1 hr",
                detail: "down from 1–2 days",
                tone: MetricTone::Signal,
            },
            TelemetryMetric {
                label: "UI Compute Cost",
                value: "27× less",
                detail: "Streamlit → Rust Dioxus",
                tone: MetricTone::Accent,
            },
            TelemetryMetric {
                label: "Image Build Time",
                value: "~1 min",
                detail: "5× faster (slim-trixie)",
                tone: MetricTone::Signal,
            },
            TelemetryMetric {
                label: "Model Tracing",
                value: "Gemma vs Gemini",
                detail: "agentic I/O diagnostics",
                tone: MetricTone::Default,
            },
        ],
        note: "Established org-wide Google Cloud Trace diagnostics, automated regression evals, and self-service load testing for AskBlue's agentic workflows.",
    },
];

pub const IMPACT_CARDS: &[ImpactCard] = &[
    ImpactCard {
        tag: "SEMANTIC SEARCH RE-ARCHITECTURE",
        org: "Best Buy",
        headline: "28× compute · 190× memory",
        title: "Flask → FastAPI → Rust (Axum)",
        desc: "Halved Search 2.0 P50 latency, cut max observed latency by over 1 second, and slashed non-peak GKE CPU/RAM spend by 83% (~US$100K projected annualised savings).",
        wide: true,
        meter: Some(ImpactMeter {
            before_label: "700 ms P50",
            before_pct: 100,
            after_label: "350 ms P50",
            after_pct: 50,
            accent_after: false,
        }),
    },
    ImpactCard {
        tag: "GPU MODEL STABILITY",
        org: "Best Buy",
        headline: "25× lower tail latency",
        title: "Eliminated GPU-hosted ESCI latency spikes",
        desc: "Introduced asynchronous write-back caching and reduced logging overhead on the GPU-hosted ESCI model that was slowing Search.",
        wide: true,
        meter: Some(ImpactMeter {
            before_label: "5,000 ms spike",
            before_pct: 100,
            after_label: "<200 ms",
            after_pct: 6,
            accent_after: false,
        }),
    },
    ImpactCard {
        tag: "RANKING PIPELINE",
        org: "Falabella",
        headline: "96× faster",
        title: "LTR model retraining",
        desc: "Compressed retraining across 5 business units while cutting cloud costs >20% and optimizing NDCG, RPV, and CTR.",
        wide: false,
        meter: Some(ImpactMeter {
            before_label: "24 hrs",
            before_pct: 100,
            after_label: "<15 min",
            after_pct: 5,
            accent_after: true,
        }),
    },
    ImpactCard {
        tag: "EVALUATION AUTOMATION",
        org: "Best Buy",
        headline: "1–2 days → <1 hr",
        title: "LLM-as-a-judge regression suite",
        desc: "Combined LLM-as-a-judge evals, an API comparator, and endpoint smoke tests to compress release validation from days to under an hour.",
        wide: false,
        meter: Some(ImpactMeter {
            before_label: "24–48 hrs",
            before_pct: 100,
            after_label: "<1 hr",
            after_pct: 8,
            accent_after: false,
        }),
    },
    ImpactCard {
        tag: "CI/CD & COMPUTE UPGRADE",
        org: "Best Buy",
        headline: "5× faster builds",
        title: "Split base/app slim-trixie images",
        desc: "Replaced Bullseye with slim-trixie and split reusable base vs. app image stages; also drove a 3-gen C4A upgrade (−30% latency, −20% pods).",
        wide: false,
        meter: Some(ImpactMeter {
            before_label: "5–6 min",
            before_pct: 100,
            after_label: "~1 min",
            after_pct: 20,
            accent_after: true,
        }),
    },
    ImpactCard {
        tag: "INFERENCE SPEED",
        org: "Best Buy",
        headline: "2 ms P50",
        title: "Python LTR → ONNX Runtime",
        desc: "Achieved 3× lower inference latency with 4 ms P95 for production ranking models.",
        wide: false,
        meter: None,
    },
    ImpactCard {
        tag: "RUST FRONTEND MIGRATION",
        org: "Best Buy",
        headline: "27× less compute",
        title: "Streamlit → Dioxus",
        desc: "Replaced Python UI workloads with Rust-based Dioxus (the same framework powering this site) while maintaining or improving UX speed.",
        wide: false,
        meter: None,
    },
    ImpactCard {
        tag: "COMMERCIAL & RISK VALUE",
        org: "Multi-Org",
        headline: "$50K · $1.2M · $2M",
        title: "Direct business impact",
        desc: "$50K/yr saved via in-house A/B testing (Falabella), $2M GCP deals (Accenture), $1.2M/yr Pfizer pricing revenue, and avoided Franklin MF's $3.4B collapse.",
        wide: false,
        meter: None,
    },
];

pub const CASE_STUDIES: &[CaseStudy] = &[
    CaseStudy {
        kicker: "Search Infrastructure",
        place: "Best Buy · Search 2.0",
        title: "Hexagonal Rust/Axum Semantic Search Platform",
        body: "Led the step-by-step migration from Flask to FastAPI and finally to Rust with Axum. Introducing hexagonal architecture isolated core search & ranking logic from infrastructure adapters, enabling C4A-highcpu-4 GKE right-sizing and RPS-aware HPA autoscaling for holiday peaks.",
        flow: "HTTP / RPS HPA → Axum Router → Core Search Domain → [NER · Sparse · ESCI · ONNX LTR]",
        chips: &[
            ("Rust", "core"),
            ("Axum", "core"),
            ("Hexagonal Arch", ""),
            ("GKE / C4A", ""),
            ("−50% P50 · 190× less RAM", "signal"),
        ],
    },
    CaseStudy {
        kicker: "Agentic AI & Observability",
        place: "Best Buy · AskBlue",
        title: "End-to-End Tracing & LLM-as-a-Judge Evaluation",
        body: "Established Google Cloud Trace and structured Cloud Logging across the org, extending request-level diagnostics into AskBlue's agentic inputs and outputs. Built monitoring dashboards comparing fine-tuned Gemma vs. Gemini models alongside automated regression evals and self-service load testing.",
        flow: "Agentic I/O → Cloud Trace + Structured Logs → Gemma/Gemini Dashboards + LLM Judge",
        chips: &[
            ("Gemini / Gemma", "core"),
            ("Cloud Trace", "core"),
            ("Google ADK", ""),
            ("LLM-as-a-Judge", ""),
            ("1–2d → <1h validation", "signal"),
        ],
    },
    CaseStudy {
        kicker: "Ranking & Experimentation",
        place: "Falabella",
        title: "High-Velocity LTR Retraining & In-House A/B Engine",
        body: "Overhauled multi-BU Learning-to-Rank pipelines from 24-hour runs to under 15 minutes using Polars and Kubernetes CronJobs, migrated Flask APIs to async FastAPI (−40% daily timeouts), shipped related-keywords search (+7% engagement), and built a cookie-based A/B platform saving $50K/yr.",
        flow: "K8s CronJobs + Polars → XGBoost LTR (5 BUs) → FastAPI Async + Loki / Datadog",
        chips: &[
            ("Learning to Rank", "core"),
            ("Polars", "core"),
            ("FastAPI", ""),
            ("K8s CronJobs", ""),
            ("24h → <15m · $50K saved", "signal"),
        ],
    },
    CaseStudy {
        kicker: "Cloud MLOps & FinTech Risk",
        place: "Accenture AI · Kaleidofin",
        title: "Terraform GCP Foundations & Underbanked Portfolio Risk",
        body: "Built standardized dev/stage/prod Terraform MLOps workflows on GCP (−50% manual setup) and a Vertex AI 3G telecom decommissioning solution removing 150+ London-outskirts towers. Earlier, as employee #70 at Seed-stage Kaleidofin, built risk analysis that avoided Franklin MF ahead of its $3.4B collapse.",
        flow: "Terraform IaC → Vertex AI / BigQuery Pipelines · Portfolio Risk Signals",
        chips: &[
            ("GCP / Vertex AI", "core"),
            ("Terraform", "core"),
            ("BigQuery", ""),
            ("Risk Analytics", ""),
            ("150+ towers · $3.4B risk avoided", "signal"),
        ],
    },
];

pub const ROLES: &[Role] = &[
    Role {
        id: "bestbuy",
        company: "Best Buy",
        title: "Senior AI Engineer — AskBlue & Search 2.0",
        period: "May 2025 – Present",
        location: "Bengaluru, India",
        badge: Some("CURRENT"),
        current: true,
        default_open: true,
        bullets: &[
            "Migrated the Search 2.0 semantic search platform from Flask to FastAPI, then to Rust with Axum, introducing hexagonal architecture to separate core search logic from infrastructure. Cut compute costs 28-fold and memory usage 190-fold; halved P50 latency from 700 ms to 350 ms and reduced maximum observed latency by over 1 second. Moving from C4A-standard-16 to C4A-highcpu-4 on GKE lowered minimum node-pool compute costs by 78% and reduced non-peak GKE CPU/RAM spend by 83%, with nearly US$100,000 in projected annualised savings at scale.",
            "Migrated Python-based LTR inference to ONNX, reducing inference latency threefold, with P50 of 2 ms and P95 of 4 ms.",
            "Supported the setup of NER, sparse, and ESCI models in the search stack and managed ongoing model upgrades.",
            "Independently identified a three-generation C4A upgrade across AML, reducing search request latency by 30% and compute pod count by 20%. Independently optimised Python image builds by replacing Bullseye with slim-trixie and splitting reusable base/application image stages, cutting build time 5× (5–6 min to ~1 min).",
            "Optimised holiday-season infrastructure costs by tuning Kubernetes HPA, adding requests-per-second (RPS) scaling, and right-sizing CPU and GPU deployments.",
            "Built a self-service performance-testing setup for AskBlue, enabling engineers to identify bottlenecks and iterate before formal performance testing.",
            "Established Google Cloud Trace within the organisation, integrating Cloud Logging and structured logs for request-level diagnostics. Extended tracing to AskBlue agentic I/O and contributed monitoring dashboards comparing fine-tuned Gemma models with Gemini models.",
            "Resolved latency spikes in the GPU-hosted ESCI model from 5 seconds to under 200 ms through asynchronous write-back caching and reduced logging overhead.",
            "Set up LLM-as-a-judge regression evaluation, an API comparator, and endpoint smoke tests, cutting product validation from 1–2 days to under 1 hour.",
            "Migrated Streamlit to Dioxus, reducing compute costs by a factor of 27 while maintaining or improving performance.",
        ],
    },
    Role {
        id: "falabella",
        company: "Falabella",
        title: "Senior Machine Learning Engineer",
        period: "April 2023 – April 2025",
        location: "Bengaluru",
        badge: None,
        current: false,
        default_open: true,
        bullets: &[
            "Migrated backend services from Flask to FastAPI using asynchronous programming, reducing daily timeouts by 40%. Added continuous monitoring for API and model-inference pipelines with Slack, Loki, and Datadog.",
            "Revamped the LTR model retraining process, reducing model creation time from 24 hours to under 15 minutes and cloud costs by over 20% across five business units while optimizing for NDCG, RPV, and CTR.",
            "Built a related-keywords search feature that increased user engagement by 7% across two major business segments.",
            "Improved code maintainability while reducing CPU and RAM consumption; migrated Pandas jobs to Polars to minimize job failures and integrated MLOps with Kubernetes CronJobs.",
            "Engineered a cookie-based A/B testing solution that eliminated reliance on third-party tools and saved $50k annually in software costs.",
            "Played a key role in the cyber team's performance testing, strengthening system resilience ahead of biannual cyber events.",
        ],
    },
    Role {
        id: "accenture",
        company: "Accenture AI",
        title: "Data Science Consultant",
        period: "December 2020 – April 2023",
        location: "Bengaluru",
        badge: None,
        current: false,
        default_open: false,
        bullets: &[
            "Collaborated with clients in the CPG and banking sectors to craft tailored proposals using Google Cloud Platform solutions, contributing to securing contracts worth $2 million.",
            "Crafted a data-driven solution to streamline the decommissioning lifecycle of 3G telecommunications infrastructure, leading to the successful removal of over 150 towers in the London outskirts using Vertex AI.",
            "Constructed robust Terraform automation workflows for GCP resource management in dev, stage, and prod ecosystems, achieving a 50% reduction in manual setup efforts while enhancing consistency across ML projects.",
        ],
    },
    Role {
        id: "kaleidofin",
        company: "Kaleidofin",
        title: "Data Science Analyst",
        period: "June 2019 – December 2020",
        location: "Chennai",
        badge: Some("Employee #70 · Seed"),
        current: false,
        default_open: false,
        bullets: &[
            "Designed a data analysis framework for consumer and corporate portfolios, reducing analysis time by 40%, enabling faster investment decisions and improving responsiveness to market changes.",
            "Identified key investment pitfalls through data-driven analysis; assessed mutual fund risks and avoided selection of Franklin MF ahead of its $3.4 billion collapse, safeguarding underbanked customers.",
        ],
    },
    Role {
        id: "fundsindia",
        company: "FundsIndia",
        title: "Data Analyst",
        period: "May 2018 – June 2019",
        location: "Bengaluru",
        badge: None,
        current: false,
        default_open: false,
        bullets: &[
            "Analyzed call center data to improve operations, reducing incoming call drop-offs by 10% and increasing customer retention and satisfaction by 15%.",
            "Designed an interactive Product Analytics Dashboard to analyze conversion funnels for mutual fund products, identifying key bottlenecks and driving a 15% increase in conversion rates.",
        ],
    },
    Role {
        id: "musigma",
        company: "MuSigma",
        title: "Trainee Decision Scientist",
        period: "July 2017 – April 2018",
        location: "Bengaluru",
        badge: None,
        current: false,
        default_open: false,
        bullets: &[
            "Led the creation of interactive dashboards for Pfizer's patented drug division, enabling the sales team to analyze competitive pricing data and optimize insurance negotiations for an estimated $1.2 million in additional annual revenue.",
            "Developed a comprehensive data pipeline transforming raw financial data into polished daily presentations for the finance team, eliminating repetitive manual tasks.",
        ],
    },
];

pub const SKILL_GROUPS: &[SkillGroup] = &[
    SkillGroup {
        id: "search-ml",
        filter_label: "Search & ML",
        title: "Search & Machine Learning",
        items: &[
            ("Learning to Rank (LTR)", true),
            ("ONNX", true),
            ("ESCI", true),
            ("NER", false),
            ("XGBoost", false),
            ("Model Fine-Tuning", false),
            ("Ranking", false),
            ("Vertex AI", false),
            ("AWS AutoML", false),
        ],
    },
    SkillGroup {
        id: "ai-eval",
        filter_label: "Applied AI & Eval",
        title: "Applied AI & Evaluation",
        items: &[
            ("Gemini", true),
            ("LLM-as-a-Judge", true),
            ("Google ADK", true),
            ("Fine-tuned Gemma", false),
            ("A/B Testing", false),
            ("Performance Testing", false),
        ],
    },
    SkillGroup {
        id: "systems",
        filter_label: "Languages & Serving",
        title: "Languages & Serving",
        items: &[
            ("Rust", true),
            ("Python", true),
            ("Axum", true),
            ("FastAPI", true),
            ("Dioxus", true),
            ("Flask", false),
            ("Streamlit", false),
            ("Bash", false),
            ("LaTeX", false),
        ],
    },
    SkillGroup {
        id: "cloud",
        filter_label: "Cloud & MLOps",
        title: "Cloud & MLOps",
        items: &[
            ("Google Cloud (GCP)", true),
            ("Kubernetes (GKE)", true),
            ("Terraform", true),
            ("Helm", false),
            ("GitLab", false),
            ("K9s", false),
            ("Makefile", false),
            ("Pre-Commit", false),
        ],
    },
    SkillGroup {
        id: "observability",
        filter_label: "Observability",
        title: "Observability",
        items: &[
            ("Google Cloud Trace", true),
            ("Cloud Logging", true),
            ("Datadog", false),
            ("Loki", false),
        ],
    },
    SkillGroup {
        id: "data",
        filter_label: "Data & Analytics",
        title: "Data & Analytics",
        items: &[
            ("Polars", true),
            ("BigQuery", true),
            ("Pandas", false),
            ("NumPy", false),
            ("MongoDB", false),
            ("Data Studio", false),
        ],
    },
];

pub const EDUCATION: &[Education] = &[
    Education {
        kind: "Graduate Degree",
        school: "Georgia Institute of Technology",
        degree: "MS, Analytics — Computational Track",
        location: "Atlanta, US",
    },
    Education {
        kind: "Undergraduate Degree",
        school: "BMS Institute of Technology",
        degree: "B.E., Electronics & Communication",
        location: "Bengaluru, India",
    },
];

pub const CERTIFICATIONS: &[Certification] = &[
    Certification {
        issuer: "Google Cloud · Verified",
        title: "GCP Professional Data Engineer",
        subtitle: "Credential.net verification",
        url: "https://www.credential.net/5a158b5b-5b95-4e79-868a-2cf6a4959512?key=c4a40ba808cb312664a87fb1f5ae292c984bcd3eafd3db40d07c5427e4b3752f",
    },
    Certification {
        issuer: "DeepLearning.AI · Coursera",
        title: "Deep Learning Specialization",
        subtitle: "Verified accomplishment",
        url: "https://www.coursera.org/account/accomplishments/specialization/GLDSULWYE2H4",
    },
    Certification {
        issuer: "Stanford Online · Coursera",
        title: "Machine Learning",
        subtitle: "Verified accomplishment",
        url: "https://www.coursera.org/account/accomplishments/verify/4P6GMFBUTRSD",
    },
];
