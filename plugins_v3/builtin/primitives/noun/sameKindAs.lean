/-- "a counter of each kind on <source>" -/
primitive sameKindAs (source : NounPhrase) : CounterKindSource := {
  spelling := "a counter of each kind on <source>",
  body :=
    Semantics.Primitives.CounterKindSource.sameAs source
}
