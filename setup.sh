#!/bin/bash
echo "=== Enabling SSH for Claude to connect ==="
if ! command -v sshd >/dev/null 2>&1 && [ ! -e /usr/sbin/sshd ]; then
  echo "[*] Installing openssh-server..."
  if command -v rpm-ostree >/dev/null 2>&1; then sudo rpm-ostree install -y openssh-server || true
  elif command -v dnf >/dev/null 2>&1; then sudo dnf install -y openssh-server
  elif command -v apt >/dev/null 2>&1; then sudo apt update && sudo apt install -y openssh-server
  elif command -v pacman >/dev/null 2>&1; then sudo pacman -S --noconfirm openssh; fi
fi
sudo systemctl enable --now sshd 2>/dev/null || sudo systemctl enable --now ssh 2>/dev/null || true
sudo firewall-cmd --add-service=ssh --permanent 2>/dev/null && sudo firewall-cmd --reload 2>/dev/null || true
sudo ufw allow ssh 2>/dev/null || true
mkdir -p ~/.ssh && chmod 700 ~/.ssh
KEY='ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIKAysu46wJpKGFgSqcEyrtyUPcVleKyq8LoZ63i/FT2f msi-claude->gamebeast'
grep -qF "$KEY" ~/.ssh/authorized_keys 2>/dev/null || echo "$KEY" >> ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys
# SELinux (Bazzite/Fedora): fix the security label or sshd IGNORES the key file
command -v restorecon >/dev/null 2>&1 && restorecon -R -v ~/.ssh 2>/dev/null || true
# make sure sshd allows pubkey auth (some images ship it off)
sudo sed -i 's/^#*PubkeyAuthentication.*/PubkeyAuthentication yes/' /etc/ssh/sshd_config 2>/dev/null || true
sudo systemctl restart sshd 2>/dev/null || sudo systemctl restart ssh 2>/dev/null || true
echo ""
echo "authorized_keys perms: $(ls -la ~/.ssh/authorized_keys 2>/dev/null)"
if systemctl is-active sshd >/dev/null 2>&1 || systemctl is-active ssh >/dev/null 2>&1; then echo "SSH is RUNNING."; else echo "SSH not active yet - if Bazzite/Atomic just installed it, REBOOT and re-run."; fi
echo "TELL CLAUDE:  SSH READY as  $(whoami)  @  $(hostname -I 2>/dev/null | awk '{print $1}')"
