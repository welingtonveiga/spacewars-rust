use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("..")
        .join("target")
        .join(build_type);
    return PathBuf::from(path);
}

fn main() {
    let target_dir = get_output_path().join("assets");

    fs::create_dir_all(&target_dir).unwrap();

    let src = Path::join(
        &env::current_dir().unwrap(),
        "../assets/NovaSquare-Regular.ttf",
    );
    let dest = Path::join(Path::new(&target_dir), Path::new("NovaSquare-Regular.ttf"));

    println!("{:?} {:?}", src, dest);

    fs::copy(src, dest).unwrap();
}
