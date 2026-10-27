/-- "Jump-start": "You may cast this card from your graveyard if the resulting spell is an
instant or sorcery spell by discarding a card as an additional cost to cast it" and "If this
spell was cast using its jump-start ability, exile this card instead of putting it anywhere else
any time it would leave the stack" [CR#702.133a]. The discard is added to the mana cost or an
alternative cost [CR#118.8,601.2f]; casting this way is taking the choice jump-start offers. -/
keyword jumpStart : Ability := {
  lexical := { lemma := "jump-start" },
  spelling := "jump-start",
  body :=
    [ static
        (can you
          (cast this_ (from_ := yourGraveyard)
            (payment := plusAdditional (.perform (discard (a .isCard))))
            (resulting := some isInstantOrSorcery))),
      static
        (replace (would thisSpell (leave stack))
          (doIf (choiceWasTaken (.byKeyword jumpStart) thisSpell)
            (instead (exile thisSpell)))) ]
}
