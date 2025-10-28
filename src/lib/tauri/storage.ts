import type { 
  ClipboardRepository, 
  ClipboardItem, 
  CreateClipboardItemDto,
  UpdateClipboardItemDto,
  GetItemsOptions
} from '$lib/types'
import * as commands from './commands'

export class TauriClipboardRepository implements ClipboardRepository {
  async getItems(options?: GetItemsOptions): Promise<ClipboardItem[]> {
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
  
  async getItem(id: string): Promise<ClipboardItem | null> {
    return await commands.tauriGetItem(id)
  }
  
  async searchItems(query: string): Promise<ClipboardItem[]> {
    return await commands.tauriSearchItems(query)
  }
  
  async getFavorites(): Promise<ClipboardItem[]> {
    return this.getItems({ isFavorite: true })
  }
  
  async getSnippets(): Promise<ClipboardItem[]> {
    return this.getItems({ isSnippet: true })
  }
  
  async createItem(data: CreateClipboardItemDto): Promise<ClipboardItem> {
    return await commands.tauriCreateItem(data)
  }
  
  async updateItem(id: string, data: UpdateClipboardItemDto): Promise<ClipboardItem> {
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