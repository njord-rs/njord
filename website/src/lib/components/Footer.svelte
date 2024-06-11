<script>
  import Github from '$lib/components/icons/Github.svelte';
  import Link from '$lib/components/Link.svelte';
</script>

<footer>
  <div
    class="container text-center justify-center items-center gap-2 flex h-20 text-sm"
  >
    <p>Website made by</p>
    <Github></Github>
    <Link
      href="https://github.com/appelskrutt34"
      class="font-bold"
      ariaLabel="Äppelskrutts Github"
    >
      äppelskrutt
    </Link>
  </div>
</footer>
