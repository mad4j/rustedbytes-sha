use clap::Parser;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};
use anyhow::{Context, Result};

mod hasher;
use hasher::{HashAlgorithm, calculate_hash};

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Calculate SHA hashes for files or stdin")]
struct Args {
    /// Hash algorithm to use
    #[arg(short, long, default_value = "sha256")]
    algorithm: HashAlgorithm,
    
    /// Input files or glob patterns (if none provided, reads from stdin)
    #[arg(value_name = "FILES")]
    files: Vec<String>,
    
    /// Output only the hash (no filename)
    #[arg(short, long)]
    quiet: bool,
    
    /// Check hash files (format: hash filename)
    #[arg(short, long)]
    check: bool,

    /// List all supported hash algorithms
    #[arg(long = "list-algorithms")]
    list_algorithms: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.list_algorithms {
        list_algorithms();
        return Ok(());
    }

    if args.check {
        return check_hashes(&args);
    }

    if args.files.is_empty() {
        // Read from stdin
        let hash = calculate_hash_from_reader(&mut io::stdin().lock(), args.algorithm)?;
        if args.quiet {
            println!("{}", hash);
        } else {
            println!("{}  -", hash);
        }
    } else {
        // Process files
        let mut all_files = Vec::new();

        for pattern in &args.files {
            if pattern.contains('*') || pattern.contains('?') || pattern.contains('[') {
                // Handle glob pattern
                let paths = glob::glob(pattern)
                    .with_context(|| format!("Failed to parse glob pattern: {}", pattern))?;

                for path in paths {
                    let path = path.with_context(|| format!("Failed to process glob: {}", pattern))?;
                    all_files.push(path.display().to_string());
                }
            } else {
                // Regular file
                all_files.push(pattern.clone());
            }
        }

        all_files.sort();

        for file_path in all_files {
            match process_file(&file_path, args.algorithm, args.quiet) {
                Ok(()) => {},
                Err(e) => {
                    eprintln!("sha-calc: {}: {}", file_path, e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
fn list_algorithms() {
    use hasher::HashAlgorithm;
    use clap::ValueEnum;
    println!("Supported hash algorithms:");
    for alg in HashAlgorithm::value_variants() {
        // Use the clap name for CLI compatibility
        println!("- {}", alg.to_possible_value().unwrap().get_name());
    }
}

fn process_file(file_path: &str, algorithm: HashAlgorithm, quiet: bool) -> Result<()> {
    let file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path))?;
    
    let mut reader = BufReader::new(file);
    let hash = calculate_hash_from_reader(&mut reader, algorithm)?;
    
    if quiet {
        println!("{}", hash);
    } else {
        println!("{}  {}", hash, file_path);
    }
    
    Ok(())
}

fn calculate_hash_from_reader<R: Read>(reader: &mut R, algorithm: HashAlgorithm) -> Result<String> {
    let mut buffer = [0; 8192];
    let hash = loop {
        let bytes_read = reader.read(&mut buffer)
            .context("Failed to read from input")?;
        
        if bytes_read == 0 {
            break calculate_hash(&[], algorithm, true);
        }
        
        if bytes_read == buffer.len() {
            // More data might be available, read all at once for efficiency
            let mut all_data = buffer.to_vec();
            reader.read_to_end(&mut all_data)
                .context("Failed to read remaining data")?;
            break calculate_hash(&all_data, algorithm, false);
        } else {
            // This is the last chunk
            break calculate_hash(&buffer[..bytes_read], algorithm, false);
        }
    };
    
    Ok(hash)
}

fn check_hashes(args: &Args) -> Result<()> {
    if args.files.is_empty() {
        anyhow::bail!("No hash files specified for checking");
    }
    
    let mut all_ok = true;
    
    for hash_file in &args.files {
        let content = std::fs::read_to_string(hash_file)
            .with_context(|| format!("Failed to read hash file: {}", hash_file))?;
        
        for (line_num, line) in content.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = line.splitn(2, "  ").collect();
            if parts.len() != 2 {
                eprintln!("sha-calc: {}: line {}: improperly formatted", hash_file, line_num + 1);
                all_ok = false;
                continue;
            }
            
            let expected_hash = parts[0];
            let file_path = parts[1];
            
            if file_path == "-" {
                eprintln!("sha-calc: cannot check stdin");
                all_ok = false;
                continue;
            }
            
            match process_file_check(file_path, expected_hash, args.algorithm) {
                Ok(true) => {
                    if !args.quiet {
                        println!("{}: OK", file_path);
                    }
                },
                Ok(false) => {
                    println!("{}: FAILED", file_path);
                    all_ok = false;
                },
                Err(e) => {
                    eprintln!("sha-calc: {}: {}", file_path, e);
                    all_ok = false;
                }
            }
        }
    }
    
    if !all_ok {
        std::process::exit(1);
    }
    
    Ok(())
}

fn process_file_check(file_path: &str, expected_hash: &str, algorithm: HashAlgorithm) -> Result<bool> {
    let file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path))?;
    
    let mut reader = BufReader::new(file);
    let actual_hash = calculate_hash_from_reader(&mut reader, algorithm)?;
    
    Ok(actual_hash.to_lowercase() == expected_hash.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    
    #[test]
    fn test_calculate_hash_from_reader_sha256() {
        let data = b"hello world";
        let mut cursor = Cursor::new(data);
        let hash = calculate_hash_from_reader(&mut cursor, HashAlgorithm::Sha256).unwrap();
        assert_eq!(hash, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    }
    
    #[test]
    fn test_calculate_hash_from_reader_empty() {
        let data = b"";
        let mut cursor = Cursor::new(data);
        let hash = calculate_hash_from_reader(&mut cursor, HashAlgorithm::Sha256).unwrap();
        assert_eq!(hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }
    
    #[test]
    fn test_different_algorithms() {
        let data = b"test";
        let mut cursor = Cursor::new(data);
        
        let sha1_hash = calculate_hash_from_reader(&mut cursor, HashAlgorithm::Sha1).unwrap();
        cursor.set_position(0);
        let sha256_hash = calculate_hash_from_reader(&mut cursor, HashAlgorithm::Sha256).unwrap();
        
        assert_ne!(sha1_hash, sha256_hash);
        assert_eq!(sha1_hash.len(), 40); // SHA-1 produces 160-bit hash (40 hex chars)
        assert_eq!(sha256_hash.len(), 64); // SHA-256 produces 256-bit hash (64 hex chars)
    }
}