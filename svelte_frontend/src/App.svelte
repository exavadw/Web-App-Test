<script>
  let message = "Loading...";
  let isLoading = false;
  import Dropdown from './Dropdown.svelte';
  async function fetchMessage() {
    isLoading = true;
    try {
      const response = await fetch("http://localhost:8080/api/hello");
      if (!response.ok) throw new Error("Failed to fetch");
      message = await response.text();
    } catch (error) {
      message = "Error fetching data!";
      console.error(error);
    } finally {
      isLoading = false;
    }
  }
</script>

<main>
  <h1>Rust + Svelte</h1>
  <p>{message}</p>
  <button on:click={fetchMessage} disabled={isLoading}>
    {isLoading ? "Fetching..." : "Fetch Message"}
  </button>
  <Dropdown>
  </Dropdown>
</main>

<style>
  main {
    text-align: center;
    margin: 2rem;
    font-family: Arial, sans-serif;
  }

  button {
    padding: 0.5rem 1rem;
    background: #007acc;
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
  }

  button:disabled {
    background: #aaa;
  }
</style>
