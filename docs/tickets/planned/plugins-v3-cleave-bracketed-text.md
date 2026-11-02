---
needs: [plugins-v3-data-language, semantics-rust-rewrite]
---
# Return `bracketed` text to `.syl` and make cleave work

Standard constraints apply. `plugins-v3-syl-conversion` deletes the `bracketed` template and
its pins in `lean/Semantics/Proofs/TextChanges.lean`, and `plugins-v3-data-language` deletes
`Authoring.Form`, so cleave is not expected to work through templates until this lands. The first goal there was a
simple `.syl`; this ticket brings the feature back on top of it.

Redesign how square-bracketed rules text is represented across English and semantics (Lean is
retired; this is `.syl`, Rust `semantics` and `discourse`), so that `TextChange.removeBracketed`
[CR#702.148a,702.148b] reads it from the parsed `.syl` tree and existing cleave cards check again
in `discourse`. Re-add the pins that were retired, as Rust tests.
