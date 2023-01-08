
<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let decision = "00";

  async function decide(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    decision = await invoke("decide")
    if (decision.length === 1) {
      decision = 0 + decision
    }
  }
</script>

<h1 on:click={decide}>
  {decision}
</h1>


<style>
@font-face {
  src: url('/Kalam-Regular.ttf');
  font-family: 'KalamRegular';
} 
h1 {
  font-family: 'KalamRegular';
  font-size: 15vh;
  color: rgba(0,0,0,0.5);
  cursor: pointer;
}

</style>
