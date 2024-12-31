<script lang="ts">
  import '../app.css';
  let { children } = $props();

  import MdHeader from '../lib/components/MdHeader.svelte';
  import MdNav from '../lib/components/MdNav.svelte';
</script>

<style>
:root {
  --primary-color: #005f73;
  --primary-color-weak: #4a5568;
  --secondary-color: #ee6c4d;
  --secondary-color-weak: #f7fafc;

  --primary-text: #1a202c;
}

:global(body) {
  margin: 0;
  background: var(--secondary-color-weak);
  color: var(--primary-text);
}
</style>

<div>
  <MdHeader/>
  <MdNav />
  <section>
    {@render children()}
  </section>
  <footer>All rights reserved</footer>
</div>
