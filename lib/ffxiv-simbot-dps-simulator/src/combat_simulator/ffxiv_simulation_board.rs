use crate::combat_simulator::SimulationBoard;
use crate::simulation_result::{PartySimulationResult, SimulationResult};
use ffxiv_simbot_combat_components::consts::SIMULATION_START_TIME_MILLISECOND;
use ffxiv_simbot_combat_components::damage_calculator::raw_damage_calculator::{
    FfxivRawDamageCalculator, RawDamageCalculator,
};
use ffxiv_simbot_combat_components::damage_calculator::rdps_calculator::FfxivRdpsCalculator;
use ffxiv_simbot_combat_components::event::ffxiv_event::FfxivEvent;
use ffxiv_simbot_combat_components::event::FfxivEventQueue;
use ffxiv_simbot_combat_components::event_ticker::auto_attack_ticker::AutoAttackTicker;
use ffxiv_simbot_combat_components::event_ticker::ffxiv_event_ticker::FfxivEventTicker;
use ffxiv_simbot_combat_components::event_ticker::global_ticker::GlobalTicker;
use ffxiv_simbot_combat_components::event_ticker::{EventTicker, PercentType, TickerKey};
use ffxiv_simbot_combat_components::id_entity::IdEntity;
use ffxiv_simbot_combat_components::live_objects::player::ffxiv_player::FfxivPlayer;
use ffxiv_simbot_combat_components::live_objects::player::player_power::add_main_stat;
use ffxiv_simbot_combat_components::live_objects::player::role::job_abbrev_to_role;
use ffxiv_simbot_combat_components::live_objects::player::{Player, StatusKey};
use ffxiv_simbot_combat_components::live_objects::target::ffxiv_target::FfxivTarget;
use ffxiv_simbot_combat_components::live_objects::target::Target;
use ffxiv_simbot_combat_components::rotation::cooldown_timer::CooldownTimer;
use ffxiv_simbot_combat_components::skill::attack_skill::AttackSkill;
use ffxiv_simbot_combat_components::skill::damage_category::DamageCategory;
use ffxiv_simbot_combat_components::skill::AUTO_ATTACK_ID;
use ffxiv_simbot_combat_components::status::buff_status::BuffStatus;
use ffxiv_simbot_combat_components::status::debuff_status::DebuffStatus;
use ffxiv_simbot_combat_components::status::status_holder::StatusHolder;
use ffxiv_simbot_combat_components::status::status_info::StatusInfo;
use ffxiv_simbot_combat_components::status::status_timer::StatusTimer;
use ffxiv_simbot_combat_components::status::Status;
use ffxiv_simbot_combat_components::types::DamageType;
use ffxiv_simbot_combat_components::types::{IdType, TimeType};
use log::{debug, info};
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::rc::Rc;

static GLOBAL_TICKER_ID: IdType = 10000;

/// The main party combat simulation board for FFXIV. Think of this simulation of one instance of combat.
/// The DpsSimulator does the following:
/// Keeps track of the party and target data.
/// Keeps RDPS contribution for all buffs that are in the party.
pub struct FfxivSimulationBoard {
    raw_damage_calculator: FfxivRawDamageCalculator,
    rdps_calculator: FfxivRdpsCalculator,

    pub main_player_id: IdType,

    pub party: Vec<Rc<RefCell<FfxivPlayer>>>,
    pub target: Rc<RefCell<FfxivTarget>>,

    current_combat_time_millisecond: RefCell<TimeType>,
    pub(crate) finish_combat_time_millisecond: TimeType,
    pub(crate) tickers: RefCell<HashMap<TickerKey, FfxivEventTicker>>,
    pub event_queue: Rc<RefCell<FfxivEventQueue>>,
}

impl SimulationBoard<FfxivTarget, FfxivPlayer, AttackSkill> for FfxivSimulationBoard {
    fn run_simulation(&self) {
        loop {
            if self.combat_time_exceeded_finish_time(self.finish_combat_time_millisecond) {
                debug!("combat finished");
                break;
            }

            let next_event = self.event_queue.borrow_mut().pop().unwrap().0;
            self.simulate_event(next_event);
        }
    }

    fn create_simulation_result(&self) -> SimulationResult {
        let mut party_simulation_results = vec![];

        for player in self.party.clone() {
            party_simulation_results.push(PartySimulationResult {
                player_id: player.borrow().get_id(),
                job: player.borrow().job_abbrev.clone(),
                role: job_abbrev_to_role(&player.borrow().job_abbrev).to_string(),
                skill_log: player.borrow().skill_logs.clone(),
                damage_log: player.borrow().damage_logs.clone(),
            });
        }

        SimulationResult {
            main_player_id: self.main_player_id,
            combat_time_millisecond: *self.current_combat_time_millisecond.borrow(),
            party_simulation_results,
        }
    }
}

impl FfxivSimulationBoard {
    fn simulate_event(&self, ffxiv_event: FfxivEvent) {
        let event_time = ffxiv_event.get_event_time();
        self.update_time_related_informations(event_time);

        match &ffxiv_event {
            FfxivEvent::PlayerTurn(player_id, _, _, time) => {
                debug!("time: {}, player turn event: player {}", *time, *player_id);
                let player = self.get_player_data(*player_id);
                let debuffs = self.target.borrow().get_status_table();

                player.borrow_mut().handle_ffxiv_event(ffxiv_event, debuffs);
            }
            FfxivEvent::Damage(
                player_id,
                skill_id,
                potency,
                trait_percent,
                guaranteed_crit,
                guaranteed_dh,
                snapshotted_buffs,
                snapshotted_debuffs,
                damage_category,
                time,
            ) => {
                let buffs = snapshotted_buffs.clone();
                let debuffs = snapshotted_debuffs.clone();
                let player = self.get_player_data(*player_id).clone();

                info!(
                    "time: {}, damage event: {}",
                    *time,
                    player.borrow().print_skill_debug(*skill_id)
                );

                self.handle_damage_event(
                    player.clone(),
                    *skill_id,
                    *potency,
                    *trait_percent,
                    *guaranteed_crit,
                    *guaranteed_dh,
                    buffs,
                    debuffs,
                    *damage_category,
                    *time,
                );
            }
            FfxivEvent::ApplyBuff(_, target_id, status, _, _, time) => {
                debug!(
                    "time: {}, apply buff event: status id {}",
                    *time,
                    status.get_name().as_str(),
                );
                let player = self.get_player_data(*target_id);
                let debuffs = self.target.borrow().get_status_table();
                player.borrow_mut().handle_ffxiv_event(ffxiv_event, debuffs);
            }
            FfxivEvent::ApplyBuffStack(_, target_id, status, _, _, time) => {
                debug!(
                    "time: {}, apply buff stack event: status {}",
                    *time,
                    status.get_name().as_str()
                );
                let player = self.get_player_data(*target_id);
                let debuffs = self.target.borrow().get_status_table();
                player.borrow_mut().handle_ffxiv_event(ffxiv_event, debuffs);
            }
            FfxivEvent::ApplyRaidBuff(_, status, _, _, time) => {
                debug!(
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
            FfxivEvent::RefreshBuff(_, player_id, status, _, _, time) => {
                debug!(
                    "time: {}, refresh buff event: player id {}, status id {}",
                    *time,
                    *player_id,
                    status.get_id()
                );
                let player = self.get_player_data(*player_id);
                let debuffs = self.target.borrow().get_status_table();
                player.borrow_mut().handle_ffxiv_event(ffxiv_event, debuffs);
            }
            FfxivEvent::ApplyDebuff(_, status, _, _, time) => {
                debug!(
                    "time: {}, apply debuff event: status {}",
                    *time,
                    status.get_name().as_str()
                );
                let target = self.get_target();
                target.borrow_mut().handle_ffxiv_event(ffxiv_event);
            }
            FfxivEvent::ApplyDebuffStack(_, status, _, _, time) => {
                debug!(
                    "time: {}, apply debuff stack event: status {}",
                    *time,
                    status.get_name().as_str()
                );
                let target = self.get_target();
                target.borrow_mut().handle_ffxiv_event(ffxiv_event);
            }
            FfxivEvent::UseSkill(player_id, _, skill_id, time) => {
                let player = self.get_player_data(*player_id);
                let debuffs = self.target.borrow().get_status_table();

                info!(
                    "time: {}, use skill event: {}",
                    *time,
                    player.borrow().print_skill_debug(*skill_id)
                );

                player.borrow_mut().handle_ffxiv_event(ffxiv_event, debuffs);
            }
            FfxivEvent::RemoveTargetBuff(_, player_id, status_id, time) => {
                debug!(
                    "time: {}, remove target buff event: skill id {}",
                    *time, *status_id
                );
                let player = self.get_player_data(*player_id);
                let debuffs = self.target.borrow().get_status_table();
                player.borrow_mut().handle_ffxiv_event(ffxiv_event, debuffs);
            }
            FfxivEvent::RemoveRaidBuff(owner_player_id, status_id, time) => {
                debug!(
                    "time: {}, remove raid buff event: status id {}",
                    *time, *status_id
                );

                for player in self.party.clone() {
                    let single_event = FfxivEvent::RemoveTargetBuff(
                        *owner_player_id,
                        player.borrow().get_id(),
                        *status_id,
                        *time,
                    );
                    let debuffs = self.target.borrow().get_status_table();
                    player
                        .borrow_mut()
                        .handle_ffxiv_event(single_event.clone(), debuffs.clone());
                }
            }
            FfxivEvent::IncreasePlayerResource(player_id, resource_id, amount, time) => {
                debug!(
                    "time: {}, increase resource event: player: {}, resource id: {}, amount: {}",
                    *time, *player_id, *resource_id, *amount
                );
                let player = self.get_player_data(*player_id);
                let debuffs = self.target.borrow().get_status_table();
                player.borrow_mut().handle_ffxiv_event(ffxiv_event, debuffs);
            }
            FfxivEvent::RemoveDebuff(_, status_id, time) => {
                debug!(
                    "time: {}, remove debuff event: status id: {}",
                    *time, *status_id
                );
                let target = self.get_target();
                target.borrow_mut().handle_ffxiv_event(ffxiv_event);
            }
            FfxivEvent::Tick(ticker_key, time) => {
                debug!(
                    "time: {}, ticker event: ticker key: {:?}",
                    *time, *ticker_key
                );
                let mut ticker_table = self.tickers.borrow_mut();
                let ticker = ticker_table.get_mut(ticker_key);
                if ticker.is_none() {
                    return;
                }
                let ticker = ticker.unwrap();

                let player = if let Some(player_id) = ticker.get_player_id() {
                    Some(self.get_player_data(player_id).clone())
                } else {
                    None
                };

                let debuffs = self.target.borrow().get_status_table();
                ticker.run_ticker(*time, player, debuffs.clone());
            }
            FfxivEvent::AddTicker(ticker, time) => {
                debug!("time: {}, add ticker event: {:?}", *time, ticker.get_id());
                let mut ticker = ticker.clone();
                ticker.set_event_queue(self.event_queue.clone());
                let player = if let Some(player_id) = ticker.get_player_id() {
                    Some(self.get_player_data(player_id).clone())
                } else {
                    None
                };
                let debuffs = self.target.borrow().get_status_table();

                if ticker.has_initial_tick() {
                    ticker.run_ticker(*time, player, debuffs);
                } else {
                    ticker.run_ticker(*time + ticker.get_tick_interval(), player, debuffs);
                }
                self.register_ticker(ticker);
            }
            FfxivEvent::RemoveTicker(ticker_key, time) => {
                debug!("time: {}, remove ticker event: {:?}", *time, *ticker_key);
                self.tickers.borrow_mut().remove(ticker_key);
            }
            FfxivEvent::ForceTicker(ticker_key, time) => {
                debug!(
                    "time: {}, force ticker event: ticker id: {:?}",
                    *time, *ticker_key
                );

                if let Some(ticker) = self.tickers.borrow().get(&ticker_key) {
                    ticker.force_tick(*time);
                }
            }
            FfxivEvent::ReduceSkillCooldown(player_id, skill_id, _, time) => {
                debug!(
                    "reduce skill cooldown event: player_id {}, skill_id {}, {}",
                    *player_id, *skill_id, *time
                );
                let player = self.get_player_data(*player_id);
                let debuffs = self.target.borrow().get_status_table().clone();
                player.borrow_mut().handle_ffxiv_event(ffxiv_event, debuffs);
            }
            FfxivEvent::DotTick(time) => {
                debug!("time: {}, dot tick event", *time);
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
        trait_percent: PercentType,
        guaranteed_crit: bool,
        guaranteed_dh: bool,
        mut snapshotted_buffs: HashMap<StatusKey, BuffStatus>,
        mut snapshotted_debuffs: HashMap<StatusKey, DebuffStatus>,
        damage_category: DamageCategory,
        current_combat_time_millisecond: TimeType,
    ) {
        let player_id = player.borrow().get_id();
        let mut power = player.borrow().power.clone();

        for (_, buff) in snapshotted_buffs.iter() {
            buff.get_status_info()
                .iter()
                .for_each(|status_info| match status_info {
                    StatusInfo::IncreaseMainStat(maximum_increase, increase_percent) => {
                        power = add_main_stat(
                            &player.borrow().power,
                            *maximum_increase,
                            *increase_percent,
                        );
                    }
                    _ => {}
                })
        }

        snapshotted_buffs.retain(|_, buff| buff.is_damage_buff());
        snapshotted_debuffs.retain(|_, debuff| debuff.is_damage_debuff(player_id));

        let (damage_rdps_profile, is_crit) = self.raw_damage_calculator.calculate_total_damage(
            player_id,
            potency,
            damage_category,
            trait_percent,
            guaranteed_crit,
            guaranteed_dh,
            &snapshotted_buffs,
            &snapshotted_debuffs,
            &power,
        );

        if is_crit {
            player.borrow_mut().update_on_crit();
        }

        player.borrow_mut().update_damage_log(
            skill_id,
            &damage_rdps_profile,
            current_combat_time_millisecond,
        );
    }

    fn get_player_data(&self, player_id: IdType) -> Rc<RefCell<FfxivPlayer>> {
        self.party[player_id].clone()
    }

    fn get_target(&self) -> Rc<RefCell<FfxivTarget>> {
        self.target.clone()
    }

    fn update_time_related_informations(&self, next_event_time: TimeType) {
        let elapsed_time = self.get_elapsed_time(next_event_time);

        if elapsed_time == 0 {
            return;
        }

        self.update_player_target_time_informations(elapsed_time);
        self.update_combat_time(elapsed_time);
        self.update_ticker_time(elapsed_time);
    }

    fn update_ticker_time(&self, elapsed_time: TimeType) {
        let mut tickers = self.tickers.borrow_mut();
        let mut ticker_remaining_times = vec![];

        for ticker in tickers.values_mut() {
            ticker.update_remaining_time(elapsed_time);
            ticker_remaining_times.push((ticker.get_id(), ticker.get_remaining_time()));
        }

        for (ticker_id, remaining_time) in ticker_remaining_times {
            if remaining_time <= 0 {
                tickers.remove(&ticker_id);
            }
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

    pub fn new(
        main_player_id: IdType,
        target: Rc<RefCell<FfxivTarget>>,
        event_queue: Rc<RefCell<FfxivEventQueue>>,
        finish_combat_time_millisecond: TimeType,
    ) -> Self {
        let tickers: RefCell<HashMap<TickerKey, FfxivEventTicker>> =
            RefCell::new(Default::default());
        let mut global_ticker = FfxivEventTicker::GlobalTicker(GlobalTicker::new(
            GLOBAL_TICKER_ID,
            event_queue.clone(),
        ));
        global_ticker.run_ticker(0, None, Default::default());
        tickers
            .borrow_mut()
            .insert(global_ticker.get_id(), global_ticker);

        FfxivSimulationBoard {
            raw_damage_calculator: Default::default(),
            rdps_calculator: Default::default(),
            main_player_id,
            party: vec![],
            target,
            current_combat_time_millisecond: RefCell::new(SIMULATION_START_TIME_MILLISECOND),
            finish_combat_time_millisecond,
            event_queue: event_queue.clone(),
            tickers,
        }
    }

    pub fn register_player(&mut self, player: Rc<RefCell<FfxivPlayer>>) {
        self.event_queue
            .borrow_mut()
            .push(Reverse(player.borrow().start_turn.clone()));

        self.party.push(player.clone());

        if player.borrow().is_melee() {
            self.register_auto_attack_ticker(
                player.borrow().get_id(),
                &player.borrow().job_abbrev,
                self.event_queue.clone(),
            );
        }
    }

    fn register_auto_attack_ticker(
        &self,
        player_id: IdType,
        job_abbrev: &String,
        event_queue: Rc<RefCell<FfxivEventQueue>>,
    ) {
        let mut auto_attack_ticker =
            AutoAttackTicker::new(AUTO_ATTACK_ID, player_id, job_abbrev, event_queue);
        let player = self.get_player_data(player_id);

        auto_attack_ticker.run_ticker(0, Some(player.clone()), Default::default());
        self.register_ticker(FfxivEventTicker::AutoAttackTicker(auto_attack_ticker));
    }

    pub fn register_ticker(&self, ticker: FfxivEventTicker) {
        self.tickers.borrow_mut().insert(ticker.get_id(), ticker);
    }
}
