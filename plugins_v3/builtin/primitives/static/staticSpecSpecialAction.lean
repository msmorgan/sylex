primitive staticSpecSpecialAction (player : Option Semantics.NounPhrase) (when : Option Semantics.Timing) (cost : Semantics.Cost) (does : Semantics.Action) : Semantics.StaticSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.StaticSpec.specialAction player when cost does
}
