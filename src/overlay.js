/* H.U.D Manager Overlay — renderer logic */
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const pill      = document.getElementById('pill');
const panel     = document.getElementById('panel');
const pillDrag  = document.getElementById('pill-drag');
const panelDrag = document.getElementById('panel-drag');
const siteView  = document.getElementById('site-view');

let expanded = false;
let fadeTimer = null;

// ── EXPAND / COLLAPSE ─────────────────────────────────────────
async function expand(page) {
  if (expanded) return;
  expanded = true;
  clearFade();
  pill.hidden = true;
  panel.hidden = false;
  await invoke('set_expanded', { expanded: true });
  if (page) navigate(page);
}

async function collapse() {
  if (!expanded) return;
  expanded = false;
  panel.hidden = true;
  pill.hidden = false;
  await invoke('set_expanded', { expanded: false });
  scheduleFade();
}

function toggle() {
  if (expanded) collapse();
  else expand();
}

// ── NAVIGATION ────────────────────────────────────────────────
function navigate(page) {
  const base = 'https://hud-manager.com';
  const url = base + (page.startsWith('/') ? page : '/' + page);
  siteView.src = url;
  document.querySelectorAll('.ptab').forEach(t => {
    t.classList.toggle('active', t.dataset.page === page);
  });
}

// ── FADE-OUT WHEN IDLE ────────────────────────────────────────
function scheduleFade() {
  clearFade();
  fadeTimer = setTimeout(() => {
    document.body.classList.add('faded');
    invoke('set_ignore_mouse', { ignore: true });
  }, 3000);
}

function clearFade() {
  clearTimeout(fadeTimer);
  document.body.classList.remove('faded');
  invoke('set_ignore_mouse', { ignore: false });
}

document.addEventListener('mousemove', clearFade);
document.addEventListener('mouseenter', clearFade);
document.addEventListener('mouseleave', () => {
  if (!expanded) scheduleFade();
});

// ── DRAGGING ──────────────────────────────────────────────────
[pillDrag, panelDrag].forEach(handle => {
  handle.addEventListener('mousedown', () => invoke('start_drag'));
});

// ── PILL BUTTONS ──────────────────────────────────────────────
document.getElementById('btn-open').addEventListener('click', () => expand('/'));
document.getElementById('btn-trade').addEventListener('click', () => expand('/trade'));
document.getElementById('btn-starmap').addEventListener('click', () => expand('/starmap'));
document.getElementById('btn-ai').addEventListener('click', () => expand('/recommend'));

// ── PANEL BUTTONS ─────────────────────────────────────────────
document.getElementById('btn-collapse').addEventListener('click', collapse);

document.getElementById('btn-site').addEventListener('click', () => {
  const url = siteView.src || 'https://hud-manager.com';
  invoke('open_url', { url }).catch(() => {
    window.open(url, '_blank');
  });
});

// ── TAB BUTTONS ───────────────────────────────────────────────
document.querySelectorAll('.ptab').forEach(btn => {
  btn.addEventListener('click', () => navigate(btn.dataset.page));
});

// ── GLOBAL HOTKEY (emitted from Rust) ─────────────────────────
listen('hotkey-toggle', toggle);

// ── INITIAL STATE ─────────────────────────────────────────────
scheduleFade();
