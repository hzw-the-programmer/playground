error:

```
go: code.hzw.org/go/ml: code.hzw.org/go/ml@v0.0.1/go.mod: verifying module: code.hzw.org/go/ml@v0.0.1/go.mod: reading https://sum.golang.org/lookup/code.hzw.org/go/ml@v0.0.1: 410 Gone
        server response: not found: code.hzw.org/go/ml@v0.0.1: unrecognized import path "code.hzw.org/go/ml": https fetch: Get "https://code.hzw.org/go/ml?go-get=1": dial tcp: lookup code.hzw.org on 8.8.8.8:53: no such host
```

solve:
https://golang.org/cmd/go/#hdr-Module_configuration_for_non_public_modules
```
go env -w GOPRIVATE="*.hzw.org"
```
