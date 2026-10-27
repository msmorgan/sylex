/-- "you own" -/
template youOwn : Predicate := {
  spelling := "you own",
  body :=
    Semantics.Templates.Predicate.ownedBy Semantics.Templates.NounPhrase.you
}
