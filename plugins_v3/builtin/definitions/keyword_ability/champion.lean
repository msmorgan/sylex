keyword champion (subject : Predicate) : Ability := {
  lexical := {
    lemma := "champion",
    parameters := [[Quality (None)]],
    verbs := [{ lemma := "champion", frames := [Monotransitive] }]
  },
  -- TODO(spelling): Champion a creature / champion an Elf; the quality needs context-dependent
  -- article or number inflection, not literal substitution.
  body :=
    [Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent) Semantics.Templates.Action.enter)
        (Semantics.Templates.Action.mayOrElse
          (Semantics.Templates.Action.pay
            (Semantics.Templates.Cost.perform
              (Semantics.Definitions.exile
                (Semantics.Templates.NounPhrase.a
                  (Semantics.Templates.Predicate.and
                    [subject, Semantics.Templates.Predicate.youControl, Semantics.Templates.Predicate.isOther]))))
            Semantics.PayTimes.once)
          (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.thisPermanent)),
      Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent) Semantics.Templates.Action.leave)
        (Semantics.Templates.Action.returnToBattlefield
          (Semantics.Templates.NounPhrase.the
            (Semantics.Templates.Predicate.and
              [Semantics.Templates.Predicate.was (Semantics.Definitions.exile Semantics.Templates.NounPhrase.which),
                Semantics.Templates.Predicate.isCard]))
          [Semantics.Templates.MoveRider.under (Semantics.Templates.NounPhrase.ownerOf Semantics.Templates.NounPhrase.it)])]
}
