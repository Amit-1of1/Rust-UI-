import { useMemo } from 'react';
import * as Checkbox from '@radix-ui/react-checkbox';
import type { Priority, Task } from './data';
import NewTaskDialog from './NewTaskDialog';

export type SortKey = 'title' | 'assignee' | 'priority' | 'estimate';
export interface SortState {
  key: SortKey;
  dir: 'asc' | 'desc';
}

interface Props {
  tasks: Task[];
  pageSize: number;
  filter: string;
  onFilterChange: (value: string) => void;
  sort: SortState | null;
  onSortChange: (sort: SortState) => void;
  page: number;
  onPageChange: (page: number) => void;
  selected: ReadonlySet<number>;
  onSelectedChange: (selected: ReadonlySet<number>) => void;
  newTaskOpen: boolean;
  onNewTaskOpenChange: (open: boolean) => void;
  onAddTask: (title: string, assignee: string, priority: Priority) => void;
}

const PRIORITY_RANK: Record<Priority, number> = { Low: 0, Medium: 1, High: 2 };

function compareTasks(a: Task, b: Task, sort: SortState): number {
  let c: number;
  if (sort.key === 'priority') c = PRIORITY_RANK[a.priority] - PRIORITY_RANK[b.priority];
  else if (sort.key === 'estimate') c = a.estimate - b.estimate;
  else {
    const x = a[sort.key];
    const y = b[sort.key];
    c = x < y ? -1 : x > y ? 1 : 0;
  }
  return sort.dir === 'asc' ? c : -c;
}

const COLUMNS: { key: SortKey; label: string }[] = [
  { key: 'title', label: 'Title' },
  { key: 'assignee', label: 'Assignee' },
  { key: 'priority', label: 'Priority' },
];

function CheckIcon() {
  return (
    <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true">
      <path d="M3 8.5l3 3 7-7" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
}

function MinusIcon() {
  return (
    <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true">
      <path d="M4 8h8" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  );
}

export default function TasksPanel(props: Props) {
  const { tasks, pageSize, filter, sort, page, selected } = props;

  const rows = useMemo(() => {
    const q = filter.toLowerCase();
    const out = q ? tasks.filter((t) => t.title.toLowerCase().includes(q)) : tasks.slice();
    if (sort) out.sort((a, b) => compareTasks(a, b, sort));
    return out;
  }, [tasks, filter, sort]);

  const total = rows.length;
  const pageCount = Math.max(1, Math.ceil(total / pageSize));
  const current = Math.min(page, pageCount);
  const start = (current - 1) * pageSize;
  const pageRows = rows.slice(start, start + pageSize);
  const from = total === 0 ? 0 : start + 1;
  const to = start + pageRows.length;

  const selectedOnPage = pageRows.filter((t) => selected.has(t.id)).length;
  const allChecked: Checkbox.CheckedState =
    pageRows.length > 0 && selectedOnPage === pageRows.length ? true : selectedOnPage > 0 ? 'indeterminate' : false;

  const toggleAll = () => {
    const next = new Set(selected);
    if (allChecked === true) pageRows.forEach((t) => next.delete(t.id));
    else pageRows.forEach((t) => next.add(t.id));
    props.onSelectedChange(next);
  };

  const toggleRow = (id: number, on: boolean) => {
    const next = new Set(selected);
    if (on) next.add(id);
    else next.delete(id);
    props.onSelectedChange(next);
  };

  const sortBy = (key: SortKey) =>
    props.onSortChange({ key, dir: sort?.key === key && sort.dir === 'asc' ? 'desc' : 'asc' });

  const ariaSort = (key: SortKey) =>
    sort?.key === key ? (sort.dir === 'asc' ? 'ascending' : 'descending') : 'none';

  const sortHeader = (key: SortKey, label: string, className?: string) => (
    <th key={key} scope="col" aria-sort={ariaSort(key)} className={className}>
      <button type="button" className="sort-button" onClick={() => sortBy(key)}>
        {label}
        <span className="sort-icon" aria-hidden="true">
          {sort?.key === key ? (sort.dir === 'asc' ? '▲' : '▼') : '↕'}
        </span>
      </button>
    </th>
  );

  return (
    <section className="panel">
      <div className="toolbar">
        <div className="field">
          <label htmlFor="filter">Filter tasks</label>
          <input
            id="filter"
            className="input"
            type="search"
            value={filter}
            onChange={(e) => props.onFilterChange(e.target.value)}
          />
        </div>
        <NewTaskDialog open={props.newTaskOpen} onOpenChange={props.onNewTaskOpenChange} onAdd={props.onAddTask} />
      </div>

      <div className="table-wrap">
        <table className="table">
          <caption>Tasks</caption>
          <thead>
            <tr>
              <th scope="col" className="col-select">
                <span className="sr-only">Select</span>
                <Checkbox.Root
                  className="checkbox"
                  aria-label="Select all rows on this page"
                  checked={allChecked}
                  onCheckedChange={toggleAll}
                >
                  <Checkbox.Indicator className="checkbox-indicator">
                    {allChecked === 'indeterminate' ? <MinusIcon /> : <CheckIcon />}
                  </Checkbox.Indicator>
                </Checkbox.Root>
              </th>
              {COLUMNS.map((c) => sortHeader(c.key, c.label))}
              <th scope="col">Status</th>
              {sortHeader('estimate', 'Estimate', 'num')}
            </tr>
          </thead>
          <tbody>
            {pageRows.map((t) => (
              <tr key={t.id} data-state={selected.has(t.id) ? 'selected' : undefined}>
                <td className="col-select">
                  <Checkbox.Root
                    className="checkbox"
                    aria-label={`Select ${t.title}`}
                    checked={selected.has(t.id)}
                    onCheckedChange={(c) => toggleRow(t.id, c === true)}
                  >
                    <Checkbox.Indicator className="checkbox-indicator">
                      <CheckIcon />
                    </Checkbox.Indicator>
                  </Checkbox.Root>
                </td>
                <td>{t.title}</td>
                <td>{t.assignee}</td>
                <td>
                  <span className={`badge badge-${t.priority.toLowerCase()}`}>{t.priority}</span>
                </td>
                <td>{t.status}</td>
                <td className="num">{t.estimate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="table-footer">
        <p role="status" className="muted">
          Showing {from}–{to} of {total} rows
        </p>
        <nav aria-label="Table pages" className="pager">
          <button
            type="button"
            className="btn btn-outline"
            disabled={current <= 1}
            onClick={() => props.onPageChange(current - 1)}
          >
            Previous
          </button>
          <span>
            Page {current} of {pageCount}
          </span>
          <button
            type="button"
            className="btn btn-outline"
            disabled={current >= pageCount}
            onClick={() => props.onPageChange(current + 1)}
          >
            Next
          </button>
        </nav>
      </div>
    </section>
  );
}
