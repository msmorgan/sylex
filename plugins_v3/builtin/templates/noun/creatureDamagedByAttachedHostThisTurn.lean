template creatureDamagedByAttachedHostThisTurn (attachment : AttachWord) : NounPhrase := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := a (and [isCreature,
    predicateDid { subj := actor (attachHost attachment creature),
                   action := dealDamage anyAmount which,
                   within := some (within soFarThisTurn none) }])
}
