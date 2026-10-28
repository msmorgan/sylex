/-- "<subj> <action>ed [<within>]" as a description of the entity `which` stands for
("creatures that attacked this turn"); with no span written, the attributive participle a
Participial Description reads ("the creature you sacrificed"). An `ordinal` picks out the
`ordinal`th matching event ("the second spell you cast each turn"). -/
template did (subj : Subject) (action : Action)
    (within : Option LookbackSpan := none) (ordinal : Option (Ordinal × Option Period) := none) :
    Predicate := {
  -- TODO(spelling): Context-dependent wording: <subj> <action>ed [<within>]; string substitution cannot express its omitted or inflected parts.
  body :=
    predicateDid { subj := subj, action := action, within := within, ordinal := ordinal }
}
