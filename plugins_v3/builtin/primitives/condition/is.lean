/-- "<subject> is <predicate>" ("if it's a creature", "if this permanent isn't renowned" under
`not`): the condition that `subject` matches `predicate` (`matches` is Lean's word). -/
primitive is (subject : NounPhrase) (predicate : Predicate) : Condition := {
  spelling := "<subject> is <predicate>",
  body :=
    Semantics.Primitives.Condition.matches subject predicate
}
