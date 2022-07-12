use std::process::Command;
use std::io::Error;

pub fn pull_image(image_dir: String, image_name: String) -> Result<Vec<u8>, Error> {

    // Run skopeo copy and download the image locally 
    let output = Command::new("skopeo").arg("copy").arg(image_name).arg(image_dir).output()?; // ? means to bubble up error to caller and return from function
    if !output.status.success() {
        return Ok(output.stderr);
    } else {
        return Ok(output.stdout)
    }
}