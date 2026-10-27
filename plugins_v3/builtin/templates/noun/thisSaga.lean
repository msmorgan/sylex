template thisSaga : NounPhrase := {
  spelling := "this Saga",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.subtype Semantics.Definitions.saga) Semantics.Templates.NounPhrase.this_
}
