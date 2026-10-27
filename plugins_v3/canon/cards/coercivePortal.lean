/-- Coercive Portal -/
card coercivePortal : Spelled :=
  { name := "Coercive Portal", cost := [generic 4], types := [.artifact],
    text :=
      [ willOfTheCouncil
          (at_ (beginningOfYour .upkeep)
            (sequentially
              [ instruct eachPlayer (voteStartingWith you .openly
                  (byLabel ["carnage", "homage"])),
                doIf (voteLead "carnage" false)
                  (sequentially
                    [ sacrifice thisArtifact,
                      destroy (allOf (and [isPermanent, non isLand])) ]),
                doIf (voteLead "homage" true) (draw 1) ])) ] }
