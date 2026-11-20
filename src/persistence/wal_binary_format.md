# Binary WAL format

The write-ahead log (WAL) stores a sequence of records, one per mutation. Replay reads them in order to rebuild the in-memory map.

## Record layout

Byte ranges are zero-based and exclude the right endpoint. `N` is the payload length in bytes.

| Byte range | Field | Encoding |
| --- | --- | --- |
| `0..2` | Payload length | Little-endian `u16`, counting only the payload bytes |
| `2..2+N` | Payload | Operation, key, and optional value as described below |
| `2+N..6+N` | Checksum | Little-endian `u32`, using CRC-32/ISO-HDLC over the stored length bytes followed by the payload |

The writer rejects payloads larger than 65,535 bytes before appending.

## Payload

Byte ranges are relative to the start of the payload and exclude the right endpoint. `X` is the key length in bytes.

| Byte range | Field | Encoding |
| --- | --- | --- |
| `0..1` | Operation | `0x01` for insert, `0x02` for delete, `0x03` for update |
| `1..3` | Key length | Little-endian `u16`, counting only the key bytes |
| `3..3+X` | Key | `X` UTF-8 bytes |
| `3+X..N` | Value | Remaining `N - 3 - X` UTF-8 bytes for insert or update |

The reader checks that the payload contains at least three bytes and that `X` is at most `N - 3` before reading the key. For delete, `N` must equal `3 + X`. For insert and update, zero remaining bytes represent an empty value.

## Writing and recovery

Validate the mutation, append the record, and call `sync_data()` before changing the map or reporting success. If the append or sync fails, stop accepting writes until the WAL has been checked.

On startup, verify each complete record and apply it directly to the map in log order before accepting new writes. If the file ends between records, replay is complete. If the file ends partway through a record, discard that incomplete tail by truncating to the end of the last verified record before allowing another append. A complete record with a bad checksum or invalid payload stops recovery with an error.

The checksum detects corruption. Recovery rebuilds the map from surviving records. Any operation whose bytes were lost is unrecoverable from the WAL.

Use a separate file for the binary WAL so existing text records in `WAL.txt` are never mixed with binary records.
