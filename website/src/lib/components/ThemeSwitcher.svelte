<script>
  import Moon from "$lib/components/icons/Moon.svelte";
  import Sun from "$lib/components/icons/Sun.svelte";

  import { browser } from "$app/environment";

  let darkMode = true;

  function handleSwitchDarkMode() {
    darkMode = !darkMode;

    localStorage.setItem("theme", darkMode ? "dark" : "light");

    darkMode
      ? document.documentElement.classList.add("dark")
      : document.documentElement.classList.remove("dark");
  }

  function handleMode() {
    if (
      localStorage.theme === "dark" ||
      (!("theme" in localStorage) &&
        window.matchMedia("(prefers-color-scheme: dark)").matches)
    ) {
      document.documentElement.classList.add("dark");
      darkMode = true;
    } else {
      document.documentElement.classList.remove("dark");
      darkMode = false;
    }
  }

  if (browser) {
    handleMode();

    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", () => {
        localStorage.removeItem("theme");
        handleMode();
      });
  }
</script>

<button
  class="p-2.5 rounded-full hover:bg-secondary flex items-center justify-center"
  on:click={handleSwitchDarkMode}
>
  {#if darkMode}
    <Sun />
  {:else}
    <Moon />
  {/if}
</button>
