primitive fixed (times : Amount) (body : Action) : Repetition := {
  body := Semantics.Primitives.Repetition.fixed times body
}
