primitive rollModifier (roll : GameEvent) (modify : Action)
    (limit : Option UsageLimit) : StaticSpec := {
  body := Semantics.Primitives.StaticSpec.rollModifier roll modify limit
}
