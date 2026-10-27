/-- "using its <keyword> ability": the alternative cost that keyword ability offers
[CR#601.2b,601.2f]. -/
-- [CR#601.2b,601.2f]
primitive usingKeyword (label : KeywordLabel) : PlayPayment := {
  -- TODO(spelling): Context-dependent wording: using its <keyword> ability; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.PlayPayment.byKeyword label
}
