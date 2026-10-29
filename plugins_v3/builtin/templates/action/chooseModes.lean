/-- "Choose N — <modes>": each mode an action alone, or a costed or weighted `Mode` record. -/
template chooseModes (quantity : Quantity) (modes : List Mode) : Action := {
  -- TODO(spelling): Format and coordinate the modes list entries, including the empty case.
  body :=
    actionChooseModes quantity modes
}
