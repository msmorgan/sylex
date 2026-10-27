/-- Exchange the subject's rules text with `other`'s [CR#612.5,701.12h]. -/
-- [CR#612.5,701.12h]
primitive exchangeWith (other : NounPhrase) : TextChange := {
  -- TODO(spelling): Exchange the subject's rules text with `other`'s .
  body :=
    Semantics.Primitives.TextChange.exchangeWith other
}
