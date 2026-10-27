/-- A 2/2 face-down creature with no text, no name, no subtypes and no mana cost [CR#708.2a]. -/
template faceDownCreature : FaceDown := {
  -- TODO(spelling): face down; the characteristic record is implicit in manifest, so printing its
  -- fields as token text would describe a different object.
  body :=
    ⟨some { types := [.creature], power := some 2, toughness := some 2 }⟩
}
