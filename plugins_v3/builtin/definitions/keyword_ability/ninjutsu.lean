/-- The Ninja enters attacking what the returned creature was attacking [CR#702.49c]. -/
keyword ninjutsu (cost : Cost) : Ability := {
  lexical := {
    lemma := "ninjutsu",
    parameters := [[Cost]],
    variants := [{ lemma := "commander ninjutsu", parameters := [[Cost]] }]
  },
  spelling := "ninjutsu <cost>",
  body :=
    have returnedCreaturesDefender :=
      Semantics.Templates.NounPhrase.the
        (Semantics.Templates.Predicate.and
          [Semantics.Templates.Predicate.or
              [Semantics.Templates.Predicate.isPlayer, Semantics.Templates.Predicate.isPlaneswalker, Semantics.Templates.Predicate.isBattle],
            Semantics.Templates.Predicate.inCombat Semantics.CombatRelation.attackedBy
              (Option.some
                (Semantics.Templates.NounPhrase.the
                  (Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.was (Semantics.Templates.Action.returnToHand Semantics.Templates.NounPhrase.which),
                      Semantics.Templates.Predicate.isCreature])))]);
    [Semantics.Templates.Ability.activated
        (Semantics.Templates.Cost.compound
          [cost, Semantics.Templates.Cost.perform (Semantics.Definitions.reveal Semantics.Templates.NounPhrase.this_),
            Semantics.Templates.Cost.perform
              (Semantics.Templates.Action.returnToHand
                (Semantics.Templates.NounPhrase.a
                  (Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.isAttacking, Semantics.Templates.Predicate.isUnblocked,
                      Semantics.Templates.Predicate.youControl])))])
        (Semantics.Templates.Action.putOntoBattlefieldTappedAttacking Semantics.Templates.NounPhrase.this_ Semantics.Templates.Location.yourHand
          (Option.some returnedCreaturesDefender))]
}
