<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { i18n } from '$lib/i18n';
  import { authStore, syncStore } from '$lib/stores';
  import SettingsLayout from '../../../components/SettingsLayout.svelte';
  import ModalWrapper from '../../../components/ModalWrapper.svelte';
  let showLogoutConfirm = $state(false);
  let syncError = $state<string | null>(null);
  let loginError = $state<string | null>(null);
  let isSigningIn = $state(false);
  let isDesktop = $state(false);

  onMount(async () => {
    // Check if we're on Desktop by checking user agent
    // iOS will have 'iPhone' or 'iPad' in the user agent
    const userAgent = navigator.userAgent.toLowerCase();
    const isIOS = /iphone|ipad|ipod/.test(userAgent);
    isDesktop = !isIOS;

    await authStore.checkSession();
    await syncStore.loadStatus();
  });

  async function handleAppleSignIn() {
    isSigningIn = true;
    loginError = null;
    try {
      await authStore.signInWithApple();
      await syncStore.loadStatus();
      // Connect to realtime after successful login
      if (authStore.session) {
        await syncStore.connectRealtime(authStore.session.access_token, authStore.session.user_id);
      }
    } catch (error) {
      console.error('Apple Sign In failed:', error);
      loginError = error instanceof Error ? error.message : String(error);
    } finally {
      isSigningIn = false;
    }
  }

  async function handleGoogleSignIn() {
    isSigningIn = true;
    loginError = null;
    try {
      // Use appropriate method based on platform
      if (isDesktop) {
        await authStore.signInWithGoogleDesktop();
      } else {
        await authStore.signInWithGoogleMobile();
      }
      await syncStore.loadStatus();
      // Connect to realtime after successful login
      if (authStore.session) {
        await syncStore.connectRealtime(authStore.session.access_token, authStore.session.user_id);
      }
    } catch (error) {
      console.error('Google Sign In failed:', error);
      loginError = error instanceof Error ? error.message : String(error);
    } finally {
      isSigningIn = false;
    }
  }

  async function handleSync() {
    syncError = null;
    const result = await syncStore.sync();
    if (!result && syncStore.error) {
      syncError = syncStore.error;
    }
  }

  async function handleLogout() {
    showLogoutConfirm = false;
    await authStore.signOut();
    await syncStore.loadStatus();
  }

  async function handleToggleSync() {
    await syncStore.setEnabled(!syncStore.isEnabled);
  }
</script>

<SettingsLayout title={i18n.t('cloudSync')} onBack={() => goto('/settings')}>
  <div class="account-content">
    {#if authStore.isLoading}
      <div class="loading-state">
        <p>{i18n.t('loading')}</p>
      </div>
    {:else if !authStore.isLoggedIn}
      <!-- Not logged in state -->
      <div class="login-section">
        <div class="login-header">
          <div class="cloud-icon">
            <svg class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" />
            </svg>
          </div>
          <h2>{i18n.t('loginRequired')}</h2>
          <p>{i18n.t('loginDescription')}</p>
        </div>

        <div class="login-buttons">
          <button
            class="login-btn apple-btn"
            onclick={handleAppleSignIn}
            disabled={isSigningIn}
          >
            {#if isSigningIn}
              <span class="login-spinner"></span>
            {:else}
              <svg class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor">
                <path d="M17.05 20.28c-.98.95-2.05.8-3.08.35-1.09-.46-2.09-.48-3.24 0-1.44.62-2.2.44-3.06-.35C2.79 15.25 3.51 7.59 9.05 7.31c1.35.07 2.29.74 3.08.8 1.18-.24 2.31-.93 3.57-.84 1.51.12 2.65.72 3.4 1.8-3.12 1.87-2.38 5.98.48 7.13-.57 1.5-1.31 2.99-2.54 4.09l.01-.01zM12.03 7.25c-.15-2.23 1.66-4.07 3.74-4.25.29 2.58-2.34 4.5-3.74 4.25z"/>
              </svg>
            {/if}
            <span>{i18n.t('signInWithApple')}</span>
          </button>

          <button
            class="login-btn google-btn"
            onclick={handleGoogleSignIn}
            disabled={isSigningIn}
          >
            {#if isSigningIn}
              <span class="login-spinner google-spinner"></span>
            {:else}
              <svg class="w-5 h-5" viewBox="0 0 24 24">
                <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
                <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
                <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
                <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
              </svg>
            {/if}
            <span>{i18n.t('signInWithGoogle')}</span>
          </button>

          {#if loginError}
            <p class="login-error">{loginError}</p>
          {/if}
        </div>
      </div>
    {:else}
      <!-- Logged in state -->
      <div class="account-section">
        <div class="user-info">
          <div class="avatar">
            {#if authStore.user?.avatar_url}
              <img src={authStore.user.avatar_url} alt="Avatar" />
            {:else}
              <div class="avatar-placeholder">
                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                </svg>
              </div>
            {/if}
          </div>
          <div class="user-details">
            <p class="user-name">{authStore.user?.full_name || authStore.user?.email || 'User'}</p>
            <p class="user-email">{authStore.user?.email || ''}</p>
          </div>
        </div>

        <button class="logout-btn" onclick={() => showLogoutConfirm = true}>
          {i18n.t('logout')}
        </button>
      </div>

      <div class="sync-section">
        <h3>{i18n.t('syncTitle')}</h3>

        <div class="sync-toggle">
          <span>{i18n.t('syncEnabled')}</span>
          <button
            class="toggle-switch"
            class:active={syncStore.isEnabled}
            onclick={handleToggleSync}
          >
            <span class="toggle-thumb"></span>
          </button>
        </div>

        {#if syncStore.isEnabled}
          <div class="sync-status">
            <div class="status-row">
              <span class="status-label">{i18n.t('lastSynced')}</span>
              <span class="status-value">{syncStore.formatLastSyncedAt() || i18n.t('never')}</span>
            </div>
            <div class="status-row">
              <span class="status-label">{i18n.t('pendingChanges')}</span>
              <span class="status-value">{syncStore.pendingCount}</span>
            </div>
            <div class="status-row">
              <span class="status-label">{i18n.t('realtimeSync')}</span>
              <span class="status-value realtime-status" class:connected={syncStore.realtimeState === 'connected'} class:connecting={syncStore.realtimeState === 'connecting' || syncStore.realtimeState === 'reconnecting'}>
                {#if syncStore.realtimeState === 'connected'}
                  <span class="realtime-dot connected"></span> {i18n.t('realtimeConnected')}
                {:else if syncStore.realtimeState === 'connecting'}
                  <span class="realtime-dot connecting"></span> {i18n.t('realtimeConnecting')}
                {:else if syncStore.realtimeState === 'reconnecting'}
                  <span class="realtime-dot connecting"></span> {i18n.t('realtimeReconnecting')}
                {:else}
                  <span class="realtime-dot disconnected"></span> {i18n.t('realtimeDisconnected')}
                {/if}
              </span>
            </div>
          </div>

          <button
            class="sync-btn"
            onclick={handleSync}
            disabled={syncStore.isSyncing}
          >
            {#if syncStore.isSyncing}
              <span class="sync-spinner"></span>
              {i18n.t('syncing')}
            {:else}
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
              {i18n.t('syncNow')}
            {/if}
          </button>

          {#if syncError}
            <p class="sync-error">{syncError}</p>
          {/if}
        {/if}
      </div>
    {/if}
  </div>
</SettingsLayout>

<!-- Logout confirmation modal -->
<ModalWrapper show={showLogoutConfirm} onClose={() => showLogoutConfirm = false}>
  <div class="confirm-modal">
    <h3>{i18n.t('logout')}</h3>
    <p>{i18n.t('logoutConfirm')}</p>
    <div class="modal-buttons">
      <button class="cancel-btn" onclick={() => showLogoutConfirm = false}>
        {i18n.t('cancel')}
      </button>
      <button class="confirm-btn" onclick={handleLogout}>
        {i18n.t('logout')}
      </button>
    </div>
  </div>
</ModalWrapper>

<style>
  .account-content {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .loading-state {
    text-align: center;
    padding: 40px 20px;
    color: var(--color-ink-muted);
  }

  /* Login section */
  .login-section {
    background: var(--color-canvas);
    border-radius: 12px;
    padding: 32px 20px;
    text-align: center;
  }

  .login-header {
    margin-bottom: 24px;
  }

  .cloud-icon {
    width: 64px;
    height: 64px;
    margin: 0 auto 16px;
    background: var(--color-accent-sky);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-ink);
  }

  .login-header h2 {
    font-size: 18px;
    font-weight: 600;
    color: var(--color-ink);
    margin-bottom: 8px;
  }

  .login-header p {
    font-size: 14px;
    color: var(--color-ink-muted);
  }

  .login-buttons {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .login-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    width: 100%;
    padding: 14px 20px;
    border: none;
    border-radius: 10px;
    font-size: 15px;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .login-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .apple-btn {
    background: #000;
    color: #fff;
  }

  .google-btn {
    background: #fff;
    color: #333;
    border: 1px solid var(--color-stroke);
  }

  .login-spinner {
    width: 20px;
    height: 20px;
    border: 2px solid rgba(255,255,255,0.3);
    border-top-color: #fff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .login-spinner.google-spinner {
    border-color: rgba(0,0,0,0.1);
    border-top-color: #4285F4;
  }

  .login-error {
    margin-top: 12px;
    padding: 12px;
    background: var(--color-accent-peach);
    border-radius: 8px;
    color: var(--color-accent-peach-strong);
    font-size: 14px;
    text-align: left;
  }

  /* Account section */
  .account-section {
    background: var(--color-canvas);
    border-radius: 12px;
    padding: 20px;
  }

  .user-info {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 16px;
  }

  .avatar {
    width: 56px;
    height: 56px;
    border-radius: 50%;
    overflow: hidden;
    flex-shrink: 0;
  }

  .avatar img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .avatar-placeholder {
    width: 100%;
    height: 100%;
    background: var(--color-accent-sky);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-ink);
  }

  .user-details {
    flex: 1;
    min-width: 0;
  }

  .user-name {
    font-size: 16px;
    font-weight: 600;
    color: var(--color-ink);
    margin-bottom: 2px;
  }

  .user-email {
    font-size: 14px;
    color: var(--color-ink-muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .logout-btn {
    width: 100%;
    padding: 12px;
    background: none;
    border: 1px solid var(--color-accent-peach-strong);
    border-radius: 8px;
    color: var(--color-accent-peach-strong);
    font-size: 15px;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }

  .logout-btn:hover {
    background: var(--color-accent-peach);
  }

  /* Sync section */
  .sync-section {
    background: var(--color-canvas);
    border-radius: 12px;
    padding: 20px;
  }

  .sync-section h3 {
    font-size: 16px;
    font-weight: 600;
    color: var(--color-ink);
    margin-bottom: 16px;
  }

  .sync-toggle {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .sync-toggle span {
    font-size: 15px;
    color: var(--color-ink);
  }

  .toggle-switch {
    width: 51px;
    height: 31px;
    background: var(--color-stroke);
    border: none;
    border-radius: 16px;
    position: relative;
    cursor: pointer;
    transition: background 0.2s;
  }

  .toggle-switch.active {
    background: var(--color-accent-mint-strong);
  }

  .toggle-thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 27px;
    height: 27px;
    background: var(--color-white);
    border-radius: 50%;
    box-shadow: 0 2px 4px rgba(0,0,0,0.2);
    transition: transform 0.2s;
  }

  .toggle-switch.active .toggle-thumb {
    transform: translateX(20px);
  }

  .sync-status {
    background: var(--color-mist);
    border-radius: 8px;
    padding: 12px;
    margin-bottom: 16px;
  }

  .status-row {
    display: flex;
    justify-content: space-between;
    font-size: 14px;
  }

  .status-row:not(:last-child) {
    margin-bottom: 8px;
  }

  .status-label {
    color: var(--color-ink-muted);
  }

  .status-value {
    color: var(--color-ink);
    font-weight: 500;
  }

  .realtime-status {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .realtime-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .realtime-dot.connected {
    background: var(--color-accent-mint-strong);
  }

  .realtime-dot.connecting {
    background: var(--color-accent-sky-strong);
    animation: pulse 1s infinite;
  }

  .realtime-dot.disconnected {
    background: var(--color-ink-muted);
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }

  .sync-btn {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 14px;
    background: var(--color-accent-sky);
    border: none;
    border-radius: 10px;
    color: var(--color-ink);
    font-size: 15px;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .sync-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .sync-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid var(--color-ink-muted);
    border-top-color: var(--color-ink);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .sync-error {
    margin-top: 12px;
    padding: 12px;
    background: var(--color-accent-peach);
    border-radius: 8px;
    color: var(--color-accent-peach-strong);
    font-size: 14px;
  }

  /* Modal */
  .confirm-modal {
    padding: 24px;
    text-align: center;
  }

  .confirm-modal h3 {
    font-size: 18px;
    font-weight: 600;
    color: var(--color-ink);
    margin-bottom: 12px;
  }

  .confirm-modal p {
    font-size: 15px;
    color: var(--color-ink-muted);
    margin-bottom: 24px;
  }

  .modal-buttons {
    display: flex;
    gap: 12px;
  }

  .modal-buttons button {
    flex: 1;
    padding: 12px;
    border-radius: 8px;
    font-size: 15px;
    font-weight: 500;
    cursor: pointer;
  }

  .cancel-btn {
    background: var(--color-mist);
    border: none;
    color: var(--color-ink);
  }

  .confirm-btn {
    background: var(--color-accent-peach-strong);
    border: none;
    color: var(--color-white);
  }
</style>
