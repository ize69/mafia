use std::collections::HashSet;

use crate::game::{chat::ChatGroup, player::PlayerReference, Game, visit::Visit, role_list::Faction, phase::{PhaseState, PhaseType}, resolution_state::ResolutionState};

use super::{journalist::Journalist, medium::Medium, same_evil_team, RoleState};


pub(super) fn can_night_select(game: &Game, actor_ref: PlayerReference, target_ref: PlayerReference) -> bool {
    
    actor_ref != target_ref &&
    !actor_ref.night_jailed(game) &&
    actor_ref.selection(game).is_empty() &&
    actor_ref.alive(game) &&
    target_ref.alive(game) &&
    !same_evil_team(game, actor_ref, target_ref)
}

pub(super) fn convert_selection_to_visits(_game: &Game, _actor_ref: PlayerReference, target_refs: Vec<PlayerReference>, attack: bool) -> Vec<Visit> {
    if !target_refs.is_empty() {
        vec![Visit{ target: target_refs[0], attack }]
    } else {
        Vec::new()
    }
}

pub(super) fn get_current_send_chat_groups(game: &Game, actor_ref: PlayerReference, mut night_chat_groups: Vec<ChatGroup>) -> HashSet<ChatGroup> {
    if !actor_ref.alive(game){
        return vec![ChatGroup::Dead].into_iter().collect();
    }
    if actor_ref.night_silenced(game){
        return HashSet::new();
    }

    match game.current_phase() {
        PhaseState::Briefing |
        PhaseState::Obituary => HashSet::new(),
        PhaseState::Discussion 
        | PhaseState::Nomination {..}
        | PhaseState::Judgement {..} 
        | PhaseState::FinalWords {..}
        | PhaseState::Dusk => vec![ChatGroup::All].into_iter().collect(),
        &PhaseState::Testimony { player_on_trial, .. } => {
            if player_on_trial == actor_ref {
                vec![ChatGroup::All].into_iter().collect()
            } else {
                HashSet::new()
            }
        },
        PhaseState::Night => {
            let mut out = vec![];
            if PlayerReference::all_players(game)
                .any(|med|{
                    match med.role_state(game) {
                        RoleState::Medium(Medium{ seanced_target: Some(seanced_target), .. }) => {
                            actor_ref == *seanced_target
                        },
                        _ => false
                    }
                })
            {
                out.push(ChatGroup::Dead);
            }
            if PlayerReference::all_players(game)
                .any(|p|
                    match p.role_state(game) {
                        RoleState::Journalist(Journalist{interviewed_target: Some(interviewed_target_ref), ..}) => {
                            *interviewed_target_ref == actor_ref
                        },
                        _ => false
                    }
                )
            {
                out.push(ChatGroup::Interview);
            }


            let mut jail_or_night_chats = if actor_ref.night_jailed(game){
                vec![ChatGroup::Jail]
            }else{
                match actor_ref.role(game).faction() {
                    Faction::Mafia => {
                        night_chat_groups.push(ChatGroup::Mafia);
                        night_chat_groups
                    },
                    Faction::Cult => {
                        night_chat_groups.push(ChatGroup::Cult);
                        night_chat_groups
                    },
                    _ => {
                        night_chat_groups
                    }
                }
            };


            out.append(&mut jail_or_night_chats);
            out.into_iter().collect()
        },
    }
}
pub(super) fn get_current_receive_chat_groups(game: &Game, actor_ref: PlayerReference) -> HashSet<ChatGroup> {
    let mut out = Vec::new();

    out.push(ChatGroup::All);

    if !actor_ref.alive(game){
        out.push(ChatGroup::Dead);
    }

    if actor_ref.role(game).faction() == Faction::Mafia {
        out.push(ChatGroup::Mafia);
    }
    if actor_ref.role(game).faction() == Faction::Cult {
        out.push(ChatGroup::Cult);
    }
    if actor_ref.night_jailed(game){
        out.push(ChatGroup::Jail);
    }
    if 
        game.current_phase().phase() == PhaseType::Night && 
        PlayerReference::all_players(game)
            .any(|med|{
                match med.role_state(game) {
                    RoleState::Medium(Medium{ seanced_target: Some(seanced_target), .. }) => {
                        actor_ref == *seanced_target
                    },
                    _ => false
                }
            })
    {
        out.push(ChatGroup::Dead);
    }
    if 
        game.current_phase().phase() == PhaseType::Night && 
        PlayerReference::all_players(game)
            .any(|p|
                match p.role_state(game) {
                    RoleState::Journalist(Journalist{interviewed_target: Some(interviewed_target_ref), ..}) => {
                        *interviewed_target_ref == actor_ref
                    },
                    _ => false
                }
            )
    {
        out.push(ChatGroup::Interview);
    }

    out.into_iter().collect()
}

///Only works for roles that win based on end game condition
pub(super) fn get_won_game(game: &Game, actor_ref: PlayerReference) -> bool {
    if let Some(end_game_condition) = ResolutionState::game_is_over(game) {
        ResolutionState::can_win_with(game, actor_ref, end_game_condition)
    } else {
        false
    }
}