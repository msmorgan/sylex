/-- "Jump-start": "You may cast this card from your graveyard if the resulting spell is an
instant or sorcery spell by discarding a card as an additional cost to cast it" and "If this
spell was cast using its jump-start ability, exile this card instead of putting it anywhere else
any time it would leave the stack" [CR#702.133a]. The discard is added to the mana cost or an
alternative cost [CR#118.8,601.2f]; casting this way is taking the choice jump-start offers. -/
keyword jumpStart : Ability := {
  lexical := { lemma := "jump-start" },
  spelling := "jump-start",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.can (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.this_ (Option.some Semantics.Templates.Location.yourGraveyard)
            (Semantics.Templates.PlayPayment.plusAdditional
              (Semantics.Templates.Cost.perform (Semantics.Definitions.discard (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isCard))))
            (Option.some Semantics.Templates.Predicate.isInstantOrSorcery))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisSpell)
            (Semantics.Templates.Action.leave Semantics.Templates.Location.stack))
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.choiceWasTaken (Semantics.LinkSource.byKeyword Semantics.Definitions.jumpStart.label)
              Semantics.Templates.NounPhrase.thisSpell)
            (Semantics.Templates.Replacement.instead (Semantics.Definitions.exile Semantics.Templates.NounPhrase.thisSpell))))]
}
