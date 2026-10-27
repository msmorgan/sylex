/-- "them" reading players, as `they` reads one: "you choose one of them". -/
primitive themPlayers : NounPhrase := {
  spelling := "them",
  body :=
    Semantics.Primitives.NounPhrase.pro (Semantics.Reach.word Semantics.NounWord.player) Semantics.Plurality.many
}
