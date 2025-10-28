import type {
  ClipboardRepository,
  ClipboardEntry,
  CreateClipboardEntryDto,
  UpdateClipboardEntryDto,
  GetItemsOptions
} from '../types'
import * as commands from './commands'

export class TauriClipboardRepository implements ClipboardRepository {
  async getItems(options?: GetItemsOptions): Promise<ClipboardEntry[]> {
    let items = await commands.tauriGetItems()

    // Apply filters
    if (options?.contentType) {
      items = items.filter(item => item.contentType === options.contentType)
    }

    if (options?.isFavorite !== undefined) {
      items = items.filter(item => item.isFavorite === options.isFavorite)
    }

    if (options?.isSnippet !== undefined) {
      items = items.filter(item => item.isSnippet === options.isSnippet)
    }

    // Apply pagination
    if (options?.offset !== undefined) {
      items = items.slice(options.offset)
    }

    if (options?.limit !== undefined) {
      items = items.slice(0, options.limit)
    }

    return items
  }

  async getItem(id: string): Promise<ClipboardEntry | null> {
    return await commands.tauriGetItem(id)
  }

  async searchItems(query: string): Promise<ClipboardEntry[]> {
    return await commands.tauriSearchItems(query)
  }

  async getFavorites(): Promise<ClipboardEntry[]> {
    return this.getItems({ isFavorite: true })
  }

  async getSnippets(): Promise<ClipboardEntry[]> {
    return this.getItems({ isSnippet: true })
  }

  async createItem(data: CreateClipboardEntryDto): Promise<ClipboardEntry> {
    return await commands.tauriCreateItem(data)
  }

  async updateItem(id: string, data: UpdateClipboardEntryDto): Promise<ClipboardEntry> {
    return await commands.tauriUpdateItem(id, data)
  }

  async deleteItem(id: string): Promise<void> {
    await commands.tauriDeleteItem(id)
  }

  async clearAll(): Promise<void> {
    await commands.tauriClearAllItems()
  }
}

// Singleton instance
export const clipboardRepository = new TauriClipboardRepository()