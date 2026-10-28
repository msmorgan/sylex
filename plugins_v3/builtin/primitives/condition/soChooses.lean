primitive soChooses (chooser : NounPhrase) : Condition := {
  spelling := "<chooser> so chooses",
  body := Semantics.Primitives.Condition.soChooses chooser
}
