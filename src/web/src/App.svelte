<script lang="ts">
  import { onMount } from "svelte";

  interface Scenario {
    id: number;
    name: string;
    traffic_delta_pct: number;
  }

  let name = "Downtown Bus Priority";
  let delta = -12;
  let result = "";
  let scenarios: Scenario[] = [];
  let loading = false;
  let error = "";
  let showForm = false;

  async function fetchScenarios() {
    try {
      const response = await fetch("http://localhost:8080/scenarios");
      if (response.ok) {
        scenarios = await response.json();
      }
    } catch (e) {
      console.error("Failed to fetch scenarios", e);
    }
  }

  async function runScenario() {
    loading = true;
    error = "";
    result = "";

    try {
      const response = await fetch("http://localhost:8080/scenarios", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name, traffic_delta_pct: delta }),
      });

      if (!response.ok) {
        throw new Error("Failed to create scenario");
      }

      const newScenario = await response.json();
      result = JSON.stringify(newScenario, null, 2);
      scenarios = [...scenarios, newScenario];
      name = "Downtown Bus Priority";
      delta = -12;
      showForm = false;
    } catch (e) {
      error = typeof e === "string" ? e : (e as Error).message;
      console.error("Error running scenario", e);
    } finally {
      loading = false;
    }
  }

  async function checkHealth() {
    try {
      const response = await fetch("http://localhost:8080/health");
      return response.ok;
    } catch {
      return false;
    }
  }

  onMount(async () => {
    const healthy = await checkHealth();
    if (healthy) {
      fetchScenarios();
    } else {
      error = "Cannot connect to API. Make sure it's running on http://localhost:8080";
    }
  });

  const styles = {
    container: "font-sans max-w-2xl mx-auto p-8 bg-gradient-to-br from-blue-50 to-indigo-50 min-h-screen",
    header: "mb-8 pb-4 border-b-2 border-indigo-300",
    title: "text-4xl font-bold text-indigo-900 mb-2",
    subtitle: "text-gray-600",
    section: "mb-8",
    sectionTitle: "text-2xl font-semibold text-indigo-800 mb-4",
    card: "bg-white rounded-lg shadow-md p-6 mb-4",
    button: "px-4 py-2 bg-indigo-600 text-white rounded hover:bg-indigo-700 font-semibold cursor-pointer",
    buttonSecondary: "px-4 py-2 bg-gray-300 text-gray-800 rounded hover:bg-gray-400 font-semibold cursor-pointer",
    input: "w-full px-3 py-2 border border-gray-300 rounded mb-3 focus:outline-none focus:border-indigo-500",
    label: "block text-sm font-medium text-gray-700 mb-1",
    error: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4",
    result: "bg-gray-100 p-4 rounded font-mono text-sm overflow-auto max-h-64",
    scenarioList: "space-y-2",
    scenarioItem: "bg-indigo-50 p-3 rounded border-l-4 border-indigo-500",
  };
</script>

<main style={styles.container}>
  <div style={styles.header}>
    <h1 style={styles.title}>Urban Twin Lab</h1>
    <p style={styles.subtitle}>City operations simulation platform</p>
  </div>

  {#if error}
    <div style={styles.error}>{error}</div>
  {/if}

  <div style={styles.section}>
    <h2 style={styles.sectionTitle}>Scenarios ({scenarios.length})</h2>
    {#if scenarios.length === 0}
      <p class="text-gray-600 italic">No scenarios created yet. Create one to get started.</p>
    {:else}
      <div style={styles.scenarioList}>
        {#each scenarios as scenario (scenario.id)}
          <div style={styles.scenarioItem}>
            <strong>{scenario.name}</strong>
            <div class="text-sm text-gray-600">
              Traffic Delta: {scenario.traffic_delta_pct > 0 ? "+" : ""}{scenario.traffic_delta_pct}%
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <div style={styles.section}>
    {#if !showForm}
      <button
        style={styles.button}
        on:click={() => (showForm = true)}
        disabled={loading}
      >
        + Create New Scenario
      </button>
    {:else}
      <div style={styles.card}>
        <h3 class="text-lg font-semibold mb-4">Create Simulation Scenario</h3>

        <label style={styles.label} for="name">Scenario Name</label>
        <input
          style={styles.input}
          id="name"
          type="text"
          bind:value={name}
          placeholder="e.g., Downtown Bus Priority"
          disabled={loading}
        />

        <label style={styles.label} for="delta">Traffic Delta (%)</label>
        <input
          style={styles.input}
          id="delta"
          type="number"
          bind:value={delta}
          placeholder="-12"
          disabled={loading}
        />

        <div class="flex gap-2">
          <button style={styles.button} on:click={runScenario} disabled={loading}>
            {loading ? "Running..." : "Run Simulation"}
          </button>
          <button
            style={styles.buttonSecondary}
            on:click={() => (showForm = false)}
            disabled={loading}
          >
            Cancel
          </button>
        </div>
      </div>
    {/if}
  </div>

  {#if result}
    <div style={styles.section}>
      <h2 style={styles.sectionTitle}>Simulation Result</h2>
      <div style={styles.result}>{result}</div>
    </div>
  {/if}
</main>
