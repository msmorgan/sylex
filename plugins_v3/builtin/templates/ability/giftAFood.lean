/-- "Gift a Food" means the effect is "The chosen player creates a Food
token." [CR#702.174d] A Food token is a colorless Food artifact token with
"{2}, {T}, Sacrifice this token: You gain 3 life." [CR#111.10b] -/
template giftAFood : Ability := {
  spelling := "gift a Food",
  body :=
    gift (create 1 foodToken)
}
