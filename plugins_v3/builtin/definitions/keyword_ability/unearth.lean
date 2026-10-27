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
