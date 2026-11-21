# AGENTS.md — Repository Guide & Engineering Notes

This file captures the architecture, design conventions, local toolchain workarounds, and Git/deployment workflow for **Aaron Joseph Mathew's** personal repository (`aaronjoseph/aaronjoseph`).

---

## 1. Repository Purpose

This repository serves two roles:
1. **GitHub Profile README** (`README.md`) — displayed on `https://github.com/aaronjoseph`.
2. **Personal Website & Interactive Résumé** — a **Rust + Dioxus 0.7 (WebAssembly)** static web application deployed via GitHub Pages.

---

## 2. Project Architecture

```text
.
├── Cargo.toml                       # Dioxus 0.7 ("web" feature), release profile tuned for WASM size
├── Dioxus.toml                      # App title, watcher paths, web.resource + web.resource.dev
├── README.md                        # GitHub Profile README + quick start
├── AGENTS.md                        # This guide
├── .github/workflows/deploy.yml     # CI build (dx build --release) & GitHub Pages deployment
├── assets/
│   ├── favicon.svg                  # Custom geometric "AJ" monogram icon
│   └── main.css                     # Full design system (Dark + Light theme + @media print résumé CSS)
└── src/
    ├── main.rs                      # App shell, theme state (localStorage), Portfolio/Résumé view switch, Cmd+K, Toast
    ├── data.rs                      # Single source of truth for all resume, telemetry, impact, career & cert data
    └── components/
        ├── mod.rs
        ├── navbar.rs                # Sticky navbar, theme toggle, Cmd+K button, mobile drawer
        ├── hero.rs                  # Split Hero + interactive Production Telemetry tabs (search-2.0 / ltr-onnx / askblue-eval)
        ├── impact.rs                # Asymmetric Before → After Bento Grid with proportional visual delta bars
        ├── systems.rs               # Architecture Deep-Dive Case Study cards
        ├── journey.rs               # Narrative + interactive expandable 6-role Career Timeline
        ├── stack.rs                 # Filterable Technical Stack Matrix
        ├── credentials.rs           # Education (Georgia Tech MS, BMSIT) + Verified Certification links
        ├── contact.rs               # Contact CTA card, one-click copy email, social links, Footer
        ├── resume.rs                # Dedicated Print / ATS-Ready Résumé view (compact toggle + window.print())
        └── cmd_palette.rs           # Interactive Cmd/Ctrl+K quick-jump & action palette
```

### Content & Maintenance Guidelines
- **Edit data in one place:** Update `src/data.rs` for new roles, metrics, skills, or certifications — both the main portfolio and the Printable Résumé view read from `src/data.rs`.
- **Keep facts grounded:** Never invent GitHub repositories, fabricated metrics, or unverified links. All content reflects Aaron's real work history, benchmarks, and verified credentials.
- **Keep `README.md` in sync:** When major career milestones change in `src/data.rs`, reflect them in `README.md` as well.

---

## 3. Design System

- **Aesthetic:** *Precision Systems Telemetry* — intentionally distinct from `ylnhari.github.io` (which uses dark-only navy/violet gradients).
- **Themes:** Dark mode default (`#090a0d` obsidian charcoal) + Editorial Light mode (`#f7f6f2`), persisted in `localStorage` under key `ajm-theme`.
- **Palette:**
  - Primary Accent: Rust Amber (`#f97316` dark / `#ea580c` light)
  - Signal Accent: Telemetry Emerald (`#10b981` dark / `#059669` light)
- **Typography:** `Space Grotesk` (display/headings), `Plus Jakarta Sans` (body), `IBM Plex Mono` (eyebrows, telemetry, chips, dates).
- **Print Support:** `assets/main.css` includes `@media print` rules so clicking **Print / Save PDF** in the Résumé view strips the navbar/toolbar and renders a clean white ATS-friendly document.

---

## 4. Build, Dev & Corporate Laptop Toolchain Notes

### Standard Commands
```bash
# Local dev server with hot reload
dx serve --platform web

# Type-check WASM target
cargo check --target wasm32-unknown-unknown

# Production release WebAssembly build
dx build --platform web --release
# Output: target/dx/aaronjoseph/release/web/public/
```

### Known Local Environment Details
1. **Dioxus CLI (`dx 0.7.x`) & `Dioxus.toml`:**
   - Dioxus 0.7 requires `[web.resource.dev]` alongside `[web.resource]`.
2. **Release profile (`Cargo.toml`) — do not set `strip = true`:**
   - Setting `strip = true` triggers `rust-objcopy` during `dx build --release`, which can fail on macOS Apple Silicon toolchains with `dyld: Library not loaded: @rpath/libLLVM.dylib`. Keeping `opt-level = "z"`, `lto = true`, and `codegen-units = 1` (without `strip = true`) avoids this error while keeping the WASM bundle compact.
3. **Corporate TLS Inspection (HTTPS `UnknownIssuer`):**
   - Tools that bundle their own root CA list instead of reading the macOS system trust store may fail with `UnknownIssuer` when downloading over HTTPS.
   - **Rust / Dioxus offline builds:** Once crates and `~/.dx/tools/` are cached, prefer:
     ```bash
     cargo check --target wasm32-unknown-unknown --offline
     CARGO_NET_OFFLINE=true dx build --platform web --release
     ```
   - **Node tools (if ever used):** Pass `NODE_USE_SYSTEM_CA=1` (Node `v25+`) so Node trusts the macOS system certificate store.

---

## 5. Repo-Local Personal Git & GitHub Auth Setup

Because this repository is worked on from a corporate laptop whose global Git config and macOS Keychain point to a work GitHub account (`Aaron-Mathew-A3126435_bestbuy`), **never modify global Git config**.

Instead, this repository's local `.git/config` overrides identity and credentials **for this repo only**:

```ini
[user]
	name = Aaron Joseph Mathew
	email = aaronjosephmathew@gmail.com
[credential]
	helper =
	helper = !/opt/homebrew/bin/gh auth git-credential
```

How this works:
- `[user]` is repo-local (`git config --local`), keeping global work commits untouched.
- The blank `helper =` resets Git's inherited global credential helper chain so Git does not send the corporate macOS Keychain token.
- `helper = !/opt/homebrew/bin/gh auth git-credential` delegates authentication to the GitHub CLI (`gh`), where the personal `aaronjoseph` account is logged in (`gh auth status`).
- For any standalone/temporary directory push (e.g., pushing a build directory that does not share `.git/config`), pass the credential helper flags inline:
  ```bash
  git -c credential.helper= \
      -c 'credential.helper=!/opt/homebrew/bin/gh auth git-credential' \
      push origin master
  ```

---

## 6. GitHub Pages Deployment

- Workflow: `.github/workflows/deploy.yml` (triggers on push to `master`/`main` or `workflow_dispatch`).
- Automatically detects the repository name:
  - If deployed from `aaronjoseph/aaronjoseph.github.io`, builds with root base path (`/`).
  - If deployed from `aaronjoseph/aaronjoseph`, builds with `--base-path aaronjoseph` (published at `https://aaronjoseph.github.io/aaronjoseph/`).
- Adds `.nojekyll` and copies `index.html` to `404.html` before uploading `target/dx/aaronjoseph/release/web/public` to GitHub Pages.
- **One-time repo setting:** In GitHub → **Settings → Pages**, set **Source** to **GitHub Actions**.
