<script>
  import GithubBar from "$lib/components/GithubBar.svelte";
import Navbar from "$lib/components/Navbar.svelte";
  import "../app.css";
  export let data;
</script>

<div class="bg-zinc-900 text-zinc-100 font-Primary pt-16 flex items-center flex-col" style="height: 100vh">
  <Navbar></Navbar>
  <GithubBar repo={data.repo}></GithubBar>
    <slot />
</div>
