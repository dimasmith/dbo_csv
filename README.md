# DBO CSV

`dbo_csv` is a simple crate to read CSV documents of DBO format.

I'm planning to use it in the [monotax](https://github.com/dimasmith/monotax) project later.

This crate won't be published on crates.io - it's hardly useful to anyone except me.

## DBO format

First thing first - DBO is a company providing banking systems to a few banks in Ukraine.

The statement format isn't tricky, but a bit annoying to work with. It is a CSV file with `;` as a separator.
An annoying part is that it's encoded in `Windows-1251`. Yeah, it's 2025, and still not `UTF-8`, can you imagine?

## Implementation

The crate uses `serde` to deserialize the CSV file into a struct.

DBO uses the `%d.%m.%Y %H:%M:%S` date format. While it looks reasonable, it requires a custom deserializer.
