<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import { Channel } from '@tauri-apps/api/core';
  import * as commands from '../generated/commands';
  import * as events from '../generated/events';
  import type * as types from '../generated/types';

  // State
  let greeting = $state('');
  let mathResult = $state(0);
  let user: types.User | null = $state(null);
  let createdUser: types.User | null = $state(null);
  let products: types.Product[] = $state([]);
  let orderStatus = $state('');
  let taskProgress = $state(0);
  let taskMessage = $state('');
  let logs: types.LogEntry[] = $state([]);
  let cpuUsage = $state(0);
  let memoryUsage = $state(0);
  let divisionResult = $state('');

  let cpuChannel = new Channel<number>();
  cpuChannel.onmessage = (response) => cpuUsage = response;

  let memoryChannel = new Channel<number>();
  memoryChannel.onmessage = (response) => memoryUsage = response;

  let newUser: types.User = $state({
    userId: 2,
    userName: '',
    email: '',
    isActive: false
  });

  // Simple command
  async function runGreet() {
    greeting = await commands.greet();
  }

  // Command with parameters
  async function runAddNumbers() {
    mathResult = await commands.addNumbers({ a: 15, b: 27 });
  }

  // Command returning custom struct
  async function runGetUser() {
    user = await commands.getUser({ userId: 1 });
  }

  // Command with custom struct parameter
  async function runCreateUser() {
    createdUser = await commands.createUser({ user: newUser });
  }

  // Command with optional parameters
  async function runSearchProducts() {
    products = await commands.searchProducts({ query: "laptop", limit: 3 });
  }

  // Command with enum parameter
  async function runUpdateOrderStatus() {
    orderStatus = await commands.updateOrderStatus({
      orderId: "ORD-12345",
      status: "PROCESSING"
    });
  }

  // Command with complex nested types
  async function runCreateOrder() {
    const order: types.Order = {
      orderId: "ORD-98765",
      user: {
        userId: 1,
        userName: "John Doe",
        email: "john@example.com",
        isActive: true
      },
      products: [
        { productId: 1, name: "Laptop", price: 999.99, in_stock: true },
        { productId: 2, name: "Mouse", price: 29.99, in_stock: true }
      ],
      status: "PENDING",
      paymentMethod: "credit_card",
      totalAmount: 1029.98
    };

    const result = await commands.createOrder({ order });
    console.log("Order created:", result);
  }

  // Command that emits events
  async function runProcessTask() {
    taskProgress = 0;
    taskMessage = "Starting task...";

    try {
      const result = await commands.processTask({ taskId: "task-001" });
      taskMessage = result;
    } catch (error) {
      taskMessage = `Error: ${error}`;
    }
  }

  // Command with single channel (streaming)
  async function runStreamLogs() {
    logs = [];
    const channel = new Channel<types.LogEntry>();

    channel.onmessage = (log) => {
      console.log(log);
      logs = [...logs, log];
    };

    try {
      await commands.streamLogs({channel});
    } catch (error) {
      console.error("Log streaming error:", error)
    }
  }


  // Command with multiple channels (streaming)
  async function runMonitorSystem() {
    try {
      await commands.monitorSystem({ cpuChannel, memoryChannel });
    } catch (error) {
      console.error("Monitor error:", error);
    }

  }

  // Command with Result type (error handling)
  async function runDivide() {
    try {
      const result = await commands.divide({ a: 10, b: 2 });
      divisionResult = `Result: ${result}`;
    } catch (error) {
      divisionResult = `Error: ${error}`;
    }
  }

  async function runDivideByZero() {
    try {
      const result = await commands.divide({ a: 10, b: 0 });
      divisionResult = `Result: ${result}`;
    } catch (error) {
      divisionResult = `Error: ${error}`;
    }
  }

  // Set up event listener on mount
  onMount(async () => {
    // Listen for progress updates from process_task command
    const unlistenProgress = await events.onProgressUpdate((payload) => {
      taskProgress = payload.progress;
      taskMessage = payload.message;
    });
  });
</script>

<div class="container">
  <h1>Tauri TypeGen Examples</h1>
  <p>Demonstrating all features of generated TypeScript bindings</p>

  <div class="examples">
    <!-- Simple Command -->
    <section>
      <h2>1. Simple Command</h2>
      <button onclick={runGreet}>Run greet()</button>
      {#if greeting}
        <p class="result">{greeting}</p>
      {/if}
    </section>

    <!-- Command with Parameters -->
    <section>
      <h2>2. Command with Parameters</h2>
      <button onclick={runAddNumbers}>Add 15 + 27</button>
      {#if mathResult > 0}
        <p class="result">Result: {mathResult}</p>
      {/if}
    </section>

    <!-- Command Returning Custom Struct -->
    <section>
      <h2>3. Custom Struct Return Type</h2>
      <button onclick={runGetUser}>Get User</button>
      {#if user}
        <div class="result">
          <p><strong>User ID:</strong> {user.userId}</p>
          <p><strong>Name:</strong> {user.userName}</p>
          <p><strong>Email:</strong> {user.email}</p>
          <p><strong>Active:</strong> {user.isActive ? '✓' : '✗'}</p>
        </div>
      {/if}
    </section>

    <!-- Command with Struct Parameter -->
    <section>
      <h2>4. Command with Struct Parameter</h2>
      <div>
        <label>
          Name:
          <input type="text" bind:value={newUser.userName} />
        </label>
        <label>
          Email:
          <input type="email" bind:value={newUser.email} />
        </label>

      </div>
      <button onclick={runCreateUser}>Create User</button>
      {#if createdUser}
        <div class="result">
          <p><strong>User ID:</strong> {createdUser.userId}</p>
          <p><strong>Name:</strong> {createdUser.userName}</p>
          <p><strong>Email:</strong> {createdUser.email}</p>
          <p><strong>Active:</strong> {createdUser.isActive ? '✓' : '✗'}</p>
        </div>
      {/if}
    </section>

    <!-- Command with Optional Parameters -->
    <section>
      <h2>5. Optional Parameters</h2>
      <button onclick={runSearchProducts}>Search Products (limit: 3)</button>
      {#if products.length > 0}
        <div class="result">
          {#each products as product}
            <div class="product">
              <p><strong>{product.name}</strong> - ${product.price}</p>
              <p>In Stock: {product.in_stock ? 'Yes' : 'No'}</p>
            </div>
          {/each}
        </div>
      {/if}
    </section>

    <!-- Command with Enum Parameter -->
    <section>
      <h2>6. Enum Parameter</h2>
      <button onclick={runUpdateOrderStatus}>Update Order Status</button>
      {#if orderStatus}
        <p class="result">{orderStatus}</p>
      {/if}
    </section>

    <!-- Command with Complex Types -->
    <section>
      <h2>7. Complex Nested Types</h2>
      <button onclick={runCreateOrder}>Create Order</button>
      <p class="info">Check browser console for result</p>
    </section>

    <!-- Events -->
    <section>
      <h2>8. Events (app.emit)</h2>
      <button onclick={runProcessTask}>Process Task</button>
      <div class="progress-container">
        <div class="progress-bar">
          <div class="progress-fill" style="width: {taskProgress}%"></div>
        </div>
        <p class="result">{taskMessage} ({taskProgress.toFixed(0)}%)</p>
      </div>
    </section>

    <!-- Single Channel -->
    <section>
      <h2>9. Single Channel (Streaming)</h2>
      <button onclick={runStreamLogs}>Stream Logs</button>
      {#if logs.length > 0}
        <div class="result logs">
          {#each logs as log}
            <div class="log-entry {log.level.toLowerCase()}">
              <span class="log-level">[{log.level}]</span>
              <span class="log-message">{log.message}</span>
            </div>
          {/each}
        </div>
      {/if}
    </section>

    <!-- Multiple Channels -->
    <section>
      <h2>10. Multiple Channels</h2>
      <button onclick={runMonitorSystem}>Monitor System</button>
      <div class="result">
        <p><strong>CPU Usage:</strong> {cpuUsage.toFixed(1)}%</p>
        <p><strong>Memory Usage:</strong> {memoryUsage.toFixed(1)}%</p>
      </div>
    </section>

    <!-- Error Handling -->
    <section>
      <h2>11. Error Handling (Result Type)</h2>
      <div class="buttons">
        <button onclick={runDivide}>Divide 10 / 2</button>
        <button onclick={runDivideByZero}>Divide 10 / 0</button>
      </div>
      {#if divisionResult}
        <p class="result">{divisionResult}</p>
      {/if}
    </section>
  </div>
</div>

<style>
  .container {
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
  }

  h1 {
    color: #333;
    margin-bottom: 0.5rem;
  }

  h2 {
    color: #555;
    font-size: 1.2rem;
    margin-bottom: 1rem;
  }

  .examples {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 1.5rem;
    margin-top: 2rem;
  }

  section {
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 1.5rem;
    background: #f9f9f9;
  }

  button {
    background: #0066cc;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
    margin-right: 0.5rem;
  }

  button:hover {
    background: #0052a3;
  }

  .buttons {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .result {
    margin-top: 1rem;
    padding: 1rem;
    background: white;
    border-radius: 4px;
    border-left: 3px solid #0066cc;
  }

  .result p {
    margin: 0.25rem 0;
  }

  .info {
    margin-top: 0.5rem;
    font-size: 0.85rem;
    color: #666;
    font-style: italic;
  }

  .product {
    padding: 0.5rem 0;
    border-bottom: 1px solid #eee;
  }

  .product:last-child {
    border-bottom: none;
  }

  .product p {
    margin: 0.25rem 0;
  }

  .progress-container {
    margin-top: 1rem;
  }

  .progress-bar {
    width: 100%;
    height: 20px;
    background: #e0e0e0;
    border-radius: 10px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #4CAF50, #8BC34A);
    transition: width 0.3s ease;
  }

  .logs {
    max-height: 200px;
    overflow-y: auto;
    font-family: monospace;
    font-size: 0.85rem;
  }

  .log-entry {
    padding: 0.25rem 0.5rem;
    margin: 0.25rem 0;
    border-radius: 3px;
  }

  .log-entry.info {
    background: #e3f2fd;
  }

  .log-entry.warn {
    background: #fff3e0;
  }

  .log-entry.error {
    background: #ffebee;
  }

  .log-level {
    font-weight: bold;
    margin-right: 0.5rem;
  }

  .log-message {
    color: #333;
  }
</style>
