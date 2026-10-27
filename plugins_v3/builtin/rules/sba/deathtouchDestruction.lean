/-- [CR#702.2b,704.5h] "If a creature has toughness greater than 0, and it's been dealt damage by
a source with deathtouch since the last time state-based actions were checked, that creature is
destroyed." The source is read as it dealt the damage, from any zone [CR#702.2d]; "is destroyed"
is the destroy action, which regeneration can replace. -/
sba deathtouchDestruction := {

  body :=
    { scope := Semantics.Templates.Predicate.isCreature,
      when :=
        Semantics.Templates.Condition.and
          [Semantics.Templates.Condition.compareAmt (Semantics.Templates.Amount.toughnessOf Semantics.Templates.NounPhrase.this_)
              Semantics.Comparator.greater 0,
            Semantics.Templates.Condition.did
              (Semantics.Templates.Subject.by_
                (Semantics.Templates.NounPhrase.a
                  (Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.isSource,
                      Semantics.Templates.Predicate.hasKeyword
                        (Semantics.Templates.KeywordTerm.theKeyword Semantics.Definitions.deathtouch.label)])))
              (Semantics.Templates.Action.dealDamage Semantics.Templates.Amount.anyAmount Semantics.Templates.NounPhrase.this_)
              (Semantics.Templates.LookbackSpan.within
                (Semantics.Templates.Duration.durationBetween (Option.some Semantics.Templates.GameEvent.gameEventStateBasedActionsChecked)
                  (Option.some Semantics.Templates.GameEvent.gameEventNow)))],
      then_ := Semantics.Definitions.destroy Semantics.Templates.NounPhrase.this_ }
}
