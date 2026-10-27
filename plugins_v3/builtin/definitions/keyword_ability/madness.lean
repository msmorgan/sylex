keyword madness (cost : Cost) : Ability := {
  lexical := { lemma := "madness", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): madness [mana cost] / madness—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isPlayer))
            (Semantics.Definitions.discard Semantics.Templates.NounPhrase.thisCard))
          (Semantics.Templates.Replacement.instead
            -- This escape crosses the nested discard scope, rather than wrapping a parameter.
            (Semantics.Action.ownScope (Option.some Semantics.Definitions.Action.discard.deed)
              (Semantics.Definitions.exileFrom (Semantics.NounPhrase.inCaller Semantics.Templates.NounPhrase.thisCard)
                Semantics.Templates.Location.hand)))),
      Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.causes
          (Semantics.Templates.Causing.byEvent
            (Semantics.Templates.GameEvent.does Semantics.Templates.Subject.any
              (Semantics.Definitions.discard Semantics.Templates.NounPhrase.thisCard)))
          (Semantics.Templates.GameEvent.does Semantics.Templates.Subject.any
            (Semantics.Definitions.exileFrom Semantics.Templates.NounPhrase.thisCard Semantics.Templates.Location.hand)))
        (Semantics.Templates.Action.instruct (Semantics.Templates.NounPhrase.ownerOf Semantics.Templates.NounPhrase.thisCard)
          (Semantics.Templates.Action.mayOrElse
            (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisCard (Option.some Semantics.Templates.Location.exileZone)
              (Semantics.Templates.PlayPayment.payingInstead cost))
            (Semantics.Templates.Action.putInto Semantics.Templates.NounPhrase.thisCard Semantics.Templates.Location.graveyard)))]
}
