# csvreader_titled

## How to use

Add this.
```rust
use csvreader_titled::CsvReader;
```

### Basics
```rust
CsvReader::open(CSV_FILE_PATH).read(|row| { println!("{:?}", row); });
```

### To get the count
```rust
let mut cr = CsvReader::open(CSV_FILE_PATH);
cr.read(|_| {});
println!("{}", cr.rows_count());
```
