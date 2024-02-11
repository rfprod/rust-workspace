//! Logistic regression module.

use ciborium::{cbor, value};
use colored::Colorize;
use csv::Reader;
use linfa::prelude::*;
use linfa::Dataset;
use linfa_trees::DecisionTree;
use ndarray::{Array, Array1, Array2};
use std::io::Read;
use std::path::Path;
use std::{env::args, fs, fs::File};

/// The entry point of the program.
pub fn main() {
    LinfaTrainDecisionTree::new();
}

/// Input arguments of the program.
struct InuputArguments {
    max_depth: usize,
}

struct LinfaTrainDecisionTree;

/// Source: https:///github.com/DataPsycho/data-pipelines-in-rust/blob/main/diabetes_ml_pipeline/Cargo.toml
impl LinfaTrainDecisionTree {
    /// Program constructor.
    fn new() -> LinfaTrainDecisionTree {
        let mut program = LinfaTrainDecisionTree;
        program.init();
        program
    }

    /// Initializes the program.
    fn init(&mut self) {
        println!("\n{}", "Linfa train initialized.".blue().bold());

        let args = self.args();

        self.train(args.max_depth);
        self.load_model();
    }

    /// Parses arguments passed to the program.
    fn args(&mut self) -> InuputArguments {
        let arguments: Vec<String> = args().collect();

        println!("\n{}:\n{:?}", "Arguments".cyan().bold(), arguments);

        let max_depth = arguments
            .get(2)
            .cloned()
            .unwrap_or_default()
            .trim()
            .parse::<usize>()
            .unwrap_or(500);

        InuputArguments { max_depth }
    }

    /// The dataset headers
    fn headers(&mut self, reader: &mut Reader<File>) -> Vec<String> {
        let result = reader
            .headers()
            .unwrap()
            .iter()
            .map(|r| r.to_owned())
            .collect();
        println!("\n{} {:?}", "Header collected, result:".yellow(), result);
        result
    }

    /// The dataset data
    fn data(&mut self, reader: &mut Reader<File>) -> Vec<Vec<f32>> {
        let result = reader
            .records()
            .map(|r| {
                r.unwrap()
                    .iter()
                    .map(|field| field.parse::<f32>().unwrap())
                    .collect::<Vec<f32>>()
            })
            .collect::<Vec<Vec<f32>>>();
        println!(
            "\n{} {:?}",
            "Data collected, length:".yellow(),
            result.len()
        );
        result
    }

    /// The dataset records
    fn records(&mut self, data: &[Vec<f32>], target_index: usize) -> Array2<f32> {
        let mut records: Vec<f32> = vec![];
        for record in data.iter() {
            records.extend_from_slice(&record[0..target_index]);
        }

        let result = Array::from(records)
            .into_shape((data.len(), target_index))
            .unwrap();
        let record_shape = result.shape();
        println!(
            "\n{} {:?} x {:?}",
            "Records collected, shape:".yellow(),
            record_shape[0],
            record_shape[1]
        );
        result
    }

    /// The dataset targets
    fn targets(&mut self, data: &[Vec<f32>], target_index: usize) -> Array1<usize> {
        let targets = data
            .iter()
            .map(|r| r[target_index] as usize)
            .collect::<Vec<usize>>();
        println!(
            "\n{} {:?}",
            "Target collected, length:".yellow(),
            targets.len()
        );
        Array::from(targets)
    }

    /// The dataset
    /// Data source: https:///github.com/plotly/datasets/blob/master/diabetes.csv
    fn dataset(&mut self) -> Dataset<f32, usize, ndarray::Dim<[usize; 1]>> {
        let file_path = ".data/input/diabetes.csv";
        let mut reader = Reader::from_path(file_path).unwrap();
        let headers = self.headers(&mut reader);
        let data = self.data(&mut reader);
        let target_index = headers.len() - 1;
        let features = headers[0..target_index].to_vec();
        let records = self.records(&data, target_index);
        let targets = self.targets(&data, target_index);
        Dataset::new(records, targets).with_feature_names(features)
    }

    /// Trains the model
    fn train(&mut self, max_depth: usize) {
        println!("\n{}", "Training the model...".yellow().bold());
        let dataset = self.dataset();
        let model = DecisionTree::params()
            .max_depth(Some(max_depth))
            .fit(&dataset)
            .expect("Can not train the model");
        let value = cbor!(model).unwrap();
        let mut vec_model = Vec::new();
        ciborium::ser::into_writer(&value, &mut vec_model).unwrap();
        // debug: start
        let prediction = model.predict(&dataset.records);
        println!("{:?}", prediction);
        // debug: end
        let output = Path::new(".data")
            .join("output")
            .join("diabetes_model.decision_tree.cbor");
        fs::write(output.clone(), vec_model).unwrap();
        println!("\n{} {:?}", "Model saved, path:".yellow(), output.as_path());
    }

    /// Loads the model
    fn load_model(&mut self) {
        println!("\n{}", "Testing the model...".yellow().bold());
        let dataset = self.dataset();
        let mut data: Vec<u8> = Vec::new();
        let path = Path::new(".data")
            .join("output")
            .join("diabetes_model.decision_tree.cbor");
        let mut file = File::open(path).unwrap();
        file.read_to_end(&mut data).unwrap();
        let value = ciborium::de::from_reader::<value::Value, _>(&data[..]).unwrap();
        let model: DecisionTree<f32, usize> = value.deserialized().unwrap();
        println!("\n{} {:?}", "Model loaded:".yellow(), model);
        let prediction = model.predict(dataset.records);
        println!(
            "\n{} {:?}",
            "Prediction test with the model success:".green().bold(),
            prediction
        );
    }
}
