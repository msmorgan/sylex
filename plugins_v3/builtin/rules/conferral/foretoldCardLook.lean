/-- [CR#702.143d] "That card's owner may look at that card as long as it remains in exile": the
look right of every foretold card, whatever made it foretold. Its cast right, "for any foretell
cost it has", needs a cost term naming a keyword's cost, which the grammar does not have. -/
conferral foretoldCardLook := {

  body :=
    {
      scope :=
        and
          [isCard, hasDesignation foretold,
            in_ exileZone],
      confer :=
        Semantics.Conferral.ability
          (static
            (visibility Semantics.ExposeVerb.lookAt you
              (objects this_))) }
}
