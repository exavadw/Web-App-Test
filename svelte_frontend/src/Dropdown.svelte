<script>
    let selectedOption = "Option 1"; // Default selected option
  
    const options = ["Option 1", "Option 2", "Option 3"];
  
    function handleSelect(event) {
      selectedOption = event.target.value;
      console.log("Selected:", selectedOption);
    }
  </script>
  
  <main>
    <h1>Dropdown Menu Example</h1>
    <label for="dropdown">Choose an option:</label>
    <select id="dropdown" on:change={handleSelect}>
      {#each options as option}
        <option value={option} selected={option === selectedOption}>{option}</option>
      {/each}
    </select>
  
    <p>You selected: {selectedOption}</p>
  </main>
  
  <style>
    main {
      font-family: Arial, sans-serif;
      text-align: center;
      margin-top: 2rem;
    }
  
    label {
      font-size: 1.2rem;
      margin-right: 1rem;
    }
  
    select {
      font-size: 1.2rem;
      padding: 0.2rem 0.5rem;
      border: 1px solid #ccc;
      border-radius: 5px;
    }
  
    p {
      margin-top: 1rem;
      font-size: 1.2rem;
      color: #007acc;
    }
  </style>
  