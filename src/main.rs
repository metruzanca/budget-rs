use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Source {
    name: String,
    amount: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Budget {
    income: Vector<Source>,
    expenses: Vector<Source>,
}

fn main() {
    let file = File::open("./config/budget.json").unwrap();
    dbg!(file);
}

/*
TODO
- [ ] Configurable paths
- [ ] toml schema
*/