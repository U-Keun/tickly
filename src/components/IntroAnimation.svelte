<script lang="ts">
  import { onMount } from 'svelte';

  let showIntro = $state(false);
  let showWhite = $state(false);
  let introText = $state('');

  async function playIntroAnimation() {
    const fullText = 'Tickly';
    for (let i = 0; i <= fullText.length; i++) {
      introText = fullText.slice(0, i);
      await new Promise(resolve => setTimeout(resolve, 150));
    }
    await new Promise(resolve => setTimeout(resolve, 500));
    showIntro = false;
    showWhite = true;
    await new Promise(resolve => setTimeout(resolve, 400));
    showWhite = false;
  }

  onMount(() => {
    // 세션 내 첫 로드시에만 애니메이션 실행
    if (sessionStorage.getItem('introPlayed')) {
      return;
    }
    sessionStorage.setItem('introPlayed', 'true');
    showIntro = true;
    playIntroAnimation();
  });
</script>

<!-- Intro Typing Animation -->
{#if showIntro}
  <div class="fixed inset-0 bg-paper z-50 flex items-center justify-center">
    <h1 class="text-5xl font-bold text-ink">{introText}<span class="animate-pulse">|</span></h1>
  </div>
{/if}
{#if showWhite}
    <div class="fixed inset-0 bg-paper z-50"></div>
{/if}
