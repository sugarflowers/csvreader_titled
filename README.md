# csvreader_titled

## How to use

Add this.
```
use csvreader_titled::CsvReader;
```

### Basics
```
CsvReader::open(CSV_FILE_PATH).read(|row| { println!("{:?}", row); });
```

### To get the count
```
let mut cr = CsvReader::open(CSV_FILE_PATH);
cr.read(|_| {});
println!("{}", cr.rows_count());
```
