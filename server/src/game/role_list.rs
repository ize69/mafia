use serde::{Serialize, Deserialize};

use super::role::Role;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Faction{
    Mafia,
    Town,
    Neutral,
    Coven,
}
impl Faction{
    pub fn all_alignments(&self)->Vec<FactionAlignment>{
        match self{
            Faction::Mafia => vec![
                FactionAlignment::MafiaKilling,
                FactionAlignment::MafiaDeception,
                FactionAlignment::MafiaSupport
            ],
            Faction::Town => vec![
                FactionAlignment::TownInvestigative,
                FactionAlignment::TownProtective,
                FactionAlignment::TownKilling,
                FactionAlignment::TownSupport
            ],
            Faction::Neutral => vec![
                FactionAlignment::NeutralEvil,
                FactionAlignment::NeutralKilling,
                FactionAlignment::NeutralChaos,
                FactionAlignment::NeutralBenign,
            ],
            Faction::Coven => vec![
                FactionAlignment::CovenEvil
            ],
        }
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum FactionAlignment{
    MafiaKilling,
    MafiaDeception,
    MafiaSupport,

    TownInvestigative,
    TownProtective,
    TownKilling,
    TownSupport,

    NeutralEvil,
    NeutralKilling,
    NeutralBenign,
    NeutralChaos,

    CovenEvil
}
impl FactionAlignment{
    pub fn faction(&self)->Faction{
        match self {
            Self::MafiaKilling | Self::MafiaDeception | Self::MafiaSupport 
                => Faction::Mafia,
            Self::TownInvestigative | Self::TownProtective | Self::TownKilling | Self::TownSupport 
                => Faction::Town,
            Self::NeutralEvil | Self::NeutralKilling | Self::NeutralBenign | Self::NeutralChaos 
                => Faction::Neutral,
            Self::CovenEvil 
                => Faction::Coven,
        }
    }
}

pub type RoleList = Vec<RoleListEntry>;

pub fn create_random_roles(role_list: &RoleList) -> Vec<Role> {
    role_list.iter()
        .map(RoleListEntry::get_random_role)
        .collect()
}

pub fn get_all_possible_roles(role_list: &RoleList) -> Vec<Role> {
    //if executioner then add jester
    //if there could be mafioso at beginning then add godfather
    //if any mafia(besides godfather) then add mafioso
    todo!()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RoleListEntry {
    #[serde(rename_all = "camelCase")]
    Exact {
        faction: Faction,
        faction_alignment: FactionAlignment,
        role: Role,
    },
    #[serde(rename_all = "camelCase")]
    FactionAlignment {
        faction: Faction,
        faction_alignment: FactionAlignment,
    },
    #[serde(rename_all = "camelCase")]
    Faction {
        faction: Faction,
    },
    Any
}

impl RoleListEntry{
    pub fn get_random_role(&self) -> Role {
        let roles = self.get_possible_roles();

        match roles.get(rand::random::<usize>() % roles.len()) {
            Some(role) => role.clone(),
            None => {
                //if cant find role and was any, crash
                if *self == RoleListEntry::Any{
                    panic!("No roles in get_possible_roles");
                }
                //if cant find role then try again with any
                else{
                    RoleListEntry::Any.get_random_role()
                }
            },
        }
    }
    pub fn get_possible_roles(&self) -> Vec<Role> {
        match self {
            RoleListEntry::Exact {role, .. } => vec![role.clone()],
            RoleListEntry::FactionAlignment { faction_alignment, .. } => 
                Role::values().into_iter().filter(|role|{
                    role.get_faction_alignment() == *faction_alignment
                }).collect(),
            RoleListEntry::Faction { faction, .. } => Role::values().into_iter().filter(|role|{
                role.get_faction_alignment().faction() == *faction
            }).collect(),
            RoleListEntry::Any => Role::values(),
        }
    }
}