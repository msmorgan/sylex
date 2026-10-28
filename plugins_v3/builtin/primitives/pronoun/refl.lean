/-- Full-arity binding for `NounPhrase.refl`. -/
primitive refl (plur : Plurality) : NounPhrase := {
  body := Semantics.Primitives.NounPhrase.refl plur
}
