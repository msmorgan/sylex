/-- "Flashback [cost]": "You may cast this card from your graveyard if the resulting spell is an
instant or sorcery spell by paying [cost] rather than paying its mana cost" and "If the flashback
cost was paid, exile this card instead of putting it anywhere else any time it would leave the
stack" [CR#702.34a]. -/
keyword flashback (cost : Cost) : Ability := {
  lexical := { lemma := "flashback", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): flashback [mana cost] / flashback—[verbal cost]; separator and terminal
  -- period depend on the Cost structure.
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.can (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.this_ (Option.some Semantics.Templates.Location.yourGraveyard)
            (Semantics.Templates.PlayPayment.payingInstead cost) (Option.some Semantics.Templates.Predicate.isInstantOrSorcery))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisSpell)
            (Semantics.Templates.Action.leave Semantics.Templates.Location.stack))
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.flashback.label Semantics.Templates.NounPhrase.thisSpell)
            (Semantics.Templates.Replacement.instead (Semantics.Definitions.exile Semantics.Templates.NounPhrase.thisSpell))))]
}
