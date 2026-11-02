---
needs: [dsl-parser]
---
# [design] A `.syl` formatter

Standard constraints apply. A formatter for `.syl` files, built on `deckmaste_dsl`'s tree
(`dsl-parser`). The style is not settled. The corpus's current layout was never chosen
deliberately; it may be a side effect of the conversion script. So this ticket opens with a style
dialogue with the owner and does not adopt the existing layout by default.

## Decisions already made

- Comments survive formatting. Each is an item in its sequence (`dsl-parser`), printed on its
  own line at that sequence's indent. Whitespace is the formatter's to decide, except where the
  tree records blank lines; which of those are kept is part of the style dialogue.
- The formatter works from the syntax tree, not from a lossless CST.

## Change

1. **Measure the corpus** to inform the dialogue: how uniform the current layout is, and how many
   files a candidate style would rewrite.
2. **Style dialogue** with the owner. Record the resulting rules in this ticket.
3. **Printer** in `deckmaste_dsl`, an xtask entry point to format or check files, and a
   `jj fix` tool entry for `glob:**/*.syl` next to the existing rustfmt tools.
4. Reformat the corpus in its own commit.

## Acceptance

- Idempotent on the whole corpus.
- Formatting preserves meaning: the formatted file parses to the same tree, ignoring spans.
- Per-crate tests and clippy for `deckmaste_dsl` and `xtask`.
