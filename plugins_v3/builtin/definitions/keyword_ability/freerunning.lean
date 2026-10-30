/-- "Freerunning [cost]": "You may pay [cost] rather than pay this spell's mana cost if a player
was dealt combat damage this turn by a creature that, at the time it dealt that damage, was an
Assassin creature or a commander under your control." [CR#702.173a] The lookback's description
reads the creature as it dealt the damage, as prowl's does. -/
keyword freerunning (cost : Cost) : Ability := {
  lexical := { lemma := "freerunning", parameters := [[Cost]] },
  spelling := "freerunning <cost>",
  body :=
    [static
        (onlyWhile (altCost this_ (some cost))
          (did
            (actor
              (a
                (and
                  [isCreature,
                    or
                      [hasSubtype assassin,
                        hasDesignation commander],
                    youControl])))
            (dealDamage anyAmount (a isPlayer)
              (some combat))
            (within soFarThisTurn)))]
}
