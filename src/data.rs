#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MetricTone {
    Default,
    Accent,
    Signal,
}

pub struct TelemetryWorkstream {
    pub index: &'static str,
    pub title: &'static str,
    pub pipeline: &'static str,
    pub advantage: &'static str,
    pub gains: &'static [(&'static str, MetricTone)],
}

pub struct TelemetryProject {
    pub id: &'static str,
    pub label: &'static str,
    pub context: &'static str,
    pub workstreams: &'static [TelemetryWorkstream],
    pub stack_summary: &'static str,
}

pub struct OutcomeMeter {
    pub label: &'static str,
    pub before_label: &'static str,
    pub before_pct: u8,
    pub after_label: &'static str,
    pub after_pct: u8,
    pub accent_after: bool,
}

pub struct DeepDiveSection {
    pub heading: &'static str,
    pub points: &'static [&'static str],
}

pub struct DeepDiveProject {
    pub id: &'static str,
    pub index: &'static str,
    pub company: &'static str,
    pub company_id: &'static str,
    pub period: &'static str,
    pub category: &'static str,
    pub title: &'static str,
    pub one_liner: &'static str,
    pub headline_badges: &'static [(&'static str, MetricTone)],
    pub flow: &'static str,
    pub meters: &'static [OutcomeMeter],
    pub key_outcomes: &'static [(&'static str, &'static str, MetricTone)], // (metric, label, tone)
    pub sections: &'static [DeepDiveSection],
    pub stack: &'static [(&'static str, bool)], // (name, is_core)
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

#[allow(dead_code)]
pub struct BlogPost {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub read_time: &'static str,
    pub tags: &'static [&'static str],
    pub summary: &'static str,
    pub paragraphs: &'static [&'static str],
}

pub const EMAIL: &str = "aaronjosephmathew@gmail.com";
pub const PHONE: &str = "+91 9844348064";
pub const LINKEDIN_URL: &str = "https://www.linkedin.com/in/aaron-joseph-mathew-4b425b71/";
pub const GITHUB_URL: &str = "https://github.com/aaronjoseph";
pub const X_URL: &str = "https://twitter.com/AaronJosephMath";

pub const SUMMARY: &str = "Senior AI Engineer with 9+ years across analytics, machine learning and AI engineering, focused on production search, ranking and reliable ML systems. Worked on search systems handling up to 12,000 requests per second, with reliability work across data pipelines and APIs. Halved Search 2.0 P50 latency from 700 ms to 350 ms. Experience includes learning-to-rank pipelines, LLM evaluation, observability for agentic workflows, and Python and Rust services.";

pub const TELEMETRY_PROJECTS: &[TelemetryProject] = &[
    TelemetryProject {
        id: "search-system",
        label: "Search System",
        context: "Best Buy · Semantic Search & Ranking Stack (up to 12,000 RPS)",
        stack_summary: "Rust · Axum · ONNX · GKE C4A",
        workstreams: &[
            TelemetryWorkstream {
                index: "01",
                title: "Flask → FastAPI → Rust (Axum)",
                pipeline: "Hexagonal Architecture · C4A GKE",
                advantage: "Separated core search logic from infra adapters and removed Python runtime overhead, enabling C4A-highcpu-4 right-sizing and clean holiday HPA scaling.",
                gains: &[
                    ("Up to 12,000 RPS", MetricTone::Signal),
                    ("700 → 350 ms P50", MetricTone::Signal),
                    ("28× less compute", MetricTone::Accent),
                    ("190× less RAM", MetricTone::Accent),
                    ("−83% non-peak GKE (~$100K/yr)", MetricTone::Default),
                ],
            },
            TelemetryWorkstream {
                index: "02",
                title: "LTR Inference → ONNX",
                pipeline: "Python Ranker → ONNX Runtime",
                advantage: "Eliminated Python inference bottlenecks in the ranking stage for predictable, single-digit-millisecond scoring under peak load.",
                gains: &[
                    ("3× faster inference", MetricTone::Signal),
                    ("2 ms P50 · 4 ms P95", MetricTone::Default),
                ],
            },
            TelemetryWorkstream {
                index: "03",
                title: "ESCI Async Write-Back Caching",
                pipeline: "GPU ESCI · Reduced Logging Overhead",
                advantage: "Eliminated GPU-hosted ESCI tail-latency spikes that were stalling Search requests through async write-back caching and leaner logging.",
                gains: &[
                    ("5,000 ms → <200 ms", MetricTone::Signal),
                    ("25× lower tail spikes", MetricTone::Accent),
                ],
            },
        ],
    },
    TelemetryProject {
        id: "agentic-ai",
        label: "Agentic AI",
        context: "Best Buy · Agentic AI Evaluation, Observability & Tooling",
        stack_summary: "Gemini · Gemma · Cloud Trace · Dioxus",
        workstreams: &[
            TelemetryWorkstream {
                index: "01",
                title: "LLM-as-a-Judge & API Comparator",
                pipeline: "Regression Evals · Endpoint Smoke Tests",
                advantage: "Replaced slow manual release checks with automated LLM-as-a-judge regression suites and self-service performance testing before formal perf runs.",
                gains: &[
                    ("1–2 days → <1 hr validation", MetricTone::Signal),
                    ("Self-service perf testing", MetricTone::Default),
                ],
            },
            TelemetryWorkstream {
                index: "02",
                title: "Agentic Tracing & Model Dashboards",
                pipeline: "Google Cloud Trace · Structured Logs",
                advantage: "Established org-wide request-level tracing across agentic AI inputs & outputs with monitoring dashboards comparing fine-tuned Gemma vs. Gemini.",
                gains: &[
                    ("End-to-end agentic I/O traces", MetricTone::Accent),
                    ("Gemma vs. Gemini dashboards", MetricTone::Default),
                ],
            },
            TelemetryWorkstream {
                index: "03",
                title: "Streamlit → Rust Dioxus & CI Builds",
                pipeline: "Dioxus WASM · slim-trixie Base/App Split",
                advantage: "Cut UI container compute dramatically by migrating from Streamlit to Dioxus, alongside 5× faster Docker image builds via split base/app stages.",
                gains: &[
                    ("27× lower UI compute", MetricTone::Signal),
                    ("5–6 min → ~1 min builds", MetricTone::Accent),
                ],
            },
        ],
    },
];

pub const DEEP_DIVE_COMPANIES: &[(&str, &str)] = &[
    ("all", "All Projects"),
    ("bestbuy", "Best Buy"),
    ("falabella", "Falabella"),
    ("accenture", "Accenture AI"),
];

pub const DEEP_DIVE_PROJECTS: &[DeepDiveProject] = &[
    DeepDiveProject {
        id: "bb-search-rust",
        index: "01",
        company: "Best Buy",
        company_id: "bestbuy",
        period: "May 2025 – Present",
        category: "Search System",
        title: "Flask → FastAPI → Rust (Axum) & GKE C4A",
        one_liner: "Re-architected semantic search into hexagonal Rust/Axum, right-sized C4A GKE node pools, and tuned holiday RPS autoscaling.",
        headline_badges: &[
            ("Up to 12,000 RPS", MetricTone::Signal),
            ("700 → 350 ms P50", MetricTone::Signal),
            ("28× compute · 190× RAM", MetricTone::Accent),
        ],
        flow: "Client Traffic (up to 12K RPS) → GKE HPA (RPS Scaling) → Axum HTTP Layer → Hexagonal Search Domain → Downstream Model Adapters",
        meters: &[
            OutcomeMeter {
                label: "Search P50 Latency",
                before_label: "700 ms",
                before_pct: 100,
                after_label: "350 ms (−50%)",
                after_pct: 50,
                accent_after: false,
            },
            OutcomeMeter {
                label: "Non-Peak GKE CPU/RAM Spend",
                before_label: "100% baseline",
                before_pct: 100,
                after_label: "17% (−83%)",
                after_pct: 17,
                accent_after: true,
            },
        ],
        key_outcomes: &[
            ("12,000 RPS", "Peak search throughput", MetricTone::Signal),
            ("28×", "Lower compute cost", MetricTone::Accent),
            ("190×", "Lower memory footprint", MetricTone::Accent),
            (">1 s", "Max latency cut", MetricTone::Signal),
            ("~$100K/yr", "Projected GKE savings", MetricTone::Signal),
            ("−78%", "Min node-pool compute", MetricTone::Default),
        ],
        sections: &[
            DeepDiveSection {
                heading: "Problem & Context",
                points: &[
                    "The original Flask semantic search layer carried heavy Python runtime overhead, high memory footprint, and 700 ms P50 latency under peak retail traffic.",
                    "Tight coupling between search orchestration and infrastructure adapters made iterative performance and cloud right-sizing harder than necessary.",
                ],
            },
            DeepDiveSection {
                heading: "Architecture & Key Decisions",
                points: &[
                    "Migrated incrementally from Flask → FastAPI → Rust with Axum, introducing hexagonal architecture to cleanly isolate core search logic from HTTP/cloud infrastructure.",
                    "Right-sized GKE node pools from C4A-standard-16 to C4A-highcpu-4 and independently identified a 3-generation C4A upgrade across AML (−30% search latency, −20% compute pods).",
                    "Tuned holiday-season Kubernetes HPA with requests-per-second (RPS) scaling and right-sized CPU/GPU deployments.",
                ],
            },
        ],
        stack: &[
            ("Rust", true),
            ("Axum", true),
            ("Hexagonal Architecture", true),
            ("GKE / C4A", true),
            ("Kubernetes HPA", false),
            ("FastAPI", false),
            ("Flask", false),
        ],
    },
    DeepDiveProject {
        id: "bb-search-onnx-esci",
        index: "02",
        company: "Best Buy",
        company_id: "bestbuy",
        period: "May 2025 – Present",
        category: "Search System",
        title: "ONNX LTR Inference & GPU ESCI Caching",
        one_liner: "Compiled Python LTR scoring to single-digit-ms ONNX Runtime and resolved 5-second GPU ESCI latency spikes via async write-back caching.",
        headline_badges: &[
            ("2 ms P50 / 4 ms P95 LTR", MetricTone::Signal),
            ("5 s → <200 ms ESCI", MetricTone::Accent),
        ],
        flow: "Search Query → [NER · Sparse Retrieval · GPU ESCI + Async Write-Back Cache] → ONNX Runtime LTR Ranker",
        meters: &[
            OutcomeMeter {
                label: "GPU ESCI Tail Latency",
                before_label: "5,000 ms spike",
                before_pct: 100,
                after_label: "<200 ms (25×)",
                after_pct: 6,
                accent_after: false,
            },
            OutcomeMeter {
                label: "LTR Inference Latency",
                before_label: "3× Python baseline",
                before_pct: 100,
                after_label: "2 ms P50 / 4 ms P95",
                after_pct: 33,
                accent_after: true,
            },
        ],
        key_outcomes: &[
            ("2 ms", "LTR ONNX P50", MetricTone::Signal),
            ("4 ms", "LTR ONNX P95", MetricTone::Signal),
            ("3×", "Faster LTR inference", MetricTone::Accent),
            ("<200 ms", "ESCI post-fix latency", MetricTone::Signal),
        ],
        sections: &[
            DeepDiveSection {
                heading: "Problem & Context",
                points: &[
                    "Python-based Learning-to-Rank inference added avoidable scoring latency on every search query.",
                    "Intermittent 5-second spikes on the GPU-hosted ESCI relevance model cascaded into overall Search slowdowns and stability issues.",
                ],
            },
            DeepDiveSection {
                heading: "Architecture & Key Decisions",
                points: &[
                    "Migrated Python LTR inference to ONNX Runtime, achieving 3× lower inference latency with 2 ms P50 and 4 ms P95.",
                    "Introduced asynchronous write-back caching and stripped unnecessary logging overhead on the GPU-hosted ESCI service, bringing latency down from 5 s to <200 ms.",
                    "Supported production setup and ongoing upgrades of NER, sparse, and ESCI models across the search stack.",
                ],
            },
        ],
        stack: &[
            ("ONNX Runtime", true),
            ("Learning to Rank", true),
            ("ESCI", true),
            ("NER / Sparse Models", false),
            ("Async Write-Back Cache", false),
            ("GPU Serving", false),
        ],
    },
    DeepDiveProject {
        id: "bb-agentic-ai",
        index: "03",
        company: "Best Buy",
        company_id: "bestbuy",
        period: "May 2025 – Present",
        category: "Agentic AI",
        title: "Observability, LLM-as-a-Judge & Rust Dioxus Tooling",
        one_liner: "Built Cloud Trace agentic diagnostics, automated LLM-as-a-judge evals, 5× faster container builds, and migrated Streamlit UIs to Dioxus.",
        headline_badges: &[
            ("1–2 days → <1 hr eval", MetricTone::Signal),
            ("27× UI compute · 5× builds", MetricTone::Accent),
        ],
        flow: "Agentic I/O → Cloud Trace + Structured Logs → Gemma vs. Gemini Dashboards · LLM-as-a-Judge + API Comparator · Dioxus UI",
        meters: &[
            OutcomeMeter {
                label: "Product Release Validation",
                before_label: "24–48 hrs manual",
                before_pct: 100,
                after_label: "<1 hr automated",
                after_pct: 6,
                accent_after: false,
            },
            OutcomeMeter {
                label: "Python Docker Image Builds",
                before_label: "5–6 min (Bullseye)",
                before_pct: 100,
                after_label: "~1 min (slim-trixie)",
                after_pct: 20,
                accent_after: true,
            },
        ],
        key_outcomes: &[
            ("<1 hr", "Regression validation", MetricTone::Signal),
            ("27×", "Lower UI compute (Dioxus)", MetricTone::Accent),
            ("5×", "Faster Docker builds", MetricTone::Accent),
            ("Gemma vs Gemini", "Live eval dashboards", MetricTone::Default),
        ],
        sections: &[
            DeepDiveSection {
                heading: "Observability & Evaluation",
                points: &[
                    "Established Google Cloud Trace across the organization, pairing Cloud Logging and structured logs for request-level diagnostics across agentic I/O.",
                    "Contributed monitoring dashboards comparing in-house fine-tuned Gemma models against Gemini models.",
                    "Built an LLM-as-a-judge regression suite, API comparator, endpoint smoke tests, and self-service performance testing — cutting product validation from 1–2 days to under 1 hour.",
                ],
            },
            DeepDiveSection {
                heading: "Rust UI & Build Velocity",
                points: &[
                    "Migrated internal Streamlit apps to Rust Dioxus, cutting compute costs 27× while maintaining or improving responsiveness.",
                    "Independently optimized Python Docker builds by replacing Bullseye with slim-trixie and splitting reusable base-image vs. code-change app-image stages (5–6 min → ~1 min).",
                ],
            },
        ],
        stack: &[
            ("Google Cloud Trace", true),
            ("LLM-as-a-Judge", true),
            ("Gemini / Gemma", true),
            ("Dioxus (Rust)", true),
            ("Google ADK", false),
            ("Cloud Logging", false),
            ("slim-trixie Docker", false),
        ],
    },
    DeepDiveProject {
        id: "fl-search-ltr",
        index: "04",
        company: "Falabella",
        company_id: "falabella",
        period: "Apr 2023 – Apr 2025",
        category: "Search & Ranking",
        title: "FastAPI Migration, 15-Min LTR Retraining & Polars",
        one_liner: "Cut LTR retraining from 24 hours to under 15 minutes across 5 business units, moved Flask to async FastAPI, and shipped related-keywords search.",
        headline_badges: &[
            ("24 hrs → <15 min LTR", MetricTone::Signal),
            ("−40% timeouts · +7% engagement", MetricTone::Accent),
        ],
        flow: "Kubernetes CronJobs + Polars → XGBoost LTR (5 BUs) → Async FastAPI Search APIs → Loki / Datadog / Slack Alerts",
        meters: &[
            OutcomeMeter {
                label: "LTR Model Retraining Time",
                before_label: "24 hours",
                before_pct: 100,
                after_label: "<15 min (96×)",
                after_pct: 5,
                accent_after: false,
            },
            OutcomeMeter {
                label: "Daily API Timeouts",
                before_label: "Flask sync baseline",
                before_pct: 100,
                after_label: "−40% (FastAPI async)",
                after_pct: 60,
                accent_after: true,
            },
        ],
        key_outcomes: &[
            ("<15 min", "LTR retraining (was 24 h)", MetricTone::Signal),
            (">20%", "Cloud cost reduction", MetricTone::Accent),
            ("−40%", "Daily API timeouts", MetricTone::Signal),
            ("+7%", "Search user engagement", MetricTone::Accent),
        ],
        sections: &[
            DeepDiveSection {
                heading: "Serving & Reliability",
                points: &[
                    "Migrated backend search services from synchronous Flask to async FastAPI, cutting daily timeouts by 40%.",
                    "Added continuous observability for API and model-inference pipelines with Slack, Loki, and Datadog for peak-load reliability.",
                ],
            },
            DeepDiveSection {
                heading: "Training Pipelines & Product Impact",
                points: &[
                    "Revamped LTR retraining from 24 hours to <15 minutes while cutting cloud costs by >20% across 5 business units, optimizing for NDCG, RPV, and CTR.",
                    "Migrated Pandas data pipelines to Polars and orchestrated ML jobs via Kubernetes CronJobs to reduce CPU/RAM consumption and job failures.",
                    "Built a related-keywords search feature driving +7% user engagement across two major business segments.",
                ],
            },
        ],
        stack: &[
            ("FastAPI", true),
            ("Learning to Rank", true),
            ("Polars", true),
            ("Kubernetes CronJobs", true),
            ("XGBoost", false),
            ("Datadog / Loki", false),
        ],
    },
    DeepDiveProject {
        id: "fl-ab-testing",
        index: "05",
        company: "Falabella",
        company_id: "falabella",
        period: "Apr 2023 – Apr 2025",
        category: "Experimentation",
        title: "In-House Cookie A/B Testing & Cyber Load Readiness",
        one_liner: "Replaced third-party experimentation vendors with an in-house cookie-based A/B engine and hardened search resilience for biannual cyber events.",
        headline_badges: &[
            ("$50K/yr saved", MetricTone::Signal),
            ("Zero 3rd-party dependency", MetricTone::Default),
        ],
        flow: "User Session Cookie Bucketing → Variant Routing → Search Telemetry & Experiment Metrics",
        meters: &[],
        key_outcomes: &[
            ("$50K/yr", "Software spend eliminated", MetricTone::Signal),
            ("Biannual", "Cyber-event load readiness", MetricTone::Accent),
        ],
        sections: &[
            DeepDiveSection {
                heading: "Problem & Context",
                points: &[
                    "Third-party experimentation tooling added recurring annual software spend ($50K/yr) and external integration overhead for search experiments.",
                    "Biannual high-traffic cyber events demanded predictable tail latency and resilience across the search and ranking stack.",
                ],
            },
            DeepDiveSection {
                heading: "Architecture & Key Decisions",
                points: &[
                    "Engineered a lightweight, cookie-based A/B testing solution in-house, eliminating third-party vendor reliance and saving $50K annually.",
                    "Partnered with the cyber team on formal performance testing ahead of biannual cyber sales events to validate system headroom.",
                ],
            },
        ],
        stack: &[
            ("A/B Testing", true),
            ("Cookie Bucketing", true),
            ("Performance Testing", true),
            ("FastAPI", false),
            ("Datadog", false),
        ],
    },
    DeepDiveProject {
        id: "ac-telecom-3g",
        index: "06",
        company: "Accenture AI",
        company_id: "accenture",
        period: "Dec 2020 – Apr 2023",
        category: "Applied ML",
        title: "3G Telecom Tower Decommissioning (Vertex AI)",
        one_liner: "Built a Vertex AI data-driven workflow streamlining 3G infrastructure sunsetting across the London outskirts.",
        headline_badges: &[
            ("150+ towers removed", MetricTone::Signal),
            ("Vertex AI · GCP", MetricTone::Default),
        ],
        flow: "Telecom Network & Traffic Data → BigQuery Feature Layer → Vertex AI Decommissioning Scoring → Rollout Schedule",
        meters: &[],
        key_outcomes: &[
            ("150+", "3G towers decommissioned", MetricTone::Signal),
            ("London", "Outskirts rollout scope", MetricTone::Default),
        ],
        sections: &[
            DeepDiveSection {
                heading: "Overview & Impact",
                points: &[
                    "Crafted a data-driven decision workflow on GCP Vertex AI to prioritize and streamline the 3G telecommunications decommissioning lifecycle.",
                    "Directly enabled the successful removal of over 150 legacy 3G towers in the London outskirts.",
                ],
            },
        ],
        stack: &[
            ("Vertex AI", true),
            ("Google Cloud (GCP)", true),
            ("BigQuery", true),
            ("Python", false),
        ],
    },
    DeepDiveProject {
        id: "ac-gcp-terraform",
        index: "07",
        company: "Accenture AI",
        company_id: "accenture",
        period: "Dec 2020 – Apr 2023",
        category: "Cloud & MLOps",
        title: "GCP Terraform MLOps Automation",
        one_liner: "Standardized dev/stage/prod GCP ML infrastructure with reusable Terraform workflows and helped win $2M in CPG & banking proposals.",
        headline_badges: &[
            ("−50% manual setup", MetricTone::Signal),
            ("$2M GCP contracts", MetricTone::Accent),
        ],
        flow: "Terraform Modules (IaC) → Automated GCP Provisioning [dev · stage · prod] → Reproducible ML Workloads",
        meters: &[OutcomeMeter {
            label: "Environment Setup Effort",
            before_label: "Manual provisioning",
            before_pct: 100,
            after_label: "−50% (Terraform IaC)",
            after_pct: 50,
            accent_after: true,
        }],
        key_outcomes: &[
            ("−50%", "Manual GCP setup effort", MetricTone::Signal),
            ("$2M", "Client contracts supported", MetricTone::Accent),
            ("3 envs", "dev / stage / prod parity", MetricTone::Default),
        ],
        sections: &[
            DeepDiveSection {
                heading: "Infrastructure Automation & Proposals",
                points: &[
                    "Constructed robust Terraform automation workflows for GCP resource management across dev, stage, and prod ecosystems, cutting manual setup effort by 50% and improving cross-project consistency.",
                    "Collaborated with CPG and banking clients to design tailored GCP ML architecture proposals, contributing to securing $2 million in contracts.",
                ],
            },
        ],
        stack: &[
            ("Terraform", true),
            ("Google Cloud (GCP)", true),
            ("MLOps IaC", true),
            ("Vertex AI", false),
            ("BigQuery", false),
        ],
    },
];

/// Add entries here to publish blog posts on the website.
/// When empty, the Blog section displays an intentional "coming soon" state with planned topics.
pub const BLOG_POSTS: &[BlogPost] = &[];

pub const PLANNED_BLOG_TOPICS: &[(&str, &str)] = &[
    (
        "Systems",
        "Migrating Semantic Search from Flask to FastAPI to Rust (Axum) with Hexagonal Architecture",
    ),
    (
        "Ranking",
        "Single-Digit-Millisecond Learning to Rank: Moving Python Inference to ONNX",
    ),
    (
        "Reliability",
        "Taming 5-Second GPU ESCI Spikes with Async Write-Back Caching",
    ),
    (
        "Applied AI",
        "Observability & LLM-as-a-Judge Regression Suites for Agentic Workflows",
    ),
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
