/-- "this": the source, by self-name [CR#113.7]. -/
-- [CR#113.7]
primitive this_ : NounPhrase := {
  spelling := "this",
  body :=
    Semantics.Primitives.NounPhrase.this
}
