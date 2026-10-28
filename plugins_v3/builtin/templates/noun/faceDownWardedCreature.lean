/-- A 2/2 face-down creature with ward {2}, no name, no subtypes and no mana cost
[CR#701.58a,702.168a]. Ward is written bare by its label: cloak, a keyword action, reads it, and
keyword actions are declared below the keyword abilities. -/
template faceDownWardedCreature : FaceDown := {

  -- TODO(spelling): face down with ward {2}; the characteristic record is implicit in cloak and
  -- needs contextual face-down wording.
  body :=
    {
      characteristics :=
        some
          { types := [Semantics.CardType.creature],
            text :=
              [.keyword (.named "ward")
                  [keywordParamCost (mana [generic 2])] []],
            power := some 2, toughness := some 2 } }
}
