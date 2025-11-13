<script lang="ts">
  import { onMount } from 'svelte';
  import { Channel } from '@tauri-apps/api/core';
  import * as commands from '../generated/commands';
  import * as events from '../generated/events';
  import {
    UserSchema,
    OrderSchema,
    type User,
    type Product,
    type LogEntry
  } from '../generated/types';

  // State
  let greeting = $state('');
  let mathResult = $state(0);
  let user: User | null = $state(null);
  let products: Product[] = $state([]);
  let orderStatus = $state('');
  let taskProgress = $state(0);
  let taskMessage = $state('');
  let logs: LogEntry[] = $state([]);
  let cpuUsage = $state(0);
  let memoryUsage = $state(0);
  let divisionResult = $state('');

  // Form state for validation demo
  let formUser = $state({
    userId: 0,
    userName: '',
    email: '',
    isActive: true
  });
  let validationErrors = $state<string[]>([]);
  let validationSuccess = $state(false);

  // Channels
  let cpuChannel = new Channel<number>();
  cpuChannel.onmessage = (response) => cpuUsage = response;

  let memoryChannel = new Channel<number>();
  memoryChannel.onmessage = (response) => memoryUsage = response;

  // Simple command
  async function runGreet() {
    greeting = await commands.greet();
  }

  // Command with parameters
  async function runAddNumbers() {
    mathResult = await commands.addNumbers({ a: 15, b: 27 });
  }

  // Command returning custom struct (no validation needed - backend is source of truth)
  async function runGetUser() {
    user = await commands.getUser({ userId: 1 });
  }

  // Validate form input with Zod before sending
  async function runCreateUser() {
    validationErrors = [];
    validationSuccess = false;

    // Validate form data with Zod schema
    const validation = UserSchema.safeParse(formUser);

    if (!validation.success) {
      // Extract validation errors with custom messages from validator attributes
      validationErrors = validation.error.issues.map((issue) =>
        `${issue.path.join('.')}: ${issue.message}`
      );
      return;
    }

    try {
      // Data is valid, send to backend
      const result = await commands.createUser({ user: validation.data });
      user = result;
      validationSuccess = true;
      console.log('✅ User created and validated:', result);
    } catch (error) {
      validationErrors = [`Backend error: ${error}`];
    }
  }

  // Command with optional parameters (no validation needed - backend is source of truth)
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

  // Command with complex nested types - validate input before sending
  async function runCreateOrder() {
    const order = {
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
      status: "PENDING" as const,
      paymentMethod: "credit_card" as const,
      totalAmount: 1029.98
    };

    // Validate order before sending (input validation)
    const validation = OrderSchema.safeParse(order);
    if (!validation.success) {
      console.error('❌ Order validation failed before sending:', validation.error);
      return;
    }

    try {
      const result = await commands.createOrder({ order: validation.data });
      console.log("✅ Order validated and sent to backend:", result);
    } catch (error) {
      console.error("Order creation error:", error);
    }
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

  // Command with single channel (streaming) - no validation needed
  async function runStreamLogs() {
    logs = [];
    const channel = new Channel<LogEntry>();

    channel.onmessage = (log) => {
      logs = [...logs, log];
    };

    try {
      await commands.streamLogs({ channel });
    } catch (error) {
      console.error("Stream error:", error);
    }
  }

  // Command with multiple channels
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

  // Set up event listener (no validation needed - backend is source of truth)
  onMount(async () => {
    const unlistenProgress = await events.onProgressUpdate((payload) => {
      taskProgress = payload.progress;
      taskMessage = payload.message;
    });
  });
</script>

<div class="container">
  <h1>Tauri TypeGen + Zod Examples</h1>
  <p>Demonstrating input validation with auto-generated Zod schemas from Rust validator attributes</p>

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

    <!-- Get User -->
    <section>
      <h2>3. Custom Struct Response</h2>
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

    <!-- Input Validation Form -->
    <section class="large">
      <h2>4. Input Validation with Validator Constraints</h2>
      <p class="info">✨ Auto-generated from Rust <code>#[validate(...)]</code> attributes</p>
      <p class="info">Try: User ID ≤ 0, Name &lt; 3 chars, invalid email</p>
      <form onsubmit={(e) => { e.preventDefault(); runCreateUser(); }}>
        <div class="form-group">
          <label>User ID <small>(must be positive)</small>:
          <input type="number" bind:value={formUser.userId} /></label>
        </div>
        <div class="form-group">
          <label>Name <small>(3-50 characters)</small>:
          <input type="text" bind:value={formUser.userName} placeholder="Enter name" /></label>
        </div>
        <div class="form-group">
          <label>Email <small>(valid email required)</small>:
          <input type="email" bind:value={formUser.email} placeholder="user@example.com" /></label>
        </div>
        <div class="form-group">
          <label>
            <input type="checkbox" bind:checked={formUser.isActive} />
            Active
          </label>
        </div>
        <button type="submit">Create User (validates input)</button>
      </form>

      {#if validationErrors.length > 0}
        <div class="validation-errors">
          <p><strong>❌ Validation Errors:</strong></p>
          <ul>
            {#each validationErrors as error}
              <li>{error}</li>
            {/each}
          </ul>
        </div>
      {/if}

      {#if validationSuccess}
        <div class="validation-success">
          <p>✅ User created successfully!</p>
        </div>
      {/if}
    </section>


    <!-- Array Response -->
    <section>
      <h2>6. Array Response</h2>
      <button onclick={runSearchProducts}>Search Products</button>
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

    <!-- Enum Parameter -->
    <section>
      <h2>7. Enum Parameters</h2>
      <button onclick={runUpdateOrderStatus}>Update Status</button>
      {#if orderStatus}
        <p class="result">{orderStatus}</p>
      {/if}
    </section>

    <!-- Complex Nested Input Validation -->
    <section>
      <h2>8. Nested Object Input Validation</h2>
      <button onclick={runCreateOrder}>Create Order</button>
      <p class="info">Validates input: nested User, Product[], and enums before sending</p>
      <p class="info">Check console for validation details</p>
    </section>

    <!-- Events -->
    <section>
      <h2>9. Event Payloads</h2>
      <button onclick={runProcessTask}>Process Task</button>
      <div class="progress-container">
        <div class="progress-bar">
          <div class="progress-fill" style="width: {taskProgress}%"></div>
        </div>
        <p class="result">{taskMessage} ({taskProgress.toFixed(0)}%)</p>
      </div>
    </section>

    <!-- Streaming -->
    <section>
      <h2>10. Streaming (Channels)</h2>
      <button onclick={runStreamLogs}>Stream Logs</button>
      {#if logs.length > 0}
        <div class="result logs">
          {#each logs as log}
            <div class="log-entry {log.level.toLowerCase()}">
              <span class="log-level">[{log.level}]</span>
              <span class="log-message">{log.message}</span>
            </div>
          {/each}
          <p class="info">✅ Each entry validated with LogEntrySchema</p>
        </div>
      {/if}
    </section>

    <!-- Multiple Channels -->
    <section>
      <h2>11. Multiple Channels</h2>
      <button onclick={runMonitorSystem}>Monitor System</button>
      <div class="result">
        <p><strong>CPU:</strong> {cpuUsage.toFixed(1)}%</p>
        <p><strong>Memory:</strong> {memoryUsage.toFixed(1)}%</p>
      </div>
    </section>

    <!-- Error Handling -->
    <section>
      <h2>12. Error Handling</h2>
      <div class="buttons">
        <button onclick={runDivide}>10 / 2</button>
        <button onclick={runDivideByZero}>10 / 0</button>
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
    max-width: 1400px;
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

  section.large {
    grid-column: span 2;
  }

  @media (max-width: 800px) {
    section.large {
      grid-column: span 1;
    }
  }

  button {
    background: #7c3aed;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
    margin-right: 0.5rem;
  }

  button:hover {
    background: #6d28d9;
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
    border-left: 3px solid #7c3aed;
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

  form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .form-group label {
    font-weight: 500;
    color: #555;
  }

  .form-group input[type="text"],
  .form-group input[type="email"],
  .form-group input[type="number"] {
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.9rem;
  }

  .form-group input[type="checkbox"] {
    width: auto;
    margin-right: 0.5rem;
  }

  .validation-errors {
    margin-top: 1rem;
    padding: 1rem;
    background: #fee;
    border-left: 3px solid #f00;
    border-radius: 4px;
  }

  .validation-errors ul {
    margin: 0.5rem 0 0 1.5rem;
    padding: 0;
  }

  .validation-errors li {
    color: #c00;
    font-size: 0.9rem;
  }

  .validation-success {
    margin-top: 1rem;
    padding: 1rem;
    background: #efe;
    border-left: 3px solid #0a0;
    border-radius: 4px;
    color: #060;
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
    background: linear-gradient(90deg, #7c3aed, #a78bfa);
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

  .constraint-tests {
    margin-top: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .constraint-test {
    padding: 0.75rem;
    border-radius: 4px;
    border: 1px solid #ddd;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }

  .constraint-test.passed {
    background: #f0fdf4;
    border-color: #86efac;
  }

  .constraint-test.failed {
    background: #fef2f2;
    border-color: #fca5a5;
  }

  .test-name {
    flex: 1;
    font-size: 0.9rem;
    color: #333;
  }

  .test-result {
    font-weight: 500;
    font-size: 0.9rem;
    white-space: nowrap;
  }

  code {
    background: #f5f5f5;
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
    font-family: 'Courier New', monospace;
    font-size: 0.85rem;
  }

  small {
    color: #666;
    font-size: 0.85rem;
  }
</style>
