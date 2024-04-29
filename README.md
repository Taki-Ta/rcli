# Geektime Rust 语言训练营

##

### chacha20poly1305 encrypt text

```bash
rcli text encrypt --key  "32_bytes_length_key"
```

### chacha20poly1305 decrypt text

```bash
rcli text encrypt --key  "32_bytes_length_key"
```

### jwt sign

```bash
rcli jwt sign --sub "11" --aud "22" --exp 1d2h
```

### jwt verify

```bash
rcli jwt verify --aud "22" -t "your jwt"
```

### http file server

```bash
rcli http serve
```
