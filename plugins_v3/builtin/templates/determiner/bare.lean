/-- the bare plural: "creatures", "players can't gain life". Its rules meaning and its plural
referent are those of "all creatures" (`allOf`), whose tree it shares. -/
template bare (p : Predicate) : NounPhrase := {
  spelling := "<p>",
  body :=
    described all p
}
