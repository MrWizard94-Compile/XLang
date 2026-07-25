import { invoke } from "@tauri-apps/api/core";

export interface CompileResponse {
  success: boolean;
  artifact: string | null;
  runtimeOutput: string | null;
  exitCode: number | null;
  diagnostic: string | null;
}

export interface OllamaStatus {
  available: boolean;
  endpoint: string;
  defaultModel: string;
  models: string[];
  error: string | null;
}

export interface ReviewResponse {
  model: string;
  content: string;
}

export const isDesktopRuntime =
  typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

function requireDesktopRuntime(): void {
  if (!isDesktopRuntime) {
    throw new Error("Open Aether Studio through the desktop application to use compiler commands.");
  }
}

export async function compileSource(source: string): Promise<CompileResponse> {
  requireDesktopRuntime();
  return invoke<CompileResponse>("compile_source", { source });
}

export async function getOllamaStatus(): Promise<OllamaStatus> {
  requireDesktopRuntime();
  return invoke<OllamaStatus>("ollama_status");
}

export async function reviewSource(
  source: string,
  model: string
): Promise<ReviewResponse> {
  requireDesktopRuntime();
  return invoke<ReviewResponse>("review_source", { source, model });
}
