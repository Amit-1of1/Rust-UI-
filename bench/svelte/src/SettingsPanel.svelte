<script lang="ts">
  import { Slider } from 'bits-ui';

  let { pageSize = $bindable() }: { pageSize: number } = $props();

  let notes = $state('');
  let notesEl: HTMLTextAreaElement;

  // Grow the textarea with its content.
  $effect(() => {
    void notes;
    notesEl.style.height = 'auto';
    notesEl.style.height = `${notesEl.scrollHeight + notesEl.offsetHeight - notesEl.clientHeight}px`;
  });
</script>

<section class="panel settings">
  <div class="field">
    <div class="progress-head">
      <span id="rows-per-page-label">Rows per page</span>
      <span class="muted">{pageSize}</span>
    </div>
    <Slider.Root type="single" class="slider" min={10} max={50} step={10} bind:value={pageSize}>
      {#snippet children({ thumbItems })}
        <span class="slider-track">
          <Slider.Range class="slider-range" />
        </span>
        {#each thumbItems as { index } (index)}
          <Slider.Thumb {index} class="slider-thumb" aria-labelledby="rows-per-page-label" />
        {/each}
      {/snippet}
    </Slider.Root>
  </div>

  <div class="field">
    <label for="notes">Notes</label>
    <textarea id="notes" bind:this={notesEl} class="input textarea" rows="3" bind:value={notes}></textarea>
  </div>
</section>
