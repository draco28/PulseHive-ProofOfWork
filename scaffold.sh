#!/usr/bin/env bash
set -euo pipefail

# ─────────────────────────────────────────────────
# ai-dev-scaffold — Interactive project setup
# ─────────────────────────────────────────────────

SCAFFOLD_DIR="$(cd "$(dirname "$0")" && pwd)"
SCAFFOLD_VERSION="$(cd "$SCAFFOLD_DIR" && git rev-parse --short HEAD 2>/dev/null || echo 'unknown')"

COMMON_SKILLS=(code-review testing-protocol release-process feature-development pair-programming skill-creator)

# ── Colors ──
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

info()  { echo -e "${CYAN}→${NC} $1"; }
ok()    { echo -e "${GREEN}✓${NC} $1"; }
warn()  { echo -e "${YELLOW}!${NC} $1"; }
err()   { echo -e "${RED}✗${NC} $1"; }

# ── Parse arguments ──
TARGET="${1:-.}"
TARGET="$(cd "$TARGET" 2>/dev/null && pwd || mkdir -p "$1" && cd "$1" && pwd)"

echo -e "\n${BOLD}ai-dev-scaffold${NC} — Project Setup\n"
echo -e "Target: ${CYAN}${TARGET}${NC}\n"

# ── 1. Project info ──
read -rp "Project name: " PROJECT_NAME
read -rp "One-line description: " PROJECT_DESC
read -rp "Version [0.1.0]: " PROJECT_VERSION
PROJECT_VERSION="${PROJECT_VERSION:-0.1.0}"
read -rp "Status [Pre-development]: " PROJECT_STATUS
PROJECT_STATUS="${PROJECT_STATUS:-Pre-development}"

# ── 2. AI tools ──
echo ""
echo "Which AI tools to configure?"
echo "  1) Claude Code only"
echo "  2) OpenCode only"
echo "  3) Both"
read -rp "Choice [3]: " TOOLS_CHOICE
TOOLS_CHOICE="${TOOLS_CHOICE:-3}"

USE_CLAUDE=false
USE_OPENCODE=false
case "$TOOLS_CHOICE" in
  1) USE_CLAUDE=true ;;
  2) USE_OPENCODE=true ;;
  *) USE_CLAUDE=true; USE_OPENCODE=true ;;
esac

# ── 3. Language stacks ──
echo ""
echo "Language stacks (comma-separated: rust,python,typescript,go):"
read -rp "Languages: " LANGS_INPUT
IFS=',' read -ra LANGS <<< "$LANGS_INPUT"

# ── 4. ProjectPulse ──
echo ""
read -rp "ProjectPulse project ID (or press Enter to skip): " PP_ID

# ── Helper: generate permissions ──
generate_permissions() {
  local perms=(
    '"Bash(git *)"' '"Bash(gh *)"'
    '"Bash(ls *)"' '"Bash(find *)"' '"Bash(wc *)"' '"Bash(sort *)"'
    '"Bash(head *)"' '"Bash(tail *)"' '"Bash(cat *)"'
    '"Bash(mkdir *)"' '"Bash(rm *)"' '"Bash(cp *)"' '"Bash(mv *)"'
    '"Read"' '"Write"' '"Edit"' '"Glob"' '"Grep"' '"WebSearch"' '"WebFetch"'
    '"Skill(code-review)"' '"Skill(feature-development)"' '"Skill(pair-programming)"'
  )

  for lang in "${LANGS[@]}"; do
    lang="$(echo "$lang" | xargs)" # trim whitespace
    case "$lang" in
      rust)
        perms+=('"Bash(cargo *)"' '"Bash(rustup *)"')
        ;;
      python)
        perms+=('"Bash(python3 *)"' '"Bash(python *)"' '"Bash(pip3 *)"' '"Bash(pip *)"' '"Bash(pytest *)"' '"Bash(uv *)"' '"Bash(maturin *)"')
        ;;
      typescript|ts)
        perms+=('"Bash(npm *)"' '"Bash(npx *)"' '"Bash(pnpm *)"' '"Bash(node *)"' '"Bash(bun *)"' '"Bash(tsx *)"')
        ;;
      go)
        perms+=('"Bash(go *)"' '"Bash(golangci-lint *)"')
        ;;
    esac
  done

  if [[ -n "$PP_ID" ]]; then
    perms+=('"mcp__projectpulse__*"')
  fi

  # Build JSON array
  local json="["
  local first=true
  for p in "${perms[@]}"; do
    if $first; then first=false; else json+=","; fi
    json+=$'\n      '"$p"
  done
  json+=$'\n    ]'
  echo "$json"
}

# ── Helper: transform skill for OpenCode ──
transform_skill_for_opencode() {
  local skill_file="$1"
  local content
  content="$(cat "$skill_file")"

  # Add tools field to frontmatter if not present
  if ! echo "$content" | grep -q "^tools:"; then
    content="$(echo "$content" | sed '/^---$/,/^---$/{
      /^user-invocable:/a\
tools:\
  read: true\
  write: true\
  edit: true\
  bash: true\
  grep: true\
  glob: true
    }')"
  fi

  # Remove claude-specific fields
  content="$(echo "$content" | sed '/^allowed-tools:/d')"
  content="$(echo "$content" | sed '/^user-invocable:/d')"

  echo "$content"
}

# ── 5. Copy common skills → .claude/ ──
if $USE_CLAUDE; then
  info "Setting up Claude Code (.claude/)..."
  mkdir -p "$TARGET/.claude/skills"

  for skill in "${COMMON_SKILLS[@]}"; do
    mkdir -p "$TARGET/.claude/skills/$skill"
    cp "$SCAFFOLD_DIR/claude/skills/$skill/SKILL.md" "$TARGET/.claude/skills/$skill/SKILL.md"
    ok "  Skill: $skill"
  done

  # Copy learning-points.md
  cp "$SCAFFOLD_DIR/claude/learning-points.md" "$TARGET/.claude/learning-points.md"

  # Generate settings.local.json
  PERMS_JSON="$(generate_permissions)"
  cat > "$TARGET/.claude/settings.local.json" <<SETTINGSEOF
{
  "permissions": {
    "allow": $PERMS_JSON
  },
  "enableAllProjectMcpServers": true
}
SETTINGSEOF
  ok "  Generated settings.local.json"
fi

# ── 6. Generate OpenCode skills from Claude source ──
if $USE_OPENCODE; then
  info "Setting up OpenCode (.opencode/)..."
  mkdir -p "$TARGET/.opencode/skills"

  # Copy static OpenCode files
  cp "$SCAFFOLD_DIR/opencode/package.json" "$TARGET/.opencode/package.json"
  cp "$SCAFFOLD_DIR/opencode/.gitignore" "$TARGET/.opencode/.gitignore"

  # Generate opencode.json
  if [[ -n "$PP_ID" ]]; then
    read -rp "ProjectPulse API token: " PP_TOKEN
    sed "s/{{PROJECTPULSE_TOKEN}}/$PP_TOKEN/" "$SCAFFOLD_DIR/opencode/opencode.json.template" > "$TARGET/opencode.json"
    ok "  Generated opencode.json with ProjectPulse"
  else
    echo '{ "$schema": "https://opencode.ai/config.json" }' > "$TARGET/opencode.json"
    ok "  Generated opencode.json (no MCP)"
  fi

  # Transform and copy skills from Claude source
  for skill in "${COMMON_SKILLS[@]}"; do
    mkdir -p "$TARGET/.opencode/skills/$skill"
    transform_skill_for_opencode "$SCAFFOLD_DIR/claude/skills/$skill/SKILL.md" > "$TARGET/.opencode/skills/$skill/SKILL.md"
    ok "  Skill (OpenCode): $skill"
  done
fi

# ── 7. Copy language-specific example skills ──
for lang in "${LANGS[@]}"; do
  lang="$(echo "$lang" | xargs)"
  if [[ -d "$SCAFFOLD_DIR/examples/$lang" ]]; then
    info "Copying $lang example skills..."
    for skill_dir in "$SCAFFOLD_DIR/examples/$lang"/*/; do
      skill_name="$(basename "$skill_dir")"
      if $USE_CLAUDE; then
        mkdir -p "$TARGET/.claude/skills/$skill_name"
        cp "$skill_dir/SKILL.md" "$TARGET/.claude/skills/$skill_name/SKILL.md"
        ok "  Skill: $skill_name"
      fi
      if $USE_OPENCODE; then
        mkdir -p "$TARGET/.opencode/skills/$skill_name"
        transform_skill_for_opencode "$skill_dir/SKILL.md" > "$TARGET/.opencode/skills/$skill_name/SKILL.md"
        ok "  Skill (OpenCode): $skill_name"
      fi
    done
  else
    warn "No example skills for '$lang' — create project-specific skills with /skill-creator"
  fi
done

# ── 8. Generate CLAUDE.md ──
info "Generating CLAUDE.md..."
sed -e "s/{{PROJECT_NAME}}/$PROJECT_NAME/g" \
    -e "s/{{ONE_LINE_DESCRIPTION}}/$PROJECT_DESC/g" \
    -e "s/{{VERSION}}/$PROJECT_VERSION/g" \
    -e "s/{{STATUS}}/$PROJECT_STATUS/g" \
    "$SCAFFOLD_DIR/templates/CLAUDE.md.template" > "$TARGET/CLAUDE.md"
ok "  Generated CLAUDE.md"

# ── 9. Generate AGENTS.md (if OpenCode + ProjectPulse) ──
if $USE_OPENCODE && [[ -n "$PP_ID" ]]; then
  info "Generating AGENTS.md..."
  sed -e "s/{{PROJECT_NAME}}/$PROJECT_NAME/g" \
      -e "s/{{PROJECTPULSE_ID}}/$PP_ID/g" \
      "$SCAFFOLD_DIR/templates/AGENTS.md.template" > "$TARGET/AGENTS.md"
  ok "  Generated AGENTS.md"
fi

# ── 10. Create manifest ──
MANIFEST_FILE="$TARGET/.claude/.scaffold-manifest.json"
SKILL_HASHES="{"
FIRST=true
for skill in "${COMMON_SKILLS[@]}"; do
  if $USE_CLAUDE && [[ -f "$TARGET/.claude/skills/$skill/SKILL.md" ]]; then
    HASH="$(shasum -a 256 "$TARGET/.claude/skills/$skill/SKILL.md" | cut -d' ' -f1)"
    if $FIRST; then FIRST=false; else SKILL_HASHES+=","; fi
    SKILL_HASHES+=$'\n    '"\"$skill\": \"$HASH\""
  fi
done
SKILL_HASHES+=$'\n  }'

cat > "$MANIFEST_FILE" <<MANIFESTEOF
{
  "scaffold_version": "$SCAFFOLD_VERSION",
  "synced_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "common_skills": $SKILL_HASHES
}
MANIFESTEOF
ok "  Created .scaffold-manifest.json"

# ── Done ──
echo ""
echo -e "${GREEN}${BOLD}Setup complete!${NC}\n"
echo "Next steps:"
echo "  1. Edit CLAUDE.md — fill in project-specific sections"
if $USE_OPENCODE && [[ -n "$PP_ID" ]]; then
  echo "  2. Edit AGENTS.md — customize for your project"
fi
echo "  3. Create project-specific skills with /skill-creator"
echo "  4. Customize .claude/settings.local.json if needed"
echo ""
echo "Common skills installed: ${COMMON_SKILLS[*]}"
echo "Language skills installed: ${LANGS[*]}"
echo ""
