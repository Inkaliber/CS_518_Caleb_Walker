use std::array;
use std::sync::Arc;

// =========================
// Core persistent stack
// =========================

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Arc<Node<T>>>,
}

#[derive(Debug)]
pub struct PersistentStack<T> {
    head: Option<Arc<Node<T>>>,
    len: usize,
}

impl<T> Clone for PersistentStack<T> {
    fn clone(&self) -> Self {
        Self {
            head: self.head.clone(),
            len: self.len,
        }
    }
}

impl<T> Default for PersistentStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PersistentStack<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    fn from_head(head: Option<Arc<Node<T>>>, len: usize) -> Self {
        Self { head, len }
    }

    pub fn push(&self, value: T) -> Self {
        let new_head = Arc::new(Node {
            value,
            next: self.head.clone(),
        });

        Self::from_head(Some(new_head), self.len + 1)
    }

    pub fn pop(&self) -> Option<(T, Self)>
    where
        T: Clone,
    {
        let head = self.head.as_ref()?;
        let value = head.value.clone();
        let next_stack = Self::from_head(head.next.clone(), self.len - 1);

        Some((value, next_stack))
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.value)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn snapshot(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut values = Vec::with_capacity(self.len);
        let mut current = self.head.as_deref();

        while let Some(node) = current {
            values.push(node.value.clone());
            current = node.next.as_deref();
        }

        values
    }
}

// =========================
// History abstraction
// =========================

pub trait HistoryPolicy<S>: Clone {
    fn new(initial: S) -> Self;
    fn record(&self, version: S) -> Self;
    fn rotate(&self) -> Self;
    fn current(&self) -> S;
}

// =========================
// Circular fixed-history policy
// =========================

#[derive(Clone, Debug)]
pub struct CircularHistory<S, const HISTORY: usize> {
    slots: [Option<S>; HISTORY],
    newest: usize,
    cursor: usize,
    count: usize,
}

impl<S: Clone, const HISTORY: usize> HistoryPolicy<S> for CircularHistory<S, HISTORY> {
    fn new(initial: S) -> Self {
        assert!(HISTORY > 0, "HISTORY must be greater than 0");

        let mut slots = array::from_fn(|_| None);
        slots[0] = Some(initial);

        Self {
            slots,
            newest: 0,
            cursor: 0,
            count: 1,
        }
    }

    fn record(&self, version: S) -> Self {
        let mut slots = self.slots.clone();

        if self.count < HISTORY {
            let next = self.count;
            slots[next] = Some(version);

            Self {
                slots,
                newest: next,
                cursor: next,
                count: self.count + 1,
            }
        } else {
            let next = (self.newest + 1) % HISTORY;
            slots[next] = Some(version);

            Self {
                slots,
                newest: next,
                cursor: next,
                count: HISTORY,
            }
        }
    }

    fn rotate(&self) -> Self {
        let next_cursor = if self.count <= 1 {
            self.cursor
        } else if self.count < HISTORY {
            if self.cursor == 0 {
                self.count - 1
            } else {
                self.cursor - 1
            }
        } else {
            (self.cursor + HISTORY - 1) % HISTORY
        };

        Self {
            slots: self.slots.clone(),
            newest: self.newest,
            cursor: next_cursor,
            count: self.count,
        }
    }

    fn current(&self) -> S {
        self.slots[self.cursor]
            .as_ref()
            .expect("history always has a current version")
            .clone()
    }
}

// =========================
// Versioned stack interface
// =========================

pub trait VersionedStackOps<T>: Sized {
    fn push(&self, value: T) -> Self;
    fn pop(&self) -> Option<(T, Self)>
    where
        T: Clone;
    fn peek(&self) -> Option<&T>;
    fn version(&self) -> Self;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// =========================
// Versioned persistent stack
// =========================

#[derive(Debug)]
pub struct PersistentVersionedStack<T, H>
where
    H: HistoryPolicy<PersistentStack<T>>,
{
    current: PersistentStack<T>,
    history: H,
}

impl<T, H> Clone for PersistentVersionedStack<T, H>
where
    H: HistoryPolicy<PersistentStack<T>>,
{
    fn clone(&self) -> Self {
        Self {
            current: self.current.clone(),
            history: self.history.clone(),
        }
    }
}

impl<T, H> Default for PersistentVersionedStack<T, H>
where
    H: HistoryPolicy<PersistentStack<T>>,
{
    fn default() -> Self {
        Self::new()
    }
}

pub type FixedHistoryStack<T, const HISTORY: usize> =
    PersistentVersionedStack<T, CircularHistory<PersistentStack<T>, HISTORY>>;

impl<T, H> PersistentVersionedStack<T, H>
where
    H: HistoryPolicy<PersistentStack<T>>,
{
    pub fn new() -> Self {
        let current = PersistentStack::new();
        let history = H::new(current.clone());

        Self { current, history }
    }

    pub fn snapshot(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.current.snapshot()
    }
}

impl<T, H> VersionedStackOps<T> for PersistentVersionedStack<T, H>
where
    H: HistoryPolicy<PersistentStack<T>>,
{
    fn push(&self, value: T) -> Self {
        let next_stack = self.current.push(value);
        let next_history = self.history.record(next_stack.clone());

        Self {
            current: next_stack,
            history: next_history,
        }
    }

    fn pop(&self) -> Option<(T, Self)>
    where
        T: Clone,
    {
        let (value, next_stack) = self.current.pop()?;
        let next_history = self.history.record(next_stack.clone());

        Some((
            value,
            Self {
                current: next_stack,
                history: next_history,
            },
        ))
    }

    fn peek(&self) -> Option<&T> {
        self.current.peek()
    }

    fn version(&self) -> Self {
        let rotated_history = self.history.rotate();
        let rotated_stack = rotated_history.current();

        Self {
            current: rotated_stack,
            history: rotated_history,
        }
    }

    fn len(&self) -> usize {
        self.current.len()
    }
}

// =========================
// Demo
// =========================

fn main() {
    type DemoStack = FixedHistoryStack<i32, 4>;

    let v0 = DemoStack::new();
    let v1 = v0.push(10);
    let v2 = v1.push(20);
    let v3 = v2.push(30);

    println!("v0 = {:?}", v0.snapshot());
    println!("v1 = {:?}", v1.snapshot());
    println!("v2 = {:?}", v2.snapshot());
    println!("v3 = {:?}", v3.snapshot());
    println!("peek(v3) = {:?}", v3.peek());

    let (removed, v4) = v3.pop().expect("stack is not empty");
    println!("popped from v3 = {}", removed);
    println!("v4 after pop = {:?}", v4.snapshot());

    println!("v3 still intact = {:?}", v3.snapshot());

    let r1 = v4.version();
    let r2 = r1.version();
    let r3 = r2.version();
    let r4 = r3.version();

    println!("rotate 1 = {:?}", r1.snapshot());
    println!("rotate 2 = {:?}", r2.snapshot());
    println!("rotate 3 = {:?}", r3.snapshot());
    println!("rotate 4 = {:?}", r4.snapshot());
}