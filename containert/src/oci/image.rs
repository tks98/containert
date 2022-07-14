use std::process::Command;
use std::io::Error;
use std::io::ErrorKind;
use std::fs;

use crate::image;

#[derive(Clone, Debug)]
struct Image {
    name: String,
    reference: String,
    dir: String,
    rootfs: String,
    uri: String
}

// Copy the layers of the specified image to the local filesystem
// Essentially performs the example here up til runc
// https://manpages.ubuntu.com/manpages/jammy/man1/umoci-raw-unpack.1.html
pub fn pull_image(image_string: String) -> Result<Vec<u8>, Error> {

    println!("Pulling image: {}", image_string);
    let mut image = parse_image(image_string.to_owned())?;
    let destination = format!("oci:{}:{}", image.dir, image.reference);
    println!("image uri: {}", image.uri);
    let output = Command::new("skopeo").arg("copy").arg(image.uri).arg(destination).output()?;
    if !output.status.success() {
        return Ok(output.stderr);
    }

    image.rootfs = unpack_image_to_rootfs(image.dir, image.reference)?;
    return Ok(output.stderr);
}

// Parses an image string into an Image struct
fn parse_image(image: String) -> Result<Image, Error> {
    let (name, reference) = image.split_once(':').ok_or(Error::new(ErrorKind::Other, "Could not parse image. Provide image in name:tag format"))?;
    fs::create_dir_all("/var/lib/containert/")?;
    let image_dir = format!("/var/lib/containert/{}", name.to_string());
    let image_uri = format!("docker://{}", image);
    let image_rootfs = format!("{}/rootfs", image_dir);

    return Ok(Image{name: name.to_string(), reference: reference.to_string(), dir: image_dir.to_string(), uri: image_uri.to_string(), rootfs: image_rootfs});
}

// umoci raw unpack --image ubuntu@sha256:bace9fb0d5923a675c894d5c815da75ffe35e24970166a48a4460a48ae6e0d19 rootfs
// Unpacks an oci image reference to a root file system
pub fn unpack_image_to_rootfs(image_dir: String, image_reference: String) -> Result<String, Error> {
    let image = format!("{}:{}", image_dir, image_reference);
    println!("{:?}", image);
    
    let destination = format!("{}/rootfs", image_dir);
    let output = Command::new("umoci").arg("raw").arg("unpack").arg("--image").arg(image).arg(destination).output()?;
    if !output.status.success() {
        let output_string = String::from_utf8(output.stderr).unwrap();
        println!("{:?}", output_string);
        return Err(Error::new(ErrorKind::Other, "Could not unpack image"));
    }

    let output_string = String::from_utf8(output.stdout).unwrap();
    println!("{:?}", output_string);

    return Ok(format!("{}/rootfs", image_dir));

}
