/-- Move the subject from the stated origin to the destination, with the specified arrival riders.
A move alone is not destruction [CR#701.8b]. -/
primitive move (subject : NounPhrase) (from_ : Location) (to : Option Location)
    (at_ : ObservationPoint) (riders : List MoveRider) : Action := {
  body := Semantics.Primitives.Action.move subject from_ to at_ riders
}
