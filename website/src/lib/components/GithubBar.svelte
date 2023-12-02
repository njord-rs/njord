<script>
    import Github from "$lib/components/icons/Github.svelte"
    import Star from "$lib/components/icons/Star.svelte"
    import Commit from "$lib/components/icons/Commit.svelte"
    export let repo;
  </script>
  
<div class="container flex-row items-center flex gap-6 justify-center py-4">
    <a href="https://github.com/mjovanc/njord"> <Github/> </a>
    <a class="hover:underline flex flex-row" href={repo.latestVersion.path}>v {repo.latestVersion.version}</a>
    <a class="hover:underline flex flex-row justify-center items-center gap-1" href={repo.stargazers.path}> <Star></Star> {repo.stargazers.stars}</a>
    <a class="hover:underline flex flex-row justify-center items-center gap-1" href={repo.latestCommit.path}> <Commit></Commit> latest {repo.latestCommit.date}</a>

    <div class="flex flex-row">
      {#each repo.contributors as contributor}
      <a class="-m-1" href={contributor.html_url}><img class="rounded-full hover:border-zinc-100 hover:border" width="32px" src={contributor.avatar_url} alt=""></a>
    {/each}
    </div>
   

</div>