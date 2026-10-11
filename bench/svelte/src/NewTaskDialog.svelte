<script lang="ts">
  import { Dialog } from 'bits-ui';
  import type { Priority } from './data';
  import NewTaskForm from './NewTaskForm.svelte';

  interface Props {
    open: boolean;
    onAdd: (title: string, assignee: string, priority: Priority) => void;
  }

  let { open = $bindable(), onAdd }: Props = $props();
  // The select popup is portalled into the dialog so it stays inside the modal's subtree.
  let content = $state<HTMLElement | null>(null);
  // Focus always returns to "New task", also when the dialog was opened from the command menu.
  let trigger = $state<HTMLElement | null>(null);
</script>

<Dialog.Root bind:open>
  <Dialog.Trigger class="btn btn-primary" bind:ref={trigger}>New task</Dialog.Trigger>
  <Dialog.Portal>
    <Dialog.Overlay class="overlay" />
    <Dialog.Content
      class="dialog"
      bind:ref={content}
      onCloseAutoFocus={(e) => {
        e.preventDefault();
        trigger?.focus();
      }}
    >
      <Dialog.Title class="dialog-title">New task</Dialog.Title>
      <Dialog.Description class="muted">Add a task to the top of the table.</Dialog.Description>
      <!-- Mounted fresh each time the dialog opens, so the form state starts empty. -->
      <NewTaskForm {onAdd} popupContainer={content} />
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
