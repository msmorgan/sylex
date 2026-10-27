/-- "all <keyword> abilities", as "all protection abilities" -/
primitive abilities (keyword : KeywordTerm) : AbilityLost := {
  spelling := "all <keyword> abilities",
  body :=
    Semantics.Primitives.AbilityLost.term keyword
}
