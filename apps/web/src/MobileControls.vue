<script setup lang="ts">
import { ref } from 'vue'
import { movementFromDrag } from './mobile'

const emit = defineEmits<{
  move: [state: { left: boolean; right: boolean; jump: boolean }]
  aim: [state: { dx: number; dy: number; fire: boolean }]
  jet: [active: boolean]
}>()

const movePointer = ref<number | null>(null)
const aimPointer = ref<number | null>(null)
const moveStart = ref({ x: 0, y: 0 })
const aimStart = ref({ x: 0, y: 0 })
const moveKnob = ref({ x: 0, y: 0 })
const aimKnob = ref({ x: 0, y: 0 })
const limit = (n: number) => Math.max(-40, Math.min(40, n))

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
</script>

<template>
  <div class="mobile-controls" aria-label="Touch game controls">
    <div class="control-pad move-pad" aria-label="Move and jump: drag left, right, or up"
      @pointerdown.prevent="startMove" @pointermove.prevent="dragMove"
      @pointerup.prevent="stopMove" @pointercancel.prevent="stopMove">
      <span class="pad-ring"></span><span class="pad-knob" :style="{ transform: `translate(${moveKnob.x}px, ${moveKnob.y}px)` }"></span>
      <span class="pad-label">MOVE / JUMP</span>
    </div>
    <button class="jet-button" aria-label="Hold to use jetpack" @pointerdown.prevent="emit('jet', true)" @pointerup.prevent="emit('jet', false)" @pointercancel.prevent="emit('jet', false)">JET</button>
    <div class="control-pad aim-pad" aria-label="Aim and fire: drag toward target"
      @pointerdown.prevent="startAim" @pointermove.prevent="dragAim"
      @pointerup.prevent="stopAim" @pointercancel.prevent="stopAim">
      <span class="pad-ring"></span><span class="pad-knob" :style="{ transform: `translate(${aimKnob.x}px, ${aimKnob.y}px)` }"></span>
      <span class="pad-label">AIM / FIRE</span>
    </div>
  </div>
</template>

<style scoped>
.mobile-controls{position:absolute;inset:0;display:flex;align-items:flex-end;justify-content:space-between;gap:12px;padding:0 max(18px,env(safe-area-inset-right)) max(18px,env(safe-area-inset-bottom)) max(18px,env(safe-area-inset-left));pointer-events:none;user-select:none;-webkit-user-select:none}
.control-pad{width:clamp(112px,28vw,154px);height:clamp(112px,28vw,154px);position:relative;pointer-events:auto;touch-action:none;border-radius:50%;background:#081019b8;border:1px solid #f2eadb7a;box-shadow:0 8px 28px #0008}
.pad-ring{position:absolute;inset:25%;border:1px solid #f2eadb76;border-radius:50%}
.pad-knob{position:absolute;left:calc(50% - 22px);top:calc(50% - 22px);width:44px;height:44px;border-radius:50%;background:#e9b94a;box-shadow:0 3px 11px #000a;will-change:transform}
.aim-pad .pad-knob{background:#f27e62}
.pad-label{position:absolute;left:0;right:0;bottom:-21px;text-align:center;color:#fff;font-size:10px;font-weight:800;letter-spacing:.08em;text-shadow:0 2px 5px #000}
.jet-button{pointer-events:auto;touch-action:none;width:64px;height:64px;align-self:flex-end;margin-bottom:16px;border:1px solid #fff9;background:#243d51d9;color:#f5f5ed;border-radius:50%;font-size:13px;box-shadow:0 5px 18px #0008}
@media(max-width:700px) and (orientation:portrait){.mobile-controls{padding-bottom:max(30px,env(safe-area-inset-bottom))}.control-pad{width:118px;height:118px}.jet-button{width:58px;height:58px;margin-bottom:8px}}
@media(max-height:420px){.control-pad{width:106px;height:106px}.jet-button{width:54px;height:54px}}
</style>
