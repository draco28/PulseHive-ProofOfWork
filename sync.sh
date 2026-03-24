#!/usr/bin/env bash
set -euo pipefail

# ─────────────────────────────────────────────────
# ai-dev-scaffold — Sync common skills to a project
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
skip()  { echo -e "${YELLOW}⊘${NC} $1"; }

# ── Parse arguments ──
TARGET="${1:-.}"
TARGET="$(cd "$TARGET" && pwd)"

MANIFEST="$TARGET/.claude/.scaffold-manifest.json"

echo -e "\n${BOLD}ai-dev-scaffold${NC} — Sync Common Skills\n"
echo -e "Source:  ${CYAN}${SCAFFOLD_DIR}${NC} (${SCAFFOLD_VERSION})"
echo -e "Target:  ${CYAN}${TARGET}${NC}\n"

if [[ ! -f "$MANIFEST" ]]; then
  err "No .scaffold-manifest.json found in $TARGET/.claude/"
  echo "Run scaffold.sh first to initialize the project."
  exit 1
fi

# ── Read manifest ──
UPDATED=0
SKIPPED=0
SYNCED=0

for skill in "${COMMON_SKILLS[@]}"; do
  SKILL_SOURCE="$SCAFFOLD_DIR/claude/skills/$skill/SKILL.md"
  SKILL_TARGET="$TARGET/.claude/skills/$skill/SKILL.md"

  if [[ ! -f "$SKILL_SOURCE" ]]; then
    warn "$skill: source not found in scaffold (skipping)"
    ((SKIPPED++))
    continue
  fi

  if [[ ! -f "$SKILL_TARGET" ]]; then
    # Skill doesn't exist in target — copy it
    mkdir -p "$TARGET/.claude/skills/$skill"
    cp "$SKILL_SOURCE" "$SKILL_TARGET"
    ok "$skill: installed (new)"
    ((UPDATED++))
    continue
  fi

  # Get the hash from manifest
  MANIFEST_HASH="$(python3 -c "
import json, sys
with open('$MANIFEST') as f:
    m = json.load(f)
print(m.get('common_skills', {}).get('$skill', ''))
" 2>/dev/null || echo '')"

  # Get current hash of the target file
  CURRENT_HASH="$(shasum -a 256 "$SKILL_TARGET" | cut -d' ' -f1)"

  if [[ "$CURRENT_HASH" != "$MANIFEST_HASH" ]] && [[ -n "$MANIFEST_HASH" ]]; then
    # File has been locally modified
    skip "$skill: locally modified — skipping (use --force to overwrite)"
    echo "       Diff: diff '$SKILL_SOURCE' '$SKILL_TARGET'"
    ((SKIPPED++))
    continue
  fi

  # Check if scaffold version is different from target
  SOURCE_HASH="$(shasum -a 256 "$SKILL_SOURCE" | cut -d' ' -f1)"

  if [[ "$SOURCE_HASH" == "$CURRENT_HASH" ]]; then
    ok "$skill: up to date"
    ((SYNCED++))
    continue
  fi

  # Safe to update
  cp "$SKILL_SOURCE" "$SKILL_TARGET"
  ok "$skill: updated"
  ((UPDATED++))

  # Also update OpenCode if it exists
  if [[ -d "$TARGET/.opencode/skills/$skill" ]]; then
    # Simple frontmatter transform
    sed '/^allowed-tools:/d; /^user-invocable:/d' "$SKILL_SOURCE" > "$TARGET/.opencode/skills/$skill/SKILL.md"
    ok "$skill: updated (OpenCode)"
  fi
done

# ── Update manifest ──
SKILL_HASHES="{"
FIRST=true
for skill in "${COMMON_SKILLS[@]}"; do
  if [[ -f "$TARGET/.claude/skills/$skill/SKILL.md" ]]; then
    HASH="$(shasum -a 256 "$TARGET/.claude/skills/$skill/SKILL.md" | cut -d' ' -f1)"
    if $FIRST; then FIRST=false; else SKILL_HASHES+=","; fi
    SKILL_HASHES+=$'\n    '"\"$skill\": \"$HASH\""
  fi
done
SKILL_HASHES+=$'\n  }'

cat > "$MANIFEST" <<MANIFESTEOF
{
  "scaffold_version": "$SCAFFOLD_VERSION",
  "synced_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "common_skills": $SKILL_HASHES
}
MANIFESTEOF

# ── Summary ──
echo ""
echo -e "${BOLD}Summary${NC}: $UPDATED updated, $SYNCED up-to-date, $SKIPPED skipped"
echo ""
