use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;


#[test]
fn test_stdin_input() {
    let mut cmd = assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.write_stdin("hello world")
        .assert()
        .success()
        .stdout("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9  -\n");
}

#[test]
fn test_single_file() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "test content").unwrap();
    
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("a1fff0ffefb9eace7230c24e50731f0a91c62f9cefdfe77121c2f607125dffae"))
        .stdout(predicate::str::contains(file.path().to_str().unwrap()));
}

#[test]
fn test_multiple_files() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();
    
    writeln!(file1, "content1").unwrap();
    writeln!(file2, "content2").unwrap();
    
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args([file1.path(), file2.path()])
        .assert()
        .success()
        .stdout(predicate::str::contains(file1.path().to_str().unwrap()))
        .stdout(predicate::str::contains(file2.path().to_str().unwrap()));
}

#[test]
fn test_sha1_algorithm() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["-a", "sha1"])
        .write_stdin("hello")
        .assert()
        .success()
        .stdout("aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d  -\n");
}

#[test]
fn test_sha512_algorithm() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["-a", "sha512"])
        .write_stdin("test")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("ee26b0dd4af7e749aa1a8ee3c10ae9923f618980772e473f8819a5d4940e0db2"))
        .stdout(predicate::str::ends_with("  -\n"));
}

#[test]
fn test_quiet_mode() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["-q"])
        .write_stdin("hello")
        .assert()
        .success()
        .stdout("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824\n");
}

#[test]
fn test_empty_input() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.write_stdin("")
        .assert()
        .success()
        .stdout("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  -\n");
}

#[test]
fn test_nonexistent_file() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("nonexistent_file.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to open file"));
}

#[test]
fn test_glob_pattern() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file1_path = temp_dir.path().join("test1.txt");
    let file2_path = temp_dir.path().join("test2.txt");
    
    fs::write(&file1_path, "content1").unwrap();
    fs::write(&file2_path, "content2").unwrap();
    
    let glob_pattern = temp_dir.path().join("*.txt");
    
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(glob_pattern.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("test1.txt"))
        .stdout(predicate::str::contains("test2.txt"));
}

#[test]
fn test_check_mode() {
    let mut content_file = NamedTempFile::new().unwrap();
    let mut hash_file = NamedTempFile::new().unwrap();
    
    writeln!(content_file, "test content").unwrap();
    
    // First generate hash
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let output = cmd.arg(content_file.path())
        .output()
        .unwrap();
    
    let hash_line = String::from_utf8(output.stdout).unwrap();
    writeln!(hash_file, "{}", hash_line.trim()).unwrap();
    
    // Now check it
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["-c", hash_file.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("OK"));
}

#[test]
fn test_check_mode_failure() {
    let mut content_file = NamedTempFile::new().unwrap();
    let mut hash_file = NamedTempFile::new().unwrap();
    
    writeln!(content_file, "test content").unwrap();
    
    // Write incorrect hash
    writeln!(hash_file, "0000000000000000000000000000000000000000000000000000000000000000  {}", 
             content_file.path().to_str().unwrap()).unwrap();
    
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["-c", hash_file.path().to_str().unwrap()])
        .assert()
        .failure()
        .stdout(predicate::str::contains("FAILED"));
}

#[test]
fn test_all_sha_algorithms() {
    let algorithms = vec![
        "sha1", "sha224", "sha256", "sha384", "sha512",
        "sha3-224", "sha3-256", "sha3-384", "sha3-512",
        "blake2b", "blake2s"
    ];
    
    for algorithm in algorithms {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.args(["-a", algorithm])
            .write_stdin("test")
            .assert()
            .success()
            .stdout(predicate::str::ends_with("  -\n"));
    }
}

#[test]
fn test_large_file() {
    let mut file = NamedTempFile::new().unwrap();
    let large_content = "x".repeat(100_000);
    write!(file, "{}", large_content).unwrap();
    
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(file.path().to_str().unwrap()));
}

#[test]
fn test_binary_file() {
    let mut file = NamedTempFile::new().unwrap();
    let binary_data: Vec<u8> = (0..=255).collect();
    file.write_all(&binary_data).unwrap();
    
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(file.path().to_str().unwrap()));
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Calculate SHA hashes"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}