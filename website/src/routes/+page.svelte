<script>
  import InfoBar from '$lib/components/sections/InfoBar.svelte';
  import Hero from '$lib/components/sections/Hero.svelte';
  export let data;
</script>

<main class="w-full">
  <Hero {data} />
  <InfoBar />
</main>
