/-- The four party roles, in rule order [CR#700.8]. -/
template partyRoles : List Predicate := {

  -- TODO(spelling): Cleric, Rogue, Warrior, and Wizard; this list supplies distinct party roles,
  -- not a single Oracle phrase.
  body :=
    [Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.cleric,
      Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.rogue,
      Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.warrior,
      Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.wizard]
}
