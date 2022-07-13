use std::process::Command;
use std::io::Error;
use std::fs;


// Copy the layers of the specified image to the local filesystem
pub fn pull_image(image: String) -> Result<Vec<u8>, Error> {

    println!("Pulling image: {}", image);
    let image_name = format!("docker://{}", image);
    let dir = format!("/var/lib/containert/{}", image);
    fs::create_dir_all(dir).expect("failed creating image directory");
    let image_dir = format!("dir:/var/lib/containert/{}", image);
    
    let output = Command::new("skopeo").arg("copy").arg(image_name).arg(image_dir).output()?;
    if !output.status.success() {
        return Ok(output.stderr);
    } else {
        return Ok(output.stdout)
    }
}

