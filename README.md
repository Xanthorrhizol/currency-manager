# currency-manager
currency manager of account

## Usage

### Report

```bash
cargo run --csv-path <csv file path>
```
You should match the csv file's format same with history.csv

### Update

- When update for today

```bash
cargo run --csv-path <csv file path>
```

- When update for some date

```bash
cargo run --csv-path <csv file path> --date <YYYY-mm-dd>
```
