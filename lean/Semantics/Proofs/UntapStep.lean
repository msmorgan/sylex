import Semantics.Cards.Data.Fixtures
import Semantics.Check
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Proofs.CardsHelpers

/-!
# Semantics.Proofs.UntapStep

The untap-step statics: what "doesn't untap", "untap … during each other player's untap step"
and "you may choose not to untap" expand to. In the untap step the active player determines which
permanents they control will untap, then untaps them [CR#502.3]; that untapping is the `untap`
Deed [CR#701.26b].
-/

open Semantics Semantics.Templates Semantics.Definitions

open Semantics.Templates.NounPhrase (a each)
open Semantics.Templates.Predicate (and)
open Semantics.Templates.Location (and)
open Semantics.Templates.NounPhrase (and)
open Semantics.Templates.StaticSpec (replace)

open Semantics.Templates.Action (replace)
open Semantics.Templates.Condition (and)
open Semantics.Templates.DetPhrase (a each)

namespace Semantics.Proofs.UntapStep

/-- "<subject> doesn't untap during [<whose>] untap step" forbids the `untap` Definition's Deed,
so it and a "can't untap" pattern constrain the same Deed. -/
theorem doesntUntapForbidsTheUntapDeed (subject : NounPhrase) (whose : Option NounPhrase) :
    (match doesntUntap subject whose with
      | .conditional (.cant _ action _) _ _ => action.namedDeed
      | _ => none) = (untap subject).namedDeed ∧
      (untap subject).namedDeed = some (.action "untap") := ⟨rfl, rfl⟩

/-- The stun counter's replacement effect [CR#122.1d]: "If a permanent with a stun counter on it
would become untapped, instead remove a stun counter from it." -/
private def stunReplacement : Ability :=
  static (replace (would any (untap (a (and [isPermanent, hasCounter (some stunCounter)]))))
    (instead (removeCounters (exactly 1) (some (Semantics.CounterKindSource.printed (stunCounter))) it)))

/-- UD-14 (`docs/engine-adrs.md`): "doesn't untap" is a prohibition, the engine's choice where
the CR is silent, not a replacement. It forbids, with `cant`, the very `untap` Deed the stun
counter's replacement watches, so on a tapped permanent with both the untap can't happen, the
stun replacement can't apply to it [CR#614.17c], and the stun counter stays; as a replacement
it would instead be ordered against the stun counter's [CR#616.1]. -/
theorem doesntUntapIsNoStunReplacement :
    (match doesntUntap thisPermanent none, stunReplacement with
      | .conditional (.cant _ forbidden _) _ _, .static (.replace (.would _ watched) _ _ _ _) =>
          forbidden.namedDeed = watched.namedDeed
      | _, _ => False) ∧
      Ability.check (DRS.ofList []) stunReplacement = [] :=
  ⟨rfl, by decide⟩

/-- The four untap-step cards contain the expanded `untapDuring` static. -/
theorem untapStepCardsUseUntapDuring :
    (match (Semantics.Cards.«Seedborn Muse» : Card) with
      | .singleFaced face => face.characteristics.text =
          [static (untapDuring (allOf youControl) (each isOtherPlayer))]
      | _ => False) ∧
    (match (Semantics.Cards.«Unwinding Clock» : Card) with
      | .singleFaced face => face.characteristics.text =
          [static (untapDuring (allOf (and [isArtifact, youControl])) (each isOtherPlayer))]
      | _ => False) ∧
    (match (Semantics.Cards.«Thousand Moons Infantry» : Card) with
      | .singleFaced face => face.characteristics.text =
          [static (untapDuring thisCreature (each isOtherPlayer))]
      | _ => False) ∧
    (match (Semantics.Cards.«Drumbellower» : Card) with
      | .singleFaced face => face.characteristics.text =
          [flying, static (untapDuring (allOf (and [isCreature, youControl])) (each isOtherPlayer))]
      | _ => False) := by
  exact ⟨rfl, rfl, rfl, rfl⟩

/-- "You may choose not to untap <subject> during [<whose>] untap step" is no permission to untap
it: it is the "doesn't untap" prohibition, holding only if you so choose. Untapping stays the
default [CR#502.3]. -/
theorem mayDeclineUntapIsChosenProhibition (subject : NounPhrase) (whose : Option NounPhrase) :
    mayDeclineUntap subject whose ≠ duringUntapStep whose (can any (untap subject)) := by
  intro h; injection h with h; cases h

/-- The choice is a player's: "if this artifact so chooses" is refused. -/
theorem soChoosesNeedsAPlayer :
    Condition.check (DRS.ofList []) (.soChooses you) = [] ∧
      Condition.check (DRS.ofList []) (.soChooses thisArtifact)
        = [.kindMismatch .player .object] := by
  decide

/-- *Ashnod's Battle Gear* and *Helm of Possession* still check. -/
theorem mayDeclineUntapCardsCheck :
    Ability.check (DRS.ofList []) Cards.ashnodsBattleGear = [] ∧
      (Semantics.Cards.«Helm of Possession»).check = [] :=
  ⟨Cards.okAshnodsBattleGear, (by decide)⟩

end Semantics.Proofs.UntapStep
