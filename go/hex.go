package main

import (
	"math"
	"encoding/binary"
	"encoding/hex"
	"bytes"
	"os"
	"strconv"
	"fmt"
)

func main() {
	fmt.Printf("%T\n", os.Args[1])

	a, _ := strconv.ParseFloat(os.Args[1], 32)
	fmt.Printf("%T\n", a)
	b := float32(a)
	fmt.Printf("%T\n", b)

	f := make([]byte, 10)
	e := math.Float32bits(b)
	binary.LittleEndian.PutUint32(f, e)
	fmt.Println(hex.EncodeToString(f[:4]))

	g := binary.LittleEndian.Uint32(f)
	h := math.Float32frombits(g)
	fmt.Println(h)

	i := &bytes.Buffer{}
	fmt.Println(i.Len(), i.Cap())
	binary.Write(i, binary.LittleEndian, b)
	fmt.Println(i.Len(), i.Cap())
	j := i.Bytes();
	fmt.Println(hex.EncodeToString(j))
}
