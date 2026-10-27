/-- "they": the player most recently named. -/
primitive they : NounPhrase := {
  spelling := "they",
  body :=
    Semantics.Primitives.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.one
}
