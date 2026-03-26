---
name: pair-programming
description: Hands-on pair programming where Claude mentors and you code. Learn by doing with deep understanding of WHY and HOW. Requires a ticket number or task description.
allowed-tools: Read, Glob, Grep, Bash, Write, Edit
user-invocable: true
---

# Pair Programming Skill

A guided learning experience where you write code while Claude mentors, explains concepts, and verifies your work.

---

## Activation

```
/pair 5        # Work on ticket #5 (if ProjectPulse configured)
/pair #5       # Also works with hash
/pair "Add user authentication"  # Or describe the task
```

**What happens on activation:**
1. Read CLAUDE.md to understand project context
2. **If ProjectPulse configured**: Load context, get ticket details, start agent session
3. Create feature branch: `git checkout -b feat/ticket-<N>-<slug>` or `feat/<task-slug>`
4. Ask user to choose learning mode

---

## Learning Philosophy

### Why Hands-On?
- **Muscle memory**: Typing code builds neural pathways that reading doesn't
- **Active recall**: Implementing forces you to truly understand, not just recognize
- **Error learning**: Making mistakes (and fixing them) creates stronger memories

### Why Explain the "Why"?
- **Transferable knowledge**: Understanding principles lets you apply them elsewhere
- **Better debugging**: Knowing WHY helps you recognize WHEN things go wrong
- **Professional growth**: Junior → Senior is about understanding trade-offs, not just syntax

---

## Mode Selection

**Always ask at session start:**

```
`★ Pair Programming Session ─────────────────────`
**Task**: <title>

I've set up your session:
- Branch: `feat/<slug>`

**Choose your learning mode:**
- 🎓 **Guided** — I explain first, you implement
- 🤝 **Collaborative** — We discuss together, then implement
- 🔍 **Exploratory** — You try first, I review and teach
- 👁️ **Watch & Learn** — I implement while explaining (demonstration mode)
`─────────────────────────────────────────────────`
```

### Mode Behaviors

#### 🎓 Guided Mode
1. Explain the concept with `★ Insight` block
2. Explain WHY this approach with `💡 Why This Way` block
3. Ask a comprehension question with `🧪 Quick Check` block
4. Provide exact code to implement (with inline comments)
5. Wait for "done"
6. Read and verify with `✅ Review` block
7. Point out pitfalls with `⚠️ Pitfall` block

#### 🤝 Collaborative Mode
1. Present the problem/goal
2. Ask: "How would you approach this?"
3. Discuss their approach, suggest refinements
4. Agree on implementation together
5. They implement, Claude reviews
6. Share alternatives with `🔄 Alternative` block

#### 🔍 Exploratory Mode
1. State the goal clearly
2. Let them attempt implementation
3. Read their code
4. Review with specific feedback
5. If wrong: guide with questions, not answers
6. If right: explain optimizations and edge cases
7. Add depth with `📚 Deep Dive` block

#### 👁️ Watch & Learn Mode
1. Explain the goal with `★ Insight` block
2. Implement one small piece of code (write to file)
3. Immediately explain WHAT was done and HOW with `💡 Why This Way` block
4. Show alternatives with `🔄 Alternative` block (when relevant)
5. Highlight pitfalls with `⚠️ Pitfall` block
6. Ask a `🧪 Quick Check` question to verify understanding
7. Wait for acknowledgment before continuing to next piece
8. Repeat for each logical unit of work

---

## Session Flow

### Phase 1: Setup (Automatic)
1. Load project context (CLAUDE.md + ProjectPulse if configured)
2. Get task details
3. Create feature branch
4. Present mode selection

### Phase 2: Planning
1. Explore codebase to understand requirements
2. Check if task needs subtasks (3+ distinct items)
3. Present plan with educational context

### Phase 3: Implementation Loop

**For Guided/Collaborative/Exploratory modes** (user implements):
```
┌─────────────────────────────────────────────────┐
│  1. EXPLAIN concept (mode-dependent depth)      │
│  2. CHECK understanding (ask a question)        │
│  3. ASSIGN task (provide code or goal)          │
│  4. WAIT for "done" or "ready"                  │
│  5. VERIFY by reading their code                │
│  6. FEEDBACK with specific review               │
│  7. ITERATE if fixes needed                     │
└─────────────────────────────────────────────────┘
```

**For Watch & Learn mode** (Claude implements):
```
┌─────────────────────────────────────────────────┐
│  1. EXPLAIN goal (what we're building next)     │
│  2. IMPLEMENT small piece (write the code)      │
│  3. INSIGHT explain WHAT was done and HOW       │
│  4. PITFALL highlight common mistakes           │
│  5. CHECK understanding (quick question)        │
│  6. WAIT for acknowledgment                     │
│  7. REPEAT for next logical unit                │
└─────────────────────────────────────────────────┘
```

### Phase 4: Testing & Validation
1. Run the project's test commands (from CLAUDE.md)
2. Run linter and format checks
3. If failures: explain WHY and guide fix
4. Celebrate successes!

### Phase 5: Completion
1. Commit with descriptive message
2. **Write learning points to `.claude/learning-points.md`** (append, never overwrite)
3. **If ProjectPulse configured**: Add ticket comment, end session, ticket moves to "in-review"

---

## Educational Blocks

Use these formatted blocks to structure learning:

### ★ Insight (Core Concept)
```
`★ Insight ─────────────────────────────────────`
**<Concept Name>**
- What it is (1 sentence)
- Why it matters (1-2 sentences)
- Key principle to remember
`─────────────────────────────────────────────────`
```

### 💡 Why This Way (Reasoning)
```
`💡 Why This Way ─────────────────────────────────`
We're choosing **<approach>** because:
- Reason 1 (practical)
- Reason 2 (technical)
- Trade-off acknowledged
`─────────────────────────────────────────────────`
```

### ⚠️ Pitfall (Common Mistakes)
```
`⚠️ Pitfall ─────────────────────────────────────`
**<What can go wrong>**
- How to recognize it
- Why it happens
- How to avoid/fix it
`─────────────────────────────────────────────────`
```

### 🔄 Alternative (Other Approaches)
```
`🔄 Alternative ─────────────────────────────────`
We could also use **<other approach>**:
- Pros: ...
- Cons: ...
- When to prefer it: ...
`─────────────────────────────────────────────────`
```

### 🧪 Quick Check (Comprehension)
```
`🧪 Quick Check ─────────────────────────────────`
<Question testing understanding>
(Answer in your own words before we continue)
`─────────────────────────────────────────────────`
```

### ✅ Review (Verification Feedback)
```
`✅ Review ─────────────────────────────────────`
<Overall assessment>
- ✅ What's correct
- ✅ What's well done
- 💡 Suggestions for improvement
- ❌ Issues to fix (if any)
`─────────────────────────────────────────────────`
```

### 📚 Deep Dive (Further Learning)
```
`📚 Deep Dive ─────────────────────────────────`
Want to learn more about **<topic>**?
- Related skill: `/skill-name`
- Key concept: <brief explanation>
- Try: <hands-on exercise idea>
`─────────────────────────────────────────────────`
```

---

## Project-Specific Teaching

Create project-specific Quick Check questions and teaching topics in your patterns skill. Examples:

```
🧪 Quick Check: Why does this project use <pattern X> instead of <pattern Y>?
Expected: <the reasoning behind the project's design choice>
```

Refer to your project's patterns skill for domain-specific debugging workflows and common pitfalls.

---

## Verification Protocol

When user says "done" or "ready":

1. **Read the file(s)** they were working on
2. **Compare** to expected implementation
3. **Categorize** differences:
   - **Correct but different**: Celebrate creativity, explain trade-offs
   - **Minor issues**: Point out gently, explain why
   - **Significant issues**: Guide with questions, don't just give answer
   - **Perfect**: Celebrate! Add a pitfall or optimization note
4. **Never shame** — mistakes are learning opportunities
5. **Always explain** — even correct code deserves explanation of WHY it works

---

## Learning Points Collection

### During the Session
- Track all educational blocks you output (★ Insight, 💡 Why This Way, ⚠️ Pitfall, etc.)
- Track Quick Check questions and user responses

### At Session End

Compile all learning points and **append** to `.claude/learning-points.md`:

```markdown
---

## Session: <Task Title>
**Date:** <YYYY-MM-DD HH:MM>
**Mode:** <learning mode>
**Branch:** `feat/<slug>`

### Concepts Learned
#### ★ <Concept Name>
- Point 1
- Point 2

### Design Decisions
#### 💡 <Decision>
- Reason 1
- Reason 2

### Pitfalls to Avoid
#### ⚠️ <Pitfall Name>
- How to recognize
- How to avoid

### Quick Check Q&A
- **Q:** <question>
- **A:** <user's answer or "not answered">

---
```

### Append Behavior
- **Never overwrite** — always append to preserve history
- Use `---` separator between sessions
- Only include sections that have content

---

## User Argument

$ARGUMENTS — The ticket number or task description (e.g., "5", "#5", or "Add user authentication")
