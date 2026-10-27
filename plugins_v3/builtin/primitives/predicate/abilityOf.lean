primitive abilityOf (source : NounPhrase) : Predicate := {
  spelling := "ability of <source>",
  body :=
    Semantics.Primitives.Predicate.abilityOf source
}
