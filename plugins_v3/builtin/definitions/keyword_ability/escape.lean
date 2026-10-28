/-- "Escape [cost]" means "You may cast this card from your graveyard by paying [cost] rather than
paying its mana cost." [CR#702.138a] -/
keyword escape (cost : Cost) : Ability := {
  lexical := {
    lemma := "escape",
    parameters := [[DashNonManaCost]],
    verbs := [{ lemma := "escape", frames := [Intransitive, Prepositional (["with"])] }]
  },
  spelling := "escape—<cost>.",
  body :=
    [static
        (can (actor you)
          (cast this_ (some yourGraveyard)
            (payingInstead cost)))]
}
