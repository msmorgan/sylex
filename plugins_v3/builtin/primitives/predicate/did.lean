/-- "<subj> <action>ed [<within>]" as a description of the entity `which` stands for
("creatures that attacked this turn"); with no span written, the attributive participle a
Participial Description reads ("the creature you sacrificed"). An `ordinal` picks out the
`ordinal`th matching event, counted within each `per` when one is written ("the second spell you
cast each turn"). -/
primitive did (subj : Subject) (action : Action)
    (within : Option LookbackSpan := none) (ordinal : Option EventOrdinal := none) :
    Predicate := {
  -- TODO(spelling): Include or omit the clauses supplied by within, ordinal.
  body :=
    Semantics.Primitives.Predicate.did { subj := subj, action := action, within := within, ordinal := ordinal }
}
