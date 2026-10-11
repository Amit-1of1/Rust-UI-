<script lang="ts">
  import { Progress } from 'bits-ui';
  import type { Status, Task } from './data';

  let { tasks }: { tasks: Task[] } = $props();

  const ORDER: Status[] = ['Todo', 'Doing', 'Done'];
  const W = 360;
  const H = 200;
  const chartH = 150;
  const barW = 72;
  const gap = (W - barW * ORDER.length) / (ORDER.length + 1);

  const counts = $derived.by(() => {
    const c: Record<Status, number> = { Todo: 0, Doing: 0, Done: 0 };
    for (const t of tasks) c[t.status]++;
    return c;
  });
  const max = $derived(Math.max(1, ...ORDER.map((s) => counts[s])));
  const percent = $derived(tasks.length ? Math.round((counts.Done / tasks.length) * 100) : 0);
  const summary = $derived(`Tasks per status: ${ORDER.map((s) => `${s} ${counts[s]}`).join(', ')}`);
</script>

<section class="panel stats">
  <h2>Tasks per status</h2>
  <svg role="img" aria-label={summary} viewBox="0 0 {W} {H}" class="chart">
    {#each ORDER as s, i (s)}
      {@const h = (counts[s] / max) * chartH}
      {@const x = gap + i * (barW + gap)}
      <g>
        <rect class="bar bar-{s.toLowerCase()}" {x} y={chartH - h + 16} width={barW} height={h} rx="4" />
        <text class="chart-value" x={x + barW / 2} y={chartH - h + 10} text-anchor="middle">{counts[s]}</text>
        <text class="chart-label" x={x + barW / 2} y={H - 12} text-anchor="middle">{s}</text>
      </g>
    {/each}
  </svg>

  <div class="field">
    <div class="progress-head">
      <span id="done-label">Done</span>
      <span class="muted">{percent}%</span>
    </div>
    <Progress.Root class="progress" value={percent} max={100} aria-labelledby="done-label">
      <div class="progress-indicator" style="transform: translateX(-{100 - percent}%)"></div>
    </Progress.Root>
  </div>
</section>
