mod mongo_db_configuration {
    use crate::data_pipeline::mongo::{configuration::MongoDbConfiguration, COLLECTIONS};

    #[test]
    fn collections() {
        let program = MongoDbConfiguration::new(COLLECTIONS);
        let collections = program.collections;
        for (i, record) in collections.iter().enumerate() {
            let index = program.choose_collection(Some(record.to_string()));
            assert_eq!(i, index);
        }
    }

    #[test]
    fn fs_config() {
        let program = MongoDbConfiguration::new(COLLECTIONS);
        let collection = String::from("test");
        let config = program.fs_config(collection.clone());

        let json_base_path = String::from("/.data/output/github/");
        let partial_json_data_dir = json_base_path + &collection + "/";
        assert!(config.json_data_dir.contains(&partial_json_data_dir));
    }
}
