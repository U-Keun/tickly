<script lang="ts">
  interface Props {
    label: string;
    value: string;
    onChange: (color: string) => void;
  }

  let { label, value, onChange }: Props = $props();

  let hexInput = $state(value);

  $effect(() => {
    hexInput = value;
  });

  function handleColorInput(e: Event) {
    const target = e.target as HTMLInputElement;
    onChange(target.value);
  }

  function handleHexInput(e: Event) {
    const target = e.target as HTMLInputElement;
    let hex = target.value.trim();

    // Add # if missing
    if (hex && !hex.startsWith('#')) {
      hex = '#' + hex;
    }

    // Validate hex format
    if (/^#[0-9A-Fa-f]{6}$/.test(hex)) {
      onChange(hex);
    }

    hexInput = hex;
  }

  function handleHexBlur() {
    // Reset to current value if invalid
    if (!/^#[0-9A-Fa-f]{6}$/.test(hexInput)) {
      hexInput = value;
    }
  }
</script>

<div class="color-picker">
  <span class="label">{label}</span>
  <div class="controls">
    <div class="preview" style="background-color: {value}"></div>
    <input
      type="color"
      value={value}
      oninput={handleColorInput}
      class="color-input"
    />
    <input
      type="text"
      value={hexInput}
      oninput={handleHexInput}
      onblur={handleHexBlur}
      class="hex-input"
      placeholder="#000000"
      maxlength="7"
    />
  </div>
</div>

<style>
  .color-picker {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 0;
    border-bottom: 1px solid var(--color-stroke);
  }

  .label {
    font-size: 14px;
    color: var(--color-ink);
    min-width: 80px;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .preview {
    width: 28px;
    height: 28px;
    border-radius: 6px;
    border: 2px solid var(--color-stroke);
  }

  .color-input {
    width: 36px;
    height: 28px;
    padding: 0;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    background: none;
  }

  .color-input::-webkit-color-swatch-wrapper {
    padding: 0;
  }

  .color-input::-webkit-color-swatch {
    border: 2px solid var(--color-stroke);
    border-radius: 4px;
  }

  .hex-input {
    width: 80px;
    padding: 6px 8px;
    font-size: 13px;
    font-family: monospace;
    border: 2px solid var(--color-stroke);
    border-radius: 6px;
    background: var(--color-white);
    color: var(--color-ink);
    text-transform: uppercase;
  }

  .hex-input:focus {
    outline: none;
    border-color: var(--color-accent-sky);
  }
</style>
