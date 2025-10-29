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
  tauriGetItems,
  tauriSearchItems,
  tauriUpdateItem,
} from "./commands";

class TauriClipboardRepository implements ClipboardRepository {
  async getItems(options?: GetItemsOptions): Promise<ClipboardItem[]> {
    let items = await tauriGetItems();

    // Apply filters
    if (options?.contentType) {
      items = items.filter((item) => item.contentType === options.contentType);
    }

    if (options?.isFavorite !== undefined) {
      items = items.filter((item) => item.isFavorite === options.isFavorite);
    }

    if (options?.isSnippet !== undefined) {
      items = items.filter((item) => item.isSnippet === options.isSnippet);
    }

    // Apply pagination
    if (options?.offset !== undefined) {
      items = items.slice(options.offset);
    }

    if (options?.limit !== undefined) {
      items = items.slice(0, options.limit);
    }

    return items;
  }

  async getItem(id: string): Promise<ClipboardItem | null> {
    return await tauriGetItem(id);
  }

  async searchItems(query: string): Promise<ClipboardItem[]> {
    return await tauriSearchItems(query);
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
