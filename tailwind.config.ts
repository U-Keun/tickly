import type { Config } from 'tailwindcss';

export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        paper: 'var(--color-paper)',
        canvas: 'var(--color-canvas)',
        mist: 'var(--color-mist)',
        stroke: 'var(--color-stroke)',
        ink: 'var(--color-ink)',
        'ink-muted': 'var(--color-ink-muted)',
        accent: {
          sky: 'var(--color-accent-sky)',
          'sky-strong': 'var(--color-accent-sky-strong)',
          mint: 'var(--color-accent-mint)',
          'mint-strong': 'var(--color-accent-mint-strong)',
          peach: 'var(--color-accent-peach)',
          'peach-strong': 'var(--color-accent-peach-strong)',
        },
      },
      fontFamily: {
        sans: ['Menlo', 'Monaco', 'Courier New', 'monospace'],
      },
    },
  },
  plugins: [],
} satisfies Config;
