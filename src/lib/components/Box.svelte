<script>
    export let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.";
</script>

<div class="rounded-xl flex flex-col border border-zinc-100 p-8 gap-8">
    <slot/>
    <p>{text}</p>
</div>