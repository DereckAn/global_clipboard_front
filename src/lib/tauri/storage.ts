import type {
  ClipboardItem,
  ClipboardRepository,
  CreateClipboardItemDto,
  GetItemsOptions,
  UpdateClipboardItemDto,
} from "$lib/types";
import {
  tauriClearAllItems,
  tauriCreateItem,
  tauriDeleteItem,
  tauriGetItem,
  tauriGetItemsPaginated,
  tauriSearchItemsPaginated,
  tauriUpdateItem,
} from "./commands";

class TauriClipboardRepository implements ClipboardRepository {
  async getItems(options?: GetItemsOptions): Promise<ClipboardItem[]> {
    // Use paginated version with default limit
    const limit = options?.limit || 100;
    const offset = options?.offset || 0;

    let items = await tauriGetItemsPaginated(limit, offset);

    // Apply filters (client-side for now)
    if (options?.contentType) {
      items = items.filter((item) => item.contentType === options.contentType);
    }

    if (options?.isFavorite !== undefined) {
      items = items.filter((item) => item.isFavorite === options.isFavorite);
    }

    if (options?.isSnippet !== undefined) {
      items = items.filter((item) => item.isSnippet === options.isSnippet);
    }

    return items;
  }

  async getItem(id: string): Promise<ClipboardItem | null> {
    return await tauriGetItem(id);
  }

  async searchItems(query: string): Promise<ClipboardItem[]> {
    // Use paginated search with default limit
    return await tauriSearchItemsPaginated(query, 100, 0);
  }

  async getFavorites(): Promise<ClipboardItem[]> {
    return await this.getItems({ isFavorite: true });
  }

  async getSnippets(): Promise<ClipboardItem[]> {
    return await this.getItems({ isSnippet: true });
  }

  async createItem(data: CreateClipboardItemDto): Promise<ClipboardItem> {
    return await tauriCreateItem(data);
  }

  async updateItem(
    id: string,
    data: UpdateClipboardItemDto
  ): Promise<ClipboardItem> {
    return await tauriUpdateItem(id, data);
  }

  async deleteItem(id: string): Promise<void> {
    await tauriDeleteItem(id);
  }

  async clearAll(): Promise<void> {
    await tauriClearAllItems();
  }
}

export const clipboardRepository = new TauriClipboardRepository();
