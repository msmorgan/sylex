/-- "Megamorph [cost]", a variant of morph: morph's abilities and "As this permanent is turned
face up, put a +1/+1 counter on it if its megamorph cost was paid to turn it face up"
[CR#702.37b]. A megamorph cost is a morph cost, so it carries morph's label; the counter is a
replacement effect on the turning face up [CR#614.1e], on as the permanent turns. -/
keyword megamorph (cost : Cost) : Ability := {
  lexical := { lemma := "megamorph", parameters := [[Cost]] },
  label := "morph",
  spelling := "megamorph <cost>",
  body :=
    [ static (can you (cast thisCard (payment := .faceDown faceDownCreature (mana [generic 3])))),
      static (.specialAction none none cost (turnFaceUp this_)),
      static
        (replace (would any (turnFaceUp thisPermanent))
          (doIf (keywordCostPaid morph thisPermanent)
            (alongside (putCounters 1 p1p1Counter thisPermanent)))) ]
}
