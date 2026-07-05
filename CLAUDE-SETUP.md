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

## 7. Enable remote SSH so Leron's main Claude (on the MSI machine) can connect
This lets the primary machine's Claude SSH in for remote help/debugging. Run it as the normal user (`legordo`), NOT via sudo for the key parts.
```bash
# install + start the SSH server (Bazzite/Fedora Atomic may need a reboot after rpm-ostree)
command -v sshd >/dev/null 2>&1 || { command -v rpm-ostree >/dev/null && sudo rpm-ostree install -y openssh-server; command -v dnf >/dev/null && sudo dnf install -y openssh-server; command -v apt >/dev/null && sudo apt install -y openssh-server; }
sudo systemctl enable --now sshd 2>/dev/null || sudo systemctl enable --now ssh 2>/dev/null || true
sudo firewall-cmd --add-service=ssh --permanent 2>/dev/null && sudo firewall-cmd --reload 2>/dev/null || true

# authorize the MSI Claude key
mkdir -p ~/.ssh && chmod 700 ~/.ssh
KEY='ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIKAysu46wJpKGFgSqcEyrtyUPcVleKyq8LoZ63i/FT2f msi-claude->gamebeast'
grep -qF "$KEY" ~/.ssh/authorized_keys 2>/dev/null || echo "$KEY" >> ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys

# CRITICAL on SELinux systems (Bazzite/Fedora): relabel or sshd IGNORES the key file
command -v restorecon >/dev/null 2>&1 && restorecon -R -v ~/.ssh || true
sudo systemctl restart sshd 2>/dev/null || sudo systemctl restart ssh 2>/dev/null || true

# verify the key is really there + correctly labeled, then report the IP
echo "AUTH KEYS:"; cat ~/.ssh/authorized_keys; ls -laZ ~/.ssh 2>/dev/null || ls -la ~/.ssh
echo "SSH READY as $(whoami) @ $(hostname -I 2>/dev/null | awk '{print $1}')"
```
**If sshd was just installed via rpm-ostree on Bazzite, a REBOOT is required before it runs** — reboot, then re-run just this section. After this, Leron's main Claude connects with: `ssh -i ~/.ssh/gamebeast_ed25519 legordo@<ip>`.

## Done
Run `claude` inside `~/hud-manager-fresh`. It now shares the same repo **and** memory as every other machine.

---

## Project context
- **HUD Manager** — Star Citizen ship-loadout configurator + fleet manager + AI mission engineer. Live at https://hud-manager.com. Vanilla HTML/JS/CSS on Cloudflare Pages. No build step, no tests, no package.json.
- **Deploy:** `node .vscode/push.mjs <files>` — defaults to `dev`. NEVER push to `main`/`--prod` without Leron's explicit per-deploy consent.
- **Read first:** `HANDOFF.md` and `MEMORY.md` in the memory folder (a clone of private `lerong33-byte/claude-memory`).
- Desktop **overlay** app is a separate private repo (`lerong33-byte/hud-manager-overlay-src`); releases go to public `lerong33-byte/hud-manager-overlay`. Overlay version must only ever go UP (a downgrade install breaks it).
