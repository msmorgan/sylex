/-- The four party roles, in rule order [CR#700.8]. -/
template partyRoles : List Predicate := {

  -- TODO(spelling): Cleric, Rogue, Warrior, and Wizard; this list supplies distinct party roles,
  -- not a single Oracle phrase.
  body :=
    [hasSubtype cleric,
      hasSubtype rogue,
      hasSubtype warrior,
      hasSubtype wizard]
}
