use alloc::boxed::Box;

pub struct Game {

}
impl Game {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self) {
    }
}

struct Player {
    length: u8,
    body: LinkedList<(usize, usize)>,
    direction: Direction,
}
impl Player {
    pub fn new(width: usize, height: usize) -> Self {
        let mut body = LinkedList::new((height / 2, width / 2));
        body.push_front((height / 2 - 1, width / 2));
        Self {
            length: 2,
            body,
            direction: Direction::Up,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct LinkedList<T> {
    head: Box<Node<T>>,
}
impl<T> LinkedList<T> {
    pub fn new(head_val: T) -> LinkedList<T> {
        LinkedList::<T> {
            head: Box::new(Node::new(head_val)),
        }
    }

    pub fn push_front(&mut self, val: T) {
        let mut new_head = Box::new(Node::new(val));
        self.head.prev = Some(new_head);
        new_head.next = Some(self.head.prev);
        self.head = new_head;
    }

    pub fn push_back(&mut self, val: T) {
        self.head.push_back(val);
    }
}

#[derive(Clone)]
struct Node<T> {
    val: T,
    prev: Option<Box<Node<T>>>,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    pub fn new(val: T) -> Node<T> {
        Node::<T> {
            val,
            prev: Option::<T>::None,
            next: Option::<T>::None,
        }
    }

    pub fn take(mut self, n: usize) -> Node<T> {
        if let None = self.next {
            return self;
        }
        if n == 0 {
            return self;
        }
        self.next = self.next.unwrap().take(n - 1);
        self
    }

    pub fn map<Ret>(&self, f: &dyn Fn(T) -> Ret) -> Node<Ret> {
        let mut rv = Node::<Ret>::new(f(self.val));
        if let Some(next) = self.next {
            rv.next = next.map(f);
        }
        rv
    }

    pub fn push_back(&mut self, val: T) {
        if let None = self.next {
            self.next = Some(Box::new(Node::new(val)));
            return;
        }
        self.next.push_back(val);
    }
}
