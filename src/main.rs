use std::path::Path;

use accounting::domain::account::account::Account;
use accounting::model::account_model::read_default_chart;

fn main() {
    let chart =
        read_default_chart(Path::new("COA.json")).expect("Failed to read chart of accounts");
    let chart = chart
        .into_iter()
        .map(|account| Account::new(account.r#ref, account.name))
        .collect::<Vec<_>>();
    println!("File content: {:?}", chart);
}
