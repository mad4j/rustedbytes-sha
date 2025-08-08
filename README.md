# SHA Calculator (rustedbytes-sha)

Un'applicazione CLI in Rust per calcolare hash SHA e altri algoritmi crittografici. Compatibile con strumenti standard come `sha256sum`, supporta input da stdin, file singoli e pattern glob.

## Caratteristiche

- **Algoritmi supportati**: SHA-1, SHA-224, SHA-256, SHA-384, SHA-512, SHA3-224, SHA3-256, SHA3-384, SHA3-512, BLAKE2b-512, BLAKE2s-256
- **Input flessibile**: stdin, file singoli, pattern glob
- **Modalità di verifica**: verifica hash esistenti come `sha256sum -c`
- **Output compatibile**: formato identico a `sha256sum`
- **Performance**: gestione efficiente di file di grandi dimensioni

## Installazione

```bash
# Clona il repository
git clone <repository-url>
cd sha-calc

# Compila
cargo build --release

# Installa (opzionale)
cargo install --path .
```

## Utilizzo

### Esempi di base

```bash
# Calcola SHA-256 da stdin
echo "hello world" | sha-calc

# Calcola hash di un file
sha-calc file.txt

# Usa un algoritmo diverso
sha-calc -a sha512 file.txt

# Output solo dell'hash (senza nome file)
sha-calc -q file.txt

# Calcola hash di più file
sha-calc file1.txt file2.txt file3.txt

# Usa pattern glob
sha-calc *.txt
sha-calc /path/to/files/*.log
```

### Algoritmi disponibili

```bash
sha-calc -a sha1 file.txt      # SHA-1 (legacy)
sha-calc -a sha224 file.txt    # SHA-224
sha-calc -a sha256 file.txt    # SHA-256 (default)
sha-calc -a sha384 file.txt    # SHA-384
sha-calc -a sha512 file.txt    # SHA-512
sha-calc -a sha3-256 file.txt  # SHA3-256
sha-calc -a blake2b file.txt   # BLAKE2b-512
sha-calc -a blake2s file.txt   # BLAKE2s-256
```

### Modalità di verifica

```bash
# Genera file di hash
sha-calc *.txt > checksums.sha256

# Verifica i file
sha-calc -c checksums.sha256

# Verifica silenziosa (mostra solo errori)
sha-calc -c -q checksums.sha256
```

### Esempi avanzati

```bash
# Pipeline con altri comandi
find . -name "*.txt" -exec sha-calc {} \; > all_hashes.txt

# Confronto con sha256sum
sha256sum file.txt
sha-calc file.txt
# Output identico!

# Gestione di file grandi
sha-calc large_file.bin

# File binari
sha-calc /bin/ls
```

## Opzioni della riga di comando

```bash
USAGE:
    sha-calc [OPTIONS] [FILES]...

ARGUMENTS:
    <FILES>...    Input files or glob patterns

OPTIONS:
    -a, --algorithm <ALGORITHM>    Hash algorithm to use [default: sha256]
    -c, --check                    Check hash files (format: hash filename)
    -q, --quiet                    Output only the hash (no filename)
    -h, --help                     Print help information
    -V, --version                  Print version information
```

## Compatibilità

L'output è completamente compatibile con gli strumenti standard:

- `sha256sum` per SHA-256
- `sha512sum` per SHA-512
- `sha1sum` per SHA-1

```bash
# Questi comandi producono output identici
sha256sum file.txt
sha-calc file.txt
sha-calc -a sha256 file.txt
```

## Performance

- Gestione efficiente della memoria per file di grandi dimensioni
- Lettura a buffer per ottimizzare I/O
- Supporto per file binari di qualsiasi dimensione
- Hash calcolati in streaming senza caricare tutto in memoria

## Test

L'applicazione include test completi:

```bash
# Esegui tutti i test
cargo test

# Test con output dettagliato
cargo test -- --nocapture

# Test di integrazione
cargo test --test integration_tests

# Test con copertura
cargo test && cargo tarpaulin --html
```

### Test inclusi

- Test unitari per tutti gli algoritmi hash
- Test di integrazione CLI
- Test di compatibilità con valori noti
- Test di gestione errori
- Test di pattern glob
- Test di modalità verifica
- Test di performance con file grandi

## Sicurezza

- **SHA-1**: Deprecato per uso crittografico, mantenuto solo per compatibilità
- **SHA-256/384/512**: Raccomandati per la maggior parte degli utilizzi
- **SHA-3**: Algoritmo più recente, ottima alternativa
- **BLAKE2**: Prestazioni eccellenti, sicurezza elevata

## Gestione errori

L'applicazione gestisce correttamente:

- File non esistenti
- Permessi insufficienti  
- Pattern glob non validi
- File di hash malformati
- Input corrotti
- Interruzioni di I/O

## Esempi pratici

### Backup e integrità

```bash
# Crea checksums per backup
find /important/data -type f -exec sha-calc {} \; > backup_checksums.txt

# Verifica integrità dopo restore
sha-calc -c backup_checksums.txt
```

### Confronto directory

```bash
# Directory A
cd /path/to/dir_a
sha-calc * > ../checksums_a.txt

# Directory B  
cd /path/to/dir_b
sha-calc * > ../checksums_b.txt

# Confronta
diff ../checksums_a.txt ../checksums_b.txt
```

### Monitoraggio modifiche

```bash
# Baseline
sha-calc /etc/passwd /etc/shadow > system_baseline.txt

# Controllo periodico
sha-calc -c system_baseline.txt || echo "ALERT: System files changed!"
```

## Contributi

1. Fork del progetto
2. Crea feature branch (`git checkout -b feature/amazing-feature`)
3. Commit delle modifiche (`git commit -m 'Add amazing feature'`)
4. Push al branch (`git push origin feature/amazing-feature`)
5. Apri una Pull Request

## Licenza

Questo progetto è licenziato sotto MIT License - vedi il file LICENSE per dettagli.

## Changelog

### v0.1.0

- Implementazione iniziale
- Supporto per tutti gli algoritmi SHA principali
- Compatibilità con sha256sum
- Modalità di verifica
- Supporto pattern glob
- Test completi
