<script>
  import DatabaseCard from './DatabaseCard.svelte';

  const databases = [
    { label: 'SQLite', isReady: true },
    { label: 'PostgreSQL', isReady: false },
    { label: 'MySQL', isReady: false },
    { label: 'MariaDB', isReady: false },
    { label: 'ORACLE', isReady: false },
    { label: 'MS SQL Server', isReady: false },
  ];
</script>

<div class="sm:flex gap-8 py-4 font-secondary hidden">
  {#each databases as database}
    <DatabaseCard label={database.label} isReady={database.isReady} />
  {/each}
</div>
