keyword disguise (cost : Cost) : Ability := {
  lexical := { lemma := "disguise", parameters := [[Cost]] },
  spelling := "disguise <cost>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.can (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisCard Option.none
            (Semantics.Templates.PlayPayment.playPaymentFaceDown Semantics.Templates.FaceDown.faceDownWardedCreature
              (Semantics.Templates.Cost.mana [Semantics.Templates.ManaSymbol.generic 3])))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.staticSpecSpecialAction Option.none Option.none cost
          (Semantics.Templates.Action.turnFaceUp Semantics.Templates.NounPhrase.this_))]
}
