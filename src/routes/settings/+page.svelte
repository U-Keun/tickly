<script lang="ts">
  import { goto } from '$app/navigation';
  import { i18n } from '$lib/i18n';
  import SettingsLayout from '../../components/SettingsLayout.svelte';
</script>

<SettingsLayout title={i18n.t('settingsTitle')} onBack={() => goto('/')}>
  <div class="settings-list">
    <!-- Theme Setting -->
    <button class="settings-item" onclick={() => goto('/settings/theme')}>
      <div class="item-left">
        <div class="item-icon">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
          </svg>
        </div>
        <span class="item-label">{i18n.t('themeChange')}</span>
      </div>
      <svg class="w-5 h-5 chevron" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </button>

    <!-- Language Setting -->
    <button class="settings-item" onclick={() => goto('/settings/language')}>
      <div class="item-left">
        <div class="item-icon language-icon">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129" />
          </svg>
        </div>
        <span class="item-label">{i18n.t('languageChange')}</span>
      </div>
      <svg class="w-5 h-5 chevron" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </button>
  </div>
</SettingsLayout>

<style>
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

  .item-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .item-icon {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-accent-sky);
    border-radius: 8px;
    color: var(--color-ink);
  }

  .language-icon {
    background: var(--color-accent-mint);
  }

  .item-label {
    font-size: 15px;
    color: var(--color-ink);
  }

  .chevron {
    color: var(--color-ink-muted);
  }
</style>
