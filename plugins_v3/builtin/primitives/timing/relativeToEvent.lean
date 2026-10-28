primitive relativeToEvent (side : EventSide) (event : GameEvent) : Timing := {
  body := Semantics.Primitives.Timing.relativeToEvent side event
}
