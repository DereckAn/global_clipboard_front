import { vi } from "vitest";

// Mock de Tauri API - necesario porque Tauri no existe en el entorno de test
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
  emit: vi.fn(),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));
