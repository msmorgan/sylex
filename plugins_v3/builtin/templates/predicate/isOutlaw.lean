/-- An outlaw: an Assassin, Mercenary, Pirate, Rogue, and/or Warlock [CR#700.12]. -/
template isOutlaw : Predicate := {
  spelling := "outlaw",
  body :=
    predicateCoord Semantics.Coordinator.andOr
      [hasSubtype assassin,
        hasSubtype mercenary,
        hasSubtype pirate,
        hasSubtype rogue,
        hasSubtype warlock]
}
