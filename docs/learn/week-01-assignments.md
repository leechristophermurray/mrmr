# Week 1 — Coding Assignments

> **Time budget: ~5–6 hours total.** If you cross 7 hours, **stop and message me**.
> Either we mis-scoped, or you're over-engineering. Both are fixable.
> Suffering in silence is not.
>
> **Hands off the keyboard for me.** I review and instruct; you type.
> If you get stuck, paste the error or your code; I will diagnose, not
> rewrite. The borrow checker is the second-best Rust teacher you'll
> have. (The best one is the one staring at the borrow checker.)

---

## Assignment 1.1 — Stand up the MurMur workspace (~3 h)

### Goal

A GitHub repository named `mrmr` (private or public, your choice) with a
working Cargo workspace, three crate skeletons, and **CI green on the
first push to a feature branch**.

### Concrete tasks

Do them in this order. Commit after each major step on a feature branch
(don't commit directly to `main`).

1. **Create the GitHub repo `mrmr`.** Initialize empty (no README, no
   gitignore — we'll add ours). Clone locally to `~/code/mrmr` (or
   wherever).
2. **Branch:** `chore/initial-workspace`. All Week 1 work happens here.
3. **Add `rust-toolchain.toml`** at repo root pinning Rust 1.95.0 with
   the components from the concept doc. Verify by running `rustc
   --version` inside the repo dir — it should auto-install 1.95.0 on
   first invocation.
4. **Add `.gitignore`** — start with the standard Rust `.gitignore` from
   GitHub's template, then append `/target/`, ~~`Cargo.lock`~~ (NO — keep
   `Cargo.lock` for binary workspaces; this one IS binary), `.DS_Store`,
   and `*.swp`. (Decision: do you commit `Cargo.lock`? See the senior
   reflection at the bottom of this file.)
5. **Add `LICENSE-APACHE` and `LICENSE-MIT`** at repo root. Use the
   official text.
6. **Create the workspace `Cargo.toml`** at repo root with:
   - `[workspace]` section listing the three members.
   - `[workspace.package]` with shared edition/rust-version/license/repo.
   - `[workspace.dependencies]` with at least `serde`, `thiserror`, `ulid`,
     `jiff`, `tracing` declared.
   - `[workspace.lints.rust]` and `[workspace.lints.clippy]` per the
     concept doc.
7. **Create the three crates** with `cargo new --lib crates/mrmr-core`,
   `cargo new --lib crates/mrmr-app`, `cargo new crates/mrmr-cli` (the
   last is a binary). For each:
   - Make its `Cargo.toml` use workspace inheritance (`edition.workspace
     = true`, etc.).
   - Add `[lints] workspace = true`.
   - In `mrmr-app`, declare `mrmr-core = { path = "../mrmr-core" }` as a dependency.
   - In `mrmr-cli`, declare both `mrmr-core` and `mrmr-app` as path
     deps.
8. **Write one trivial item with one passing test** in each library
   crate. Example shape (do not copy verbatim — use your own naming):

   ```rust
   //! mrmr-core — pure domain types for MurMur.
   //!
   //! Licensed under either of Apache-2.0 OR MIT at your option.

   /// Returns the protocol family name. Placeholder until W2.
   #[must_use]
   pub fn protocol_family() -> &'static str { "mrmr" }

   #[cfg(test)]
   mod tests {
       use super::*;
       #[test] fn family_is_mrmr() { assert_eq!(protocol_family(), "mrmr"); }
   }
   ```

   In `mrmr-cli/src/main.rs`, write a `main` that calls into
   `mrmr_app` (which calls into `mrmr_core`) and prints a single line.
   Verify the dependency rule by feeling: try, *as a thought experiment
   only*, to make `mrmr-core` import `mrmr-app`. Don't actually do it
   — just notice that you'd have to add a path dep that creates a
   cycle, and Cargo would refuse.
9. **Set up `prek` for local pre-commit hooks.**
   - Install: `cargo install prek` (or use the shell installer per the
     concept doc).
   - Verify: `prek --version` prints 0.3.x or newer.
   - Create `prek.toml` at the repo root with the configuration shown
     in concept doc §8 (cargo-fmt, cargo-clippy, plus the standard
     pre-commit-hooks: trailing-whitespace, end-of-file-fixer,
     check-yaml, check-toml, check-added-large-files).
   - `prek install` to write the git hook shim.
   - `prek install-hooks` to pre-fetch remote hook repos.
   - **Test the hooks fire.** Make a deliberate format violation
     (extra spaces or unsorted imports) and try to commit. The hook
     must reject. Fix and commit cleanly.
   - **Test the YAML/TOML hooks too** — they catch real bugs in CI
     workflow files later, so it's good to know they're active.
10. **Add `.github/workflows/ci.yml`** with these jobs (you write the
    YAML — I'll review):
    - Job `lint`: checkout → install toolchain (use
      `dtolnay/rust-toolchain@stable` or similar — but read the
      `rust-toolchain.toml` so it picks 1.95.0) → cache `target/` with
      `Swatinem/rust-cache@v2` → `cargo fmt --check` → `cargo clippy
      --all-targets -- -D warnings`.
    - Job `test`: same setup → `cargo test --workspace`.
    - Job `dco`: use the GitHub-published DCO check action or roll your
      own that scans commits in the PR for `Signed-off-by:`.
    - Job `commitlint`: validate the PR title matches Conventional
      Commits format. (`wagoid/commitlint-github-action` or similar.)

    Trigger on `push` (any branch) and `pull_request` (against `main`).
11. **Configure DCO sign-off** in your local repo: `git config
    format.signoff true`. From now on every commit is auto-signed.
12. **Configure branch protection on `main`** (in GitHub repo settings):
    - Require PRs.
    - Require status checks: `lint`, `test`, `dco`, `commitlint`.
    - Require linear history.
    - Disallow force pushes.
    - Allow only squash-merge.
13. **Write `README.md`.** Minimum content:
    - Project name + one-sentence pitch.
    - Status: "very early — Week 1 of an 18-month build".
    - Build: `cargo test --workspace`.
    - License: dual Apache-2.0 OR MIT, with the standard footer from the
      concept doc.
    - A pointer to `docs/SPEC.md` (which you'll add in step 15).
14. **Open a PR** from `chore/initial-workspace` → `main` with the title
    `chore: initial workspace skeleton`. The PR description should
    explain *why* (mention SPEC.md goals 1, 5, 7, 11, 12 from the
    workspace section). Watch CI run. **All checks must pass.**
15. **Squash-merge yourself.** The merge commit on `main` should have a
    clean conventional-commit subject and a body that explains the
    decisions made.

### Acceptance criteria

- [ ] Repo exists, public or private.
- [ ] `rust-toolchain.toml` pins 1.95.0.
- [ ] Both `LICENSE-*` files at repo root.
- [ ] Workspace has 3 members; each declares `[lints] workspace = true`.
- [ ] `cargo test --workspace` passes locally.
- [ ] `prek.toml` at repo root; `prek install` has run; hooks fire on
      a deliberate format violation.
- [ ] CI is green on the PR.
- [ ] DCO check is green.
- [ ] Commitlint check is green.
- [ ] Branch protection rules configured per task 12.
- [ ] PR squash-merged to `main` with a conventional commit.

---

## Assignment 1.2 — Add `SPEC.md` and `00-curriculum-overview.md` (~30 min)

### Goal

The two documents I produced are now part of *your* repo, where they
belong.

### Tasks

1. New branch: `docs/initial-spec`.
2. `mkdir docs && mkdir curriculum`.
3. Copy the `SPEC.md` from this curriculum drop to `docs/SPEC.md`.
4. Copy `00-curriculum-overview.md` to `curriculum/00-curriculum-overview.md`.
5. Copy this Week 1 trio to `curriculum/week-01/`.
6. Commit on the branch with `docs(spec): initial SPEC and curriculum
   plan`.
7. Open PR. CI must pass. Squash-merge.

### Acceptance criteria

- [ ] `docs/SPEC.md` exists.
- [ ] `curriculum/00-curriculum-overview.md` exists.
- [ ] `curriculum/week-01/{week-01-concept.md,week-01-assignments.md,week-01-quiz.md}` exist.
- [ ] All committed via PR.

---

## Assignment 1.3 — Extract workspace skeleton as a kickstart template (~1 h)

### Goal

Capture the architectural shape you just built so it bootstraps the
next Rust workspace you start (and the one after that) in 30 seconds.

### Tasks

1. **Install kickstart.** `cargo install kickstart`. Verify with
   `kickstart --version` (should be 0.5.0 or newer).
2. **New branch:** `feat/kickstart-workspace-template`.
3. **Create `templates/workspace-skeleton/`** at the repo root.
4. **Write `templates/workspace-skeleton/template.toml`** based on the
   schema in concept doc §9. Required `[[variables]]`:
   - `project_name` (string, regex-validated to kebab-case)
   - `project_slug` (`derived = true`, default `{{ project_name | kebab_case }}`)
   - `binary_prefix` (string, sensible default derived from `project_name`)
   - `authors_line` (string)
   - `edition` (choices: 2024, 2021)
   - `rust_version` (string, default 1.95.0)
   - `license` (choices including "Apache-2.0 OR MIT")
   - `include_prek` (bool, default true)
   - `include_cargo_deny` (bool, default true)
   - `description` (string)

   Decide whether to add a `post_gen_hooks` entry that runs `git init`.
   Note: hooks are executable scripts inside the template that get
   templated themselves — write a small `init_git.sh` that does
   `git init && git add .` if you go this route. (You can also leave
   hooks empty for v1 and run `git init` manually after generation.)

5. **Copy the workspace files into the template directory** with Tera
   substitutions. Notes:
   - **No special extension needed.** kickstart templates files in
     place — there's no `.j2` suffix convention. The file `Cargo.toml`
     in the template dir contains Tera syntax and is rendered to
     `Cargo.toml` in the output.
   - **Directory and file names can also be templated.** Rename
     `crates/mrmr-core/` → `crates/{{ project_name }}-core/` directly.
   - In `Cargo.toml`, substitute the workspace member list, edition,
     rust-version, license, etc.
   - In `crates/{{ project_name }}-cli/Cargo.toml`, set
     `[[bin]] name = "{{ binary_prefix }}"`.
   - In `LICENSE-APACHE` and `LICENSE-MIT`, substitute the year and
     `{{ authors_line }}`.
   - In `README.md`, use `{{ project_name }}` and `{{ description }}`.
   - In `.github/workflows/ci.yml`, the file is mostly stable but
     workflow names can include the project name.
   - **Conditional inclusion**: if `include_prek = false`, add the
     `prek.toml` path to `cleanup` so it gets removed post-generation.
     Same for `deny.toml` with `include_cargo_deny`.

6. **Validate the template.** kickstart ships a built-in validator:
   ```bash
   kickstart validate templates/workspace-skeleton/template.toml
   ```
   Fix anything it complains about.

7. **Test the template end-to-end** from a scratch directory outside
   the repo:
   ```bash
   cd /tmp
   kickstart /path/to/mrmr/templates/workspace-skeleton -o ./test-proj
   # Answer the prompts (try project_name = "test-proj").
   cd test-proj
   cargo test --workspace            # must pass
   prek install
   prek run --all-files              # must pass
   ```
   Repeat with `include_prek = false` to confirm the cleanup branch
   works. If anything breaks, fix the template and re-test until it's
   clean from a cold scratch directory.

8. **Document the templates directory.** Add `templates/README.md`
   explaining:
   - What templates exist and what each is for.
   - How to use one:
     `kickstart https://github.com/<you>/mrmr -s templates/workspace-skeleton -o ./my-project`
     (and the local-path variant).
   - The extraction rhythm (we add templates as patterns mature).
   - A note that all templates use the kickstart + Tera stack to keep
     a consistent mental model with the rest of MurMur's TOML configs.

9. **Commit and PR.** Title:
   `feat(templates): extract workspace skeleton as kickstart template`.
   Body explains *why* this captures the v1.0 workspace shape and
   what future templates will join it.

### Acceptance criteria

- [ ] `templates/workspace-skeleton/template.toml` exists and passes
      `kickstart validate`.
- [ ] All workspace files are templated (no hard-coded `mrmr` strings
      that should be variable).
- [ ] Running `kickstart` from a clean directory produces a workspace
      where `cargo test --workspace` passes.
- [ ] `prek run --all-files` passes in the generated workspace (when
      `include_prek = true`).
- [ ] Generating with `include_prek = false` correctly cleans up the
      `prek.toml` file.
- [ ] `templates/README.md` documents the template and the rhythm.
- [ ] PR squash-merged with a conventional commit.

### Why this matters

You just made yourself ~3 hours faster on the next Rust workspace you
start. More importantly: **you forced yourself to articulate every
decision in the workspace as either "always this way" (hardcoded in
the template) or "varies per project" (a question)**. That
articulation *is* senior-level engineering knowledge.

By month 18 you should have 6–10 templates here. Treat them as a
deliverable.

---

## Assignment 1.4 — Read & annotate (~1 h)

Pick **one** of the reading-list items in `week-01-concept.md` §12 and
take notes — in your own words — on three things you learned that you
didn't know before. Keep these notes locally; we'll discuss in the
Week 1 reflection.

You don't have to commit these notes. But if you do, put them in
`curriculum/week-01/notes-yourname.md` so they're preserved.

---

## Senior reflection: do you commit `Cargo.lock`?

This is a real architectural decision, not trivia. Two camps:

- **Library crates** typically *don't* commit `Cargo.lock` because
  downstream consumers want to resolve their own versions.
- **Binary crates / applications / workspaces with binaries** *do*
  commit `Cargo.lock` for reproducible builds.

MurMur is a workspace that produces multiple binaries (`mrmr`,
`mrmr-leader`, `mrmr-agent`, etc.). **Commit `Cargo.lock`.** If we
ever publish individual library crates from this workspace to
crates.io (we might), the published version still works fine for
consumers — `Cargo.lock` is for the workspace's own builds, not
embedded in published crate tarballs.

Reflect: when *would* you not commit a workspace's `Cargo.lock`? (No
need to write an essay; just be able to articulate it.)

---

## When you're done

Drop the PR URL(s) in the chat. I'll review:

- The actual structure of your `Cargo.toml` files.
- Your CI workflow YAML.
- Your `prek.toml` configuration.
- Your kickstart `template.toml` schema and any tricky Tera substitutions.
- Your commit messages on the squash-merged commits.
- Your branch protection settings (paste a screenshot or describe).
- A demonstration that `kickstart` from your template produces a
  working workspace.

Then we move to Week 2 — ownership and your first `AgentId` newtype.
