mod artifact_configuration {
    use crate::data_pipeline::artifact::{configuration::ArtifactConfiguration, CONTEXTS};

    #[test]
    fn choose_context() {
        let program = ArtifactConfiguration::new(CONTEXTS);
        let contexts = program.contexts;
        for (i, record) in contexts.iter().enumerate() {
            let index = program.choose_context(Some(record.to_string()));
            assert_eq!(i, index);
        }
    }

    #[test]
    fn fs_config() {
        let program = ArtifactConfiguration::new(CONTEXTS);
        let collection = String::from("test");
        let config = program.fs_config(collection.clone());

        let json_base_path = String::from("./.data/output/github/");
        assert_eq!(
            config.json_collection_path,
            json_base_path + &collection + "/"
        );

        let partial_artifact_base_path = "/.data/artifact/github/";
        assert!(config
            .artifact_base_path
            .contains(partial_artifact_base_path));

        let artifact_file_name_prefix = String::from("github-");
        assert_eq!(
            config.artifact_file_name,
            artifact_file_name_prefix.clone() + &collection + ".tar.gz"
        );

        assert_eq!(
            config.encrypted_artifact_file_name,
            artifact_file_name_prefix + &collection + ".tar.gz.gpg"
        )
    }
}
