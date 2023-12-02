<script>
    export let items = [];
  </script>
  
  <div class="relative group/nav">
    <slot />
    <div class="group-hover/nav:flex hover:flex hidden absolute">
      <div
        class="bg-zinc-800 text-zinc-50 rounded-md py-2 px-2 flex-col flex gap-2 mt-4 w-52"
      >
        {#each items as item}
          <a
            href={item.link}
            class={"relative rounded-md overflow-hidden group/item hover:bg-zinc-500 hover:bg-gradient-to-r hover:from-zinc-500 hover:to-zinc-400 text-white transition-all ease-out duration-300 " +
              "p-2"}
          >
            <span
              class="absolute right-0 w-8 h-44 -mt-32 transition-all duration-1000 transform translate-x-32 bg-white opacity-10 rotate-12 group-hover/item:-translate-x-40 ease"
            />
            <span class="relative flex items-center gap-2">{item.text}</span>
          </a>
        {/each}
      </div>
    </div>
  </div>
  