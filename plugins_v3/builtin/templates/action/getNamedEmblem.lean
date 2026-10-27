template getNamedEmblem (name : String) (abilities : List Ability) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Templates.Action.createObject 1 (Semantics.Templates.CreationSpec.creationSpecEmblem (Option.some name) abilities)
}
