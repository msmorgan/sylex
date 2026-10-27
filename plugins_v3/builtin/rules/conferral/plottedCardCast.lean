/-- [CR#702.170d] "A plotted card's owner may cast it from exile without paying its mana cost
during their main phase while the stack is empty during any turn after the turn in which it became
plotted. … A plotted card may be cast this way even if it doesn't have the plot ability while in
exile." A rule for every plotted card, not part of plot's text [CR#702.170a]; "you" on a card in
exile is its owner [CR#109.5]. -/
conferral plottedCardCast := {

  body :=
    {
      scope :=
        Semantics.Templates.Predicate.and
          [Semantics.Templates.Predicate.isCard, Semantics.Templates.Predicate.hasDesignation Semantics.Definitions.plotted,
            Semantics.Templates.Predicate.in_ Semantics.Templates.Location.exileZone],
      confer :=
        Semantics.Conferral.ability
          (Semantics.Templates.Ability.static
            (Semantics.Templates.StaticSpec.onlyUnless
              (Semantics.Templates.StaticSpec.can (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
                (Semantics.Definitions.cast Semantics.Templates.NounPhrase.this_ (Option.some Semantics.Templates.Location.exileZone)
                  Semantics.Templates.PlayPayment.withoutPaying)
                (Option.some Semantics.Templates.Timing.asSorcery))
              (Semantics.Templates.Condition.did Semantics.Templates.Subject.any
                (Semantics.Templates.Action.gainDesignation Semantics.Templates.NounPhrase.this_ Semantics.Definitions.plotted)
                (Semantics.Templates.LookbackSpan.within Semantics.Templates.Duration.soFarThisTurn)))) }
}
