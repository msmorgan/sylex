/-- Restricts mode choice to modes not chosen within the given Duration. -/
primitive notChosenBefore (within : Duration) : ModeChange := {
  body :=
    Semantics.Primitives.ModeChange.notChosenBefore within
}
