primitive chapterMark (chapters : List Nat := []) : GameEvent := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.GameEvent.chapterMark chapters
}
