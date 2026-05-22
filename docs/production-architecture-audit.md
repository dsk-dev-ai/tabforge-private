# TabForge Production Architecture Audit & Roadmap (Phase D–G)

Date: 2026-05-22  
Repository: `dsk-dev-ai/tabforge-private`

## Baseline Context Confirmed

Current enforced runtime chain (preserved):

`Popup -> Background -> chrome.tabCapture -> Capture Runtime -> WebSocket Runtime -> Rust Session Runtime -> Worker Pool -> FFmpeg Session Ownership -> recordings output`

The current repository already reflects core backend ownership boundaries:
- Extension background service manages capture session lifecycle and script injection.
- Rust runtime bootstraps capture/encoder/workers/socket services.
- Worker pool owns session assignment and usage metrics.

---

## PART 1 — Project Audit

### 1) Missing Components (Production-Critical)

1. **Auth + identity layer not wired in desktop runtime**
   - No profile/account state machine in current desktop UI skeleton.
   - Missing token persistence, refresh handling, and workspace binding.

2. **Billing/entitlement enforcement absent**
   - No plan-gating checks in capture start pipeline.
   - Missing seat/license enforcement for team usage.

3. **Session metadata persistence layer incomplete**
   - Worker/session metrics are memory-only at runtime level.
   - Needs durable index (SQLite/Postgres sync) for history, analytics, and AI summaries.

4. **Observability stack incomplete**
   - Console logging exists, but no structured telemetry pipeline.
   - Missing error taxonomy, trace IDs, release/version tagging.

5. **Recovery orchestration incomplete**
   - Needs deterministic resume policy after desktop crash/browser restart.
   - Needs orphaned FFmpeg process reconciliation on app relaunch.

6. **Export/publishing subsystem absent in UI+runtime contract**
   - No explicit export queue manager (codec presets, retries, destination providers).

7. **AI post-processing contract missing**
   - No stable event bus from session completion -> transcript -> summary -> chapterization -> embeddings.

### 2) Weak Points

1. **Current desktop app UI remains scaffold-level**
   - Tauri starter view still present; no production navigation shell.

2. **Potential protocol fragility between extension and Rust socket runtime**
   - Requires explicit versioned message schema + compatibility checks.

3. **Runtime lifecycle coordination risk**
   - Multiple async monitors run, but policy-level lifecycle states (booting/ready/degraded/recovering) are not explicit.

4. **Multi-session contention edge cases**
   - Worker assignment exists, but needs policy for overload/backpressure and fair scheduling under spikes.

### 3) Technical Debt

1. **Formatting/readability debt in runtime modules**
   - Several Rust files use non-idiomatic spacing/line wrapping, increasing maintenance cost.

2. **State handling debt in extension runtime**
   - Global mutable state map/set pattern is efficient but needs typed contract and stricter cleanup guards.

3. **Testing debt**
   - No visible comprehensive integration/e2e matrix for long sessions, browser reload, crash recovery.

4. **Config debt**
   - Runtime constraints (FPS/resolution/worker counts) appear hardcoded; need environment/profile-driven config.

### 4) Product Blockers

- No polished desktop IA/navigation for daily production use.
- No session library UX (search, tags, filters, status, corruption markers).
- No confidence indicators for recording health (dropped frames, audio drift, encoder fallback).

### 5) Monetization Blockers

- No plan enforcement hooks (limits per month/session length/features).
- No in-app subscription upgrade surface.
- No team administration primitives (workspace roles, seat assignment, audit logs).

### 6) UX Blockers

- Missing first-run onboarding and permissions wizard.
- Missing "recording confidence" panel (live quality indicators + actionable warnings).
- Missing AI value surfacing immediately after recording completion.

---

## PART 2 — Production Roadmap (Phase D/E/F/G)

## Phase D — Runtime Hardening + Data Foundation

**Priority:** P0  
**Theme:** Make the existing pipeline deterministic, inspectable, and recoverable.

### Features
- Versioned WebSocket protocol (`v1` envelope with schema hash).
- Session state machine (`created -> capturing -> encoding -> finalized -> failed/recovering`).
- Durable local metadata store (SQLite) for sessions, chunks, export jobs.
- Crash-recovery coordinator for FFmpeg ownership reconciliation.
- Structured telemetry (JSON logs, trace/session IDs, severity channels).

### Dependencies
- Existing socket runtime + worker metrics.
- Tauri filesystem permissions and secure local DB path.

### Testing Requirements
- Protocol compatibility tests (extension message replay fixtures).
- Recovery tests (kill desktop app mid-session, relaunch, reconcile).
- Soak test: 2-hour multi-tab capture with telemetry assertions.

## Phase E — Product Surface + Control Plane

**Priority:** P0/P1  
**Theme:** Ship a usable premium desktop workflow.

### Features
- Full desktop navigation shell and page system.
- Live Recording Center with OBS-like controls + health panel.
- Session History with filters, status chips, retry actions.
- Export Center queue with presets (H.264/H.265, bitrate ladder).
- Settings domains: capture, encoder, storage, privacy, integrations.

### Dependencies
- Phase D session metadata APIs.
- Shared UI component system.

### Testing Requirements
- UI e2e flows for start/pause/stop/export.
- Accessibility sweep (keyboard navigation + contrast).
- IPC contract tests between frontend and Rust commands.

## Phase F — AI Intelligence + Analytics

**Priority:** P1  
**Theme:** Convert recordings into searchable, compounding value.

### Features
- Transcript ingestion pipeline.
- AI summaries: executive summary, action items, decisions, blockers.
- Timeline chapterization and semantic bookmarks.
- Analytics dashboards: recording time, session quality, focus metrics.
- Search across transcripts/summaries (local-first index with optional cloud sync).

### Dependencies
- Phase D metadata and event hooks.
- Exported audio/text artifacts.

### Testing Requirements
- Deterministic AI job orchestration tests (idempotent retries).
- Latency SLA tests for summary generation.
- Privacy mode tests (local-only AI toggle enforcement).

## Phase G — Commercialization + Team Platform

**Priority:** P1/P2  
**Theme:** Monetize safely and support organizations.

### Features
- Auth + subscription + entitlement middleware.
- Plan gating (duration limits, AI quotas, storage caps, export presets).
- Team workspace model (owner/admin/member), seat billing.
- Audit log and compliance exports.
- Enterprise toggles: SSO/SAML, retention policies, admin controls.

### Dependencies
- Stable Phase E UX and Phase F job/value surfaces.
- Billing provider integration (Stripe/Paddle).

### Testing Requirements
- Entitlement tests at every gated action boundary.
- Billing webhook replay tests.
- Team permission matrix tests.

---

## PART 3 — Modern UI Architecture

## A) Page Hierarchy (Required 10 pages)

1. **Splash**
   - boot status, service checks, auto-recovery banner.
2. **Login/Profile**
   - auth, workspace switcher, avatar/account state.
3. **Workspace Dashboard**
   - active sessions, recent recordings, system health cards.
4. **Live Recording Center**
   - tab source selection, per-session controls, meters.
5. **Session History**
   - searchable table/grid, status, tags, pinning.
6. **Analytics**
   - trends, utilization, quality metrics.
7. **AI Summaries**
   - summaries, chapters, action items, semantic search.
8. **Export Center**
   - queue, preset manager, destinations.
9. **Settings**
   - capture, encoder, storage, privacy, integrations.
10. **Subscription**
   - plan details, usage, upgrade/manage billing.

## B) Component Hierarchy

- `AppShell`
  - `TopCommandBar` (Raycast-like command palette, global search, quick actions)
  - `SidebarNav` (Linear-style slim nav with workspace context)
  - `StatusRail` (runtime/socket/worker health indicators)
  - `MainRouterOutlet`

Shared primitives:
- `GlassCard`, `MetricCard`, `AnimatedStatPill`, `GradientButton`
- `SessionStateBadge`, `HealthSignal`, `PlanLimitMeter`
- `TimelineScrubber`, `ChapterAccordion`, `ExportPresetChip`

Recording-specific:
- `LivePreviewPane`
- `TransportControls` (record/pause/stop/mark)
- `AudioMeter`, `FrameDropIndicator`, `EncoderFallbackBanner`
- `WorkerAllocationPanel`

## C) Navigation Flow

- Startup: `Splash -> Login/Profile (if unauth) -> Workspace Dashboard`
- Primary loop:
  - Dashboard -> Live Recording Center -> Session History -> AI Summaries/Export Center
- Secondary loop:
  - Any page -> Command Palette quick action (start recording / open latest / export now)
- Commercial loop:
  - Usage threshold event -> Subscription page upgrade CTA

## D) Visual Direction

- **Dark mode-first** with adaptive accent gradients.
- **Glassmorphism surfaces** for cards/panels.
- **Linear + Arc**: minimal chrome, fast contextual panels.
- **Notion simplicity**: low-noise typography and spacing.
- **OBS controls**: unmistakable transport controls and meter confidence.
- **Raycast interactions**: keyboard-first command palette and quick switching.

---

## PART 4 — Folder Architecture (Target)

```text
tabforge-private/
  browser-extension/
    manifest.json
    popup/
      popup.html
      popup.ts
      components/
    background/
      background.ts
      session-controller.ts
      protocol/
        messages.ts
    capture/
      capture.ts
      stream-bridge.ts

  desktop-app/
    src/
      app/
        AppShell.tsx
        router.tsx
      pages/
        Splash/
        LoginProfile/
        WorkspaceDashboard/
        LiveRecordingCenter/
        SessionHistory/
        Analytics/
        AISummaries/
        ExportCenter/
        Settings/
        Subscription/
      components/
        layout/
        cards/
        recording/
        ai/
        billing/
      state/
        session-store.ts
        runtime-store.ts
        entitlement-store.ts
      services/
        ipc/
        api/
        analytics/

    src-tauri/
      src/
        runtime/
          bootstrap.rs
          state_machine.rs
          recovery.rs
          telemetry.rs
        ipc/
          socket.rs
          bridge.rs
          protocol/
            envelope.rs
            v1.rs
        workers/
          pool.rs
          scheduler.rs
        encoder/
          ffmpeg.rs
          ownership.rs
        session/
          registry.rs
          persistence.rs
        analytics/
          aggregates.rs
        ai/
          orchestrator.rs
          summarizer.rs
        billing/
          entitlements.rs

  runtime/
    contracts/
      ws-protocol-v1.json
      session-state-machine.md
    fixtures/
      replay/

  ai/
    prompts/
    pipelines/
    evals/

  analytics/
    schemas/
    dashboards/

  docs/
    architecture/
    roadmap/
    runbooks/
```

---

## PART 5 — Monetization Strategy

## Plan Matrix

### Free (Creator Starter) — **$0**
- 720p max
- 30 min/session cap
- 10 sessions/month
- Basic local exports
- No team workspaces
- No advanced AI summaries

### Pro — **$19/month** (or $190/year)
- 1080p/60fps
- Unlimited session length
- Advanced export presets
- AI summaries + chapters
- Priority encoding queue
- 1 user, 3 devices

### Team — **$49/user/month** (min 3 seats)
- Everything in Pro
- Shared workspace and role controls
- Centralized session library
- Team analytics
- Shared AI knowledge index

### Enterprise — **Custom (starting ~$1,500/month)**
- SSO/SAML
- Advanced retention/compliance controls
- Audit exports
- Dedicated support/SLA
- Optional on-prem or private cloud AI routing

## Monetization Notes
- Upsell trigger: session/time/AI quota nearing limit.
- Annual discounts: 15–20% to improve retention/cash flow.
- Team trial: 14 days with seat-based conversion prompts.

---

## PART 6 — Testing Plan Before Packaging

## Test Matrix

1. **Multi-tab capture correctness**
   - 1, 2, 4, 8 parallel tabs.
   - Validate per-tab output isolation + A/V sync.

2. **Stress tests**
   - Rapid start/stop loops (100 cycles).
   - Burst tab open/close while recording.

3. **Memory/CPU profiling**
   - 60-minute and 4-hour sessions.
   - Ensure no unbounded memory growth.

4. **Browser reload/reconnect**
   - Reload extension and target tabs mid-session.
   - Validate reconnect and state reconciliation.

5. **Crash recovery**
   - Kill desktop app process and relaunch.
   - Kill browser process and relaunch.
   - Confirm orphan cleanup + resumable state markers.

6. **Long-session reliability**
   - 8-hour recording on production-like machine.
   - Validate chunk continuity and final output integrity.

7. **Windows coverage**
   - Windows 11 baseline + lower-end hardware profile.

8. **Linux coverage**
   - Ubuntu LTS primary + one secondary distro.

Acceptance gate recommendation: block packaging until all P0 test suites pass twice consecutively on clean machines.

---

## PART 7 — Packaging Roadmap

## Windows (Priority 1)
- Produce signed MSI installer.
- Add upgrade-safe config migration path.
- Add post-install dependency validation (FFmpeg/hardware capability checks).

## Linux (Priority 1)
- AppImage for universal portability.
- DEB for managed installs on Debian/Ubuntu.
- Add distro diagnostics command for support cases.

## macOS (Future Phase)
- Hardened runtime + notarization prep.
- Media capture entitlement strategy and QA matrix for Apple Silicon + Intel.

---

## PART 8 — Immediate Next Implementation Task

## Selected Task (Do Next)

**Task:** Implement Phase D foundation: **Versioned WebSocket protocol envelope + Rust-side validation + extension message alignment**.

This is the highest-leverage next step because it stabilizes every downstream surface (UI, analytics, AI, billing hooks) without changing your architecture.

### Proposed Branch Name

`feat/phase-d-ws-protocol-envelope-v1`

### Files (initial scope)

- `desktop-app/src-tauri/src/ipc/socket.rs`
- `desktop-app/src-tauri/src/ipc/bridge.rs`
- `desktop-app/src-tauri/src/runtime/bootstrap.rs`
- `browser-extension/background.js`
- `browser-extension/capture.js`
- `runtime/contracts/ws-protocol-v1.json` (new)
- `docs/architecture/session-state-machine.md` (new)

### Implementation Steps

1. Define `EnvelopeV1` schema (`version`, `type`, `sessionId`, `timestamp`, `payload`, `traceId`).
2. Add validation layer in Rust socket ingress (reject unknown version/type).
3. Add structured error responses for protocol violations.
4. Update extension emitters (`background.js`/`capture.js`) to send envelope format.
5. Add protocol fixtures and replay tests for backward/invalid messages.
6. Add telemetry counters (`protocol_rejected`, `protocol_accepted`, `schema_mismatch`).

### Test Plan for This Task

- Unit tests: Rust message parsing + validation.
- Integration tests: replay fixture suite against socket runtime.
- Manual validation:
  - start capture, stream chunks, stop capture.
  - check accepted counters increment.
- Negative tests:
  - send invalid version.
  - missing sessionId.
  - malformed payload type.

Success criteria:
- 100% messages in normal flow use `v1` envelope.
- Invalid messages are rejected without crashing runtime.
- Recording pipeline remains operational end-to-end.
