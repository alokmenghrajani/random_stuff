package positionqueue

// Queue implementation with position looking.
// Adding an item to the tail, removing an item from the head, and looking up position are all O(1) operations.
// This is a toy implementation and is not concurrent safe.

type Item struct {
	value interface{}
	next  *Item
	id    uint
}

type QueueInterface interface {
	add(e *Item)
	remove() *Item
	position(e Item) uint
}

func NewItem(value interface{}) Item {
	return Item{
		value: value,
		next:  nil,
		id:    0,
	}
}

func (i Item) Value() interface{} {
	return i.value
}

type Queue struct {
	head *Item
	tail *Item
}

var _ QueueInterface = &Queue{}

func NewQueue() *Queue {
	return &Queue{
		head: nil,
		tail: nil,
	}
}

func (q *Queue) add(e *Item) {
	// There are two cases here
	if q.tail != nil {
		// queue contains an item.
		assert(q.head != nil)
		e.id = q.tail.id + 1
		q.tail.next = e
		q.tail = e
	} else {
		// queue is empty
		assert(q.head == nil)
		q.tail = e
		q.head = e
		e.id = 1
	}
}

func (q *Queue) remove() *Item {
	if q.head == nil {
		// queue is empty, do nothing.
		assert(q.tail == nil)
		return nil
	}
	r := q.head
	q.head = r.next
	if q.head == nil {
		// queue has only one element, update tail
		q.tail = nil
	}
	// clean up r before returning it to avoid leaking memory
	r.id = 0
	r.next = nil
	return r
}

func (q *Queue) position(e Item) uint {
	if e.id == 0 {
		return 0
	}
	// r.id is 0 when an item is outside the queue and non-zero when it's in the queue.
	assert(q.head != nil)
	return e.id - q.head.id + 1
}

func assert(expr bool) {
	if !expr {
		panic("assertion failure")
	}
}
