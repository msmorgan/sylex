template pip (color : Color) : ManaSymbol := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    .simple (.specific (.of color))
}
