use std::cmp::Ordering;

pub struct PriorityQueue<T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    items: Vec<T>,
    cmp: F,
}

impl<T, F> PriorityQueue<T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    pub fn new(cmp: F) -> Self {
        Self {
            items: Vec::new(),
            cmp,
        }
    }

    // Add an item in the correct sorted position.
    // Items are stored from lowest priority to highest priority.
    pub fn push(&mut self, item: T) {
        let mut index = 0;

        while index < self.items.len()
            && (self.cmp)(&self.items[index], &item) != Ordering::Greater
        {
            index += 1;
        }

        self.items.insert(index, item);
    }

    // Remove and return the highest-priority item.
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // Borrow the highest-priority item without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    // Return true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // Return the number of items in the queue.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    // Let the caller traverse items from highest priority to lowest priority.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter().rev()
    }
}

fn main() {
    let mut pq = PriorityQueue::new(|a: &i32, b: &i32| a.cmp(b));

    pq.push(4);
    pq.push(1);
    pq.push(9);
    pq.push(3);

    println!("Queue contents in priority order:");
    for item in pq.iter() {
        println!("{item}");
    }

    println!("Popped: {:?}", pq.pop());

    println!("After pop:");
    for item in pq.iter() {
        println!("{item}");
    }

    #[derive(Debug)]
    struct Task {
        name: String,
        priority: i32,
    }

    let mut task_q = PriorityQueue::new(|a: &Task, b: &Task| a.priority.cmp(&b.priority));

    task_q.push(Task {
        name: "Wash dishes".to_string(),
        priority: 2,
    });
    task_q.push(Task {
        name: "Finish project".to_string(),
        priority: 10,
    });
    task_q.push(Task {
        name: "Check email".to_string(),
        priority: 4,
    });

    println!("\nTasks in priority order:");
    for task in task_q.iter() {
        println!("{} (priority {})", task.name, task.priority);
    }

    if let Some(task) = task_q.pop() {
        println!("Popped task: {} (priority {})", task.name, task.priority);
    }
}