card := {
  name := "Coercive Portal",
  cost := [4],
  types := [artifact],
  text := [
    willOfTheCouncil
      (at_ (beginningOfYour upkeep)
         [
           instruct eachPlayer (voteStartingWith you openly (byLabel ["carnage", "homage"])),
           doIf (voteLead "carnage" false)
             [sacrifice thisArtifact, destroy (allOf [isPermanent, non isLand])],
           doIf (voteLead "homage" true) (draw 1)
         ])
  ]
}
