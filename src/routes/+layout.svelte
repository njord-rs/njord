<script>
  import Navbar from "$lib/components/Navbar.svelte";
  import "../app.css";
  let pageTitle = 'Njord - A lightweight ORM library for Rust';
</script>


<svelte:head>
  <title>{pageTitle}</title>
	<meta property="og:title" content="{pageTitle}" />
	<meta
		property="og:description"
		content="A lightweight and extensible ORM library and Query Builder for Rust."
	/>
	<meta property="og:image" content="/meta.png" />
	<meta property="twitter:card" content="summary_large_image" />
	<meta property="twitter:title" content="{pageTitle}" />
	<meta
		property="twitter:description"
		content="A lightweight and extensible ORM library and Query Builder for Rust."
	/>
	<meta property="twitter:image" content="/meta.png" />
</svelte:head>

<div class="bg-zinc-900 text-zinc-50 font-primary pt-16 flex items-center flex-col">
  <Navbar></Navbar>
    <slot />
</div>
