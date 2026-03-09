use crate::effect::effect_def::*;
use crate::player::PlayerId;
use mlua::UserData;

impl UserData for Effect {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("trigger", |_, effect, trigger_str: String| {
            let trigger = match trigger_str.as_str() {
                "turn_start" => Trigger::TurnStart(true),
                "opponent_turn_start" => Trigger::TurnStart(false),
                "own_main_phase" => Trigger::OwnMainPhase,
                "opponent_main_phase" => Trigger::OpponentMainPhase,
                "either_main_phase" => Trigger::EitherMainPhase,
                "attack_phase" => Trigger::AttackPhase,
                "defense_phase" => Trigger::DefensePhase,
                "damage_phase" => Trigger::DamagePhase,
                "exposed" => Trigger::Exposed,
                "destroyed" => Trigger::Destroyed,
                "summoned" => Trigger::Summoned,
                "end_phase" => Trigger::EndPhase,
                "on_demand" => Trigger::OnDemand,
                _ => Trigger::Custom(trigger_str),
            };
            effect.trigger = trigger;
            Ok(())
        });

        methods.add_method_mut("optional", |_, effect, optional: bool| {
            effect.optional = optional;
            Ok(())
        });

        methods.add_method_mut("activation_limit", |_, effect, limit_str: String| {
            let limit = match limit_str.as_str() {
                "once_per_turn" => ActivationLimit::OncePerTurn,
                "once_per_game" => ActivationLimit::OncePerGame,
                _ => ActivationLimit::OncePerPlayer(limit_str),
            };
            effect.activation_limit = Some(limit);
            Ok(())
        });

        methods.add_method_mut("name", |_, effect, name: String| {
            effect.name = name;
            Ok(())
        });

        methods.add_method_mut("description", |_, effect, description: String| {
            effect.description = description;
            Ok(())
        });

        methods.add_method_mut(
            "addDrawCardAction",
            |_, effect, (player_id, count): (PlayerId, u32)| {
                let new_action = Action::DrawCard(player_id, count);
                effect.actions.push(new_action);
                Ok(())
            },
        );

        methods.add_method_mut("addDestroyCardAction", |_, effect, target_index: usize| {
            let new_action = Action::DestroyCard(TargetSelector::ChosenInChoice(target_index));
            effect.actions.push(new_action);
            Ok(())
        });

        methods.add_method_mut(
            "addHealAction",
            |_, effect, (player_id, amount): (PlayerId, u32)| {
                let new_action = Action::Heal(player_id, amount);
                effect.actions.push(new_action);
                Ok(())
            },
        );

        methods.add_method_mut(
            "addDealDamageAction",
            |_, effect, (target_index, damage): (usize, u32)| {
                let new_action =
                    Action::DealDamage(TargetSelector::ChosenInChoice(target_index), damage);
                effect.actions.push(new_action);
                Ok(())
            },
        );

        methods.add_method_mut(
            "addRealPointAction",
            |_, effect, (player_id, points): (PlayerId, i32)| {
                let new_action = Action::AddRealPoint(player_id, points);
                effect.actions.push(new_action);
                Ok(())
            },
        );

        methods.add_method_mut(
            "addSendToGraveyardAction",
            |_, effect, target_index: usize| {
                let new_action =
                    Action::SendCardToGraveyard(TargetSelector::ChosenInChoice(target_index));
                effect.actions.push(new_action);
                Ok(())
            },
        );

        methods.add_method_mut(
            "addMoveCardAction",
            |_, effect, (target_index, zone_str, count): (usize, String, u32)| {
                let zone = match zone_str.as_str() {
                    "hand" => ZoneType::Hand,
                    "front" => ZoneType::FieldFront,
                    "back" => ZoneType::FieldBack,
                    "deck" => ZoneType::Deck,
                    "graveyard" => ZoneType::Graveyard,
                    "cost_zone" => ZoneType::CostZone,
                    _ => ZoneType::Hand,
                };

                let new_action =
                    Action::MoveCard(TargetSelector::ChosenInChoice(target_index), zone, count);
                effect.actions.push(new_action);
                Ok(())
            },
        );

        methods.add_method_mut("addCustomAction", |_, effect, desc: String| {
            let action = Action::Custom(desc, vec![]);
            effect.actions.push(action);
            Ok(())
        });

        methods.add_method_mut("addChoice", |_, effect, _choice_desc: String| {
            // For this specific user requirement: define an empty choice for now
            let choice = Choice::NoChoice;
            effect.choices.push(choice);
            Ok(())
        });
    }
}
