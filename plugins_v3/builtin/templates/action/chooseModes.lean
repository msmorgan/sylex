/-- "Choose N — <modes>", no mode costing anything. -/
template chooseModes (quantity : Quantity) (modes : List Action) : Action := {
  -- TODO(spelling): Choose N — <modes>; wording varies with quantity, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    actionChooseModes quantity (List.map (fun (x : Semantics.Action) => (none, none, x)) modes)
}
