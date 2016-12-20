// I was going to write a linked-list but then I read Learning Rust With Entirely Too Many Linked
// Lists (http://cglab.ca/~abeinges/blah/too-many-lists/book/README.html) which is worth reading
// but also convinced me to use a Vec instead.
//
// Note: this puzzle is very similar to https://en.wikipedia.org/wiki/Josephus_problem. The first
// part can be solved without a computer using some pure math. I'm not too sure about the second
// part.

pub fn solve(input: usize) {
    assert_eq!(part1(5), 3);
    println!("part 1: {}", part1(input));

    assert_eq!(part2(5), 2);
    println!("part 2: {}", part2(input));
}

struct CircularBuffer {
    next: Vec<usize>,
    prev: Vec<usize>, // we could do without prev...
    curr: usize,
    opp: usize,
    size: usize,
}

impl CircularBuffer {
    fn new(size: usize) -> CircularBuffer {
        let mut next = Vec::with_capacity(size);
        let mut prev = Vec::with_capacity(size);
        for i in 0..size {
            next.push((i + 1) % size);
            prev.push((i + size - 1) % size);
        }
        CircularBuffer {
            next: next,
            prev: prev,
            curr: 0,
            opp: size / 2,
            size: size,
        }
    }

    fn remove_opp(&mut self) {
        assert!(self.size > 1);
        let next = self.next[self.opp];
        let prev = self.prev[self.opp];
        self.next[prev] = next;
        self.prev[next] = prev;
        self.next[self.opp] = 0;
        self.prev[self.opp] = 0;
        if self.size % 2 == 0 {
            self.opp = prev;
        } else {
            self.opp = next;
        }
        self.size -= 1;
    }

    fn remove_next(&mut self) {
        assert!(self.size > 1);
        let remove = self.next[self.curr];
        let next = self.next[remove];
        let prev = self.prev[remove];
        self.next[prev] = next;
        self.prev[next] = prev;
        self.next[remove] = 0;
        self.prev[remove] = 0;
        self.size -= 1;
    }

    fn inc(&mut self) {
        self.opp = self.next[self.opp];
        self.curr = self.next[self.curr];
    }
}

fn part1(input: usize) -> usize {
    _solve(input, true)
}

fn part2(input: usize) -> usize {
    _solve(input, false)
}

fn _solve(input: usize, part1: bool) -> usize {
    let mut elves = CircularBuffer::new(input);
    for _ in 1..input {
        if part1 {
            elves.remove_next();
        } else {
            elves.remove_opp();
        }
        elves.inc();
    }
    elves.curr + 1
}
