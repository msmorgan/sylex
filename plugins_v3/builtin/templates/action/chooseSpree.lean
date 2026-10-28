/-- "Choose one or more — [cost] — <mode>; …" [CR#702.172a] -/
-- [CR#702.172a]
template chooseSpree (modes : List (Option Cost × Action)) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by modes. Format and coordinate the modes
  -- list entries, including the empty case.
  body :=
    actionChooseModes (orMore 1)
      (List.map
        (fun (x : Option Cost × Action) =>
          match x with
          | (c, e) => (c, none, e))
        modes)
}
