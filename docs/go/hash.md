golang标准库hash接口定义：src\hash\hash.go
```
type Hash interface {
	io.Writer
	Sum(b []byte) []byte
	Reset()
	Size() int
	BlockSize() int
}

type Hash32 interface {
	Hash
	Sum32() uint32
}

type Hash64 interface {
	Hash
	Sum64() uint64
}
```

标准库实现

hash包

adler32: src\hash\adler32\adler32.go
crc32: src\hash\crc32\crc32.go
crc64: src\hash\crc64\crc64.go
fnv: src\hash\fnv\fnv.go
maphash: src\hash\maphash\maphash.go

src\crypto\crypto.go列出了crypto包及golang.org/x/crypto包实现的Hash接口
crypto包

md5: src\crypto\md5\md5.go
sha1: src\crypto\sha1\sha1.go
sha256: src\crypto\sha256\sha256.go
SHA224
SHA256
sha512: src\crypto\sha512\sha512.go
SHA384
SHA512
SHA512_224
SHA512_256

golang.org/x/crypto 包

md4: crypto\md4\md4.go
sha3: crypto\sha3\sha3.go
SHA3_224
SHA3_256
SHA3_384
SHA3_512 

blake2s: crypto\blake2s\blake2s.go
BLAKE2s_256

blake2b: crypto\blake2b\blake2b.go
BLAKE2b_256
BLAKE2b_384
BLAKE2b_512

ripemd160: crypto\ripemd160\ripemd160.go
RIPEMD160