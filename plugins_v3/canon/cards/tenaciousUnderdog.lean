/-- Tenacious Underdog. Cast from the graveyard "using its blitz ability", the spell takes blitz's
alternative cost, so blitz's cost was paid and its Resolution Delayed Trigger sacrifices the
permanent [CR#702.152a,601.2b,608.3g]. -/
card tenaciousUnderdog : Spelled :=
  { name := "Tenacious Underdog", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [human, warrior],
    text :=
      [ blitz (compound [[generic 2, pip .black, pip .black], payLife 2]),
        static (can you
          (cast this_ (from_ := yourGraveyard) (payment := usingKeyword blitz))) ],
    power := stat 3, toughness := stat 2 }
