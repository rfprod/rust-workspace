mod data_pipeline_configuration {
    use crate::data_pipeline::{
        artifact::CONTEXTS, configuration::DataPipelineConfiguration, mongo::COLLECTIONS,
    };

    #[test]
    fn choose_context() {
        let program = DataPipelineConfiguration::new(CONTEXTS, COLLECTIONS);
        let contexts = program.contexts;
        for (i, record) in contexts.iter().enumerate() {
            let index = program.choose_context(Some(i.to_string()));
            assert_eq!(contexts.get(index), Some(record));
        }
    }

    #[test]
    fn choose_collection() {
        let program = DataPipelineConfiguration::new(CONTEXTS, COLLECTIONS);
        let collections = program.collections;
        for (i, record) in collections.iter().enumerate() {
            let index = program.choose_collection(Some(i.to_string()));
            assert_eq!(collections.get(index), Some(record));
        }
    }
}
