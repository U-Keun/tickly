import type { Config } from 'tailwindcss';

export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        paper: '#f8f7f3',
        canvas: '#f2efe8',
        mist: '#ede9e1',
        stroke: '#e2ded5',
        ink: '#5b5852',
        'ink-muted': '#7a776f',
        accent: {
          sky: '#a8bddb',
          'sky-strong': '#8ea9cf',
          mint: '#bfd9c8',
          'mint-strong': '#a7c8b5',
          peach: '#e9c1ad',
          'peach-strong': '#dba892',
        },
      },
      fontFamily: {
        sans: ['Menlo', 'Monaco', 'Courier New', 'monospace'],
      },
    },
  },
  plugins: [],
} satisfies Config;
