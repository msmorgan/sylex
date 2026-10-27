/-- "As <subject> enters, choose <player>", as "choose an opponent". -/
template entersChoosingPlayer (subject player : NounPhrase) : StaticSpec := {
  spelling := "As <subject> enters, choose <player>",
  body :=
    Semantics.Templates.StaticSpec.entryChoice subject player
}
