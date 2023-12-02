<script>
    import { slide } from "svelte/transition";
    import Close from "$lib/components/icons/Close.svelte";
    import Hamburger from "$lib/components/icons/Hamburger.svelte";
    let openMenu = false;
  </script>
  
  <nav
    class="fixed top-0 px-4 h-16 w-full hidden md:flex justify-center z-10 border-b border-zinc-100 font-Secondary text-lg"
  >
    <div class="container flex-row items-center flex gap-6">
        <a href="https://x.com/njord">News</a>
        <a href="https://docs.njord.rs">Book</a>
        <a href="http://docs.rs/njord/0.1.0/njord">API Docs</a>
    </div>
  </nav>
  
  <nav
    class="fixed flex flex-col items-end md:hidden p-4 bg-zinc-900 top-0 w-full z-10 border-b border-zinc-100"
  >
    <button
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
  
    {#if openMenu}
      <div
        in:slide
        out:slide
        class="w-full flex flex-col gap-6 items-center text-zinc-50 "
      >
        <a
          href="/"
          on:click={() => {
            openMenu = !openMenu;
          }}>Home</a
        >
        <a
          href="/history"
          on:click={() => {
            openMenu = !openMenu;
          }}>News</a
        >
        <a
          href="/about"
          on:click={() => {
            openMenu = !openMenu;
          }}>Book</a
        >
        <a
          href="/gallery"
          on:click={() => {
            openMenu = !openMenu;
          }}>API Docs</a
        >
      </div>
    {/if}
  </nav>
  