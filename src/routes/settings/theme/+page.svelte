<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import type { ThemeColors, ThemePreset } from '../../../types';
  import {
    themePresets,
    getDefaultColors,
    applyTheme,
    saveTheme,
    loadSavedTheme,
    type SavedTheme
  } from '../../../lib/themes';
  import ColorPicker from '../../../components/ColorPicker.svelte';
  import ThemePreview from '../../../components/ThemePreview.svelte';
  import SettingsLayout from '../../../components/SettingsLayout.svelte';
  import SaveFooter from '../../../components/SaveFooter.svelte';
  import { i18n } from '$lib/i18n';

  type ColorKey = keyof ThemeColors;

  // Color key to i18n key mapping
  const colorKeyMap: Record<ColorKey, string> = {
    paper: 'colorPaper',
    canvas: 'colorCanvas',
    mist: 'colorMist',
    stroke: 'colorStroke',
    ink: 'colorInk',
    inkMuted: 'colorInkMuted',
    accentSky: 'colorAccentSky',
    accentSkyStrong: 'colorAccentSkyStrong',
    accentMint: 'colorAccentMint',
    accentMintStrong: 'colorAccentMintStrong',
    accentPeach: 'colorAccentPeach',
    accentPeachStrong: 'colorAccentPeachStrong',
    white: 'colorWhite',
    border: 'colorBorder',
  };

  // Preset ID to i18n key mapping
  const presetNameMap: Record<string, string> = {
    default: 'themeDefault',
    dark: 'themeDark',
    ocean: 'themeOcean',
    forest: 'themeForest',
    sunset: 'themeSunset',
  };

  function getPresetName(presetId: string): string {
    const key = presetNameMap[presetId];
    return key ? i18n.t(key as keyof typeof i18n.t) : presetId;
  }

  function getColorLabel(colorKey: ColorKey): string {
    const key = colorKeyMap[colorKey];
    return key ? i18n.t(key as keyof typeof i18n.t) : colorKey;
  }

  let selectedPresetId = $state<string | null>('default');
  let isCustomMode = $state(false);
  let currentColors = $state<ThemeColors>(getDefaultColors());
  let originalTheme = $state<SavedTheme | null>(null);

  onMount(async () => {
    const saved = await loadSavedTheme();
    originalTheme = saved;

    if (saved) {
      if (saved.customColors) {
        isCustomMode = true;
        selectedPresetId = null;
        currentColors = { ...saved.customColors };
      } else if (saved.presetId) {
        selectedPresetId = saved.presetId;
        const preset = themePresets.find(p => p.id === saved.presetId);
        if (preset) {
          currentColors = { ...preset.colors };
        }
      }
    }
  });

  function selectPreset(preset: ThemePreset) {
    isCustomMode = false;
    selectedPresetId = preset.id;
    currentColors = { ...preset.colors };
    applyTheme(currentColors);
  }

  function enableCustomMode() {
    isCustomMode = true;
    selectedPresetId = null;
  }

  function handleColorChange(key: ColorKey, value: string) {
    currentColors = { ...currentColors, [key]: value };
    applyTheme(currentColors);
  }

  async function handleSave() {
    const theme: SavedTheme = isCustomMode
      ? { presetId: null, customColors: currentColors }
      : { presetId: selectedPresetId, customColors: null };

    await saveTheme(theme);
    goto('/settings');
  }

  function handleBack() {
    // Restore original theme
    const saved = originalTheme;
    if (saved) {
      if (saved.customColors) {
        applyTheme(saved.customColors);
      } else if (saved.presetId) {
        const preset = themePresets.find(p => p.id === saved.presetId);
        if (preset) {
          applyTheme(preset.colors);
        }
      }
    } else {
      applyTheme(getDefaultColors());
    }
    goto('/settings');
  }
</script>

<SettingsLayout title={i18n.t('themeTitle')} onBack={handleBack} contentClass="pb-32">
  <!-- Preset Selection -->
  <section class="section">
    <h2 class="section-title">{i18n.t('presetTheme')}</h2>
    <div class="preset-grid">
      {#each themePresets as preset}
        <button
          class="preset-btn"
          class:active={selectedPresetId === preset.id && !isCustomMode}
          onclick={() => selectPreset(preset)}
        >
          <div
            class="preset-preview"
            style="background: linear-gradient(135deg, {preset.colors.paper} 0%, {preset.colors.canvas} 50%, {preset.colors.accentSky} 100%);"
          ></div>
          <span class="preset-name">{getPresetName(preset.id)}</span>
        </button>
      {/each}
      <button
        class="preset-btn"
        class:active={isCustomMode}
        onclick={enableCustomMode}
      >
        <div class="preset-preview custom-preview">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
          </svg>
        </div>
        <span class="preset-name">{i18n.t('custom')}</span>
      </button>
    </div>
  </section>

  <!-- Theme Preview -->
  <section class="section">
    <h2 class="section-title">{i18n.t('preview')}</h2>
    <ThemePreview colors={currentColors} />
  </section>

  <!-- Custom Colors (only in custom mode) -->
  {#if isCustomMode}
    <section class="section">
      <h2 class="section-title">{i18n.t('customColors')}</h2>
      <div class="color-list">
        {#each Object.keys(colorKeyMap) as key}
          <ColorPicker
            label={getColorLabel(key as ColorKey)}
            value={currentColors[key as ColorKey]}
            onChange={(value) => handleColorChange(key as ColorKey, value)}
          />
        {/each}
      </div>
    </section>
  {/if}

  {#snippet footer()}
    <SaveFooter onSave={handleSave} />
  {/snippet}
</SettingsLayout>

<style>
  .section {
    margin-bottom: 24px;
  }

  .section-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--color-ink-muted);
    margin-bottom: 12px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .preset-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
  }

  .preset-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 8px;
    background: var(--color-canvas);
    border: 2px solid transparent;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .preset-btn:hover {
    background: var(--color-mist);
  }

  .preset-btn.active {
    border-color: var(--color-accent-sky-strong);
    background: var(--color-white);
  }

  .preset-preview {
    width: 48px;
    height: 48px;
    border-radius: 8px;
    border: 1px solid var(--color-stroke);
  }

  .custom-preview {
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-mist);
    color: var(--color-ink-muted);
  }

  .preset-name {
    font-size: 12px;
    color: var(--color-ink);
  }

  .color-list {
    background: var(--color-canvas);
    border-radius: 12px;
    padding: 4px 12px;
  }
</style>
