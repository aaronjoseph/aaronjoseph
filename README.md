### Hi there, I'm Aaron Joseph Mathew 👋

I'm a **Senior AI Engineer** with **9+ years** across analytics, machine learning, and AI engineering — focused on production search, ranking, LLM evaluation, and reliable low-latency ML systems in **Python** and **Rust**.

Currently at **Best Buy** (Bengaluru), working on **AskBlue** and **Search 2.0** — building search and agentic AI systems serving up to **12,000 requests per second**.

---

### Selected Highlights

- **Search 2.0 (Flask → FastAPI → Rust/Axum):** Introduced hexagonal architecture, cut compute costs **28×** and memory usage **190×**, halved P50 latency (**700 ms → 350 ms**), and cut non-peak GKE spend by **83%** (~US$100K projected annualised savings).
- **Ranking & Inference:** Migrated Python LTR inference to **ONNX** (**2 ms P50 / 4 ms P95**, 3× faster), resolved GPU ESCI spikes (**5 s → <200 ms**), and revamped LTR retraining (**24 h → <15 min**).
- **Agentic AI Observability & Eval:** Established **Google Cloud Trace** + structured logging for agentic I/O, built **LLM-as-a-judge** regression suites (**1–2 days → <1 hr** validation), and migrated Streamlit apps to **Dioxus** (**27×** lower compute).

---

### Career Journey

- **Senior AI Engineer — AskBlue & Search 2.0 | Best Buy** _(May 2025 – Present · Bengaluru)_
- **Senior Machine Learning Engineer | [Falabella](https://en.wikipedia.org/wiki/Falabella_(retail_store))** _(Apr 2023 – Apr 2025 · Bengaluru)_
- **Data Science Consultant | [Accenture AI](https://www.linkedin.com/company/accentureai/)** _(Dec 2020 – Apr 2023 · Bengaluru)_
- **Data Science Analyst | [Kaleidofin](https://kaleidofin.com/)** _(Jun 2019 – Dec 2020 · Chennai — Employee #70, Seed stage)_
- **Data Analyst | [FundsIndia](https://www.fundsindia.com/)** _(May 2018 – Jun 2019 · Bengaluru)_
- **Trainee Decision Scientist | [MuSigma](https://www.mu-sigma.com/)** _(Jul 2017 – Apr 2018 · Bengaluru — ask me about the 3-year bond over coffee)_

---

### Education & Certifications

- **M.S. in Analytics (Computational Track)** — Georgia Institute of Technology
- **B.E. in Electronics & Communication** — BMS Institute of Technology
- [GCP Professional Data Engineering Certification](https://www.credential.net/5a158b5b-5b95-4e79-868a-2cf6a4959512?key=c4a40ba808cb312664a87fb1f5ae292c984bcd3eafd3db40d07c5427e4b3752f)
- [Deep Learning Specialization — Coursera](https://www.coursera.org/account/accomplishments/specialization/GLDSULWYE2H4)
- [Machine Learning (Stanford Online — Coursera)](https://www.coursera.org/account/accomplishments/verify/4P6GMFBUTRSD)

---

### Personal Website (Built with Rust + Dioxus)

This repository also hosts the source for my personal portfolio & printable résumé site, built in **Rust** with **Dioxus 0.7 (WebAssembly)**.

#### Local Development

```bash
# Run locally with hot reload
dx serve --platform web

# Production WebAssembly build
dx build --platform web --release
```

#### Deploying to GitHub Pages

A GitHub Actions workflow is included at `.github/workflows/deploy.yml`. Enable **Settings → Pages → Source: GitHub Actions** in either this repository (`aaronjoseph/aaronjoseph`) or a dedicated `aaronjoseph/aaronjoseph.github.io` repository — the workflow automatically sets `--base-path` accordingly.

---

📫 Find me on [LinkedIn](https://www.linkedin.com/in/aaron-joseph-mathew-4b425b71/), [X / Twitter](https://twitter.com/AaronJosephMath), or **aaronjosephmathew@gmail.com**.
