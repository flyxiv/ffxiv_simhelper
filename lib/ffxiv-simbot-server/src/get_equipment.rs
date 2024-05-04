use axum::extract::{Query, State};
use axum::Json;
use ffxiv_simbot_db::equipment::{Equipment, EquipmentKey};
use ffxiv_simbot_engine::engine::Engine;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct EquipmentApiParameters {
    job_abbrev: String,
}

pub async fn get_equipment_handler(
    State(engine): State<Engine>,
    Query(params): Query<EquipmentApiParameters>,
) -> Json<HashMap<usize, Vec<Equipment>>> {
    let job_abbrev = params.job_abbrev;

    let mut equipments = HashMap::new();

    for slot_id in 0..14 {
        let equipment_key = EquipmentKey {
            job_id: job_abbrev.clone(),
            slot_id,
        };

        let slot_equipments = engine.context.equipments.get(&equipment_key);

        if let Some(slot_equipments2) = slot_equipments {
            equipments.insert(slot_id, slot_equipments2.clone());
        }
    }

    Json(equipments)
}
