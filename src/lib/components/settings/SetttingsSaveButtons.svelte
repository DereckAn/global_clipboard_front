<script lang="ts">
  import Button from "../ui/Button.svelte";
  interface Props {
    onSave: () => void;
    onReset: () => void;
    onCancel: () => void;
  }
  let { onSave, onReset, onCancel }: Props = $props();
</script>

<section
  class="flex gap-3 border border-border/60 bg-surface p-2 rounded-2xl relative"
>
  <Button variant="other" onclick={onSave} class="h-fit text-sm bg-white/5 hover:bg-blue-500/20 rounded-md z-10">Save Changes</Button>
  <Button variant="ghost" onclick={onCancel} class="h-fit text-sm bg-white/5 rounded-md z-10">Cancel</Button>
  <Button variant="other" onclick={onReset} class="ml-auto h-fit text-sm bg-blue-500/50 rounded-md z-10">
    Reset to Defaults
  </Button>
</section>
    