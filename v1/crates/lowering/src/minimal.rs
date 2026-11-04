//! Minimal values, one per grammar type, for the per-variant lowering
//! tests. Generated; each picks a variant whose fields all ground.

#![allow(
    dead_code,
    reason = "a type not used at any field position needs no helper caller"
)]

/// Lower `f` inside a minimal spell region.
///
/// Instruction lowering DEFINES registers — a damage or life-change magnitude
/// is pinned by a `Let` at its evaluation moment ([CR#608.2h]) — so any value
/// carrying an instruction can only be lowered with a region context open.
pub fn in_spell_region<T>(f: impl FnOnce() -> T) -> T {
    crate::region::in_region(crate::region::RegionKind::Spell, 0, f).1
}

/// The lowered image of [`minimal_one_shot_effect`]: `DealDamage` pins its
/// magnitude in a `Let` at the instruction's program point and then reads that
/// register ([CR#608.2h]), so one authored action becomes a two-instruction
/// sequence whose second half reads the first half's definition.
pub fn is_minimal_lowered_effect(effect: &sylex_core::Instruction) -> bool {
    let sylex_core::Instruction::Sequentially(parts) = effect else {
        return false;
    };
    let [
        sylex_core::Instruction::Let(sylex_core::Let {
            dest,
            expr: sylex_core::Expr::Number(_),
        }),
        sylex_core::Instruction::Act {
            dest: None,
            action:
                sylex_core::Action::DealDamage(
                    sylex_core::Reference::Reg(sylex_core::RefId(0)),
                    sylex_core::Count::Reg(read),
                    sylex_core::Reference::Reg(sylex_core::RefId(0)),
                ),
        },
    ] = parts.as_ref()
    else {
        return false;
    };
    sylex_core::RefId::from(*dest) == *read
}

pub fn minimal_ability() -> sylex_semantics_v1::Ability {
    sylex_semantics_v1::Ability::Static(std::sync::Arc::new(minimal_static_effect()))
}

pub fn minimal_action() -> sylex_semantics_v1::Action {
    sylex_semantics_v1::Action::DealDamage(
        minimal_reference(),
        minimal_count(),
        minimal_reference(),
    )
}

pub fn minimal_activated_ability() -> sylex_semantics_v1::ActivatedAbility {
    sylex_semantics_v1::ActivatedAbility {
        ability_word: None,
        cost: minimal_cost(),
        from: None,
        window: None,
        condition: None,
        limits: [].into(),
        effect: minimal_one_shot_effect(),
    }
}

pub fn minimal_additional_cost() -> sylex_semantics_v1::AdditionalCost {
    sylex_semantics_v1::AdditionalCost {
        pay: minimal_cost(),
        body: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_adjacency() -> sylex_semantics_v1::Adjacency {
    sylex_semantics_v1::Adjacency::Above
}

pub fn minimal_agency() -> sylex_semantics_v1::Agency {
    sylex_semantics_v1::Agency::CostPayment
}

pub fn minimal_aggregate_op() -> sylex_semantics_v1::AggregateOp {
    sylex_semantics_v1::AggregateOp::SumOf
}

pub fn minimal_alternative_cost() -> sylex_semantics_v1::AlternativeCost {
    sylex_semantics_v1::AlternativeCost::Free
}

pub fn minimal_anchor() -> sylex_semantics_v1::Anchor {
    sylex_semantics_v1::Anchor::FromTop(minimal_count())
}

pub fn minimal_arrangement() -> sylex_semantics_v1::Arrangement {
    sylex_semantics_v1::Arrangement::ChosenOrder(minimal_reference())
}

pub fn minimal_as_though() -> sylex_semantics_v1::AsThough {
    sylex_semantics_v1::AsThough::Counterfactual {
        premise: minimal_predicate(),
        then: std::sync::Arc::new(minimal_deontic()),
    }
}

pub fn minimal_beginning_step() -> sylex_semantics_v1::BeginningStep {
    sylex_semantics_v1::BeginningStep::Untap
}

pub fn minimal_binder() -> sylex_semantics_v1::Binder {
    sylex_semantics_v1::Binder::TheRef(minimal_reference())
}

pub fn minimal_card() -> sylex_semantics_v1::Card {
    sylex_semantics_v1::Card::Normal(minimal_card_face())
}

pub fn minimal_card_face() -> sylex_semantics_v1::CardFace {
    sylex_semantics_v1::CardFace {
        name: "x".into(),
        mana_cost: sylex_semantics_v1::ManaCost::from(std::sync::Arc::<
            [sylex_semantics_v1::ManaSymbol],
        >::from([])),
        color_indicator: Vec::new(),
        supertypes: Vec::new(),
        types: Vec::new(),
        subtypes: Vec::new(),
        abilities: Vec::new(),
        power: None,
        toughness: None,
        loyalty: None,
        defense: None,
    }
}

pub fn minimal_cause() -> sylex_semantics_v1::Cause {
    sylex_semantics_v1::Cause::Cause(minimal_cause_pattern())
}

pub fn minimal_cause_pattern() -> sylex_semantics_v1::CausePattern {
    sylex_semantics_v1::CausePattern {
        verb: None,
        agency: None,
        agent: None,
    }
}

pub fn minimal_characteristic() -> sylex_semantics_v1::Characteristic {
    sylex_semantics_v1::Characteristic::Colors
}

pub fn minimal_characteristic_predicate() -> sylex_semantics_v1::CharacteristicPredicate {
    sylex_semantics_v1::CharacteristicPredicate::Type(minimal_type_ref())
}

pub fn minimal_choose_pile() -> sylex_semantics_v1::ChoosePile {
    sylex_semantics_v1::ChoosePile {
        from: minimal_pile_source(),
        by: minimal_reference(),
        random: false,
        then: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_choose_spec() -> sylex_semantics_v1::ChooseSpec {
    sylex_semantics_v1::ChooseSpec {
        count: minimal_quantity(),
        up_to: false,
        repeats: false,
        chooser: minimal_reference(),
        rider: None,
    }
}

pub fn minimal_chosen_value_kind() -> sylex_semantics_v1::ChosenValueKind {
    sylex_semantics_v1::ChosenValueKind::Color
}

pub fn minimal_cmp() -> sylex_semantics_v1::Cmp {
    sylex_semantics_v1::Cmp::Eq
}

pub fn minimal_collection_op<T>() -> sylex_semantics_v1::CollectionOp<T> {
    sylex_semantics_v1::CollectionOp::Set([].into())
}

pub fn minimal_color() -> sylex_semantics_v1::Color {
    sylex_semantics_v1::Color::White
}

pub fn minimal_color_or_colorless() -> sylex_semantics_v1::ColorOrColorless {
    sylex_semantics_v1::ColorOrColorless::Colorless
}

pub fn minimal_combat_step() -> sylex_semantics_v1::CombatStep {
    sylex_semantics_v1::CombatStep::BeginningOfCombat
}

pub fn minimal_condition() -> sylex_semantics_v1::Condition {
    sylex_semantics_v1::Condition::Compare(minimal_count(), minimal_cmp(), minimal_count())
}

pub fn minimal_conferral_rule() -> sylex_semantics_v1::ConferralRule {
    sylex_semantics_v1::ConferralRule {
        scope: minimal_predicate(),
        confer: minimal_property(),
    }
}

pub fn minimal_continuously() -> sylex_semantics_v1::Continuously {
    sylex_semantics_v1::Continuously {
        effect: std::sync::Arc::new(minimal_static_effect()),
        duration: minimal_duration(),
    }
}

pub fn minimal_copiable_values() -> sylex_semantics_v1::CopiableValues {
    sylex_semantics_v1::CopiableValues {
        name: "x".into(),
        mana_cost: sylex_semantics_v1::ManaCost::from(std::sync::Arc::<
            [sylex_semantics_v1::ManaSymbol],
        >::from([])),
        color_indicator: Vec::new(),
        supertypes: Vec::new(),
        types: Vec::new(),
        subtypes: Vec::new(),
        abilities: Vec::new(),
        power: None,
        toughness: None,
        loyalty: None,
        defense: None,
    }
}

pub fn minimal_copy_exception() -> sylex_semantics_v1::CopyException {
    sylex_semantics_v1::CopyException::Modify(minimal_modification())
}

pub fn minimal_copy_retarget() -> sylex_semantics_v1::CopyRetarget {
    sylex_semantics_v1::CopyRetarget::AsIs
}

pub fn minimal_copy_source() -> sylex_semantics_v1::CopySource {
    sylex_semantics_v1::CopySource::Object(minimal_reference())
}

pub fn minimal_copy_spec() -> sylex_semantics_v1::CopySpec {
    sylex_semantics_v1::CopySpec {
        source: minimal_copy_source(),
        exceptions: Vec::new(),
    }
}

pub fn minimal_cost() -> sylex_semantics_v1::Cost {
    sylex_semantics_v1::Cost([].into())
}

pub fn minimal_cost_change() -> sylex_semantics_v1::CostChange {
    sylex_semantics_v1::CostChange::Increase([].into())
}

pub fn minimal_cost_component() -> sylex_semantics_v1::CostComponent {
    sylex_semantics_v1::CostComponent::Mana(sylex_semantics_v1::ManaCost::from(std::sync::Arc::<
        [sylex_semantics_v1::ManaSymbol],
    >::from([])))
}

pub fn minimal_cost_predicate() -> sylex_semantics_v1::CostPredicate {
    sylex_semantics_v1::CostPredicate::IncludesTapSymbol
}

pub fn minimal_cost_tag() -> sylex_semantics_v1::CostTag {
    sylex_semantics_v1::CostTag("X".into())
}

pub fn minimal_count() -> sylex_semantics_v1::Count {
    sylex_semantics_v1::Count::Literal(0)
}

pub fn minimal_count_bound() -> sylex_semantics_v1::CountBound {
    sylex_semantics_v1::CountBound::Eq(minimal_count())
}

pub fn minimal_countable() -> sylex_semantics_v1::Countable {
    sylex_semantics_v1::Countable::Objects(std::sync::Arc::new(minimal_predicate()))
}

pub fn minimal_counter() -> sylex_semantics_v1::Counter {
    sylex_semantics_v1::Counter {
        name: "X".into(),
        scope: minimal_counter_scope(),
        confers: Vec::new(),
    }
}

pub fn minimal_counter_ref() -> sylex_semantics_v1::CounterRef {
    sylex_semantics_v1::CounterRef("X".into())
}

pub fn minimal_counter_scope() -> sylex_semantics_v1::CounterScope {
    sylex_semantics_v1::CounterScope::Object
}

pub fn minimal_counter_spec() -> sylex_semantics_v1::CounterSpec {
    sylex_semantics_v1::CounterSpec::Named(minimal_counter_ref(), minimal_count())
}

pub fn minimal_damage_result_rule() -> sylex_semantics_v1::DamageResultRule {
    sylex_semantics_v1::DamageResultRule {
        recipient: minimal_predicate(),
        remove: minimal_counter_ref(),
    }
}

pub fn minimal_decider_spec() -> sylex_semantics_v1::DeciderSpec {
    sylex_semantics_v1::DeciderSpec::Controller
}

pub fn minimal_deed_agent() -> sylex_semantics_v1::DeedAgent {
    sylex_semantics_v1::DeedAgent {
        stack_object: None,
        source: None,
    }
}

pub fn minimal_deontic() -> sylex_semantics_v1::Deontic {
    sylex_semantics_v1::Deontic::May(minimal_deontic_action())
}

pub fn minimal_deontic_action() -> sylex_semantics_v1::DeonticAction {
    sylex_semantics_v1::DeonticAction::Attack {
        by: minimal_predicate(),
        on: minimal_predicate(),
    }
}

pub fn minimal_designation_decl() -> sylex_semantics_v1::DesignationDecl {
    sylex_semantics_v1::DesignationDecl {
        name: "X".into(),
        definition: minimal_designation_def(),
    }
}

pub fn minimal_designation_def() -> sylex_semantics_v1::DesignationDef {
    sylex_semantics_v1::DesignationDef::Stored {
        scope: minimal_designation_scope(),
        shape: minimal_designation_shape(),
        uniqueness: minimal_designation_uniqueness(),
        persistence: minimal_designation_persistence(),
        payload: [].into(),
    }
}

pub fn minimal_designation_persistence() -> sylex_semantics_v1::DesignationPersistence {
    sylex_semantics_v1::DesignationPersistence::ObjectLifetime
}

pub fn minimal_designation_scope() -> sylex_semantics_v1::DesignationScope {
    sylex_semantics_v1::DesignationScope::Object
}

pub fn minimal_designation_shape() -> sylex_semantics_v1::DesignationShape {
    sylex_semantics_v1::DesignationShape::Flag
}

pub fn minimal_designation_uniqueness() -> sylex_semantics_v1::DesignationUniqueness {
    sylex_semantics_v1::DesignationUniqueness::None
}

pub fn minimal_destination() -> sylex_semantics_v1::Destination {
    sylex_semantics_v1::Destination::Zone(minimal_zone())
}

pub fn minimal_distribute() -> sylex_semantics_v1::Distribute {
    sylex_semantics_v1::Distribute {
        amount: minimal_count(),
        binder: minimal_binder(),
        body: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_duration() -> sylex_semantics_v1::Duration {
    sylex_semantics_v1::Duration::FixedUntil(minimal_turn_marker())
}

pub fn minimal_each() -> sylex_semantics_v1::Each {
    sylex_semantics_v1::Each {
        binder: minimal_binder(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_ending_step() -> sylex_semantics_v1::EndingStep {
    sylex_semantics_v1::EndingStep::End
}

pub fn minimal_enter_rider() -> sylex_semantics_v1::EnterRider {
    sylex_semantics_v1::EnterRider::Tapped
}

pub fn minimal_event_filter() -> sylex_semantics_v1::EventFilter {
    sylex_semantics_v1::EventFilter::ZoneChange {
        what: minimal_predicate(),
        from: None,
        to: None,
        cause: None,
    }
}

pub fn minimal_face() -> sylex_semantics_v1::Face {
    sylex_semantics_v1::Face::Up
}

pub fn minimal_face_down_characteristics() -> sylex_semantics_v1::FaceDownCharacteristics {
    sylex_semantics_v1::FaceDownCharacteristics {
        name: None,
        types: Vec::new(),
        subtypes: Vec::new(),
        abilities: Vec::new(),
        power: None,
        toughness: None,
    }
}

pub fn minimal_face_down_spec() -> sylex_semantics_v1::FaceDownSpec {
    sylex_semantics_v1::FaceDownSpec::Listed(minimal_face_down_characteristics())
}

pub fn minimal_face_layout() -> sylex_semantics_v1::FaceLayout {
    sylex_semantics_v1::FaceLayout::Transforming
}

pub fn minimal_if() -> sylex_semantics_v1::If {
    sylex_semantics_v1::If {
        condition: minimal_condition(),
        then: std::sync::Arc::new(minimal_one_shot_effect()),
        otherwise: None,
    }
}

pub fn minimal_ignore_rule() -> sylex_semantics_v1::IgnoreRule {
    sylex_semantics_v1::IgnoreRule::IgnoreLowest
}

pub fn minimal_keyword_ability() -> sylex_semantics_v1::KeywordAbility {
    sylex_semantics_v1::KeywordAbility::FirstStrike
}

pub fn minimal_keyword_decl() -> sylex_semantics_v1::KeywordDecl {
    sylex_semantics_v1::KeywordDecl {
        name: "X".into(),
        shape: minimal_param_shape(),
    }
}

pub fn minimal_keyword_ref() -> sylex_semantics_v1::KeywordRef {
    sylex_semantics_v1::KeywordRef("X".into())
}

pub fn minimal_label() -> sylex_semantics_v1::Label {
    sylex_semantics_v1::Label {
        r#as: "X".into(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_life_op() -> sylex_semantics_v1::LifeOp {
    sylex_semantics_v1::LifeOp::Set(minimal_count())
}

pub fn minimal_lock_point() -> sylex_semantics_v1::LockPoint {
    sylex_semantics_v1::LockPoint::Announce
}

pub fn minimal_lookback() -> sylex_semantics_v1::Lookback {
    sylex_semantics_v1::Lookback::ThisTurn
}

pub fn minimal_mana_cost() -> sylex_semantics_v1::ManaCost {
    sylex_semantics_v1::ManaCost::from(std::sync::Arc::<[sylex_semantics_v1::ManaSymbol]>::from([]))
}

pub fn minimal_mana_production() -> sylex_semantics_v1::ManaProduction {
    sylex_semantics_v1::ManaProduction::WithRiders {
        mana: minimal_mana_spec(),
        riders: [].into(),
    }
}

pub fn minimal_mana_rider() -> sylex_semantics_v1::ManaRider {
    sylex_semantics_v1::ManaRider::SpendOnly(minimal_predicate())
}

pub fn minimal_mana_spec() -> sylex_semantics_v1::ManaSpec {
    sylex_semantics_v1::ManaSpec::AnyColor
}

pub fn minimal_mana_symbol() -> sylex_semantics_v1::ManaSymbol {
    sylex_semantics_v1::ManaSymbol::Variable
}

pub fn minimal_may() -> sylex_semantics_v1::May {
    sylex_semantics_v1::May {
        who: minimal_reference(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
        if_did: None,
        if_not: None,
    }
}

pub fn minimal_modal() -> sylex_semantics_v1::Modal {
    sylex_semantics_v1::Modal {
        choose: minimal_choose_spec(),
        modes: [].into(),
    }
}

pub fn minimal_modal_cost_rider() -> sylex_semantics_v1::ModalCostRider {
    sylex_semantics_v1::ModalCostRider::Entwine(minimal_cost())
}

pub fn minimal_mode() -> sylex_semantics_v1::Mode {
    sylex_semantics_v1::Mode {
        effect: minimal_one_shot_effect(),
        cost: None,
    }
}

pub fn minimal_modification() -> sylex_semantics_v1::Modification {
    sylex_semantics_v1::Modification::Power(minimal_numeric_op())
}

pub fn minimal_noted_kind() -> sylex_semantics_v1::NotedKind {
    sylex_semantics_v1::NotedKind::Objects
}

pub fn minimal_noting() -> sylex_semantics_v1::Noting {
    sylex_semantics_v1::Noting {
        key: "X".into(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_numeric_op() -> sylex_semantics_v1::NumericOp {
    sylex_semantics_v1::NumericOp::Set(minimal_stat_value())
}

pub fn minimal_object_kind() -> sylex_semantics_v1::ObjectKind {
    sylex_semantics_v1::ObjectKind::Ability
}

pub fn minimal_one_shot_effect() -> sylex_semantics_v1::OneShotEffect {
    sylex_semantics_v1::OneShotEffect::Act(minimal_action())
}

pub fn minimal_optional_cost() -> sylex_semantics_v1::OptionalCost {
    sylex_semantics_v1::OptionalCost {
        components: [].into(),
        tag: minimal_cost_tag(),
        repeatable: false,
    }
}

pub fn minimal_outcome_gate_kind() -> sylex_semantics_v1::OutcomeGateKind {
    sylex_semantics_v1::OutcomeGateKind::CantLose
}

pub fn minimal_param_shape() -> sylex_semantics_v1::ParamShape {
    sylex_semantics_v1::ParamShape::None
}

pub fn minimal_pay_act() -> sylex_semantics_v1::PayAct {
    sylex_semantics_v1::PayAct::TapToPay(minimal_predicate())
}

pub fn minimal_phase_kind() -> sylex_semantics_v1::PhaseKind {
    sylex_semantics_v1::PhaseKind::Beginning
}

pub fn minimal_phase_step() -> sylex_semantics_v1::PhaseStep {
    sylex_semantics_v1::PhaseStep::Beginning(minimal_beginning_step())
}

pub fn minimal_phasing() -> sylex_semantics_v1::Phasing {
    sylex_semantics_v1::Phasing::In
}

pub fn minimal_pile_source() -> sylex_semantics_v1::PileSource {
    sylex_semantics_v1::PileSource::Labels([].into())
}

pub fn minimal_pip_class() -> sylex_semantics_v1::PipClass {
    sylex_semantics_v1::PipClass::Generic
}

pub fn minimal_planar_face() -> sylex_semantics_v1::PlanarFace {
    sylex_semantics_v1::PlanarFace::Blank
}

pub fn minimal_player_attr() -> sylex_semantics_v1::PlayerAttr {
    sylex_semantics_v1::PlayerAttr::Life
}

pub fn minimal_player_mod() -> sylex_semantics_v1::PlayerMod {
    sylex_semantics_v1::PlayerMod::SetTo(minimal_player_attr(), minimal_count())
}

pub fn minimal_predefined_token() -> sylex_semantics_v1::PredefinedToken {
    sylex_semantics_v1::PredefinedToken::Treasure
}

pub fn minimal_predicate() -> sylex_semantics_v1::Predicate {
    sylex_semantics_v1::Predicate::Kind(minimal_object_kind())
}

pub fn minimal_prevention() -> sylex_semantics_v1::Prevention {
    sylex_semantics_v1::Prevention::PreventNext {
        n: minimal_count(),
        from: minimal_predicate(),
        to: minimal_predicate(),
        duration: None,
    }
}

pub fn minimal_projection() -> sylex_semantics_v1::Projection {
    sylex_semantics_v1::Projection {
        of: minimal_countable(),
        by: std::sync::Arc::new(minimal_count()),
    }
}

pub fn minimal_property() -> sylex_semantics_v1::Property {
    sylex_semantics_v1::Property::Ability(std::sync::Arc::new(minimal_ability()))
}

pub fn minimal_quantity() -> sylex_semantics_v1::Quantity {
    sylex_semantics_v1::Quantity::Range(None, None)
}

pub fn minimal_reference() -> sylex_semantics_v1::Reference {
    sylex_semantics_v1::Reference::This
}

pub fn minimal_relation_predicate() -> sylex_semantics_v1::RelationPredicate {
    sylex_semantics_v1::RelationPredicate::ControlledBy(std::sync::Arc::new(minimal_predicate()))
}

pub fn minimal_replacement() -> sylex_semantics_v1::Replacement {
    sylex_semantics_v1::Replacement::Instead {
        would: minimal_event_filter(),
        instead: minimal_one_shot_effect(),
    }
}

pub fn minimal_retarget_mode() -> sylex_semantics_v1::RetargetMode {
    sylex_semantics_v1::RetargetMode::ChangeAll
}

pub fn minimal_reveal_until() -> sylex_semantics_v1::RevealUntil {
    sylex_semantics_v1::RevealUntil {
        whose: minimal_reference(),
        matches: minimal_predicate(),
        body: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_round_mode() -> sylex_semantics_v1::RoundMode {
    sylex_semantics_v1::RoundMode::RoundUp
}

pub fn minimal_sba_rule() -> sylex_semantics_v1::SbaRule {
    sylex_semantics_v1::SbaRule {
        scope: minimal_predicate(),
        when: minimal_condition(),
        then: minimal_one_shot_effect(),
    }
}

pub fn minimal_selection() -> sylex_semantics_v1::Selection {
    sylex_semantics_v1::Selection::SelectAll(minimal_predicate())
}

pub fn minimal_separate_piles() -> sylex_semantics_v1::SeparatePiles {
    sylex_semantics_v1::SeparatePiles {
        group: minimal_selection(),
        into: [].into(),
        by: minimal_reference(),
        note: None,
        then: None,
    }
}

pub fn minimal_simple_mana_symbol() -> sylex_semantics_v1::SimpleManaSymbol {
    sylex_semantics_v1::SimpleManaSymbol::Generic(0)
}

pub fn minimal_sort() -> sylex_semantics_v1::Sort {
    sylex_semantics_v1::Sort::Player
}

pub fn minimal_spell_ability() -> sylex_semantics_v1::SpellAbility {
    sylex_semantics_v1::SpellAbility {
        ability_word: None,
        effect: minimal_one_shot_effect(),
    }
}

pub fn minimal_stat() -> sylex_semantics_v1::Stat {
    sylex_semantics_v1::Stat::Power
}

pub fn minimal_stat_value() -> sylex_semantics_v1::StatValue {
    sylex_semantics_v1::StatValue::DefinedByAbility
}

pub fn minimal_state_change() -> sylex_semantics_v1::StateChange {
    sylex_semantics_v1::StateChange::Tapped
}

pub fn minimal_state_predicate() -> sylex_semantics_v1::StatePredicate {
    sylex_semantics_v1::StatePredicate::InZone(minimal_zone())
}

pub fn minimal_static_effect() -> sylex_semantics_v1::StaticEffect {
    sylex_semantics_v1::StaticEffect::Modify(minimal_reference(), minimal_modification())
}

pub fn minimal_status() -> sylex_semantics_v1::Status {
    sylex_semantics_v1::Status::Tapped
}

pub fn minimal_subtype() -> sylex_semantics_v1::Subtype {
    sylex_semantics_v1::Subtype {
        name: "X".into(),
        types: [].into(),
        confers: [].into(),
    }
}

pub fn minimal_subtype_ref() -> sylex_semantics_v1::SubtypeRef {
    sylex_semantics_v1::SubtypeRef(std::sync::Arc::new(minimal_subtype()))
}

pub fn minimal_supertype() -> sylex_semantics_v1::Supertype {
    sylex_semantics_v1::Supertype::Basic
}

pub fn minimal_symbol_pred() -> sylex_semantics_v1::SymbolPred {
    sylex_semantics_v1::SymbolPred::AnyColor
}

pub fn minimal_target_spec() -> sylex_semantics_v1::TargetSpec {
    sylex_semantics_v1::TargetSpec::Target(minimal_quantity(), minimal_predicate())
}

pub fn minimal_targeted() -> sylex_semantics_v1::Targeted {
    sylex_semantics_v1::Targeted {
        targets: [].into(),
        effect: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_timing() -> sylex_semantics_v1::Timing {
    sylex_semantics_v1::Timing::InstantSpeed
}

pub fn minimal_token() -> sylex_semantics_v1::Token {
    sylex_semantics_v1::Token {
        name: None,
        color_indicator: [].into(),
        supertypes: [].into(),
        types: [].into(),
        subtypes: [].into(),
        abilities: [].into(),
        power: None,
        toughness: None,
    }
}

pub fn minimal_token_name() -> sylex_semantics_v1::TokenName {
    sylex_semantics_v1::TokenName("X".into())
}

pub fn minimal_token_spec() -> sylex_semantics_v1::TokenSpec {
    sylex_semantics_v1::TokenSpec::Token(std::sync::Arc::new(minimal_token()))
}

pub fn minimal_total_cost() -> sylex_semantics_v1::TotalCost {
    sylex_semantics_v1::TotalCost {
        base: [].into(),
        trace: [].into(),
        locked: false,
    }
}

pub fn minimal_triggered_ability() -> sylex_semantics_v1::TriggeredAbility {
    sylex_semantics_v1::TriggeredAbility {
        ability_word: None,
        event: minimal_event_filter(),
        from: None,
        condition: None,
        limits: [].into(),
        where_x: None,
        effect: minimal_one_shot_effect(),
    }
}

pub fn minimal_turn_marker() -> sylex_semantics_v1::TurnMarker {
    sylex_semantics_v1::TurnMarker::EndOfTurn
}

pub fn minimal_type() -> sylex_semantics_v1::Type {
    sylex_semantics_v1::Type::Artifact
}

pub fn minimal_type_def() -> sylex_semantics_v1::TypeDef {
    sylex_semantics_v1::TypeDef {
        name: "X".into(),
        permanent: false,
        confers: [].into(),
    }
}

pub fn minimal_type_ref() -> sylex_semantics_v1::TypeRef {
    sylex_semantics_v1::TypeRef(std::sync::Arc::new(minimal_type_def()))
}

pub fn minimal_use_limit() -> sylex_semantics_v1::UseLimit {
    sylex_semantics_v1::UseLimit::OncePerTurn
}

pub fn minimal_verb_name() -> sylex_semantics_v1::VerbName {
    sylex_semantics_v1::VerbName("X".into())
}

pub fn minimal_visibility() -> sylex_semantics_v1::Visibility {
    sylex_semantics_v1::Visibility::Open
}

pub fn minimal_whose_turn() -> sylex_semantics_v1::WhoseTurn {
    sylex_semantics_v1::WhoseTurn::Your
}

pub fn minimal_with() -> sylex_semantics_v1::With {
    sylex_semantics_v1::With {
        binder: minimal_binder(),
        body: std::sync::Arc::new(minimal_one_shot_effect()),
    }
}

pub fn minimal_zone() -> sylex_semantics_v1::Zone {
    sylex_semantics_v1::Zone::Battlefield
}
