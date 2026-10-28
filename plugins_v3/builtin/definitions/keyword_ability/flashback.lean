/-- "Flashback [cost]": "You may cast this card from your graveyard if the resulting spell is an
instant or sorcery spell by paying [cost] rather than paying its mana cost" and "If the flashback
cost was paid, exile this card instead of putting it anywhere else any time it would leave the
stack" [CR#702.34a]. -/
keyword flashback (cost : Cost) : Ability := {
  lexical := { lemma := "flashback", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): flashback [mana cost] / flashback—[verbal cost]; separator and terminal
  -- period depend on the Cost structure.
  body :=
    [static
        (can (actor you)
          (cast this_ (some yourGraveyard)
            (payingInstead cost) (some isInstantOrSorcery))),
      static
        (replace
          (would (actor thisSpell)
            (leave stack))
          (doIf
            (keywordCostPaid flashback.label thisSpell)
            (instead (exile thisSpell))))]
}
