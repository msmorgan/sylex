/-- "Splice onto [quality] [cost]": "You may reveal this card from your hand as you cast a
[quality] spell. If you do, that spell gains the text of this card's rules text and you pay [cost]
as an additional cost to cast that spell" [CR#702.47a]. The reveal is a step of casting that
spell [CR#601.2b], which it does not replace; the cards spliced onto one spell are revealed at
once and their text follows the main spell's in the order chosen [CR#702.47b,702.47c]. Named in
its sort, `Ability.splice`, as a declaration whose bare name is taken is (`Subtype.army`,
`DesignationLabel.solved`): `splice` is the authoring syntax's word for a supplied body. -/
keyword Ability.splice (quality : Predicate) (cost : Cost) : Ability := {
  lexical := {
    lemma := "splice",
    parameters := [[Quality (Some ("onto")), Cost]],
    verbs := [{ lemma := "splice", frames := [TransitiveWithPP (["onto"])] }]
  },
  spelling := "splice onto <quality> <cost>",
  body :=
    [static
        (revealedAsCast
          (a (and [isSpell, quality]))
          (conjunction [addedCost cost false once,
              textChange (that spell)
                (appendTextOf this_)] none))]
}
