<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';
  import { fly, fade } from 'svelte/transition';
  import { beforeNavigate } from '$app/navigation';
  import { cubicOut } from 'svelte/easing';

  let direction = $state(1); // 1: 오른쪽에서, -1: 왼쪽에서
  let hasNavigated = $state(false); // 네비게이션 발생 여부

  // 경로 깊이 계산
  function getDepth(path: string): number {
    if (path === '/') return 0;
    return path.split('/').filter(Boolean).length;
  }

  beforeNavigate(({ from, to }) => {
    hasNavigated = true;

    if (!from?.url || !to?.url) return;

    const fromDepth = getDepth(from.url.pathname);
    const toDepth = getDepth(to.url.pathname);

    // 하위 페이지로 가면 오른쪽에서, 상위로 가면 왼쪽에서
    direction = toDepth > fromDepth ? 1 : -1;
  });

  // 첫 로드: fade, 이후: fly
  function transitionIn(node: Element) {
    const el = node as HTMLElement;
    el.style.zIndex = '1'; // 들어오는 페이지가 위에

    if (!hasNavigated) {
      return fade(node, { duration: 3000 });
    }
    return fly(node, { x: direction * 60, duration: 600, easing: cubicOut });
  }

  function transitionOut(node: Element) {
    const el = node as HTMLElement;
    el.style.zIndex = '0'; // 나가는 페이지가 아래에
    el.style.pointerEvents = 'none'; // 클릭 방지

    if (!hasNavigated) {
      return fade(node, { duration: 0 });
    }
    return fly(node, { x: direction * -60, duration: 550, easing: cubicOut });
  }
</script>

{#key $page.url.pathname}
  <div
    class="page-wrapper"
    in:transitionIn
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
