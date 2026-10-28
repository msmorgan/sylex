/-- "the monarch": only one player can be the monarch at a time [CR#725.3]. -/
template theMonarch : NounPhrase := {
  spelling := "the monarch",
  body :=
    the
      (and
        [isPlayer, hasDesignation monarch none])
}
