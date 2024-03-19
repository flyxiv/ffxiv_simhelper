use std::collections::HashMap;

pub type JobId = usize;

/// Crude JobClass Data Fetched from the database.
/// Gets converted to JobCLass, the engine's preferred type for the corresponding entity.
#[derive(PartialEq, Eq)]
pub struct JobClassDbData {
    class_id: usize,
    role_id: usize,
    main_stat_id: usize,
    role_name: String,
    job_name: String,
    armor_id: usize,
    accessory_id: usize,
}

/// Main Class representing info needed about each FFXIV's Job
/// Saves which main stat the job uses, which role(ex) Tank, Heealer) the job is in, and what
/// armor and accessory the job uses.
/// * role/main_stat/armor ids are stated in the following document: <TODO>
#[derive(PartialEq, Eq)]
pub struct JobClass {
    role_id: usize,
    main_stat_id: usize,
    role_name: String,
    job_name: String,
    armor_id: usize,
    accessory_id: usize,
}

impl JobClassDbData {
    pub(crate) fn new(
        class_id: usize,
        role_id: usize,
        main_stat_id: usize,
        role_name: String,
        job_name: String,
        armor_id: usize,
        accessory_id: usize,
    ) -> JobClassDbData {
        JobClassDbData {
            class_id,
            role_id,
            main_stat_id,
            role_name,
            job_name,
            armor_id,
            accessory_id,
        }
    }
}

pub(crate) fn make_jobclass_data_table(
    job_class_db_data: Vec<JobClassDbData>,
) -> HashMap<JobId, JobClass> {
    let mut jobclass_table = HashMap::<JobId, JobClass>::new();

    for jobclass_data in job_class_db_data {
        jobclass_table.insert(jobclass_data.class_id, JobClass::from(jobclass_data));
    }

    jobclass_table
}

impl From<JobClassDbData> for JobClass {
    fn from(data: JobClassDbData) -> Self {
        JobClass {
            role_id: data.role_id,
            main_stat_id: data.main_stat_id,
            role_name: data.role_name,
            job_name: data.job_name,
            armor_id: data.armor_id,
            accessory_id: data.accessory_id,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::database_manager::{make_database_mock, DatabaseManager};
    use crate::jobclass::{make_jobclass_data_table, JobClass, JobClassDbData};

    #[test]
    fn test_jobclass_default() {
        let mut jobclass = JobClass {
            role_id: 0,
            main_stat_id: 0,
            role_name: "".to_string(),
            job_name: "".to_string(),
            armor_id: 0,
            accessory_id: 0,
        };
        assert_eq!(jobclass.role_id, 0);
        assert_eq!(jobclass.main_stat_id, 0);
        assert_eq!(jobclass.role_name, "".to_string());
        assert_eq!(jobclass.job_name, "".to_string());
        assert_eq!(jobclass.armor_id, 0);
        assert_eq!(jobclass.accessory_id, 0);
    }

    #[test]
    fn test_jobclass_db() {
        let jobclass_db =
            JobClassDbData::new(1, 2, 3, "Tank".to_string(), "Paladin".to_string(), 4, 5);

        let jobclass = JobClass::from(jobclass_db);

        assert_eq!(jobclass.role_id, 2);
        assert_eq!(jobclass.main_stat_id, 3);
        assert_eq!(jobclass.role_name, "Tank".to_string());
        assert_eq!(jobclass.job_name, "Paladin".to_string());
        assert_eq!(jobclass.armor_id, 4);
        assert_eq!(jobclass.accessory_id, 5);
    }

    #[test]
    fn test_make_jobclass_table() {
        let mut database_manager = make_database_mock();
        let jobclasses = database_manager.get_jobclass().unwrap();
        let jobclass_table = make_jobclass_data_table(jobclasses);

        let paladin = jobclass_table.get(&0).unwrap();
        let white_mage = jobclass_table.get(&1).unwrap();
        let black_mage = jobclass_table.get(&2).unwrap();

        assert_eq!(paladin.job_name, "Paladin");
        assert_eq!(white_mage.job_name, "White Mage");
        assert_eq!(black_mage.job_name, "Black Mage");
    }
}
