/-- "<action>ed [<within>]", the passive with no agent: "the exiled card", "each creature
destroyed this way", "creatures that were dealt damage this turn". -/
template was (action : Action) (within : Option LookbackSpan := none) :
    Predicate := {
  -- TODO(spelling): Include or omit the clauses supplied by within.
  body :=
    did (any) (action) (within := within)
}
