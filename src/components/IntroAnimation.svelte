<script lang="ts">
  import { onMount } from 'svelte';

  let showIntro = $state(false);
  let introText = $state('');

  async function playIntroAnimation() {
    const fullText = 'Tickly';
    for (let i = 0; i <= fullText.length; i++) {
      introText = fullText.slice(0, i);
      await new Promise(resolve => setTimeout(resolve, 150));
    }
    await new Promise(resolve => setTimeout(resolve, 500));
    showIntro = false;
    localStorage.setItem('tickly_intro_seen', 'true');
  }

  onMount(() => {
    // Check if first time user
    const hasSeenIntro = localStorage.getItem('tickly_intro_seen');
    if (!hasSeenIntro) {
      showIntro = true;
      playIntroAnimation();
    }
  });
</script>

<!-- Intro Typing Animation -->
{#if showIntro}
  <div class="fixed inset-0 bg-white z-50 flex items-center justify-center">
    <h1 class="text-5xl font-bold text-gray-900">{introText}<span class="animate-pulse">|</span></h1>
  </div>
{/if}
