# Week 1 — Quiz

> **How to use this.** Answer in your own words, in a markdown file
> (`curriculum/week-01/quiz-answers-yourname.md` or just paste in chat).
> Don't peek at the concept doc until you've taken your shot. The
> friction *is* the learning.
>
> Week 1 has no reinforcement section because there are no prior weeks.
> Future quizzes will pull 20–30% from earlier material.

---

## Part A — Concept recall (10 questions, 1–2 sentences each)

**A1.** What is the difference between a Cargo *workspace* and a single
crate? Give one architectural reason workspaces matter beyond
"compilation speed".

**A2.** Why does `mrmr-app` have a `path = "../mrmr-core"` dependency in
its `Cargo.toml`, but `mrmr-core` has *no* dependency on `mrmr-app`?
What property of the system does this enforce, and at what time
(compile, link, run)?

**A3.** Why pin the toolchain in `rust-toolchain.toml` instead of
letting `rustup` use whatever's installed? Give one concrete failure
mode that pinning prevents.

**A4.** Why is `unsafe_code = "deny"` set at the workspace level rather
than per-crate? When *would* a crate legitimately opt out, and how does
it do so?

**A5.** Conventional Commits format is `<type>(<scope>): <subject>`.
What goes in `<scope>` for MurMur, and why?

**A6.** What's the practical difference between a *git pre-commit
hook* (via `prek`) and a *CI status check*? Why do we have both?

**A7.** Apache-2.0 OR MIT — what does the `OR` mean for a downstream
*consumer* of MurMur? Pick one license to operate under, both, neither,
or it's their choice?

**A8.** What is the **Developer Certificate of Origin** (DCO), and how
does a contributor attest to it on every commit? Why might a project
prefer DCO over a CLA?

**A9.** `prek` is described as a "drop-in replacement for `pre-commit`."
Name three concrete advantages `prek` has over the original Python
`pre-commit` for a Rust project.

**A10.** `kickstart` lets you scaffold projects from templates.
Articulate the *senior engineering reason* we extract templates after
building a pattern by hand once, rather than (a) before, or (b) never.

---

## Part B — Code reading (4 questions)

**B1.** Look at this `Cargo.toml` snippet. What's wrong with it for a
workspace consumer crate?

```toml
[package]
name = "mrmr-app"
version = "0.1.0"
edition = "2024"
rust-version = "1.95"
license = "Apache-2.0 OR MIT"

[dependencies]
serde = "1"
mrmr-core = { path = "../mrmr-core" }
```

(Hint: think about what we said in §2 of the concept doc about
diamond dependencies.)

**B2.** Predict what happens when this hypothetical commit is pushed.
Will CI accept it, reject it, or partially accept it? Explain why.

```
commit message: "fix issues"
diff: cargo fmt complaint fixed in mrmr-core/src/lib.rs;
      cargo clippy complaint fixed in mrmr-app/src/lib.rs;
      one new clippy warning introduced in mrmr-cli/src/main.rs
sign-off: missing
```

**B3.** Below is a slice of a `prek.toml`. Identify the bug — there's a
real one, not a stylistic preference. Why is it a bug?

```toml
[[repos]]
repo = "local"
[[repos.hooks]]
id          = "cargo-clippy"
name        = "cargo clippy"
language    = "system"
entry       = "cargo clippy --all-targets -- -D warnings"
files       = '\.rs$'
```

**B4.** Look at this `template.toml` slice. Two things will go wrong
when someone runs `kickstart` and answers `project_name = "Cool Robot"`.
Identify both, and propose fixes. (Hint: one is about kickstart
behavior, one is about the regex.)

```toml
[[variables]]
name    = "project_name"
default = "my-project"
prompt  = "Project name"

[[variables]]
name    = "binary_prefix"
default = "{{ project_name }}"
prompt  = "Binary prefix"
```

---

## Part C — Senior judgment (3 questions, no single right answer)

**C1.** You're reviewing a teammate's PR. They've added a new crate to
the workspace. Their `Cargo.toml` declares `serde = "1.0.215"` directly
instead of `serde.workspace = true`. The build works. CI passes.
What's your review comment, and *what are you protecting against by
making it*?

**C2.** A junior contributor argues: "Why do we need both prek hooks
*and* CI? Hooks catch everything; CI is duplication." Steelman their
case in one paragraph. Then give your senior-level response in another.

**C3.** Pretend it's Week 30 of MurMur. You've built three port-and-
adapter pairs (e.g., `MissionStore` ↔ `redb` adapter; `Mailer` ↔
`Mailgun` adapter; `MatterBridge` ↔ `rs-matter` adapter). A teammate
says: "We should extract a kickstart template for new port-and-adapter
pairs." Name two arguments *for* and one argument *against* extracting
the template *now* vs. waiting. Then say which side you'd land on and
why.

---

## Submission

Reply in chat with:

1. Your answers (paste markdown, link to the file in your repo, or both).
2. A link to the merged PR(s) from the assignments.
3. Anything that confused you. **The questions you couldn't answer
   tell me more than the ones you nailed.**

I'll respond with feedback. Whatever you got wrong, we'll dig into in
the Week 2 prelude before we start ownership. Foundations matter; we
don't paper over.
