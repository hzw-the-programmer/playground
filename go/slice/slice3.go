package main

import(
	"log"
)

func main() {
	s1 := []byte{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	log.Print(len(s1), cap(s1))
	log.Print(s1)

	s2 := s1[:5]
	log.Print(len(s2), cap(s2))
	log.Print(s2)

	s2 = append(s2, 10, 11)
	log.Print(len(s2), cap(s2))
	log.Print(s2)
	log.Print(s1)

	s3 := s1[:5:5]
	log.Print(len(s3), cap(s3))
	s3 = append(s3, 12, 13)
	log.Print(len(s3), cap(s3))
	log.Print(s3)
	log.Print(s1)
}