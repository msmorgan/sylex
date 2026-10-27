/-- [CR#702.143d] "That card's owner may look at that card as long as it remains in exile": the
look right of every foretold card, whatever made it foretold. Its cast right, "for any foretell
cost it has", needs a cost term naming a keyword's cost, which the grammar does not have. -/
conferral foretoldCardLook := {

  body :=
    {
      scope :=
        Semantics.Templates.Predicate.and
          [Semantics.Templates.Predicate.isCard, Semantics.Templates.Predicate.hasDesignation Semantics.Definitions.foretold,
            Semantics.Templates.Predicate.in_ Semantics.Templates.Location.exileZone],
      confer :=
        Semantics.Conferral.ability
          (Semantics.Templates.Ability.static
            (Semantics.Templates.StaticSpec.visibility Semantics.ExposeVerb.lookAt Semantics.Templates.NounPhrase.you
              (Semantics.Templates.VisibleThing.objects Semantics.Templates.NounPhrase.this_))) }
}
