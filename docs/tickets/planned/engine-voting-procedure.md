---
needs: []
---
# Voting in the engine

## Why

To vote, each player, starting with a specified player (not necessarily the active player) and
proceeding in turn order, chooses one of the listed choices [CR#701.38a]; a player with several
votes casts them all when they would otherwise vote [CR#701.38d]. The effect then depends on the
result. The engine has the decision's shape, `Vote { player, options }`
(`crates/engine/src/decide/pending/choice.rs`), and the default strategy answers one, but nothing
opens a vote and `Vote::resolve` is a `todo!()` naming this ticket. The `.syl` corpus already
writes votes (`vote`, `voteStartingWith`, `votesFor`, `withMostVotes`, `voteLead`), for cards
such as Council's Judgment, Coercive Portal and Ballot Broker.

## Done when

- An effect can open a vote that starts with its specified player and asks each player in turn
  order, collecting a player's extra votes at their turn to vote [CR#701.38a,701.38d].
- A continuation accumulates the votes across players and resumes the effect once the last vote
  is in, with the tally available to it: votes per choice, the choice or choices with the most
  votes, ties included.
- Tests with Council's Judgment (votes for objects, a tie exiling each tied permanent), Coercive
  Portal (votes for words with no rules meaning [CR#701.38b], a tie) and a player with an extra
  vote (Ballot Broker).

## Pointers

- `crates/engine/src/decide/pending/choice.rs` (`Vote`), `crates/engine/src/strategy.rs`
- `plugins/builtin/definitions/keyword_action/vote.syl`, `voteStartingWith.syl`,
  `plugins/builtin/primitives/quantity/votesFor.syl`,
  `plugins/builtin/primitives/predicate/withMostVotes.syl`,
  `plugins/builtin/primitives/condition/voteLead.syl`
- `plugins/canon/cards/Council's Judgment.syl`, `Coercive Portal.syl`, `Ballot Broker.syl`
