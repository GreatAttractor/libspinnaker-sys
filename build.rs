use std::env;
use std::path::Path;

fn main() {
    let target = env::var("TARGET").expect("getting target");

    let libdir = match env::var("SPINNAKER_LIBDIR") {
        Ok(dir) => dir,
        Err(_) => {
            match target.as_ref() {
                "x86_64-pc-windows-msvc"
                | "x86_64-pc-windows-gnu"
                | "i686-pc-windows-msvc"
                | "i686-pc-windows-gnu" => panic!("Missing env. variable SPINNAKER_LIBDIR. Set it to Spinnaker_C DLL location."),

                _ => "/opt/spinnaker/lib".to_string(),
            }
        }
    };

    let libdir = Path::new(&libdir);

    let libname = match target.as_ref() {
        "x86_64-pc-windows-msvc"
        | "i686-pc-windows-msvc"
        | "x86_64-pc-windows-gnu"
        | "i686-pc-windows-gnu" => find_windows_library_in_dir(&libdir),

        _ => "Spinnaker_C".to_string(),
    };

    println!("cargo:rustc-link-search=native={}", libdir.display());
    println!("cargo:rustc-link-lib={}", libname);
}

fn find_windows_library_in_dir(path: &Path) -> String {
    let mut dlls = vec![];

    for d_entry in std::fs::read_dir(path).unwrap() {
        if d_entry.is_err() { continue; }
        if let Ok(fname) = d_entry.unwrap().file_name().into_string() {
            dlls.push(fname);
        }
    }

    let stem = "SpinnakerC";

    let get_stem = |s| { return Path::new(s).file_stem().unwrap().to_str().unwrap().to_string(); };

    // if the user took care to rename one of the versioned DLLs as "SpinnakerC.dll", use it
    if let Some(item) = dlls.iter().find(|s| **s == format!("{}.dll", stem)) {
        return get_stem(item);
    }

    // otherwise, use the first non-debug one
    if let Some(item) = dlls.iter().find(|s| s.starts_with(&format!("{}_", stem)) && s.ends_with(".dll")) {
        return get_stem(item);
    }

    // otherwise, use the first debug one
    if let Some(item) = dlls.iter().find(|s| s.starts_with(&format!("{}d", stem)) && s.ends_with(".dll")) {
        return get_stem(item);
    }

    panic!("Could not find SpinnakerC*.dll in {}.", path.display());
}
