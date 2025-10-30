import type {
  ClipboardItem,
  CreateClipboardItemDto,
  UpdateClipboardItemDto,
} from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

// Helper to deserialize dates from strings
function deserializeClipboardItem(item: any): ClipboardItem {
  return {
    ...item,
    createdAt: new Date(item.created_at),
    updatedAt: new Date(item.updated_at),
    // Map snake_case from Rust to camelCase for TypeScript
    contentType: item.content_type,
    contentText: item.content_text,
    contentMetadata: item.content_metadata
      ? JSON.parse(item.content_metadata)
      : {},
    sourceApp: item.source_app,
    codeLanguage: item.code_language,
    fileUrl: item.file_url,
    fileName: item.file_name,
    fileSizeBytes: item.file_size_bytes,
    fileMimeType: item.file_mime_type,
    isFavorite: item.is_favorite,
    isSnippet: item.is_snippet,
    snippetName: item.snippet_name,
    synced: item.synced,
    serverId: item.server_id,
  };
}

export async function tauriGetItems(): Promise<ClipboardItem[]> {
  const items = await invoke<any[]>("get_clipboard_items");
  return items.map(deserializeClipboardItem);
}

export async function tauriGetItem(id: string): Promise<ClipboardItem | null> {
  const item = await invoke<any | null>("get_clipboard_item", { id });
  return item ? deserializeClipboardItem(item) : null;
}

export async function tauriCreateItem(
  dto: CreateClipboardItemDto
): Promise<ClipboardItem> {
  // Convert to snake_case for Rust
  const rustDto = {
    content_type: dto.contentType,
    content_text: dto.contentText,
    content_metadata: dto.contentMetadata
      ? JSON.stringify(dto.contentMetadata)
      : undefined,
    source_app: undefined, // Will be detected by backend
  };

  const item = await invoke<any>("create_clipboard_item", { dto: rustDto });
  return deserializeClipboardItem(item);
}

export async function tauriUpdateItem(
  id: string,
  dto: UpdateClipboardItemDto
): Promise<ClipboardItem> {
  // Convert to snake_case for Rust
  const rustDto = {
    content_text: dto.contentText,
    is_favorite: dto.isFavorite,
    is_snippet: dto.isSnippet,
    snippet_name: dto.snippetName,
  };

  const item = await invoke<any>("update_clipboard_item", { id, dto: rustDto });
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

export async function tauriWriteToClipboard(text: string): Promise<void> {
  await invoke("write_to_clipboard", { text });
}

export async function tauriReadFromClipboard(): Promise<string> {
  return await invoke<string>("read_from_clipboard");
}

// Color conversion types
export interface ColorFormat {
  name: string;
  value: string;
  original: boolean;
}

export interface ColorConversion {
  formats: ColorFormat[];
  rgb_preview: string;
}

export interface LinkMetadata {
  title: string | null;
  description: string | null;
  image: string | null;
  site_name: string | null;
}

// Color conversion
export async function tauriConvertColor(
  color: string
): Promise<ColorConversion> {
  return await invoke("convert_color_formats", { color });
}

// Extract domain from URL
export async function tauriExtractDomain(url: string): Promise<string> {
  return await invoke("extract_domain_from_url", { url });
}

// Fetch link metadata
export async function tauriFetchLinkMetadata(
  url: string
): Promise<LinkMetadata> {
  return await invoke("fetch_link_metadata", { url });
}

// Remove duplicate items
export async function tauriRemoveDuplicates(): Promise<number> {
  return await invoke("remove_duplicate_items");
}

// Settings
export async function tauriGetSetting(key: string): Promise<string> {
  return await invoke("get_setting", { key });
}

export async function tauriSaveSetting(
  key: string,
  value: string
): Promise<void> {
  await invoke("save_setting", { key, value });
}

export async function tauriUpdateGlobalHotkey(
  newHotkey: string
): Promise<void> {
  await invoke("update_global_hotkey", { newHotkey });
}
