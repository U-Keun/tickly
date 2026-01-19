# CLAUDE.md

This file provides guidance to Claude Code when working with code in this repository.

## Project Overview

**Tickly** is a minimalist todo/task management app designed to help users remember items when going out. The core philosophy is **simplicity and speed** - get the app to users as quickly as possible with a clean, focused interface.

### Primary Goal
Deploy to iOS App Store as quickly as possible with a simple, functional design.

### Original Motivation
외출할 때 까먹는 물건이 없게 하기 위한 앱 (An app to prevent forgetting items when going out)

## Project Architecture

### Tech Stack
- **Frontend**: SvelteKit with TypeScript, Svelte 5 (runes syntax)
- **Backend**: Rust with Tauri v2 framework
- **Styling**: TailwindCSS (utility-first, minimal custom CSS)
- **Build System**: Vite + Cargo
- **Target Platform**: iOS (primary), Desktop (secondary)

### Key Directories
```
Tickly/
├── src/
│   ├── routes/
│   │   ├── +layout.svelte      # Global layout with CSS imports
│   │   ├── +layout.ts          # SPA mode config
│   │   ├── +page.svelte        # Main app page
│   │   └── settings/
│   │       ├── +page.svelte    # Settings main page
│   │       └── theme/
│   │           └── +page.svelte # Theme customization page
│   ├── components/             # Reusable Svelte components
│   ├── lib/
│   │   ├── themes.ts           # Theme presets and utilities
│   │   └── iosFocusFix.ts      # iOS input focus fix
│   ├── types.ts                # TypeScript type definitions
│   ├── app.css                 # Tailwind directives + CSS variables
│   └── app.html                # Root HTML template
├── src-tauri/
│   ├── src/lib.rs              # Tauri commands and app logic
│   ├── src/main.rs             # App entry point
│   └── tauri.conf.json         # Tauri configuration
├── static/                     # Static assets (IMPORTANT: use for iOS)
└── tailwind.config.ts          # TailwindCSS configuration
```

## Design Philosophy

### Core Principles
1. **Simplicity First**: Minimal UI, essential features only
2. **Speed to Market**: Ship fast, iterate based on user feedback
3. **Mobile-First**: Optimize for iOS touch experience
4. **Clean Design**: TailwindCSS utilities, avoid custom styling

### UI Guidelines
- Use simple, clear typography
- Minimal colors (focus on usability over decoration)
- Touch-friendly button sizes (min 44x44pt on iOS)
- Consistent spacing using Tailwind spacing scale
- Avoid unnecessary animations or effects

## Svelte 5 Syntax (IMPORTANT)

This project uses **Svelte 5** with runes syntax:

### Event Handling
- ✅ **Use**: `onclick={handler}`, `onsubmit={handler}`
- ❌ **Don't use**: `on:click={handler}`, `on:submit={handler}`
- **Exception**: Custom events still use `on:eventname` syntax

### State Management
- ✅ **Use**: `let items = $state([])` for reactive state
- ✅ **Use**: `let computed = $derived(items.length)` for computed values
- ❌ **Don't use**: `let items = []` with `$:` reactive statements

### Props
- ✅ **Use**: `let { prop1, prop2 = defaultValue } = $props()`
- ❌ **Don't use**: `export let prop1, prop2 = defaultValue`

### Example
```svelte
<script lang="ts">
  // State
  let count = $state(0);
  let doubled = $derived(count * 2);

  // Props
  let { title, items = [] } = $props();

  // Event handlers
  function handleClick() {
    count++;
  }
</script>

<button onclick={handleClick}>Click me</button>
```

## Development Commands

### Frontend Development
- `yarn dev` - Start SvelteKit dev server (port 1420)
- `yarn build` - Build frontend for production
- `yarn check` - Run Svelte type checking

### Tauri Development
- `yarn tauri dev` - Start Tauri desktop app in dev mode
- `yarn tauri build` - Build production desktop app

### iOS Development
- `yarn tauri ios init` - Initialize iOS project (one-time setup)
- `yarn tauri ios dev` - Deploy to iOS simulator or device
- `yarn tauri ios build` - Build production iOS app (.ipa)

## iOS Deployment Guide

### Prerequisites
1. Xcode installed (Mac only)
2. Apple Developer account (free for testing, paid for App Store)
3. iOS device or simulator

### Initial Setup
```bash
# Initialize iOS project
yarn tauri ios init

# Build frontend first
yarn build

# Deploy to device/simulator
yarn tauri ios dev [DEVICE_ID]

# List available devices
yarn tauri ios dev --list
```

### Important iOS Considerations

#### 1. Static Build Required
iOS cannot run internal HTTP servers. Always build frontend first:
```bash
yarn build  # Creates static files in build/
yarn tauri ios dev  # Uses built static files
```

#### 2. Asset Management
- ❌ **Don't use**: `/src/assets/image.png` (won't work on iOS)
- ✅ **Use**: `/static/image.png` → `/image.png` in code
- All images must be in `static/` folder
- Use absolute paths starting with `/`

#### 3. Bundle Identifier
Update in `src-tauri/tauri.conf.json`:
```json
{
  "identifier": "com.yourname.tickly"
}
```
Must be unique for App Store submission.

#### 4. Content Security Policy (CSP)
If using external resources (fonts, CDNs), update CSP in `tauri.conf.json`:
```json
{
  "security": {
    "csp": {
      "default-src": "'self' tauri: asset:",
      "font-src": "'self' asset: https://fonts.googleapis.com"
    }
  }
}
```

### App Store Submission Workflow
1. Get Apple Developer account ($99/year)
2. Set up certificates and provisioning profiles in Xcode
3. Build release version: `yarn tauri ios build`
4. Upload .ipa to App Store Connect
5. Fill in app metadata and screenshots
6. Submit for review

## Project Structure Best Practices

### File Organization
```
src/
├── components/
│   ├── TodoItem.svelte
│   ├── TodoList.svelte
│   └── AddItemInput.svelte
├── routes/
│   └── +page.svelte  # Main app page (import components here)
└── app.css  # Only Tailwind directives
```

### Component Guidelines
- One component per file
- Use Svelte 5 runes syntax
- Keep components small and focused
- Prefer composition over complexity

### Styling Guidelines
- Use TailwindCSS utilities exclusively
- Avoid custom CSS unless absolutely necessary
- **Use CSS variables for colors** (theme system support)
- Follow mobile-first responsive design

## Theme System

The app uses CSS variables for theming, allowing users to customize colors.

### CSS Variables (defined in `app.css`)
```css
:root {
  --color-paper: #f8f7f3;      /* Main background */
  --color-canvas: #f2efe8;     /* Secondary background */
  --color-ink: #5b5852;        /* Primary text */
  --color-ink-muted: #7a776f;  /* Secondary text */
  --color-accent-sky: #a8bddb; /* Primary accent */
  /* ... more colors */
}
```

### Using Theme Colors
- **In Tailwind**: Use custom colors like `bg-paper`, `text-ink`, `bg-accent-sky`
- **In CSS**: Use `var(--color-paper)`, `var(--color-ink)`, etc.
- **Avoid hardcoding colors** - always use CSS variables for theme consistency

### Theme Presets
5 built-in presets: 기본(Default), 다크(Dark), 오션(Ocean), 포레스트(Forest), 선셋(Sunset)

### Adding New Theme Colors
1. Add CSS variable in `src/app.css` `:root`
2. Add Tailwind mapping in `tailwind.config.ts`
3. Add to `ThemeColors` interface in `src/types.ts`
4. Update theme presets in `src/lib/themes.ts`

## Git Workflow

### Commit Messages
- Be concise and descriptive
- Use conventional commit format when appropriate
- Include Co-Authored-By for AI assistance

### Example
```bash
git commit -m "feat: add todo item completion toggle

- Add checkbox component
- Implement toggle state in Tauri backend
- Update UI to show completed items

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

## Implemented Features

- ✅ Todo CRUD (add, edit, delete, toggle)
- ✅ Category management (add, edit, delete, reorder)
- ✅ Item reordering (drag & drop)
- ✅ Swipe to delete
- ✅ Memo for each item
- ✅ Auto daily reset
- ✅ Theme customization (5 presets + custom colors)
- ✅ Settings page structure
- ✅ iOS-optimized UI with safe area handling

## Next Steps

1. Prepare App Store assets (icon, screenshots, description)
2. Submit to App Store
3. Iterate based on user feedback

Keep it simple. Ship fast. Iterate based on real user feedback.
