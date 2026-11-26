| n_ctx | the device's free memory before the model (MiB) | once the context is made (MiB) | the model and its context (MiB) |
|---:|---:|---:|---:|
| 4096 | 7685 | 4479 | 3206 |
| 16384 | 7685 | 4077 | 3608 |
| 32768 | 7685 | 3555 | 4130 |
| 65536 | 7685 | 2499 | 5186 |
| 131072 | 7685 | 387 | 7298 |

| prompt tokens | prompt ms | prompt tokens/s | generated | generation ms | generated tokens/s |
|---:|---:|---:|---:|---:|---:|
| 1211 | 485 | 2498 | 256 | 3826 | 66.9 |
| 5498 | 2084 | 2639 | 256 | 4183 | 61.2 |
| 13745 | 5689 | 2416 | 256 | 4563 | 56.1 |
| 30239 | 14226 | 2126 | 256 | 4983 | 51.4 |
| 54980 | 29484 | 1865 | 256 | 6241 | 41.0 |
