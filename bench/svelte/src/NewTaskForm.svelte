<script lang="ts">
  import { Dialog, RadioGroup, Select } from 'bits-ui';
  import { NAMES, PRIORITIES, type Priority } from './data';

  interface Props {
    onAdd: (title: string, assignee: string, priority: Priority) => void;
    popupContainer: HTMLElement | null;
  }

  let { onAdd, popupContainer }: Props = $props();

  let title = $state('');
  let assignee = $state(NAMES[0]);
  let priority = $state<Priority>('Medium');
  let error = $state(false);
  let assigneeOpen = $state(false);
  let titleInput: HTMLInputElement;

  const items = NAMES.map((n) => ({ value: n, label: n }));

  function submit(e: SubmitEvent) {
    e.preventDefault();
    const value = title.trim();
    if (!value) {
      error = true;
      titleInput.focus();
      return;
    }
    onAdd(value, assignee, priority);
  }
</script>

<form class="form" novalidate onsubmit={submit}>
  <div class="field">
    <label for="task-title">Title</label>
    <input
      id="task-title"
      bind:this={titleInput}
      class="input"
      required
      bind:value={title}
      aria-invalid={error}
      aria-describedby={error ? 'task-title-error' : undefined}
      oninput={() => {
        if (error && title.trim()) error = false;
      }}
    />
    {#if error}
      <p id="task-title-error" class="field-error">Title is required</p>
    {/if}
  </div>

  <div class="field">
    <label id="task-assignee-label" for="task-assignee">Assignee</label>
    <Select.Root type="single" bind:value={assignee} bind:open={assigneeOpen} {items}>
      <!-- role=combobox (+ aria-controls): the trigger carries aria-activedescendant, which a plain button may not. -->
      <Select.Trigger
        id="task-assignee"
        role="combobox"
        aria-controls={assigneeOpen ? 'task-assignee-listbox' : undefined}
        class="select-trigger"
      >
        <span>{assignee}</span>
        <span class="select-icon" aria-hidden="true">▾</span>
      </Select.Trigger>
      <Select.Portal to={popupContainer ?? undefined}>
        <Select.Content
          id="task-assignee-listbox"
          class="select-content"
          sideOffset={4}
          aria-labelledby="task-assignee-label"
        >
          <Select.Viewport>
            {#each items as item (item.value)}
              <Select.Item value={item.value} label={item.label} class="select-item">
                {#snippet children({ selected })}
                  {item.label}
                  {#if selected}<span class="select-check" aria-hidden="true">✓</span>{/if}
                {/snippet}
              </Select.Item>
            {/each}
          </Select.Viewport>
        </Select.Content>
      </Select.Portal>
    </Select.Root>
  </div>

  <div class="field">
    <span id="task-priority-label" class="label">Priority</span>
    <RadioGroup.Root class="radio-group" aria-labelledby="task-priority-label" bind:value={priority}>
      {#each PRIORITIES as p (p)}
        <div class="field-inline">
          <RadioGroup.Item id="priority-{p}" value={p} class="radio">
            {#snippet children({ checked })}
              {#if checked}<span class="radio-indicator"></span>{/if}
            {/snippet}
          </RadioGroup.Item>
          <label for="priority-{p}">{p}</label>
        </div>
      {/each}
    </RadioGroup.Root>
  </div>

  <div class="dialog-actions">
    <Dialog.Close type="button" class="btn btn-outline">Cancel</Dialog.Close>
    <button type="submit" class="btn btn-primary">Save</button>
  </div>
</form>
