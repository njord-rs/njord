<script>
  import Link from '$lib/components/Link.svelte';
  import Github from '$lib/components/icons/Github.svelte';
  import Star from '$lib/components/icons/Star.svelte';
  import Commit from '$lib/components/icons/Commit.svelte';
  import Merge from '$lib/components/icons/Merge.svelte';
  export let repo;
</script>

<div class="flex flex-col md:flex-row gap-6 py-4 font-secondary">
  <div class="flex gap-4 md:gap-6">
    <Link
      class="lg:hover:underline flex flex-row justify-center items-center gap-1"
      href={repo.latestVersion?.path}
      ariaLabel="Github version"
    >
      <Github />
      <span class="hidden md:inline">Version:</span>
      <span class="font-bold">{repo.latestVersion?.version ?? '0'}</span>
    </Link>

    <Link
      class="lg:hover:underline flex flex-row justify-center items-center gap-1"
      href={repo.stargazers?.path}
      ariaLabel="Github stargazers"
    >
      <Star />
      <span class="hidden md:inline">Stars:</span>
      <span class="font-bold">{repo.stargazers?.stars ?? '0'}</span>
    </Link>

    <Link
      class="lg:hover:underline flex flex-row justify-center items-center gap-1"
      href={repo.latestCommit?.path}
      ariaLabel="Github latest commit"
    >
      <Commit />
      <span class="hidden md:inline">Latest commit:</span>
      <span class="font-bold">{repo.latestCommit?.date ?? '00/00/00'}</span>
    </Link>
  </div>

  <div class="flex flex-row">
    <p class="mr-3 flex flex-row justify-center items-center gap-1">
      <span class="hidden md:inline"><Merge></Merge></span>Made by:
    </p>
    {#if repo.contributors != null}
      {#each repo.contributors as contributor}
        <Link
          class="-m-1"
          href={contributor.html_url}
          ariaLabel="Contributor {contributor.login}"
        >
          <img
            class="rounded-full lg:hover:scale-125"
            width="32px"
            src={contributor.avatar_url}
            alt="Contributor avatar"
          />
        </Link>
      {/each}
    {/if}
  </div>
</div>
