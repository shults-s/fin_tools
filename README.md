# fin_tools

Yandex Practicum's Middle Rust Course. Project of 1st Module: Reading, Parsing, and Analyzing Data in Rust

## Domain constraints

- all of text data must be ASCII strings, since I don't want to struggle with Unicode Normalization Forms and other pitfalls during subsequent string comparison
- category len > 0
- amount > 0

## Binary data layout

### Header

- 3-byte signature:
  - fixed bytes 'F', 'T' (financial tools)
  - format version for further migrations

| Bytes |  0  |  1  |    2    |
|:-----:|:---:|:---:|:-------:|
|  Type |  u8 |  u8 |    u8   |
| Field | 'F' | 'T' | Version |

### Entry (transaction)

All of multi-byte values are Little Endian.

- `Date`, number of days from 1970-01-01, 2-bytes enough until mid. of 2149 year.
- `Kind` of transaction
- `Amount` of transferred monies
- `l`, bytes length of `Category`
- `Category`, name of transaction category, max 255 characters
  - should be enum also, but I want to demonstrate processing of variable-length data

| Bytes | 0..=1 |   2  | 3..=11 | 12 |  13..=l  |
|:-----:|:-----:|:----:|:------:|:--:|:--------:|
|  Type |  u16  |  u8  |   u64  | u8 |   &str   |
| Field |  Date | Kind | Amount |  l | Category |
