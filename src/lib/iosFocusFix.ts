export function iosFocusFix(node: HTMLElement) {
  const onFocus = () => {
    node.classList.remove('ios-focus-fix'); // 재적용 보장
    // 다음 tick에 붙여야 애니메이션이 다시 돈다
    requestAnimationFrame(() => node.classList.add('ios-focus-fix'));
  };

  const onBlur = () => node.classList.remove('ios-focus-fix');

  node.addEventListener('focus', onFocus);
  node.addEventListener('blur', onBlur);

  return {
    destroy() {
      node.removeEventListener('focus', onFocus);
      node.removeEventListener('blur', onBlur);
    }
  };
}
