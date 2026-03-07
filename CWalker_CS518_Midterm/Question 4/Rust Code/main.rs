use std::collections::HashSet;
use std::hash::Hash;

// ---------- Policy abstraction ----------

trait SeenPolicy<T> {
    fn mark_if_new(&mut self, value: &T) -> bool;
}

// ---------- Policy implementations ----------

struct HashSeenPolicy<T> {
    seen: HashSet<T>,
}

impl<T> HashSeenPolicy<T>
where
    T: Eq + Hash,
{
    fn with_capacity(capacity: usize) -> Self {
        Self {
            seen: HashSet::with_capacity(capacity),
        }
    }
}

impl<T> SeenPolicy<T> for HashSeenPolicy<T>
where
    T: Eq + Hash + Clone,
{
    fn mark_if_new(&mut self, value: &T) -> bool {
        self.seen.insert(value.clone())
    }
}

struct LinearSeenPolicy<T> {
    seen: Vec<T>,
}

impl<T> LinearSeenPolicy<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            seen: Vec::with_capacity(capacity),
        }
    }
}

impl<T> SeenPolicy<T> for LinearSeenPolicy<T>
where
    T: PartialEq + Clone,
{
    fn mark_if_new(&mut self, value: &T) -> bool {
        if self.seen.contains(value) {
            false
        } else {
            self.seen.push(value.clone());
            true
        }
    }
}

// ---------- Core algorithm ----------

fn stable_dedup_with<T, P>(xs: &[T], mut policy: P) -> Vec<T>
where
    T: Clone,
    P: SeenPolicy<T>,
{
    let mut result = Vec::with_capacity(xs.len());

    for x in xs {
        if policy.mark_if_new(x) {
            result.push(x.clone());
        }
    }

    result
}

// ---------- Public APIs ----------

fn stable_dedup<T>(xs: &[T]) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    stable_dedup_with(xs, HashSeenPolicy::with_capacity(xs.len()))
}

fn stable_dedup_no_hash<T>(xs: &[T]) -> Vec<T>
where
    T: PartialEq + Clone,
{
    stable_dedup_with(xs, LinearSeenPolicy::with_capacity(xs.len()))
}

// ---------- Example type without Hash ----------

#[derive(Clone, PartialEq, Debug)]
struct Person {
    id: i32,
    name: String,
}

// ---------- Main ----------

fn main() {
    let nums = vec![3, 1, 3, 2, 1, 4, 2];
    let deduped_nums = stable_dedup(&nums);
    println!("Duped ints: {:?}", nums);
    println!("Deduped ints: {:?}", deduped_nums);
    println!("");

    let words = vec!["cat", "dog", "cat", "bird", "dog"];
    let deduped_words = stable_dedup(&words);
    println!("Duped strings: {:?}", words);
    println!("Deduped strings: {:?}", deduped_words);
    println!("");

    let people = vec![
        Person {
            id: 1,
            name: "Alice".to_string(),
        },
        Person {
            id: 2,
            name: "Bob".to_string(),
        },
        Person {
            id: 1,
            name: "Alice".to_string(),
        },
        Person {
            id: 3,
            name: "Cara".to_string(),
        },
    ];

    let deduped_people = stable_dedup_no_hash(&people);
    println!("Duped people (no hash): {:?}", people);
    println!("Deduped people (no hash): {:?}", deduped_people);
    println!("");
}