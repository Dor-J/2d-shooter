<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { TOUCH_BUTTONS, movementFromDrag, touchLayout, type TouchButton } from './input/touch'
import type { Action } from './input/actions'

const emit = defineEmits<{
  move: [state: { left: boolean; right: boolean; jump: boolean }]
  aim: [state: { dx: number; dy: number; fire: boolean }]
  action: [state: { pointerId: number; action: Action; down: boolean }]
  cancel: []
}>()

const movePointer = ref<number | null>(null)
const aimPointer = ref<number | null>(null)
const moveStart = ref({ x: 0, y: 0 })
const aimStart = ref({ x: 0, y: 0 })
const moveKnob = ref({ x: 0, y: 0 })
const aimKnob = ref({ x: 0, y: 0 })
const active = ref(new Set<Action>())
const viewport = ref({ width: 390, height: 780 })
const limit = (n: number) => Math.max(-40, Math.min(40, n))

const layout = computed(() => touchLayout(viewport.value.width, viewport.value.height))
const stanceButtons = computed(() => TOUCH_BUTTONS.filter(button => button.group === 'stance'))
const weaponButtons = computed(() => TOUCH_BUTTONS.filter(button => button.group === 'weapons'))
const panelButtons = computed(() => TOUCH_BUTTONS.filter(button => button.group === 'panel'))
const combatButtons = computed(() => TOUCH_BUTTONS.filter(button => button.group === 'combat'))

function measure() {
  viewport.value = { width: window.innerWidth, height: window.innerHeight }
}
onMounted(() => {
  measure()
  window.addEventListener('resize', measure)
  window.addEventListener('orientationchange', measure)
})
onBeforeUnmount(() => {
  window.removeEventListener('resize', measure)
  window.removeEventListener('orientationchange', measure)
  releaseEverything()
})

function releaseEverything() {
  active.value = new Set()
  emit('cancel')
}

function startMove(event: PointerEvent) {
  if (movePointer.value !== null) return
  movePointer.value = event.pointerId
  moveStart.value = { x: event.clientX, y: event.clientY }
  ;(event.currentTarget as HTMLElement).setPointerCapture(event.pointerId)
}
function dragMove(event: PointerEvent) {
  if (movePointer.value !== event.pointerId) return
  const dx = event.clientX - moveStart.value.x
  const dy = event.clientY - moveStart.value.y
  moveKnob.value = { x: limit(dx), y: limit(dy) }
  emit('move', movementFromDrag(dx, dy))
}
function stopMove(event: PointerEvent) {
  if (movePointer.value !== event.pointerId) return
  movePointer.value = null
  moveKnob.value = { x: 0, y: 0 }
  emit('move', { left: false, right: false, jump: false })
}
function startAim(event: PointerEvent) {
  if (aimPointer.value !== null) return
  aimPointer.value = event.pointerId
  aimStart.value = { x: event.clientX, y: event.clientY }
  ;(event.currentTarget as HTMLElement).setPointerCapture(event.pointerId)
  emit('aim', { dx: 1, dy: 0, fire: true })
}
function dragAim(event: PointerEvent) {
  if (aimPointer.value !== event.pointerId) return
  const dx = event.clientX - aimStart.value.x
  const dy = event.clientY - aimStart.value.y
  aimKnob.value = { x: limit(dx), y: limit(dy) }
  const length = Math.hypot(dx, dy)
  if (length > 8) emit('aim', { dx: dx / length, dy: dy / length, fire: true })
}
function stopAim(event: PointerEvent) {
  if (aimPointer.value !== event.pointerId) return
  aimPointer.value = null
  aimKnob.value = { x: 0, y: 0 }
  emit('aim', { dx: 0, dy: 0, fire: false })
}

function pressButton(button: TouchButton, event: PointerEvent) {
  ;(event.currentTarget as HTMLElement).setPointerCapture?.(event.pointerId)
  active.value = new Set(active.value).add(button.action)
  emit('action', { pointerId: event.pointerId, action: button.action, down: true })
}
function releaseButton(button: TouchButton, event: PointerEvent) {
  const next = new Set(active.value)
  next.delete(button.action)
  active.value = next
  emit('action', { pointerId: event.pointerId, action: button.action, down: false })
}
</script>

<template>
  <div
    class="mobile-controls"
    :class="layout.orientation"
    aria-label="Touch game controls"
    :style="{
      '--pad-size': `${layout.padSize}px`,
      '--button-size': `${layout.buttonSize}px`,
      paddingTop: layout.padding.top,
      paddingBottom: layout.padding.bottom,
      paddingLeft: layout.padding.left,
      paddingRight: layout.padding.right,
    }"
    @pointercancel="releaseEverything"
    @contextmenu.prevent
  >
    <div class="column left">
      <div class="button-row" role="group" aria-label="Stance controls">
        <button
          v-for="button in stanceButtons"
          :key="button.action"
          class="action-button"
          :class="{ active: active.has(button.action) }"
          :aria-label="button.hint"
          :aria-pressed="active.has(button.action)"
          @pointerdown.prevent="pressButton(button, $event)"
          @pointerup.prevent="releaseButton(button, $event)"
          @pointercancel.prevent="releaseButton(button, $event)"
        >{{ button.label }}</button>
      </div>
      <div
        class="control-pad move-pad"
        aria-label="Move and jump: drag left, right, or up"
        @pointerdown.prevent="startMove"
        @pointermove.prevent="dragMove"
        @pointerup.prevent="stopMove"
        @pointercancel.prevent="stopMove"
      >
        <span class="pad-ring"></span>
        <span class="pad-knob" :style="{ transform: `translate(${moveKnob.x}px, ${moveKnob.y}px)` }"></span>
        <span class="pad-label">MOVE / JUMP</span>
      </div>
    </div>

    <div class="column center">
      <div class="button-row wrap" role="group" aria-label="Panel controls">
        <button
          v-for="button in panelButtons"
          :key="button.action"
          class="action-button subtle"
          :class="{ active: active.has(button.action) }"
          :aria-label="button.hint"
          :aria-pressed="active.has(button.action)"
          @pointerdown.prevent="pressButton(button, $event)"
          @pointerup.prevent="releaseButton(button, $event)"
          @pointercancel.prevent="releaseButton(button, $event)"
        >{{ button.label }}</button>
      </div>
    </div>

    <div class="column right">
      <div class="button-row wrap" role="group" aria-label="Weapon controls">
        <button
          v-for="button in weaponButtons"
          :key="button.action"
          class="action-button"
          :class="{ active: active.has(button.action) }"
          :aria-label="button.hint"
          :aria-pressed="active.has(button.action)"
          @pointerdown.prevent="pressButton(button, $event)"
          @pointerup.prevent="releaseButton(button, $event)"
          @pointercancel.prevent="releaseButton(button, $event)"
        >{{ button.label }}</button>
      </div>
      <div class="pad-with-combat">
        <div class="button-column" role="group" aria-label="Combat controls">
          <button
            v-for="button in combatButtons"
            :key="button.action"
            class="action-button strong"
            :class="{ active: active.has(button.action) }"
            :aria-label="button.hint"
            :aria-pressed="active.has(button.action)"
            @pointerdown.prevent="pressButton(button, $event)"
            @pointerup.prevent="releaseButton(button, $event)"
            @pointercancel.prevent="releaseButton(button, $event)"
          >{{ button.label }}</button>
        </div>
        <div
          class="control-pad aim-pad"
          aria-label="Aim and fire: drag toward target"
          @pointerdown.prevent="startAim"
          @pointermove.prevent="dragAim"
          @pointerup.prevent="stopAim"
          @pointercancel.prevent="stopAim"
        >
          <span class="pad-ring"></span>
          <span class="pad-knob" :style="{ transform: `translate(${aimKnob.x}px, ${aimKnob.y}px)` }"></span>
          <span class="pad-label">AIM / FIRE</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mobile-controls{position:absolute;inset:0;display:flex;align-items:flex-end;justify-content:space-between;gap:12px;pointer-events:none;user-select:none;-webkit-user-select:none}
.column{display:flex;flex-direction:column;align-items:flex-start;gap:10px}
.column.right{align-items:flex-end}
.column.center{align-self:flex-start;align-items:center;flex:1}
.button-row{display:flex;gap:8px;pointer-events:none}
.button-row.wrap{flex-wrap:wrap;justify-content:flex-end;max-width:min(52vw,260px)}
.button-column{display:flex;flex-direction:column;gap:8px;justify-content:flex-end}
.pad-with-combat{display:flex;align-items:flex-end;gap:10px}
.control-pad{width:var(--pad-size);height:var(--pad-size);position:relative;pointer-events:auto;touch-action:none;border-radius:50%;background:#081019b8;border:1px solid #f2eadb7a;box-shadow:0 8px 28px #0008}
.pad-ring{position:absolute;inset:25%;border:1px solid #f2eadb76;border-radius:50%}
.pad-knob{position:absolute;left:calc(50% - 22px);top:calc(50% - 22px);width:44px;height:44px;border-radius:50%;background:#e9b94a;box-shadow:0 3px 11px #000a;will-change:transform}
.aim-pad .pad-knob{background:#f27e62}
.pad-label{position:absolute;left:0;right:0;bottom:-21px;text-align:center;color:#fff;font-size:10px;font-weight:800;letter-spacing:.08em;text-shadow:0 2px 5px #000}
.action-button{pointer-events:auto;touch-action:none;min-width:var(--button-size);height:var(--button-size);padding:0 9px;border:1px solid #fff9;background:#16222ee0;color:#f5f5ed;border-radius:calc(var(--button-size) / 2);font-size:10px;font-weight:800;letter-spacing:.06em;box-shadow:0 4px 14px #0008}
.action-button.subtle{background:#101a24c9;opacity:.92}
.action-button.strong{background:#243d51e6}
.action-button.active{background:#e9b94a;color:#10161d;border-color:#fff}
@media(max-height:420px){.button-row.wrap{max-width:min(40vw,220px)}}
</style>
