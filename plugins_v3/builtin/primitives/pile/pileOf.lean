/-- A pile described by its count, optionally naming the player who chooses it. -/
primitive pileOf (count : SliceCount) (by_ : Option NounPhrase := none) : NounPhrase := {
  body :=
    Semantics.Primitives.NounPhrase.pileOf count by_
}
