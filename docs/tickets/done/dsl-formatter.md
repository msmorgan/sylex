---
needs: [dsl-parser, syl-parameter-sigil]
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

## Style (owner dialogue, 2026-11-02)

The measurement found one consistent rule in the current layout: 99.8% of break decisions follow
it. The style keeps that rule and adds one change, list hugging (candidate C).

- Width 100. A node fits only if the closers and comma that follow it on the same line fit too.
  An unbreakable string may run past the width.
- 2-space block indent, never aligned.
- A node goes on one line if it fits. Otherwise every child goes on its own line, never packed,
  and outer nodes break first. A broken node's first child goes on the next line.
- Inner closers hug the last child, Lisp style (`...)]))`). A declaration body's `}` goes on its
  own line.
- A list or record that is a call's first positional argument hugs the call's `(`, as in
  `and([` and `conjunction([`. Only the opener moves: the children stay at their true depth, two
  levels below the call line, and any later arguments follow one level in, as in any broken
  call. A call argument does not hug. The printer falls back to the unhugged layout only when
  that takes strictly fewer lines.
- No trailing commas inside nodes, since hugged closers rule them out.
- Flat records have no inner spaces: `{a = b}`. Separators are `, `, `: ` and ` = `.
- A broken parameter list follows the call rule.
- Blank lines: a recorded blank line is kept as one blank line. None at the start or end of a
  sequence or file, none between a doc comment and its declaration. Every file ends with one
  newline.
- Comments are printed verbatim at their sequence's indent and never reflowed.
- Declaration bodies never have a trailing comma, for now: this keeps the reformat small.
- **The style is configurable.** Each choice the dialogue weighed is an option of the printer,
  and the defaults are the style above:
  - width;
  - closers hugged or on their own line;
  - trailing commas (never, in declaration bodies only, or wherever a closer has its own line);
  - inner spaces in flat records;
  - first-argument list/record hugging.

  The xtask entry point takes them as flags, and the `jj fix` tool entry passes them on its
  command line. Switching styles later means editing that entry and running `jj fix` over
  `glob:**/*.syl`. Tests cover each option in each of its settings, and each combination must be
  idempotent on the corpus.

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
