//! This build script clones the Neutron version specified in the "revenue" execution parameter,
//! uses that to build the required proto files and makes a "proto_types" module in the "neutron-sdk"
//! lib out of the generated files. This is based on the proto-compiler code in
//! github.com/informalsystems/ibc-rs.

use regex::Regex;
use std::collections::BTreeMap;
use std::env;
use std::io::Write;
use std::{
    ffi::{OsStr, OsString},
    fs::{self, create_dir_all, remove_dir_all, File},
    io,
    path::{Path, PathBuf},
    process,
};
use walkdir::WalkDir;

const PROTO_BUILD_DIR: &str = "proto-build";
const BUF_CONFIG_FILE: &str = "buf.yaml";
const BUF_GEN_CONFIG_FILE: &str = "buf.neutron.gen.yaml";
const PROTO_DIR: &str = "packages/neutron-sdk/src/proto_types";
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";
const NEUTRON_DIR: &str = "neutron";
const NEUTRON_REPO: &str = "https://github.com/neutron-org/neutron.git";

macro_rules! info {
    ($msg:expr) => {
            println!("[info] {}", $msg)
    };
    ($fmt:expr, $($arg:tt)+) => {
        info!(&format!($fmt, $($arg)+))
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("neutron revision (commit hash) is expected as the only argument");
    }
    let revision = &args[1];

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let proto_dir: PathBuf = PROTO_DIR.parse().unwrap();

    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone()).unwrap();
    }

    let temp_neutron_dir = tmp_build_dir.join("neutron");

    let neutron_repo_dir: PathBuf = NEUTRON_DIR.parse().unwrap();
    if neutron_repo_dir.exists() {
        fs::remove_dir_all(neutron_repo_dir.clone()).unwrap();
    }

    clone_neutron(revision);
    compile_neutron_proto_and_services(&temp_neutron_dir);
    copy_generated_files(&temp_neutron_dir, &proto_dir);
    output_neutron_version(&proto_dir, revision);

    info!("Running rustfmt on prost/tonic-generated code");
    run_rustfmt(&proto_dir);

    println!(
        "Rebuild protos with proto-build (neutron revision: {})",
        revision
    );
    fs::remove_dir_all(neutron_repo_dir.clone()).unwrap();
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

fn clone_neutron(revision: &String) {
    info!("Cloning neutron repo...");
    run_git(["clone", NEUTRON_REPO]);
    run_git(["-C", NEUTRON_DIR, "fetch"]);
    run_git(["-C", NEUTRON_DIR, "reset", "--hard", revision]);
}

fn output_neutron_version(out_dir: &Path, revision: &String) {
    let path = out_dir.join("NEUTRON_COMMIT");
    fs::write(path, revision).unwrap();
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

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    info!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    remove_dir_all(to_dir).unwrap_or_default();
    create_dir_all(to_dir).unwrap();

    // Copy new compiled files (prost does not use folder structures)
    let files = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && str::ends_with(e.file_name().to_str().unwrap_or_default(), ".rs")
        })
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            copy_and_patch(e.path(), format!("{}/{}", to_dir.display(), filename)).unwrap();
            filename
        })
        .collect::<Vec<String>>();

    let mut file =
        File::create("packages/neutron-sdk/src/proto_types/mod.rs").expect("Unable to create file");
    file.write_all(generate_mod_rs(files).as_bytes())
        .expect("Unable to write data");
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

enum ModuleContent {
    Submodule(BTreeMap<String, ModuleContent>),
    File(String),
}

/// ChatGPT-generated code that creates a mod.rs file content out of a list of proto files generated
/// in Rust. All the passed files are expected to have names of modules and versions split by dots,
/// and these split parts are used to create layered submodules. Merging of submodules is supported.
///
/// Example: ["neutron.contractmanager.v1.rs", "neutron.contractmanager.rs"] result in the following:
///
/// pub mod neutron {
///     pub mod contractmanager {
///         include!("neutron.contractmanager.rs");
///         pub mod v1 {
///             include!("neutron.contractmanager.v1.rs");
///         }
///     }
/// }
fn generate_mod_rs(file_names: Vec<String>) -> String {
    let mut mod_rs = String::new();
    let mut modules = BTreeMap::new();

    for file_name in file_names {
        let parts: Vec<&str> = file_name.split('.').collect();
        insert_into_module(&mut modules, &parts, file_name.clone());
    }

    fn insert_into_module(
        modules: &mut BTreeMap<String, ModuleContent>,
        parts: &[&str],
        file_name: String,
    ) {
        if parts.len() == 1 {
            modules.insert(
                parts[0].to_string(),
                ModuleContent::File(file_name.to_string()),
            );
        } else {
            let module_name = parts[0];
            let sub_parts = &parts[1..];
            let sub_module = modules
                .entry(module_name.to_string())
                .or_insert_with(|| ModuleContent::Submodule(BTreeMap::new()));
            match sub_module {
                ModuleContent::Submodule(sub_module_map) => {
                    insert_into_module(sub_module_map, sub_parts, file_name);
                }
                ModuleContent::File(_) => {
                    modules.insert(
                        module_name.to_string(),
                        ModuleContent::File(file_name.to_string()),
                    );
                }
            }
        }
    }

    fn generate_module(
        module_dict: &BTreeMap<String, ModuleContent>,
        mod_rs: &mut String,
        indentation: &str,
    ) {
        for (module_name, content) in module_dict {
            if module_name != "rs" {
                mod_rs.push_str(&format!("{}pub mod {} {{\n", indentation, module_name));
                match content {
                    ModuleContent::Submodule(submodule) => {
                        generate_module(submodule, mod_rs, &format!("{}    ", indentation));
                    }
                    ModuleContent::File(file) => {
                        mod_rs.push_str(&format!("{}    include!(\"{}\");\n", indentation, file));
                    }
                }
                mod_rs.push_str(&format!("{}}}\n", indentation));
            } else {
                match content {
                    ModuleContent::Submodule(submodule) => {
                        generate_module(submodule, mod_rs, indentation);
                    }
                    ModuleContent::File(file) => {
                        mod_rs.push_str(&format!("{}include!(\"{}\");\n", indentation, file));
                    }
                }
            }
        }
    }

    generate_module(&modules, &mut mod_rs, "");
    mod_rs
}
