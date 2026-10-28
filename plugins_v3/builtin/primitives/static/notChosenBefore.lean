/-- Restricts mode choice to modes not chosen within the given Duration. -/
primitive notChosenBefore (within : Duration) : ModeChange := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.ModeChange.notChosenBefore within
}
