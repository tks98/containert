use std::process::Command;
use std::io::Error;
use std::io::ErrorKind;
use std::fs;
use std::path::Path;
use std::path::PathBuf;


#[derive(Clone, Debug)]
struct Image {
    name: String,
    reference: String,
    dir: Box<PathBuf>,
    rootfs: Box<PathBuf>,
    uri: String
}

// Copy the layers of the specified image to the local filesystem
// Essentially performs the example here up til runc
// https://manpages.ubuntu.com/manpages/jammy/man1/umoci-raw-unpack.1.html
pub fn pull_image(image_string: String, path: String) -> Result<Vec<u8>, Error> {

    println!("Pulling image: {}", image_string);
    let image = parse_image(image_string, path)?;
    let destination = format!("oci:{}:{}", image.dir.as_path().display().to_string(), image.reference);
  
    let output = Command::new("skopeo").arg("copy").arg(&image.uri).arg(&destination).output()?;
    if !output.status.success() {
        println!("{}", image.uri);
        println!("{}", destination);
        return Ok(output.stderr);
    }

    unpack_image_to_rootfs(image.rootfs, image.dir, image.reference)?;
    return Ok(output.stderr);
}

// Parses an image string into an Image struct
fn parse_image(image: String, mut path_str: String) -> Result<(Image), Error> {
    let (name, reference) = image.split_once(':').ok_or(Error::new(ErrorKind::Other, "Could not parse image. Provide image in name:tag format"))?;

    // Determine where to save the image rootfs depending on if user supplied a path
    let default_path = "/var/lib/containert".to_string();
    if path_str.is_empty() {
        path_str = default_path
    }

    // Create and check if the path specified (or the default path) exists, create it if it doesnt
    let path = Path::new(&path_str);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    
    println!("path: {:?}", path.to_str());

    // Build the image structure
    let image_dir = Box::new(path.join(name));
    println!("image dir: {:?}", image_dir.to_str());
    let image_uri = format!("docker://{}", image);
    let image_rootfs = Box::new(path.join("rootfs"));
    println!("image rootfs dir: {:?}", path.to_str());

    return Ok(Image{name: name.to_string(), reference: reference.to_string(), dir: image_dir, uri: image_uri.to_string(), rootfs: image_rootfs});
}

// umoci raw unpack --image ubuntu@sha256:bace9fb0d5923a675c894d5c815da75ffe35e24970166a48a4460a48ae6e0d19 rootfs
// Unpacks an oci image reference to a root file system
pub fn unpack_image_to_rootfs(image_rootfs: Box<PathBuf>, image_dir: Box<PathBuf>, image_reference: String) -> Result<(), Error> {
    let image_name = format!("{}:{}", image_dir.display(), image_reference);
    println!("image_name{:?}", image_name);


    let output = Command::new("umoci").arg("raw").arg("unpack").arg("--image").arg(image_name).arg(image_rootfs.as_path()).output()?;
    if !output.status.success() {
        let output_string = String::from_utf8(output.stderr).unwrap();
        println!("{:?}", output_string);
        return Err(Error::new(ErrorKind::Other, "Could not unpack image"));
    }

    let output_string = String::from_utf8(output.stdout).unwrap();
    println!("{:?}", output_string);

    return Ok(());

}
