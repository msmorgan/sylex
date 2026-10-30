# Elaborated value dumps

From `lean/`:

```sh
lake build -q dumpValues
lake exe dumpValues dump before
lake exe dumpValues dump after
cmp ../docs/evidence/lean-dumps/{before,after}/values.txt
lake env lean DumpValues/Tests.lean
```

`dumpValues` and its support library are opt-in; neither the default build nor
`scripts/build` includes them. The executable imports all generated card **data**
modules, so Lake regenerates and rebuilds affected data before running it.
Enumeration uses the authoring environment extensions, never a directory scan.

Each `values.txt` line is a JSON pair `[key, value]`, sorted by the key string.
The key itself encodes a JSON array: `["card", printed_name]` or
`["builtin", bare_name, return_type, sample]`. A sample is either `"unsampled"`
or a JSON array of argument encodings; `"default"` means omission. This nested
encoding permits arbitrary names, strings and line breaks without ambiguous records.
The card name is the printed name used by its registered constant, not its filename.

Values are recursively reduced with full transparency and printed as fully qualified
constructors, type parameters and fields. Natural numbers and strings are literals.
This avoids derived printers on the large mutual inductives and cannot truncate fields
or hide a definition behind its name. Residual code, open terms, functions and unsupported
type parameters are errors. Registry companions (`_definition`) are included alongside
the declared value so changes to registry rules are visible too.

The basis follows the Rust expansion sampler: numeric values use parameter ordinal + 3,
strings use `Sample <ordinal>`, and Amount, NounPhrase and Cost use literal, `you` and
`{2}` bases. Constructors are ordered by field count, preserving declaration order on ties.
Constructor fields use declared defaults where available, otherwise base values; recursive fields therefore receive a finite base,
including through mutual recursion. Structures additionally vary each field individually.
Bool has both values; Option has none and some (every finite enum value, otherwise the
base); List has empty, each singleton, and the first two distinct elements together.
Type parameters use Nat; instance parameters are synthesized. Unsupported dependent
explicit parameters fail with a reason.

Defaulted parameters include omission as their first choice. Every individual parameter
variant is retained; remaining combinations fill the lexicographic product up to 256.
A basis exceeding 256 is an explicit sampling error. Arbitrary-precision counts report
exact omissions per declaration. This is a bounded comparison, not an equivalence proof.

Dispatch aliases are resolved and deduplicated; the summary counts them. The ten core
card-type dispatch registrations are not builtin declarations; their builtin counterparts
carry the registry rules and receive the records. Canon dispatch aliases are represented
by the card records. Unexpected key collisions are errors, even when their values agree.

`summary.txt` reports record kinds, declarations, samples, omissions and all errors.
A dump with errors exits 2 and retains its diagnostic output. Starting a new dump
invalidates the old summary, so interruption cannot leave a partial dump marked successful. Successful dumps exit 0.
Labels must be single components containing only letters, digits, `_` or `-`. All dump
output is under the gitignored `docs/evidence/lean-dumps/`; run from `lean/`.

Compare modes (also from `lean/`):

```sh
lake exe dumpValues compare before after
lake exe dumpValues compare before after --rename ../docs/evidence/renames.json
python3 scripts/test-dump-values.py
```

The rename file is a JSON array of `[old_key, new_key]` string pairs, using the exact
key strings from `values.txt`. Renaming is simultaneous (swaps work), and applies only
to the old dump. Unknown sources, duplicate sources or targets, collisions, malformed
records, and incomplete dumps are errors (exit 2). Added, removed and changed keys are
printed in key order, followed by counts. No differences exits 0; differences exit 1.
