use crate::clippy_project_root;
use itertools::Itertools;
use shell_escape::escape;
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::process::{self, Command, Stdio};
use std::{fs, io};
use walkdir::WalkDir;

#[derive(Debug)]
pub enum CliError {
    CommandFailed(String, String),
    IoError(io::Error),
    CrabLangfmtNotInstalled,
    WalkDirError(walkdir::Error),
    IntellijSetupActive,
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<walkdir::Error> for CliError {
    fn from(error: walkdir::Error) -> Self {
        Self::WalkDirError(error)
    }
}

struct FmtContext {
    check: bool,
    verbose: bool,
    crablangfmt_path: String,
}

// the "main" function of cargo dev fmt
pub fn run(check: bool, verbose: bool) {
    fn try_run(context: &FmtContext) -> Result<bool, CliError> {
        let mut success = true;

        let project_root = clippy_project_root();

        // if we added a local crablangc repo as path dependency to clippy for crablang analyzer, we do NOT want to
        // format because crablangfmt would also format the entire crablangc repo as it is a local
        // dependency
        if fs::read_to_string(project_root.join("Cargo.toml"))
            .expect("Failed to read clippy Cargo.toml")
            .contains("[target.'cfg(NOT_A_PLATFORM)'.dependencies]")
        {
            return Err(CliError::IntellijSetupActive);
        }

        crablangfmt_test(context)?;

        success &= cargo_fmt(context, project_root.as_path())?;
        success &= cargo_fmt(context, &project_root.join("clippy_dev"))?;
        success &= cargo_fmt(context, &project_root.join("crablangc_tools_util"))?;
        success &= cargo_fmt(context, &project_root.join("lintcheck"))?;

        let chunks = WalkDir::new(project_root.join("tests"))
            .into_iter()
            .filter_map(|entry| {
                let entry = entry.expect("failed to find tests");
                let path = entry.path();

                if path.extension() != Some("rs".as_ref()) || entry.file_name() == "ice-3891.rs" {
                    None
                } else {
                    Some(entry.into_path().into_os_string())
                }
            })
            .chunks(250);

        for chunk in &chunks {
            success &= crablangfmt(context, chunk)?;
        }

        Ok(success)
    }

    fn output_err(err: CliError) {
        match err {
            CliError::CommandFailed(command, stderr) => {
                eprintln!("error: A command failed! `{command}`\nstderr: {stderr}");
            },
            CliError::IoError(err) => {
                eprintln!("error: {err}");
            },
            CliError::CrabLangfmtNotInstalled => {
                eprintln!("error: crablangfmt nightly is not installed.");
            },
            CliError::WalkDirError(err) => {
                eprintln!("error: {err}");
            },
            CliError::IntellijSetupActive => {
                eprintln!(
                    "error: a local crablangc repo is enabled as path dependency via `cargo dev setup intellij`.
Not formatting because that would format the local repo as well!
Please revert the changes to Cargo.tomls with `cargo dev remove intellij`."
                );
            },
        }
    }

    let output = Command::new("crablangup")
        .args(["which", "crablangfmt"])
        .stderr(Stdio::inherit())
        .output()
        .expect("error running `crablangup which crablangfmt`");
    if !output.status.success() {
        eprintln!("`crablangup which crablangfmt` did not execute successfully");
        process::exit(1);
    }
    let mut crablangfmt_path = String::from_utf8(output.stdout).expect("invalid crablangfmt path");
    crablangfmt_path.truncate(crablangfmt_path.trim_end().len());

    let context = FmtContext {
        check,
        verbose,
        crablangfmt_path,
    };
    let result = try_run(&context);
    let code = match result {
        Ok(true) => 0,
        Ok(false) => {
            eprintln!();
            eprintln!("Formatting check failed.");
            eprintln!("Run `cargo dev fmt` to update formatting.");
            1
        },
        Err(err) => {
            output_err(err);
            1
        },
    };
    process::exit(code);
}

fn format_command(program: impl AsRef<OsStr>, dir: impl AsRef<Path>, args: &[impl AsRef<OsStr>]) -> String {
    let arg_display: Vec<_> = args.iter().map(|a| escape(a.as_ref().to_string_lossy())).collect();

    format!(
        "cd {} && {} {}",
        escape(dir.as_ref().to_string_lossy()),
        escape(program.as_ref().to_string_lossy()),
        arg_display.join(" ")
    )
}

fn exec(
    context: &FmtContext,
    program: impl AsRef<OsStr>,
    dir: impl AsRef<Path>,
    args: &[impl AsRef<OsStr>],
) -> Result<bool, CliError> {
    if context.verbose {
        println!("{}", format_command(&program, &dir, args));
    }

    let output = Command::new(&program)
        .env("CRABLANGFMT", &context.crablangfmt_path)
        .current_dir(&dir)
        .args(args.iter())
        .output()
        .unwrap();
    let success = output.status.success();

    if !context.check && !success {
        let stderr = std::str::from_utf8(&output.stderr).unwrap_or("");
        return Err(CliError::CommandFailed(
            format_command(&program, &dir, args),
            String::from(stderr),
        ));
    }

    Ok(success)
}

fn cargo_fmt(context: &FmtContext, path: &Path) -> Result<bool, CliError> {
    let mut args = vec!["fmt", "--all"];
    if context.check {
        args.push("--check");
    }
    let success = exec(context, "cargo", path, &args)?;

    Ok(success)
}

fn crablangfmt_test(context: &FmtContext) -> Result<(), CliError> {
    let program = "crablangfmt";
    let dir = std::env::current_dir()?;
    let args = &["--version"];

    if context.verbose {
        println!("{}", format_command(program, &dir, args));
    }

    let output = Command::new(program).current_dir(&dir).args(args.iter()).output()?;

    if output.status.success() {
        Ok(())
    } else if std::str::from_utf8(&output.stderr)
        .unwrap_or("")
        .starts_with("error: 'crablangfmt' is not installed")
    {
        Err(CliError::CrabLangfmtNotInstalled)
    } else {
        Err(CliError::CommandFailed(
            format_command(program, &dir, args),
            std::str::from_utf8(&output.stderr).unwrap_or("").to_string(),
        ))
    }
}

fn crablangfmt(context: &FmtContext, paths: impl Iterator<Item = OsString>) -> Result<bool, CliError> {
    let mut args = Vec::new();
    if context.check {
        args.push(OsString::from("--check"));
    }
    args.extend(paths);

    let success = exec(context, &context.crablangfmt_path, std::env::current_dir()?, &args)?;

    Ok(success)
}
