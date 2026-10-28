/-- "Soulshift N" means "When this permanent is put into a graveyard from the battlefield, you may
return target Spirit card with mana value N or less from your graveyard to your hand."
[CR#702.46a] -/
keyword soulshift (amount : Amount) : Ability := {
  lexical := { lemma := "soulshift", parameters := [[Amount]] },
  spelling := "soulshift <amount>",
  body :=
    [when
        (does (actor thisPermanent) die)
        (may
          (returnToHand
            (target
              (and
                [isCard, hasSubtype spirit,
                  hasManaValue atMost amount,
                  in_ yourGraveyard]))
            yourGraveyard))]
}
