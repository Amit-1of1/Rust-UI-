<script lang="ts">
  import { onMount } from 'svelte';
  import { Command, Dialog, Switch, Tabs } from 'bits-ui';
  import { Toaster, toast } from 'svelte-sonner';
  import { makeTasks, rowCountFromUrl, type Priority, type Task } from './data';
  import TasksPanel, { type SortState } from './TasksPanel.svelte';
  import StatsPanel from './StatsPanel.svelte';
  import SettingsPanel from './SettingsPanel.svelte';

  let tasks = $state<Task[]>(makeTasks(0, rowCountFromUrl()));
  let nextId = tasks.length + 1;
  let dark = $state(false);
  let tab = $state('tasks');
  let pageSize = $state(20);
  let newTaskOpen = $state(false);
  let commandOpen = $state(false);

  // Table view state lives here so it is shared with the command menu and settings.
  let filter = $state('');
  let sort = $state<SortState | null>(null);
  let page = $state(1);
  let selected = $state(new Set<number>());

  onMount(() => {
    document.body.dataset.ready = '1';
  });

  $effect(() => {
    document.documentElement.dataset.theme = dark ? 'dark' : 'light';
  });

  function onWindowKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
      e.preventDefault();
      commandOpen = !commandOpen;
    }
  }

  function addTask(title: string, assignee: string, priority: Priority) {
    tasks.unshift({ id: nextId++, title, assignee, priority, status: 'Todo', estimate: 1 });
    page = 1;
    newTaskOpen = false;
    toast('Task added');
  }

  // When a command opens another dialog, the command dialog must not pull focus back to its trigger.
  let keepFocus = false;
  function runCommand(cmd: () => void, handsOffFocus = false) {
    keepFocus = handsOffFocus;
    commandOpen = false;
    cmd();
  }
</script>

<svelte:window onkeydown={onWindowKeydown} />

<header class="app-header">
  <h1>Taskboard</h1>
  <div class="header-actions">
    <div class="field-inline">
      <Switch.Root id="dark-mode" class="switch" bind:checked={dark}>
        <Switch.Thumb class="switch-thumb" />
      </Switch.Root>
      <label for="dark-mode">Dark mode</label>
    </div>
    <Dialog.Root bind:open={commandOpen}>
      <Dialog.Trigger class="btn btn-outline">Command menu (Ctrl+K)</Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Overlay class="overlay" />
        <Dialog.Content
          class="dialog command-dialog"
          onCloseAutoFocus={(e) => {
            if (keepFocus) e.preventDefault();
            keepFocus = false;
          }}
        >
          <Dialog.Title class="sr-only">Command menu</Dialog.Title>
          <Command.Root label="Search commands" vimBindings={false} class="command">
            <Command.Input class="command-input" placeholder="Type a command…" />
            <Command.List class="command-list" aria-label="Commands">
              <Command.Viewport>
                <Command.Empty class="command-empty">No results.</Command.Empty>
                <Command.Item
                  onSelect={() =>
                    runCommand(() => {
                      tab = 'tasks';
                      newTaskOpen = true;
                    }, true)}
                >
                  New task
                </Command.Item>
                <Command.Item onSelect={() => runCommand(() => (dark = !dark))}>Toggle dark mode</Command.Item>
                <Command.Item onSelect={() => runCommand(() => (tab = 'stats'))}>Go to Stats</Command.Item>
                <Command.Item onSelect={() => runCommand(() => (tab = 'settings'))}>Go to Settings</Command.Item>
              </Command.Viewport>
            </Command.List>
          </Command.Root>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  </div>
</header>

<main class="app-main">
  <Tabs.Root class="tabs" bind:value={tab}>
    <Tabs.List class="tabs-list" aria-label="Sections">
      <Tabs.Trigger class="tabs-trigger" value="tasks">Tasks</Tabs.Trigger>
      <Tabs.Trigger class="tabs-trigger" value="stats">Stats</Tabs.Trigger>
      <Tabs.Trigger class="tabs-trigger" value="settings">Settings</Tabs.Trigger>
    </Tabs.List>
    <Tabs.Content class="tabs-content" value="tasks">
      <TasksPanel
        {tasks}
        {pageSize}
        bind:filter
        bind:sort
        bind:page
        bind:selected
        bind:newTaskOpen
        onAddTask={addTask}
      />
    </Tabs.Content>
    <Tabs.Content class="tabs-content" value="stats">
      <StatsPanel {tasks} />
    </Tabs.Content>
    <Tabs.Content class="tabs-content" value="settings">
      <SettingsPanel
        bind:pageSize={
          () => pageSize,
          (n) => {
            pageSize = n;
            page = 1;
          }
        }
      />
    </Tabs.Content>
  </Tabs.Root>
</main>

<Toaster theme={dark ? 'dark' : 'light'} position="bottom-right" />
