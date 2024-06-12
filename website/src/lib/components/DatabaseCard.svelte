<script>
  import Database from '$lib/components/icons/Database.svelte';

  export let label = 'database';
  export let isReady = false;

  const readyStyles = isReady ? '' : 'opacity-30';
</script>

<div class={`flex flex-col items-center gap-1 ${readyStyles}`}>
  <Database class="text-primary" />
  <p class="text-sm font-semibold">{label}</p>
</div>
