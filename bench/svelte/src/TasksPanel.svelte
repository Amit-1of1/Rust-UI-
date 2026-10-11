<script lang="ts" module>
  export type SortKey = 'title' | 'assignee' | 'priority' | 'estimate';
  export interface SortState {
    key: SortKey;
    dir: 'asc' | 'desc';
  }
</script>

<script lang="ts">
  import { Checkbox } from 'bits-ui';
  import type { Priority, Task } from './data';
  import NewTaskDialog from './NewTaskDialog.svelte';

  interface Props {
    tasks: Task[];
    pageSize: number;
    filter: string;
    sort: SortState | null;
    page: number;
    selected: Set<number>;
    newTaskOpen: boolean;
    onAddTask: (title: string, assignee: string, priority: Priority) => void;
  }

  let {
    tasks,
    pageSize,
    filter = $bindable(),
    sort = $bindable(),
    page = $bindable(),
    selected = $bindable(),
    newTaskOpen = $bindable(),
    onAddTask,
  }: Props = $props();

  const PRIORITY_RANK: Record<Priority, number> = { Low: 0, Medium: 1, High: 2 };

  function compareTasks(a: Task, b: Task, s: SortState): number {
    let c: number;
    if (s.key === 'priority') c = PRIORITY_RANK[a.priority] - PRIORITY_RANK[b.priority];
    else if (s.key === 'estimate') c = a.estimate - b.estimate;
    else {
      const x = a[s.key];
      const y = b[s.key];
      c = x < y ? -1 : x > y ? 1 : 0;
    }
    return s.dir === 'asc' ? c : -c;
  }

  const COLUMNS: { key: SortKey; label: string }[] = [
    { key: 'title', label: 'Title' },
    { key: 'assignee', label: 'Assignee' },
    { key: 'priority', label: 'Priority' },
  ];

  const rows = $derived.by(() => {
    const q = filter.toLowerCase();
    const out = q ? tasks.filter((t) => t.title.toLowerCase().includes(q)) : tasks.slice();
    if (sort) {
      const s = sort;
      out.sort((a, b) => compareTasks(a, b, s));
    }
    return out;
  });

  const total = $derived(rows.length);
  const pageCount = $derived(Math.max(1, Math.ceil(total / pageSize)));
  const current = $derived(Math.min(page, pageCount));
  const start = $derived((current - 1) * pageSize);
  const pageRows = $derived(rows.slice(start, start + pageSize));
  const from = $derived(total === 0 ? 0 : start + 1);
  const to = $derived(start + pageRows.length);

  const selectedOnPage = $derived(pageRows.filter((t) => selected.has(t.id)).length);
  const allChecked = $derived(pageRows.length > 0 && selectedOnPage === pageRows.length);
  const someChecked = $derived(selectedOnPage > 0 && !allChecked);

  function toggleAll() {
    const next = new Set(selected);
    if (allChecked) pageRows.forEach((t) => next.delete(t.id));
    else pageRows.forEach((t) => next.add(t.id));
    selected = next;
  }

  function toggleRow(id: number, on: boolean) {
    const next = new Set(selected);
    if (on) next.add(id);
    else next.delete(id);
    selected = next;
  }

  function sortBy(key: SortKey) {
    sort = { key, dir: sort?.key === key && sort.dir === 'asc' ? 'desc' : 'asc' };
  }

  function ariaSort(key: SortKey) {
    return sort?.key === key ? (sort.dir === 'asc' ? 'ascending' : 'descending') : 'none';
  }

  function sortIcon(key: SortKey) {
    return sort?.key === key ? (sort.dir === 'asc' ? '▲' : '▼') : '↕';
  }
</script>

{#snippet checkIcon()}
  <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true">
    <path d="M3 8.5l3 3 7-7" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
  </svg>
{/snippet}

{#snippet minusIcon()}
  <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true">
    <path d="M4 8h8" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
  </svg>
{/snippet}

{#snippet sortHeader(key: SortKey, label: string, cls?: string)}
  <th scope="col" aria-sort={ariaSort(key)} class={cls}>
    <button type="button" class="sort-button" onclick={() => sortBy(key)}>
      {label}
      <span class="sort-icon" aria-hidden="true">{sortIcon(key)}</span>
    </button>
  </th>
{/snippet}

<section class="panel">
  <div class="toolbar">
    <div class="field">
      <label for="filter">Filter tasks</label>
      <input
        id="filter"
        class="input"
        type="search"
        value={filter}
        oninput={(e) => {
          filter = e.currentTarget.value;
          page = 1;
        }}
      />
    </div>
    <NewTaskDialog bind:open={newTaskOpen} onAdd={onAddTask} />
  </div>

  <div class="table-wrap">
    <table class="table">
      <caption>Tasks</caption>
      <thead>
        <tr>
          <th scope="col" class="col-select">
            <span class="sr-only">Select</span>
            <Checkbox.Root
              class="checkbox"
              aria-label="Select all rows on this page"
              checked={allChecked}
              indeterminate={someChecked}
              onCheckedChange={toggleAll}
            >
              {#snippet children({ checked, indeterminate })}
                {#if indeterminate}
                  <span class="checkbox-indicator">{@render minusIcon()}</span>
                {:else if checked}
                  <span class="checkbox-indicator">{@render checkIcon()}</span>
                {/if}
              {/snippet}
            </Checkbox.Root>
          </th>
          {#each COLUMNS as c (c.key)}
            {@render sortHeader(c.key, c.label)}
          {/each}
          <th scope="col">Status</th>
          {@render sortHeader('estimate', 'Estimate', 'num')}
        </tr>
      </thead>
      <tbody>
        {#each pageRows as t (t.id)}
          <tr data-state={selected.has(t.id) ? 'selected' : undefined}>
            <td class="col-select">
              <Checkbox.Root
                class="checkbox"
                aria-label={`Select ${t.title}`}
                checked={selected.has(t.id)}
                onCheckedChange={(c) => toggleRow(t.id, c)}
              >
                {#snippet children({ checked })}
                  {#if checked}
                    <span class="checkbox-indicator">{@render checkIcon()}</span>
                  {/if}
                {/snippet}
              </Checkbox.Root>
            </td>
            <td>{t.title}</td>
            <td>{t.assignee}</td>
            <td><span class="badge badge-{t.priority.toLowerCase()}">{t.priority}</span></td>
            <td>{t.status}</td>
            <td class="num">{t.estimate}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <div class="table-footer">
    <p role="status" class="muted">Showing {from}–{to} of {total} rows</p>
    <nav aria-label="Table pages" class="pager">
      <button type="button" class="btn btn-outline" disabled={current <= 1} onclick={() => (page = current - 1)}>
        Previous
      </button>
      <span>Page {current} of {pageCount}</span>
      <button type="button" class="btn btn-outline" disabled={current >= pageCount} onclick={() => (page = current + 1)}>
        Next
      </button>
    </nav>
  </div>
</section>
