/-- "if there are <np>", as "three or more lands with the same name", "if two or more creatures
are tied for least toughness": a counted indefinite, published for the consequent. -/
primitive thereAre (np : NounPhrase) : Condition := {
  spelling := "if there are <np>",
  body :=
    Semantics.Primitives.Condition.exists_ np
}
