/-- The chapter numbers in a Saga chapter ability. -/
primitive chapterMark (chapters : List Nat := []) : GameEvent := {
  body :=
    Semantics.Primitives.GameEvent.chapterMark chapters
}
