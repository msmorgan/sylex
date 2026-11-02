---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, semantics-rust-rewrite]
---
# A term for the active player; phasing's direct phase-out

Standard constraints apply. Both halves rewrite the phasing turn-based action
(`plugins_v3/builtin/rules/conferral/phasingTurnBased.syl`); the second was folded in from
`lean-v3-phasing-direct-phase-out`. Its header comment names both slugs; update it with the row.

## The active player

The grammar has no noun phrase for the active player [CR#102.1]. The phasing turn-based action
needs one: "all phased-out permanents that the active player controlled when they phased out
phase in" [CR#703.4a,502.1]. The row reads the phased-out permanent's present controller during
that controller's turn instead. Phasing changes no control [CR#702.26d], so the two agree unless a
control effect ended while the permanent was phased out [CR#702.26f]. In that case the rule phases
it in during the untap step of the player who controlled it as it phased out, and the row would
wait for its present controller's.

Add the term, then rewrite the phase-in half as a lookback: this permanent phased out while the
active player controlled it.

## Direct and indirect phase-out

An Aura, Equipment or Fortification that phases out with the permanent it is attached to phases
out "indirectly" and won't phase in by itself, only with that permanent [CR#702.26g]. One that
phased out directly phases in during its controller's untap step [CR#703.4a,702.26i]. If both
would happen at once, it phases out indirectly [CR#702.26h].

The row reads "indirectly" from the attachment: it doesn't phase in a phased-out permanent
attached to a phased-out one. That is wrong in one case. An attachment that phased out directly
(Teferi's Protection on an Equipment, say) and whose host phased out later, directly, is skipped,
though the rule phases it in.

Fix: record how a permanent phased out. The phasing event is one `setStatus .phasedOut`, and an
indirect phase-out writes none of its own, so a lookback can't separate the two. Either give the
indirect phase-out its own event (the attachment's `setStatus` caused by the host's), so the row
reads "did not phase out caused by its host's phasing out", or add a status distinction. Choose
by the CR's structure (ADR `semantics-v3.md` §0.1 item 5).
