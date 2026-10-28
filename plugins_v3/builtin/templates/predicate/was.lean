/-- "<action>ed [<within>]", the passive with no agent: "the exiled card", "each creature
destroyed this way", "creatures that were dealt damage this turn". -/
template was (action : Action) (within : Option LookbackSpan := none) :
    Predicate := {
  -- TODO(spelling): Context-dependent wording: <action>ed [<within>]; string substitution cannot express its omitted or inflected parts.
  body :=
    predicateDid { subj := any, action := action, within := within }
}
