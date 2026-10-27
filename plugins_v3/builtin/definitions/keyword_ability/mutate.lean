/-- "Mutate [cost]": "You may pay [cost] rather than pay this spell's mana cost. If you do, it
becomes a mutating creature spell and targets a non-Human creature with the same owner as this
spell" [CR#702.140a]. One "if you do" gates the added target [CR#601.2c] and the merge alike:
the mutate cost's choice was taken. -/
keyword mutate (cost : Cost) : Ability := {
  lexical := {
    lemma := "mutate",
    parameters := [[Cost]],
    verbs := [{ lemma := "mutate", frames := [Intransitive] }]
  },
  spelling := "mutate <cost>",
  body :=
    [Semantics.Templates.Ability.static (Semantics.Templates.StaticSpec.altCost Semantics.Templates.NounPhrase.this_ (Option.some cost)),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.addsTarget Semantics.Templates.NounPhrase.thisSpell
            (Semantics.Templates.Predicate.and
              [Semantics.Templates.Predicate.isCreature,
                Semantics.Templates.Predicate.not (Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.human),
                Semantics.Templates.Predicate.ownedBy (Semantics.Templates.NounPhrase.ownerOf Semantics.Templates.NounPhrase.thisSpell)]))
          (Semantics.Templates.Condition.choiceWasTaken (Semantics.LinkSource.byKeyword Semantics.Definitions.mutate.label)
            Semantics.Templates.NounPhrase.thisSpell)),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisSpell) Semantics.Templates.Action.enter)
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.choiceWasTaken (Semantics.LinkSource.byKeyword Semantics.Definitions.mutate.label)
              Semantics.Templates.NounPhrase.thisSpell)
            (Semantics.Templates.Replacement.instead
              (Semantics.Templates.Action.chooseOne
                (Semantics.Templates.Action.merge Semantics.Templates.NounPhrase.thisSpell
                  (Semantics.Templates.NounPhrase.the
                    (Semantics.Templates.Predicate.was (Semantics.Templates.Action.target Semantics.Templates.NounPhrase.which)))
                  Semantics.MergePosition.top)
                (Semantics.Templates.Action.merge Semantics.Templates.NounPhrase.thisSpell
                  (Semantics.Templates.NounPhrase.the
                    (Semantics.Templates.Predicate.was (Semantics.Templates.Action.target Semantics.Templates.NounPhrase.which)))
                  Semantics.MergePosition.bottom))))
          Option.none Semantics.ReplUse.repeatedly Option.none)]
}
