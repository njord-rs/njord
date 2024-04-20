<script>
    import Database from "$lib/components/icons/Database.svelte"
</script>

<div class="container sm:flex gap-8 py-4 font-secondary hidden">
    <div class="flex flex-col items-center">
        <Database color="#78350f"/>
        <p>SQLite</p>
    </div>
    <div class="flex flex-col items-center opacity-20">
        <Database />
        <p>PostgreSQL</p>
    </div>
    <div class="flex flex-col items-center opacity-20">
        <Database />
        <p>MySQL</p>
    </div>
    <div class="flex flex-col items-center opacity-20">
        <Database />
        <p>MariaDB</p>
    </div>
    <div class="flex flex-col items-center opacity-20">
        <Database />
        <p>ORACLE</p>
    </div>
    <div class="flex flex-col items-center opacity-20">
        <Database />
        <p>MS SQL Server</p>
    </div>
</div>
