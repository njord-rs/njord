<script>
  import Link from '$lib/components/Link.svelte';
  import SocialLink from '$lib/components/SocialLink.svelte';
  import Github from '$lib/components/icons/Github.svelte';
  import XTwitter from '$lib/components/icons/XTwitter.svelte';
  import Discord from '$lib/components/icons/Discord.svelte';
</script>

<header class="sticky top-0 inset-x-0 z-10 bg-zinc-900 font-secondary w-full">
  <div class="container items-center justify-between flex h-16">
    <!-- Logo -->
    <Link class="flex items-center gap-2" href="/" ariaLabel="Home">
      <img src="logo.png" alt="Logo" class="w-12" />
      <span class="text-xl font-semibold">Njord</span>
    </Link>

    <nav class="flex gap-8 items-center">
      <!-- Main menu -->
      <ul>
        <li><Link href="https://docs.rs/njord/latest/njord/">Docs</Link></li>
      </ul>

      <!-- Social links -->
      <ul class="flex items-center gap-3">
        <li>
          <SocialLink href="https://x.com/njord_rs" ariaLabel="X">
            <XTwitter />
          </SocialLink>
        </li>
        <li>
          <SocialLink
            href="https://discord.com/invite/2uppTzjUHE"
            ariaLabel="Discord"
          >
            <Discord />
          </SocialLink>
        </li>
        <li>
          <SocialLink
            href="https://github.com/njord-rs/njord"
            ariaLabel="GitHub"
          >
            <Github />
          </SocialLink>
        </li>
      </ul>
    </nav>
  </div>
</header>
