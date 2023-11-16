src\crypto\tls\conn.go
encrypt

cbdMode

record header (5 bytes)

record header (5 bytes) | explicitNonce

record header (5 bytes) | explicitNonce | payload | mac | padding |
                                        |      dst                |
                                        | plaintext len |