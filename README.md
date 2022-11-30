# busterminal

busterminal is a **BLAZINGLY FAST**, **RUST-BASED** CLI for.. jk. It's just a simple CLI for retrieving information from the public transportation system in Norway

![busterminal example usage](https://s3.fr-par.scw.cloud/io.tmn.publicfiles/images/busterminal-5.png)

## Installation

Install **busterminal** using cargo:
```bash
$ cargo install busterminal
```

## Usage

Get departures from stops:
```bash
$ busterminal departure --stop "Tyholt"
```

Plan a trip:
```bash
$ busterminal trip --from "Oslo S" --to "Oslo Gardemoen Lufthavn"
```

<sub>This project is not affiliated with EnTur in any way.</sub>