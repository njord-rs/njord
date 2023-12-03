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

<div class="container flex flex-col px-4 py-24">
    <h1 class="header text-8xl mb-4">Njord</h1>
    <h2 class="text-2xl">Safe, Extensible ORM and Query Builder for Rust</h2>
    <GithubBar repo={data.repo}></GithubBar>  

    <div class="grid grid-cols-1 md:grid-cols-2 gap-8 lg:grid-cols-3 my-16">
    <Box>
       <Coffee></Coffee>
    </Box>
    <Box>
        <Code></Code>
     </Box>
     <Box>
        <Alert></Alert>
     </Box>
    </div>
<div class="flex flex-col gap-6">
    <div>
        <p class="text-lg mb-4">Setup connection</p>
        <div class="relative">
            <button class="hover:bg-zinc-800 p-2 rounded-sm absolute top-3 right-3" on:click={() => {copy("code-1")}}><Copy></Copy></button>
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
            <button class="hover:bg-zinc-800 p-2 rounded-sm absolute top-3 right-3" on:click={() => {copy("code-1")}}><Copy></Copy></button>
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
            <button class="hover:bg-zinc-800 p-2 rounded-sm absolute top-3 right-3" on:click={() => {copy("code-1")}}><Copy></Copy></button>
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


