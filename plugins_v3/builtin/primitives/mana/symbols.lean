/-- "<cost>" written out as mana symbols -/
primitive symbols (cost : ManaCost) : ManaCostTerm := {
  spelling := "<cost>",
  body :=
    Semantics.Primitives.ManaCostTerm.lit cost
}
