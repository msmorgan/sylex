keyword morph (cost : Cost) : Ability := {
  lexical := {
    lemma := "morph",
    parameters := [[Cost], [DashNonManaCost]],
    variants := [{ lemma := "megamorph", parameters := [[Cost]] }]
  },
  -- TODO(spelling): morph [mana cost] / morph—[verbal cost]; separator and terminal period depend
  -- on the Cost structure.
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.can (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisCard Option.none
            (Semantics.Templates.PlayPayment.playPaymentFaceDown Semantics.Templates.FaceDown.faceDownCreature
              (Semantics.Templates.Cost.mana [Semantics.Templates.ManaSymbol.generic 3])))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.staticSpecSpecialAction Option.none Option.none cost
          (Semantics.Templates.Action.turnFaceUp Semantics.Templates.NounPhrase.this_))]
}
