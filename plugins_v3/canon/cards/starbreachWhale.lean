/-- Starbreach Whale. Warp's "exile the permanent this spell becomes" is a Resolution Delayed
Trigger [CR#702.185a,608.3g]. -/
card starbreachWhale : Spelled :=
  { name := "Starbreach Whale", cost := [generic 4, pip .blue], types := [.creature],
    subtypes := [whale],
    text :=
      [ flying,
        when (does thisCreature enter) (surveil 2),
        warp [generic 1, pip .blue] ],
    power := stat 3, toughness := stat 5 }
