# fin_tools

Yandex Practicum's Middle Rust Course. Project of 1st Module: Reading, Parsing, and Analyzing Data in Rust

## Domain

Each transaction is defined by:
- `Date` of completion;
- `Kind` of transaction, a value from a fixed set: "Income", "Expence";
- `Amount` of transferred monies, always greater than zero;
- `Category` of transaction, arbitary non empty text value.

### Constraints

All of text data must be ASCII strings, since I don't want to struggle with Unicode Normalization Forms and other pitfalls during subsequent string comparison.

## I/O data layouts

### Binary format

#### Header

- 3-byte signature:
  - fixed bytes 'F', 'T' (financial tools),
  - format version for further migrations.

| Bytes |  0  |  1  |    2    |
|:-----:|:---:|:---:|:-------:|
|  Type |  u8 |  u8 |    u8   |
| Field | 'F' | 'T' | Version |

#### Entry (transaction)

All of multi-byte values are Little Endian.

- `Date`, number of days from Unix Epoch, is within range 1970.01.01..2149.06.06;
- `Kind` of transaction;
- `Amount` of transferred monies;
- `l`, bytes length of `Category`;
- `Category`, name of transaction category, max length is 255 characters;
  - should be enum also, but I want to demonstrate processing of variable-length data.

| Bytes | 0..=1 |   2  | 3..=10 | 11 |   13..l  |
|:-----:|:-----:|:----:|:------:|:--:|:--------:|
|  Type |  u16  |  u8  |   u64  | u8 |   &str   |
| Field |  Date | Kind | Amount |  l | Category |

### CSV format

- fields separator ",";
- have header with enumeration of columns, separated with field separator;
- order of columns arbitary, but all of columns must be present: "date", "kind", "amount", "category".

### Text format

- fields separator ";",
- due to missing header, order of columns fixed: "date", "kind", "amount", "category".
