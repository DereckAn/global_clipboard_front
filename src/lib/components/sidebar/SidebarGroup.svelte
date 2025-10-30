<script lang="ts">
  import type { ClipboardItem } from "$lib/types";
  import SidebarItem from "./SidebarItem.svelte";

  interface Props {
    items: ClipboardItem[];
  }

  let { items }: Props = $props();

  // Función para agrupar items por fecha
  function groupItemsByDate(items: ClipboardItem[]) {
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const yesterday = new Date(today);
    yesterday.setDate(yesterday.getDate() - 1);
    const thisWeek = new Date(today);
    thisWeek.setDate(thisWeek.getDate() - 7);
    const thisMonth = new Date(today);
    thisMonth.setMonth(thisMonth.getMonth() - 1);

    const groups: Record<string, ClipboardItem[]> = {
      Today: [],
      Yesterday: [],
      "This Week": [],
      "This Month": [],
      Older: [],
    };

    items.forEach((item) => {
      const itemDate = new Date(item.createdAt);
      const itemDay = new Date(
        itemDate.getFullYear(),
        itemDate.getMonth(),
        itemDate.getDate()
      );

      if (itemDay.getTime() === today.getTime()) {
        groups.Today.push(item);
      } else if (itemDay.getTime() === yesterday.getTime()) {
        groups.Yesterday.push(item);
      } else if (itemDay >= thisWeek) {
        groups["This Week"].push(item);
      } else if (itemDay >= thisMonth) {
        groups["This Month"].push(item);
      } else {
        groups.Older.push(item);
      }
    });

    // Filtrar grupos vacíos
    return Object.entries(groups).filter(([_, items]) => items.length > 0);
  }

  const groupedItems = $derived(groupItemsByDate(items));
</script>

<div class="flex flex-col">
  {#each groupedItems as [groupName, groupItems]}
    <div class="mb-2">
      <!-- Group header -->
      <div class="px-3 py-2 sticky top-0 bg-surface/95 backdrop-blur-sm z-10">
        <h3
          class="text-xs font-semibold text-text-muted uppercase tracking-wide"
        >
          {groupName}
        </h3>
      </div>

      <!-- Group items -->
      <div class="space-y-0.5">
        {#each groupItems as item (item.id)}
          <SidebarItem {item} />
        {/each}
      </div>
    </div>
  {/each}
</div>
