import { useLayoutEffect, useRef, useState } from 'react';
import * as Slider from '@radix-ui/react-slider';

interface Props {
  pageSize: number;
  onPageSizeChange: (n: number) => void;
}

export default function SettingsPanel({ pageSize, onPageSizeChange }: Props) {
  const [notes, setNotes] = useState('');
  const notesRef = useRef<HTMLTextAreaElement>(null);

  // Grow the textarea with its content.
  useLayoutEffect(() => {
    const el = notesRef.current;
    if (!el) return;
    el.style.height = 'auto';
    el.style.height = `${el.scrollHeight + el.offsetHeight - el.clientHeight}px`;
  }, [notes]);

  return (
    <section className="panel settings">
      <div className="field">
        <div className="progress-head">
          <span id="rows-per-page-label">Rows per page</span>
          <span className="muted">{pageSize}</span>
        </div>
        <Slider.Root
          className="slider"
          min={10}
          max={50}
          step={10}
          value={[pageSize]}
          onValueChange={([v]) => onPageSizeChange(v)}
        >
          <Slider.Track className="slider-track">
            <Slider.Range className="slider-range" />
          </Slider.Track>
          <Slider.Thumb className="slider-thumb" aria-labelledby="rows-per-page-label" />
        </Slider.Root>
      </div>

      <div className="field">
        <label htmlFor="notes">Notes</label>
        <textarea
          id="notes"
          ref={notesRef}
          className="input textarea"
          rows={3}
          value={notes}
          onChange={(e) => setNotes(e.target.value)}
        />
      </div>
    </section>
  );
}
