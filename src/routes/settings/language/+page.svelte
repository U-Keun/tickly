<script lang="ts">
  import { goto } from '$app/navigation';
  import { i18n, type Locale } from '$lib/i18n';

  const languages: { id: Locale; name: string; nativeName: string }[] = [
    { id: 'ko', name: 'Korean', nativeName: '한국어' },
    { id: 'en', name: 'English', nativeName: 'English' },
  ];

  async function selectLanguage(locale: Locale) {
    await i18n.setLocale(locale);
    goto('/settings');
  }
</script>

<div class="settings-container bg-paper">
  <!-- Header -->
  <header class="settings-header">
    <button class="back-btn" onclick={() => goto('/settings')} aria-label={i18n.t('back')}>
      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
    </button>
    <h1 class="header-title">{i18n.t('languageTitle')}</h1>
    <div class="w-6"></div>
  </header>

  <!-- Content -->
  <div class="settings-content">
    <div class="settings-list">
      {#each languages as lang}
        <button
          class="settings-item"
          class:active={i18n.locale === lang.id}
          onclick={() => selectLanguage(lang.id)}
        >
          <div class="item-left">
            <span class="item-label">{lang.nativeName}</span>
          </div>
          {#if i18n.locale === lang.id}
            <svg class="w-5 h-5 check-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          {/if}
        </button>
      {/each}
    </div>
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
    min-height: 0;
  }

  .settings-list {
    background: var(--color-canvas);
    border-radius: 12px;
    overflow: hidden;
  }

  .settings-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 14px 16px;
    background: none;
    border: none;
    cursor: pointer;
    transition: background 0.2s;
  }

  .settings-item:hover {
    background: var(--color-mist);
  }

  .settings-item:not(:last-child) {
    border-bottom: 1px solid var(--color-stroke);
  }

  .settings-item.active {
    background: var(--color-mist);
  }

  .item-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .item-label {
    font-size: 15px;
    color: var(--color-ink);
  }

  .check-icon {
    color: var(--color-accent-sky-strong);
  }
</style>
