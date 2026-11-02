---
needs: []
---
# `deckmaste_dsl`: parse `.syl` files to a syntax tree

Standard constraints apply. A new crate, `crates/deckmaste_dsl`, that parses `.syl` files
(`docs/decisions/semantics-v3.md` §4.4) to a purely syntactic tree. It is the parser that the
future `semantics` crate (`semantics-rust-rewrite`) and the `.syl` formatter (`dsl-formatter`)
both consume. The crate is renamed with the rest of the project when it becomes sylex.

## Decisions already made

- **Syntax only.** No resolution, no expansion, no sort checking, and nothing that depends on
  the declaration kind. Everything the Lean parser (`lean/SylGen/Parser.lean`) checks beyond
  syntax belongs to `semantics`:
  - "only a primitive may name a constructor", "redundant constructor binding", and which kinds
    may write a result sort;
  - reserved names (`none`, `true`, `false`) used as declaration or parameter names;
  - duplicate record fields, duplicate parameters, and a positional argument after a named one;
  - Lean's derived header data: the result sort implied by the kind (`kindSort`) and a card's
    printed name (`printedName`).
- **Two declaration shapes, told apart by bounded lookahead, never by the kind word:**
  - *Named:* `kind name`, optionally followed by a parameter list `(p: Sort, p: Sort = default)`,
    optionally `: Sort`, optionally `= .constructor`, then optionally a record body
    `{ field = value, ... }`. The body is optional because 291 primitives have none, for example
    `primitive actor: NounPhrase`.
  - *Nameless:* `kind value`, where the whole body is a single value. Today only `card` uses
    this, as `card { ... }` or as a call like `card split({ ... })` (24 files use `split`,
    `transforming`, `flip`, `leveler`, `prototype`, `adventurer`, `modalDfc` or
    `sharedLineSplit`).

  A word after the kind starts a named header when it is followed by `(` and then `ident :`, or by
  `:`, `=`, `{`, the next declaration, or end of file. Otherwise the rest of the declaration is
  one value.
- **Values mirror §4.4's productions:** string, integer, list, record, and call. A bare word is a
  call with no arguments. `true`, `false` and `none` are bare words, as they are in Lean; their
  reserved meaning is resolution's business. Arguments keep the order and names the author wrote
  (`Vec<Arg { name: Option<Ident>, value }>`). Sorts in headers are written like calls
  (`Option(List(Cost))`) and parse to a small recursive sort reference.
- **Lexical rules:**
  - Words match `[A-Za-z_][A-Za-z0-9_]*`. The constructor after `=` is written `.word`, and that
    is the only place a dot appears. Lean's tokenizer also accepts dots inside words; no `.syl`
    code uses them, and this parser rejects them.
  - Integers are `-?[0-9]+`, stored as `i64`. A `-` anywhere else is an error.
  - Strings are double-quoted, escape only `\"` and `\\`, and are stored unescaped as `String`.
  - Every comma-separated list (list items, record fields, call arguments, parameters, sort
    arguments) allows a trailing comma. `[]` and `{}` are valid. `head()` and an empty parameter
    list `name()` are errors.
- **`dpsi::Ident` for every name:** declaration kind, declaration and parameter names, field
  names, argument names, call heads, sort heads, and the constructor. The kind is an `Ident`,
  not a closed enum. Whether a card name or other string becomes an `Ident` is for `semantics` to
  decide.
- **Comments are kept; whitespace is not.** A comment's text is everything after the `#`,
  verbatim. There are no attachment rules: **a comment is an item in the sequence it appears
  in.** The sequences are the file's declarations, list items, record fields, call arguments and
  parameters. The formatter prints each comment on its own line at that sequence's indent.
  - Comments between items, before the first item, or after the last item (before a closing
    `]`, `}`, `)` or the end of the file) are all just items. Today 113 files have comments
    inside expressions, and seven primitives end with a comment, for example
    `primitives/pronoun/actor.syl`.
  - A comment after an item on the same line (`x, # note`) is accepted, and it becomes the next
    item, so the formatter moves it to its own line. The corpus has none today.
  - A comment that is not between items (between `name =` and its value, inside a header, or
    inside a sort) is moved to just before the item that contains it. Comments never cause a
    parse error.
  - Each item, comment or not, records whether a blank line precedes it.

  A declaration's doc comment is the run of comment items directly before it in the file, with
  no blank line between them and the header. This is how the Lean parser defines it. It is
  derived from the sequence when needed, not stored separately.
- **Spans on every node:** a file id plus a byte range. Line and column are computed when needed.
- **The lexer is separate from the tree builder**, so a later move to a lossless rowan-style CST
  (for an LSP) replaces only the builder. A CST is not part of this ticket.
- A file is a sequence of declarations and comments. Parse errors carry a span, and
  stopping at the first error is fine.

## Change

- Create the crate: lexer, tree, parser, error type.
- In its own commit, fix the seven primitives that end with a comment. They are conversion
  leftovers: a `TODO(spelling):` comment split around a body-less primitive's header. Rejoin each
  one into the doc block above its header. The parser still accepts a comment at end of file.

## Acceptance

- Every `plugins_v3/**/*.syl` file parses.
- Focused tests for:
  - both header shapes, with and without parameters, defaults, result sort, `= .constructor` and
    body;
  - `card { ... }` and `card split({ ... })`;
  - sorts, negative integers, string escapes, trailing commas, `[]` and `{}`, and the
    `head()` / `name()` / stray `-` / dotted-word errors;
  - comment items: in a list, a record, an argument list, a parameter list and at file level,
    including before each closing delimiter and at end of file; a same-line comment becoming the
    next item; a comment between `name =` and its value moved before its item; the blank-line
    flag; and a doc comment versus a detached comment.
- Malformed input gives an error located by its span.
- `cargo test -p deckmaste_dsl` and `cargo clippy -p deckmaste_dsl --tests -- -D warnings`.

Checking against the Lean parser is `dsl-parse-differential`, and formatting is
`dsl-formatter`. Neither belongs to this ticket.
