/-- "Choose one or more — [cost] — <mode>; …" [CR#702.172a] -/
-- [CR#702.172a]
template chooseSpree (modes : List (Option Cost × Action)) : Action := {
  -- TODO(spelling): Context-dependent wording: Choose one or more — [cost] — <mode>; …; string substitution cannot express its omitted or inflected parts.
  body :=
    actionChooseModes (orMore 1)
      (List.map
        (fun (x : Option Cost × Action) =>
          match x with
          | (c, e) => (c, none, e))
        modes)
}
