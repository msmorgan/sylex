primitive specialAction (when : Option Timing) (cost : Cost) (does : Action)
    (player : Option NounPhrase := none) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body := Semantics.Primitives.StaticSpec.specialAction player when cost does
}
