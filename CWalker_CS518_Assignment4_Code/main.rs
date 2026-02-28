use assignment4::{List, XorLinkedList};

fn snapshot(list: &XorLinkedList<i32>) -> Vec<i32> {
    list.to_vec().into_iter().copied().collect()
}

fn print_state(label: &str, list: &XorLinkedList<i32>) {
    println!("{label:<14} len={}  {:?}", list.len(), snapshot(list));
}

fn main() {
    let mut xs = XorLinkedList::new();

    print_state("start", &xs);

    // push_front / push_back
    xs.push_back(2);
    print_state("push_back(2)", &xs);

    xs.push_front(1);
    print_state("push_front(1)", &xs);

    xs.push_back(3);
    print_state("push_back(3)", &xs);

    xs.push_back(4);
    print_state("push_back(4)", &xs);

    xs.push_back(5);
    print_state("push_back(5)", &xs);

    xs.push_front(0);
    print_state("push_front(0)", &xs);

    // insert
    let ok = xs.insert(1, 99);
    println!("insert(1,99)  -> {ok}");
    print_state("after insert", &xs);

    let ok = xs.insert(0, 42);
    println!("insert(0,42)  -> {ok}");
    print_state("after insert", &xs);

    let ok = xs.insert(xs.len(), 77);
    println!("insert(len,77)-> {ok}");
    print_state("after insert", &xs);

    let ok = xs.insert(999, 123);
    println!("insert(999,123)-> {ok}  (expected false)");
    print_state("after bad ins", &xs);

    // delete
    let d = xs.delete(2);
    println!("delete(2)     -> {d:?}");
    print_state("after delete", &xs);

    let d = xs.delete(0);
    println!("delete(0)     -> {d:?}");
    print_state("after delete", &xs);

    let d = xs.delete(0);
    println!("delete(0)     -> {d:?}");
    print_state("after delete", &xs);

    let d = xs.delete(xs.len().saturating_sub(1));
    println!("delete(last)  -> {d:?}");
    print_state("after delete", &xs);

    let d = xs.delete(999);
    println!("delete(999)   -> {d:?}  (expected None)");
    print_state("after bad del", &xs);

    // traverse demo: print and sum
    print!("traverse:      ");
    xs.traverse(|v| print!("{v} "));
    println!();

    let mut sum = 0;
    xs.traverse(|v| sum += *v);
    println!("sum via traverse = {sum}");
}