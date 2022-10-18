use std::process::Command;

fn main() {
  println!("cargo:rerun-if-changed=../config/nidaqmx-test-config.ini");

  let _nidaqconfig = Command::new("nidaqmxconfig")
    .args([
      "--import",
      "/home/virt/trebuchet/config/nidaqmx-test-config.ini",
    ])
    .output()
    .expect("failed to process devices!");
}
