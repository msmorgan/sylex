/-- "Morph [cost]" means "You may cast this card as a 2/2 face-down creature with no text, no name,
no subtypes, and no mana cost by paying {3} rather than paying its mana cost." [CR#702.37a]
Turning it face up for its morph cost is a special action any time you have priority.
[CR#702.37e] -/
keyword morph (cost : Cost) : Ability := {
  lexical := {
    lemma := "morph",
    parameters := [[Cost]],
    variants := [{ lemma := "megamorph", parameters := [[Cost]] }]
  },
  spelling := "morph <cost>",
  body :=
    [static
        (can (actor you)
          (cast thisCard none
            (faceDown faceDownCreature
              (mana [generic 3])))),
      static
        (specialAction none cost (turnFaceUp this_) none)]
}
