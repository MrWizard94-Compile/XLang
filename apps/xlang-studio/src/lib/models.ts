export const DEFAULT_MODEL = "qwen2.5:3b";

export function chooseModel(
  availableModels: readonly string[],
  preferredModel: string,
  fallbackModel = DEFAULT_MODEL
): string {
  if (availableModels.includes(preferredModel)) {
    return preferredModel;
  }

  if (availableModels.includes(fallbackModel)) {
    return fallbackModel;
  }

  return availableModels[0] ?? (preferredModel || fallbackModel);
}
