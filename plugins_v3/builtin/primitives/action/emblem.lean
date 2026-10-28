primitive emblem (name : Option String) (abilities : List Ability) : CreationSpec := {
  body := Semantics.Primitives.CreationSpec.emblem name abilities
}
