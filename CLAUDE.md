# CLAUDE.md

This file provides guidance to Claude Code when working with code in this repository.

## Project Overview

**Tickly** is a minimalist checklist app for everyday use. The core philosophy is **simplicity and speed** - no complex features, just a clean and focused interface that lets users manage their tasks quickly.

### Primary Goal
Deploy to iOS App Store as quickly as possible with a simple, functional design.

## Project Architecture

### Tech Stack
- **Frontend**: SvelteKit with TypeScript, Svelte 5 (runes syntax)
- **Backend**: Rust with Tauri v2 framework
- **Database**: SQLite (rusqlite) + Supabase (PostgreSQL)
- **Cloud**: Supabase (authentication, data sync)
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
│   │       ├── theme/
│   │       │   └── +page.svelte      # Theme customization page
│   │       ├── language/
│   │       │   └── +page.svelte      # Language settings page
│   │       └── account/
│   │           └── +page.svelte      # Cloud sync & account page
│   ├── components/                   # Reusable Svelte components
│   │   ├── ModalWrapper.svelte       # Common modal layout
│   │   ├── SettingsLayout.svelte     # Common settings page layout
│   │   ├── BottomNav.svelte          # Bottom navigation bar
│   │   ├── FloatingActions.svelte    # FAB buttons (add, reset)
│   │   └── ...                       # Other components
│   ├── lib/
│   │   ├── api/                      # API Layer (Tauri invoke wrappers)
│   │   │   ├── index.ts              # Re-exports
│   │   │   ├── client.ts             # Base invoke wrapper
│   │   │   ├── categoryApi.ts        # Category API functions
│   │   │   ├── todoApi.ts            # Todo API functions
│   │   │   ├── settingsApi.ts        # Settings API functions
│   │   │   ├── streakApi.ts          # Streak tracking API functions
│   │   │   ├── authApi.ts            # Auth API functions
│   │   │   └── syncApi.ts            # Sync API functions
│   │   ├── stores/                   # Svelte 5 reactive stores
│   │   │   ├── index.ts              # Re-exports
│   │   │   ├── appStore.svelte.ts    # App state (categories, items)
│   │   │   ├── modalStore.svelte.ts  # Modal visibility state
│   │   │   ├── authStore.svelte.ts   # Auth state (login, user)
│   │   │   └── syncStore.svelte.ts   # Sync state (status, pending)
│   │   ├── i18n/                     # Internationalization
│   │   │   ├── index.ts              # Re-exports
│   │   │   ├── i18nStore.svelte.ts   # i18n store with locale state
│   │   │   ├── ko.ts                 # Korean translations
│   │   │   └── en.ts                 # English translations
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
│   │   │   ├── todo_item.rs          # TodoItem struct
│   │   │   └── completion_log.rs     # CompletionLog, HeatmapData structs
│   │   ├── repository/               # Data access layer
│   │   │   ├── mod.rs
│   │   │   ├── database.rs           # DB initialization
│   │   │   ├── migration.rs          # Schema migrations
│   │   │   ├── category_repo.rs      # Category CRUD
│   │   │   ├── todo_repo.rs          # Todo CRUD
│   │   │   ├── settings_repo.rs      # Settings CRUD
│   │   │   └── completion_log_repo.rs # Completion log CRUD
│   │   ├── service/                  # Business logic layer
│   │   │   ├── mod.rs
│   │   │   ├── category_service.rs   # Category business logic
│   │   │   ├── todo_service.rs       # Todo business logic
│   │   │   ├── reset_service.rs      # Reset/auto-reset logic
│   │   │   ├── repeat_service.rs     # Repeat rules logic
│   │   │   ├── streak_service.rs     # Streak tracking logic
│   │   │   ├── auth_service.rs       # Auth (Apple/Google sign in)
│   │   │   ├── oauth_service.rs      # OAuth PKCE flow
│   │   │   ├── sync_service.rs       # Cloud sync (push/pull)
│   │   │   └── supabase_client.rs    # Supabase REST API client
│   │   └── commands/                 # Tauri command handlers
│   │       ├── mod.rs
│   │       ├── category_commands.rs  # Category Tauri commands
│   │       ├── todo_commands.rs      # Todo Tauri commands
│   │       ├── settings_commands.rs  # Settings Tauri commands
│   │       ├── streak_commands.rs    # Streak Tauri commands
│   │       ├── auth_commands.rs      # Auth Tauri commands
│   │       └── sync_commands.rs      # Sync Tauri commands
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

**streakApi.ts**:
- `getTrackedItems()` - Get all items with streak tracking enabled
- `getItemHeatmapData(itemId)` - Get heatmap data for a specific item
- `updateTrackStreak(id, trackStreak)` - Enable/disable streak tracking for an item

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

## Common Components

The project uses shared layout components to maintain consistency and reduce code duplication.

### ModalWrapper

All modals use `ModalWrapper.svelte` for consistent styling and behavior:

```svelte
<script lang="ts">
  import ModalWrapper from './ModalWrapper.svelte';
</script>

<ModalWrapper show={isOpen} onClose={handleClose} size="sm" position="center">
  <!-- Modal content here -->
</ModalWrapper>
```

**Props:**
- `show`: boolean - Controls visibility
- `onClose`: () => void - Called when clicking overlay or pressing Escape
- `size`: 'sm' | 'md' - Modal width (default: 'sm')
- `position`: 'center' | 'top' - Vertical alignment (default: 'center')

**Features:**
- Slide-up animation (fly transition)
- Fade overlay
- Keyboard support (Escape to close)
- Click outside to close

### SettingsLayout

Settings pages use `SettingsLayout.svelte` for consistent header and layout:

```svelte
<script lang="ts">
  import SettingsLayout from '../../components/SettingsLayout.svelte';
</script>

<SettingsLayout title={i18n.t('settingsTitle')} onBack={() => goto('/')}>
  <!-- Page content here -->

  {#snippet footer()}
    <!-- Optional fixed footer -->
  {/snippet}
</SettingsLayout>
```

## Internationalization (i18n)

The app supports Korean and English with a reactive i18n system.

### Usage

```typescript
import { i18n } from '$lib/i18n';

// Get translation
const text = i18n.t('settingsTitle');

// Get current locale
const locale = i18n.locale; // 'ko' | 'en'

// Change locale
await i18n.setLocale('en');
```

### Adding Translations

1. Add key to `src/lib/i18n/ko.ts`:
```typescript
export const ko = {
  // ...existing keys
  newKey: '새로운 텍스트',
};
```

2. Add same key to `src/lib/i18n/en.ts`:
```typescript
export const en: Translations = {
  // ...existing keys
  newKey: 'New text',
};
```

### Template Functions

For dynamic text with parameters:

```typescript
// In ko.ts
categoryDeleteConfirmTemplate: (name: string) =>
  `"${name}" 카테고리를 삭제하시겠습니까?`,

// Usage
i18n.t('categoryDeleteConfirmTemplate')(categoryName)
```

## Store Pattern

The app uses Svelte 5 reactive stores for state management.

### appStore

Manages app-wide state (categories, items, selected category):

```typescript
import { appStore } from '$lib/stores';

// Read state
appStore.categories  // Category[]
appStore.items       // TodoItem[]
appStore.selectedCategoryId  // number | null

// Actions
await appStore.loadCategories();
await appStore.addItem(text, memo);
await appStore.toggleItem(id);
appStore.selectCategory(id);
```

### modalStore

Manages modal visibility state:

```typescript
import { modalStore } from '$lib/stores';

// Check state
modalStore.showAddItemModal
modalStore.showResetConfirm
modalStore.showCategoryMenu

// Actions
modalStore.openAddItemModal();
modalStore.closeAddItemModal();
modalStore.openCategoryMenu(category);
```

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

## Cloud Sync Architecture

### Overview

Cloud sync uses Supabase (PostgreSQL + REST API) with the following flow:

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Local     │ --> │    Sync     │ --> │  Supabase   │
│   SQLite    │ <-- │   Service   │ <-- │  PostgreSQL │
└─────────────┘     └─────────────┘     └─────────────┘
```

### Sync Fields

All syncable entities (todos, categories) have:
- `sync_id`: Supabase UUID (NULL if not yet synced)
- `created_at`, `updated_at`: ISO 8601 timestamps
- `sync_status`: 'pending' | 'synced' | 'deleted'

### Sync Flow

1. **Push Phase**: Local changes → Supabase
   - Items with `sync_status = 'pending'` are upserted
   - Items with `sync_status = 'deleted'` are deleted from server

2. **Pull Phase**: Supabase → Local
   - Compare `updated_at` timestamps
   - Newer remote data overwrites local

3. **Delete Handling** (Soft Delete Pattern):
   - Delete in UI → `sync_status = 'deleted'` (not actually deleted)
   - Query filters exclude `sync_status = 'deleted'`
   - After sync push → permanent local delete

### Authentication

**Apple Sign In (iOS)**:
- `tauri-plugin-sign-in-with-apple` → native iOS 다이얼로그
- identity token → Supabase `sign_in_with_apple` API

**Google Sign In (Desktop)**:
- `tauri-plugin-oauth` → localhost 콜백 서버 시작
- OAuth PKCE flow (code_verifier + code_challenge)
- 브라우저에서 Google 로그인 → localhost 리다이렉트 → code 수신

**Google Sign In (iOS/Android)**:
- `tauri-plugin-deep-link` → `tickly://auth/callback` 딥 링크
- OAuth PKCE flow (동일)
- 브라우저에서 Google 로그인 → 딥 링크 리다이렉트 → code 수신
- `+layout.svelte`의 `onOpenUrl` 리스너에서 콜백 처리

**플랫폼 감지**:
- `navigator.userAgent` 기반 (`@tauri-apps/plugin-os` 사용하지 않음)
- iOS: `signInWithGoogleMobile()` (딥 링크)
- Desktop: `signInWithGoogleDesktop()` (localhost)

### Key Services

**`auth_service.rs`**: Apple Sign In, Google OAuth, session management
**`oauth_service.rs`**: OAuth PKCE flow, code exchange
**`sync_service.rs`**: Push/pull logic, conflict resolution
**`supabase_client.rs`**: REST API calls (reqwest)

### Environment Variables

Required in `.env` (loaded at build time for iOS):
```
SUPABASE_URL=https://xxx.supabase.co
SUPABASE_ANON_KEY=eyJ...
```

### iOS Build Note

iOS cannot read `.env` at runtime. Environment variables are injected at build time via `build.rs`:

```rust
// build.rs
if let Ok(url) = std::env::var("SUPABASE_URL") {
    println!("cargo:rustc-env=SUPABASE_URL={}", url);
}
```

### Stores (Frontend)

**`authStore.svelte.ts`**: Login state, user profile
**`syncStore.svelte.ts`**: Sync status, last synced time, pending count

### Troubleshooting

See `docs/troubleshooting.md` for common issues:
- Apple Sign In permissions
- iOS environment variables
- Deleted items reappearing
- Category ID mapping on first sync
- Google OAuth plugin import errors (`plugin-os`, `plugin-opener`)
- iOS deep link configuration for OAuth callback

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

### Core Features
- ✅ Todo CRUD (add, edit, delete, toggle)
- ✅ Category management (add, edit, delete, reorder)
- ✅ Item reordering (drag & drop)
- ✅ Swipe to delete
- ✅ Memo for each item
- ✅ Repeat rules (daily/weekly/monthly scheduling)
- ✅ Streak tracking per item with heatmap visualization
- 🚧 Cloud sync (Supabase) - Apple Sign In, Google Sign In, manual sync 완료

### UI/UX
- ✅ Theme customization (5 presets + custom colors)
- ✅ Font customization
- ✅ Internationalization (Korean/English)
- ✅ iOS-optimized UI with safe area handling
- ✅ Unified modal styles with slide-up animation
- ✅ Common layout components (SettingsLayout, ModalWrapper)
- ✅ Bottom navigation bar
- ✅ Floating action buttons

### Architecture
- ✅ Layered backend (Repository → Service → Commands)
- ✅ Frontend API layer
- ✅ Svelte 5 reactive stores (appStore, modalStore, authStore, syncStore)
- ✅ i18n system with reactive locale
- ✅ Supabase integration (auth, sync)

## Roadmap

### Current Focus
1. Prepare App Store assets (icon, screenshots, description)
2. Submit to App Store

### Completed Versions
- **v0.2.0**: ✅ Repeat rules (daily/weekly/monthly scheduling)
- **v0.3.0**: ✅ Streak heatmap (per-item tracking with GitHub-style visualization)

### In Progress
- **v0.4.0**: 🚧 Cloud sync
  - ✅ Apple Sign In (iOS)
  - ✅ Google Sign In (Desktop + iOS/Android)
  - ✅ Manual sync (push/pull)
  - ⬜ Realtime sync (WebSocket)

### Planned Features (see `docs/roadmap.md` for details)
- **v0.5.0**: Shared lists (family/team collaboration)
- **v0.6.0**: iOS widgets

### Notes
- Notifications on hold due to Tauri compatibility issues

Keep it simple. Ship fast. Iterate based on real user feedback.
