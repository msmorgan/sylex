/-- Count distinct values of the specified axis in the domain. -/
primitive distinctCount (axis : KindAxis) (domain : NounPhrase) : Amount := {
  spelling := "the number of different <axis> among <domain>",
  body :=
    Semantics.Primitives.Amount.distinctCount axis domain
}
