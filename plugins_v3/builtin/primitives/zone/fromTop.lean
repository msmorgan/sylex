/-- "<n>th from the top of <loc>", as "seventh from the top" [CR#401.7]. -/
-- [CR#401.7]
primitive fromTop (n : Nat) (loc : Location) : Location := {
  -- TODO(spelling): <n>th from the top of <loc>; string substitution cannot inflect verbs, nouns, or ordinals.
  body :=
    Semantics.Primitives.Location.fromEnd Semantics.PileEnd.top (Semantics.Ordinal.nth n) loc
}
