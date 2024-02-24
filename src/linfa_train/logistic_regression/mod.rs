//! Logistic regression module.

use ciborium::{cbor, value};
use colored::Colorize;
use csv::Reader;
use linfa::prelude::*;
use linfa::Dataset;
use linfa_logistic::FittedLogisticRegression;
use linfa_logistic::LogisticRegression;
use ndarray::Axis;
use ndarray::{Array, Array1, Array2};
use plotters::prelude::*;
use std::io::Read;
use std::path::Path;
use std::{env::args, fs, fs::File};

/// The entry point of the program.
pub fn main() {
    LinfaTrainLogisticRegression::new();
}

/// Input arguments of the program.
struct InuputArguments {
    max_iterations: u64,
}

struct LinfaTrainLogisticRegression;

/// Source: https:///github.com/DataPsycho/data-pipelines-in-rust/blob/main/diabetes_ml_pipeline/Cargo.toml
impl LinfaTrainLogisticRegression {
    /// Program constructor.
    fn new() -> LinfaTrainLogisticRegression {
        let mut program = LinfaTrainLogisticRegression;
        program.init();
        program
    }

    /// Initializes the program.
    fn init(&mut self) {
        println!("\n{}", "Linfa train initialized.".blue().bold());

        let args = self.args();

        self.train(args.max_iterations);
        self.load_model();
        self.generate_plots();
    }

    /// Parses arguments passed to the program.
    fn args(&mut self) -> InuputArguments {
        let arguments: Vec<String> = args().collect();

        println!("\n{}:\n{:?}", "Arguments".cyan().bold(), arguments);

        let max_iterations = arguments
            .get(2)
            .cloned()
            .unwrap_or_default()
            .trim()
            .parse::<u64>()
            .unwrap_or(500);

        InuputArguments { max_iterations }
    }

    /// The dataset headers.
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

    /// The dataset data.
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

    /// The dataset records.
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

    /// The dataset targets.
    fn targets(&mut self, data: &[Vec<f32>], target_index: usize) -> Array1<i32> {
        let targets = data
            .iter()
            .map(|r| r[target_index] as i32)
            .collect::<Vec<i32>>();
        println!(
            "\n{} {:?}",
            "Target collected, length:".yellow(),
            targets.len()
        );
        Array::from(targets)
    }

    /// The dataset.
    /// Data source: https:///github.com/plotly/datasets/blob/master/diabetes.csv
    fn dataset(&mut self) -> Dataset<f32, i32, ndarray::Dim<[usize; 1]>> {
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

    /// Trains the model.
    fn train(&mut self, max_iterations: u64) {
        println!("\n{}", "Training the model...".yellow().bold());
        let dataset = self.dataset();
        let model = LogisticRegression::default()
            .max_iterations(max_iterations)
            .gradient_tolerance(0.0001)
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
            .join("diabetes_model.logistic_regression.cbor");
        fs::write(output.clone(), vec_model).unwrap();
        println!("\n{} {:?}", "Model saved, path:".yellow(), output.as_path());
    }

    /// Loads the model.
    fn load_model(&mut self) {
        println!("\n{}", "Testing the model...".yellow().bold());
        let dataset = self.dataset();
        let mut data: Vec<u8> = Vec::new();
        let path = Path::new(".data")
            .join("output")
            .join("diabetes_model.logistic_regression.cbor");
        let mut file = File::open(path).unwrap();
        file.read_to_end(&mut data).unwrap();
        let value = ciborium::de::from_reader::<value::Value, _>(&data[..]).unwrap();
        let model: FittedLogisticRegression<f32, i32> = value.deserialized().unwrap();
        println!("\n{} {:?}", "Model loaded:".yellow(), model);
        let prediction = model.predict(dataset.records);
        println!(
            "\n{} {:?}",
            "Prediction test with the model success:".green().bold(),
            prediction
        );
    }

    /// Generates plots (2D Cartesian coordinate system).
    fn generate_plots(&mut self) {
        let dataset = self.dataset();
        let records = dataset.records().to_owned();
        let length = records.index_axis(Axis(0), 0).len();
        let x_axis_column_index = length - 1;
        for index in 0..x_axis_column_index {
            let y_axis_column_index = index;
            match self.plot(dataset.clone(), x_axis_column_index, y_axis_column_index) {
                Ok(()) => {
                    println!("Generated a plot {:?}", index);
                }
                Err(error) => {
                    panic!("Plot error\n{:?}", error.source());
                }
            }
        }
    }

    /// Generates a plot (2D Cartesian coordinate system).
    fn plot(
        &mut self,
        dataset: Dataset<f32, i32, ndarray::Dim<[usize; 1]>>,
        x_axis_column_index: usize,
        y_axis_column_index: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let records = dataset.records().to_owned();
        let x_axis = records.column(x_axis_column_index);
        let x_values = x_axis.to_owned().to_vec();
        let x_range = x_values
            .clone()
            .into_iter()
            .reduce(f32::min)
            .unwrap()
            .to_owned()
            ..x_values
                .clone()
                .into_iter()
                .reduce(f32::max)
                .unwrap()
                .to_owned();
        let y_axis = records.column(y_axis_column_index);
        let y_values = y_axis.to_owned().to_vec();
        let y_range = y_values
            .clone()
            .into_iter()
            .reduce(f32::min)
            .unwrap()
            .to_owned()
            ..y_values
                .clone()
                .into_iter()
                .reduce(f32::max)
                .unwrap()
                .to_owned();

        let features = dataset.feature_names();
        let x_feature_default = String::from("x");
        let x_feature = features
            .get(x_axis_column_index)
            .unwrap_or(&x_feature_default);
        let y_feature_default = String::from("y");
        let y_feature = features
            .get(y_axis_column_index)
            .unwrap_or(&y_feature_default);
        let caption = y_feature.to_owned() + " / " + x_feature;

        let file_name =
            y_feature.to_owned().to_lowercase() + "-by-" + x_feature.to_lowercase().as_str();
        let plot_path = Path::new(".data")
            .join("plots")
            .join(file_name + "_diabetes_model.png");
        let root = BitMapBackend::new(&plot_path, (1600, 1200)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(caption, ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_range, y_range)?;

        let binding = x_values.to_owned().to_vec();
        let plot_data = binding.iter().enumerate().map(|(index, value)| {
            (
                value.to_owned(),
                y_values.get(index).unwrap_or(&0.0).to_owned(),
            )
        });

        chart.configure_mesh().draw()?;

        chart.draw_series(PointSeries::of_element(plot_data, 2, &RED, &|c, s, st| {
            EmptyElement::at(c) + Circle::new((0, 0), s, st.filled())
        }))?;

        root.present()?;

        Ok(())
    }
}
