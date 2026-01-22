<script lang="ts">
  import type { FontSettings } from '../types';
  import { fontPresets, fontSizes } from '../lib/fonts';
  import { i18n } from '$lib/i18n';

  let { settings }: { settings: FontSettings } = $props();

  let fontFamily = $derived(
    fontPresets.find(p => p.id === settings.presetId)?.fontFamily || fontPresets[0].fontFamily
  );
  let fontSize = $derived(fontSizes[settings.size]?.base || 16);
</script>

<div class="preview-container">
  <div class="preview-card">
    <div
      class="preview-text"
      style="font-family: {fontFamily}; font-size: {fontSize}px;"
    >
      {i18n.t('fontPreviewText')}
    </div>
    <div
      class="preview-text-sm"
      style="font-family: {fontFamily}; font-size: {fontSize - 2}px;"
    >
      {i18n.t('fontPreviewText')}
    </div>
  </div>
</div>

<style>
  .preview-container {
    background: var(--color-canvas);
    border-radius: 12px;
    padding: 16px;
  }

  .preview-card {
    background: var(--color-white);
    border-radius: 8px;
    padding: 16px;
    border: 1px solid var(--color-stroke);
  }

  .preview-text {
    color: var(--color-ink);
    margin-bottom: 8px;
    line-height: 1.5;
  }

  .preview-text-sm {
    color: var(--color-ink-muted);
    line-height: 1.5;
  }
</style>
