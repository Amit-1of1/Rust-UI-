<script lang="ts">
  import { onMount } from 'svelte';
  import { makeTask } from './data';

  interface BenchRow {
    id: number;
    title: string;
  }

  let rows = $state<BenchRow[]>([]);
  let selected = $state<number | null>(null);
  // Ids keep counting up across every create, as in js-framework-benchmark.
  let nextIndex = 0;

  function build(count: number): BenchRow[] {
    const start = nextIndex;
    nextIndex += count;
    const out = new Array<BenchRow>(count);
    for (let k = 0; k < count; k++) {
      const { id, title } = makeTask(start + k);
      out[k] = { id, title };
    }
    return out;
  }

  function run() {
    rows = build(1000);
    selected = null;
  }
  function runLots() {
    rows = build(10000);
    selected = null;
  }
  function add() {
    rows.push(...build(1000));
  }
  function update() {
    for (let i = 0; i < rows.length; i += 10) rows[i].title += ' !!!';
  }
  function swapRows() {
    if (rows.length < 999) return;
    const a = rows[1];
    rows[1] = rows[998];
    rows[998] = a;
  }
  function clear() {
    rows = [];
    selected = null;
  }
  function remove(id: number) {
    const i = rows.findIndex((r) => r.id === id);
    if (i !== -1) rows.splice(i, 1);
  }

  onMount(() => {
    document.body.dataset.ready = '1';
  });
</script>

<main class="bench">
  <div class="bench-buttons">
    <button type="button" id="run" onclick={run}>Create 1,000 rows</button>
    <button type="button" id="runlots" onclick={runLots}>Create 10,000 rows</button>
    <button type="button" id="add" onclick={add}>Append 1,000 rows</button>
    <button type="button" id="update" onclick={update}>Update every 10th row</button>
    <button type="button" id="swaprows" onclick={swapRows}>Swap rows</button>
    <button type="button" id="clear" onclick={clear}>Clear</button>
  </div>
  <table id="bench" class="table">
    <tbody>
      {#each rows as row (row.id)}
        <tr class={row.id === selected ? 'selected' : undefined}>
          <td>{row.id}</td>
          <td onclick={() => (selected = row.id)}>{row.title}</td>
          <td><button type="button" onclick={() => remove(row.id)}>×</button></td>
        </tr>
      {/each}
    </tbody>
  </table>
</main>
