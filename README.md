# ai-dev-scaffold

Reusable AI development scaffold for bootstrapping new projects with Claude Code and OpenCode configuration.

## Philosophy

- **Skills, not agents** — All expertise lives in composable skill files. No `agents/` directory. Modern coding agents (Claude Code, OpenCode) can invoke built-in subagents with specific skill context.
- **Common + Project-specific** — 6 common skills are always present. Language and domain skills are created per-project using the `skill-creator` guide.
- **Single source of truth** — Claude Code skills are the source. OpenCode variants are generated via frontmatter transformation.
- **Update without overwrite** — `sync.sh` pulls common skill updates while protecting local customizations.

## Quick Start

```bash
# Clone the scaffold
git clone https://github.com/draco28/ai-dev-scaffold.git

# Scaffold a new project
./ai-dev-scaffold/scaffold.sh /path/to/your/project
```

The interactive setup will ask for:
1. Project name, description, version
2. AI tools (Claude Code / OpenCode / both)
3. Language stacks (rust, python, typescript, go)
4. ProjectPulse project ID (optional)

## What Gets Created

```
your-project/
├── CLAUDE.md                          # Project context (fill in sections)
├── AGENTS.md                          # OpenCode workflow guide (if OpenCode + ProjectPulse)
├── opencode.json                      # OpenCode MCP config (if OpenCode)
├── .claude/
│   ├── settings.local.json            # Permissions (auto-configured for your languages)
│   ├── learning-points.md             # Pair programming session log
│   ├── .scaffold-manifest.json        # Sync tracking
│   └── skills/
│       ├── code-review/SKILL.md       # 8-point gap analysis review
│       ├── testing-protocol/SKILL.md  # Test tier strategy
│       ├── release-process/SKILL.md   # SemVer, changelog, publishing
│       ├── feature-development/SKILL.md # 6-phase dev workflow
│       ├── pair-programming/SKILL.md  # 4 learning modes
│       ├── skill-creator/SKILL.md     # Guide for creating new skills
│       └── <language-patterns>/SKILL.md # Language-specific (from examples/)
└── .opencode/                         # (if OpenCode selected)
    ├── package.json
    └── skills/                        # Generated from Claude skills
```

## Common Skills

| Skill | Purpose |
|-------|---------|
| **code-review** | Reviews work against requirements using 8-point gap analysis. Supports ProjectPulse tickets and GitHub PRs. |
| **testing-protocol** | Test tier strategy (unit/integration/property/fuzz), coverage targets, anti-patterns. |
| **release-process** | SemVer versioning, changelog management, registry publishing, rollback procedures. |
| **feature-development** | 6-phase workflow: Design → Implement → Test → Validate → Review → Merge. |
| **pair-programming** | 4 learning modes (Guided, Collaborative, Exploratory, Watch & Learn) with educational blocks. |
| **skill-creator** | Meta-skill: guide for creating new project-specific skills. |

## Language Examples

The `examples/` directory contains language-specific skill templates:

| Language | Skills |
|----------|--------|
| **Rust** | `rust-patterns` (error handling, async, API design), `benchmark-development` (Criterion.rs) |
| **Python** | `python-patterns` (type hints, pytest, async, project structure) |
| **TypeScript** | `ts-patterns` (strict typing, vitest, error handling) |
| **Go** | `go-patterns` (error handling, concurrency, table-driven tests) |

These are copied as starting points during `scaffold.sh` — customize them for your project.

## Updating Common Skills

When common skills improve in this repo, sync them to your project:

```bash
./ai-dev-scaffold/sync.sh /path/to/your/project
```

- **Unmodified skills**: Updated automatically
- **Locally modified skills**: Skipped with a warning (shows diff command)
- **Project-specific skills**: Never touched

## Creating Project-Specific Skills

Use the `skill-creator` skill (invoke `/skill-creator` in Claude Code) or create manually:

```
.claude/skills/my-skill/
└── SKILL.md
```

See `templates/skill-template/SKILL.md.template` for the skeleton.

## Repo Structure

```
ai-dev-scaffold/
├── scaffold.sh              # Interactive setup
├── sync.sh                  # Update common skills
├── claude/                  # Source of truth for Claude Code config
│   ├── settings.local.json.template
│   └── skills/              # 6 common skills
├── opencode/                # Static OpenCode config
│   ├── package.json
│   └── opencode.json.template
├── templates/               # Project file templates
│   ├── CLAUDE.md.template
│   ├── AGENTS.md.template
│   └── skill-template/
└── examples/                # Language-specific skill examples
    ├── rust/
    ├── python/
    ├── typescript/
    └── go/
```
