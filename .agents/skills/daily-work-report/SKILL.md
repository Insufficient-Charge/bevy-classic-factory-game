---
name: daily-work-report
description: >-
  Generates a comprehensive daily project work report and AI agent handoff document summarizing recent development activities, architectural decisions, code changes, current system state, and prioritized next steps for incoming AI agents or human team members. Use this skill at the end of a work session, prior to AI agent handoffs, when wrapping up daily progress, or when requested to summarize work performed.
---

# Daily Work Report & AI Agent Handoff Skill

## Overview

The **Daily Work Report** skill equips AI agents to construct structured, high-fidelity daily progress reports specifically designed for context preservation and seamless handoffs between AI agent sessions (or human maintainers).

When an AI agent finishes a session or reaches a shift handoff, critical context—such as architectural rationale, transient verification states, failed attempts, and exact file offsets—can easily be lost. This skill standardizes the process of collecting evidence, auditing changes, snapshotting system health, and documenting actionable next steps for the incoming AI agent.

---

## When to Use This Skill

Activate this skill in any of the following scenarios:

1. **End of Shift / Daily Session**: At the conclusion of a day's development work or extended coding session.
2. **AI-to-AI Handoff**: Before transferring project execution to a new AI agent session or conversation thread.
3. **Context Window Exhaustion**: When context limits dictate ending the current agent turn and starting a fresh session.
4. **User Request**: Whenever the user asks for a "daily report", "session summary", "handoff document", or "work summary".
5. **Milestone / Goal Completion**: After finishing a major multi-task objective or feature sprint.

---

## What to Include in the Report

The handoff report must be clear, precise, and empirical. It combines human-readable summaries with machine-actionable metadata tailored for an incoming AI agent.

### Core Report Structure

| Section | Required Content | Purpose for Incoming AI Agent |
| :--- | :--- | :--- |
| **1. Executive Summary** | Date, core focus area, session status, high-level outcome summary. | Immediate situational awareness of session goal and completion state. |
| **2. Work Completed** | Categorized list of features built, bugs fixed, files modified (`file:///...`), and commits created. | Prevents duplicating work or re-implementing existing changes. |
| **3. Architecture & Design Decisions** | Table of decisions made, rationale, trade-offs, rejected alternatives, and affected APIs. | Prevents incoming agent from undoing architectural choices or breaking constraints. |
| **4. Current System Health & Verification** | Build status (`cargo check` / `npm build`), test results, failing test names, active background tasks/daemons. | Establishes the exact baseline state of the project before touching code. |
| **5. Incoming Agent Action Items** | Prioritized checklist (High/Medium/Low) with starting file paths, line ranges, and acceptance criteria. | Gives the incoming agent an immediate, unambiguous starting task list. |
| **6. Context Restoration Guide** | Primary files to read first, sanity-check commands to run, and critical gotchas/warnings. | Enables rapid context reconstruction without wasting tokens browsing files. |

---

## Step-by-Step Guide: Creating the Report

Follow these steps when creating a daily handoff report:

### Step 1: Collect Empirical Evidence & Data

Do not guess or rely on memory alone. Gather data directly from the system:

1. **Audit Git Activity**:
   - Check modified/uncommitted files: `git status`
   - Review recent commits: `git log -n 10 --oneline` or `git log --since="24 hours ago" --stat`
   - Inspect diff summary: `git diff --stat`
2. **Verify System State**:
   - Run compilation check: e.g., `cargo check` or `npm run build`
   - Run test suite: e.g., `cargo test` or `npm test`
   - Record exact pass/fail counts and failure output if any.
3. **Review Conversation & Task Logs**:
   - Identify decisions made, design trade-offs discussed, and user preferences expressed.
   - List any active background tasks, daemons, or cron schedules using `manage_task` with action `list`.

### Step 2: Format the Report Using the Template

Use the standardized Markdown template located at:
[report_template.md](./resources/report_template.md)

1. Load `resources/report_template.md`.
2. Populate every section with exact details, including full file links with line references (e.g. [`src/main.rs`](file:///path/to/src/main.rs#L40-L75)).
3. Include GitHub-style alerts (`> [!NOTE]`, `> [!WARNING]`, `> [!IMPORTANT]`) for critical gotchas or breaking changes.

### Step 3: Save the Report

Save the completed report to the project's daily report directory:
- Primary Target Path: `.agents/reports/daily/YYYY-MM-DD_agent_handoff.md`
- Alternative Target Path (if `.agents/reports/` is not used): `docs/reports/YYYY-MM-DD_daily_report.md`

Ensure parent directories are created as needed.

---

## Step-by-Step Guide: Consuming the Report (Incoming AI Agent)

When an incoming AI agent starts a new session and finds a daily handoff report:

1. **Locate & Read Report**:
   - Open the latest file in `.agents/reports/daily/` (e.g., `2026-08-09_agent_handoff.md`).
2. **Execute Initial Sanity Checks**:
   - Run the exact verification commands listed in Section 4 (e.g. `cargo check`, `cargo test`) to confirm system health matches the handoff report.
3. **Inspect High-Priority Action Items**:
   - Go to Section 5 ("Handoff Plan & Next AI Agent Action Items").
   - Read the starting point files specified in the top checklist item using `view_file`.
4. **Begin Execution**:
   - Proceed directly with the highest priority pending task, updating the report checklist status as tasks complete.

---

## Best Practices for AI-to-AI Handoffs

1. **Absolute Paths & Line Ranges**: Always use clickable markdown links with line numbers (e.g., [`systems/conveyor.rs`](file:///path/to/systems/conveyor.rs#L12-L30)). This saves the next agent from wasting tool calls searching for symbols.
2. **No Ambiguity**: State clearly whether a feature is fully completed, partially implemented, or only planned.
3. **Document Failures & Dead Ends**: If an approach was tried and failed (e.g., "Using mutex X caused a deadlock in system Y"), explicitly document it under Architectural Decisions so the next agent doesn't repeat the mistake.
4. **Verifiable Baseline**: Never state "all tests pass" without actually running the test command during report generation.
