<script>
  import Box from '$lib/components/Box.svelte';
  import Coffee from '$lib/components/icons/Coffee.svelte';
  import Code from '$lib/components/icons/Code.svelte';
  import Alert from '$lib/components/icons/Alert.svelte';
</script>

<section class="py-20">
  <div class="container grid grid-cols-1 md:grid-cols-2 gap-8 lg:grid-cols-3">
    <Box
      text="Join our community on Discord! Connect with our developers, seek help on usage, share your ideas, and stay updated on the latest developments on the project. Your input fuels Njord's growth."
    >
      <div class="text-primary">
        <Coffee size="48" />
      </div>
    </Box>
    <Box
      text="Njord is a FOSS project, and we welcome your contributions. Whether it's bug fixes, new features, documentation, or other improvements, your help is valuable. Check out our GitHub repository for more information."
    >
      <div class="text-primary">
        <Code size="48" />
      </div>
    </Box>
    <Box
      text="Njord is currently in alpha. While it's shaping up to be a powerful ORM and Query Builder, exercise caution in production environments. Your feedback is invaluable as we work towards stability. Stay tuned for updates and improvements!"
    >
      <div class="text-primary">
        <Alert size="48" />
      </div>
    </Box>
  </div>
</section>
