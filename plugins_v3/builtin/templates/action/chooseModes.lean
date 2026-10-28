/-- "Choose N — <modes>", no mode costing anything. -/
template chooseModes (quantity : Quantity) (modes : List Action) : Action := {
  -- TODO(spelling): Format and coordinate the modes list entries, including the empty case.
  body :=
    actionChooseModes quantity (List.map (fun (x : Action) => (none, none, x)) modes)
}
