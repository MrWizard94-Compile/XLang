import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { loadModel, loadSource, saveModel, saveSource } from "./persistence";

class MemoryStorage {
  private readonly values = new Map<string, string>();

  getItem(key: string): string | null {
    return this.values.get(key) ?? null;
  }

  setItem(key: string, value: string): void {
    this.values.set(key, value);
  }

  keys(): string[] {
    return Array.from(this.values.keys()).sort((left, right) => left.localeCompare(right));
  }
}

describe("Studio local persistence", () => {
  let storage: MemoryStorage;

  beforeEach(() => {
    storage = new MemoryStorage();
    vi.stubGlobal("window", { localStorage: storage });
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("keeps source and model selections in WebView-local storage only", () => {
    const source = "world local\n\nweave main [] -> Whole:\n  yield 3\n";

    saveSource(source);
    saveModel("qwen2.5:7b");

    expect(loadSource()).toBe(source);
    expect(loadModel()).toBe("qwen2.5:7b");
    expect(storage.keys()).toEqual(["aether.model", "aether.source"]);
  });
});
