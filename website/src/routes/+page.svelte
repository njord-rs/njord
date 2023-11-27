<script>
	import { onMount } from 'svelte';

	let pageTitle = 'njord - A lightweight ORM library for Rust';
	onMount(() => {
		document.title = pageTitle;
	});
</script>

<svelte:head>
	<meta property="og:title" content="{pageTitle}" />
	<meta
		property="og:description"
		content="A lightweight ORM library for Rust with strong-typed SQL DSL and sequence APIs."
	/>
	<meta property="og:image" content="/meta.png" />
	<meta property="twitter:card" content="summary_large_image" />
	<meta property="twitter:title" content="{pageTitle}" />
	<meta
		property="twitter:description"
		content="A lightweight ORM library for Rust with strong-typed SQL DSL and sequence APIs."
	/>
	<meta property="twitter:image" content="/meta.png" />
</svelte:head>