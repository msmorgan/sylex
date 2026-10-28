/-- "you may choose not to untap <subject> during [<whose>] untap step": the active player
determines which of their permanents untap [CR#502.3], so this is the prohibition `doesntUntap`
writes, holding only if you so choose. Untapping stays the default. -/
template mayDeclineUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec := {

  -- TODO(spelling): you may choose not to untap <subject> during [<whose>] untap step; string
  -- substitution cannot branch on optional fields or supply contextual agreement and morphology.
  body :=
    duringUntapStep whose
      (onlyIfSo
        (cant any (untap subject))
        (soChooses you))
}
