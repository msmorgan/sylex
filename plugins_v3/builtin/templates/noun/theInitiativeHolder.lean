/-- "the player who has the initiative": only one player can have it at a time [CR#726.3]. -/
template theInitiativeHolder : NounPhrase := {
  spelling := "the player who has the initiative",
  body :=
    Semantics.Templates.NounPhrase.the
      (Semantics.Templates.Predicate.and
        [Semantics.Templates.Predicate.isPlayer, Semantics.Templates.Predicate.hasDesignation Semantics.Definitions.initiative Option.none])
}
