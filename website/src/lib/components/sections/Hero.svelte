<script>
  import GithubBar from '$lib/components/GithubBar.svelte';
  import DatabaseBar from '$lib/components/DatabaseBar.svelte';
  export let data;
</script>

<section class="relative w-full h-[50vh] py-20">
  <img
    src="/banner.png"
    class="opacity-5 absolute w-full h-full top-0 left-0 object-cover pointer-events-none"
    alt="Viking city"
  />

  <div class="fade-out pointer-events-none bg-gradient-to-b from-transparent to-zinc-200 dark:to-zinc-900"></div>

  <div class="container">
    <div>
      <h1 class="header text-8xl mb-6">Njord</h1>

      <p class="text-xl leading-relaxed">
        Lightweight, Extensible ORM and Query Builder for Rust
      </p>
    </div>

    <GithubBar repo={data.repo}></GithubBar>
    <DatabaseBar></DatabaseBar>
  </div>
</section>

<style lang="postcss">
  .fade-out {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 100px;
    /* background: linear-gradient(to bottom, rgba(255, 255, 255, 0), #18181b); */
  }
</style>
