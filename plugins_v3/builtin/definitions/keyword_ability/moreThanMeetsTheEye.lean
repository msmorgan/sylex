/-- "More Than Meets the Eye [cost]": "You may cast this card converted by paying [cost] rather
than its mana cost." [CR#702.162a] A converted spell has its back face up [CR#712.8c]. -/
keyword moreThanMeetsTheEye (cost : Cost) : Ability := {
  lexical := { lemma := "More Than Meets the Eye", parameters := [[Cost]] },
  spelling := "More Than Meets the Eye <cost>",
  body :=
    [static (can you (cast thisCard (payment := convertedPaying cost)))]
}
