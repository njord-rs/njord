<script>
  import Link from '$lib/components/Link.svelte';

  export let href;
  export let ariaLabel = '';
</script>

<Link
  class="p-2.5 rounded-full hover:bg-secondary flex items-center justify-center"
  {href}
  {ariaLabel}
>
  <slot />
</Link>
