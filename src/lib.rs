use log::{debug};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[cfg(test)]
mod tests {
    use super::{
        run_action, Action, Card, Event, Game, Player, PlayerEvent, Unit, UnitEvent, UnitType,
    };

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn player1_skip_turn() {
        let mut game = Game {
            current_player_index: 0,
            ..Default::default()
        };
        run_action(&mut game, &Action::Skip);
        assert_eq!(game.current_player_index, 1);
    }

    #[test]
    fn player1_summon_unit() {
        let mut game = Game {
            player1: Player {
                hands: vec![Card {
                    unit: Unit {
                        hp: 50,
                        attack: 10,
                        defence: 5,
                        unit_type: UnitType::Attacker,
                        ..Default::default()
                    },
                }],
                ..Default::default()
            },
            current_player_index: 0,
            ..Default::default()
        };
        run_action(&mut game, &Action::Hand(0));
        assert_eq!(game.current_player_index, 1);
        assert_eq!(game.player1.units.len(), 1);
    }

    #[test]
    fn player1_draw_card() {
        let mut game = Game {
            player1: Player {
                decks: vec![Card {
                    unit: Default::default(),
                }],
                hp: 100,
                ..Default::default()
            },
            current_player_index: 1,
            ..Default::default()
        };

        assert_eq!(game.player1.decks.len(), 1);
        assert_eq!(game.player1.hands.len(), 0);
        assert_eq!(game.player1.graves.len(), 0);
        run_action(&mut game, &Action::Skip);
        assert_eq!(game.current_player_index, 0);
        assert_eq!(game.player1.decks.len(), 0);
        assert_eq!(game.player1.hands.len(), 1);
        assert_eq!(game.player1.graves.len(), 0);
    }

    #[test]
    fn player1_shuffle_and_draw_card() {
        let mut game = Game {
            player1: Player {
                graves: vec![Card {
                    unit: Default::default(),
                }],
                hp: 100,
                ..Default::default()
            },
            current_player_index: 1,
            ..Default::default()
        };

        assert_eq!(game.player1.decks.len(), 0);
        assert_eq!(game.player1.hands.len(), 0);
        assert_eq!(game.player1.graves.len(), 1);
        run_action(&mut game, &Action::Skip);
        assert_eq!(game.current_player_index, 0);
        assert_eq!(game.player1.decks.len(), 0);
        assert_eq!(game.player1.hands.len(), 1);
        assert_eq!(game.player1.graves.len(), 0);
    }
    #[test]
    fn player1_unit_attack() {
        let mut game = Game {
            player1: Player {
                units: vec![Unit {
                    hp: 50,
                    attack: 10,
                    unit_type: UnitType::Attacker,
                    ..Default::default()
                }],
                hp: 100,
                ..Default::default()
            },
            player2: Player {
                hp: 100,
                defence: 5,
                ..Default::default()
            },
            current_player_index: 1,
        };

        run_action(&mut game, &Action::Skip);
        assert_eq!(game.current_player_index, 0);
        // damage = attacker attack - player defence
        assert_eq!(game.player2.hp, 100 - (10 - 5));
    }

    #[test]
    fn player1_unit_heal() {
        let mut game = Game {
            player1: Player {
                units: vec![Unit {
                    hp: 50,
                    attack: 10,
                    unit_type: UnitType::Healer,
                    ..Default::default()
                }],
                hp: 100,
                ..Default::default()
            },
            player2: Player {
                hp: 100,
                defence: 5,
                ..Default::default()
            },
            current_player_index: 1,
        };

        run_action(&mut game, &Action::Skip);
        assert_eq!(game.current_player_index, 0);
        assert_eq!(game.player1.units.len(), 1);
        assert_eq!(game.player1.units[0].hp, 50 + 10);
    }

    #[test]
    fn player2_unit_die() {
        let mut game = Game {
            player1: Player {
                units: vec![Unit {
                    hp: 10,
                    attack: 10,
                    unit_type: UnitType::Attacker,
                    ..Default::default()
                }],
                hp: 100,
                ..Default::default()
            },
            player2: Player {
                units: vec![Unit {
                    hp: 10,
                    attack: 10,
                    unit_type: UnitType::Attacker,
                    ..Default::default()
                }],
                hp: 100,
                defence: 5,
                ..Default::default()
            },
            current_player_index: 1,
        };

        run_action(&mut game, &Action::Skip);
        assert_eq!(game.current_player_index, 0);
        // damage = attacker attack - player defence
        assert_eq!(game.player2.units.len(), 0);
    }
    #[test]
    fn player2_unit_die_and_attack_next() {
        let mut game = Game {
            player1: Player {
                units: vec![
                    Unit {
                        hp: 10,
                        attack: 10,
                        unit_type: UnitType::Attacker,
                        ..Default::default()
                    },
                    Unit {
                        hp: 10,
                        attack: 10,
                        unit_type: UnitType::Attacker,
                        ..Default::default()
                    },
                ],
                hp: 100,
                ..Default::default()
            },
            player2: Player {
                units: vec![
                    Unit {
                        hp: 10,
                        attack: 10,
                        unit_type: UnitType::Attacker,
                        ..Default::default()
                    },
                    Unit {
                        hp: 10,
                        attack: 10,
                        unit_type: UnitType::Attacker,
                        ..Default::default()
                    },
                ],
                hp: 100,
                defence: 5,
                ..Default::default()
            },
            current_player_index: 1,
        };

        run_action(&mut game, &Action::Skip);
        assert_eq!(game.current_player_index, 0);
        // damage = attacker attack - player defence
        assert_eq!(game.player2.units.len(), 0);
    }
    #[test]
    fn event_skip_and_4_attack_and_2_die() {
        let mut game = Game {
            player1: Player {
                units: vec![
                    Unit {
                        hp: 10,
                        attack: 5,
                        unit_type: UnitType::Attacker,
                        ..Default::default()
                    },
                    Unit {
                        hp: 10,
                        attack: 5,
                        unit_type: UnitType::Attacker,
                        ..Default::default()
                    },
                    Unit {
                        hp: 10,
                        attack: 5,
                        unit_type: UnitType::Attacker,
                        ..Default::default()
                    },
                    Unit {
                        hp: 10,
                        attack: 5,
                        unit_type: UnitType::Attacker,
                        ..Default::default()
                    },
                ],
                hp: 100,
                ..Default::default()
            },
            player2: Player {
                units: vec![
                    Unit {
                        hp: 10,
                        attack: 10,
                        unit_type: UnitType::Attacker,
                        ..Default::default()
                    },
                    Unit {
                        hp: 10,
                        attack: 10,
                        unit_type: UnitType::Attacker,
                        ..Default::default()
                    },
                ],
                hp: 100,
                defence: 5,
                ..Default::default()
            },
            current_player_index: 1,
        };

        let events = run_action(&mut game, &Action::Skip);
        assert_eq!(game.current_player_index, 0);
        // damage = attacker attack - player defence
        assert_eq!(game.player2.units.len(), 0);
        assert_eq!(events.len(), 7);
        assert_eq!(events[0], Event::Player(1, PlayerEvent::Skip));
        assert_eq!(events[1], Event::Unit(0, 0, UnitEvent::AttackUnit(0, 5)));
        assert_eq!(events[2], Event::Unit(0, 1, UnitEvent::AttackUnit(0, 5)));
        assert_eq!(events[3], Event::Unit(1, 0, UnitEvent::Die));
        assert_eq!(events[4], Event::Unit(0, 2, UnitEvent::AttackUnit(0, 5)));
        assert_eq!(events[5], Event::Unit(0, 3, UnitEvent::AttackUnit(0, 5)));
        assert_eq!(events[6], Event::Unit(1, 0, UnitEvent::Die));
    }
}

#[derive(Default, Debug, Clone)]
pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub current_player_index: usize,
}

#[derive(Default, Debug, Clone)]
pub struct Player {
    pub hands: Vec<Card>,
    pub graves: Vec<Card>,
    pub decks: Vec<Card>,

    pub units: Vec<Unit>,

    pub hp: i32,
    pub attack: i32,
    pub defence: i32,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Unit {
    pub name: String,
    pub hp: i32,
    pub attack: i32,
    pub defence: i32,
    pub sleep: i32,
    pub stun: i32,
    pub charm: i32,
    pub unit_type: UnitType,
}
#[derive(Debug, Clone, PartialEq)]
pub enum UnitType {
    Attacker,
    Healer,
    Archer,
    Assassin,
    Summoner,
    Mage,
}
impl Default for UnitType {
    fn default() -> Self {
        UnitType::Attacker
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Card {
    pub unit: Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Skip,
    Hand(usize),
}
impl Default for Action {
    fn default() -> Self {
        Action::Skip
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Unit(usize, usize, UnitEvent),
    Player(usize, PlayerEvent),
}
#[derive(Debug, Clone, PartialEq)]
pub enum UnitEvent {
    Die,
    Sleep,
    Stun,
    AttackUnit(usize, i32),
    AttackPlayer(i32),
    Heal(usize, i32),
    Summon(Unit),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerEvent {
    UseCard(usize, Card),
    Skip,
    Die,
    Shuffle,
    DrawCard,
}

pub fn run_action(game: &mut Game, action: &Action) -> Vec<Event> {
    let mut events = Vec::new();

    let (player, _opponent, player_index,_opponent_index) = if game.current_player_index == 0 {
        (&mut game.player1, &mut game.player2, 0, 1)
    } else {
        (&mut game.player2, &mut game.player1, 1, 0)
    };

    match action {
        Action::Skip => {
            events.push(Event::Player(player_index, PlayerEvent::Skip));
        }
        Action::Hand(index) => {
            let card = player.hands.remove(*index);
            events.push(Event::Player(
                player_index,
                PlayerEvent::UseCard(*index, card.clone()),
            ));
            player.units.push(card.unit.clone());
            player.graves.push(card);
        }
    }

    // Change player
    game.current_player_index = (game.current_player_index + 1) % 2;

    let (player, opponent, player_index, opponent_index) = if game.current_player_index == 0 {
        (&mut game.player1, &mut game.player2, 0, 1)
    } else {
        (&mut game.player2, &mut game.player1, 1, 0)
    };
    // Unit start
    let mut unit_index = 0;
    while unit_index < player.units.len() {
        // Start
        let mut skip_normal = false;
        {
            let unit = &mut player.units[unit_index];
            if unit.stun > 0 {
                skip_normal = true;
                events.push(Event::Unit(player_index, unit_index, UnitEvent::Stun));
                debug!("Unit {} is stun!", unit_index);
            }
            if unit.sleep > 0 {
                skip_normal = true;
                events.push(Event::Unit(player_index, unit_index, UnitEvent::Sleep));
                debug!("Unit {} is sleep!", unit_index);
            }
        }
        // Normal action
        if !skip_normal {
            let unit = &mut player.units[unit_index];
            match unit.unit_type {
                UnitType::Attacker => {
                    if opponent.units.is_empty() {
                        let damage = (unit.attack - opponent.defence).max(0);
                        opponent.hp -= damage;
                        events.push(Event::Unit(
                            player_index,
                            unit_index,
                            UnitEvent::AttackPlayer(damage),
                        ));
                        debug!("Opponent take damage: {}", damage);
                        if opponent.hp <= 0 {
                            events.push(Event::Player(opponent_index, PlayerEvent::Die));
                            debug!("Player die");
                            break;
                        }
                    } else {
                        let target_index = 0;
                        let last = &mut opponent.units[0];
                        let damage = (unit.attack - last.defence).max(0);
                        last.hp -= damage;
                        events.push(Event::Unit(
                            player_index,
                            unit_index,
                            UnitEvent::AttackUnit(target_index, damage),
                        ));
                        debug!("Unit (last) take damage: {}", damage);
                        if last.hp <= 0 {
                            opponent.units.remove(target_index);
                            events.push(Event::Unit(opponent_index, target_index, UnitEvent::Die));
                            debug!("Unit (last) die");
                        }
                    }
                }
                UnitType::Healer => {
                    let attack = unit.attack;
                    let target_index = 0;
                    let u = &mut player.units[0];
                    u.hp += attack.max(0);
                    events.push(Event::Unit(
                        player_index,
                        unit_index,
                        UnitEvent::Heal(target_index, attack),
                    ));
                    debug!("Unit (last) heal: {}", attack);
                }
                UnitType::Archer => {
                    if opponent.units.is_empty() {
                        let damage = (unit.attack - opponent.defence).max(0);
                        opponent.hp -= damage;
                        events.push(Event::Unit(
                            player_index,
                            unit_index,
                            UnitEvent::AttackPlayer(damage),
                        ));
                        debug!("Opponent take damage: {}", damage);
                        if opponent.hp <= 0 {
                            events.push(Event::Player(opponent_index, PlayerEvent::Die));
                            debug!("Player die");
                            break;
                        }
                    } else {
                        let target_index = opponent.units.len() - 1;
                        let last = &mut opponent.units[target_index];
                        let damage = (unit.attack - last.defence).max(0);
                        last.hp -= damage;
                        events.push(Event::Unit(
                            player_index,
                            unit_index,
                            UnitEvent::AttackUnit(target_index, damage),
                        ));
                        debug!("Unit (last) take damage: {}", damage);
                        if last.hp <= 0 {
                            opponent.units.remove(target_index);
                            events.push(Event::Unit(opponent_index, target_index, UnitEvent::Die));
                            debug!("Unit (last) die");
                        }
                    }
                }
                UnitType::Assassin => {
                    if opponent.units.is_empty() {
                        let damage = (unit.attack - opponent.defence).max(0);
                        opponent.hp -= damage;
                        events.push(Event::Unit(
                            player_index,
                            unit_index,
                            UnitEvent::AttackPlayer(damage),
                        ));
                        debug!("Opponent take damage: {}", damage);
                        if opponent.hp <= 0 {
                            events.push(Event::Player(opponent_index, PlayerEvent::Die));
                            debug!("Player die");
                            break;
                        }
                    } else {
                        let target_index = {
                            let mut index = 0;
                            let mut min = opponent.units[0].hp;
                            for (i, u) in opponent.units.iter().enumerate() {
                                if u.hp < min {
                                    min = u.hp;
                                    index = i
                                }
                            }
                            index
                        };
                        let last = &mut opponent.units[target_index];
                        let damage = (unit.attack - last.defence).max(0);
                        last.hp -= damage;
                        events.push(Event::Unit(
                            player_index,
                            unit_index,
                            UnitEvent::AttackUnit(target_index, damage),
                        ));
                        debug!("Unit (last) take damage: {}", damage);
                        if last.hp <= 0 {
                            opponent.units.remove(target_index);
                            events.push(Event::Unit(opponent_index, target_index, UnitEvent::Die));
                            debug!("Unit (last) die");
                        }
                    }
                }
                UnitType::Summoner => {
                    let attack = unit.attack;
                    let u = Unit {
                        name: "SummonerCreature".to_string(),
                        hp: attack,
                        attack: attack,
                        defence: 0,
                        unit_type: UnitType::Attacker,
                        ..Default::default()
                    };
                    player.units.insert(0, u.clone());
                    events.push(Event::Unit(player_index, unit_index, UnitEvent::Summon(u)));
                    unit_index += 1;
                }
                UnitType::Mage => {
                    if opponent.units.is_empty() {
                        let damage = (unit.attack - opponent.defence).max(0);
                        opponent.hp -= damage;
                        events.push(Event::Unit(
                            player_index,
                            unit_index,
                            UnitEvent::AttackPlayer(damage),
                        ));
                        debug!("Opponent take damage: {}", damage);
                        if opponent.hp <= 0 {
                            events.push(Event::Player(opponent_index, PlayerEvent::Die));
                            debug!("Player die");
                            break;
                        }
                    } else {
                        let mut target_index = 0;
                        while target_index < opponent.units.len() {
                            let last = &mut opponent.units[target_index];
                            let damage = (unit.attack - last.defence).max(0);
                            last.hp -= damage;
                            events.push(Event::Unit(
                                player_index,
                                unit_index,
                                UnitEvent::AttackUnit(target_index, damage),
                            ));
                            debug!("Unit (last) take damage: {}", damage);
                            if last.hp <= 0 {
                                opponent.units.remove(target_index);
                                events.push(Event::Unit(
                                    opponent_index,
                                    target_index,
                                    UnitEvent::Die,
                                ));
                                debug!("Unit (last) die");
                            } else {
                                target_index += 1;
                            }
                        }
                    }
                }
            }
        }
        // End
        {
            let unit = &mut player.units[unit_index];
            if unit.stun > 0 {
                unit.stun -= 1;
            }
            if unit.sleep > 0 {
                unit.sleep -= 1;
            }
            if unit.charm > 0 {
                unit.charm -= 1;
            }
        }
        // Next unit
        unit_index += 1;
    }
    // Start turn for player
    // Draw card
    if player.decks.is_empty() && !player.graves.is_empty() {
        player.decks.append(&mut player.graves);
        player.decks.shuffle(&mut thread_rng());
        events.push(Event::Player(player_index, PlayerEvent::Shuffle));
    }
    if !player.decks.is_empty() {
        let card = player.decks.pop().unwrap();
        player.hands.push(card);
        events.push(Event::Player(player_index, PlayerEvent::DrawCard));
    }

    events
}

pub fn new_game() -> Game {
    let attacker = Card {
        unit: Unit {
            name: "Attacker".to_string(),
            hp: 100,
            attack: 20,
            defence: 0,
            unit_type: UnitType::Attacker,
            ..Default::default()
        },
    };
    let healer = Card {
        unit: Unit {
            name: "Healer".to_string(),
            hp: 50,
            attack: 20,
            defence: 0,
            unit_type: UnitType::Healer,
            ..Default::default()
        },
    };
    let archer = Card {
        unit: Unit {
            name: "Archer".to_string(),
            hp: 75,
            attack: 20,
            defence: 0,
            unit_type: UnitType::Archer,
            ..Default::default()
        },
    };
    let assassin = Card {
        unit: Unit {
            name: "Assassin".to_string(),
            hp: 25,
            attack: 40,
            defence: 0,
            unit_type: UnitType::Assassin,
            ..Default::default()
        },
    };
    let summoner = Card {
        unit: Unit {
            name: "Summoner".to_string(),
            hp: 50,
            attack: 10,
            defence: 0,
            unit_type: UnitType::Summoner,
            ..Default::default()
        },
    };
    let mage = Card {
        unit: Unit {
            name: "Mage".to_string(),
            hp: 25,
            attack: 5,
            defence: 0,
            unit_type: UnitType::Mage,
            ..Default::default()
        },
    };
    let player1 = Player {
        hands: vec![attacker.clone(), attacker.clone(), attacker.clone()],
        decks: vec![
            attacker.clone(),
            healer.clone(),
            archer.clone(),
            assassin.clone(),
            summoner.clone(),
            mage.clone(),
        ],
        hp: 100,
        ..Default::default()
    };
    let player2 = Player {
        hands: vec![attacker.clone(), attacker.clone(), attacker.clone()],
        decks: vec![
            attacker.clone(),
            healer.clone(),
            archer.clone(),
            assassin.clone(),
            summoner.clone(),
            mage.clone(),
        ],
        hp: 100,
        ..Default::default()
    };
    Game {
        player1: player1,
        player2: player2,
        current_player_index: 0,
        ..Default::default()
    }
}
