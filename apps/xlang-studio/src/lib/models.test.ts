import { describe, expect, it } from "vitest";
import { chooseModel, DEFAULT_MODEL } from "./models";

describe("chooseModel", () => {
  it("keeps an installed preferred model", () => {
    expect(chooseModel(["qwen2.5:3b", "qwen2.5:7b"], "qwen2.5:7b")).toBe(
      "qwen2.5:7b"
    );
  });

  it("falls back to the default installed model", () => {
    expect(chooseModel([DEFAULT_MODEL], "missing:model")).toBe(DEFAULT_MODEL);
  });

  it("retains the prior model while the inventory is unavailable", () => {
    expect(chooseModel([], "qwen2.5-coder:7b")).toBe("qwen2.5-coder:7b");
  });
});
