import { memo, useCallback, useEffect, useRef, useState } from 'react';
import { makeTask } from './data';

interface BenchRow {
  id: number;
  title: string;
}

interface RowProps {
  row: BenchRow;
  selected: boolean;
  onSelect: (id: number) => void;
  onRemove: (id: number) => void;
}

const Row = memo(function Row({ row, selected, onSelect, onRemove }: RowProps) {
  return (
    <tr className={selected ? 'selected' : undefined}>
      <td>{row.id}</td>
      <td onClick={() => onSelect(row.id)}>{row.title}</td>
      <td>
        <button type="button" onClick={() => onRemove(row.id)}>
          ×
        </button>
      </td>
    </tr>
  );
});

export default function Bench() {
  const [rows, setRows] = useState<BenchRow[]>([]);
  const [selected, setSelected] = useState<number | null>(null);
  // Ids keep counting up across every create, as in js-framework-benchmark.
  const nextIndex = useRef(0);

  const build = useCallback((count: number): BenchRow[] => {
    const start = nextIndex.current;
    nextIndex.current += count;
    const out = new Array<BenchRow>(count);
    for (let k = 0; k < count; k++) {
      const { id, title } = makeTask(start + k);
      out[k] = { id, title };
    }
    return out;
  }, []);

  useEffect(() => {
    document.body.dataset.ready = '1';
  }, []);

  const run = () => {
    setRows(build(1000));
    setSelected(null);
  };
  const runLots = () => {
    setRows(build(10000));
    setSelected(null);
  };
  const add = () => setRows((r) => r.concat(build(1000)));
  const update = () =>
    setRows((r) => r.map((row, i) => (i % 10 === 0 ? { ...row, title: row.title + ' !!!' } : row)));
  const swapRows = () =>
    setRows((r) => {
      if (r.length < 999) return r;
      const next = r.slice();
      next[1] = r[998];
      next[998] = r[1];
      return next;
    });
  const clear = () => {
    setRows([]);
    setSelected(null);
  };

  const onSelect = useCallback((id: number) => setSelected(id), []);
  const onRemove = useCallback((id: number) => setRows((r) => r.filter((row) => row.id !== id)), []);

  return (
    <main className="bench">
      <div className="bench-buttons">
        <button type="button" id="run" onClick={run}>Create 1,000 rows</button>
        <button type="button" id="runlots" onClick={runLots}>Create 10,000 rows</button>
        <button type="button" id="add" onClick={add}>Append 1,000 rows</button>
        <button type="button" id="update" onClick={update}>Update every 10th row</button>
        <button type="button" id="swaprows" onClick={swapRows}>Swap rows</button>
        <button type="button" id="clear" onClick={clear}>Clear</button>
      </div>
      <table id="bench" className="table">
        <tbody>
          {rows.map((row) => (
            <Row key={row.id} row={row} selected={row.id === selected} onSelect={onSelect} onRemove={onRemove} />
          ))}
        </tbody>
      </table>
    </main>
  );
}
