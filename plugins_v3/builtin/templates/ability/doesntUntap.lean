/-- "<subject> doesn't untap during [<whose>] untap step": the untap step's untapping is the
`untap` Deed [CR#502.3,701.26b], so this forbids the same Deed a "can't untap" pattern names. -/
template doesntUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec := {
  -- TODO(spelling): <subject> doesn't/don't untap during [<whose>] untap step; string substitution
  -- cannot branch on optional fields or supply contextual agreement and morphology.
  body :=
    duringUntapStep whose (cant any (untap subject))
}
