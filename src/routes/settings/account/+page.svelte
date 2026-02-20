<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  import { detectDesktopFromUserAgent, getErrorMessage, runSignInFlow } from '$lib/account/signInFlow';
  import { i18n } from '$lib/i18n';
  import { authStore, syncStore } from '$lib/stores';
  import AccountLoginSection from '../../../components/account/AccountLoginSection.svelte';
  import AccountLogoutModal from '../../../components/account/AccountLogoutModal.svelte';
  import AccountProfileSection from '../../../components/account/AccountProfileSection.svelte';
  import AccountSyncSection from '../../../components/account/AccountSyncSection.svelte';
  import SettingsLayout from '../../../components/SettingsLayout.svelte';

  let showLogoutConfirm = $state(false);
  let syncError = $state<string | null>(null);
  let loginError = $state<string | null>(null);
  let isSigningIn = $state(false);
  let isDesktop = $state(false);

  onMount(async () => {
    isDesktop = detectDesktopFromUserAgent(navigator.userAgent);

    await authStore.checkSession();
    await syncStore.loadStatus();
  });

  async function handleSignIn(signInAction: () => Promise<void>, providerName: string) {
    isSigningIn = true;
    loginError = null;

    try {
      await runSignInFlow({
        signIn: signInAction,
        loadSyncStatus: syncStore.loadStatus,
        connectRealtime: syncStore.connectRealtime,
        getSession: () => authStore.session,
      });
    } catch (error) {
      console.error(`${providerName} failed:`, error);
      loginError = getErrorMessage(error);
    } finally {
      isSigningIn = false;
    }
  }

  async function handleAppleSignIn() {
    await handleSignIn(() => authStore.signInWithApple(), 'Apple Sign In');
  }

  async function handleGoogleSignIn() {
    const signInAction = isDesktop
      ? () => authStore.signInWithGoogleDesktop()
      : () => authStore.signInWithGoogleMobile();
    await handleSignIn(signInAction, 'Google Sign In');
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

  function openLogoutConfirm() {
    showLogoutConfirm = true;
  }

  function closeLogoutConfirm() {
    showLogoutConfirm = false;
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
      <AccountLoginSection
        {isSigningIn}
        {loginError}
        onAppleSignIn={handleAppleSignIn}
        onGoogleSignIn={handleGoogleSignIn}
      />
    {:else}
      <AccountProfileSection
        userName={authStore.user?.full_name || authStore.user?.email || 'User'}
        userEmail={authStore.user?.email || ''}
        avatarUrl={authStore.user?.avatar_url || null}
        onLogoutRequest={openLogoutConfirm}
      />

      <AccountSyncSection
        isEnabled={syncStore.isEnabled}
        isSyncing={syncStore.isSyncing}
        realtimeState={syncStore.realtimeState}
        lastSyncedText={syncStore.formatLastSyncedAt() || i18n.t('never')}
        {syncError}
        onToggleSync={handleToggleSync}
        onSync={handleSync}
      />
    {/if}
  </div>
</SettingsLayout>

<AccountLogoutModal
  show={showLogoutConfirm}
  onClose={closeLogoutConfirm}
  onConfirm={handleLogout}
/>

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
</style>
