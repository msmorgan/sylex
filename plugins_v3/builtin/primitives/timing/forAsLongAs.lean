primitive forAsLongAs (condition : Condition) : Duration := {
  spelling := "for as long as <condition>",
  body :=
    Semantics.Primitives.Duration.forAsLongAs condition
}
