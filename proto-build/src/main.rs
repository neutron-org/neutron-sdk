//! Build CosmosSDK/Tendermint/IBC proto files. This build script clones the CosmosSDK version
//! specified in the COSMOS_SDK_REV constant and then uses that to build the required
//! proto files for further compilation. This is based on the proto-compiler code
//! in github.com/informalsystems/ibc-rs

use regex::Regex;
use std::{
    ffi::{OsStr, OsString},
    fs::{self, create_dir_all, remove_dir_all},
    io,
    ops::Add,
    path::{Path, PathBuf},
    process,
};
use walkdir::WalkDir;

const PROTO_BUILD_DIR: &str = "proto-build";
const BUF_CONFIG_FILE: &str = "buf.yaml";
const BUF_GEN_CONFIG_FILE: &str = "buf.neutron.gen.yaml";

const PROTO_DIR: &str = "packages/neutron-sdk/src/proto_types";

/// The directory the generated neutron proto files go into in this repo
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

const NEUTRON_DIR: &str = "neutron";

/// Reformat proto https://github.com/neutron-org/neutron/pull/396
const NEUTRON_REV: &str = "60b7d38358efb2f8e45e25ee4db8e19a6462dd9e";

// TODO(tarcieri): use a logger for this
macro_rules! info {
    ($msg:expr) => {
            println!("[info] {}", $msg)
    };
    ($fmt:expr, $($arg:tt)+) => {
        info!(&format!($fmt, $($arg)+))
    };
}

fn main() {
    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let proto_dir: PathBuf = PROTO_DIR.parse().unwrap();

    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone()).unwrap();
    }

    let temp_neutron_dir = tmp_build_dir.join("neutron");

    fs::create_dir_all(&temp_neutron_dir).unwrap();

    update_submodules();
    output_neutron_version(&temp_neutron_dir);
    compile_neutron_proto_and_services(&temp_neutron_dir);

    copy_generated_files(&temp_neutron_dir, &proto_dir);

    info!("Running rustfmt on prost/tonic-generated code");
    run_rustfmt(&proto_dir);

    println!(
        "Rebuild protos with proto-build (neutron-rev: {}))",
        NEUTRON_REV
    );
}

fn run_cmd(cmd: impl AsRef<OsStr>, args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = process::Stdio::inherit();
    let exit_status = process::Command::new(&cmd)
        .args(args)
        .stdout(stdout)
        .status()
        .unwrap_or_else(|e| match e.kind() {
            io::ErrorKind::NotFound => panic!(
                "error running '{:?}': command not found. Is it installed?",
                cmd.as_ref()
            ),
            _ => panic!("error running '{:?}': {:?}", cmd.as_ref(), e),
        });

    if !exit_status.success() {
        match exit_status.code() {
            Some(code) => panic!("{:?} exited with error code: {:?}", cmd.as_ref(), code),
            None => panic!("{:?} exited without error code", cmd.as_ref()),
        }
    }
}

fn run_buf(proto_path: impl AsRef<Path>, out_dir: impl AsRef<Path>) {
    let proto_build_dir = Path::new(PROTO_BUILD_DIR);
    let buf_cfg_path = proto_build_dir.join(BUF_CONFIG_FILE);
    let buf_gen_cfg_path = proto_build_dir.join(BUF_GEN_CONFIG_FILE);
    run_cmd(
        "buf",
        [
            "generate",
            "--template",
            buf_gen_cfg_path.to_str().unwrap(),
            "--config",
            buf_cfg_path.to_str().unwrap(),
            "-o",
            &out_dir.as_ref().display().to_string(),
            &proto_path.as_ref().display().to_string(),
        ],
    );
}

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    run_cmd("git", args)
}

fn run_rustfmt(dir: &Path) {
    let mut args = ["--edition", "2021"]
        .iter()
        .map(Into::into)
        .collect::<Vec<OsString>>();

    args.extend(
        WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.path().extension() == Some(OsStr::new("rs")))
            .map(|e| e.into_path())
            .map(Into::into),
    );

    run_cmd("rustfmt", args);
}

fn update_submodules() {
    info!("Updating neutron submodule...");
    run_git(["submodule", "update", "--init"]);
    run_git(["-C", NEUTRON_DIR, "fetch"]);
    run_git(["-C", NEUTRON_DIR, "reset", "--hard", NEUTRON_REV]);
}

fn output_neutron_version(out_dir: &Path) {
    let path = out_dir.join("NEUTRON_COMMIT");
    fs::write(path, NEUTRON_REV).unwrap();
}

fn compile_neutron_proto_and_services(out_dir: &Path) {
    let sdk_dir = Path::new(NEUTRON_DIR);
    let proto_path = sdk_dir.join("proto");
    let proto_paths = [format!("{}/proto/", sdk_dir.display())];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    // Compile all proto client for GRPC services
    info!("Compiling neutron proto clients for GRPC services!");
    run_buf(proto_path, out_dir);
    info!("=> Done!");
}

/// collect_protos walks every path in `proto_paths` and recursively locates all .proto
/// files in each path's subdirectories, adding the full path of each file to `protos`
///
/// Any errors encountered will cause failure for the path provided to WalkDir::new()
fn collect_protos(proto_paths: &[String], protos: &mut Vec<PathBuf>) {
    for proto_path in proto_paths {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }
}

// pub struct Module {
//     pub submodules: HashMap<String, ModulePart>,
// }

// pub struct ModulePart {
//     pub name: String,
//     pub ancestor: Option<String>,
//     pub children: HashMap<String, ModulePart>,
// }

// impl Module {
//     pub fn add_module_from_proto(filename: String) {
//         let parts = filename.split(".").collect::<Vec<&str>>();
//     }
// }

// impl ModulePart {
//     pub fn new_from_proto(filename: String) -> Self {
//         let parts = filename.split(".").collect::<Vec<&str>>();
//         let name: String = parts.get(0).unwrap().to_string();
//         let mut res = Self {
//             name,
//             ancestor: None,
//             children: HashMap::new(),
//         };
//         res
//     }

//     pub fn extend()
// }

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    info!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    remove_dir_all(to_dir).unwrap_or_default();
    create_dir_all(to_dir).unwrap();

    let mut mod_content: String = Default::default();

    // let mut modules: HashMap<String, ModulePart> = HashMap::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            mod_content = mod_content
                .clone()
                .add(build_lib_mod_for_proto(filename.clone()).as_str());

            info!("new mod content:");
            info!(mod_content);
            copy_and_patch(e.path(), format!("{}/{}", to_dir.display(), filename))
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
    info!("build mod content:");
    info!(mod_content);
}

fn build_lib_mod_for_proto(proto_file: String) -> String {
    info!("bulding mod content for {}", proto_file);
    let parts = proto_file.split('.').collect::<Vec<&str>>();
    info!("got its parts: {:?}", parts);
    add_part_to_mod(parts, 0)
}

fn add_part_to_mod(parts: Vec<&str>, i: usize) -> String {
    let part = parts.get(i).unwrap().to_string();

    let mut res: String = format!("pub mod {} {{", part);
    if parts.get(i + 1).is_none() {
        return format!("include!(\"{}\");", parts.join("."));
    }
    res = res.clone().add(add_part_to_mod(parts, i + 1).as_str());
    res = res.clone().add("}");
    info!("got a part for mod: {}", res);
    res
}

fn copy_and_patch(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    /// Regex substitutions to apply to the prost-generated output
    const REPLACEMENTS: &[(&str, &str)] = &[
        // Use `tendermint-proto` proto definitions
        ("(super::)+tendermint", "tendermint_proto"),
        // Feature-gate gRPC client modules
        (
            "/// Generated client implementations.",
            "/// Generated client implementations.\n\
             #[cfg(feature = \"grpc\")]",
        ),
        // Feature-gate gRPC impls which use `tonic::transport`
        (
            "impl(.+)tonic::transport(.+)",
            "#[cfg(feature = \"grpc-transport\")]\n    \
             impl${1}tonic::transport${2}",
        ),
        // Feature-gate gRPC server modules
        (
            "/// Generated server implementations.",
            "/// Generated server implementations.\n\
             #[cfg(feature = \"grpc\")]",
        ),
    ];

    let mut contents = fs::read_to_string(src)?;

    for &(regex, replacement) in REPLACEMENTS {
        contents = Regex::new(regex)
            .unwrap_or_else(|_| panic!("invalid regex: {}", regex))
            .replace_all(&contents, replacement)
            .to_string();
    }

    fs::write(dest, &contents)
}
