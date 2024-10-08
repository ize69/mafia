use serde::{Serialize, Deserialize};

use crate::game::chat::ChatMessageVariant;
use crate::game::grave::GraveKiller;
use crate::game::phase::PhaseType;
use crate::game::player::PlayerReference;
use crate::game::role_list::Faction;
use crate::game::Game;

use super::jester::Jester;
use super::{Priority, RoleStateImpl, Role, RoleState};

#[derive(Clone, Debug, Serialize, Default)]
pub struct Doomsayer {
    pub guesses: [(PlayerReference, DoomsayerGuess); 3],
    pub won: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum DoomsayerGuess{
    Mafia, #[default] Neutral, Fiends, Cult,

    Jailor, 
    // No TI
    Doctor, Bodyguard, Cop, Bouncer, Engineer, Armorsmith,
    Vigilante, Veteran, Marksman, Deputy,
    Escort, Medium, Retributionist, Journalist, Mayor, Transporter
}
impl DoomsayerGuess{
    fn convert_to_guess(role: Role)->Option<DoomsayerGuess>{
        match role {
            Role::Jailor => Some(DoomsayerGuess::Jailor),

            Role::Detective | Role::Lookout | Role::Spy | 
            Role::Tracker | Role::Philosopher | Role::Psychic | 
            Role::Auditor | Role::Snoop | Role::Gossip | Role::FlowerGirl => None, 

            Role::Doctor => Some(DoomsayerGuess::Doctor),
            Role::Bodyguard => Some(DoomsayerGuess::Bodyguard),
            Role::Cop => Some(DoomsayerGuess::Cop),
            Role::Bouncer => Some(DoomsayerGuess::Bouncer),
            Role::Engineer => Some(DoomsayerGuess::Engineer),
            Role::Armorsmith => Some(DoomsayerGuess::Armorsmith),

            Role::Vigilante => Some(DoomsayerGuess::Vigilante),
            Role::Veteran => Some(DoomsayerGuess::Veteran),
            Role::Marksman => Some(DoomsayerGuess::Marksman),
            Role::Deputy => Some(DoomsayerGuess::Deputy),

            Role::Escort => Some(DoomsayerGuess::Escort),
            Role::Medium => Some(DoomsayerGuess::Medium),
            Role::Retributionist => Some(DoomsayerGuess::Retributionist),
            Role::Journalist => Some(DoomsayerGuess::Journalist),
            Role::Mayor => Some(DoomsayerGuess::Mayor),
            Role::Transporter => Some(DoomsayerGuess::Transporter),

            //Mafia
            Role::Godfather | Role::Mafioso | Role::Eros | Role::Counterfeiter | Role::Retrainer |
            Role::Hypnotist | Role::Blackmailer | Role::Informant | 
            Role::Witch | Role::Necromancer | Role::Consort |
            Role::Mortician | Role::Framer | Role::Forger | 
            Role::Cupid | Role::MafiaSupportWildcard => Some(DoomsayerGuess::Mafia),

            //Neutral
            Role::Jester | Role::RabbleRouser | Role::Politician |
            Role::Doomsayer | Role::Death | Role::Minion | Role::Scarecrow |
            Role::Wildcard | Role::TrueWildcard => Some(DoomsayerGuess::Neutral),
            Role::Martyr => None,

            //Fiends
            Role::Arsonist | Role::Werewolf | 
            Role::Ojo | Role::Puppeteer | Role::Pyrolisk | Role::Kira |
            Role::FiendsWildcard => Some(DoomsayerGuess::Fiends),
            
            //Cult
            Role::Apostle | Role::Disciple | Role::Zealot => Some(DoomsayerGuess::Cult),
        }
    }
    fn guess_matches_role(&self, role: Role)->bool{
        if let Some(guess) = Self::convert_to_guess(role) {
            *self == guess
        }else{
            false
        }
    }
}

pub(super) const FACTION: Faction = Faction::Neutral;
pub(super) const MAXIMUM_COUNT: Option<u8> = Some(1);
pub(super) const DEFENSE: u8 = 0;

impl RoleStateImpl for Doomsayer {
    fn do_night_action(self, game: &mut Game, actor_ref: PlayerReference, priority: Priority) {
        if priority != Priority::TopPriority {return;}

        if actor_ref.night_blocked(game) {return;}
        if !actor_ref.alive(game) {return;}


        let mut won = true;
        for (player, guess) in self.guesses.iter(){
            if 
                *player == actor_ref || //cant guess yourself
                !player.alive(game) || //cant guess dead player
                !guess.guess_matches_role(player.role(game)) || //cant guess wrong
                self.guesses.iter().filter(|(other_p, _other_g)|{
                    *other_p == *player
                }).count() != 1 //cant guess a player more than once
            {
                won = false;
                break;
            }
        };

        if won{
            actor_ref.add_private_chat_message(game, ChatMessageVariant::DoomsayerWon);
            self.guesses[0].0.try_night_kill(actor_ref, game, GraveKiller::Role(super::Role::Doomsayer), 3, true);
            self.guesses[1].0.try_night_kill(actor_ref, game, GraveKiller::Role(super::Role::Doomsayer), 3, true);
            self.guesses[2].0.try_night_kill(actor_ref, game, GraveKiller::Role(super::Role::Doomsayer), 3, true);
            actor_ref.try_night_kill(actor_ref, game, GraveKiller::Suicide, 3, false);
            actor_ref.set_role_state(game, RoleState::Doomsayer(Doomsayer { guesses: self.guesses, won: true }));
        }else{
            actor_ref.add_private_chat_message(game, ChatMessageVariant::DoomsayerFailed);
        }
    
    }
    fn get_won_game(self, _game: &Game, _actor_ref: PlayerReference) -> bool {
        self.won
    }
    fn on_phase_start(self, game: &mut Game, actor_ref: PlayerReference, _phase: PhaseType) {
        Doomsayer::check_and_convert_to_jester(game, self, actor_ref);
    }
    fn on_role_creation(self, game: &mut Game, actor_ref: PlayerReference) {
        Doomsayer::check_and_convert_to_jester(game, self, actor_ref);
    }
    fn on_any_death(self, game: &mut Game, actor_ref: PlayerReference, _dead_player_ref: PlayerReference){
        Doomsayer::check_and_convert_to_jester(game, self, actor_ref);
    }
}
impl Doomsayer{
    pub fn check_and_convert_to_jester(game: &mut Game, doomsayer: Doomsayer, actor_ref: PlayerReference){
        if
            !doomsayer.won && actor_ref.alive(game) &&
            PlayerReference::all_players(game).filter(|player|
                player.alive(game) && DoomsayerGuess::convert_to_guess(player.role(game)).is_some() && *player != actor_ref
            ).count() < 3
        {
            actor_ref.set_role(game, RoleState::Jester(Jester::default()));
        }
    }
}