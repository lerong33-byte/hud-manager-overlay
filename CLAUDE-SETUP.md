# Set up Claude Code CLI + HUD Manager on this machine

**Goal:** install the Claude Code CLI, connect it to the `hud-manager-fresh` repository, and link the **shared git memory** so this machine has the same context as all of Leron's other machines. Works on Linux or macOS (Windows notes inline). An AI agent can execute these directly.

**You need:** a terminal + Leron's GitHub Personal Access Token (PAT) with `repo` scope. Substitute it wherever you see `<TOKEN>`.

---

## 1. Install Node.js (skip if `node -v` works)
```bash
curl -fsSL https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash
export NVM_DIR="$HOME/.nvm"; . "$NVM_DIR/nvm.sh"; nvm install --lts
```
_(Windows: install Node LTS from https://nodejs.org)_

## 2. Install the Claude Code CLI
```bash
npm install -g @anthropic-ai/claude-code
claude --version
```

## 3. Clone the HUD Manager repo + save the token
```bash
export GH_TOKEN=<TOKEN>
git clone https://$GH_TOKEN@github.com/lerong33-byte/hud-manager.git ~/hud-manager-fresh
cd ~/hud-manager-fresh
git remote set-url origin https://github.com/lerong33-byte/hud-manager.git
mkdir -p .vscode && printf '%s' "$GH_TOKEN" > .vscode/.token && chmod 600 .vscode/.token
```
_(Windows: clone to `%USERPROFILE%\Documents\hud-manager-fresh`)_

## 4. Launch Claude once (creates its project folder + log in)
```bash
cd ~/hud-manager-fresh
claude
```
Type `/login`, authenticate, then exit. Must happen before step 5 (creates `~/.claude/projects/<hash>/`).

## 5. Link the SHARED memory (git-based — same memory as every other machine)
```bash
PROJ=$(ls -d ~/.claude/projects/*hud-manager* | head -1)
MEM="$PROJ/memory"
[ -d "$MEM" ] && mv "$MEM" "$MEM.bak"
git clone https://$GH_TOKEN@github.com/lerong33-byte/claude-memory.git "$MEM"
git -C "$MEM" config user.email "lerong33@gmail.com"
git -C "$MEM" config user.name "lerong33-byte"
```

## 6. Auto-sync memory (pull on session start, push on stop)
```bash
cat > ~/.claude/mem-pull.sh <<EOF
git -C "$MEM" pull --rebase --autostash -q 2>/dev/null || true
EOF
cat > ~/.claude/mem-push.sh <<EOF
git -C "$MEM" add -A 2>/dev/null || true
git -C "$MEM" diff --cached --quiet 2>/dev/null || { git -C "$MEM" commit -q -m "memory sync" 2>/dev/null; git -C "$MEM" push -q 2>/dev/null; }
EOF
chmod +x ~/.claude/mem-pull.sh ~/.claude/mem-push.sh
```
Then merge this `hooks` block into `~/.claude/settings.json`:
```json
"hooks": {
  "SessionStart": [{ "hooks": [{ "type": "command", "command": "bash ~/.claude/mem-pull.sh" }] }],
  "Stop":         [{ "hooks": [{ "type": "command", "command": "bash ~/.claude/mem-push.sh" }] }]
}
```

## Done
Run `claude` inside `~/hud-manager-fresh`. It now shares the same repo **and** memory as every other machine.

---

## Project context
- **HUD Manager** — Star Citizen ship-loadout configurator + fleet manager + AI mission engineer. Live at https://hud-manager.com. Vanilla HTML/JS/CSS on Cloudflare Pages. No build step, no tests, no package.json.
- **Deploy:** `node .vscode/push.mjs <files>` — defaults to `dev`. NEVER push to `main`/`--prod` without Leron's explicit per-deploy consent.
- **Read first:** `HANDOFF.md` and `MEMORY.md` in the memory folder (a clone of private `lerong33-byte/claude-memory`).
- Desktop **overlay** app is a separate private repo (`lerong33-byte/hud-manager-overlay-src`); releases go to public `lerong33-byte/hud-manager-overlay`. Overlay version must only ever go UP (a downgrade install breaks it).
