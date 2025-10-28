import type {
  ClipboardItem,
  CreateClipboardItemDto,
  UpdateClipboardItemDto,
} from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

// ============================================
// CLIPBOARD COMMANDS
// ============================================

export async function tauriGetItems(): Promise<ClipboardItem[]> {
  const items = await invoke<any[]>("get_clipboard_items");
  return items.map(deserializeClipboardItem);
}

export async function tauriGetItem(id: string): Promise<ClipboardItem | null> {
  try {
    const item = await invoke<any>("get_clipboard_item", { id });
    return deserializeClipboardItem(item);
  } catch (error) {
    console.error("Error getting item:", error);
    return null;
  }
}

export async function tauriCreateItem(
  data: CreateClipboardItemDto
): Promise<ClipboardItem> {
  const item = await invoke<any>("create_clipboard_item", { data });
  return deserializeClipboardItem(item);
}

export async function tauriUpdateItem(
  id: string,
  data: UpdateClipboardItemDto
): Promise<ClipboardItem> {
  const item = await invoke<any>("update_clipboard_item", { id, data });
  return deserializeClipboardItem(item);
}

export async function tauriDeleteItem(id: string): Promise<void> {
  await invoke("delete_clipboard_item", { id });
}

export async function tauriSearchItems(
  query: string
): Promise<ClipboardItem[]> {
  const items = await invoke<any[]>("search_clipboard_items", { query });
  return items.map(deserializeClipboardItem);
}

export async function tauriClearAllItems(): Promise<void> {
  await invoke("clear_all_clipboard_items");
}

// ============================================
// CLIPBOARD OPERATIONS (OS)
// ============================================

export async function tauriWriteToClipboard(text: string): Promise<void> {
  await invoke("write_to_clipboard", { text });
}

export async function tauriReadFromClipboard(): Promise<string> {
  return await invoke<string>("read_from_clipboard");
}

// ============================================
// HELPERS
// ============================================

function deserializeClipboardItem(item: any): ClipboardItem {
  return {
    ...item,
    createdAt: new Date(item.createdAt),
    updatedAt: new Date(item.updatedAt),
  };
}
