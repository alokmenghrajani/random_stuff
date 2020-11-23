package positionqueue

import (
	"fmt"
)

// Example usage
func Example() {
	q := NewQueue()
	a := NewItem("aaa")
	b := NewItem("bbb")
	c := NewItem("ccc")
	d := NewItem("ddd")
	e := NewItem("eee")
	f := NewItem("fff")
	g := NewItem("ggg")

	q.add(&a) // [a]
	q.add(&b) // [a, b]
	fmt.Printf("pos a: %d\n", q.position(a))
	fmt.Printf("pos b: %d\n", q.position(b))
	fmt.Printf("head: %s\n", q.remove().Value()) // [b]
	fmt.Printf("pos b: %d\n", q.position(b))
	fmt.Printf("head: %s\n", q.remove().Value()) // []
	q.add(&c)                                    // [c]
	q.add(&d)                                    // [c, d]
	q.add(&e)                                    // [c, d, e]
	q.add(&f)                                    // [c, d, e, f]
	q.add(&g)                                    // [c, d, e, f, g]
	fmt.Printf("pos c: %d\n", q.position(c))
	fmt.Printf("pos d: %d\n", q.position(d))
	fmt.Printf("pos e: %d\n", q.position(e))
	fmt.Printf("pos f: %d\n", q.position(f))
	fmt.Printf("pos g: %d\n", q.position(g))
	fmt.Printf("head: %s\n", q.remove().Value()) // [d, e, f, g]
	fmt.Printf("head: %s\n", q.remove().Value()) // [e, f, g]
	fmt.Printf("pos e: %d\n", q.position(e))
	fmt.Printf("pos f: %d\n", q.position(f))
	fmt.Printf("pos g: %d\n", q.position(g))

	// Output:
	// pos a: 1
	// pos b: 2
	// head: aaa
	// pos b: 1
	// head: bbb
	// pos c: 1
	// pos d: 2
	// pos e: 3
	// pos f: 4
	// pos g: 5
	// head: ccc
	// head: ddd
	// pos e: 1
	// pos f: 2
	// pos g: 3
}
