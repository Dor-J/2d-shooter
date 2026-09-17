<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { ACTIONS, type Action, type ActionCategory } from './input/actions'
import { DEFAULT_BINDINGS, conflictsFor, rebind, clearBinding, resetBindings, type Binding } from './input/bindings'
import { ACCESSIBILITY_COMBOS, ProfileStore, exportProfile, importProfile, type ControlProfile } from './input/profiles'
import { clampSensitivity } from './input/mouse'
import { gamepadCodes } from './input/gamepad'
import type { InputSystem } from './input'

const props = defineProps<{ system: InputSystem; store: ProfileStore }>()
const emit = defineEmits<{ close: []; profile: [profile: ControlProfile]; scoreboardMode: [mode: 'hold' | 'toggle'] }>()

const CATEGORIES: { id: ActionCategory; title: string }[] = [
  { id: 'movement', title: 'Movement' },
  { id: 'combat', title: 'Combat' },
  { id: 'weapons', title: 'Weapons' },
  { id: 'communication', title: 'Communication' },
  { id: 'interface', title: 'Interface' },
  { id: 'media', title: 'Media' },
  { id: 'system', title: 'System' },
]

const profile = ref<ControlProfile>(props.store.active())
const capturing = ref<{ action: Action; slot: number } | null>(null)
const notice = ref('')
const importText = ref('')
const holdActions = ACTIONS.filter(action => action.kind === 'hold')

const profiles = computed(() => props.store.list())
const bindings = computed(() => profile.value.bindings)

function commit(next: ControlProfile) {
  profile.value = props.store.update(next.id, () => next)
  props.system.applyProfile(profile.value)
  emit('profile', profile.value)
}

function describeBinding(binding: Binding) {
  const label = binding.code.replace(/^(Key|Digit)/, '').replace('Mouse', 'Mouse ').replace('Button', 'Pad ').replace('Axis', 'Stick ')
  return binding.device === 'keyboard' ? label : `${label}`
}

function startCapture(action: Action, slot: number) {
  capturing.value = { action, slot }
  notice.value = 'Press any key, mouse button, or gamepad button. Escape cancels, Backspace clears.'
}

function applyCapture(binding: Binding) {
  const target = capturing.value
  if (!target) return
  const taken = conflictsFor(bindings.value, binding, target.action)
  commit({ ...profile.value, bindings: rebind(bindings.value, target.action, target.slot, binding) })
  notice.value = taken.length > 0 ? `Taken from ${taken.join(', ')}.` : 'Control saved.'
  capturing.value = null
}

function onCaptureKey(event: KeyboardEvent) {
  if (!capturing.value) return
  event.preventDefault()
  event.stopPropagation()
  if (event.code === 'Escape') {
    capturing.value = null
    notice.value = 'Rebinding cancelled.'
    return
  }
  if (event.code === 'Backspace') {
    commit({ ...profile.value, bindings: clearBinding(bindings.value, capturing.value.action, capturing.value.slot) })
    capturing.value = null
    notice.value = 'Control cleared.'
    return
  }
  applyCapture({ device: 'keyboard', code: event.code })
}

function onCaptureMouse(event: MouseEvent) {
  if (!capturing.value) return
  event.preventDefault()
  event.stopPropagation()
  applyCapture({ device: 'mouse', code: `Mouse${event.button}` })
}

let padFrame = 0
function pollPad() {
  padFrame = requestAnimationFrame(pollPad)
  if (!capturing.value) return
  const pads = navigator.getGamepads ? navigator.getGamepads() : []
  for (const pad of pads ?? []) {
    if (!pad?.connected) continue
    const index = pad.buttons.findIndex(button => button.pressed)
    if (index >= 0) {
      applyCapture({ device: 'gamepad', code: `Button${index}` })
      return
    }
  }
}

onMounted(() => {
  window.addEventListener('keydown', onCaptureKey, true)
  window.addEventListener('mousedown', onCaptureMouse, true)
  padFrame = requestAnimationFrame(pollPad)
})
onBeforeUnmount(() => {
  window.removeEventListener('keydown', onCaptureKey, true)
  window.removeEventListener('mousedown', onCaptureMouse, true)
  cancelAnimationFrame(padFrame)
})

function selectProfile(id: string) {
  profile.value = props.store.select(id)
  props.system.applyProfile(profile.value)
  emit('profile', profile.value)
}
function newProfile() {
  const created = props.store.create(`Profile ${profiles.value.length + 1}`)
  selectProfile(created.id)
}
function renameProfile(event: Event) {
  const name = (event.target as HTMLInputElement).value
  commit({ ...profile.value, name: name.trim().slice(0, 24) || profile.value.name })
}
function deleteProfile() {
  profile.value = props.store.remove(profile.value.id)
  props.system.applyProfile(profile.value)
  emit('profile', profile.value)
}
function download() {
  const blob = new Blob([exportProfile(profile.value)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download = `${profile.value.name.replace(/\W+/g, '-').toLowerCase()}-controls.json`
  link.click()
  URL.revokeObjectURL(url)
}
function runImport() {
  const imported = importProfile(importText.value)
  if (!imported) {
    notice.value = 'That is not an exported control profile.'
    return
  }
  props.store.add(imported)
  importText.value = ''
  selectProfile(imported.id)
  notice.value = `Imported ${imported.name}.`
}
function restoreDefaults() {
  commit({ ...profile.value, bindings: resetBindings() })
  notice.value = 'Default controls restored.'
}
function setSensitivity(event: Event) {
  const sensitivity = clampSensitivity(Number((event.target as HTMLInputElement).value))
  commit({ ...profile.value, settings: { ...profile.value.settings, sensitivity } })
}
function setVolume(key: 'volume' | 'musicVolume', event: Event) {
  const value = Math.min(1, Math.max(0, Number((event.target as HTMLInputElement).value)))
  commit({ ...profile.value, settings: { ...profile.value.settings, [key]: value } })
}
function toggleLatch(action: Action, event: Event) {
  const checked = (event.target as HTMLInputElement).checked
  const toggleActions = checked
    ? [...profile.value.settings.toggleActions, action]
    : profile.value.settings.toggleActions.filter(item => item !== action)
  commit({ ...profile.value, settings: { ...profile.value.settings, toggleActions } })
}
function setScoreboardMode(event: Event) {
  emit('scoreboardMode', (event.target as HTMLSelectElement).value === 'toggle' ? 'toggle' : 'hold')
}
function actionsIn(category: ActionCategory) {
  return ACTIONS.filter(action => action.category === category)
}
function slotsFor(action: Action) {
  const current = bindings.value[action] ?? []
  return current.length >= 3 ? current : [...current, null]
}
</script>

<template>
  <section class="controls-settings" role="dialog" aria-label="Controls and input settings">
    <header>
      <h2>Controls</h2>
      <button class="outline" @click="emit('close')">Done</button>
    </header>

    <div class="profile-row">
      <label>
        Profile
        <select :value="profile.id" aria-label="Control profile" @change="selectProfile(($event.target as HTMLSelectElement).value)">
          <option v-for="item in profiles" :key="item.id" :value="item.id">{{ item.name }}</option>
        </select>
      </label>
      <label>
        Name
        <input :value="profile.name" maxlength="24" aria-label="Profile name" @change="renameProfile" />
      </label>
      <button class="outline" @click="newProfile">New</button>
      <button class="outline" :disabled="profiles.length < 2" @click="deleteProfile">Delete</button>
      <button class="outline" @click="download">Export</button>
      <button class="outline" @click="restoreDefaults">Reset to defaults</button>
    </div>

    <p v-if="notice" class="notice" role="status">{{ notice }}</p>

    <div class="sliders">
      <label>
        Mouse sensitivity <b>{{ profile.settings.sensitivity.toFixed(2) }}</b>
        <input type="range" min="0.1" max="5" step="0.05" :value="profile.settings.sensitivity" @input="setSensitivity" />
      </label>
      <label>
        Sound volume <b>{{ Math.round(profile.settings.volume * 100) }}%</b>
        <input type="range" min="0" max="1" step="0.05" :value="profile.settings.volume" @input="setVolume('volume', $event)" />
      </label>
      <label>
        Music volume <b>{{ Math.round(profile.settings.musicVolume * 100) }}%</b>
        <input type="range" min="0" max="1" step="0.05" :value="profile.settings.musicVolume" @input="setVolume('musicVolume', $event)" />
      </label>
      <label>
        Scoreboard
        <select :value="system.ui.scoreboardMode" @change="setScoreboardMode">
          <option value="hold">Hold to show</option>
          <option value="toggle">Toggle on press</option>
        </select>
      </label>
    </div>

    <div class="accessibility">
      <h3>Accessibility</h3>
      <p>
        Every control that normally needs two keys also has a single-key alternative:
        <span v-for="combo in ACCESSIBILITY_COMBOS" :key="combo.alternative">{{ combo.label }} ({{ combo.keys.join(' + ') }} or its own key). </span>
      </p>
      <div class="latch-grid">
        <label v-for="action in holdActions" :key="action.id">
          <input
            type="checkbox"
            :checked="profile.settings.toggleActions.includes(action.id)"
            @change="toggleLatch(action.id, $event)"
          />
          Toggle {{ action.label.toLowerCase() }} instead of holding
        </label>
      </div>
    </div>

    <div v-for="category in CATEGORIES" :key="category.id" class="category">
      <h3>{{ category.title }}</h3>
      <div v-for="action in actionsIn(category.id)" :key="action.id" class="binding-row">
        <span class="binding-name">{{ action.label }}</span>
        <button
          v-for="(binding, slot) in slotsFor(action.id)"
          :key="slot"
          class="chip"
          :class="{ capturing: capturing?.action === action.id && capturing?.slot === slot, empty: !binding }"
          :aria-label="`Rebind ${action.label}, slot ${slot + 1}`"
          @click="startCapture(action.id, slot)"
        >
          {{ capturing?.action === action.id && capturing?.slot === slot ? 'Press…' : binding ? describeBinding(binding) : 'Add' }}
        </button>
      </div>
    </div>

    <div class="import">
      <h3>Import a profile</h3>
      <textarea v-model="importText" rows="3" aria-label="Exported profile JSON" placeholder="Paste an exported profile"></textarea>
      <button class="outline" :disabled="!importText.trim()" @click="runImport">Import</button>
    </div>

    <p class="footnote">
      Gamepad codes available for binding: {{ gamepadCodes().slice(0, 8).join(', ') }}…
      Defaults follow {{ Object.keys(DEFAULT_BINDINGS).length }} named controls.
    </p>
  </section>
</template>

<style scoped>
.controls-settings{display:flex;flex-direction:column;gap:14px;max-height:min(80vh,720px);overflow:auto;padding:18px;background:#0d151cf5;border:1px solid #f2eadb2e;border-radius:14px}
header{display:flex;align-items:center;justify-content:space-between;gap:12px}
h2{margin:0;font-size:18px;letter-spacing:.06em}
h3{margin:0 0 6px;font-size:12px;letter-spacing:.14em;text-transform:uppercase;color:#f2eadb9c}
.profile-row{display:flex;flex-wrap:wrap;align-items:flex-end;gap:8px}
.profile-row label,.sliders label{display:flex;flex-direction:column;gap:4px;font-size:11px;letter-spacing:.08em;text-transform:uppercase;color:#f2eadb9c}
.sliders{display:grid;grid-template-columns:repeat(auto-fit,minmax(190px,1fr));gap:12px}
.sliders b{color:#e9b94a}
.accessibility p{margin:0 0 8px;font-size:12px;color:#f2eadbb5;line-height:1.5}
.latch-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(230px,1fr));gap:6px;font-size:12px}
.latch-grid label{display:flex;align-items:center;gap:8px}
.category{display:flex;flex-direction:column;gap:4px}
.binding-row{display:flex;align-items:center;gap:8px;flex-wrap:wrap}
.binding-name{flex:1 1 180px;font-size:13px}
.chip{min-width:76px;padding:5px 10px;border-radius:7px;border:1px solid #f2eadb3d;background:#16222e;color:#f5f5ed;font-size:11px;letter-spacing:.06em}
.chip.empty{opacity:.55;border-style:dashed}
.chip.capturing{background:#e9b94a;color:#10161d;border-color:#fff}
.notice{margin:0;font-size:12px;color:#e9b94a}
.import{display:flex;flex-direction:column;gap:6px}
textarea{width:100%;background:#0a1219;color:#f5f5ed;border:1px solid #f2eadb2e;border-radius:8px;padding:8px;font-family:inherit;font-size:12px}
.footnote{margin:0;font-size:11px;color:#f2eadb6b}
@media(max-width:700px){.controls-settings{padding:14px}.binding-name{flex-basis:100%}}
</style>
