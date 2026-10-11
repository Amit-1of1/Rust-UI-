import { useCallback, useEffect, useRef, useState } from 'react';
import * as Dialog from '@radix-ui/react-dialog';
import * as Switch from '@radix-ui/react-switch';
import * as Tabs from '@radix-ui/react-tabs';
import * as Toast from '@radix-ui/react-toast';
import { Command } from 'cmdk';
import { makeTasks, rowCountFromUrl, type Priority, type Task } from './data';
import TasksPanel, { type SortState } from './TasksPanel';
import StatsPanel from './StatsPanel';
import SettingsPanel from './SettingsPanel';

type TabValue = 'tasks' | 'stats' | 'settings';

export default function App() {
  const [tasks, setTasks] = useState<Task[]>(() => makeTasks(0, rowCountFromUrl()));
  const nextId = useRef(tasks.length + 1);
  const [dark, setDark] = useState(false);
  const [tab, setTab] = useState<TabValue>('tasks');
  const [pageSize, setPageSize] = useState(20);
  const [newTaskOpen, setNewTaskOpen] = useState(false);
  const [commandOpen, setCommandOpen] = useState(false);
  const [toasts, setToasts] = useState<number[]>([]);

  // Table view state lives here so it survives switching tabs.
  const [filter, setFilter] = useState('');
  const [sort, setSort] = useState<SortState | null>(null);
  const [page, setPage] = useState(1);
  const [selected, setSelected] = useState<ReadonlySet<number>>(() => new Set());

  useEffect(() => {
    document.body.dataset.ready = '1';
  }, []);

  useEffect(() => {
    document.documentElement.dataset.theme = dark ? 'dark' : 'light';
  }, [dark]);

  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        setCommandOpen((open) => !open);
      }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
  }, []);

  const addTask = useCallback((title: string, assignee: string, priority: Priority) => {
    const task: Task = { id: nextId.current++, title, assignee, priority, status: 'Todo', estimate: 1 };
    setTasks((t) => [task, ...t]);
    setPage(1);
    setNewTaskOpen(false);
    setToasts((t) => [...t, task.id]);
  }, []);

  // When a command opens another dialog, the command dialog must not pull focus back to its trigger.
  const keepFocus = useRef(false);
  const runCommand = (cmd: () => void, handsOffFocus = false) => {
    keepFocus.current = handsOffFocus;
    setCommandOpen(false);
    cmd();
  };

  return (
    <Toast.Provider swipeDirection="right">
      <header className="app-header">
        <h1>Taskboard</h1>
        <div className="header-actions">
          <div className="field-inline">
            <Switch.Root id="dark-mode" className="switch" checked={dark} onCheckedChange={setDark}>
              <Switch.Thumb className="switch-thumb" />
            </Switch.Root>
            <label htmlFor="dark-mode">Dark mode</label>
          </div>
          <Dialog.Root open={commandOpen} onOpenChange={setCommandOpen}>
            <Dialog.Trigger className="btn btn-outline">Command menu (Ctrl+K)</Dialog.Trigger>
            <Dialog.Portal>
              <Dialog.Overlay className="overlay" />
              <Dialog.Content
                className="dialog command-dialog"
                aria-describedby={undefined}
                onCloseAutoFocus={(e) => {
                  if (keepFocus.current) e.preventDefault();
                  keepFocus.current = false;
                }}
              >
                <Dialog.Title className="sr-only">Command menu</Dialog.Title>
                <Command label="Search commands" vimBindings={false} className="command">
                  <Command.Input className="command-input" placeholder="Type a command…" />
                  <Command.List label="Commands" className="command-list">
                    <Command.Empty className="command-empty">No results.</Command.Empty>
                    <Command.Item
                      onSelect={() =>
                        runCommand(() => {
                          setTab('tasks');
                          setNewTaskOpen(true);
                        }, true)
                      }
                    >
                      New task
                    </Command.Item>
                    <Command.Item onSelect={() => runCommand(() => setDark((d) => !d))}>Toggle dark mode</Command.Item>
                    <Command.Item onSelect={() => runCommand(() => setTab('stats'))}>Go to Stats</Command.Item>
                    <Command.Item onSelect={() => runCommand(() => setTab('settings'))}>Go to Settings</Command.Item>
                  </Command.List>
                </Command>
              </Dialog.Content>
            </Dialog.Portal>
          </Dialog.Root>
        </div>
      </header>

      <main className="app-main">
        <Tabs.Root className="tabs" value={tab} onValueChange={(v) => setTab(v as TabValue)}>
          <Tabs.List className="tabs-list" aria-label="Sections">
            <Tabs.Trigger className="tabs-trigger" value="tasks">Tasks</Tabs.Trigger>
            <Tabs.Trigger className="tabs-trigger" value="stats">Stats</Tabs.Trigger>
            <Tabs.Trigger className="tabs-trigger" value="settings">Settings</Tabs.Trigger>
          </Tabs.List>
          <Tabs.Content className="tabs-content" value="tasks">
            <TasksPanel
              tasks={tasks}
              pageSize={pageSize}
              filter={filter}
              onFilterChange={(f) => {
                setFilter(f);
                setPage(1);
              }}
              sort={sort}
              onSortChange={setSort}
              page={page}
              onPageChange={setPage}
              selected={selected}
              onSelectedChange={setSelected}
              newTaskOpen={newTaskOpen}
              onNewTaskOpenChange={setNewTaskOpen}
              onAddTask={addTask}
            />
          </Tabs.Content>
          <Tabs.Content className="tabs-content" value="stats">
            <StatsPanel tasks={tasks} />
          </Tabs.Content>
          <Tabs.Content className="tabs-content" value="settings">
            <SettingsPanel
              pageSize={pageSize}
              onPageSizeChange={(n) => {
                setPageSize(n);
                setPage(1);
              }}
            />
          </Tabs.Content>
        </Tabs.Root>
      </main>

      {toasts.map((id) => (
        <Toast.Root
          key={id}
          className="toast"
          onOpenChange={(open) => {
            if (!open) setToasts((t) => t.filter((x) => x !== id));
          }}
        >
          <Toast.Title className="toast-title">Task added</Toast.Title>
        </Toast.Root>
      ))}
      <Toast.Viewport className="toast-viewport" />
    </Toast.Provider>
  );
}
