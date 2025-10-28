export type ContentType =
  | "text"
  | "rich_text"
  | "code"
  | "link"
  | "color"
  | "image"
  | "file";

export interface ClipboardEntry {
  id: string;
  contentType: ContentType;
  contentText: string | null;
  contentMetadata: Record<string, any>;

  // Files (futuro)
  fileUrl: string | null;
  fileName: string | null;
  fileSizeBytes: number | null;
  fileMimeType: string | null;

  // Organization
  isFavorite: boolean;
  isSnippet: boolean;
  snippetName: string | null;

  // Timestamps
  createdAt: Date;
  updatedAt: Date;

  // Sync (futuro - Pro)
  synced: boolean;
  serverId: string | null;
}

export interface CreateClipboardEntryDto {
  contentType: ContentType;
  contentText: string;
  contentMetadata?: Record<string, any>;
}

export interface UpdateClipboardEntryDto {
  contentText?: string;
  isFavorite?: boolean;
  isSnippet?: boolean;
  snippetName?: string;
}

// Repository interface (contrato)
export interface ClipboardRepository {
  // Queries
  getItems(options?: GetItemsOptions): Promise<ClipboardEntry[]>;
  getItem(id: string): Promise<ClipboardEntry | null>;
  searchItems(query: string): Promise<ClipboardEntry[]>;
  getFavorites(): Promise<ClipboardEntry[]>;
  getSnippets(): Promise<ClipboardEntry[]>;

  // Mutations
  createItem(data: CreateClipboardEntryDto): Promise<ClipboardEntry>;
  updateItem(id: string, data: UpdateClipboardEntryDto): Promise<ClipboardEntry>;
  deleteItem(id: string): Promise<void>;
  clearAll(): Promise<void>;
}

export interface GetItemsOptions {
  limit?: number;
  offset?: number;
  contentType?: ContentType;
  isFavorite?: boolean;
  isSnippet?: boolean;
}
