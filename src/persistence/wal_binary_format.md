# Binary WAL format

The write-ahead log (WAL) stores a sequence of records, one per mutation. Replay reads them in order to rebuild the in-memory map.

## Record layout

Byte ranges are zero-based and exclude the right endpoint. `N` is the payload length in bytes.

| Byte range | Field | Encoding |
| --- | --- | --- |
| `0..2` | Payload length | Little-endian `u16`, counting only the payload bytes |
| `2..2+N` | Payload | Operation, key, and optional value as described below |
| `2+N..6+N` | Checksum | Little-endian `u32`, using CRC-32/ISO-HDLC over the stored length bytes followed by the payload |

The payload starts with a one-byte operation code (`0x01` for insert, `0x02` for update, `0x03` for delete), followed by a little-endian `u16` key length and that many UTF-8 key bytes. For insert and update, the remaining payload bytes are the UTF-8 value. For delete, the payload ends immediately after the key. Lengths count bytes. The writer rejects payloads larger than 65,535 bytes before appending.

## Writing and recovery

Validate the mutation, append the record, and call `sync_data()` before changing the map or reporting success. If the append or sync fails, stop accepting writes until the WAL has been checked.

On startup, verify each complete record and apply it directly to the map in log order before accepting new writes. If the file ends between records, replay is complete. If the file ends partway through a record, discard that incomplete tail by truncating to the end of the last verified record before allowing another append. A complete record with a bad checksum or invalid payload stops recovery with an error.

The checksum detects corruption. Recovery rebuilds the map from surviving records. Any operation whose bytes were lost is unrecoverable from the WAL.

Use a separate file for the binary WAL so existing text records in `WAL.txt` are never mixed with binary records.
