
use fs;

use pest::Parser;
use pest_derive::Parser;


#[derive(Parser)]
#[grammar = "hana.pest"]
pub struct HanaParser;

fn main() {

    let unparsed_file = fs::read_to_string("src/test.hana");

    let file = HanaParser::parse(Rule::file, &unparsed_file)
        .next().unwrap();
   
    println!("{:?}", file);
}
