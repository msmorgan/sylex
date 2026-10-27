/-- "<n>th from the bottom of <loc>" -/
primitive fromBottom (n : Nat) (loc : Location) : Location := {
  -- TODO(spelling): <n>th from the bottom of <loc>; string substitution cannot inflect verbs, nouns, or ordinals.
  body :=
    Semantics.Primitives.Location.fromEnd Semantics.PileEnd.bottom (Semantics.Ordinal.nth n) loc
}
