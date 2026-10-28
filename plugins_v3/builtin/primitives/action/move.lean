primitive move (subject : NounPhrase) (from_ : Location) (to : Option Location)
    (at_ : ObservationPoint) (riders : List MoveRider) : Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.move subject from_ to at_ riders
}
