use crate::equipment::EquipmentDbData;
use crate::jobclass::JobClassDbData;
use crate::race::RaceDbData;
use itertools::Itertools;
use mockall::automock;
use mysql::prelude::Queryable;
use mysql::*;
use serde::Deserialize;
use serde_yaml;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    ip: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

#[automock]
pub trait DatabaseManager {
    fn create(config: DatabaseConfig) -> Self;
    fn get_equipment(&mut self) -> Result<Vec<EquipmentDbData>>;
    fn get_races(&mut self) -> Result<Vec<RaceDbData>>;
    fn get_jobclass(&mut self) -> Result<Vec<JobClassDbData>>;
}

pub struct FfxivDatabaseManager {
    config: DatabaseConfig,
    pool: Option<Pool>,
    connection: Option<PooledConn>,
}

impl DatabaseConfig {
    pub fn load(file_path: &str) -> Result<Self> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(serde_yaml::from_str::<DatabaseConfig>(contents.as_str()).expect("couldn't get config"))
    }
}

impl FfxivDatabaseManager {
    fn make_database_opts(&self) -> OptsBuilder {
        OptsBuilder::new()
            .ip_or_hostname(Some(&self.config.ip))
            .tcp_port(self.config.port)
            .user(Some(&self.config.username))
            .pass(Some(&self.config.password))
            .db_name(Some(&self.config.database))
    }

    fn make_pool(&mut self) {
        self.pool = Some(Pool::new(self.make_database_opts()).expect("failed to make pool"));
    }

    fn make_connection(&mut self) {
        if let Some(pool) = &self.pool {
            let conn = pool.get_conn().expect("failed to get PooledConn");
            self.connection = Some(conn);
        }
    }
}

impl DatabaseManager for FfxivDatabaseManager {
    fn create(config: DatabaseConfig) -> Self {
        let mut manager = FfxivDatabaseManager {
            config,
            pool: None,
            connection: None,
        };

        manager.make_pool();
        manager.make_connection();
        manager
    }

    fn get_equipment(&mut self) -> Result<Vec<EquipmentDbData>> {
        if let Some(connection) = &mut self.connection {
            // there's more than 12 columns, so we have to manually collect them
            let query = "
                SELECT
                    eq.equipmentid, eq.equipmentslotid, eq.equipmentname, eq.equipmentclassid,
                    eq.itemlevel, eq.mainstat, eq.crit, eq.dh, eq.det, eq.gcdspeed, eq.tenacity, eq.piety,
                    eqc.mainstatid
                FROM Equipment eq INNER JOIN EquipmentClass eqc on eq.equipmentclassid = eqc.equipmentclassid";

            let equipments = connection
                .query_iter(query)
                .expect("cannot fetch query")
                .map(|row| {
                    let mut row_data = row.unwrap();

                    let eq_id = row_data.take("equipmentid").unwrap();
                    let slot = row_data.take("equipmentslotid").unwrap();
                    let name = row_data.take("equipmentname").unwrap();
                    let class_id = row_data.take("equipmentclassid").unwrap();
                    let ilvl = row_data.take("itemlevel").unwrap();
                    let main_stat = row_data.take("mainstat").unwrap();
                    let crit = row_data.take("crit").unwrap();
                    let dh = row_data.take("dh").unwrap();
                    let det = row_data.take("det").unwrap();
                    let speed = row_data.take("gcdspeed").unwrap();
                    let tenacity = row_data.take("tenacity").unwrap();
                    let piety = row_data.take("piety").unwrap();
                    let main_stat_id = row_data.take("mainstatid").unwrap();

                    EquipmentDbData::new(
                        eq_id,
                        slot,
                        name,
                        class_id,
                        None,
                        ilvl,
                        main_stat,
                        crit,
                        dh,
                        det,
                        speed,
                        tenacity,
                        piety,
                        main_stat_id,
                    )
                })
                .collect_vec();

            Ok(equipments)
        } else {
            Err(Error::from(std::io::Error::new(
                ErrorKind::Other,
                "no connection",
            )))
        }
    }

    fn get_races(&mut self) -> Result<Vec<RaceDbData>> {
        if let Some(connection) = &mut self.connection {
            let query = "
            SELECT
                raceid, racename, strength, intelligence, dexterity, mind
            FROM
                race
            ";

            connection.query_map(
                query,
                |(raceid, racename, strength, intelligence, dexterity, mind)| {
                    RaceDbData::new(raceid, racename, strength, intelligence, dexterity, mind)
                },
            )
        } else {
            Err(Error::from(std::io::Error::new(
                ErrorKind::Other,
                "no connection",
            )))
        }
    }

    fn get_jobclass(&mut self) -> Result<Vec<JobClassDbData>> {
        if let Some(connection) = &mut self.connection {
            let query = "
            SELECT
                j.classid, j.roleid, j.mainstatid, r.rolename, j.jobname, j.armorid, j.accessoryid
            FROM
                role r INNER JOIN job j ON r.roleid = j.roleid
            ";

            connection.query_map(
                query,
                |(class_id, role_id, main_stat_id, role_name, job_name, armor_id, accessory_id)| {
                    JobClassDbData::new(
                        class_id,
                        role_id,
                        main_stat_id,
                        role_name,
                        job_name,
                        armor_id,
                        accessory_id,
                    )
                },
            )
        } else {
            Err(Error::from(std::io::Error::new(
                ErrorKind::Other,
                "no connection",
            )))
        }
    }
}

pub(crate) fn make_database_mock() -> MockDatabaseManager {
    let mut mock = MockDatabaseManager::new();
    mock.expect_get_jobclass().returning(|| {
        Ok({
            vec![
                JobClassDbData::new(0, 1, 2, "Tank".to_string(), "Paladin".to_string(), 3, 4),
                JobClassDbData::new(
                    1,
                    2,
                    3,
                    "Healer".to_string(),
                    "White Mage".to_string(),
                    4,
                    5,
                ),
                JobClassDbData::new(2, 3, 4, "DPS".to_string(), "Black Mage".to_string(), 5, 6),
            ]
        })
    });

    mock.expect_get_equipment().returning(|| {
        Ok(vec![
            EquipmentDbData::new(
                0,
                1,
                "Iron Sword".to_string(),
                2,
                Some(3),
                4,
                5,
                Some(6),
                Some(7),
                Some(8),
                Some(9),
                Some(10),
                Some(11),
                12,
            ),
            EquipmentDbData::new(
                1,
                2,
                "Iron Shield".to_string(),
                3,
                Some(4),
                5,
                6,
                Some(7),
                Some(8),
                Some(9),
                Some(10),
                Some(11),
                Some(12),
                13,
            ),
        ])
    });

    mock.expect_get_races().returning(|| {
        Ok(vec![
            RaceDbData::new(0, "Hyur".to_string(), 0, 1, 2, 3),
            RaceDbData::new(1, "Elezen".to_string(), 4, 5, 6, 7),
            RaceDbData::new(2, "Lalafell".to_string(), 8, 9, 10, 11),
            RaceDbData::new(3, "Miqo'te".to_string(), 12, 13, 14, 15),
        ])
    });

    mock
}
