use crate::combat_simulator::{
    PlayerSimulationData, SimulationBoard, SIMULATION_START_TIME_MILLISECOND,
};
use crate::event_ticker::auto_attack_ticker::AutoAttackTicker;
use crate::event_ticker::global_ticker::GlobalTicker;
use crate::event_ticker::EventTicker;
use crate::simulation_result::RotationLog;
use ffxiv_simbot_combat_components::damage_calculator::raw_damage_calculator::{
    FfxivRawDamageCalculator, RawDamageCalculator,
};
use ffxiv_simbot_combat_components::damage_calculator::rdps_calculator::{
    FfxivRdpsCalculator, RdpsCalculator,
};
use ffxiv_simbot_combat_components::event::ffxiv_event::FfxivEvent;
use ffxiv_simbot_combat_components::event::FfxivEventQueue;
use ffxiv_simbot_combat_components::id_entity::IdEntity;
use ffxiv_simbot_combat_components::live_objects::player::ffxiv_player::FfxivPlayer;
use ffxiv_simbot_combat_components::live_objects::player::{Player, StatusKey};
use ffxiv_simbot_combat_components::live_objects::target::ffxiv_target::FfxivTarget;
use ffxiv_simbot_combat_components::live_objects::target::Target;
use ffxiv_simbot_combat_components::rotation::cooldown_timer::CooldownTimer;
use ffxiv_simbot_combat_components::skill::attack_skill::AttackSkill;
use ffxiv_simbot_combat_components::status::buff_status::BuffStatus;
use ffxiv_simbot_combat_components::status::debuff_status::DebuffStatus;
use ffxiv_simbot_combat_components::status::status_holder::StatusHolder;
use ffxiv_simbot_combat_components::status::status_timer::StatusTimer;
use ffxiv_simbot_combat_components::status::Status;
use ffxiv_simbot_combat_components::{DamageType, DpsType, IdType, StatusTable, TimeType};
use log::{debug, info};
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::rc::Rc;

static GLOBAL_TICKER_ID: IdType = 0;

/// The main party combat simulation board for FFXIV. Think of this simulation of one instance of combat.
/// The DpsSimulator does the following:
/// Keeps track of the party and target data.
/// Keeps RDPS contribution for all buffs that are in the party.
pub struct FfxivSimulationBoard {
    raw_damage_calculator: FfxivRawDamageCalculator,
    rdps_calculator: FfxivRdpsCalculator,

    pub(crate) rotation_logs: Rc<RefCell<HashMap<IdType, Vec<RotationLog>>>>,

    current_turn_player_id: RefCell<IdType>,
    pub party: Vec<Rc<RefCell<FfxivPlayer>>>,
    pub target: Rc<RefCell<FfxivTarget>>,

    current_combat_time_millisecond: RefCell<TimeType>,
    pub(crate) finish_combat_time_millisecond: TimeType,
    pub(crate) tickers: RefCell<HashMap<IdType, Box<dyn EventTicker>>>,
    pub(crate) ticker_cnt: RefCell<IdType>,
    pub event_queue: Rc<RefCell<FfxivEventQueue>>,
}

impl SimulationBoard<FfxivTarget, FfxivPlayer, AttackSkill> for FfxivSimulationBoard {
    fn run_simulation(&self) {
        loop {
            if self.combat_time_exceeded_finish_time(self.finish_combat_time_millisecond) {
                info!("combat finished");
                break;
            }

            let next_event = self.event_queue.borrow_mut().pop().unwrap().0;
            self.simulate_event(next_event);
        }
    }

    fn get_simulation_result(&self) -> HashMap<IdType, DpsType> {
        todo!()
    }
}

impl FfxivSimulationBoard {
    fn simulate_event(&self, mut ffxiv_event: FfxivEvent) {
        let event_time = ffxiv_event.get_event_time();
        self.update_time_related_informations(event_time);

        match &ffxiv_event {
            FfxivEvent::PlayerTurn(player_id, _, _, time) => {
                info!("time: {}, player turn event: player {}", *time, *player_id);
                let player = self.get_player_data(*player_id);
                let debuffs = self.target.borrow().get_status_table();

                player.borrow_mut().handle_ffxiv_event(ffxiv_event, debuffs);
            }
            FfxivEvent::Damage(
                player_id,
                skill_id,
                potency,
                guaranteed_crit,
                guaranteed_dh,
                snapshotted_buffs,
                snapshotted_debuffs,
                time,
            ) => {
                info!("time: {}, damage event: skill id {}", *time, *skill_id);
                let buffs = snapshotted_buffs.clone();
                let debuffs = snapshotted_debuffs.clone();
                let player = self.get_player_data(*player_id);

                self.handle_damage_event(
                    player.clone(),
                    *skill_id,
                    *potency,
                    *guaranteed_crit,
                    *guaranteed_dh,
                    buffs,
                    debuffs,
                );
            }
            FfxivEvent::ApplyBuff(_, target_id, status, _, _, time) => {
                info!(
                    "time: {}, apply buff event: status id {}",
                    *time,
                    status.get_name().as_str(),
                );
                let player = self.get_player_data(*target_id);
                let debuffs = self.target.borrow().get_status_table();
                player.borrow_mut().handle_ffxiv_event(ffxiv_event, debuffs);
            }
            FfxivEvent::ApplyBuffStack(_, target_id, status, _, _, time) => {
                info!(
                    "time: {}, apply buff stack event: status {}",
                    *time,
                    status.get_name().as_str()
                );
                let player = self.get_player_data(*target_id);
                let debuffs = self.target.borrow().get_status_table();
                player.borrow_mut().handle_ffxiv_event(ffxiv_event, debuffs);
            }
            FfxivEvent::ApplyRaidBuff(_, status, _, _, time) => {
                info!(
                    "time: {}, : raid buff event: status {}",
                    *time,
                    status.get_name().as_str()
                );
                let debuffs = self.target.borrow().get_status_table();

                for player in self.party.clone() {
                    player
                        .borrow_mut()
                        .handle_ffxiv_event(ffxiv_event.clone(), debuffs.clone());
                }
            }
            FfxivEvent::ApplyDebuff(_, status, _, _, time) => {
                info!(
                    "time: {}, apply debuff event: status {}",
                    *time,
                    status.get_name().as_str()
                );
                let target = self.get_target();
                target.borrow_mut().handle_ffxiv_event(ffxiv_event);
            }
            FfxivEvent::ApplyDebuffStack(_, status, _, _, time) => {
                info!(
                    "time: {}, apply debuff stack event: status {}",
                    *time,
                    status.get_name().as_str()
                );
                let target = self.get_target();
                target.borrow_mut().handle_ffxiv_event(ffxiv_event);
            }
            FfxivEvent::UseSkill(player_id, skill_id, time) => {
                info!("time: {}, use skill event: skill id {}", *time, *skill_id);
                let player = self.get_player_data(*player_id);
                let debuffs = self.target.borrow().get_status_table();
                player.borrow_mut().handle_ffxiv_event(ffxiv_event, debuffs);
            }
            FfxivEvent::RemoveTargetBuff(_, player_id, status_id, time) => {
                info!(
                    "time: {}, remove target buff event: skill id {}",
                    *time, *status_id
                );
                let player = self.get_player_data(*player_id);
                let debuffs = self.target.borrow().get_status_table();
                player.borrow_mut().handle_ffxiv_event(ffxiv_event, debuffs);
            }
            FfxivEvent::IncreasePlayerResource(player_id, resource_id, amount, time) => {
                info!(
                    "time: {}, increase resource event: player: {}, resource id: {}, amount: {}",
                    *time, *player_id, *resource_id, *amount
                );
                let player = self.get_player_data(*player_id);
                let debuffs = self.target.borrow().get_status_table();
                player.borrow_mut().handle_ffxiv_event(ffxiv_event, debuffs);
            }
            FfxivEvent::RemoveDebuff(_, status_id, time) => {
                info!(
                    "time: {}, remove debuff event: status id: {}",
                    *time, *status_id
                );
                let target = self.get_target();
                target.borrow_mut().handle_ffxiv_event(ffxiv_event);
            }
            FfxivEvent::Tick(ticker_id, time) => {
                info!("time: {}, ticker event: ticker id: {}", *time, *ticker_id);
                let player = if let Some(player_id) = self
                    .tickers
                    .borrow_mut()
                    .get_mut(ticker_id)
                    .unwrap()
                    .get_player_id()
                {
                    Some(self.get_player_data(player_id).clone())
                } else {
                    None
                };

                let debuffs = self.target.borrow().get_status_table();
                self.tickers
                    .borrow_mut()
                    .get_mut(ticker_id)
                    .unwrap()
                    .run_ticker(*time, player, debuffs.clone());
            }
            FfxivEvent::DotTick(time) => {
                info!("time: {}, dot tick event", *time);
                let target = self.get_target();
                target.borrow_mut().handle_ffxiv_event(ffxiv_event);
            }
        }
    }

    #[inline]
    fn combat_time_exceeded_finish_time(&self, target_combat_time_millisecond: TimeType) -> bool {
        *self.current_combat_time_millisecond.borrow() >= target_combat_time_millisecond
    }

    fn handle_damage_event(
        &self,
        player: Rc<RefCell<FfxivPlayer>>,
        skill_id: IdType,
        potency: DamageType,
        guaranteed_crit: bool,
        guaranteed_dh: bool,
        snapshotted_buffs: HashMap<StatusKey, BuffStatus>,
        snapshotted_debuffs: HashMap<StatusKey, DebuffStatus>,
    ) {
        let raw_damage = self.raw_damage_calculator.calculate_raw_damage(
            potency,
            guaranteed_crit,
            guaranteed_dh,
            &player.borrow().power,
        );

        let damage_rdps_profile = self.rdps_calculator.make_damage_profile(
            skill_id,
            snapshotted_buffs.clone(),
            snapshotted_debuffs.clone(),
            raw_damage,
            &player.borrow().power,
            player.borrow().get_id(),
        );

        player
            .borrow_mut()
            .update_skill_damage_table(skill_id, &damage_rdps_profile);
    }

    fn get_player_data(&self, player_id: IdType) -> Rc<RefCell<FfxivPlayer>> {
        self.party[player_id].clone()
    }

    fn get_target(&self) -> Rc<RefCell<FfxivTarget>> {
        self.target.clone()
    }

    fn add_skill_to_rotation_log(&self, skill_id: IdType) {
        let rotation_log = RotationLog {
            casted_time_millisecond: *self.current_combat_time_millisecond.borrow(),
            skill_id,
        };

        if self
            .rotation_logs
            .borrow()
            .contains_key(&self.current_turn_player_id.borrow())
        {
            self.rotation_logs
                .borrow_mut()
                .get_mut(&self.current_turn_player_id.borrow())
                .unwrap()
                .push(rotation_log);
        } else {
            self.rotation_logs
                .borrow_mut()
                .insert(*self.current_turn_player_id.borrow(), vec![rotation_log]);
        }
    }

    fn update_time_related_informations(&self, next_event_time: TimeType) {
        let elapsed_time = self.get_elapsed_time(next_event_time);

        self.update_player_target_time_informations(elapsed_time);
        self.update_combat_time(elapsed_time);
        self.update_ticker_time(elapsed_time);
    }

    fn update_ticker_time(&self, elapsed_time: TimeType) {
        let mut tickers = self.tickers.borrow_mut();
        for ticker in tickers.values_mut() {
            ticker.update_remaining_time(elapsed_time);
        }
    }

    #[inline]
    fn update_combat_time(&self, elapsed_time: TimeType) {
        *self.current_combat_time_millisecond.borrow_mut() += elapsed_time;
    }

    #[inline]
    fn get_elapsed_time(&self, next_event_time_millisecond: TimeType) -> TimeType {
        next_event_time_millisecond - *self.current_combat_time_millisecond.borrow()
    }

    fn update_player_target_time_informations(&self, elapsed_time: TimeType) {
        for player in self.party.clone() {
            player.borrow_mut().update_status_time(elapsed_time);
            player.borrow_mut().update_cooldown(elapsed_time);
        }

        self.target.borrow_mut().update_status_time(elapsed_time);
    }

    fn get_final_rotation_log(&self) -> Vec<PlayerSimulationData<FfxivPlayer, AttackSkill>> {
        todo!()
    }

    fn get_final_rdps_table(&self) -> HashMap<IdType, DpsType> {
        todo!()
    }
    pub fn new(
        target: Rc<RefCell<FfxivTarget>>,
        event_queue: Rc<RefCell<FfxivEventQueue>>,
        finish_combat_time_millisecond: TimeType,
    ) -> Self {
        let tickers: RefCell<HashMap<IdType, Box<dyn EventTicker>>> =
            RefCell::new(Default::default());
        let mut global_ticker = Box::new(GlobalTicker::new(GLOBAL_TICKER_ID, event_queue.clone()));
        global_ticker.run_ticker(0, None, Default::default());
        tickers.borrow_mut().insert(GLOBAL_TICKER_ID, global_ticker);

        FfxivSimulationBoard {
            raw_damage_calculator: Default::default(),
            rdps_calculator: Default::default(),
            rotation_logs: Rc::new(RefCell::new(HashMap::new())),
            current_turn_player_id: RefCell::new(0),
            party: vec![],
            target,
            current_combat_time_millisecond: RefCell::new(SIMULATION_START_TIME_MILLISECOND),
            finish_combat_time_millisecond,
            event_queue: event_queue.clone(),
            tickers,
            ticker_cnt: RefCell::new(1),
        }
    }

    pub fn register_player(&mut self, player: Rc<RefCell<FfxivPlayer>>) {
        self.event_queue
            .borrow_mut()
            .push(Reverse(player.borrow().start_turn.clone()));

        self.party.push(player.clone());

        if player.borrow().is_melee() {
            self.register_auto_attack_ticker(player.borrow().get_id(), self.event_queue.clone());
        }
    }

    fn register_auto_attack_ticker(
        &self,
        player_id: IdType,
        event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) {
        let id = *self.ticker_cnt.borrow();
        let mut auto_attack_ticker = AutoAttackTicker::new(id, player_id, event_queue);
        let player = self.get_player_data(player_id);

        auto_attack_ticker.run_ticker(0, Some(player.clone()), Default::default());
        self.register_ticker(id, Box::new(auto_attack_ticker));
    }

    pub fn register_ticker(&self, id: IdType, ticker: Box<dyn EventTicker>) {
        self.ticker_cnt.replace(id + 1);
        self.tickers.borrow_mut().insert(id, ticker);
    }
}
