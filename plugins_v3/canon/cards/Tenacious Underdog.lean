/-- Tenacious Underdog. Cast from the graveyard "using its blitz ability", the spell takes blitz's
alternative cost, so blitz's cost was paid and its Resolution Delayed Trigger sacrifices the
permanent [CR#702.152a,601.2b,608.3g]. -/
card := {
  name := "Tenacious Underdog",
  cost := [1, black],
  types := [creature],
  subtypes := [human, warrior],
  text := [
    blitz (compound [[2, black, black], payLife 2]),
    can you (cast this_ (from_ := yourGraveyard) (payment := usingKeyword blitz))
  ],
  power := 3,
  toughness := 2
}
