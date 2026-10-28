/-- "Unearth [cost]" means "[Cost]: Return this card from your graveyard to the battlefield. It
gains haste. Exile it at the beginning of the next end step. If it would leave the battlefield,
exile it instead of putting it anywhere else. Activate only as a sorcery." [CR#702.84a] -/
keyword unearth (cost : Cost) : Ability := {
  lexical := { lemma := "unearth", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): unearth [mana cost] / unearth—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [ activatedOnlyDuring cost
        (sequentially
          [ returnToBattlefield thisCard (from_ := yourGraveyard),
            gainHaste thisPermanent,
            atNext .endStep
              (exile thisPermanent),
            replaceEvent (would thisPermanent leave)
              (instead (exile thisPermanent))
              none ])
        asSorcery ]
}
