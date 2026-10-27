/-- "Choose N — <modes>", no mode costing anything. -/
template chooseModes (quantity : Quantity) (modes : List Action) : Action := {
  -- TODO(spelling): Choose N — <modes>; wording varies with quantity, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Templates.Action.actionChooseModes quantity (List.map (fun (x : Semantics.Action) => (Option.none, Option.none, x)) modes)
}
