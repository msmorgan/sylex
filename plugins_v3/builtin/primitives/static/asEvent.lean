/-- An Action performed as the Event happens, without using the stack [CR#113.9]. -/
primitive asEvent (event : GameEvent) (does : Action) : StaticSpec := {
  body :=
    Semantics.Primitives.StaticSpec.asEvent event does
}
