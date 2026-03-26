---
name: code-review
description: >
  Reviews completed work against requirements using an 8-point gap analysis framework.
  Analyzes implementation via git history and code inspection, then reports findings.
  If ProjectPulse is configured, moves tickets to "done" (approved) or back to
  "in-progress" (with detailed gap comments). Use when reviewing completed work.
  Invoke: /code-review [ticket-number-or-PR].
allowed-tools: Read, Glob, Grep, Bash
user-invocable: true
---

# Code Review

## Activation

```
/code-review [target]
```

- With ticket number (if ProjectPulse configured): review that specific ticket
- With PR number: review that pull request
- Without argument: review all pending items

---

## Workflow

### 1. Establish Context

Read the project's `CLAUDE.md` to understand conventions, tech stack, and commands.

**If ProjectPulse is configured** (project ID exists in CLAUDE.md Resources section):
```
projectpulse_context_load({ projectId: <ID> })
```

### 2. Resolve What to Review

**If `$ARGUMENTS` contains a ticket number** (e.g., `5`, `#5`, `ticket-5`):
```
projectpulse_ticket_get({ ticketNumber: <parsed-number> })
```

**If `$ARGUMENTS` contains a PR number or URL:**
```bash
gh pr view <number> --json title,body,files,commits
```

**If no argument provided and ProjectPulse is configured:**
```
projectpulse_sprint_getCurrentPosition({ projectId: <ID> })
projectpulse_kanban_getBoard({ sprintId: <from above> })
# Collect all tickets from the "in-review" column
```

If no items to review, report that and stop.

### 3. For Each Item

#### 3a. Gather Requirements

Extract and understand:
- Title, description, acceptance criteria
- Implementation context (files to modify/create, key decisions)
- Prior review feedback or comments

#### 3b. Analyze Git History

```bash
# Commits for this item
git log --all --oneline --grep="<identifier>"

# Detailed changes
git log -p --all --grep="<identifier>"

# If no tagged commits, check recent commits on the current branch
git log -10 --oneline
git diff main...HEAD --stat
```

#### 3c. Review Modified Files

Read all files identified from:
- Implementation context (files to modify/create)
- Files shown in git diff output

Use `Grep` and `Glob` to trace related code paths when needed (e.g., check callers, interface implementations, test files).

#### 3d. Gap Analysis

Evaluate the implementation against **all 8 categories**. Only flag genuine issues — do not nitpick style when the code is correct and clear.

| # | Category | Key Checks |
|---|----------|------------|
| 1 | **Functionality** | All requirements implemented? Edge cases (empty, null, boundaries)? Integration points correct? |
| 2 | **Testing** | Tests for new code? Edge cases tested? Deterministic? Adequate coverage? |
| 3 | **Error Handling** | Errors handled gracefully? No unhandled exceptions or panics? Informative messages? |
| 4 | **Safety** | No security vulnerabilities? Input validation at boundaries? No unsafe patterns without justification? |
| 5 | **Performance** | No unnecessary allocations or N+1 queries? Hot paths optimized? Benchmarks for critical paths? |
| 6 | **API Design** | Follows project conventions? Type-safe? Good defaults? Breaking changes documented? |
| 7 | **Documentation** | Public API documented? Non-obvious logic explained? Examples where helpful? |
| 8 | **Code Quality** | No linter warnings? Readable? DRY? No dead/commented code? |

**Run verification commands** from the project's CLAUDE.md (Development Commands section):
- Build command
- Test command
- Lint command
- Format check command

#### 3e. Decision & Action

**If gaps found:**

Report a structured review with specific issues.

**If ProjectPulse is configured:**
```
projectpulse_ticket_addComment({
  ticketNumber: <number>,
  content: <structured gap report>
})

projectpulse_kanban_moveTicket({
  ticketNumber: <number>,
  status: "in-progress",
  displayOrder: 0
})
```

**If no gaps found:**

Report approval.

**If ProjectPulse is configured:**
```
projectpulse_ticket_addComment({
  ticketNumber: <number>,
  content: <approval summary>
})

projectpulse_kanban_moveTicket({
  ticketNumber: <number>,
  status: "done"
})
```

---

## Comment Format

### For Gaps Found

```markdown
## Code Review — Issues Found

### Issue: [Category] — [Specific Issue]

Description of the issue with code reference (file:line).

**Expected:** What should happen
**Actual:** What's currently happening
**Fix:** Suggested fix with code example

---

(repeat for each issue)

---

## Summary

Total gaps: [number]
Categories affected: [list]
Please address these issues and re-submit for review.
```

### For Approval

```markdown
## Code Review — Approved

All checks passed:

- Functionality: requirements implemented correctly
- Testing: adequate coverage, tests passing
- Error handling: errors handled gracefully
- Safety: no vulnerabilities found
- Performance: acceptable, no regressions
- API design: follows conventions
- Documentation: public items documented
- Code quality: clean, no warnings
```

---

## Related Skills

When reviewing, consider referencing these for specialized checks:
- Project-specific patterns skill — domain-specific review criteria
- `testing-protocol` — test quality standards

---

## User Argument

$ARGUMENTS — Optional ticket number or PR to review (e.g., `5`, `#5`, `PR-12`). If omitted, reviews all pending items.
