Overview:

- Encrypt files using AES-256 encryption algorithm.
- Generate secure encryption keys.
- Provide both encryption and decryption functionalities.
- Handle files of any size efficiently.

Commands:

```bash
cargo run generate-key -o key.bin
```

```bash
cargo run encrypt -i secrets.txt -o encrypted.bin -k key.bin
```

```bash
cargo run decrypt -i encrypted.bin -o decrypted.txt -k key.bin
```
