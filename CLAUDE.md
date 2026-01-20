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
- **Database**: SQLite (rusqlite)
- **Styling**: TailwindCSS (utility-first, minimal custom CSS)
- **Build System**: Vite + Cargo
- **Target Platform**: iOS (primary), Desktop (secondary)

### Architecture Overview

The project follows a layered architecture for maintainability and separation of concerns:

```
┌─────────────────────────────────────────────────────────────┐
│                        Frontend                              │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │   Routes    │ -> │ Components  │ -> │  API Layer  │     │
│  │ (+page.svelte)   │ (*.svelte)  │    │ (lib/api/*) │     │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
└─────────────────────────────────────────────────────────────┘
                              │ invoke()
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     Rust Backend                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │  Commands   │ -> │  Services   │ -> │ Repository  │     │
│  │ (Tauri API) │    │ (Business)  │    │ (Data Access)│    │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
│                                               │              │
│                                               ▼              │
│                                        ┌─────────────┐      │
│                                        │   SQLite    │      │
│                                        └─────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

### Key Directories

```
Tickly/
├── src/                              # Frontend (SvelteKit)
│   ├── routes/
│   │   ├── +layout.svelte            # Global layout with CSS imports
│   │   ├── +layout.ts                # SPA mode config
│   │   ├── +page.svelte              # Main app page
│   │   └── settings/
│   │       ├── +page.svelte          # Settings main page
│   │       └── theme/
│   │           └── +page.svelte      # Theme customization page
│   ├── components/                   # Reusable Svelte components
│   ├── lib/
│   │   ├── api/                      # API Layer (Tauri invoke wrappers)
│   │   │   ├── index.ts              # Re-exports
│   │   │   ├── client.ts             # Base invoke wrapper
│   │   │   ├── categoryApi.ts        # Category API functions
│   │   │   ├── todoApi.ts            # Todo API functions
│   │   │   └── settingsApi.ts        # Settings API functions
│   │   ├── themes.ts                 # Theme presets and utilities
│   │   └── iosFocusFix.ts            # iOS input focus fix
│   ├── types.ts                      # TypeScript type definitions
│   ├── app.css                       # Tailwind directives + CSS variables
│   └── app.html                      # Root HTML template
├── src-tauri/                        # Backend (Rust + Tauri)
│   ├── src/
│   │   ├── lib.rs                    # App entry point & module exports
│   │   ├── main.rs                   # Main entry
│   │   ├── models/                   # Data models
│   │   │   ├── mod.rs
│   │   │   ├── category.rs           # Category struct
│   │   │   └── todo_item.rs          # TodoItem struct
│   │   ├── repository/               # Data access layer
│   │   │   ├── mod.rs
│   │   │   ├── database.rs           # DB initialization
│   │   │   ├── migration.rs          # Schema migrations
│   │   │   ├── category_repo.rs      # Category CRUD
│   │   │   ├── todo_repo.rs          # Todo CRUD
│   │   │   └── settings_repo.rs      # Settings CRUD
│   │   ├── service/                  # Business logic layer
│   │   │   ├── mod.rs
│   │   │   ├── category_service.rs   # Category business logic
│   │   │   ├── todo_service.rs       # Todo business logic
│   │   │   └── reset_service.rs      # Reset/auto-reset logic
│   │   └── commands/                 # Tauri command handlers
│   │       ├── mod.rs
│   │       ├── category_commands.rs  # Category Tauri commands
│   │       ├── todo_commands.rs      # Todo Tauri commands
│   │       └── settings_commands.rs  # Settings Tauri commands
│   └── tauri.conf.json               # Tauri configuration
├── static/                           # Static assets (IMPORTANT: use for iOS)
└── tailwind.config.ts                # TailwindCSS configuration
```

## Design Philosophy

### Core Principles
1. **Simplicity First**: Minimal UI, essential features only
2. **Speed to Market**: Ship fast, iterate based on user feedback
3. **Mobile-First**: Optimize for iOS touch experience
4. **Clean Design**: TailwindCSS utilities, avoid custom styling
5. **Separation of Concerns**: Clear boundaries between layers

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

## API Layer Usage (Frontend)

The frontend uses an API layer to communicate with the Rust backend. **Never call `invoke()` directly** - always use the API modules.

### Using API Modules

```typescript
// ✅ Correct - use API modules
import * as todoApi from '$lib/api/todoApi';
import * as categoryApi from '$lib/api/categoryApi';

const items = await todoApi.getItems(categoryId);
const newItem = await todoApi.addItem('Buy milk', categoryId);
await todoApi.toggleItem(itemId);

const categories = await categoryApi.getCategories();
const newCategory = await categoryApi.addCategory('Work');
```

```typescript
// ❌ Wrong - direct invoke call
import { invoke } from '@tauri-apps/api/core';
const items = await invoke('get_items', { categoryId });
```

### Available API Functions

**todoApi.ts**:
- `getItems(categoryId)` - Get all items in a category
- `addItem(text, categoryId)` - Create a new item
- `toggleItem(id)` - Toggle item completion
- `deleteItem(id)` - Delete an item
- `editItem(id, text)` - Update item text
- `updateItemMemo(id, memo)` - Update item memo
- `reorderItems(itemIds)` - Reorder items
- `resetAllItems(categoryId)` - Reset all items' done status
- `checkAndAutoReset()` - Auto-reset if new day

**categoryApi.ts**:
- `getCategories()` - Get all categories
- `addCategory(name)` - Create a new category
- `editCategory(id, name)` - Update category name
- `deleteCategory(id)` - Delete a category
- `reorderCategories(categoryIds)` - Reorder categories

**settingsApi.ts**:
- `getSetting(key)` - Get a setting value
- `setSetting(key, value)` - Set a setting value

## Rust Backend Architecture

### Layer Responsibilities

1. **Models** (`models/`): Pure data structures with Serde serialization
2. **Repository** (`repository/`): Direct database operations (CRUD)
3. **Service** (`service/`): Business logic that may combine multiple repositories
4. **Commands** (`commands/`): Tauri command handlers, thin wrappers around services

### Adding a New Feature

1. **Add model** (if needed) in `models/`
2. **Add repository methods** in `repository/`
3. **Add service methods** (if business logic needed) in `service/`
4. **Add Tauri command** in `commands/`
5. **Export command** in `lib.rs` invoke_handler
6. **Add API function** in frontend `lib/api/`
7. **Use in component** via API module

### Example: Adding a New Command

```rust
// 1. commands/todo_commands.rs
#[tauri::command]
pub fn my_new_command(id: i64, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    TodoService::my_new_operation(&db, id).map_err(|e| e.to_string())
}

// 2. lib.rs - add to invoke_handler
.invoke_handler(tauri::generate_handler![
    // ... existing commands
    my_new_command,
])
```

```typescript
// 3. lib/api/todoApi.ts
export async function myNewOperation(id: number): Promise<void> {
  return invoke<void>('my_new_command', { id });
}
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

### Frontend Guidelines
- One component per file
- Use Svelte 5 runes syntax
- Keep components small and focused
- **Always use API layer** for backend communication
- Prefer composition over complexity

### Backend Guidelines
- Keep commands thin (delegate to services)
- Put business logic in services
- Put SQL operations in repositories
- Use transactions for multi-step operations

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

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
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
- ✅ Layered architecture (Repository → Service → Commands)
- ✅ Frontend API layer

## Next Steps

1. Prepare App Store assets (icon, screenshots, description)
2. Submit to App Store
3. Iterate based on user feedback

Keep it simple. Ship fast. Iterate based on real user feedback.
