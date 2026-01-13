export function iosFocusFix(node: HTMLInputElement | HTMLTextAreaElement) {
  // iOS auto-scroll prevention using readonly trick
  node.setAttribute('readonly', 'readonly');

  const onFocus = () => {
    // Remove readonly on focus to allow typing
    node.removeAttribute('readonly');

    // Also add the animation class as backup
    node.classList.remove('ios-focus-fix');
    requestAnimationFrame(() => node.classList.add('ios-focus-fix'));
  };

  const onBlur = () => {
    // Restore readonly when not focused
    node.setAttribute('readonly', 'readonly');
    node.classList.remove('ios-focus-fix');
  };

  node.addEventListener('focus', onFocus);
  node.addEventListener('blur', onBlur);

  return {
    destroy() {
      node.removeEventListener('focus', onFocus);
      node.removeEventListener('blur', onBlur);
      node.removeAttribute('readonly');
    }
  };
}
