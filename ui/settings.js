'use strict';
const { invoke } = window.__TAURI__.core;
const params = new URLSearchParams(location.search);
const section = ['general', 'instance', 'token', 'topics', 'notifications'].includes(params.get('section'))
  ? params.get('section') : 'general';
const root = document.getElementById(section);
root.hidden = false;
const status = document.getElementById('status');

function say(msg, err) {
  status.textContent = msg;
  status.classList.toggle('err', !!err);
}

const fields = [...root.querySelectorAll('[data-key]')];
let tokenSet = false;

async function load() {
  const s = await invoke('get_settings');
  for (const el of fields) {
    const v = s[el.dataset.key];
    el.value = v === undefined || v === null ? '' : String(v);
  }
  if (section === 'instance') {
    // the official instance is the "empty" value
    const el = root.querySelector('[data-key=instanceURL]');
    if (el.value === s.defaults.instanceURL) el.value = '';
  }
  if (section === 'token') {
    tokenSet = s.apiTokenSet;
    document.getElementById('tokenHint').textContent = tokenSet
      ? 'A token is saved. Leave empty to keep it, or enter a new one to replace it.'
      : 'No token saved.';
    document.getElementById('clearToken').hidden = !tokenSet;
  }
}

async function save() {
  try {
    if (section === 'token') {
      const t = document.getElementById('tokenInput').value.trim();
      if (!t) { say('Nothing to save.'); return; }
      await invoke('set_api_token', { token: t });
    } else {
      const patch = {};
      for (const el of fields) {
        if (!el.checkValidity()) { el.reportValidity(); return; }
        const t = el.dataset.type;
        patch[el.dataset.key] = t === 'flag' ? Number(el.value) : t === 'number' ? Number(el.value) : el.value.trim();
      }
      await invoke('save_settings', { patch });
    }
    say('Saved.');
    setTimeout(() => invoke('close_window', { label: 'settings' }), 600);
  } catch (e) {
    say(String(e), true);
  }
}

document.getElementById('save').addEventListener('click', save);
document.getElementById('cancel').addEventListener('click', () => invoke('close_window', { label: 'settings' }));
document.getElementById('clearToken').addEventListener('click', async () => {
  await invoke('set_api_token', { token: '' });
  say('Token removed.');
  setTimeout(() => invoke('close_window', { label: 'settings' }), 600);
});
document.addEventListener('keydown', (e) => {
  if (e.key === 'Enter' && e.target.tagName !== 'BUTTON') save();
  if (e.key === 'Escape') invoke('close_window', { label: 'settings' });
});
load().catch((e) => say(String(e), true));
