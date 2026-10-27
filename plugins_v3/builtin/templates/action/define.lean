/-- "X is <amount>": a numeric definition at action position, by the shared static
definition. -/
template define (letter : Letter) (amount : Amount) : Action := {
  spelling := "<letter> is <amount>",
  body :=
    Semantics.Templates.Action.establish (Semantics.Templates.StaticSpec.letterDefinition letter amount) Option.none
}
