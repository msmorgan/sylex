/-- "the monarch": only one player can be the monarch at a time [CR#725.3]. -/
template theMonarch : NounPhrase := {
  spelling := "the monarch",
  body :=
    Semantics.Templates.NounPhrase.the
      (Semantics.Templates.Predicate.and
        [Semantics.Templates.Predicate.isPlayer, Semantics.Templates.Predicate.hasDesignation Semantics.Definitions.monarch Option.none])
}
