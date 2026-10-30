---
needs: [plugins-v3-data-language]
---
# Return `bracketed` text to `.syl` and make cleave work

Standard constraints apply. `plugins-v3-data-language` deletes the `bracketed` template,
`Authoring.Form` and the `bracketed` pins in `Proofs/TextChanges.lean`, so
cleave is not expected to work through templates until this lands. The first goal there was a
simple `.syl`; this ticket brings the feature back on top of it.

Decide, with the owner, whether the Lean bindings and checker are still required for this, then
redesign how square-bracketed rules text is represented across English and semantics, so that
`TextChange.removeBracketed` [CR#702.148a,702.148b] reads it from the parsed `.syl` tree and
existing cleave cards check again. Re-add the pins that were retired.
