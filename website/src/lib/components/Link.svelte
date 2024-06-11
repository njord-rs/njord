<script>
  let clazz = '';
  export { clazz as class };
  export let href = '';
  export let ariaLabel = '';
</script>

{#if href.startsWith('http')}
  <a
    class={clazz}
    {href}
    target="_blank"
    rel="noopener noreferrer"
    aria-label={ariaLabel}
  >
    <slot />
  </a>
{:else}
  <a class={clazz} {href} aria-label={ariaLabel}>
    <slot />
  </a>
{/if}
