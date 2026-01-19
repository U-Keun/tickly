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

  type ColorKey = keyof ThemeColors;

  const colorLabels: Record<ColorKey, string> = {
    paper: '배경 (Paper)',
    canvas: '캔버스 (Canvas)',
    mist: '미스트 (Mist)',
    stroke: '테두리 (Stroke)',
    ink: '텍스트 (Ink)',
    inkMuted: '텍스트 흐림 (Ink Muted)',
    accentSky: '스카이 (Sky)',
    accentSkyStrong: '스카이 진함 (Sky Strong)',
    accentMint: '민트 (Mint)',
    accentMintStrong: '민트 진함 (Mint Strong)',
    accentPeach: '피치 (Peach)',
    accentPeachStrong: '피치 진함 (Peach Strong)',
    white: '흰색 (White)',
    border: '경계선 (Border)',
  };

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

<div class="settings-container bg-paper">
  <!-- Header -->
  <header class="settings-header">
    <button class="back-btn" onclick={handleBack} aria-label="뒤로 가기">
      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
    </button>
    <h1 class="header-title">테마 설정</h1>
    <div class="w-6"></div>
  </header>

  <!-- Content -->
  <div class="settings-content">
    <!-- Preset Selection -->
    <section class="section">
      <h2 class="section-title">프리셋 테마</h2>
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
            <span class="preset-name">{preset.name}</span>
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
          <span class="preset-name">커스텀</span>
        </button>
      </div>
    </section>

    <!-- Theme Preview -->
    <section class="section">
      <h2 class="section-title">미리보기</h2>
      <ThemePreview colors={currentColors} />
    </section>

    <!-- Custom Colors (only in custom mode) -->
    {#if isCustomMode}
      <section class="section">
        <h2 class="section-title">커스텀 색상</h2>
        <div class="color-list">
          {#each Object.entries(colorLabels) as [key, label]}
            <ColorPicker
              {label}
              value={currentColors[key as ColorKey]}
              onChange={(value) => handleColorChange(key as ColorKey, value)}
            />
          {/each}
        </div>
      </section>
    {/if}
  </div>

  <!-- Save Button -->
  <div class="save-section">
    <button class="save-btn" onclick={handleSave}>
      저장
    </button>
  </div>
</div>

<style>
  .settings-container {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    flex-direction: column;
    padding-top: env(safe-area-inset-top, 0);
    padding-bottom: env(safe-area-inset-bottom, 0);
    overflow: hidden;
  }

  .settings-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid var(--color-stroke);
    background: var(--color-paper);
    position: sticky;
    top: 0;
    z-index: 10;
  }

  .back-btn {
    padding: 8px;
    color: var(--color-ink);
    background: none;
    border: none;
    cursor: pointer;
    border-radius: 8px;
  }

  .back-btn:hover {
    background: var(--color-canvas);
  }

  .header-title {
    font-size: 17px;
    font-weight: 600;
    color: var(--color-ink);
  }

  .settings-content {
    flex: 1;
    overflow-y: auto;
    -webkit-overflow-scrolling: touch;
    padding: 16px;
    padding-bottom: 100px;
    min-height: 0;
  }

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

  .save-section {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    padding: 16px;
    padding-bottom: calc(16px + env(safe-area-inset-bottom, 0));
    background: var(--color-paper);
    border-top: 1px solid var(--color-stroke);
  }

  .save-btn {
    width: 100%;
    padding: 14px;
    background: var(--color-accent-sky-strong);
    color: var(--color-white);
    font-size: 16px;
    font-weight: 600;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    transition: background 0.2s;
  }

  .save-btn:hover {
    background: var(--color-accent-sky);
  }
</style>
