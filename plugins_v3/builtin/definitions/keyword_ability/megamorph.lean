/-- "Megamorph [cost]", a variant of morph: morph's abilities and "As this permanent is turned
face up, put a +1/+1 counter on it if its megamorph cost was paid to turn it face up"
[CR#702.37b]. A megamorph cost is a morph cost, so it carries morph's label; the counter is a
replacement effect on the turning face up [CR#614.1e], on as the permanent turns. -/
keyword megamorph (cost : Cost) : Ability := {
  lexical := { lemma := "megamorph", parameters := [[Cost]] },
  label := "morph",
  spelling := "megamorph <cost>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.can (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisCard Option.none
            (Semantics.Templates.PlayPayment.playPaymentFaceDown Semantics.Templates.FaceDown.faceDownCreature
              (Semantics.Templates.Cost.mana [Semantics.Templates.ManaSymbol.generic 3])))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.staticSpecSpecialAction Option.none Option.none cost
          (Semantics.Templates.Action.turnFaceUp Semantics.Templates.NounPhrase.this_)),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would Semantics.Templates.Subject.any
            (Semantics.Templates.Action.turnFaceUp Semantics.Templates.NounPhrase.thisPermanent))
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.morph.label Semantics.Templates.NounPhrase.thisPermanent)
            (Semantics.Templates.Replacement.alongside
              (Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
                Semantics.Templates.NounPhrase.thisPermanent))))]
}
