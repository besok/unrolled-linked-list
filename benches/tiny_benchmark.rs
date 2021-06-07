use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use unrolled_linked_list::UnrolledLinkedList;
use std::collections::LinkedList;

#[derive(Clone)]
struct TestStruct {
    id: i32,
    name: String,
}

impl TestStruct {
    fn new(id: i32) -> Self {
        TestStruct { id, name: "name_name_name".to_string() }
    }
}

pub fn push_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("push");


    group.bench_function("unrolled_linked_list", |b| b.iter(||
        {
            let mut unrolled_list = UnrolledLinkedList::<TestStruct>::new();
            for el in 1..100 {
                unrolled_list.push(black_box(TestStruct::new(el)))
            }
        }
    ));
    group.bench_function("vec", |b| b.iter(||
        {
            let mut v = vec![];
            for el in 1..100 {
                v.push(black_box(TestStruct::new(el)))
            }
        }
    ));
    group.bench_function("linked_list", |b| b.iter(||
        {
            let mut linked_list = LinkedList::<TestStruct>::new();
            for el in 1..100 {
                linked_list.push_back(black_box(TestStruct::new(el)))
            }
        }
    ));
    group.finish();
}

pub fn pop_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop");

    group.bench_function("unrolled_linked_list", |b| b.iter(||
        {
            let mut unrolled_list = UnrolledLinkedList::<TestStruct>::new();
            for el in 1..100 {
                unrolled_list.push(black_box(TestStruct::new(el)))
            }
            for _ in 1..100 {
                let _ = unrolled_list.pop();
            }
        }
    ));
    group.bench_function("vec", |b| b.iter(||
        {
            let mut v = vec![];
            for el in 1..100 {
                v.push(black_box(TestStruct::new(el)))
            }
            for _ in 1..100 {
                let _ = v.pop();
            }
        }
    ));
    group.bench_function("linked_list", |b| b.iter(||
        {
            let mut linked_list = LinkedList::<TestStruct>::new();
            for el in 1..100 {
                linked_list.push_back(black_box(TestStruct::new(el)))
            }
            for _ in 1..100 {
                let _ = linked_list.pop_back();
            }
        }
    ));
    group.finish();
}

pub fn insert_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert");

    group.bench_function("unrolled_linked_list", |b| b.iter(||
        {
            let mut unrolled_list = UnrolledLinkedList::<TestStruct>::new();
            for el in 1..100 {
                unrolled_list.insert(0, black_box(TestStruct::new(el)))
            }
        }
    ));
    group.bench_function("vec", |b| b.iter(||
        {
            let mut v = vec![];
            for el in 1..100 {
                v.insert(0, black_box(TestStruct::new(el)))
            }
        }
    ));
    group.bench_function("linked_list", |b| b.iter(||
        {
            let mut linked_list = LinkedList::<TestStruct>::new();
            for el in 1..100 {
                linked_list.push_front(black_box(TestStruct::new(el)))
            }
        }
    ));
    group.finish();
}

pub fn insert_middle_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_middle");

    group.bench_function("unrolled_linked_list", |b| b.iter(||
        {
            let mut unrolled_list = UnrolledLinkedList::<TestStruct>::new();
            for el in 1..100 {
                unrolled_list.insert(unrolled_list.len() / 2, black_box(TestStruct::new(el)))
            }
        }
    ));
    group.bench_function("vec", |b| b.iter(||
        {
            let mut v = vec![];
            for el in 1..100 {
                v.insert(v.len() / 2, black_box(TestStruct::new(el)))
            }
        }
    ));
    group.bench_function("linked_list", |b| b.iter(||
        {
            let mut linked_list = LinkedList::<TestStruct>::new();
            for el in 1..100 {
                linked_list.push_back(black_box(TestStruct::new(el)))
            }
        }
    ));
    group.finish();
}

pub fn get_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");

    group.bench_function("unrolled_linked_list", |b| b.iter(||
        {
            let mut unrolled_list = UnrolledLinkedList::<TestStruct>::new();
            for el in 1..100 {
                unrolled_list.push(black_box(TestStruct::new(el)));
            }

            for el in 1..100 {
                let _ = unrolled_list.get(el);
            }
        }
    ));
    group.bench_function("vec", |b| b.iter(||
        {
            let mut v = vec![];
            for el in 1..100 {
                v.push(black_box(TestStruct::new(el)));
            }

            for el in 1..100 {
                let _ = v.get(el);
            }
        }
    ));
    group.bench_function("linked_list", |b| b.iter(||
        {
            let mut linked_list = LinkedList::<TestStruct>::new();
            for el in 1..100 {
                linked_list.push_back(black_box(TestStruct::new(el)))
            }
            for el in 1..100 {
                let _ = linked_list.pop_back();
            }
        }
    ));
    group.finish();
}

pub fn remove_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("remove");

    group.bench_function("unrolled_linked_list", |b| b.iter(||
        {
            let mut unrolled_list = UnrolledLinkedList::<TestStruct>::new();
            for el in 1..100 {
                unrolled_list.push(black_box(TestStruct::new(el)));
            }

            for el in (1..30).step_by(2) {
                let _ = unrolled_list.remove(el);
            }
        }
    ));
    group.bench_function("vec", |b| b.iter(||
        {
            let mut v = vec![];
            for el in 1..100 {
                v.push(black_box(TestStruct::new(el)));
            }

            for el in (1..30).step_by(2) {
                let _ = v.remove(el);
            }
        }
    ));
    group.bench_function("linked_list", |b| b.iter(||
        {
            let mut linked_list = LinkedList::<TestStruct>::new();
            for el in 1..100 {
                linked_list.push_back(black_box(TestStruct::new(el)))
            }
            for el in (1..30).step_by(2) {
                let _ = linked_list.pop_back();
            }
        }
    ));
    group.finish();
}

pub fn iter_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter");

    group.bench_function("unrolled_linked_list", |b| b.iter(||
        {
            let mut unrolled_list = UnrolledLinkedList::<TestStruct>::new();
            for el in 1..100 {
                unrolled_list.push(black_box(TestStruct::new(el)));
            }

            for el in unrolled_list.iter() {
               let _x = el;
            }
        }
    ));
    group.bench_function("vec", |b| b.iter(||
        {
            let mut v = vec![];
            for el in 1..100 {
                v.push(black_box(TestStruct::new(el)));
            }

            for el in v.iter() {
                let _x = el;
            }
        }
    ));
    group.bench_function("linked_list", |b| b.iter(||
        {
            let mut linked_list = LinkedList::<TestStruct>::new();
            for el in 1..100 {
                linked_list.push_back(black_box(TestStruct::new(el)))
            }
            for el in linked_list.iter() {
                let _x = el;
            }
        }
    ));
    group.finish();
}
pub fn iter_mut_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter_mut");

    group.bench_function("unrolled_linked_list", |b| b.iter(||
        {
            let mut unrolled_list = UnrolledLinkedList::<TestStruct>::new();
            for el in 1..100 {
                unrolled_list.push(black_box(TestStruct::new(el)));
            }

            for el in unrolled_list.iter_mut() {
                el.name = String::from("new_name");
            }
        }
    ));
    group.bench_function("vec", |b| b.iter(||
        {
            let mut v = vec![];
            for el in 1..100 {
                v.push(black_box(TestStruct::new(el)));
            }

            for el in v.iter_mut() {
                el.name = String::from("new_name");
            }
        }
    ));
    group.bench_function("linked_list", |b| b.iter(||
        {
            let mut linked_list = LinkedList::<TestStruct>::new();
            for el in 1..100 {
                linked_list.push_back(black_box(TestStruct::new(el)))
            }
            for el in linked_list.iter_mut() {
                el.name = String::from("new_name");
            }
        }
    ));
    group.finish();
}
pub fn into_iter_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("into_iter");

    group.bench_function("unrolled_linked_list", |b| b.iter(||
        {
            let mut unrolled_list = UnrolledLinkedList::<TestStruct>::new();
            for el in 1..100 {
                unrolled_list.push(black_box(TestStruct::new(el)));
            }

            for el in unrolled_list.into_iter() {
               let _x = el;
            }
        }
    ));
    group.bench_function("vec", |b| b.iter(||
        {
            let mut v = vec![];
            for el in 1..100 {
                v.push(black_box(TestStruct::new(el)));
            }

            for el in v.into_iter() {
                let _x = el;
            }
        }
    ));
    group.bench_function("linked_list", |b| b.iter(||
        {
            let mut linked_list = LinkedList::<TestStruct>::new();
            for el in 1..100 {
                linked_list.push_back(black_box(TestStruct::new(el)))
            }
            for el in linked_list.into_iter() {
                let _x = el;
            }
        }
    ));
    group.finish();
}

criterion_group!(benches,
push_bench,
pop_bench,
insert_bench,
insert_middle_bench,
get_bench,
remove_bench,
iter_bench,
iter_mut_bench,
into_iter_bench,
);
criterion_main!(benches);