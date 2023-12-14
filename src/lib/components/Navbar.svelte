<script>
    import { slide } from "svelte/transition";
    import Close from "$lib/components/icons/Close.svelte";
    import Hamburger from "$lib/components/icons/Hamburger.svelte";
    let openMenu = false;
  </script>
  
  <nav
    class="fixed top-0 px-4 h-16 w-full hidden justify-center md:flex z-10 border-b border-zinc-100 bg-zinc-900 font-secondary"
  >
    <div class="container flex-row items-center justify-between flex">
        <a class="flex justify-center items-center gap-2" href="/" aria-label="Home"> <img src="logo.png" alt="Logo" class="w-12" /> Njord</a>
        <div class="flex gap-6">
          <a target="_blank" href="https://x.com/njord_rs">News</a>
          <a target="_blank" href="https://docs.rs/njord/latest/njord/">Docs</a>
          <a target="_blank" href="https://discord.com/invite/2uppTzjUHE">Discord</a>
        </div>
       
    </div>
  </nav>
  
  <nav
    class="fixed flex flex-col items-end md:hidden p-4 bg-zinc-900 top-0 w-full z-10 border-b border-zinc-100 font-secondary"
  >
  <div class="flex flex-row justify-between w-full">
    <a class="flex justify-center items-center gap-2" href="/" aria-label="Home"><img src="logo.png" alt="Logo" class="w-12" /> Njord</a>
    <button
      aria-label="Toggle menu"
      on:click={() => {
        openMenu = !openMenu;
      }}
    >
      {#if openMenu}
        <Close />
      {:else}
        <Hamburger />
      {/if}
    </button>
  </div>
  
    {#if openMenu}
      <div
        in:slide
        out:slide
        class="w-full flex flex-col gap-6 items-center "
      >
        <a
          href="/"
          on:click={() => {
            openMenu = !openMenu;
          }}>Home</a
        >
        <a
          href="https://x.com/njord_rs"
          on:click={() => {
            openMenu = !openMenu;
          }}>News</a
        >
        <a
          href="https://docs.rs/njord/latest/njord/"
          on:click={() => {
            openMenu = !openMenu;
          }}>Docs</a
        >
        <a
          href="https://discord.com/invite/2uppTzjUHE"
          on:click={() => {
            openMenu = !openMenu;
          }}>Discord</a
        >
      </div>
    {/if}
  </nav>
  
