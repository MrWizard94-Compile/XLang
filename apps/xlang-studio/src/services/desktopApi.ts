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

export interface AuthoringDiagnostic {
  schema: string;
  code: string;
  span: {
    line: number;
    column: number;
  };
  message: string;
}

export interface StructuralAuthoringResponse {
  success: boolean;
  source: string | null;
  document: string | null;
  operationCount: number | null;
  diagnostic: AuthoringDiagnostic | null;
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

export async function inspectStructure(source: string): Promise<StructuralAuthoringResponse> {
  requireDesktopRuntime();
  return invoke<StructuralAuthoringResponse>("inspect_structure", { source });
}

export async function applyStructuralEdit(
  source: string,
  edit: string
): Promise<StructuralAuthoringResponse> {
  requireDesktopRuntime();
  return invoke<StructuralAuthoringResponse>("apply_structural_edit", { source, edit });
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
