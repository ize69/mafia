
use rand::seq::SliceRandom;
use serde::Serialize;

use crate::game::chat::{ChatGroup, ChatMessageVariant};
use crate::game::phase::{PhaseType, PhaseState};
use crate::game::player::PlayerReference;
use crate::game::role::RoleState;
use crate::game::role_list::Faction;
use crate::game::verdict::Verdict;

use crate::game::Game;
use super::{Priority, RoleStateImpl};

#[derive(Clone, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Jester {
    lynched_yesterday: bool,
    won: bool,
}

pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = None;
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Jester {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::TopPriority {return;}

        if game.day_number() == 1 {
            actor_ref.increase_defense_to(game, 2);
        }

        if actor_ref.alive(game) {return;}
    
        if !self.lynched_yesterday {return}
        
        let all_killable_players: Vec<PlayerReference> = PlayerReference::all_players(game)
            .filter(|player_ref|{
                player_ref.alive(game) &&
                *player_ref != actor_ref &&
                player_ref.verdict(game) != Verdict::Innocent
            }).collect();
    
        let player = match actor_ref.selection(game).first() {
            Some(v) => *v,
            None => {
                let Some(target_ref) = all_killable_players.choose(&mut rand::thread_rng()) else {return};
                *target_ref
            },
        };
        player.try_night_kill(actor_ref, game, 
            crate::game::grave::GraveKiller::Role(super::Role::Jester), 3, true
        );
    }
    fn can_select(self, game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
        actor_ref != target_ref &&
        actor_ref.selection(game).is_empty() &&
        !actor_ref.alive(game) &&
        target_ref.alive(game) &&
        target_ref.verdict(game) != Verdict::Innocent &&
        self.lynched_yesterday
    }
    fn get_won_game(self, _game: &Game, _actor_ref: PlayerReference) -> bool {
        self.won
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType){
        match game.current_phase() {
            &PhaseState::FinalWords { player_on_trial } => {
                if player_on_trial == actor_ref {
                    actor_ref.set_role_state(game, RoleState::Jester(Jester { 
                        lynched_yesterday: true,
                        won: true
                    }));
                }
            }
            PhaseState::Obituary => {
                actor_ref.set_role_state(game, RoleState::Jester(Jester { 
                    lynched_yesterday: false,
                    won: self.won
                }));
            }
            _ => {}
        }
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, dead_player_ref: PlayerReference){
        if 
            actor_ref == dead_player_ref && 
            game.current_phase().phase() == PhaseType::FinalWords
        {
            game.add_message_to_chat_group(ChatGroup::All, ChatMessageVariant::JesterWon);
        }
    }
}
