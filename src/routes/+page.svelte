<script>
  import GithubBar from "$lib/components/GithubBar.svelte";
  import Copy from "$lib/components/icons/Copy.svelte";
  import Box from "$lib/components/Box.svelte";
  import Coffee from "$lib/components/icons/Coffee.svelte";
  import Code from "$lib/components/icons/Code.svelte";
  import Alert from "$lib/components/icons/Alert.svelte";
  export let data;

  function copy(id) {
        let text = document.getElementById(id);
        navigator.clipboard.writeText(text.value)
    }
</script>

<div class="flex flex-col w-full items-center">
    <div class="relative py-36 w-full flex justify-center">
        <img src="/banner.png" class="opacity-5 absolute w-full align-top h-full top-0 bottom-0 object-cover" alt="Viking city" style="pointer-events: none">
        <div class="fade-out" style="pointer-events: none"></div>
        <div class="container px-4">
            <h1 class="header text-8xl mb-4">Njord</h1>
            <h2 class="text-2xl">Lightweight, Extensible ORM and Query Builder for Rust</h2>
            <GithubBar repo={data.repo}></GithubBar> 
        </div>

    </div>

<div class="container px-4 pb-16">
    <div class="grid grid-cols-1 md:grid-cols-2 gap-8 lg:grid-cols-3 mb-16">
        <Box text="Join our community on Discord! Connect with our developers, seek help on usage, share your ideas, and stay updated on the latest developments on the project. Your input fuels Njord's growth.">
           <Coffee></Coffee>
        </Box>
        <Box text="Njord is a FOSS project, and we welcome your contributions. Whether it's bug fixes, new features, documentation, or other improvements, your help is valuable. Check out our GitHub repository for more information.">
            <Code></Code>
         </Box>
         <Box text="Njord is currently in alpha. While it's shaping up to be a powerful ORM and Query Builder, exercise caution in production environments. Your feedback is invaluable as we work towards stability. Stay tuned for updates and improvements!">
            <Alert></Alert>
         </Box>
        </div>
    <div class="flex flex-col gap-6">
        <div>
            <p class="text-lg mb-4">Getting started</p>
            <div class="relative">
                <button class="hover:bg-zinc-800 p-2 rounded-sm absolute top-3 right-3" on:click={() => {copy("code-0")}} aria-label="Copy code"><Copy></Copy></button>
                <textarea id="code-0" class="px-6 rounded-sm bg-zinc-950 py-6 w-full overflow-hidden lg:h-56 h-64 resize-none outline-none font-secondary" spellcheck="false" readonly>
[dependencies]

# The core APIs, including the Table trait. Always
# required when using njord. The "derive" feature is only required when
# using #[derive(Table)] to make njord work with structs
# and enums defined in your crate.
njord = {'{'} version = "0.1.0", features = ["derive"] {'}'}
                </textarea>
            </div>
        </div>

        <div>
            <p class="text-lg mb-4">Setup connection</p>
            <div class="relative">
                <button class="hover:bg-zinc-800 p-2 rounded-sm absolute top-3 right-3" on:click={() => {copy("code-1")}} aria-label="Copy code"><Copy></Copy></button>
                <textarea id="code-1" class="px-6 rounded-sm bg-zinc-950 py-6 w-full overflow-hidden lg:h-36 h-40 resize-none outline-none font-secondary" spellcheck="false" readonly>
let table_row: Posts = Posts {'{'}
    title: "A post title".to_string(),
    description: "Some description for for a post".to_string(),
{'}'};
                </textarea>
            </div>
        </div>
    
        <div>
            <p class="text-lg mb-4">Initialize database with tables</p>
            <div class="relative">
                <button class="hover:bg-zinc-800 p-2 rounded-sm absolute top-3 right-3" on:click={() => {copy("code-1")}} aria-label="Copy code"><Copy></Copy></button>
                <textarea id="code-2" class="px-6 rounded-sm bg-zinc-950 py-6 w-full overflow-hidden lg:h-44 h-52 resize-none outline-none font-secondary" spellcheck="false" readonly>
let posts_table = Box::<Posts>::default();
let categories_table = Box::<Categories>::default();
        
let mut tables: Vec<Box<dyn Table>> = vec![posts_table, categories_table];
sqlite::init(conn, tables);
            </textarea>
            </div>
        </div>
    
        <div>
            <p class="text-lg mb-4">Insert data to table</p>
            <div class="relative">
                <button class="hover:bg-zinc-800 p-2 rounded-sm absolute top-3 right-3" on:click={() => {copy("code-1")}} aria-label="Copy code"><Copy></Copy></button>
                <textarea id="code-3" class="px-6 rounded-sm bg-zinc-950 py-6 w-full overflow-hidden lg:h-48 h-52 resize-none outline-none font-secondary" spellcheck="false" readonly>
let table_row: Posts = Posts {'{'}
    title: "A post title".to_string(),
    description: "Some description for for a post".to_string(),
{'}'};
    
sqlite::insert(conn, &table_row);
            </textarea>
            </div>
        </div>
    
    </div>
        
</div>
</div>

<style>
.fade-out {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 100px; /* Adjust the height of the fade-out effect as needed */
  background: linear-gradient(to bottom, rgba(255, 255, 255, 0), #18181b);
}
</style>