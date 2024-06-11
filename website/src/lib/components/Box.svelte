<script>
  export let text = '';
</script>

<div class="rounded-xl flex flex-col border border-secondary p-8 gap-8">
  <slot />
  <p class="text-sm font-medium leading-relaxed">{text}</p>
</div>
