#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MetricTone {
    Default,
    Accent,
    Signal,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NotebookComparison {
    pub label: &'static str,
    pub before: &'static str,
    pub after: &'static str,
    pub delta: &'static str,
    pub tone: MetricTone,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CalloutStyle {
    PurpleDashed,
    GreenCheck,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NotebookCallout {
    pub style: CalloutStyle,
    pub title: &'static str,
    pub body: &'static str,
    pub link: Option<(&'static str, &'static str)>, // (label, url)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StickyTone {
    Yellow,
    Pink,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StickyNote {
    pub tone: StickyTone,
    pub title: &'static str,
    pub items: &'static [&'static str],
}

pub struct DeepDiveSection {
    pub heading: &'static str,
    pub points: &'static [&'static str],
}

pub struct DeepDiveProject {
    pub id: &'static str,
    pub company: &'static str,
    pub company_id: &'static str,
    pub period: &'static str,
    pub category: &'static str,
    pub title: &'static str,
    pub one_liner: &'static str,
    pub headline_badges: &'static [(&'static str, MetricTone)],
    pub flow: &'static str,
    pub comparisons: &'static [NotebookComparison],
    pub callouts: &'static [NotebookCallout],
    pub sticky_notes: &'static [StickyNote],
    pub key_outcomes: &'static [(&'static str, &'static str, MetricTone)],
    pub sections: &'static [DeepDiveSection],
    pub stack: &'static [(&'static str, bool)],
}

pub struct Role {
    pub id: &'static str,
    pub company: &'static str,
    pub title: &'static str,
    pub period: &'static str,
    pub location: &'static str,
    pub badge: Option<&'static str>,
    pub current: bool,
    pub story_kicker: &'static str,
    pub story_headline: &'static str,
    pub story_body: &'static str,
    pub story_takeaway: &'static str,
    pub bullets: &'static [&'static str],
}

pub struct SkillGroup {
    pub id: &'static str,
    pub filter_label: &'static str,
    pub title: &'static str,
    pub items: &'static [(&'static str, bool)],
}

pub struct Education {
    pub kind: &'static str,
    pub school: &'static str,
    pub degree: &'static str,
    pub location: &'static str,
    pub url: Option<(&'static str, &'static str)>,
}

pub struct Certification {
    pub issuer: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub url: &'static str,
}

#[allow(dead_code)]
pub enum ArticleBlock {
    Heading(&'static str),
    Paragraph(&'static str),
    ArchitectureFlow(&'static str),
    HexagonalSketch,
    AsyncCacheSketch,
    OnnxPoolSketch,
    Bullets(&'static [(&'static str, &'static str)]),
    FieldNotes {
        stamp: &'static str,
        comparisons: &'static [NotebookComparison],
        callouts: &'static [NotebookCallout],
        sticky_notes: &'static [StickyNote],
    },
}

pub struct BlogPost {
    pub slug: &'static str,
    pub date: &'static str,
    pub year: &'static str,
    pub topic: &'static str,
    pub read_time: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub tags: &'static [&'static str],
    pub blocks: &'static [ArticleBlock],
}

pub struct PlannedTopic {
    pub date_label: &'static str,
    pub year: &'static str,
    pub topic: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
}

pub const EMAIL: &str = "aaronjosephmathew@gmail.com";
pub const PHONE: &str = "+91 9844348064";
pub const LINKEDIN_URL: &str = "https://www.linkedin.com/in/aaron-joseph-mathew-4b425b71/";
pub const GITHUB_URL: &str = "https://github.com/aaronjoseph";
pub const X_URL: &str = "https://twitter.com/AaronJosephMath";

pub const SUMMARY: &str = "Senior AI Engineer with 9+ years across analytics, machine learning and AI engineering, focused on production search, ranking and reliable ML systems. Worked on search systems handling up to 12,000 requests per second, with reliability work across data pipelines and APIs. Cut Search 2.0 median (P50) latency by 50% and compute footprint by 95% via Rust migration. Experience includes learning-to-rank pipelines, LLM evaluation, observability for agentic workflows, and Python and Rust services.";

pub const DEEP_DIVE_PROJECTS: &[DeepDiveProject] = &[
    DeepDiveProject {
        id: "bb-search-rust",
        company: "Best Buy",
        company_id: "bestbuy",
        period: "May 2025 – Present",
        category: "Search System",
        title: "Flask → FastAPI → Rust (Axum) & GKE ARM C4A",
        one_liner: "Migrated GKE from x86 n1-standard-16 to ARM C4A, consolidated search microservices into a hexagonal Rust monolith with per-service bulkheads, and right-sized idle node pools.",
        headline_badges: &[
            ("−14% agent cost / turn", MetricTone::Signal),
            ("−30% ARM C4A latency", MetricTone::Signal),
            ("−95% CPU · −98% RAM", MetricTone::Accent),
            ("−50% P50 · −57% idle cost", MetricTone::Default),
        ],
        flow: "Client Traffic → GKE ARM C4A HPA → Rust/Axum Monolith (api → domain ← infra) → Bulkhead Pools [Product Search | Recall Fan-Out | Metadata | LTR | ESCI]",
        comparisons: &[
            NotebookComparison {
                label: "Agentic Cost / Turn",
                before: "Python search baseline",
                after: "Rust/Axum search path",
                delta: "−14% cost / turn",
                tone: MetricTone::Signal,
            },
            NotebookComparison {
                label: "GKE x86 → ARM C4A (Titanium)",
                before: "x86 n1-standard-16",
                after: "ARM c4a-standard-16",
                delta: "−30% latency",
                tone: MetricTone::Signal,
            },
            NotebookComparison {
                label: "Search P50 Latency",
                before: "Python baseline",
                after: "Rust/Axum monolith",
                delta: "−50% P50",
                tone: MetricTone::Signal,
            },
            NotebookComparison {
                label: "FastAPI → Rust Footprint",
                before: "Python workers",
                after: "Rust single binary",
                delta: "−95% CPU · −98% RAM",
                tone: MetricTone::Accent,
            },
            NotebookComparison {
                label: "Idle / Low-Traffic Node Pool",
                before: "c4a-standard-16",
                after: "c4a-highcpu-8",
                delta: "−57% idle cost/node",
                tone: MetricTone::Accent,
            },
        ],
        callouts: &[
            NotebookCallout {
                style: CalloutStyle::GreenCheck,
                title: "Platform Win · x86 n1-standard-16 → ARM c4a-standard-16",
                body: "Partnered with the Platform team to evaluate and adopt Google Cloud's Arm/Titanium-backed C4A instances across AML, delivering an immediate 30% search latency drop and 20% fewer compute pods even before the Rust rewrite.",
                link: Some(("Deep dive: Google Cloud Titanium ↗", "https://cloud.google.com/titanium")),
            },
            NotebookCallout {
                style: CalloutStyle::PurpleDashed,
                title: "Why Monolith + Bulkhead Connectors + Fan-Out?",
                body: "Instead of paying network hops between Python microservices, search, recall, and reranking now run in-process inside one hexagonal Rust binary. Each downstream dependency gets its own isolated bulkhead connection pool (e.g., separate pools for recall fan-out, large metadata HMGET pipelines, small LTR reads, and ESCI cache) so high-concurrency recall fan-out or heavy metadata payloads never head-of-line-block latency-sensitive scoring.",
                link: None,
            },
        ],
        sticky_notes: &[
            StickyNote {
                tone: StickyTone::Yellow,
                title: "FastAPI → Rust Migration",
                items: &[
                    "−95% CPU & −98% RAM usage",
                    "~$180K/yr saved (~$500/day)",
                    "~14% lower cost per agent turn",
                    "−50% Search P50 latency",
                ],
            },
            StickyNote {
                tone: StickyTone::Pink,
                title: "C4A Node Right-Sizing",
                items: &[
                    "c4a-standard-16 → c4a-highcpu-8",
                    "−57% idle cost per node at low load",
                    "RPS-aware HPA tuned for peak events",
                ],
            },
        ],
        key_outcomes: &[
            ("−14%", "Agent cost per turn", MetricTone::Signal),
            ("−30%", "Immediate ARM C4A latency drop", MetricTone::Signal),
            ("−95% / −98%", "CPU & RAM cut (Rust)", MetricTone::Accent),
            ("−50%", "Search P50 latency cut", MetricTone::Signal),
            ("−57%", "Idle node cost (highcpu-8)", MetricTone::Accent),
        ],
        sections: &[
            DeepDiveSection {
                heading: "1 · GKE Architecture: x86 → ARM C4A (Titanium)",
                points: &[
                    "Independently identified the 3-generation jump from x86 n1-standard-16 to ARM c4a-standard-16 on Google Cloud Titanium and worked with the Platform team to drive adoption across AML.",
                    "Delivered an immediate 30% latency reduction and 20% pod-count drop from the hardware/architecture upgrade alone.",
                ],
            },
            DeepDiveSection {
                heading: "2 · Microservices → Hexagonal Rust Monolith + Bulkheads",
                points: &[
                    "Migrated Flask → FastAPI → Rust (Axum) with strict compile-time hexagonal layering (api → domain ← infra), cutting CPU by 95%, RAM by 98%, halving Search P50 latency (−50%), and driving ~$180K/yr (~$500/day) in savings — reducing end-to-end cost per turn for the agentic shopping assistant by ~14%.",
                    "Collapsed separate Python search/recall/reranker microservices into a single in-process Rust binary to remove internal HTTP serialization overhead while preserving distinct OpenTelemetry spans per stage.",
                    "Designed per-service bulkhead connection pools (isolating product-search, ESCI cache, reranker metadata, and LTR Redis managers) so large metadata batches never head-of-line-block small LTR lookups, paired with order-preserving concurrent fan-out for multi-tier recall.",
                ],
            },
            DeepDiveSection {
                heading: "3 · Idle Cost Right-Sizing (c4a-standard-16 → c4a-highcpu-8)",
                points: &[
                    "Because the Rust monolith needed a fraction of Python's RAM, downsized the node pool from c4a-standard-16 to c4a-highcpu-8 — cutting idle/low-traffic per-node compute costs by 57%.",
                    "Paired with RPS-based Kubernetes HPA scaling to stay lean overnight while expanding smoothly for holiday peaks.",
                ],
            },
            DeepDiveSection {
                heading: "4 · Python Polars DataFrames → Native Rust Structs & LLVM Optimization",
                points: &[
                    "Replaced Python Polars DataFrames in the hot request path with native typed Rust structs, borrowed slices, small domain collections, and zero-cost iterators. While Polars excels at large offline batch analytics, per-request search scoring operates on small candidate cohorts where DataFrame schema setup, Arrow buffer allocations, and Python↔FFI crossings add avoidable overhead.",
                    "Modelling candidates, ranking features, scores, and grouped results directly in contiguous Rust memory eliminated intermediate materializations and heap churn, making feature assembly and merging far more CPU-cache friendly.",
                    "Compiled the production serving binary with aggressive LLVM release flags (opt-level = 3 across dependencies, Fat LTO, and codegen-units = 1), enabling cross-crate inlining across hexagonal layers, dead-code elimination, and tighter ARM Neoverse (C4A) machine code generation.",
                ],
            },
        ],
        stack: &[
            ("Rust", true),
            ("Axum", true),
            ("Hexagonal Monolith", true),
            ("Bulkhead Pools", true),
            ("GKE ARM C4A / Titanium", true),
            ("LLVM Fat LTO", false),
            ("Concurrent Fan-Out", false),
            ("Kubernetes HPA", false),
            ("FastAPI / Flask", false),
        ],
    },
    DeepDiveProject {
        id: "bb-search-onnx-esci",
        company: "Best Buy",
        company_id: "bestbuy",
        period: "May 2025 – Present",
        category: "Search System",
        title: "ONNX LTR Inference & GPU ESCI Caching",
        one_liner: "Compiled Python LTR scoring to single-digit-ms ONNX Runtime (3× faster) and resolved severe GPU ESCI tail-latency spikes (>95% drop) via async write-back caching.",
        headline_badges: &[
            ("3× faster LTR (single-digit ms)", MetricTone::Signal),
            ("25× lower ESCI tail spikes", MetricTone::Accent),
        ],
        flow: "Search Query → [NER · Sparse Retrieval · GPU ESCI + Async Write-Back Cache] → ONNX Runtime LTR Ranker",
        comparisons: &[
            NotebookComparison {
                label: "GPU ESCI Tail Latency",
                before: "Multi-second spikes",
                after: "Sub-second steady",
                delta: "25× faster tail (−96%)",
                tone: MetricTone::Signal,
            },
            NotebookComparison {
                label: "LTR Inference Latency",
                before: "Python scorer",
                after: "ONNX Runtime (single-digit ms)",
                delta: "3× faster (−67%)",
                tone: MetricTone::Accent,
            },
        ],
        callouts: &[NotebookCallout {
            style: CalloutStyle::PurpleDashed,
            title: "Keeping Async Workers Free",
            body: "ONNX sessions run inside a bounded per-branch session pool offloaded via spawn_blocking so CPU-bound scoring never starves async Redis/network reader tasks under load, while ESCI writes happen asynchronously in the background.",
            link: None,
        }],
        sticky_notes: &[],
        key_outcomes: &[
            ("3×", "Faster LTR inference", MetricTone::Signal),
            ("−96%", "ESCI tail-latency reduction", MetricTone::Accent),
            ("Single-digit", "Millisecond LTR scoring", MetricTone::Signal),
        ],
        sections: &[
            DeepDiveSection {
                heading: "Problem & Context",
                points: &[
                    "Python-based Learning-to-Rank inference added avoidable scoring latency on every search query.",
                    "Intermittent multi-second spikes on the GPU-hosted ESCI relevance model cascaded into overall Search slowdowns and stability issues.",
                ],
            },
            DeepDiveSection {
                heading: "Architecture & Key Decisions",
                points: &[
                    "Migrated Python LTR inference to ONNX Runtime, achieving 3× lower inference latency with predictable single-digit-millisecond scoring.",
                    "Introduced asynchronous write-back caching and stripped unnecessary logging overhead on the GPU-hosted ESCI service, slashing tail-latency spikes by ~96% (25× improvement).",
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
        id: "bb-agentic-tracing",
        company: "Best Buy",
        company_id: "bestbuy",
        period: "May 2025 – Present",
        category: "Agentic AI · Observability",
        title: "Agentic Tracing (Arize Phoenix → Cloud Trace), TTFT & On-Call",
        one_liner: "Migrated agentic tracing from an inefficient Cloud Run Arize Phoenix collector to Google Cloud Trace, uncovered latency spikes to cut pre-launch TTFT by 20%, and wired Cloud Monitoring + GoAlert on-call routines.",
        headline_badges: &[
            ("−20% pre-launch TTFT", MetricTone::Signal),
            ("Arize Phoenix → Cloud Trace", MetricTone::Accent),
            ("Cloud Monitoring · GoAlert", MetricTone::Default),
        ],
        flow: "Agentic AI I/O → Cloud Trace + Structured Logs → Gemma vs. Gemini Dashboards → Cloud Alerting → GoAlert On-Call",
        comparisons: &[
            NotebookComparison {
                label: "Trace Collector & Backend",
                before: "Arize Phoenix (Cloud Run)",
                after: "Google Cloud Trace",
                delta: "Native · Low-overhead",
                tone: MetricTone::Accent,
            },
            NotebookComparison {
                label: "Pre-Launch Agent TTFT",
                before: "Pre-launch baseline",
                after: "Optimized critical path",
                delta: "−20% TTFT",
                tone: MetricTone::Signal,
            },
            NotebookComparison {
                label: "Incident Response",
                before: "Ad-hoc debugging",
                after: "Monitoring + GoAlert",
                delta: "Eng & DS on-call",
                tone: MetricTone::Signal,
            },
        ],
        callouts: &[
            NotebookCallout {
                style: CalloutStyle::PurpleDashed,
                title: "Why Move Off Arize Phoenix on Cloud Run?",
                body: "The Arize Phoenix trace collector was inefficient on our Cloud Run instance. Migrating the entire tracing pipeline to managed Google Cloud Trace + structured Cloud Logging unlocked reliable request-level visibility across every agentic hop.",
                link: None,
            },
            NotebookCallout {
                style: CalloutStyle::GreenCheck,
                title: "Pre-Launch Latency Triage & On-Call Readiness",
                body: "Pinpointed the exact agentic spans causing latency spikes to cut pre-launch TTFT by 20%, and coupled live dashboards with Cloud Alerting + GoAlert rotations so engineers and scientists can triage production issues cleanly.",
                link: None,
            },
        ],
        sticky_notes: &[StickyNote {
            tone: StickyTone::Yellow,
            title: "Observability & On-Call Stack",
            items: &[
                "End-to-end Cloud Trace spans + structured agentic I/O logs",
                "Live model dashboards comparing fine-tuned Gemma vs. Gemini",
                "Alert policies routed into GoAlert for engineer & scientist on-call",
            ],
        }],
        key_outcomes: &[
            ("−20%", "Pre-launch TTFT reduction", MetricTone::Signal),
            ("Cloud Trace", "Replaced Cloud Run Phoenix", MetricTone::Accent),
            ("GoAlert", "Eng & DS on-call routing", MetricTone::Signal),
            ("Gemma vs Gemini", "Live model dashboards", MetricTone::Default),
        ],
        sections: &[
            DeepDiveSection {
                heading: "1 · Arize Phoenix → Google Cloud Trace Migration",
                points: &[
                    "Migrated the entire agentic tracing pipeline off Arize Phoenix — whose trace collector ran inefficiently on our Cloud Run instance — onto native Google Cloud Trace paired with Cloud Logging and structured agentic I/O logs.",
                    "Established organization-wide request-level span visibility across prompts, tool invocations, retrieval hops, and LLM responses.",
                ],
            },
            DeepDiveSection {
                heading: "2 · Pre-Launch Latency Diagnostics (−20% TTFT)",
                points: &[
                    "Used request-level Cloud Trace waterfalls to uncover bottlenecks in the agentic orchestration where latencies were spiking.",
                    "Eliminated critical-path stalls during initial pre-launch testing, cutting Time to First Token (TTFT) by 20% ahead of initial release.",
                ],
            },
            DeepDiveSection {
                heading: "3 · Monitoring, Alerting & GoAlert On-Call",
                points: &[
                    "Contributed monitoring dashboards comparing in-house fine-tuned Gemma models against Gemini models across latency and runtime behavior.",
                    "Set up Cloud Monitoring & Alerting integrated with GoAlert on-call routines so both engineers and data scientists could reliably triage and handle production issues when on call.",
                ],
            },
        ],
        stack: &[
            ("Google Cloud Trace", true),
            ("Cloud Monitoring", true),
            ("GoAlert", true),
            ("Cloud Logging", true),
            ("Arize Phoenix → GCP", false),
            ("Gemini / Gemma", false),
            ("Cloud Run", false),
        ],
    },
    DeepDiveProject {
        id: "bb-agentic-ai",
        company: "Best Buy",
        company_id: "bestbuy",
        period: "May 2025 – Present",
        category: "Agentic AI · Tooling & Evals",
        title: "LLM-as-a-Judge Evals & Unified Rust/Dioxus Suite",
        one_liner: "Automated LLM-as-a-judge regression evals (1–2 days → <1 hr) and evolved a plain chat-only Chainlit UI into a full Rust/Dioxus suite (Chat Comparator, Perf Test, Regression & utilities) under one endpoint.",
        headline_badges: &[
            ("1–2 days → <1 hr eval", MetricTone::Signal),
            ("27× lower UI compute", MetricTone::Accent),
            ("Single-endpoint Dioxus suite", MetricTone::Default),
        ],
        flow: "Unified Dioxus Suite [Chat Comparator · Perf Test · Regression · Utilities] → LLM-as-a-Judge & Endpoint Smoke Tests → Automated Release Validation",
        comparisons: &[
            NotebookComparison {
                label: "Product Release Validation",
                before: "1–2 days manual",
                after: "<1 hr automated",
                delta: "24×+ faster",
                tone: MetricTone::Signal,
            },
            NotebookComparison {
                label: "Internal AI Workbench",
                before: "Chat-only Chainlit / Streamlit",
                after: "Unified Dioxus Suite",
                delta: "27× lower compute",
                tone: MetricTone::Accent,
            },
        ],
        callouts: &[NotebookCallout {
            style: CalloutStyle::GreenCheck,
            title: "One Endpoint for the Entire AI Workflow",
            body: "Consolidated scattered manual scripts and a chat-only Chainlit UI into a single Rust/Dioxus app covering Chat Comparator, Self-Service Perf Testing, Regression Evals, and everyday engineering utilities.",
            link: None,
        }],
        sticky_notes: &[],
        key_outcomes: &[
            ("<1 hr", "Regression validation", MetricTone::Signal),
            ("27×", "Lower UI compute (Dioxus)", MetricTone::Accent),
            ("1 endpoint", "Comparator · Perf · Regression", MetricTone::Accent),
        ],
        sections: &[
            DeepDiveSection {
                heading: "1 · Automated Evaluation & Self-Service Testing",
                points: &[
                    "Built an LLM-as-a-judge regression suite, API comparator, endpoint smoke tests, and self-service performance testing — cutting product validation from 1–2 days to under 1 hour.",
                    "Enabled engineers and scientists to catch quality and latency regressions before formal performance runs.",
                ],
            },
            DeepDiveSection {
                heading: "2 · Unified Rust/Dioxus Engineering Suite",
                points: &[
                    "Transformed a plain vanilla chat-only Chainlit UI (and Streamlit tooling) into a full self-service Rust/Dioxus suite under a single endpoint — bringing together Chat Comparator, Perf Testing, Regression Evaluation, and everyday engineering utilities.",
                    "Cut UI container compute costs by 27× while maintaining or improving responsiveness.",
                ],
            },
        ],
        stack: &[
            ("LLM-as-a-Judge", true),
            ("Dioxus (Rust)", true),
            ("Gemini / Gemma", true),
            ("Google ADK", false),
            ("Chainlit", false),
            ("Streamlit", false),
        ],
    },
    DeepDiveProject {
        id: "fl-ltr-polars",
        company: "Falabella",
        company_id: "falabella",
        period: "Apr 2023 – Apr 2025",
        category: "Ranking & MLOps",
        title: "15-Min LTR Retraining Pipeline with Polars",
        one_liner: "Rebuilt Learning-to-Rank retraining using Polars and Kubernetes CronJobs — cutting model creation from 24 hours to under 15 minutes and cloud spend by >20% across 5 business units.",
        headline_badges: &[
            ("24 hrs → <15 min LTR", MetricTone::Signal),
            (">20% lower cloud cost · 5 BUs", MetricTone::Accent),
            ("Pandas → Polars", MetricTone::Default),
        ],
        flow: "Kubernetes CronJobs → Polars Feature Pipelines → XGBoost LTR Retraining (5 BUs) → NDCG / RPV / CTR Optimization",
        comparisons: &[
            NotebookComparison {
                label: "LTR Model Retraining (5 BUs)",
                before: "24 hours (Pandas)",
                after: "<15 minutes (Polars)",
                delta: "96× faster",
                tone: MetricTone::Signal,
            },
            NotebookComparison {
                label: "Retraining Cloud Spend",
                before: "Legacy Pandas jobs",
                after: "Polars + K8s CronJobs",
                delta: ">20% cost cut",
                tone: MetricTone::Accent,
            },
        ],
        callouts: &[NotebookCallout {
            style: CalloutStyle::GreenCheck,
            title: "Polars Inside the LTR Retraining Loop",
            body: "Migrated data prep inside the LTR retraining pipeline from Pandas to Polars — slashing CPU and RAM usage, minimizing job failures, and scaling cleanly across five business units via Kubernetes CronJobs.",
            link: None,
        }],
        sticky_notes: &[],
        key_outcomes: &[
            ("<15 min", "LTR retraining (was 24 h)", MetricTone::Signal),
            (">20%", "Cloud cost reduction", MetricTone::Accent),
            ("5 BUs", "Multi-unit ranking scale", MetricTone::Default),
        ],
        sections: &[
            DeepDiveSection {
                heading: "1 · Pandas → Polars LTR Retraining",
                points: &[
                    "Revamped the Learning-to-Rank (LTR) model retraining pipeline using Polars in place of Pandas, cutting model creation time from 24 hours to under 15 minutes across five business units while optimizing for NDCG, RPV, and CTR.",
                    "Improved pipeline maintainability while lowering CPU and RAM consumption and minimizing training job failures.",
                ],
            },
            DeepDiveSection {
                heading: "2 · Kubernetes CronJobs & Cost Efficiency",
                points: &[
                    "Orchestrated scheduled MLOps retraining via Kubernetes CronJobs, reducing cloud infrastructure costs by >20% across all five business units.",
                ],
            },
        ],
        stack: &[
            ("Polars", true),
            ("Learning to Rank", true),
            ("Kubernetes CronJobs", true),
            ("XGBoost", false),
            ("Pandas", false),
        ],
    },
    DeepDiveProject {
        id: "fl-fastapi-migration",
        company: "Falabella",
        company_id: "falabella",
        period: "Apr 2023 – Apr 2025",
        category: "Search Serving",
        title: "Flask → FastAPI Async Migration & Search Reliability",
        one_liner: "Migrated synchronous Flask search APIs to async FastAPI (−40% daily timeouts), added Datadog/Loki/Slack monitoring, and shipped related-keywords search (+7% engagement).",
        headline_badges: &[
            ("−40% daily timeouts", MetricTone::Signal),
            ("+7% engagement · Related Keywords", MetricTone::Accent),
        ],
        flow: "Client Search Traffic → Async FastAPI Services → Model Inference Pipelines → Datadog · Loki · Slack Alerting",
        comparisons: &[
            NotebookComparison {
                label: "Daily API Timeouts",
                before: "Sync Flask",
                after: "Async FastAPI",
                delta: "−40% timeouts",
                tone: MetricTone::Signal,
            },
            NotebookComparison {
                label: "Search Discovery",
                before: "Baseline search",
                after: "Related-keywords feature",
                delta: "+7% engagement",
                tone: MetricTone::Accent,
            },
        ],
        callouts: &[],
        sticky_notes: &[],
        key_outcomes: &[
            ("−40%", "Daily API timeouts", MetricTone::Signal),
            ("+7%", "User engagement (2 BUs)", MetricTone::Accent),
            ("24/7", "Datadog · Loki · Slack alerts", MetricTone::Default),
        ],
        sections: &[
            DeepDiveSection {
                heading: "1 · Flask → Async FastAPI Serving",
                points: &[
                    "Migrated backend search services from synchronous Flask to async FastAPI, eliminating blocking I/O bottlenecks and reducing daily timeouts by 40%.",
                    "Added continuous monitoring and alerting for API and model-inference pipelines with Slack, Loki, and Datadog.",
                ],
            },
            DeepDiveSection {
                heading: "2 · Search Product Impact",
                points: &[
                    "Built a related-keywords search feature that increased user engagement by 7% across two major business segments.",
                ],
            },
        ],
        stack: &[
            ("FastAPI", true),
            ("Flask", true),
            ("Datadog", true),
            ("Loki", false),
            ("Slack Alerts", false),
        ],
    },
    DeepDiveProject {
        id: "fl-ab-testing",
        company: "Falabella",
        company_id: "falabella",
        period: "Apr 2023 – Apr 2025",
        category: "Experimentation",
        title: "In-House Cookie A/B Testing & Cyber Load Readiness",
        one_liner: "Replaced third-party experimentation vendors with an in-house cookie-based A/B engine ($50K/yr saved) and stress-tested search resilience up to 15,000 RPS ahead of biannual cyber events.",
        headline_badges: &[
            ("Up to 15K RPS cyber load", MetricTone::Accent),
            ("$50K/yr saved", MetricTone::Signal),
            ("Zero 3rd-party dependency", MetricTone::Default),
        ],
        flow: "User Session Cookie Bucketing → Variant Routing → Search Telemetry · Cyber Load Testing (Up to 15K RPS)",
        comparisons: &[NotebookComparison {
            label: "Experimentation Software Spend",
            before: "3rd-party SaaS vendor",
            after: "In-house cookie engine",
            delta: "$50K/yr saved",
            tone: MetricTone::Signal,
        }],
        callouts: &[NotebookCallout {
            style: CalloutStyle::GreenCheck,
            title: "Cyber Load Preparedness: Tested Up to 15,000 RPS",
            body: "Partnered with the cyber team on formal performance & stress testing up to 15K RPS ahead of biannual cyber sales events — validating autoscaling headroom, throughput limits, and tail latency under peak burst traffic.",
            link: None,
        }],
        sticky_notes: &[],
        key_outcomes: &[
            ("15K RPS", "Cyber load-test preparedness", MetricTone::Accent),
            ("$50K/yr", "Software spend eliminated", MetricTone::Signal),
            ("Biannual", "Cyber-event load readiness", MetricTone::Default),
        ],
        sections: &[
            DeepDiveSection {
                heading: "Problem & Context",
                points: &[
                    "Third-party experimentation tooling added recurring annual software spend ($50K/yr) and external integration overhead for search experiments.",
                    "Biannual high-traffic cyber events demanded predictable tail latency and resilience up to 15,000 RPS across the search and ranking stack.",
                ],
            },
            DeepDiveSection {
                heading: "Architecture & Key Decisions",
                points: &[
                    "Partnered with the cyber team on formal performance and stress testing up to 15,000 RPS ahead of biannual cyber sales events to validate system headroom, scaling behavior, and tail latency.",
                    "Engineered a lightweight, cookie-based A/B testing solution in-house, eliminating third-party vendor reliance and saving $50K annually.",
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
        id: "ac-gcp-terraform",
        company: "Accenture AI",
        company_id: "accenture",
        period: "Dec 2020 – Apr 2023",
        category: "Cloud & MLOps",
        title: "GCP Terraform MLOps Automation",
        one_liner: "Standardized dev/stage/prod GCP ML infrastructure with reusable Terraform workflows for consistent, repeatable deployments.",
        headline_badges: &[
            ("−50% manual setup", MetricTone::Signal),
            ("dev · stage · prod IaC", MetricTone::Default),
        ],
        flow: "Terraform Modules (IaC) → Automated GCP Provisioning [dev · stage · prod] → Reproducible ML Workloads",
        comparisons: &[NotebookComparison {
            label: "Environment Setup Effort",
            before: "Manual GCP provisioning",
            after: "Reusable Terraform IaC",
            delta: "−50% setup time",
            tone: MetricTone::Signal,
        }],
        callouts: &[],
        sticky_notes: &[],
        key_outcomes: &[
            ("−50%", "Manual GCP setup effort", MetricTone::Signal),
            ("3 envs", "dev / stage / prod parity", MetricTone::Default),
        ],
        sections: &[DeepDiveSection {
            heading: "Infrastructure Automation",
            points: &[
                "Constructed robust Terraform automation workflows for GCP resource management across dev, stage, and prod ecosystems, cutting manual setup effort by 50% and improving cross-project consistency.",
            ],
        }],
        stack: &[
            ("Terraform", true),
            ("Google Cloud (GCP)", true),
            ("MLOps IaC", true),
            ("Vertex AI", false),
            ("BigQuery", false),
        ],
    },
    DeepDiveProject {
        id: "kf-amfi-risk",
        company: "Kaleidofin",
        company_id: "kaleidofin",
        period: "Jun 2019 – Dec 2020",
        category: "Fintech Risk & Quant Analytics",
        title: "AMFI Mutual Fund Risk Engine & Avoiding the $3.4B Franklin Collapse",
        one_liner: "Scraped AMFI daily NAV feeds and built Sharpe-ratio & credit-quality risk tooling — flagging hidden credit risk behind Franklin MF's +1–2% XIRR edge ahead of its $3.4B collapse.",
        headline_badges: &[
            ("Avoided $3.4B MF collapse", MetricTone::Signal),
            ("AMFI NAV · Sharpe Ratio", MetricTone::Accent),
            ("Underbanked customer protection", MetricTone::Default),
        ],
        flow: "AMFI (India) Daily NAV Scraper → Unified Fund Return Dataset → Risk Engine [Sharpe Ratio · XIRR · Credit Rating Audit] → Safe Fund Shortlist",
        comparisons: &[
            NotebookComparison {
                label: "Fund Selection Screen",
                before: "Surface XIRR & headline ratings",
                after: "Sharpe Ratio + rating-agency audit",
                delta: "Flagged hidden credit risk",
                tone: MetricTone::Signal,
            },
            NotebookComparison {
                label: "Franklin MF Debt Schemes",
                before: "+1–2% higher XIRR (enticing)",
                after: "Excluded from customer offers",
                delta: "Dodged $3.4B collapse",
                tone: MetricTone::Accent,
            },
        ],
        callouts: &[
            NotebookCallout {
                style: CalloutStyle::PurpleDashed,
                title: "Quantitative Screen · Sharpe Ratio = (Rp − Rf) / σp",
                body: "Measured excess return per unit of volatility — where Rp is the mutual fund return, Rf is the risk-free rate, and σp is the standard deviation of the fund's returns — to narrow the entire AMFI fund universe to schemes with genuinely strong risk-adjusted performance.",
                link: None,
            },
            NotebookCallout {
                style: CalloutStyle::GreenCheck,
                title: "Spotting the Franklin Templeton Red Flag Before the Crash",
                body: "Franklin schemes outperformed peers by 1–2% XIRR and looked well-rated on the surface, but deep-diving the holdings revealed risky, lower-grade paper backed by weaker credit rating agencies. We refused to recommend them — safeguarding underbanked customers months before Franklin's $3.4B debt-fund collapse.",
                link: None,
            },
        ],
        sticky_notes: &[StickyNote {
            tone: StickyTone::Yellow,
            title: "Why the Tooling Worked",
            items: &[
                "Scraped AMFI for full historical mutual fund NAVs",
                "Ranked funds by Sharpe Ratio = (Rp − Rf) / σp & XIRR",
                "Audited underlying credit ratings & agency credibility",
                "Protected underbanked customers from $3.4B wipeout",
            ],
        }],
        key_outcomes: &[
            ("$3.4B", "Franklin collapse avoided", MetricTone::Signal),
            ("+1–2% XIRR", "Outlier yield trap flagged", MetricTone::Accent),
            ("AMFI NAV", "Full India MF universe scraped", MetricTone::Default),
        ],
        sections: &[
            DeepDiveSection {
                heading: "1 · AMFI Data Scraping & NAV Capture",
                points: &[
                    "Built an automated scraping pipeline against AMFI (Association of Mutual Funds in India) to continuously capture daily Net Asset Value (NAV) histories across Indian mutual fund schemes.",
                    "Consolidated raw NAV series into an analytical dataset that let the team systematically evaluate and narrow the investment search space rather than relying on broker pitches or headline rankings.",
                ],
            },
            DeepDiveSection {
                heading: "2 · Risk-Adjusted Scoring & Sharpe Ratio Formula",
                points: &[
                    "Engineered quantitative screening tooling on top of the AMFI NAV dataset to compute XIRR and risk-adjusted return metrics, anchored by the Sharpe Ratio: Sharpe Ratio = (Rp − Rf) / σp — where Rp is the portfolio/fund return, Rf is the risk-free benchmark rate, and σp is the standard deviation (volatility) of excess returns.",
                    "Used a strong Sharpe Ratio threshold alongside volatility and drawdown checks to filter for funds that delivered consistent risk-adjusted performance suited to lower-income, underbanked savers.",
                ],
            },
            DeepDiveSection {
                heading: "3 · Exposing Hidden Credit Risk Ahead of the $3.4B Franklin Collapse",
                points: &[
                    "During screening, Franklin Mutual Fund debt schemes stood out by outperforming peer funds by 1–2% XIRR and displaying attractive headline ratings.",
                    "A deeper inspection into the underlying holdings revealed the giveaway: the yield advantage came from insecure, poorly rated debt instruments whose favorable ratings were issued by weaker, lower-tier credit rating agencies rather than top-tier agencies.",
                    "Based on this analysis, we chose not to sponsor or recommend Franklin funds to Kaleidofin's underbanked customers. Months later, Franklin Templeton abruptly wound down six debt funds in a $3.4 billion collapse — validating the risk framework and directly safeguarding customer savings.",
                ],
            },
        ],
        stack: &[
            ("AMFI NAV Scraping", true),
            ("Sharpe Ratio / XIRR", true),
            ("Credit Risk Analysis", true),
            ("Python", true),
            ("Portfolio Analytics", false),
            ("Mutual Funds", false),
        ],
    },
];

/// Add entries here (newest first) to publish blog posts on the dedicated Writing page.
pub const BLOG_POSTS: &[BlogPost] = &[BlogPost {
    slug: "fastapi-to-rust-semantic-search",
    date: "Oct 2026",
    year: "2026",
    topic: "Search & Rust Systems",
    read_time: "6 min read",
    title: "Migrating Semantic Search from FastAPI to Rust (Axum)",
    subtitle: "A practical walkthrough of how to migrate a live Python semantic search stack to a hexagonal Rust monolith — covering vertical-slice porting, bulkhead connection pools, async-safe inference, and safe production cutover.",
    tags: &[
        "Rust",
        "Axum",
        "FastAPI",
        "Hexagonal Architecture",
        "Bulkhead Pools",
        "ARM / Titanium",
        "ONNX",
        "Migration Playbook",
    ],
    blocks: &[
        ArticleBlock::Paragraph(
            "FastAPI is a great way to get a semantic search and ranking pipeline into production quickly. Over time, though, high-concurrency workloads start exposing the hidden cost of a Python microservice stack: multi-worker memory overhead, repeated JSON serialization between internal services, and unpredictable tail latency when CPU-bound scoring competes with async I/O.",
        ),
        ArticleBlock::Paragraph(
            "When we set out to migrate a live semantic search system from FastAPI to Rust (Axum), the hardest part was never Rust syntax — it was changing the architecture in flight without breaking behavioral parity or disrupting production traffic. Below is a high-level walkthrough of how the migration actually took place, step by step.",
        ),
        ArticleBlock::FieldNotes {
            stamp: "Architecture Field Notes",
            comparisons: &[
                NotebookComparison {
                    label: "Service Topology",
                    before: "Separate Python HTTP services",
                    after: "Single hexagonal Rust binary",
                    delta: "Zero internal HTTP hops",
                    tone: MetricTone::Signal,
                },
                NotebookComparison {
                    label: "Async Connection Pools",
                    before: "Shared multiplexed clients",
                    after: "Per-workload bulkheads",
                    delta: "No head-of-line blocking",
                    tone: MetricTone::Signal,
                },
                NotebookComparison {
                    label: "Hot-Path Data Model",
                    before: "Per-request DataFrames",
                    after: "Native structs & slices + LTO",
                    delta: "Lower memory & FFI churn",
                    tone: MetricTone::Accent,
                },
                NotebookComparison {
                    label: "Model Inference Scheduling",
                    before: "Blocking on async workers",
                    after: "Session pool + spawn_blocking",
                    delta: "Unblocked async I/O",
                    tone: MetricTone::Signal,
                },
                NotebookComparison {
                    label: "Compute & Node Shape",
                    before: "Legacy x86 + memory-heavy nodes",
                    after: "ARM Titanium + CPU-focused nodes",
                    delta: "Leaner cloud footprint",
                    tone: MetricTone::Accent,
                },
            ],
            callouts: &[NotebookCallout {
                style: CalloutStyle::GreenCheck,
                title: "Step 0 · Evaluate Hardware Before Rewriting Application Code",
                body: "Before touching a single endpoint, look closely at your compute generation. Evaluating newer ARM-based cloud VMs with hardware-offloaded networking and I/O (such as Google Cloud Titanium) can unlock substantial latency and efficiency gains on your existing Python service while buying breathing room for the rewrite.",
                link: Some((
                    "Reference: Google Cloud Titanium ↗",
                    "https://cloud.google.com/titanium",
                )),
            }],
            sticky_notes: &[],
        },
        ArticleBlock::Heading("1. Don't Transliterate — Port in Vertical Slices"),
        ArticleBlock::Paragraph(
            "The easiest trap in a Python-to-Rust migration is trying to translate files one by one. Python codebases naturally lean on dynamic dictionaries, shared mutable helpers, and loose implicit contracts; porting those idioms literally into Rust creates painful borrow-checker friction without delivering cleaner architecture.",
        ),
        ArticleBlock::Paragraph(
            "Instead, we migrated by vertical slice: pick one stage or request path, model its inputs and outputs with explicit domain types, wire it through real production adapters (no long-lived placeholder stubs), and prove behavioral equivalence against the existing FastAPI service before moving to the next slice.",
        ),
        ArticleBlock::Heading("2. Collapsing Microservices into a Hexagonal Monolith"),
        ArticleBlock::Paragraph(
            "In a multi-stage search pipeline — query understanding, multi-tier retrieval, metadata hydration, and learning-to-rank reranking — splitting every stage into its own Python microservice means paying network latency and JSON encode/decode overhead on every hop. Collapsing those stages into one Rust/Axum binary lets them pass typed structs in memory.",
        ),
        ArticleBlock::HexagonalSketch,
        ArticleBlock::Paragraph(
            "To keep a single binary modular as the codebase grows, we leaned on Cargo workspace boundaries so compile-time dependencies only ever flow inward (api → domain ← infra):",
        ),
        ArticleBlock::Bullets(&[
            (
                "domain",
                "Owns business types, orchestration logic, typed errors, and async port traits — completely unaware of HTTP clients, caches, or databases.",
            ),
            (
                "infra",
                "Implements those domain ports and keeps wire-format JSON, database rows, and SDK clients strictly isolated at the edges.",
            ),
            (
                "api",
                "Stays paper-thin: validate request DTOs, invoke a domain service, and map typed outputs back into the API contract — with zero unwrap() or expect() on request paths.",
            ),
            (
                "Observability",
                "Keeps microservice-clear visibility: distinct OpenTelemetry span prefixes per logical stage (query parsing, recall, reranking, and downstream I/O) preserve readable trace waterfalls inside one process.",
            ),
        ]),
        ArticleBlock::Heading("3. Isolating Workloads with Bulkheads & Async-Safe Inference"),
        ArticleBlock::Paragraph(
            "Rewriting in Rust does not automatically eliminate tail latency if unrelated I/O and CPU workloads contend for the same resources. Two practical concurrency patterns mattered more than almost any micro-optimization:",
        ),
        ArticleBlock::FieldNotes {
            stamp: "Concurrency & Isolation Notes",
            comparisons: &[],
            callouts: &[NotebookCallout {
                style: CalloutStyle::PurpleDashed,
                title: "Why Per-Workload Bulkhead Pools?",
                body: "Multiplexed async connections return responses in order. If a small, latency-sensitive ranking lookup shares a connection pool with large batch metadata fetches, a single heavy payload can head-of-line-block tiny lookups behind it. Giving each logical dependency its own dedicated pool isolates tail latency cleanly.",
                link: None,
            }],
            sticky_notes: &[
                StickyNote {
                    tone: StickyTone::Yellow,
                    title: "Retrieval & Caching Hygiene",
                    items: &[
                        "Keep multi-tier recall fan-out concurrent AND order-preserving so merge/dedup parity stays deterministic",
                        "Start independent cache lookups early — overlapping them with retrieval rather than waiting sequentially",
                        "Use fast content hashing for cache freshness and flush updates via a bounded background writer that backs off under memory pressure",
                        "Cache deterministic query-parsing responses in a small pod-local LRU to skip redundant network hops",
                    ],
                },
                StickyNote {
                    tone: StickyTone::Pink,
                    title: "Keeping Async Workers Unblocked",
                    items: &[
                        "Never run CPU-heavy ONNX model scoring behind a single mutex on Tokio async workers — it starves network I/O tasks",
                        "Back each model branch with a small session pool and an admission semaphore",
                        "Offload synchronous Session::run calls via tokio::task::spawn_blocking with bounded per-session threads",
                    ],
                },
            ],
        },
        ArticleBlock::Heading("4. Replacing Hot-Path DataFrames with Native Structs & LTO"),
        ArticleBlock::Paragraph(
            "DataFrame libraries shine in offline batch pipelines where you transform millions of rows at once. Inside an online search request, however, you are usually scoring a compact candidate cohort. Constructing DataFrame schemas, allocating columnar buffers, and crossing FFI boundaries per request adds avoidable overhead.",
        ),
        ArticleBlock::Paragraph(
            "Modeling candidates, features, scores, and grouped results directly as typed Rust structs, borrowed slices, and zero-cost iterators keeps hot data contiguous and cache-friendly while cutting allocator churn. Pairing that data model with an aggressive release profile — full optimization across dependencies, Fat LTO, and a single codegen unit — allows LLVM to inline cleanly across hexagonal crate boundaries and emit lean ARM machine code.",
        ),
        ArticleBlock::Heading("5. Parity Verification, Cold-Start Safety & Right-Sizing"),
        ArticleBlock::Paragraph(
            "The last mile of any migration is building confidence that the new binary behaves identically under real traffic and scales cleanly when clusters expand:",
        ),
        ArticleBlock::Bullets(&[
            (
                "Automated Parity Checks",
                "Replay curated queries side-by-side against FastAPI and Rust, diffing ranked outputs, response envelopes, and structured stage logs before shifting live traffic.",
            ),
            (
                "Scale-Out Startup Safety",
                "When many pods start together during spikes, racing ahead of the service-mesh proxy or cloud metadata server causes avoidable boot crashes. Holding startup until the proxy is ready — paired with a bounded metadata readiness wait — keeps autoscaling smooth.",
            ),
            (
                "Right-Sizing Node Shapes",
                "Once a compiled binary dramatically lowers memory requirements compared to Python workers, memory-heavy node pools are often over-provisioned. Shifting toward leaner, CPU-focused ARM profiles keeps clusters light off-peak while scaling cleanly for peak events.",
            ),
        ]),
    ],
},
BlogPost {
    slug: "gpu-reranker-async-writeback-caching",
    date: "Sep 2026",
    year: "2026",
    topic: "Performance & Reliability",
    read_time: "5 min read",
    title: "Taming GPU Reranker Tail-Latency Spikes with Async Write-Back Caching",
    subtitle: "How overlapping cache reads with retrieval, content-hash freshness checks, bounded async write-back, and leaner logging eliminate multi-second GPU tail spikes without sacrificing freshness.",
    tags: &[
        "GPU Inference",
        "Async Write-Back",
        "Redis",
        "XXH3",
        "Tail Latency",
        "Resilience",
    ],
    blocks: &[
        ArticleBlock::Paragraph(
            "Cross-encoder and relevance reranking models running on GPUs deliver a huge leap in search quality, but they behave very differently from CPU services under burst traffic. Even when median latency looks completely healthy, a sudden wave of high-fan-out queries can saturate GPU queues and trigger dramatic multi-second tail-latency spikes.",
        ),
        ArticleBlock::Paragraph(
            "Throw verbose per-request payload logging and synchronous cache writes on top of that, and the slow path starts compounding itself. Below is the caching and resilience pattern we used to cut GPU reranker tail latency by an order of magnitude while keeping results fresh and responses non-blocking.",
        ),
        ArticleBlock::FieldNotes {
            stamp: "Architecture Field Notes",
            comparisons: &[
                NotebookComparison {
                    label: "Cache Lookup Scheduling",
                    before: "Sequential wait after recall",
                    after: "Early read overlapping recall",
                    delta: "Hidden lookup latency",
                    tone: MetricTone::Signal,
                },
                NotebookComparison {
                    label: "Freshness Invalidation",
                    before: "Blind TTL or full re-score",
                    after: "Override → XXH3 content hash",
                    delta: "Misses-only GPU scoring",
                    tone: MetricTone::Signal,
                },
                NotebookComparison {
                    label: "Cache Persistence Path",
                    before: "Inline blocking Redis writes",
                    after: "Bounded async batch writer",
                    delta: "Zero write wait on response",
                    tone: MetricTone::Accent,
                },
                NotebookComparison {
                    label: "Cache Memory Safety",
                    before: "Unbounded write bursts",
                    after: "Memory watermark circuit breaker",
                    delta: "Prevents eviction storms",
                    tone: MetricTone::Signal,
                },
                NotebookComparison {
                    label: "Hot-Path Observability",
                    before: "Verbose payload dumps on INFO",
                    after: "Lean metrics + debug-guarded logs",
                    delta: "Removes serialization drag",
                    tone: MetricTone::Accent,
                },
            ],
            callouts: &[NotebookCallout {
                style: CalloutStyle::GreenCheck,
                title: "Golden Rule · The Cache Must Never Be a New Failure Mode",
                body: "Every cache read fails open directly to the GPU model, and every cache write happens asynchronously off the critical path. If the cache store slows down, fills up, or drops connections, user requests still return cleanly with zero error amplification.",
                link: None,
            }],
            sticky_notes: &[],
        },
        ArticleBlock::Heading("1. Overlap Cache Reads with Retrieval"),
        ArticleBlock::Paragraph(
            "A naive cache wrapper sits right in front of the GPU client: wait for retrieval to finish, build the full candidate list, check Redis, and then score misses. That adds an extra serial network round-trip to every single request — even when the cache is cold.",
        ),
        ArticleBlock::Paragraph(
            "Instead, kick off the cache lookup as soon as the normalized query is available so it executes concurrently alongside candidate retrieval and metadata hydration. By the time candidates are ready for reranking, the cached relevance map is already sitting in memory waiting to be joined.",
        ),
        ArticleBlock::AsyncCacheSketch,
        ArticleBlock::Heading("2. Score Only Stale or Missing Items with Content Hashing"),
        ArticleBlock::Paragraph(
            "Catalog items change over time — titles, attributes, and specs get updated — so blindly serving cached relevance scores until a long TTL expires risks serving stale rankings. Conversely, setting tiny TTLs destroys hit rates and sends redundant traffic back to the GPU.",
        ),
        ArticleBlock::Paragraph(
            "We solved this by storing a compact 64-bit content hash (XXH3) alongside each cached score and evaluating every candidate through a deterministic lookup ladder:",
        ),
        ArticleBlock::Bullets(&[
            (
                "Override First",
                "Check curated or manual relevance overrides before touching either the cache or the GPU so pinned business rules always take precedence.",
            ),
            (
                "XXH3 Freshness Match",
                "Compare the candidate's current text/attribute hash against the cached hash. Exact matches bypass the GPU completely; changed items are automatically treated as misses.",
            ),
            (
                "Misses-Only GPU Batch",
                "Send only uncached or hash-stale candidates to the GPU reranker, dramatically shrinking batch sizes and relieving GPU queue pressure during traffic bursts.",
            ),
            (
                "Order-Preserving Merge",
                "Stitch cached hits and freshly scored GPU misses back into the exact original candidate order so downstream ranking stages remain 100% deterministic.",
            ),
        ]),
        ArticleBlock::Heading("3. Push Writes Off the Critical Path with Backpressure"),
        ArticleBlock::Paragraph(
            "Once the GPU finishes scoring the misses, the user should never have to wait for Redis to persist those new entries. Hand the freshly computed `(item, hash, score)` tuples to a bounded in-memory channel and return the response immediately.",
        ),
        ArticleBlock::FieldNotes {
            stamp: "Write-Back Safety Notes",
            comparisons: &[],
            callouts: &[NotebookCallout {
                style: CalloutStyle::PurpleDashed,
                title: "Why Jittered EXPIRE NX Matters",
                body: "Setting an identical TTL across thousands of keys written during a traffic surge creates synchronized expiration cliffs days later, causing a sudden thundering herd back onto the GPU. Combining HSET with jittered whole-day EXPIRE NX spreads expirations evenly across time without resetting active TTLs on every write.",
                link: None,
            }],
            sticky_notes: &[
                StickyNote {
                    tone: StickyTone::Yellow,
                    title: "Bounded Background Flushing",
                    items: &[
                        "Coalesce queued updates into pipelined HSET batches on a short flush cadence",
                        "Drop newest writes cleanly if the bounded channel fills up — never let memory grow unbounded",
                        "Gate reads and writes with independent runtime switches for safe dark launches",
                    ],
                },
                StickyNote {
                    tone: StickyTone::Pink,
                    title: "Memory Watermark Circuit Breaker",
                    items: &[
                        "Periodically sample Redis memory usage against maxmemory",
                        "Automatically pause background writes above a high-water threshold",
                        "Resume writes only once memory cools back below a lower hysteresis watermark",
                    ],
                },
            ],
        },
        ArticleBlock::Heading("4. Trim Logging Overhead & Ship Safely"),
        ArticleBlock::Paragraph(
            "One of the most surprising contributors to tail latency was not the model itself, but hot-path logging. Serializing large candidate lists and full model request/response payloads at INFO level burns CPU, bloats log pipelines, and adds jitter right when concurrency peaks.",
        ),
        ArticleBlock::Paragraph(
            "Moving verbose payload logs behind debug guards — while keeping lightweight structured counters for hit ratio, miss batch size, and fallback rate — immediately removed hidden serialization drag. Combined with independent read/write feature gates, we could warm the cache silently in write-only mode, verify hit rates and memory headroom, and then enable cache reads gradually with zero risk to live search quality.",
        ),
    ],
},
BlogPost {
    slug: "low-latency-ltr-onnx-runtime",
    date: "Aug 2026",
    year: "2026",
    topic: "Search & Rust Systems",
    read_time: "5 min read",
    title: "Low-Latency Learning to Rank with ONNX Runtime",
    subtitle: "Moving gradient-boosted Learning-to-Rank inference off Python runtimes into a CPU-pinned ONNX session pool with flat feature tensors, non-blocking Tokio scheduling, and multi-branch A/B support.",
    tags: &[
        "Learning to Rank",
        "ONNX Runtime",
        "Rust",
        "Tokio",
        "Concurrency",
        "A/B Testing",
    ],
    blocks: &[
        ArticleBlock::Paragraph(
            "Training a gradient-boosted Learning-to-Rank (LTR) model in Python is comfortable and fast, especially with modern columnar engines. Serving that same model inside a high-concurrency search path, however, often exposes subtle runtime bottlenecks: GIL contention, per-request DataFrame construction overhead, and unpredictable garbage-collection pauses.",
        ),
        ArticleBlock::Paragraph(
            "Exporting the trained tree ensemble to ONNX Runtime unlocks a massive speedup — roughly 3× faster inference with a fraction of the memory footprint — but only if you wire the runtime carefully into an async server. Here are the serving patterns that turned ONNX LTR into a rock-solid, low-jitter ranking stage.",
        ),
        ArticleBlock::FieldNotes {
            stamp: "Architecture Field Notes",
            comparisons: &[
                NotebookComparison {
                    label: "Inference Runtime",
                    before: "Python booster + DataFrames",
                    after: "Native ONNX Runtime (ort)",
                    delta: "~3× faster scoring",
                    tone: MetricTone::Signal,
                },
                NotebookComparison {
                    label: "Feature Packing",
                    before: "Columnar DataFrame per request",
                    after: "Flat contiguous Vec<f32> buffer",
                    delta: "Zero FFI & schema churn",
                    tone: MetricTone::Signal,
                },
                NotebookComparison {
                    label: "Async Concurrency",
                    before: "Single Mutex<Session> lock",
                    after: "Session pool + spawn_blocking",
                    delta: "No async I/O starvation",
                    tone: MetricTone::Accent,
                },
                NotebookComparison {
                    label: "Thread Topology",
                    before: "Default multi-thread per session",
                    after: "1 intra · 1 inter thread / session",
                    delta: "Zero CPU oversubscription",
                    tone: MetricTone::Signal,
                },
                NotebookComparison {
                    label: "Experiment Rollouts",
                    before: "Separate model deployments",
                    after: "In-process A/B branch registry",
                    delta: "Zero-hop model switching",
                    tone: MetricTone::Accent,
                },
            ],
            callouts: &[NotebookCallout {
                style: CalloutStyle::GreenCheck,
                title: "Key Insight · Small Online Batches Favor Concurrency Over Intra-Op Parallelism",
                body: "Online search ranking scores compact candidate cohorts per query under high RPS. Running one thread per ONNX session across a pool of sessions consistently beats letting a single session fan out across many cores — avoiding thread-pool contention and smoothing tail latency.",
                link: None,
            }],
            sticky_notes: &[],
        },
        ArticleBlock::Heading("1. Why a Single Mutex Breaks Async Servers"),
        ArticleBlock::Paragraph(
            "The first working prototype of ONNX in an async Rust server usually wraps a single Session inside a mutex and calls run() directly inside the async request handler. Under light local tests, it looks blazingly fast. Under real production concurrency, tail latency collapses.",
        ),
        ArticleBlock::Paragraph(
            "Two things go wrong simultaneously: concurrent queries serialize behind the single lock, and synchronous CPU-bound tree traversal blocks Tokio's async worker threads — starving unrelated Redis reads, HTTP calls, and health checks sharing the same runtime.",
        ),
        ArticleBlock::OnnxPoolSketch,
        ArticleBlock::Heading("2. Session Pooling, Semaphore Admission & Thread Pinning"),
        ArticleBlock::Paragraph(
            "To keep network I/O responsive while squeezing predictable throughput out of every CPU core, separate feature preparation on async threads from model execution on dedicated blocking threads:",
        ),
        ArticleBlock::Bullets(&[
            (
                "Per-Branch Session Pool",
                "Pre-warm a small pool of ONNX sessions sized to available CPU cores for each active model variant so concurrent requests score in parallel without lock contention.",
            ),
            (
                "Async Admission Semaphore",
                "Acquire a bounded permit asynchronously before checking out a session. Under bursts, waiting tasks suspend cheaply on the async runtime instead of spawning unbounded blocking threads.",
            ),
            (
                "tokio::task::spawn_blocking",
                "Hand the owned flat tensor buffer and checked-out session to spawn_blocking for Session::run, returning the session to the pool cleanly even if execution fails.",
            ),
            (
                "1 Intra / 1 Inter Thread",
                "Pin each ONNX session to 1 intra-op and 1 inter-op thread. Letting N concurrent sessions each spin up M internal worker threads causes severe context-switch thrashing on Kubernetes pods.",
            ),
        ]),
        ArticleBlock::Heading("3. Flat Typed Feature Matrices Over Per-Request DataFrames"),
        ArticleBlock::Paragraph(
            "Even with a fast inference engine, building a DataFrame object for a few dozen or hundred candidates on every query can easily cost more CPU time than the model evaluation itself. Schema validation, dictionary hashing, and scattered heap allocations add up quickly.",
        ),
        ArticleBlock::Paragraph(
            "Instead, model the LTR stage as four clean, testable interfaces: an orchestrator, a feature provider, a deterministic feature encoder, and a model executor. The encoder writes every candidate's numerical and encoded categorical signals directly into a single pre-allocated row-major Vec<f32> sized to candidates × features, passing a zero-copy tensor view straight into ONNX Runtime.",
        ),
        ArticleBlock::FieldNotes {
            stamp: "Resilience & Parity Notes",
            comparisons: &[],
            callouts: &[NotebookCallout {
                style: CalloutStyle::PurpleDashed,
                title: "Multi-Branch A/B Experiments in One Binary",
                body: "Because each ONNX model artifact is compact and self-contained, a single service instance can load control and treatment ranker branches side by side at startup — routing each request by experiment flag with zero extra network hops and isolated pools per branch.",
                link: None,
            }],
            sticky_notes: &[
                StickyNote {
                    tone: StickyTone::Yellow,
                    title: "Graceful Feature Fallbacks",
                    items: &[
                        "Keep strict per-request timeouts on external feature stores",
                        "Fall back cleanly to neutral default values on missing features or timeouts",
                        "Preserve baseline retrieval ordering if the ranker is bypassed",
                    ],
                },
                StickyNote {
                    tone: StickyTone::Pink,
                    title: "Export & Parity Verification",
                    items: &[
                        "Replay real candidate feature sets through both Python and ONNX models",
                        "Verify floating-point score tolerance AND exact top-K rank order parity",
                        "Lock feature ordering with compile-time constants to prevent silent drift",
                    ],
                },
            ],
        },
        ArticleBlock::Heading("4. Proving Rank Parity Before Cutover"),
        ArticleBlock::Paragraph(
            "Tree-model exports can harbor subtle edge cases: missing-value (NaN) branch directions, float32 vs. float64 split boundaries, and categorical encoding mismatches. Checking only aggregate NDCG offline is rarely enough to catch single-query surprises.",
        ),
        ArticleBlock::Paragraph(
            "Before routing live traffic to ONNX, we replayed representative query cohorts through both the legacy Python ranker and the Rust/ONNX path, asserting tight score tolerances and top-K rank-order agreement across edge cases like missing features and tied scores. Once parity held cleanly, flipping traffic over delivered immediate latency and CPU relief with zero relevance regressions.",
        ),
    ],
}];

pub const PLANNED_BLOG_TOPICS: &[PlannedTopic] = &[
    PlannedTopic {
        date_label: "Draft",
        year: "Upcoming",
        topic: "Agentic AI & Evaluation",
        title: "Observability & LLM-as-a-Judge Suites for Agentic Workflows",
        subtitle: "Designing end-to-end distributed tracing, model comparison dashboards, and automated regression checks for agentic systems.",
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
        story_kicker: "Full-Stack AI & Cost Impact",
        story_headline: "Bringing nine years of lessons together at scale",
        story_body: "Everything I've learned across analytics, finance, MLOps, and high-RPS search comes together here — turning engineering fundamentals into massive cost and speed wins. Moving GKE to ARM C4A (−30% latency), rewriting search & UI paths in Rust (Axum & Dioxus) to cut CPU by 95%, RAM by 98%, Search P50 latency by 50%, and agent cost per turn by ~14%, plus Cloud Trace observability (−20% pre-launch TTFT) and LLM-as-a-judge evals.",
        story_takeaway: "−95% CPU · −50% Search P50 · −14% agent cost / turn",
        bullets: &[
            "Migrated the Search 2.0 semantic search platform from Flask to FastAPI, then to Rust with Axum, consolidating microservices into a hexagonal monolith with bulkhead connection pools and concurrent recall fan-out. Cut CPU by 95%, RAM by 98%, Search P50 latency by 50%, drove ~$180K/yr (~$500/day) in savings, and reduced end-to-end agentic cost per turn by ~14%.",
            "Independently identified a 3-generation GKE migration from x86 n1-standard-16 to ARM c4a-standard-16 (Google Cloud Titanium) and partnered with the Platform team for adoption across AML, achieving an immediate 30% latency drop and 20% pod reduction; later right-sized node pools to c4a-highcpu-8, cutting idle per-node cost by 57%.",
            "Migrated Python-based LTR inference to ONNX Runtime, reducing inference latency 3× to predictable single-digit-millisecond scoring.",
            "Supported the setup of NER, sparse, and ESCI models in the search stack and managed ongoing model upgrades.",
            "Optimised holiday-season infrastructure costs by tuning Kubernetes HPA, adding requests-per-second (RPS) scaling, and right-sizing CPU and GPU deployments.",
            "Migrated agentic tracing off an inefficient Cloud Run Arize Phoenix collector to Google Cloud Trace + structured logs, diagnosing latency spikes to cut pre-launch TTFT by 20%, alongside Gemma vs. Gemini dashboards and Cloud Monitoring + GoAlert on-call routines.",
            "Resolved multi-second tail-latency spikes in the GPU-hosted ESCI model (~96% / 25× tail reduction) through asynchronous write-back caching and reduced logging overhead.",
            "Set up LLM-as-a-judge regression evaluation, an API comparator, endpoint smoke tests, and self-service performance testing — cutting product validation from 1–2 days to under 1 hour.",
            "Evolved a plain chat-only Chainlit / Streamlit UI into a unified Rust Dioxus suite under a single endpoint (Chat Comparator, Perf Testing, Regression Evals, and utilities), reducing UI compute costs by 27×.",
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
        story_kicker: "Search & High-RPS Scale",
        story_headline: "Where systems engineering truly clicked",
        story_body: "Falabella is where I learned search inside out and picked up the real craft of scaling — handling high RPS, stress-testing up to 15,000 RPS for biannual cyber events, moving synchronous Flask APIs to async FastAPI (−40% timeouts), and rebuilding LTR retraining with Polars from 24 hours down to under 15 minutes.",
        story_takeaway: "Learned search, 15K RPS load readiness & 15-min LTR",
        bullets: &[
            "Migrated backend services from Flask to FastAPI using asynchronous programming, reducing daily timeouts by 40%. Added continuous monitoring for API and model-inference pipelines with Slack, Loki, and Datadog.",
            "Revamped the LTR model retraining process using Polars, reducing model creation time from 24 hours to under 15 minutes and cloud costs by over 20% across five business units while optimizing for NDCG, RPV, and CTR.",
            "Built a related-keywords search feature that increased user engagement by 7% across two major business segments.",
            "Improved code maintainability while reducing CPU and RAM consumption; migrated Pandas jobs to Polars to minimize job failures and integrated MLOps with Kubernetes CronJobs.",
            "Engineered a cookie-based A/B testing solution that eliminated reliance on third-party tools and saved $50k annually in software costs.",
            "Played a key role in the cyber team's performance testing up to 15,000 RPS, strengthening system resilience ahead of biannual cyber events.",
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
        story_kicker: "MLOps & Production Craft",
        story_headline: "Discovering engineering over pure science",
        story_body: "Accenture is where I really learned engineering. Working heavily on the MLOps side — automating GCP dev, stage, and prod environments with reusable Terraform workflows (−50% manual setup) — showed me that I enjoyed engineering even more than pure data science, because getting models reliably into production mattered so much.",
        story_takeaway: "Turning point: Productionizing ML > pure modeling",
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
        story_kicker: "Mutual Fund Risk Analytics",
        story_headline: "Safeguarding underbanked savers with data",
        story_body: "An opportunity opened up at seed-stage Kaleidofin to dive deeper into mutual fund analytics, and this place taught me how real risk analysis works. Scraping AMFI NAV data and building Sharpe Ratio & credit-quality tooling helped us keep underbanked customers out of Franklin MF months before its $3.4B collapse — work I'm genuinely proud of.",
        story_takeaway: "Proud moment: Avoided $3.4B Franklin MF collapse",
        bullets: &[
            "Designed a data analysis framework for consumer and corporate portfolios, reducing analysis time by 40%, enabling faster investment decisions and improving responsiveness to market changes.",
            "Scraped AMFI daily NAV feeds and built Sharpe Ratio & credit-quality risk tooling — flagging hidden credit risk behind Franklin MF's +1–2% XIRR ahead of its $3.4 billion collapse and safeguarding underbanked customers.",
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
        story_kicker: "Finance & Product Design",
        story_headline: "Entering finance through product analytics",
        story_body: "Following my pull toward finance brought me to FundsIndia, where I focused on product analytics and worked closely with the product and design teams. Mapping mutual fund conversion funnels (+15% conversion) and call-center drop-offs taught me how thoughtful financial products are designed around real users.",
        story_takeaway: "Worked closely with Product & Design · +15% conversion",
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
        story_kicker: "Where It All Started",
        story_headline: "Learning the science of industry analytics",
        story_body: "This is where my career began — learning the science side of industry analytics from the ground up. I worked heavily on interactive pricing dashboards for Pfizer (~$1.2M/yr impact) and automated executive financial pipelines, while realizing my real interest lay in finance.",
        story_takeaway: "Started with dashboards & analytics → spark for finance",
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
        url: Some((
            "Parchment verified diploma",
            "https://www.parchment.com/lp/award/ed53759e-129b-4fdc-a78d-e90fa10e8197",
        )),
    },
    Education {
        kind: "Undergraduate Degree",
        school: "BMS Institute of Technology",
        degree: "B.E., Electronics & Communication",
        location: "Bengaluru, India",
        url: None,
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
