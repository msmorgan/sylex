/-- An outlaw: an Assassin, Mercenary, Pirate, Rogue, and/or Warlock [CR#700.12]. -/
template isOutlaw : Predicate := {
  spelling := "outlaw",
  body :=
    Semantics.Templates.Predicate.predicateCoord Semantics.Coordinator.andOr
      [Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.assassin,
        Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.mercenary,
        Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.pirate,
        Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.rogue,
        Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.warlock]
}
