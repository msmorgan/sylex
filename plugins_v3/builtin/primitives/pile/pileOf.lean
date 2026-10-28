/-- A pile described by its count, optionally naming the player who chooses it. -/
primitive pileOf (count : SliceCount) (by_ : Option NounPhrase := none) : NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.NounPhrase.pileOf count by_
}
