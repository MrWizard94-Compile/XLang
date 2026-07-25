export const EXAMPLE_SOURCE = [
  "world genesis",
  "",
  "weave format_count [borrow label: Text, count: Whole] -> Text:",
  "  bind count_text <- render count",
  "  yield join borrow label borrow count_text",
  "",
  "weave main [] -> Whole:",
  "  bind label <- \"Aether Stage 1: \"",
  "  bind greeting <- call format_count borrow label 2",
  "  speak borrow greeting",
  "  yield 0",
  ""
].join("\n");
