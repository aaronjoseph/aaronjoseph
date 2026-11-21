### Hi, I'm Aaron Joseph Mathew 👋

**Senior AI Engineer** with **9+ years** across analytics, machine learning, and production AI engineering — focused on high-throughput search, learning-to-rank, LLM evaluation, agentic observability, and low-latency **Python** & **Rust** services.

Currently at **Best Buy** (Bengaluru) working on **AskBlue** and **Search 2.0**, building search and AI systems serving up to **12,000 requests per second**.

🌐 **Personal Website & Interactive Résumé:** **[aaronjoseph.github.io/aaronjoseph](https://aaronjoseph.github.io/aaronjoseph/)** _(built with Rust + Dioxus WASM)_

---

### Production Impact Highlights

- **Search 2.0 (Flask → FastAPI → Rust/Axum):** Introduced hexagonal architecture, cut compute costs **28×** and memory usage **190×**, halved P50 latency (**700 ms → 350 ms**), reduced max latency by **>1 s**, and cut non-peak GKE CPU/RAM spend by **83%** (**~US$100K** projected annualised savings).
- **Low-Latency Ranking & GPU Stability:** Migrated Python LTR inference to **ONNX** (**2 ms P50 / 4 ms P95**, 3× faster), resolved GPU-hosted ESCI latency spikes (**5 s → <200 ms**) via async write-back caching, and compressed LTR retraining from **24 hours → <15 minutes** across 5 business units.
- **Agentic AI Observability & Eval:** Established org-wide **Google Cloud Trace** + structured logging for AskBlue agentic I/O, built **LLM-as-a-judge** regression evals & API comparators (**1–2 days → <1 hr** product validation), and built Gemma vs. Gemini comparison dashboards.
- **Infrastructure & Developer Velocity:** Drove a 3-gen **C4A** upgrade (**−30%** latency, **−20%** pods), cut Python Docker builds **5×** (**5–6 min → ~1 min** via `slim-trixie` base/app splits), and migrated Streamlit tooling to **Rust Dioxus** (**27×** lower compute).

---

### Core Stack

| Layer | Technologies |
|---|---|
| **Search & ML** | Learning to Rank (LTR), ONNX, ESCI, NER, XGBoost, Model Fine-Tuning, Vertex AI, AWS AutoML |
| **Applied AI & Eval** | Gemini, Fine-tuned Gemma, LLM-as-a-Judge, Google ADK, A/B Testing, Performance Testing |
| **Languages & Serving** | **Rust**, **Python**, **Axum**, **FastAPI**, **Dioxus**, Flask, Streamlit, Bash, LaTeX |
| **Cloud & MLOps** | Google Cloud Platform (GCP), Kubernetes (GKE / HPA / CronJobs), Terraform, Helm, GitLab, K9s |
| **Observability** | Google Cloud Trace, Cloud Logging, Datadog, Loki |
| **Data & Analytics** | Polars, BigQuery, Pandas, NumPy, MongoDB, Data Studio |

---

### Career Journey

- **Best Buy** — *Senior AI Engineer, AskBlue & Search 2.0* _(May 2025 – Present · Bengaluru)_
  Search 2.0 Rust/Axum migration, ONNX LTR, GPU ESCI stabilization, C4A & HPA cost optimization, Cloud Trace agentic observability, LLM-as-a-judge evals, Streamlit → Dioxus.
- **[Falabella](https://en.wikipedia.org/wiki/Falabella_(retail_store))** — *Senior Machine Learning Engineer* _(Apr 2023 – Apr 2025 · Bengaluru)_
  Flask → async FastAPI (−40% timeouts), 24 h → <15 min LTR retraining (−20%+ cloud spend), related-keywords search (+7% engagement), Pandas → Polars, in-house cookie A/B testing ($50K/yr saved).
- **[Accenture AI](https://www.linkedin.com/company/accentureai/)** — *Data Science Consultant* _(Dec 2020 – Apr 2023 · Bengaluru)_
  GCP proposals ($2M contracts), Vertex AI 3G telecom decommissioning (150+ London towers), Terraform GCP automation (−50% manual setup).
- **[Kaleidofin](https://kaleidofin.com/)** — *Data Science Analyst* _(Jun 2019 – Dec 2020 · Chennai · Employee #70, Seed)_
  Portfolio analysis framework (−40% analysis time); mutual fund risk assessment avoiding Franklin MF ahead of its $3.4B collapse.
- **[FundsIndia](https://www.fundsindia.com/)** — *Data Analyst* _(May 2018 – Jun 2019 · Bengaluru)_
  Call center analytics (−10% drop-offs, +15% retention) and mutual fund conversion-funnel dashboard (+15% conversion).
- **[MuSigma](https://www.mu-sigma.com/)** — *Trainee Decision Scientist* _(Jul 2017 – Apr 2018 · Bengaluru)_
  Competitive pricing dashboards for Pfizer (~$1.2M/yr estimated revenue) and automated executive financial pipelines. _(Ask me about the 3-year bond over coffee.)_

---

### Education & Verified Certifications

- **Georgia Institute of Technology** — M.S. in Analytics *(Computational Track)*
- **BMS Institute of Technology** — B.E. in Electronics & Communication
- [GCP Professional Data Engineering Certification](https://www.credential.net/5a158b5b-5b95-4e79-868a-2cf6a4959512?key=c4a40ba808cb312664a87fb1f5ae292c984bcd3eafd3db40d07c5427e4b3752f)
- [Deep Learning Specialization — Coursera](https://www.coursera.org/account/accomplishments/specialization/GLDSULWYE2H4)
- [Machine Learning (Stanford Online — Coursera)](https://www.coursera.org/account/accomplishments/verify/4P6GMFBUTRSD)

---

### About This Repository

This repo doubles as my GitHub profile README and the source code for **[aaronjoseph.github.io/aaronjoseph](https://aaronjoseph.github.io/aaronjoseph/)**, built in **Rust + Dioxus 0.7 (WebAssembly)** and deployed automatically via GitHub Actions (`.github/workflows/deploy.yml`).

```bash
dx serve --platform web            # Local dev server with hot reload
dx build --platform web --release  # Production WebAssembly build
```

See [AGENTS.md](./AGENTS.md) for architecture notes, design system tokens, and local toolchain details.

---

📫 Connect with me on **[Website](https://aaronjoseph.github.io/aaronjoseph/)** · **[LinkedIn](https://www.linkedin.com/in/aaron-joseph-mathew-4b425b71/)** · **[X / Twitter](https://twitter.com/AaronJosephMath)** · **aaronjosephmathew@gmail.com**
