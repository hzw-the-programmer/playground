https://go.dev/doc/effective_go

var p *[]int = new([]int)
 stack      heap
 +---+      +-----+
 | p | ---> | ptr | ---> array
 +---+      | len |
            | cap |
            +-----+

var v []int = make([]int, 100)
stack        heap
+-----+
| ptr | ---> array
| len |
| cap |
+-----+

var v []int
statck
+-----+
| ptr |
| len |
| cap |
+-----+

// Unnecessarily complex:
var p *[]int = new([]int)
*p = make([]int, 100, 100)

a pointer to a nil slice value
A slice, for example, is a three-item descriptor containing a pointer to the data (inside an array), the length, and the capacity, and until those items are initialized, the slice is nil

func Append(slice, data []byte) []byte {
    l := len(slice)
    if l + len(data) > cap(slice) {  // reallocate
        // Allocate double what's needed, for future growth.
        newSlice := make([]byte, (l+len(data))*2)
        // The copy function is predeclared and works for any slice type.
        copy(newSlice, slice)
        slice = newSlice
    }
    slice = slice[0:l+len(data)]
    copy(slice[l:], data)
    return slice
}

We must return the slice afterwards because, although Append can modify the elements of slice, the slice itself (the run-time data structure holding the pointer, length, and capacity) is passed by value.

// Allocate the top-level slice.
picture := make([][]uint8, YSize) // One row per unit of y.
// Loop over the rows, allocating the slice for each row.
for i := range picture {
	picture[i] = make([]uint8, XSize)
}

stack        heap
+-----+      +-----------------+-----------------+
| ptr | ---> | ptr | len | cap | ptr | len | cap |...
| len |      +-----------------+-----------------+
| cap |         |                 |
+-----+      +-----+           +-----+
             |     |           |     |
             +-----+           +-----+

// Allocate the top-level slice, the same as before.
picture := make([][]uint8, YSize) // One row per unit of y.
// Allocate one large slice to hold all the pixels.
pixels := make([]uint8, XSize*YSize) // Has type []uint8 even though picture is [][]uint8.
// Loop over the rows, slicing each row from the front of the remaining pixels slice.
for i := range picture {
	picture[i], pixels = pixels[:XSize], pixels[XSize:]
}

stack        heap
+-----+      +-----------------+-----------------+
| ptr | ---> | ptr | len | cap | ptr | len | cap | ***
| len |      +-----------------+-----------------+
| cap |         |                 |
+-----+         +------------------------------------
                |                 |                 | ***
                +------------------------------------