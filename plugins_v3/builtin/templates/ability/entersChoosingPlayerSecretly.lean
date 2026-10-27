/-- "As <subject> enters, secretly choose <player>". -/
template entersChoosingPlayerSecretly (subject player : NounPhrase) : StaticSpec := {
  spelling := "As <subject> enters, secretly choose <player>",
  body :=
    Semantics.Templates.StaticSpec.entryChoice subject player Semantics.Disclosure.secretly
}
