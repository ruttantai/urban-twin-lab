<script lang="ts">
  let name = "Downtown Bus Priority";
  let delta = -12;
  let result = "";

  async function runScenario() {
    const response = await fetch("http://localhost:8080/scenarios", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name, traffic_delta_pct: delta })
    });
    result = JSON.stringify(await response.json(), null, 2);
  }
</script>

<main style="font-family: sans-serif; margin: 2rem; max-width: 48rem;">
  <h1>Urban Twin Lab</h1>
  <label>Scenario name <input bind:value={name} /></label>
  <label>Traffic delta % <input type="number" bind:value={delta} /></label>
  <button on:click={runScenario}>Run simulation</button>
  <pre>{result}</pre>
</main>
