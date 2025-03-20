use assert_cmd::Command;
use predicates::prelude::*;
use rand::distr::Alphanumeric;
use rand::Rng;
use std::fs;
use std::io::Read;

const PRG: &str = "wcr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const ATLAMAL: &str = "tests/inputs/atlamal.txt";

type TestResult = Result<(), Box<dyn std::error::Error>>;
#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicates::str::contains("Usage:"));
    }
    Ok(())
}

fn rand_string() -> String {
    let rng = rand::rng();
    rng.clone()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect()
}

fn gen_bad_file() -> String {
    loop {
        let file_name = rand_string();
        if fs::metadata(&file_name).is_err() {
            return file_name;
        }
    }
}
#[test]
fn dies_chars_and_bytes() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(["-m", "-c"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "the argument '--chars' cannot be used with '--bytes'",
        ));
    Ok(())
}
#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .arg(bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let mut file = fs::File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::eq(&expected.as_bytes() as &[u8]));

    Ok(())
}

#[allow(dead_code)]
fn run_assert_string(args: &[&str], expected_file: &str) -> TestResult {
    let mut file = fs::File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected_output = String::from_utf8_lossy(&buffer);

    let actual_output = String::from_utf8(
        Command::cargo_bin(PRG)?
            .args(args)
            .assert()
            .success()
            .get_output()
            .stdout
            .clone(),
    )?;

    assert_eq!(actual_output, expected_output);
    Ok(())
}

#[test]
fn empty() -> TestResult { run(&[EMPTY], "tests/expected/empty.txt.out")}
#[test]
fn empty_l() -> TestResult { run(&[EMPTY, "-l"], "tests/expected/empty.txt.l.out")}
#[test]
fn empty_w() -> TestResult { run(&[EMPTY, "-w"], "tests/expected/empty.txt.w.out")}
#[test]
fn empty_c() -> TestResult { run(&[EMPTY, "-c"], "tests/expected/empty.txt.c.out")}
#[test]
fn empty_m() -> TestResult { run(&[EMPTY, "-m"], "tests/expected/empty.txt.m.out")}
#[test]
fn empty_lwm() -> TestResult { run(&[EMPTY, "-lwm"], "tests/expected/empty.txt.lwm.out")}
#[test]
fn empty_wc() -> TestResult { run(&[EMPTY, "-wc"], "tests/expected/empty.txt.wc.out")}
#[test]
fn empty_wm() -> TestResult { run(&[EMPTY, "-wm"], "tests/expected/empty.txt.wm.out")}
#[test]
fn empty_wl() -> TestResult { run(&[EMPTY, "-wl"], "tests/expected/empty.txt.wl.out")}
#[test]
fn empty_cl() -> TestResult { run(&[EMPTY, "-cl"], "tests/expected/empty.txt.cl.out")}
#[test]
fn empty_ml() -> TestResult { run(&[EMPTY, "-ml"], "tests/expected/empty.txt.ml.out")}

#[test]
fn fox() -> TestResult { run(&[FOX], "tests/expected/fox.txt.out")}
#[test]
fn fox_l() -> TestResult { run(&[FOX, "-l"], "tests/expected/fox.txt.l.out")}
#[test]
fn fox_w() -> TestResult { run(&[FOX, "-w"], "tests/expected/fox.txt.w.out")}
#[test]
fn fox_c() -> TestResult { run(&[FOX, "-c"], "tests/expected/fox.txt.c.out")}
#[test]
fn fox_m() -> TestResult { run(&[FOX, "-m"], "tests/expected/fox.txt.m.out")}
#[test]
fn fox_lwm() -> TestResult { run(&[FOX, "-lwm"], "tests/expected/fox.txt.lwm.out")}
#[test]
fn fox_wc() -> TestResult { run(&[FOX, "-wc"], "tests/expected/fox.txt.wc.out")}
#[test]
fn fox_wm() -> TestResult { run(&[FOX, "-wm"], "tests/expected/fox.txt.wm.out")}
#[test]
fn fox_wl() -> TestResult { run(&[FOX, "-wl"], "tests/expected/fox.txt.wl.out")}
#[test]
fn fox_cl() -> TestResult { run(&[FOX, "-cl"], "tests/expected/fox.txt.cl.out")}
#[test]
fn fox_ml() -> TestResult { run(&[FOX, "-ml"], "tests/expected/fox.txt.ml.out")}
#[test]
fn atlamal() -> TestResult { run(&[ATLAMAL], "tests/expected/atlamal.txt.out")}
#[test]
fn atlamal_l() -> TestResult { run(&[ATLAMAL, "-l"], "tests/expected/atlamal.txt.l.out")}
#[test]
fn atlamal_w() -> TestResult { run(&[ATLAMAL, "-w"], "tests/expected/atlamal.txt.w.out")}
#[test]
fn atlamal_c() -> TestResult { run(&[ATLAMAL, "-c"], "tests/expected/atlamal.txt.c.out")}
#[test]
fn atlamal_m() -> TestResult { run(&[ATLAMAL, "-m"], "tests/expected/atlamal.txt.m.out")}
#[test]
fn atlamal_lwm() -> TestResult { run(&[ATLAMAL, "-lwm"], "tests/expected/atlamal.txt.lwm.out")}
#[test]
fn atlamal_wc() -> TestResult { run(&[ATLAMAL, "-wc"], "tests/expected/atlamal.txt.wc.out")}
#[test]
fn atlamal_wm() -> TestResult { run(&[ATLAMAL, "-wm"], "tests/expected/atlamal.txt.wm.out")}
#[test]
fn atlamal_wl() -> TestResult { run(&[ATLAMAL, "-wl"], "tests/expected/atlamal.txt.wl.out")}
#[test]
fn atlamal_cl() -> TestResult { run(&[ATLAMAL, "-cl"], "tests/expected/atlamal.txt.cl.out")}
#[test]
fn atlamal_ml() -> TestResult { run(&[ATLAMAL, "-ml"], "tests/expected/atlamal.txt.ml.out")}
#[test]
fn all() -> TestResult { run(&[EMPTY, FOX, ATLAMAL], "tests/expected/all.out")}
#[test]
fn all_l() -> TestResult { run(&[EMPTY, FOX, ATLAMAL, "-l"], "tests/expected/all.l.out")}
#[test]
fn all_w() -> TestResult { run(&[EMPTY, FOX, ATLAMAL, "-w"], "tests/expected/all.w.out")}
#[test]
fn all_c() -> TestResult { run(&[EMPTY, FOX, ATLAMAL, "-c"], "tests/expected/all.c.out")}
#[test]
fn all_m() -> TestResult { run(&[EMPTY, FOX, ATLAMAL, "-m"], "tests/expected/all.m.out")}
#[test]
fn all_lwm() -> TestResult { run(&[EMPTY, FOX, ATLAMAL, "-lwm"], "tests/expected/all.lwm.out")}
#[test]
fn all_wc() -> TestResult { run(&[EMPTY, FOX, ATLAMAL, "-wc"], "tests/expected/all.wc.out")}
#[test]
fn all_wm() -> TestResult { run(&[EMPTY, FOX, ATLAMAL, "-wm"], "tests/expected/all.wm.out")}
#[test]
fn all_wl() -> TestResult { run(&[EMPTY, FOX, ATLAMAL, "-wl"], "tests/expected/all.wl.out")}
#[test]
fn all_cl() -> TestResult { run(&[EMPTY, FOX, ATLAMAL, "-cl"], "tests/expected/all.cl.out")}
#[test]
fn all_ml() -> TestResult { run(&[EMPTY, FOX, ATLAMAL, "-ml"], "tests/expected/all.ml.out")}


fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let mut file = fs::File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let expected = String::from_utf8_lossy(&buffer);
    let input = fs::read_to_string(input_file)?;
    let output = Command::cargo_bin(PRG)?
        .write_stdin(input)
        .args(args)
        .output()
        .expect("fail");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected);

    Ok(())
}

#[test]
fn atlamal_stdin() -> TestResult {
    run_stdin(ATLAMAL, &[], "tests/expected/atlamal.txt.stdin.out")
}
