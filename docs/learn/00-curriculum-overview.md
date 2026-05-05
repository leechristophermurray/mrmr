# MurMur — Master Curriculum Overview

> **You are here.** This is the meta-document. It plans the entire 18-month
> arc, week-by-week. Refer back to it every week. Companion to `SPEC.md`.

---

## 1. Calibration Snapshot

| Dimension | Setting |
|---|---|
| Current Rust level | Tutorials / Rustlings done; first real projects ahead |
| Embedded background | Some Arduino / RPi tinkering |
| Hours/week | 5–8 (one weekday evening + part of weekend) |
| MCU procurement | Within 1–2 months → embedded track joins month 3 |
| Realistic finish | **18 months** for v1.0 of MurMur per `SPEC.md` |
| Editor / OS | Zed 1.0 on Fedora 44, x86_64 (X1 Carbon Gen 13) |
| Hardware | 1× Pi5 16GB · 3× Pi5 8GB · 1× Pi4 8GB · Bambu A1 mini · MCUs incoming |
| Compiler pin | Rust 1.95.0 (2026-04-16) |

**The cadence rule.** If a week's coding assignment takes more than ~5 h,
**stop and message me**. Either we mis-scoped or you're over-engineering.
Either is fine; both are fixable. Suffering in silence is the only failure
mode I won't be able to coach you back from.

---

## 2. The Single-Project Bet

Unlike the original two-track plan, MurMur is **one cohesive project**.
Every hour you spend compounds on the same mental model. The trade-off:
no fallback project to retreat to when MurMur is hard. The compensation:
a deliverable at month 18 that's a real, hardened, multi-agent-capable
robotics platform with a published wire protocol — a portfolio piece a
senior Rust shop will read with interest.

**v1.0 promise.** One physical robot operating, plus the platform
infrastructure, certificate/enrollment flow, and operations runbook
needed to onboard agents 2 through N. Multi-agent communication is
*proven* in v1.0 (with at least one test second agent — Pi or VM) but
the second and third *physical* robots are v2.0+.

---

## 3. The Concept Spine

72 weeks, divided into 9 phases that mirror SPEC.md's milestones M0–M9.

```mermaid
gantt
  title MurMur Curriculum — 18 Months at 5–8 h/wk
  dateFormat  X
  axisFormat  W%s
  section M0 · Foundations (1–8)
  Workspace, toolchain, CI, git, license   :w01, 0, 1w
  Ownership & moves                        :w02, after w01, 1w
  Newtypes & validated constructors        :w03, after w02, 1w
  ADTs & exhaustive matching               :w04, after w03, 1w
  Error handling, thiserror                :w05, after w04, 1w
  Borrowing & lifetimes I                  :w06, after w05, 1w
  Generics & monomorphisation              :w07, after w06, 1w
  Trait design, ports                      :w08, after w07, 1w
  section M1 · Concurrency (9–16)
  Iterators, closures, Fn family           :w09, after w08, 1w
  Smart pointers Box/Rc/Arc                :w10, after w09, 1w
  Interior mutability                      :w11, after w10, 1w
  Send + Sync                              :w12, after w11, 1w
  Threads, scoped threads                  :w13, after w12, 1w
  Channels (mpsc, broadcast)               :w14, after w13, 1w
  Async fundamentals                       :w15, after w14, 1w
  Tokio runtime, spawn semantics           :w16, after w15, 1w
  section M2 · Networking (17–24)
  Async cancellation, structured concurrency :w17, after w16, 1w
  HTTP, axum hello world                   :w18, after w17, 1w
  Protobuf, prost, schema design           :w19, after w18, 1w
  tonic, gRPC unary RPCs                   :w20, after w19, 1w
  Streaming RPC (telemetry)                :w21, after w20, 1w
  TLS via rustls                           :w22, after w21, 1w
  mTLS + the leader CA                     :w23, after w22, 1w
  Leader↔agent over mTLS gRPC              :w24, after w23, 1w
  section M3 · Web + Embedded (25–32)
  Leptos signals & components              :w25, after w24, 1w
  Leptos SSR + islands                     :w26, after w25, 1w
  Server functions, axum integration       :w27, after w26, 1w
  Live SSE telemetry stream                :w28, after w27, 1w
  Embedded toolchain, probe-rs, defmt      :w29, after w28, 1w
  no_std fundamentals                      :w30, after w29, 1w
  embedded-hal, drivers, blinky            :w31, after w30, 1w
  Embassy basics                           :w32, after w31, 1w
  section M4 · Robotics core (33–40)
  PWM, motor control                       :w33, after w32, 1w
  PID, calibration                         :w34, after w33, 1w
  I²C, IMU, sensor fusion intro            :w35, after w34, 1w
  UART framing protocol (mrmr-mcu)         :w36, after w35, 1w
  Camera capture (nokhwa)                  :w37, after w36, 1w
  ONNX inference with tract                :w38, after w37, 1w
  ASL classifier integration               :w39, after w38, 1w
  Hysteresis, confidence thresholds        :w40, after w39, 1w
  section M5 · Production posture (41–48)
  NIST 800-82 / 213 mapping                :w41, after w40, 1w
  SOC 2 Type 2 control families            :w42, after w41, 1w
  systemd hardening, AppArmor              :w43, after w42, 1w
  Supply chain & SBOM                      :w44, after w43, 1w
  Reproducible & signed builds             :w45, after w44, 1w
  Tracing, OTel, Prometheus                :w46, after w45, 1w
  Profiling: flamegraph, criterion         :w47, after w46, 1w
  Memory profiling, dhat                   :w48, after w47, 1w
  section M6–M7 · Matter + chassis (49–56)
  Matter protocol basics                   :w49, after w48, 1w
  rs-matter, OnOff cluster                 :w50, after w49, 1w
  OccupancySensing + bridge                :w51, after w50, 1w
  Chassis CAD + Bambu print iterations     :w52, after w51, 1w
  Robot 1 assembly                         :w53, after w52, 1w
  Robot 1 integration testing              :w54, after w53, 1w
  Multi-agent test (Pi or VM)              :w55, after w54, 1w
  OPERATIONS.md drafting                   :w56, after w55, 1w
  section M8 · Hardening & docs (57–64)
  Threat modelling, STRIDE                 :w57, after w56, 1w
  Pen-test against own system              :w58, after w57, 1w
  Conformance test suite                   :w59, after w58, 1w
  CONTRIBUTING, governance, RFCs           :w60, after w59, 1w
  Doc site (mdbook)                        :w61, after w60, 1w
  Cross-compilation pipeline               :w62, after w61, 1w
  Release dry-run                          :w63, after w62, 1w
  Bug-bash week                            :w64, after w63, 1w
  section M9 · v1.0 ship (65–72)
  Bug fix sprint                           :w65, after w64, 4w
  OPERATIONS.md validation                 :w69, after w65, 2w
  Release v1.0                             :w71, after w69, 1w
  Retrospective + v1.1 plan                :w72, after w71, 1w
```

---

## 4. How Each Week Is Structured

Every week ships as four files in `curriculum/week-NN/`:

1. **`week-NN-concept.md`** — read first. The concept explainer with
   diagrams, anti-patterns, and senior-level "why behind the why".
2. **`week-NN-quiz.md`** — short-answer + code-reading. Mixes new
   material with reinforcement of older concepts.
3. **`week-NN-assignments.md`** — concrete coding tasks tied to MurMur
   crates. Sized to fit your hours.
4. **`week-NN-reflection.md`** — one open-ended engineering judgment
   prompt. Optional but high-value.

I keep a running ledger (`concept-ledger.md`) of every concept covered.
It's the bag I draw from for reinforcement quizzes.

---

## 5. Mapping to Your Stated Goals

The receipt. Every goal you named has at least three weeks where it
lives.

| Goal | Weeks |
|---|---|
| Memory model, ownership, lifetimes, drop semantics | W2, W6, W10, W11, W30 (no_std), W47–W48 |
| Systems programming, syscalls, fd lifetimes | W13, W17, W22, W43 |
| Linux internals (cgroups, namespaces, seccomp) | W43, W57 |
| Mutex, RwLock, atomics, lock-free thinking | W11, W12, W14 |
| Code smells & anti-patterns | every week — embedded in concept doc + code review |
| Security engineering in Rust | W22, W23, W41–W45, W57, W58 |
| Tree Architecture | W1 (intro), W2, W6–W8, every refactor week |
| Basics → senior Rust fluency | the entire curriculum |
| Architect projects from scratch | W1, every architectural decision week |
| Production-ready, resilient | W17 (cancellation), W43–W45, W47, W57, W58 |
| Compliance: HIPAA, NIST, ITSP, GDPR, SOC 2 Type 2 | W41–W42 (NIST/ITSP/SOC2), with HIPAA as a deliberate non-goal (see SPEC §11) |
| Cross-platform Rust | W30 (no_std/cross), W62 (cross-compile pipeline), W71 (release) |
| Local vs cloud | W18, W24, W42 |
| Parallelism & concurrency | W12–W17 (full arc) |
| Security hardening | W41–W45, W57, W58 |
| DevOps (CI/CD/CD) | W1 (CI from day one), W44 (supply chain), W45 (signed releases), W62 (CD), W71 (release) |
| Git best practices | W1 (commit format, branching), reinforced in every PR |

---

## 6. The Reinforcement Schedule

Concepts decay. Every quiz pulls from this distribution:

- **70% new** — current week's concept.
- **20% medium-term** — concepts from 2–4 weeks ago.
- **10% long-term** — concepts from 8+ weeks ago.

By W30 you'll re-encounter ownership in `no_std` (the only way to
*really* understand `Drop`'s deterministic discipline). By W45 you'll
re-encounter trait design when modelling TLS handshakes.

Repetition with variation is how senior fluency is built. There is no
shortcut.

---

## 7. Compliance Reality Check

You named HIPAA, NIST, ITSP, GDPR, SOC 2 Type 2. Honest assessment:

**What MurMur exercises directly:**

- **NIST 800-82r3** (ICS/OT Security) — MurMur *is* a small ICS.
  Direct fit. (W41)
- **NIST 800-213 / 213A** (IoT Device Cybersecurity) — every Pi and
  MCU is an IoT device. Direct fit. (W41)
- **NIST 800-53 moderate** subset — engine repo's CI/CD. (W41–W42)
- **SOC 2 Type 2** control families (CC1–CC9) — design + tabletop
  exercise, not a real audit. (W42)
- **CIS Linux Benchmarks** for the Pis. (W43)
- **GDPR** — only relevant if you opt-in to telemetry export. We'll
  design Article 5 / Article 32 controls into the export path. (W46)
- **ITSP.40.111** (Canadian Centre for Cyber Security network
  protection) — applies to Mycelium-style internal networks. (W41)
- **ITSP.30.031** (user authentication) — applies to operator login.
  (W42)

**HIPAA is a non-goal.** No PHI in MurMur. If still wanted, we'd add
a separate small capstone (~6 weeks) around month 13–14 — a synthetic
PHI ingestion service. Tell me when we get to W41 if you want to slot
that in.

---

## 8. The Tree Architecture Throughline

You'll learn it three times, deeper each time:

1. **W1 (verbal intro):** the workspace itself *is* the architecture.
   You feel this when Cargo refuses to compile a circular dependency.
2. **W2–W7 (green layer):** you live in `mrmr-core`. Pure types, no
   I/O. You feel why this layer is sacred.
3. **W8 (blue layer):** you define the first ports — traits like
   `AgentRegistry`, `MissionStore`. You feel dependency inversion in
   your bones.
4. **W23–W24 (red layer):** you build the first real adapters
   (`mrmr-grpc`, `mrmr-store`). You feel infrastructure plugging in.
5. **W30 (typestate):** you encode mission lifecycles as compile-time
   state machines. You realize Tree Architecture is just *taking that
   idea up one altitude*.

---

## 9. Habits to Build From Week 1

These compound. Start them on day one. Non-negotiable.

1. **Conventional Commits.** Every commit. CI rejects malformed.
2. **Branch per change.** No commits on `main`. PR template even when
   you're the only reviewer.
3. **`cargo fmt` + `cargo clippy` before commit.** Pre-commit hook.
4. **Doctests.** Every `pub` item gets a `///` block with at least one
   example.
5. **`tracing` from day one.** Not `println!`. Even when overkill.
6. **Read the source.** When a crate surprises you, open its source.
   Highest-leverage habit you can form.
7. **Commit messages explain the why.** `feat(grpc): backpressure on
   Dispatch stream to prevent slow-agent queue blow-up` not
   `feat: add backpressure`.
8. **Write `docs/` as you build.** Not at the end. Especially
   `OPERATIONS.md` — it grows with the system.
9. **Extract templates when patterns prove themselves.** After each
   phase ships a new architectural pattern (workspace skeleton, domain
   crate, port-and-adapter pair, gRPC service, etc.), we extract it
   into `templates/` as a kickstart template. Rule of thumb: the *first*
   time you build a thing, build it by hand and feel the design. The
   *second* time you'd need to build the same shape, extract the
   first into a template before doing the second. By month 18 you
   should have a small library of kickstart templates that bootstrap
   senior-quality Rust projects in one command.

---

## 10. What I Will Not Do

- **I will not write your production code.** I review, instruct,
  diagram. You type. This is how fluency forms.
- **I will not skip ahead** when you ask me to (and you will).
- **I will not pretend you've understood something** when the code or
  quiz says you haven't. We slow down and dig.
- **I will not let "compiles + tests pass" be the bar.** The bar is
  *you can defend every design choice to a senior reviewer*.

---

## 11. What's Next

You're about to read `week-01-concept.md`. The Week 1 deliverable is the
MurMur workspace skeleton with three crates (not all sixteen — we'll add
the rest as we need them), CI green from the first push, dual licenses
in place, and conventional commits set up.

After Week 1's assignments are reviewed, we move to Week 2: ownership.

Ready when you are.
