template sawInHalfBody : Action := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := sequentially
    [ destroy (target isCreature),
      doIf (did (that creature) die thisWay)
        (instruct (controllerOf it)
          (create 2
            (copyOf (that creature)
              (copyCharacteristics
                { power := half .up (powerOf (that creature)),
                  toughness := half .up (toughnessOf (that creature)) } false))
            [])) ]
}
