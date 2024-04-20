<script>
  import GithubBar from "$lib/components/GithubBar.svelte";
  import Box from "$lib/components/Box.svelte";
  import Coffee from "$lib/components/icons/Coffee.svelte";
  import Code from "$lib/components/icons/Code.svelte";
  import Alert from "$lib/components/icons/Alert.svelte";
  import DatabaseBar from "$lib/components/DatabaseBar.svelte";
  export let data;
</script>

<div class="flex flex-col w-full items-center">
    <div class="relative py-36 w-full flex justify-center">
        <img src="/banner.png" class="opacity-5 absolute w-full align-top h-full top-0 bottom-0 object-cover" alt="Viking city" style="pointer-events: none">
        <div class="fade-out" style="pointer-events: none"></div>
        <div class="container px-4">
            <h1 class="header text-8xl mb-4">Njord</h1>
            <h2 class="text-2xl">Lightweight, Extensible ORM and Query Builder for Rust</h2>
            <GithubBar repo={data.repo}></GithubBar> 
            <DatabaseBar></DatabaseBar>
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
</div>    
</div>

<style>
.fade-out {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  height: 100px; 
  background: linear-gradient(to bottom, rgba(255, 255, 255, 0), #18181b);
}
</style>