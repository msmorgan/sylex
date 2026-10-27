card branchloftPathway : Spelled := { body := .modalDfc
  { characteristics :=
    { name := "Branchloft Pathway", types := [.land],
      text := [activated tapSymbol (addMana 1 (manaOf [[.of .green]]))] }
          }
  { characteristics :=
    { name := "Boulderloft Pathway", types := [.land],
      text := [activated tapSymbol (addMana 1 (manaOf [[.of .white]]))] }
          } }
