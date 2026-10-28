/-- "the …" -/
primitive the (plur : optParam Plurality Plurality.one) : DetPhrase := {
  body := Semantics.Primitives.DetPhrase.the plur
}
