import { useRef, useState, type FormEvent } from 'react';
import * as Dialog from '@radix-ui/react-dialog';
import * as RadioGroup from '@radix-ui/react-radio-group';
import * as Select from '@radix-ui/react-select';
import { NAMES, PRIORITIES, type Priority } from './data';

interface Props {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onAdd: (title: string, assignee: string, priority: Priority) => void;
}

export default function NewTaskDialog({ open, onOpenChange, onAdd }: Props) {
  // The select popup is portalled into the dialog so it stays inside the modal's subtree.
  const [content, setContent] = useState<HTMLDivElement | null>(null);
  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Trigger className="btn btn-primary">New task</Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay className="overlay" />
        <Dialog.Content className="dialog" ref={setContent}>
          <Dialog.Title className="dialog-title">New task</Dialog.Title>
          <Dialog.Description className="muted">Add a task to the top of the table.</Dialog.Description>
          <NewTaskForm onAdd={onAdd} popupContainer={content} />
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}

// Mounted fresh each time the dialog opens, so the form state starts empty.
function NewTaskForm({ onAdd, popupContainer }: Pick<Props, 'onAdd'> & { popupContainer: HTMLElement | null }) {
  const [title, setTitle] = useState('');
  const [assignee, setAssignee] = useState(NAMES[0]);
  const [priority, setPriority] = useState<Priority>('Medium');
  const [error, setError] = useState(false);
  const titleRef = useRef<HTMLInputElement>(null);

  const submit = (e: FormEvent) => {
    e.preventDefault();
    const value = title.trim();
    if (!value) {
      setError(true);
      titleRef.current?.focus();
      return;
    }
    onAdd(value, assignee, priority);
  };

  return (
    <form className="form" noValidate onSubmit={submit}>
      <div className="field">
        <label htmlFor="task-title">Title</label>
        <input
          id="task-title"
          ref={titleRef}
          className="input"
          required
          value={title}
          aria-invalid={error}
          aria-describedby={error ? 'task-title-error' : undefined}
          onChange={(e) => {
            setTitle(e.target.value);
            if (error && e.target.value.trim()) setError(false);
          }}
        />
        {error && (
          <p id="task-title-error" className="field-error">
            Title is required
          </p>
        )}
      </div>

      <div className="field">
        <label htmlFor="task-assignee">Assignee</label>
        <Select.Root value={assignee} onValueChange={setAssignee}>
          <Select.Trigger id="task-assignee" className="select-trigger">
            <Select.Value />
            <Select.Icon className="select-icon">▾</Select.Icon>
          </Select.Trigger>
          <Select.Portal container={popupContainer}>
            <Select.Content className="select-content" position="popper" sideOffset={4}>
              <Select.Viewport>
                {NAMES.map((name) => (
                  <Select.Item key={name} value={name} className="select-item">
                    <Select.ItemText>{name}</Select.ItemText>
                    <Select.ItemIndicator className="select-check">✓</Select.ItemIndicator>
                  </Select.Item>
                ))}
              </Select.Viewport>
            </Select.Content>
          </Select.Portal>
        </Select.Root>
      </div>

      <div className="field">
        <span id="task-priority-label" className="label">
          Priority
        </span>
        <RadioGroup.Root
          className="radio-group"
          aria-labelledby="task-priority-label"
          value={priority}
          onValueChange={(v) => setPriority(v as Priority)}
        >
          {PRIORITIES.map((p) => (
            <div key={p} className="field-inline">
              <RadioGroup.Item id={`priority-${p}`} value={p} className="radio">
                <RadioGroup.Indicator className="radio-indicator" />
              </RadioGroup.Item>
              <label htmlFor={`priority-${p}`}>{p}</label>
            </div>
          ))}
        </RadioGroup.Root>
      </div>

      <div className="dialog-actions">
        <Dialog.Close type="button" className="btn btn-outline">
          Cancel
        </Dialog.Close>
        <button type="submit" className="btn btn-primary">
          Save
        </button>
      </div>
    </form>
  );
}
