<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';
  import { fly } from 'svelte/transition';
  import { beforeNavigate } from '$app/navigation';
  import { cubicOut } from 'svelte/easing';

  let direction = $state(1); // 1: 오른쪽에서, -1: 왼쪽에서

  // 경로 깊이 계산
  function getDepth(path: string): number {
    if (path === '/') return 0;
    return path.split('/').filter(Boolean).length;
  }

  beforeNavigate(({ from, to }) => {
    if (!from?.url || !to?.url) return;

    const fromDepth = getDepth(from.url.pathname);
    const toDepth = getDepth(to.url.pathname);

    // 하위 페이지로 가면 오른쪽에서, 상위로 가면 왼쪽에서
    direction = toDepth > fromDepth ? 1 : -1;
  });
</script>

{#key $page.url.pathname}
  <div
    class="page-wrapper"
    in:fly={{ x: direction * 60, duration: 600, easing: cubicOut }}
    out:fly={{ x: direction * -60, duration: 550, easing: cubicOut }}
  >
    <slot />
  </div>
{/key}

<style>
  :global(body) {
    overflow: hidden;
  }

  .page-wrapper {
    position: absolute;
    width: 100%;
    height: 100%;
  }
</style>
