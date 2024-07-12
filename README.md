# address-checker

Checks if the addresses in a csv file are correct, outputs a csv file with the incorrect and corrected addresses

## Usage

If it is not build:

```bash
cargo run -- --file-path "path/to/addresses_file.csv"
```

### CSV file

`file-path` is the path to the file containing the addresses. It must be a `.csv` file.
Here are the expected header:

### Other CLI flags

- `skip-error-check` - will skip if the GMaps API encountered an error fetching the addresses.

```csv
site,group,address,city,zip,country,administrative_area_level1,administrative_area_level2,lat,lng
```

## Disclaimer

_I am a `Rust` newbie, so the code might not be perfect :D_
_But it is a good first project with more depth than it initially shows._
