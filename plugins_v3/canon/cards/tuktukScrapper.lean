/-- Tuktuk Scrapper: "Whenever this creature or another Ally you control enters, you may destroy
target artifact. If that artifact is put into a graveyard this way, this creature deals damage to
that artifact's controller equal to the number of Allies you control." The target is chosen as the
ability is put on the stack, whether or not its controller destroys it [CR#601.2c,603.3d,115.1]:
"that artifact" reads it after the optional body, in the lookback and as the damage's possessor
(owner ruling 2026-10-23). -/
card tuktukScrapper : Spelled :=
  { name := "Tuktuk Scrapper", cost := [generic 3, pip .red], types := [.creature],
    subtypes := [goblin, artificer, ally],
    text :=
      [ whenever
          (does (eitherOf thisCreature (a (and [ally, youControl, isOther]))) enter)
          (sequentially
            [ may (destroy (target isArtifact)),
              doIf (did (that artifact) (putInto theActor graveyard) thisWay)
                (instruct thisCreature
                  (dealDamage (countOf (and [ally, youControl]))
                    (controllerOf (that artifact)))) ]) ],
    power := stat 2, toughness := stat 2 }
