/-- "As <subject> enters, secretly choose <player>". -/
template entersChoosingPlayerSecretly (subject player : NounPhrase) : StaticSpec := {
  spelling := "As <subject> enters, secretly choose <player>",
  body :=
    entryChoice subject player Semantics.Disclosure.secretly
}
