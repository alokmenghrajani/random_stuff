package positionqueue

// Queue implementation with position looking.
// Adding an item to the tail, removing an item from the head, and looking up position are all O(1) operations.
// This toy implementation is not concurrent safe.

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
	added   uint
	removed uint
	head    *Item
	tail    *Item
}

var _ QueueInterface = &Queue{}

func NewQueue() *Queue {
	return &Queue{
		added:   0,
		removed: 0,
		head:    nil,
		tail:    nil,
	}
}

func (q *Queue) add(e *Item) {
	q.added++
	e.id = q.added
	if q.tail != nil {
		q.tail.next = e
		q.tail = e
	} else {
		q.tail = e
		q.head = e
	}
}

func (q *Queue) remove() *Item {
	if q.head == nil {
		return nil
	}
	r := q.head
	q.head = r.next
	if q.head == nil {
		q.tail = nil
	}
	q.removed++
	// clean up r before returning it to avoid leaking memory
	r.id = 0
	r.next = nil
	return r
}

func (q *Queue) position(e Item) uint {
	if e.id == 0 {
		return 0
	}
	return e.id - q.removed
}
