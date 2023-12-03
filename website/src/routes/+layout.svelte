<script>
import Navbar from "$lib/components/Navbar.svelte";
  import "../app.css";
</script>

<div class="bg-zinc-900 text-zinc-50 font-primary pt-16 flex items-center flex-col">
  <Navbar></Navbar>
    <slot />
</div>
