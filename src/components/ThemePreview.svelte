<script lang="ts">
  import type { ThemeColors } from '../types';

  interface Props {
    colors: ThemeColors;
  }

  let { colors }: Props = $props();
</script>

<div
  class="preview-container"
  style="
    --preview-paper: {colors.paper};
    --preview-canvas: {colors.canvas};
    --preview-ink: {colors.ink};
    --preview-ink-muted: {colors.inkMuted};
    --preview-stroke: {colors.stroke};
    --preview-accent-sky: {colors.accentSky};
    --preview-accent-peach: {colors.accentPeach};
    --preview-white: {colors.white};
  "
>
  <div class="preview-header">
    <div class="preview-tabs">
      <span class="tab active">기본</span>
      <span class="tab">외출</span>
    </div>
  </div>

  <div class="preview-content">
    <div class="preview-item">
      <div class="checkbox checked">
        <svg viewBox="0 0 24 24">
          <path d="M20 7L10 17l-5-5" />
        </svg>
      </div>
      <span class="item-text done">완료된 항목</span>
    </div>

    <div class="preview-item">
      <div class="checkbox"></div>
      <span class="item-text">할 일 항목</span>
    </div>
  </div>

  <div class="preview-buttons">
    <div class="btn-add">+</div>
    <div class="btn-reset">↻</div>
  </div>
</div>

<style>
  .preview-container {
    background: var(--preview-paper);
    border: 2px solid var(--preview-stroke);
    border-radius: 16px;
    padding: 12px;
    min-height: 160px;
    position: relative;
    overflow: hidden;
  }

  .preview-header {
    margin-bottom: 12px;
  }

  .preview-tabs {
    display: flex;
    gap: 8px;
  }

  .tab {
    font-size: 11px;
    padding: 4px 10px;
    border-radius: 12px;
    color: var(--preview-ink-muted);
    background: var(--preview-canvas);
  }

  .tab.active {
    background: var(--preview-accent-sky);
    color: var(--preview-ink);
    font-weight: 500;
  }

  .preview-content {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .preview-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    background: var(--preview-canvas);
    border: 1.5px solid var(--preview-ink);
    border-radius: 0 12px 0 12px;
  }

  .checkbox {
    width: 18px;
    height: 18px;
    border: 1.5px solid var(--preview-ink);
    border-radius: 3px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .checkbox.checked svg {
    width: 12px;
    height: 12px;
    fill: none;
    stroke: var(--preview-ink);
    stroke-width: 2.5;
    stroke-linecap: round;
    stroke-linejoin: round;
  }

  .item-text {
    font-size: 12px;
    color: var(--preview-ink);
  }

  .item-text.done {
    text-decoration: line-through;
    opacity: 0.5;
  }

  .preview-buttons {
    position: absolute;
    bottom: 12px;
    right: 12px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .btn-add,
  .btn-reset {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    font-weight: bold;
  }

  .btn-add {
    background: var(--preview-accent-sky);
    color: var(--preview-white);
  }

  .btn-reset {
    background: var(--preview-accent-peach);
    color: var(--preview-ink);
  }
</style>
