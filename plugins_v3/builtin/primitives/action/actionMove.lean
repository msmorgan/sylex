primitive actionMove (subject : Semantics.NounPhrase) (from_ : Semantics.Location) (to : Option Semantics.Location) (at_ : Semantics.ObservationPoint) (riders : List Semantics.MoveRider) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.move subject from_ to at_ riders
}
