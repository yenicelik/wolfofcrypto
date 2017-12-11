extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use serde_json::Value;
use serde_json::value::from_value;

const JSON: &str = include_str!("./test.json");

#[derive(Debug, Clone)]
struct DataElement {
    exch_id: u32,
    exch_name: String,
    exch_code: String,
    mkt_id: u32,
    mkt_name: String,
    exchmkt_id: u32
}

impl From<DataElementTemp> for DataElement {
    fn from(val: DataElementTemp) -> Self {
        DataElement {
            exch_id: val.exch_id.parse().unwrap(),
            exch_name: val.exch_name,
            exch_code: val.exch_code,
            mkt_id: val.mkt_id.parse().unwrap(),
            mkt_name: val.mkt_name,
            exchmkt_id: val.exchmkt_id.parse().unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataElementTemp {
    exch_id: String,
    exch_name: String,
    exch_code: String,
    mkt_id: String,
    mkt_name: String,
    exchmkt_id: String
}


#[derive(Deserialize, Serialize, Debug)]
struct Response(Vec<DataElementTemp>);

fn convert<A, B>(val: Vec<B>) -> Vec<A> where B: Into<A>, B: Clone {
    val.iter().map(|x| x.clone().into()).collect()
}

fn main() {
    let json: Value = serde_json::from_str(JSON).unwrap();
    let data = json.get("data").unwrap();
    let mut resp: Response = from_value(data.clone()).unwrap();
    let x: Vec<DataElement> = convert(resp.0);
    println!("{:?}", x);
}