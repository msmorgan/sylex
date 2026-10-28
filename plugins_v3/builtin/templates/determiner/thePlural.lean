/-- "the …s", the definite plural: "the exiled cards", "the discarded cards" [CR#607.2a]. -/
-- [CR#607.2a]
template thePlural (p : Predicate) : NounPhrase := {
  spelling := "the <p>",
  body :=
    described (the .many) p
}
