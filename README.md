# fin_tools

Yandex Practicum's Middle Rust Course. Project of 1st Module: Reading, Parsing, and Analyzing Data in Rust

## Binary data layout

### Header

- 3-byte signature:
  - fixed bytes 'F', 'T' (financial tools)
  - format version for further migrations
- `N`, number of entries (max 65 535)

| Bytes |  0  |  1  |    2    | 3..4 |
|:-----:|:---:|:---:|:-------:|:----:|
|  Type |  u8 |  u8 |    u8   |  u16 |
| Field | 'F' | 'T' | Version |   N  |

### Entry (transaction)

- `Date`, number of days from 1970-01-01, 2-bytes enough until mid. of 2149 year
- `l`, bytes length of `Category`
- `Category`, name of transaction category
  - forced ASCII, since I don't want to struggle with Unicode Normalization Forms and other pitfalls during subsequent string comparison
- `Kind` of transaction
- `Amount` of transferred monies

| Bytes | 0..1 |  2 |   3..l   |  l+1 | l+2..l+10 |
|:-----:|:----:|:--:|:--------:|:----:|-----------|
|  Type |  u16 | u8 |   &str   |  u8  | u64       |
| Field | Date |  l | Category | Kind | Amount    |
