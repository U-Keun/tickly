<script lang="ts">
  import { i18n } from '$lib/i18n';

  let {
    userName,
    userEmail,
    avatarUrl = null,
    onLogoutRequest
  } = $props<{
    userName: string;
    userEmail: string;
    avatarUrl?: string | null;
    onLogoutRequest: () => void;
  }>();
</script>

<div class="account-section">
  <div class="user-info">
    <div class="avatar">
      {#if avatarUrl}
        <img src={avatarUrl} alt="Avatar" />
      {:else}
        <div class="avatar-placeholder">
          <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
          </svg>
        </div>
      {/if}
    </div>
    <div class="user-details">
      <p class="user-name">{userName}</p>
      <p class="user-email">{userEmail}</p>
    </div>
  </div>

  <button class="logout-btn" onclick={onLogoutRequest}>
    {i18n.t('logout')}
  </button>
</div>

<style>
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
</style>
