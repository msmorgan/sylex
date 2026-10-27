primitive staticSpecRollModifier (roll : Semantics.GameEvent) (modify : Semantics.Action) (limit : Option Semantics.UsageLimit) : Semantics.StaticSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.StaticSpec.rollModifier roll modify limit
}
