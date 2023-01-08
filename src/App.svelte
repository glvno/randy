<script lang="ts">
  import Decide from './lib/Decide.svelte'
</script>

<main data-tauri-drag-region class="container">
  <img data-tauri-drag-region src="/wizard_transparent.png" class="wizard" />
  <div class="sign">
    <Decide />
  </div>
</main>
<style>

  .wizard {
    height: 90vh;
    width: auto;
  }

  .sign {
    position: absolute;
    left: 57vw;
    top: 1.5vh;
  }

</style>