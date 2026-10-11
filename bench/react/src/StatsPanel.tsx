import * as Progress from '@radix-ui/react-progress';
import type { Status, Task } from './data';

const ORDER: Status[] = ['Todo', 'Doing', 'Done'];

export default function StatsPanel({ tasks }: { tasks: Task[] }) {
  const counts: Record<Status, number> = { Todo: 0, Doing: 0, Done: 0 };
  for (const t of tasks) counts[t.status]++;
  const max = Math.max(1, ...ORDER.map((s) => counts[s]));
  const percent = tasks.length ? Math.round((counts.Done / tasks.length) * 100) : 0;
  const summary = `Tasks per status: ${ORDER.map((s) => `${s} ${counts[s]}`).join(', ')}`;

  const W = 360;
  const H = 200;
  const chartH = 150;
  const barW = 72;
  const gap = (W - barW * ORDER.length) / (ORDER.length + 1);

  return (
    <section className="panel stats">
      <h2>Tasks per status</h2>
      <svg role="img" aria-label={summary} viewBox={`0 0 ${W} ${H}`} className="chart">
        {ORDER.map((s, i) => {
          const h = (counts[s] / max) * chartH;
          const x = gap + i * (barW + gap);
          return (
            <g key={s}>
              <rect className={`bar bar-${s.toLowerCase()}`} x={x} y={chartH - h + 16} width={barW} height={h} rx={4} />
              <text className="chart-value" x={x + barW / 2} y={chartH - h + 10} textAnchor="middle">
                {counts[s]}
              </text>
              <text className="chart-label" x={x + barW / 2} y={H - 12} textAnchor="middle">
                {s}
              </text>
            </g>
          );
        })}
      </svg>

      <div className="field">
        <div className="progress-head">
          <span id="done-label">Done</span>
          <span className="muted">{percent}%</span>
        </div>
        <Progress.Root className="progress" value={percent} aria-labelledby="done-label">
          <Progress.Indicator className="progress-indicator" style={{ transform: `translateX(-${100 - percent}%)` }} />
        </Progress.Root>
      </div>
    </section>
  );
}
