/-- "them" reading players, as `they` reads one: "you choose one of them". -/
template themPlayers : NounPhrase := {
  spelling := "them",
  body := pro (.word player) .many
}
