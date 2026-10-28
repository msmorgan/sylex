/-- "the …s", the definite plural: "the exiled cards", "the discarded cards" [CR#607.2a]. -/
-- [CR#607.2a]
template thePlural (p : Predicate) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: the …s; string substitution cannot express its omitted or inflected parts.
  body :=
    described (detPhraseThe .many) p
}
