import { tauriRemoveDuplicates } from "$lib/tauri/commands";
import { clipboardRepository } from "$lib/tauri/storage";
import type {
  ClipboardItem,
  CreateClipboardItemDto,
  GetItemsOptions,
  UpdateClipboardItemDto,
} from "$lib/types";

class ClipboardStore {
  items = $state<ClipboardItem[]>([]);
  isLoading = $state(false);
  error = $state<string | null>(null);

  // Load items
  async loadItems(options?: GetItemsOptions) {
    this.isLoading = true;
    this.error = null;

    try {
      this.items = await clipboardRepository.getItems(options);
    } catch (err) {
      this.error = err instanceof Error ? err.message : "Failed to load items";
      console.error("Failed to load items:", err);
    } finally {
      this.isLoading = false;
    }
  }

  // Create item
  async createItem(data: CreateClipboardItemDto) {
    try {
      const newItem = await clipboardRepository.createItem(data);
      this.items = [newItem, ...this.items];
      return newItem;
    } catch (err) {
      this.error = err instanceof Error ? err.message : "Failed to create item";
      throw err;
    }
  }

  // Update item
  async updateItem(id: string, data: UpdateClipboardItemDto) {
    try {
      const updatedItem = await clipboardRepository.updateItem(id, data);
      this.items = this.items.map((item) =>
        item.id === id ? updatedItem : item
      );
      return updatedItem;
    } catch (err) {
      this.error = err instanceof Error ? err.message : "Failed to update item";
      throw err;
    }
  }

  // Delete item
  async deleteItem(id: string) {
    try {
      await clipboardRepository.deleteItem(id);
      this.items = this.items.filter((item) => item.id !== id);
    } catch (err) {
      this.error = err instanceof Error ? err.message : "Failed to delete item";
      throw err;
    }
  }

  // Toggle favorite
  async toggleFavorite(id: string) {
    const item = this.items.find((i) => i.id === id);
    if (!item) return;

    await this.updateItem(id, { isFavorite: !item.isFavorite });
  }

  // Clear all
  async clearAll() {
    try {
      await clipboardRepository.clearAll();
      this.items = [];
    } catch (err) {
      this.error = err instanceof Error ? err.message : "Failed to clear items";
      throw err;
    }
  }

  // Search
  async search(query: string) {
    if (!query.trim()) {
      return [];
    }

    try {
      return await clipboardRepository.searchItems(query);
    } catch (err) {
      this.error = err instanceof Error ? err.message : "Failed to search";
      throw err;
    }
  }

  // Remove duplicates
  async removeDuplicates() {
    try {
      const deletedCount = await tauriRemoveDuplicates();
      await this.loadItems(); // Reload items
      return deletedCount;
    } catch (err) {
      this.error =
        err instanceof Error ? err.message : "Failed to remove duplicates";
      throw err;
    }
  }
}

export const clipboardStore = new ClipboardStore();
