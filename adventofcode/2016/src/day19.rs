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
    next: Vec<Option<usize>>,
    curr: usize,
    // the position for the opposite is next[opp]. We need one level of indirection so we can
    // remove the opposite element.
    opp: usize,
    size: usize,
}

impl CircularBuffer {
    fn new(size: usize) -> CircularBuffer {
        let mut next = Vec::with_capacity(size);
        for i in 0..size {
            next.push(Some((i + 1) % size));
        }
        CircularBuffer {
            next: next,
            curr: 0,
            opp: size / 2 - 1,
            size: size,
        }
    }

    fn remove_opp(&mut self) {
        assert!(self.size > 1);
        let remove = self.next[self.opp].unwrap();
        let next = self.next[remove].unwrap();

        self.next[self.opp] = Some(next);
        self.next[remove] = None;
        if self.size % 2 == 1 {
            // move forward
            self.opp = next;
        }
        self.curr = self.next[self.curr].unwrap();
        self.size -= 1;
    }

    fn remove_next(&mut self) {
        assert!(self.size > 1);
        let remove = self.next[self.curr].unwrap();
        let next = self.next[remove].unwrap();
        self.next[self.curr] = Some(next);
        self.next[remove] = None;
        self.curr = self.next[self.curr].unwrap();
        self.size -= 1;
    }
}

fn part1(input: usize) -> usize {
    _solve(input, |x| x.remove_next())
}

fn part2(input: usize) -> usize {
    _solve(input, |x| x.remove_opp())
}

fn _solve<F>(input: usize, closure: F) -> usize
    where F: Fn(&mut CircularBuffer) -> ()
{
    let mut elves = CircularBuffer::new(input);
    for _ in 1..input {
        closure(&mut elves);
    }
    elves.curr + 1
}
