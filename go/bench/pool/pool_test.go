package pool

import (
	"sync"
	"testing"
)

type Person struct {
	Name string
	Age  int
}

var pool = sync.Pool{
	New: func() interface{} { return new(Person) },
}

func BenchmarkWithoutPool(b *testing.B) {
	var p *Person
	b.ReportAllocs()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		for j := 0; j < 10000; j++ {
			p = new(Person)
			p.Name = "hzw"
			p.Age = 35
		}
	}
}

func BenchmarkWithPool(b *testing.B) {
	b.ReportAllocs()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		for j := 0; j < 10000; j++ {
			p := pool.Get().(*Person)
			p.Name = "hzw"
			p.Age = 35
			pool.Put(p)
		}
	}
}
