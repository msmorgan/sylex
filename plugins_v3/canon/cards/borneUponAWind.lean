card borneUponAWind : Spelled :=
  { name := "Borne Upon a Wind", cost := [generic 1, pip .blue], types := [.instant],
    text :=
      [ spell
          (establishThroughout
             (can you (cast (bare isSpell)) (asThough := asThoughHadFlash them)) thisTurn),
        spell (draw 1) ] }
