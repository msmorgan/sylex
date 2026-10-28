template getNamedEmblem (name : String) (abilities : List Ability) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    createObject 1 (creationSpecEmblem (some name) abilities)
}
