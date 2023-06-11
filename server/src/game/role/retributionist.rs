use serde::Serialize;

use crate::game::chat::ChatGroup;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::{FactionAlignment, Faction};
use crate::game::end_game_condition::EndGameCondition;
use crate::game::visit::Visit;
use crate::game::Game;
use crate::game::team::Team;
use super::{Priority, RoleState, RoleStateImpl};

pub(super) const DEFENSE: u8 = 0;
pub(super) const ROLEBLOCKABLE: bool = true;
pub(super) const WITCHABLE: bool = false;
pub(super) const SUSPICIOUS: bool = false;
pub(super) const FACTION_ALIGNMENT: FactionAlignment = FactionAlignment::TownSupport;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const END_GAME_CONDITION: EndGameCondition = EndGameCondition::Faction;
pub(super) const TEAM: Option<Team> = None;

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Retributionist { 
    used_bodies: Vec<PlayerReference>, 
    currently_used_player: Option<PlayerReference> 
}

impl RoleStateImpl for Retributionist {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if actor_ref.night_jailed(game) {return;}
        
        let retributionist_visits = actor_ref.night_visits(game).clone();
        let Some(first_visit) = retributionist_visits.get(0) else {return};
        let Some(second_visit) = retributionist_visits.get(1) else {return};
    
        if first_visit.target.alive(game) {return;}
    
        match priority{
            Priority::Necromancy => {
                first_visit.target.set_night_visits(
                    game,
                    first_visit.target.convert_targets_to_visits(game, vec![second_visit.target])
                );
                
                let mut used_bodies = self.used_bodies.clone();
                used_bodies.push(first_visit.target);
                actor_ref.set_role_state(game, RoleState::Retributionist(Retributionist { used_bodies, currently_used_player: Some(first_visit.target) }));
            },
            Priority::StealMessages => {
                if let Some(currently_used_player) = self.currently_used_player {
                    for message in currently_used_player.night_messages(game).clone() {
                        actor_ref.push_night_messages(game, message.clone());
                    }
                }
            },
            _ => {}
        }
    }
    fn can_night_target(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        !actor_ref.night_jailed(game) &&
        actor_ref.alive(game) &&
        ((
            actor_ref.chosen_targets(game).is_empty() &&
            !target_ref.alive(game) &&
            target_ref.role(game).faction_alignment().faction() == Faction::Town &&
            !self.used_bodies.iter().any(|p| *p == target_ref)
        ) || (
            actor_ref != target_ref &&
            actor_ref.chosen_targets(game).len() == 1 &&
            target_ref.alive(game)
    
        ))
    }
    fn do_day_action(self, _game: &mut Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) {
        
    }
    fn can_day_target(self, _game: &Game, _actor_ref: PlayerReference, _target_ref: PlayerReference) -> bool {
        false
    }
    fn convert_targets_to_visits(self, _game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>) -> Vec<Visit> {
        if target_refs.len() == 2 {
            vec![
                Visit{target: target_refs[0], astral: false, attack: false}, 
                Visit{target: target_refs[1], astral: true, attack: false}
            ]
        } else {
            Vec::new()
        }
    }
    fn get_current_send_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_send_chat_groups(game, actor_ref, vec![])
    }
    fn get_current_recieve_chat_groups(self, game: &Game, actor_ref: PlayerReference) -> Vec<ChatGroup> {
        crate::game::role::common_role::get_current_recieve_chat_groups(game, actor_ref)
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, phase: PhaseType){
        if phase == PhaseType::Night {
            actor_ref.set_role_state(game, RoleState::Retributionist(Retributionist { used_bodies: self.used_bodies, currently_used_player: None }));
        }
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference){
        crate::game::role::common_role::on_role_creation(game, actor_ref);
    }
    
}
