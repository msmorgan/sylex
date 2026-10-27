/-- "you get an emblem with <abilities>" -/
template getEmblem (abilities : List Ability) : Action := {
  spelling := "you get an emblem with <abilities>",
  body :=
    Semantics.Templates.Action.createObject 1 (Semantics.Templates.CreationSpec.creationSpecEmblem Option.none abilities) Option.none
}
