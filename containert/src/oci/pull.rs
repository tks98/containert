use std::process::Command;
use std::io::Error;
use std::io::ErrorKind;
use std::fs;


struct Image {
    name: String,
    reference: String
}


// Copy the layers of the specified image to the local filesystem
// Essentially performs the example here up til runc
// https://manpages.ubuntu.com/manpages/jammy/man1/umoci-raw-unpack.1.html
pub fn pull_image(image_string: String) -> Result<Vec<u8>, Error> {

    println!("Pulling image: {}", image_string);
    let image = parse_image(image_string.to_owned())?;
   
    
    let image_name = format!("docker://{}", image_string);
    
    fs::create_dir_all("/var/lib/containert/")?;
    let image_dir = format!("oci:/var/lib/containert/{}:{}", image.name, image.reference);
    
    let output = Command::new("skopeo").arg("copy").arg(image_name).arg(image_dir).output()?;
    if !output.status.success() {
        // unpack the image into a rootfs
        return Ok(output.stderr);
    } else {
        return Ok(output.stdout)
    }
}


// Parses an image string into an Image struct
fn parse_image(image: String) -> Result<Image, Error> {
    let (name, reference) = image.split_once(':').ok_or(Error::new(ErrorKind::Other, "Could not parse image. Provide image in name:tag format"))?;
    return Ok(Image{name: name.to_string(), reference: reference.to_string()});
}
