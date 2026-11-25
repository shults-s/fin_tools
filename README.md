# fin_tools

Yandex Practicum's Middle Rust Course. Project of 1st Module: Reading, Parsing, and Analyzing Data in Rust.

## FOR_REVIEWER

I've left these markers in all of places in code where:

- I'm unsure or don't know the correct (idiomatic) way to implement things,
- I intentionally use `.unwrap()` despite of prohibition of using this (as specified in problem statement).
  - In a few places, where meeting a certain condition would mean violating an invariant, I've opted to call `panic!()`. Yeah, guys from Cloudflare thought that was safe, too.

When reviewing my solution, I ask for the **harshest criticism**. Point out what has been done well (to remember successful decisions), and what is frankly poor. No pain, no gain.

In this task, I've mixed various syntactic constructs, simply to explore different ways of implementing similar things (`&[impl AsRef<str>]` and `impl IntoIterator<Item: AsRef<str>>`, `&str` and `AsRef<str>`, etc.). As a result, the code as a whole might look a bit inconsistent.

## Fast check

```bash
cargo run --bin converter -- --src="./data/test_nosig.ftx" --dst="./data/tmp.csv"
```

> Error: ParsingError("Bad FTX signature bytes.")

```bash
cargo run --bin comparer -- --txs1="./data/test.ftx" --txs2="./data/test_reorder.csv"
```

> Error: "Transactions numbered 1 differ. Date: 2026-11-15 != 2026-11-16"

```bash
cargo run --bin comparer -- --txs1="./data/test.csv" --txs2="./data/test.ftx"
```

> Transaction lists are identical.

## Domain

Each transaction is defined by:

- `Date` of completion;
- `Kind` of transaction, a value from a fixed set: "Income", "Expence";
- `Amount` of transferred monies, always greater than zero;
- `Category` of transaction, arbitary non empty value which may only contain characters `a-z`, `0-9`, `-`, `_`.

### Constraints

All of text data is treated as lowercased ASCII strings, since I don't want to struggle with Unicode Normalization Forms and other pitfalls during subsequent string comparison.

## I/O data layouts

### Binary format

#### Header

- 3-byte signature:
  - fixed bytes 'F', 'T' (financial tools),
  - format version for further migrations.

| Bytes |  0  |  1  |    2    |
| :---: | :-: | :-: | :-----: |
| Type  | u8  | u8  |   u8    |
| Field | 'F' | 'T' | Version |

#### Entry (transaction)

All of multi-byte integer values are Little Endian.

- `Date`, number of days from Unix Epoch, is within range 1970.01.01..2149.06.06;
- `Kind` of transaction;
- `Amount` of transferred monies;
- `l`, bytes length of `Category`;
- `Category`, name of transaction category, max length is 255 characters;
  - should be enum also, but I want to demonstrate processing of variable-length data.

| Bytes | 0..=1 |  2   | 3..=10 | 11  |  13..l   |
| :---: | :---: | :--: | :----: | :-: | :------: |
| Type  |  u16  |  u8  |  u64   | u8  |   &str   |
| Field | Date  | Kind | Amount |  l  | Category |

### CSV format

- fields separator ",";
- have header with enumeration of columns, separated with field separator;
- order of columns arbitary, but all of columns must be present: "date", "category", "kind", "amount".

### Text format

- fields separator ";",
- due to missing header, order of columns fixed: "date", "category", "kind", "amount".
