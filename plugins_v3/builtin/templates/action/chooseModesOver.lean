/-- "Choose N. <frame>" over bullets that are not clauses: the header sentence written once as
`frame`, completed by each bullet in `items` into one mode, no mode costing anything. The modes
are whole, as `chooseModes` writes them; the header names no plural over them, as modes are chosen
before targets [CR#601.2b,601.2c] and a mode's targets are chosen only if it is
[CR#700.2a,700.2c]. -/
-- [CR#601.2b,601.2c]
-- [CR#700.2a,700.2c]
template chooseModesOver {α : Type} (quantity : Quantity) (items : List α)
    (frame : splice (α → Action)) : Action := {
  -- TODO(spelling): Format and coordinate the items list entries, including the empty case.
  body :=
    actionChooseModes quantity (List.map (fun (item : α) => (none, none, frame item)) items)
}
