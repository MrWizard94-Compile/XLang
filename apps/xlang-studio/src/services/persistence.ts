import { EXAMPLE_SOURCE } from "../lib/example";
import { DEFAULT_MODEL } from "../lib/models";

const SOURCE_KEY = "xlang.source";
const MODEL_KEY = "xlang.model";

function readValue(key: string, fallback: string): string {
  try {
    return window.localStorage.getItem(key) ?? fallback;
  } catch {
    return fallback;
  }
}

function writeValue(key: string, value: string): void {
  try {
    window.localStorage.setItem(key, value);
  } catch {
    // Local persistence is optional; the current editor session remains usable.
  }
}

export function loadSource(): string {
  return readValue(SOURCE_KEY, EXAMPLE_SOURCE);
}

export function saveSource(source: string): void {
  writeValue(SOURCE_KEY, source);
}

export function loadModel(): string {
  return readValue(MODEL_KEY, DEFAULT_MODEL);
}

export function saveModel(model: string): void {
  writeValue(MODEL_KEY, model);
}
