# sylex_lexicon

Lexical analysis and realization for the English layer. It reads text as every
declared Lexeme occurrence it could be, and spells a lexical value back out.
Morphology, spelling variants, Onsets and the numeral codecs live here. It
depends on no grammar, vocabulary file or Game Model.

The design of this layer is [Lexicon](../../docs/decisions/lexicon.md); what it
owes the grammar is the parser contract in
[English](../../docs/decisions/english.md) §7. Terms are the
[glossary's](../../docs/contexts/oracle-english/CONTEXT.md).

## Data flow

1. Lexemes are declared elsewhere. `sylex_vocabulary`'s `LexicalEnvironment`
   builds them from its RON files, the Catalogs and the card names; tests build
   them with `Lexeme::invariant`, `Lexeme::noun` and `Lexeme::verb`.
2. `Lexicon::new` validates the declarations, spells every form, and freezes
   the analysis and realization indexes.
3. `analyze_source_scoped`, given the face's names as scope, returns an
   `AnalyzedText`: overlapping `LexicalMatch` edges over Unicode scalars, and
   the unknown words. `sylex_syntax`'s chart reads it from there.
4. `realize` spells a `LexicalReading`, whether analysis produced it or it was
   constructed independently.

```rust
let lexicon = sylex_lexicon::Lexicon::new(lexemes)?;
let analyzed = lexicon.analyze_source_scoped(text, None, &face_names);
for occurrence in &analyzed.matches {
    let spelling = lexicon.realize(&occurrence.reading)?;
}
```

Subcategorization Frames are not lexicon data: they live in
`sylex_syntax::FrameCatalog`.

## Inspecting

```sh
cargo xtask lexical --all \
  --output /tmp/lexical-inventory.json --export /tmp/lexemes.ron
cargo run -p sylex_lexicon --example inspect -- /tmp/lexemes.ron 'counters'
```

`cargo xtask lexical` analyzes each selected face's raw Oracle text, reminder
text included, in its card-name scope. Its report names every unknown word,
classifies every scalar no Lexeme covers, and checks both lexical laws below
across the corpus. Narrow it with the face selectors (`--card-name`,
`--text-contains` and the rest) in place of `--all`; `--export-subset` saves
the selected cards as JSONL. The `--export` RON is an inspection artifact, not
an input format: regenerate it after the model changes. The `inspect` example
loads it and prints the occurrences in a string.

## Working rules

- Declare words in `sylex_vocabulary`, never here.
- Fix a wrong default spelling with an override on its Lexeme, not with a new
  rule in the morphology ([Lexicon](../../docs/decisions/lexicon.md) §2).
- Keep both lexical laws: every returned occurrence realizes to exactly its
  source substring, and every indexed value realizes and analyzes back to the
  same reading. `tests/analysis.rs` holds them, and `cargo xtask lexical`
  checks them over the corpus.
- Lexical coverage is not grammatical coverage. A face whose every word
  analyzes is no evidence that the grammar admits it; the Census in
  `sylex_english` measures that.
