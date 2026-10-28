primitive rollModifier (roll : GameEvent) (modify : Action)
    (limit : Option UsageLimit) : StaticSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.StaticSpec.rollModifier roll modify limit
}
