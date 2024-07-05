use std::{error::Error, io, fs::File, path::PathBuf, process};
use csv::{Reader, StringRecord};
use clap::Parser;
use simple_xml_builder::XMLElement;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Config {

    file: PathBuf,

    #[arg(short, long, value_name = "element")]
    element: String,

    #[arg(short, long, value_name = "root")]
    root: String,

    #[arg(short, long, value_name = "output")]
    output: PathBuf,
}

impl Config { 
    fn process(&self) -> Result<(), Box<dyn Error>> {
        let mut rdr = self.read_file()?; 
        let headers = rdr.headers()?.clone();
        let mut root_node = XMLElement::new(self.root.trim());

        for result in rdr.records() {
            let mut element_node = XMLElement::new(self.element.trim());
            let row = result?;
            let row_nodes = process_row(&headers, row);
            for node in row_nodes {
                element_node.add_child(node);
            }
            root_node.add_child(element_node);
        }
        let output_file = self.get_file()?;
        root_node.write(output_file)?;
        Ok(())
    }

    fn read_file(&self) -> Result<Reader<File>, io::Error> {
        let file = File::open(&self.file)?;
        Ok(csv::Reader::from_reader(file))
    }

    fn get_file(&self) -> Result<File, io::Error> {
        File::create(&self.output)
    }
}

fn process_row(headers: &StringRecord, row: StringRecord) -> Vec<XMLElement> {
    let iter = headers.into_iter().zip(row.into_iter());
    iter.map( |result| {
        let mut row_node = XMLElement::new(result.0.trim());
        row_node.add_text(result.1.trim());
        row_node
    }).collect::<Vec<XMLElement>>()
}

fn main() {
    let config = Config::parse();
    if let Err(err) = config.process() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
