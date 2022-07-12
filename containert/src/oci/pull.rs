use std::process::Command;

pub fn pull_image(image_dir: String, image_name: String) -> String {
    let output = Command::new("skopeo").arg("copy").arg(image_name).arg(image_dir).output().expect("failed to execute process");
    if !output.status.success() {
        println!("{}", "Command executed with failing error code");
    }
    let output_string = String::from_utf8(output.stdout).ok();
    return output_string.unwrap();
}