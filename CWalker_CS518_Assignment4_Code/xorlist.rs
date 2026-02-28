use std::ptr;

/// Public “list protocol” trait (matches your assignment wording).
/// (You can rename it to whatever your course expects.)
pub trait List<T> {
    fn push_front(&mut self, item: T);
    fn push_back(&mut self, item: T);

    /// Insert `item` so it becomes the element at `index` (0..=len).
    /// Returns `true` if inserted, `false` if `index` is out of range.
    fn insert(&mut self, index: usize, item: T) -> bool;

    /// Delete and return the element at `index` (0..len-1).
    fn delete(&mut self, index: usize) -> Option<T>;

    /// Apply `f` to each element from head -> tail.
    fn traverse<'a, F>(&'a self, f: F)
    where
    T: 'a,
    F: FnMut(&'a T);

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// One node in an XOR list.
/// `link` stores (prev_addr XOR next_addr).
struct Node<T> {
    elem: Option<T>,
    link: usize,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node {
            elem: Some(elem),
            link: 0,
        }
    }
}

/// XOR linked list using an arena (`Vec<Box<Node<T>>>`) to keep nodes pinned in memory.
pub struct XorLinkedList<T> {
    arena: Vec<Box<Node<T>>>,
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> Default for XorLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> XorLinkedList<T> {
    pub fn new() -> Self {
        XorLinkedList {
            arena: Vec::new(),
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    #[inline]
    fn addr(p: *mut Node<T>) -> usize {
        p as usize
    }

    /// Given (prev, curr), compute next = prev_addr XOR curr.link
    #[inline]
    unsafe fn next_ptr(&self, prev: *mut Node<T>, curr: *mut Node<T>) -> *mut Node<T> {
        let link = unsafe { (*curr).link };   // <-- explicit unsafe deref
        let next_addr = Self::addr(prev) ^ link;
        next_addr as *mut Node<T>
    }

    /// Walk to `index` and return (prev, curr), where curr is the node at `index`.
    ///
    /// If `index == len`, curr will be null and prev will be tail (or null if empty).
    fn find_prev_curr(&self, index: usize) -> (*mut Node<T>, *mut Node<T>) {
        let mut prev = ptr::null_mut();
        let mut curr = self.head;

        // After k steps: curr is the node at position k (or null if k==len),
        // and prev is the node at position k-1 (or null if k==0).
        for _ in 0..index {
            if curr.is_null() {
                return (ptr::null_mut(), ptr::null_mut());
            }
            let next = unsafe { self.next_ptr(prev, curr) };
            prev = curr;
            curr = next;
        }
        (prev, curr)
    }

    /// Convenience: collect items into a Vec (useful for debugging/tests).
    pub fn to_vec<'a>(&'a self) -> Vec<&'a T>
    where
    T: 'a,
    {
        let mut out = Vec::new();
        self.traverse(|v| out.push(v));
        out
    }
}

impl<T> List<T> for XorLinkedList<T> {
    fn push_front(&mut self, item: T) {
        // Allocate node and get stable pointer.
        let mut boxed = Box::new(Node::new(item));
        let new_ptr: *mut Node<T> = &mut *boxed;

        // new.link = null XOR old_head
        boxed.link = Self::addr(self.head);

        self.arena.push(boxed);

        unsafe {
            if !self.head.is_null() {
                // old_head was head, so its prev was null.
                // Update old_head.link: (null XOR next) -> (new XOR next)
                (*self.head).link ^= Self::addr(new_ptr);
            } else {
                // Empty list: tail also becomes new
                self.tail = new_ptr;
            }
        }

        self.head = new_ptr;
        self.len += 1;
    }

    fn push_back(&mut self, item: T) {
        let mut boxed = Box::new(Node::new(item));
        let new_ptr: *mut Node<T> = &mut *boxed;

        // new.link = old_tail XOR null
        boxed.link = Self::addr(self.tail);

        self.arena.push(boxed);

        unsafe {
            if !self.tail.is_null() {
                // old_tail was tail, so its next was null.
                // Update old_tail.link: (prev XOR null) -> (prev XOR new)
                (*self.tail).link ^= Self::addr(new_ptr);
            } else {
                // Empty list: head also becomes new
                self.head = new_ptr;
            }
        }

        self.tail = new_ptr;
        self.len += 1;
    }

    fn insert(&mut self, index: usize, item: T) -> bool {
        if index > self.len {
            return false;
        }
        if index == 0 {
            self.push_front(item);
            return true;
        }
        if index == self.len {
            self.push_back(item);
            return true;
        }

        // Insert between prev and curr (both non-null here).
        let (prev, curr) = self.find_prev_curr(index);
        if prev.is_null() || curr.is_null() {
            return false; // should not happen if len/index checks are correct
        }

        let mut boxed = Box::new(Node::new(item));
        let new_ptr: *mut Node<T> = &mut *boxed;

        // new.link = prev XOR curr
        boxed.link = Self::addr(prev) ^ Self::addr(curr);

        self.arena.push(boxed);

        unsafe {
            // prev.link currently = (prev_prev XOR curr)
            // replace curr with new: prev.link ^= curr ^ new
            (*prev).link ^= Self::addr(curr) ^ Self::addr(new_ptr);

            // curr.link currently = (prev XOR curr_next)
            // replace prev with new: curr.link ^= prev ^ new
            (*curr).link ^= Self::addr(prev) ^ Self::addr(new_ptr);
        }

        self.len += 1;
        true
    }

    fn delete(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        let (prev, curr) = self.find_prev_curr(index);
        if curr.is_null() {
            return None;
        }

        let next = unsafe { self.next_ptr(prev, curr) };

        unsafe {
            // Relink prev -> next (removing curr)
            if !prev.is_null() {
                // prev.link ^= curr ^ next
                (*prev).link ^= Self::addr(curr) ^ Self::addr(next);
            } else {
                // Removing head
                self.head = next;
            }

            // Relink next -> prev
            if !next.is_null() {
                // next.link ^= curr ^ prev
                (*next).link ^= Self::addr(curr) ^ Self::addr(prev);
            } else {
                // Removing tail
                self.tail = prev;
            }

            // Take the element out (node stays allocated in arena).
            let taken = (*curr).elem.take();

            // Optional: scrub link so accidental use is easier to detect in debug.
            (*curr).link = 0;

            self.len -= 1;
            taken
        }
    }

    fn traverse<'a, F>(&'a self, mut f: F)
    where
    T: 'a,
    F: FnMut(&'a T),
    {
        let mut prev = ptr::null_mut();
        let mut curr = self.head;

        while !curr.is_null() {
            unsafe {
                // In a correct list, elem should always be Some for reachable nodes.
                if let Some(ref v) = (*curr).elem {
                    f(v);
                } else {
                    // If you ever hit this, something was deleted but still reachable.
                    debug_assert!(false, "reachable node had elem=None");
                }

                let next = self.next_ptr(prev, curr);
                prev = curr;
                curr = next;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

/* ------------------------- Use-cases + tests ------------------------- */

#[cfg(test)]
mod tests {
    use super::{List, XorLinkedList};

    #[test]
    fn push_front_and_back_order() {
        let mut xs = XorLinkedList::new();
        xs.push_back(2);
        xs.push_front(1);
        xs.push_back(3);

        let got: Vec<i32> = xs.to_vec().into_iter().copied().collect();
        assert_eq!(got, vec![1, 2, 3]);
        assert_eq!(xs.len(), 3);
    }

    #[test]
    fn insert_middle() {
        let mut xs = XorLinkedList::new();
        xs.push_back('A');
        xs.push_back('C');
        assert!(xs.insert(1, 'B'));

        let got: String = xs.to_vec().into_iter().copied().collect();
        assert_eq!(got, "ABC");
    }

    #[test]
    fn insert_bounds() {
        let mut xs = XorLinkedList::new();
        assert!(xs.insert(0, 10)); // empty insert ok
        assert!(!xs.insert(2, 99)); // out of range
        let got: Vec<i32> = xs.to_vec().into_iter().copied().collect();
        assert_eq!(got, vec![10]);
    }

    #[test]
    fn delete_head_tail_middle() {
        let mut xs = XorLinkedList::new();
        for v in 0..5 {
            xs.push_back(v);
        }
        assert_eq!(xs.delete(0), Some(0)); // delete head
        assert_eq!(xs.delete(xs.len() - 1), Some(4)); // delete tail
        assert_eq!(xs.delete(1), Some(2)); // delete middle (current list: 1,2,3 -> after head/tail removed: 1,2,3; delete index 1 => 2)

        let got: Vec<i32> = xs.to_vec().into_iter().copied().collect();
        assert_eq!(got, vec![1, 3]);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn traverse_can_apply_closure() {
        let mut xs = XorLinkedList::new();
        xs.push_back(1);
        xs.push_back(2);
        xs.push_back(3);

        let mut sum = 0;
        xs.traverse(|v| sum += *v);
        assert_eq!(sum, 6);
    }

    #[test]
    fn works_with_owned_types() {
        let mut xs = XorLinkedList::new();
        xs.push_back(String::from("hello"));
        xs.push_back(String::from("world"));
        assert_eq!(xs.delete(0), Some(String::from("hello")));

        let got: Vec<String> = xs.to_vec().into_iter().map(|s| s.clone()).collect();
        assert_eq!(got, vec![String::from("world")]);
    }

    #[test]
    fn visual_proof_run() {
        let mut xs = XorLinkedList::new();
        
        xs.push_back(2);
        xs.push_front(1);
        xs.push_back(3);
        eprintln!("start     = {:?}", xs.to_vec().into_iter().copied().collect::<Vec<_>>());
        
        xs.insert(1, 99);
        eprintln!("insert(1) = {:?}", xs.to_vec().into_iter().copied().collect::<Vec<_>>());
        
        let removed = xs.delete(2);
        eprintln!(
            "delete(2) = {:?}, list = {:?}",
            removed,
            xs.to_vec().into_iter().copied().collect::<Vec<_>>()
        );
        
        assert_eq!(xs.to_vec().into_iter().copied().collect::<Vec<_>>(), vec![1, 99, 3]);
    }
}