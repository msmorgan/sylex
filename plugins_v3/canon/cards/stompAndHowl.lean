/-- Stomp and Howl -/
card stompAndHowl : Spelled :=
  { name := "Stomp and Howl", cost := [generic 2, pip .green], types := [.sorcery],
    text := [ spell (destroy (both (target isArtifact) (target isEnchantment))) ] }
