template pip (color : Color) : ManaSymbol := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.specific (Semantics.ColorOrColorless.of color))
}
