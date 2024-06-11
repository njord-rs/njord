<script>
  import { inject } from '@vercel/analytics'
  import Navbar from '$lib/components/Navbar.svelte';
  import Footer from '$lib/components/Footer.svelte';
  import '../app.css';
  let pageTitle = 'Njord - A lightweight ORM library for Rust';
</script>

<svelte:head>
  <title>{pageTitle}</title>
  <meta property="og:title" content={pageTitle} />
  <meta
    property="og:description"
    content="A lightweight and extensible ORM library and Query Builder for Rust."
  />
  <meta property="og:image" content="/meta.png" />
  <meta property="twitter:card" content="summary_large_image" />
  <meta property="twitter:title" content={pageTitle} />
  <meta
    property="twitter:description"
    content="A lightweight and extensible ORM library and Query Builder for Rust."
  />
  <meta property="twitter:image" content="/meta.png" />

  <!-- Prevent page flash -->
  <script>
    if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
    document.documentElement.classList.add('dark')
    } else {
        document.documentElement.classList.remove('dark')
    }
  </script>
</svelte:head>

<div
  class="bg-background text-foreground font-primary flex items-center flex-col relative min-h-screen"
>
  <Navbar></Navbar>
  <slot />
  <Footer></Footer>
</div>
