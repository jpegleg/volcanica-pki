use uuid::Uuid;
use chrono::prelude::*;
use std::{time, thread, env};
use std::process::Command;

fn main() {
  loop {
    let readi: DateTime<Utc> = Utc::now();
    let txid = Uuid::new_v4().to_string();
    env::set_var("txid", &txid);
    let output = Command::new("bash")
      .arg("-c")
      .arg("/usr/local/bin/lava_volcanica_sign_user ~/.ssh/dormant root ~/.ssh/peak.pub")
      .output()
      .expect("FATAL - Failed to execute command!");
    println!("{} {} - START: signing the public key with the user CA.", readi, txid);
    let fini: DateTime<Utc> = Utc::now();
    println!("{} {} - FINISH: returned {:?}", fini, txid, output);
    println!("{} {} - sleep for 23 hours", fini, txid);
    let millis = time::Duration::from_millis(82800000);
    thread::sleep(millis);
  }
}
