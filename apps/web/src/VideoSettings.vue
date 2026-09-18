<script setup lang="ts">
// What the player can change about the picture: how big the HUD is, which preset it uses, how much
// the machine is asked to draw, and what the weather is doing.

import { computed } from 'vue'
import { HUD_PRESETS, MAX_SCALE, MIN_SCALE, type HudPreset } from './hud/layout.ts'
import { PARTICLE_LEVELS, MIN_RESOLUTION_SCALE, MAX_RESOLUTION_SCALE, type ParticleLevel, type QualitySettings } from './render/quality.ts'
import { WEATHER_KINDS, type WeatherKind, type WeatherSettings } from './render/weather.ts'

const props = defineProps<{
  preset: HudPreset
  hudScale: number
  quality: QualitySettings
  weather: WeatherSettings
}>()

const emit = defineEmits<{
  (event: 'update:preset', value: HudPreset): void
  (event: 'update:hudScale', value: number): void
  (event: 'update:quality', value: QualitySettings): void
  (event: 'update:weather', value: WeatherSettings): void
  (event: 'close'): void
}>()

const scalePercent = computed(() => Math.round(props.hudScale * 100))

function setQuality(patch: Partial<QualitySettings>) {
  emit('update:quality', { ...props.quality, ...patch })
}

function setWeather(patch: Partial<WeatherSettings>) {
  emit('update:weather', { ...props.weather, ...patch })
}

function titleCase(value: string) {
  return value.charAt(0).toUpperCase() + value.slice(1)
}
</script>

<template>
  <section class="video-settings" aria-label="Video settings">
    <header>
      <h2>Video</h2>
      <button class="outline" @click="emit('close')">Close</button>
    </header>

    <label>
      HUD preset
      <select :value="preset" aria-label="HUD preset" @change="emit('update:preset', ($event.target as HTMLSelectElement).value as HudPreset)">
        <option v-for="name in HUD_PRESETS" :key="name" :value="name">{{ titleCase(name) }}</option>
      </select>
    </label>

    <label>
      HUD size · {{ scalePercent }}%
      <input
        type="range"
        :min="MIN_SCALE"
        :max="MAX_SCALE"
        step="0.05"
        :value="hudScale"
        aria-label="HUD size"
        @input="emit('update:hudScale', Number(($event.target as HTMLInputElement).value))"
      />
    </label>

    <label>
      Resolution · {{ Math.round(quality.resolutionScale * 100) }}%
      <input
        type="range"
        :min="MIN_RESOLUTION_SCALE"
        :max="MAX_RESOLUTION_SCALE"
        step="0.1"
        :value="quality.resolutionScale"
        aria-label="Resolution scale"
        @input="setQuality({ resolutionScale: Number(($event.target as HTMLInputElement).value) })"
      />
    </label>

    <label>
      Particles
      <select :value="quality.particles" aria-label="Particle detail" @change="setQuality({ particles: ($event.target as HTMLSelectElement).value as ParticleLevel })">
        <option v-for="level in PARTICLE_LEVELS" :key="level" :value="level">{{ titleCase(level) }}</option>
      </select>
    </label>

    <label class="switch">
      <input type="checkbox" :checked="quality.filtering === 'linear'" aria-label="Smooth textures" @change="setQuality({ filtering: ($event.target as HTMLInputElement).checked ? 'linear' : 'nearest' })" />
      Smooth textures
    </label>

    <label class="switch">
      <input type="checkbox" :checked="quality.mipmaps" aria-label="Mipmapping" @change="setQuality({ mipmaps: ($event.target as HTMLInputElement).checked })" />
      Mipmapping
    </label>

    <label class="switch">
      <input type="checkbox" :checked="quality.compatibility" aria-label="Compatibility rendering" @change="setQuality({ compatibility: ($event.target as HTMLInputElement).checked })" />
      Compatibility rendering
    </label>

    <label class="switch">
      <input type="checkbox" :checked="quality.weather" aria-label="Weather effects" @change="setQuality({ weather: ($event.target as HTMLInputElement).checked })" />
      Weather effects
    </label>

    <label>
      Weather
      <select :value="weather.kind" :disabled="!quality.weather" aria-label="Weather" @change="setWeather({ kind: ($event.target as HTMLSelectElement).value as WeatherKind })">
        <option v-for="kind in WEATHER_KINDS" :key="kind" :value="kind">{{ titleCase(kind) }}</option>
      </select>
    </label>

    <label>
      Wind
      <input
        type="range"
        min="-300"
        max="300"
        step="20"
        :value="weather.wind"
        :disabled="!quality.weather || weather.kind === 'none'"
        aria-label="Wind"
        @input="setWeather({ wind: Number(($event.target as HTMLInputElement).value) })"
      />
    </label>
  </section>
</template>
