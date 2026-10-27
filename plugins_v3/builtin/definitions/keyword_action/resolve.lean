/-- "<subject> resolves" (`Definition`, [CR#608.1,608.2]): the resolution of a spell or ability.
No player pays for, casts it from a zone or chooses its result, so the payment, origin and
resulting slots are the neutral `itsOwnCost`, `none` and `none`.

"Put into a graveyard during its resolution" (haunt's spell half [CR#702.55a]) and "as it
resolves" (buyback [CR#702.27a], rebound [CR#702.88a]) write the resolution as the cause of the
move: `GameEvent.causes (.event (does .any (resolve thisSpell))) <move>` on a trigger event and
`WouldClause.causedBy (.event …) <clause>` on a replacement's would-clause. `Causing.event` is the
relation the rules give: the spell is put into the graveyard as the final part of its resolution
[CR#608.2n], so the resolution causes the move, and the one form serves a trigger event and a
would-clause alike. `Timing.whileDoing` would state only a co-occurrence and lives in the timing
slot only, which a replacement's would-clause has not. A countered spell's move (`counter`) is
caused by no resolution [CR#701.6a], so none of the three applies to it. -/
action resolve (subject : NounPhrase) : Action := {
  -- TODO(spelling): <subject> resolves/resolve; string substitution cannot choose
  -- subject-verb agreement for a singular spell versus multiple spells or abilities.
  body :=
    .procedure .resolve subject .itsOwnCost none
        none none
}
