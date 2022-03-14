use transaction_log::TransactionLog;

fn main() {
    let mut transaction_log = TransactionLog::new_empty();
    for i in 1..1048576 {
        transaction_log.append(i.to_string());
    }
    // println!("{:?}", transaction_log.head);
}
