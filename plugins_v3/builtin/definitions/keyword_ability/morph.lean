keyword morph (cost : Cost) : Ability := {
  lexical := {
    lemma := "morph",
    parameters := [[Cost], [DashNonManaCost]],
    variants := [{ lemma := "megamorph", parameters := [[Cost]] }]
  },
  -- TODO(spelling): morph [mana cost] / morph—[verbal cost]; separator and terminal period depend
  -- on the Cost structure.
  body :=
    [static
        (can (actor you)
          (cast thisCard none
            (faceDown faceDownCreature
              (mana [generic 3])))),
      static
        (specialAction none cost (turnFaceUp this_) none)]
}
