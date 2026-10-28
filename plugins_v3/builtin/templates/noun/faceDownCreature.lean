/-- A 2/2 face-down creature with no text, no name, no subtypes and no mana cost [CR#708.2a]. -/
template faceDownCreature : FaceDown := {
  body :=
    ⟨some { types := [.creature], power := some 2, toughness := some 2 }⟩
}
