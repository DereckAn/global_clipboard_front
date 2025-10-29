<script lang="ts">
    import type { ClipboardItem } from '$lib/types'
    import SidebarItem from './SidebarItem.svelte'
    import Spinner from '$lib/components/ui/Spinner.svelte'
    import Icon from '$lib/components/icons/Icon.svelte'
    import { groupItemsByDate } from '$lib/utils/date'

    interface Props {
      items: ClipboardItem[]
      isLoading?: boolean
    }

    let { items, isLoading = false }: Props = $props()

    // Group items by date
    const groupedItems = $derived(groupItemsByDate(items))
  </script>

  <aside
    class="w-sidebar h-full bg-surface border-r border-border flex flex-col"
  >
    <!-- Items list -->
    <div class="flex-1 overflow-y-auto">
      {#if isLoading}
        <div class="flex items-center justify-center h-32">
          <Spinner size="md" />
        </div>
      {:else if items.length === 0}
        <div class="flex flex-col items-center justify-center h-full px-4 text-center">
          <Icon name="text" size={48} class="text-text-muted mb-4" />
          <p class="text-sm text-color-text-muted">
            No items yet
          </p>
          <p class="text-xs text-color-text-muted mt-1">
            Copy something to get started
          </p>
        </div>
      {:else}
        <div class="py-2">
          {#each groupedItems as group}
            <!-- Group header -->
            <div class="px-4 py-2 sticky top-0 bg-surface z-10 border-b border-border">
              <h3 class="text-xs font-semibold text-color-text-muted uppercase tracking-wider">
                {group.label}
              </h3>
            </div>

            <!-- Group items -->
            <div class="py-1">
              {#each group.items as item (item.id)}
                <SidebarItem {item} />
              {/each}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </aside>