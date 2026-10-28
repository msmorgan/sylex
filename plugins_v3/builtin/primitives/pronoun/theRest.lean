/-- "the rest [of them]" -/
primitive theRest (plur : Plurality := .many) : NounPhrase := {
  spelling := "the rest",
  body := Semantics.Primitives.NounPhrase.theRest plur
}
