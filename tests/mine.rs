extern crate minivec;

use minivec::{mini_vec, MiniVec};
use std::{
    collections::hash_map::DefaultHasher,
    convert::From,
    hash::{Hash, Hasher},
    ops::{Index, IndexMut},
};

#[test]
fn minivec_default_constructed() {
    let v: MiniVec<i32> = MiniVec::new();
    assert_eq!(v.capacity(), 0);
    assert_eq!(v.len(), 0);
    assert!(v.is_empty());

    let v: MiniVec<i32> = Default::default();
    assert_eq!(v.capacity(), 0);
    assert_eq!(v.len(), 0);
    assert!(v.is_empty());
}

#[test]
fn minivec_as_mut() {
    let mut v = mini_vec![1, 2, 3];
    let x: &mut [i32] = v.as_mut();
    assert_eq!(x, [1, 2, 3]);

    let mut v = mini_vec![1, 2, 3];
    let mut vv: &mut MiniVec<i32> = &mut v;
    let x: &mut MiniVec<_> = std::convert::AsMut::<MiniVec<i32>>::as_mut(&mut vv);
    assert_eq!(x, &mut [1, 2, 3].as_mut());
}

#[test]
fn minivec_push() {
    let mut v: MiniVec<i32> = MiniVec::new();

    assert_eq!(v.len(), v.capacity());

    v.push(1);
    v.push(2);
    v.push(3);

    assert_eq!(v.len(), 3);
    assert!(v.capacity() >= v.len());

    let mut v: MiniVec<String> = MiniVec::new();

    assert_eq!(v.len(), v.capacity());

    v.push(String::from("Hello"));
    v.push(String::from("Rust"));
    v.push(String::from("World!"));

    assert_eq!(v.len(), 3);
    assert!(v.capacity() >= v.len());

    let mut v: MiniVec<String> = MiniVec::new();

    assert_eq!(v.len(), v.capacity());

    for _ in 0..32 {
        v.push(String::from("Hello, world!"));
    }

    assert_eq!(v.len(), 32);
    assert!(v.capacity() >= v.len());
}

#[test]
fn minivec_deref_test() {
    let mut v: MiniVec<i32> = MiniVec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    assert_eq!(v[0], 1);
    assert_eq!(v[1], 2);
    assert_eq!(v[2], 3);
}

#[test]
fn minivec_dedup_by_test() {
    let mut v = mini_vec![1, 2, 1, 1, 3, 3, 3, 4, 5, 4];
    v.dedup_by(|x, y| x == y);

    assert_eq!(v, [1, 2, 1, 3, 4, 5, 4]);
}

#[test]
fn minivec_dedup_needs_drop() {
    let mut v: MiniVec<Box<_>> = mini_vec![Box::new(1), Box::new(1), Box::new(2), Box::new(3)];
    v.dedup();

    assert_eq!(v.len(), 3);
}

#[test]
fn minivec_drain_into_vec() {
    let mut vec: MiniVec<usize> = mini_vec![];

    let drain = vec.drain(..).collect::<MiniVec<_>>();

    assert_eq!(drain.len(), 0);
}

#[test]
fn minvec_drain() {
    let mut vec = mini_vec![1, 2, 3];

    let mut drain = vec.drain(..);

    assert_eq!(drain.size_hint(), (3, Some(3)));

    drain.next();

    assert_eq!(drain.size_hint(), (2, Some(2)));
}

#[test]
fn minivec_with_capacity() {
    let size = 128;
    let mut v: MiniVec<i32> = MiniVec::with_capacity(size);

    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), size);

    v.push(1);
    v.push(2);
    v.push(3);

    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), size);
}

#[test]
fn minivec_extend_from_slice() {
    let a = [2, 3, 4, 5];
    let mut v = mini_vec![1];
    v.extend_from_slice(&a);

    assert_eq!(v.len(), 5);

    v.extend_from_slice(&[6, 7, 8]);
    assert_eq!(v.len(), 8);

    assert_eq!(a.len(), 4);
    assert_eq!(a, [2, 3, 4, 5]);

    let a: MiniVec<_> = [2, 3, 4, 5].iter().map(|x| x.to_string()).collect();
    let mut v = mini_vec![String::from("1")];
    v.extend_from_slice(&a);

    assert_eq!(v.len(), 5);

    v.extend_from_slice(&[6.to_string(), 7.to_string(), 8.to_string()]);
    assert_eq!(v.len(), 8);

    assert_eq!(a.len(), 4);
    assert_eq!(
        a,
        [2.to_string(), 3.to_string(), 4.to_string(), 5.to_string()]
    );
}

#[test]
fn minivec_from_raw_part() {
    use std::{mem, ptr};

    let v = mini_vec![1, 2, 3];
    let mut v = mem::ManuallyDrop::new(v);

    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe {
        for i in 0..len as isize {
            ptr::write(p.offset(i), 4 + i);
        }

        let rebuilt = MiniVec::from_raw_part(p);
        assert_eq!(rebuilt, [4, 5, 6]);
        assert_eq!(rebuilt.capacity(), cap);
        assert_eq!(rebuilt.len(), len);
    }
}

#[test]
fn minivec_from_raw_parts() {
    use std::{mem, ptr};

    let v = mini_vec![1, 2, 3];
    let mut v = mem::ManuallyDrop::new(v);

    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe {
        for i in 0..len as isize {
            ptr::write(p.offset(i), 4 + i);
        }

        let rebuilt = MiniVec::from_raw_parts(p, len, cap);
        assert_eq!(rebuilt, [4, 5, 6]);
        assert_eq!(rebuilt.capacity(), cap);
        assert_eq!(rebuilt.len(), len);
    }
}

#[test]
fn minivec_insert() {
    let mut vec = mini_vec![1, 2, 3];

    vec.insert(1, 4);
    assert_eq!(vec, [1, 4, 2, 3]);

    vec.insert(4, 5);
    assert_eq!(vec, [1, 4, 2, 3, 5]);

    let mut vec: MiniVec<String> = mini_vec![1.to_string(), 2.to_string(), 3.to_string()];

    vec.insert(1, 4.to_string());
    assert_eq!(
        vec,
        [1.to_string(), 4.to_string(), 2.to_string(), 3.to_string()]
    );

    vec.insert(4, 5.to_string());
    assert_eq!(
        vec,
        [
            1.to_string(),
            4.to_string(),
            2.to_string(),
            3.to_string(),
            5.to_string()
        ]
    );
}

#[test]
// This is quite heavy-handed, but stops MIRI complaining about the leak.
#[cfg_attr(miri, ignore)]
fn minivec_leak() {
    let x = mini_vec![1, 2, 3];
    let static_ref: &'static mut [usize] = MiniVec::leak(x);
    static_ref[0] += 1;
    assert_eq!(static_ref, &[2, 2, 3]);
}

#[test]
fn minivec_pop() {
    let mut vec = mini_vec![1, 2, 3];
    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec, [1, 2]);

    let mut vec = mini_vec![1.to_string(), 2.to_string(), 3.to_string()];
    assert_eq!(vec.pop(), Some(String::from("3")));
    assert_eq!(vec, [String::from("1"), String::from("2")]);
}

#[test]
fn minivec_remove() {
    let mut v = mini_vec![1, 2, 3];
    assert_eq!(v.remove(1), 2);
    assert_eq!(v, [1, 3]);
}

#[test]
#[should_panic]
fn minivec_remove_panic() {
    let mut v = mini_vec![1, 2, 3];
    v.remove(v.len());
}

#[test]
fn minivec_remove_item() {
    let mut vec = mini_vec![1, 2, 3, 1];

    vec.remove_item(&1);

    assert_eq!(vec, mini_vec![2, 3, 1]);
}

#[test]
fn minivec_test_resize() {
    let mut vec = mini_vec!["hello"];
    vec.resize(3, "world");
    assert_eq!(vec, ["hello", "world", "world"]);

    let mut vec = mini_vec![1, 2, 3, 4];
    vec.resize(2, 0);
    assert_eq!(vec, [1, 2]);

    let mut vec = mini_vec![1, 2, 3, 4];
    vec.resize(vec.len(), -1);
    assert_eq!(vec, [1, 2, 3, 4]);
}

#[test]
fn minivec_resize_with() {
    let mut vec = mini_vec![1, 2, 3];
    vec.resize_with(5, Default::default);
    assert_eq!(vec, [1, 2, 3, 0, 0]);

    let mut vec = mini_vec![];
    let mut p = 1;
    vec.resize_with(4, || {
        p *= 2;
        p
    });

    assert_eq!(vec, [2, 4, 8, 16]);
}

#[test]
fn minivec_retain() {
    let mut vec = mini_vec![1, 2, 3, 4];
    vec.retain(|&x| x % 2 == 0);
    assert_eq!(vec, [2, 4]);

    let mut vec = mini_vec![1, 2, 3, 4, 5];
    let keep = [false, true, true, false, true];

    let mut i = 0;
    vec.retain(|_| {
        let should_keep = keep[i];
        i += 1;
        should_keep
    });

    assert_eq!(vec, [2, 3, 5]);
}

#[test]
fn minivec_shrink_to() {
    let mut vec = MiniVec::<i32>::with_capacity(10);
    vec.push(1);
    vec.push(2);
    vec.push(3);

    assert_eq!(vec.capacity(), 10);
    assert_eq!(vec.len(), 3);

    vec.shrink_to(4);
    assert_eq!(vec.capacity(), 4);
    assert_eq!(vec.len(), 3);

    vec.shrink_to(0);
    assert_eq!(vec.capacity(), 3);
}

#[test]
#[should_panic]
fn minivec_shrink_to_panic() {
    let mut vec = MiniVec::<String>::with_capacity(10);

    vec.push(String::from("1"));
    vec.push(String::from("2"));
    vec.push(String::from("3"));
    vec.push(String::from("4"));
    vec.push(String::from("5"));

    vec.shrink_to(10000);
}

#[test]
fn minivec_extend() {
    let mut v = mini_vec![1, 2, 3];
    let other = mini_vec![4, 5, 6];

    v.extend(other.iter());

    assert_eq!(v, mini_vec![1, 2, 3, 4, 5, 6]);

    let mut v = mini_vec![String::from("1"), String::from("2"), String::from("3")];
    let other = vec![String::from("4"), String::from("5"), String::from("6")];

    v.extend(other.into_iter());

    assert_eq!(
        v,
        mini_vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6")
        ]
    );
}

#[test]
fn minivec_from() {
    // From<&'a [T]>
    //
    let data = [1, 2, 3];
    let x: &[i32] = &data[..];
    let v = MiniVec::<i32>::from(x);

    assert_eq!(v, [1, 2, 3]);

    // From<&'a mut [T]>
    //
    let mut data = [1, 2, 3];
    let x: &mut [i32] = &mut data[..];
    let v = MiniVec::<i32>::from(x);

    assert_eq!(v, [1, 2, 3]);

    // From<&'a str>
    //
    let data: &str = "Hello, world!\n";
    let v = MiniVec::from(data);
    assert_eq!(v, Vec::from(data));

    // From<&'a MiniVec<T>> to Cow<'a, [T]>
    //
    let v = mini_vec![1, 2, 3];
    let cow_v = std::borrow::Cow::from(&v);
    assert_eq!(cow_v, &[1, 2, 3][..]);
}

#[test]
fn minivec_hash() {
    let v = mini_vec![1, 2, 3, 4, 5, 6];
    let w = vec![1, 2, 3, 4, 5, 6];

    let mut h = DefaultHasher::new();
    v.hash(&mut h);

    let hashed_mini_vec = h.finish();

    assert_eq!(hashed_mini_vec, 4819070452177268435);

    let mut h = DefaultHasher::new();
    w.hash(&mut h);

    assert_eq!(hashed_mini_vec, h.finish());
}

#[test]
fn minivec_index() {
    let mut v = mini_vec![1, 2, 3, 4, 5, 6];

    assert_eq!(*v.index(0), 1);
    assert_eq!(*v.index(1), 2);
    assert_eq!(*v.index(2), 3);
    assert_eq!(*v.index(3), 4);
    assert_eq!(*v.index(4), 5);
    assert_eq!(*v.index(5), 6);

    *v.index_mut(0) = 3;
    *v.index_mut(1) = 2 * 3;
    *v.index_mut(2) = 3 * 3;
    *v.index_mut(3) = 4 * 3;
    *v.index_mut(4) = 5 * 3;
    *v.index_mut(5) = 6 * 3;

    assert_eq!(*v.index(0), 3);
    assert_eq!(*v.index(1), 2 * 3);
    assert_eq!(*v.index(2), 3 * 3);
    assert_eq!(*v.index(3), 4 * 3);
    assert_eq!(*v.index(4), 5 * 3);
    assert_eq!(*v.index(5), 6 * 3);
}

#[test]
fn minivec_into_iter() {
    // test normal Iterator trait
    //
    let v = mini_vec![1, 2, 3, 4, 5];

    let iter = v.into_iter();
    assert_eq!(iter.size_hint(), (5, Some(5)));

    for (idx, x) in iter.enumerate() {
        assert_eq!(x, idx + 1);
    }

    let v = mini_vec![
        1.to_string(),
        2.to_string(),
        3.to_string(),
        4.to_string(),
        5.to_string()
    ];

    let iter = v.into_iter();
    for (idx, x) in iter.enumerate() {
        assert_eq!(x, (idx + 1).to_string());
    }

    // test as_slice() + as_mut_slice()
    //
    let v = mini_vec![1, 2, 3, 4, 5];

    let mut iter = v.into_iter();
    iter.next();
    iter.next();

    assert_eq!(iter.as_slice(), &[3, 4, 5]);

    let v = mini_vec![1, 2, 3, 4, 5];

    let mut iter = v.into_iter();
    iter.next();
    iter.next();

    assert_eq!(iter.as_mut_slice(), &[3, 4, 5]);

    // test AsRef
    //
    let v = mini_vec![1, 2, 3, 4, 5];

    let iter = v.into_iter();
    let s: &[i32] = iter.as_ref();
    assert_eq!(s, &[1, 2, 3, 4, 5]);

    // test clone()
    //
    let v = mini_vec![1, 2, 3, 4, 5];

    let mut iter = v.into_iter();

    #[allow(clippy::redundant_clone)]
    let w = iter.clone();

    assert_eq!(w.as_slice(), &[1, 2, 3, 4, 5]);

    iter.next();
    iter.next();

    #[allow(clippy::redundant_clone)]
    let w = iter.clone();

    assert_eq!(w.as_slice(), &[3, 4, 5]);

    // test DoubleEndedIterator
    //
    let v = mini_vec![1, 2, 3, 4, 5];

    let mut iter = v.into_iter();

    assert_eq!(iter.next().unwrap(), 1);
    assert_eq!(iter.next_back().unwrap(), 5);

    assert_eq!(iter.next().unwrap(), 2);
    assert_eq!(iter.next_back().unwrap(), 4);

    assert_eq!(iter.next().unwrap(), 3);
    assert_eq!(iter.next_back(), None);

    // test automatic impl of Drop to make sure it frees owning objects
    //
    let v = mini_vec![
        1.to_string(),
        2.to_string(),
        3.to_string(),
        4.to_string(),
        5.to_string()
    ];

    let mut iter = v.into_iter();

    iter.next();
    iter.next();

    // ExactSizeIterator test
    //
    let v = mini_vec![1, 2, 3, 4, 5];

    let mut iter = v.into_iter();

    assert_eq!(iter.len(), 5);

    iter.next();
    assert_eq!(iter.len(), 4);

    iter.next();
    assert_eq!(iter.len(), 3);

    iter.next();
    assert_eq!(iter.len(), 2);

    iter.next();
    assert_eq!(iter.len(), 1);

    iter.next();
    assert_eq!(iter.len(), 0);

    // FusedIterator test
    //
    let v = mini_vec![1];

    let mut iter = v.into_iter();
    assert_eq!(iter.next().unwrap(), 1);

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    // IntoIterator for &MiniVec<T>
    //
    let v = mini_vec![
        1.to_string(),
        2.to_string(),
        3.to_string(),
        4.to_string(),
        5.to_string()
    ];
    let mut slice_iter = (&v).into_iter();

    assert_eq!(slice_iter.next().unwrap(), &(1.to_string()));

    // IntoIterator for &mut MiniVec<T>
    //
    let mut v = mini_vec![
        1.to_string(),
        2.to_string(),
        3.to_string(),
        4.to_string(),
        5.to_string()
    ];

    let x: &mut MiniVec<_> = &mut v;

    let slice_iter = x.into_iter();

    for s in slice_iter {
        *s = "4321".to_string();
    }
}

#[test]
fn minivec_swap_remove() {
    let mut v = mini_vec!["foo", "bar", "baz", "qux"];

    assert_eq!(v.swap_remove(1), "bar");
    assert_eq!(v, ["foo", "qux", "baz"]);

    assert_eq!(v.swap_remove(0), "foo");
    assert_eq!(v, ["baz", "qux"]);

    assert_eq!(v.swap_remove(0), "baz");
    assert_eq!(v, ["qux"]);

    assert_eq!(v.swap_remove(0), "qux");
    assert_eq!(v, []);
}

#[test]
fn minivec_split_off() {
    let mut vec = mini_vec![1, 2, 3];
    let vec2 = vec.split_off(1);
    assert_eq!(vec, [1]);
    assert_eq!(vec2, [2, 3]);

    let mut vec = mini_vec![1, 2, 3];
    let vec2 = vec.split_off(3);
    assert_eq!(vec, [1, 2, 3]);
    assert_eq!(vec2, []);

    let mut vec = mini_vec![1, 2, 3];
    let vec2 = vec.split_off(0);
    assert_eq!(vec, []);
    assert_eq!(vec2, [1, 2, 3]);
}

#[test]
fn minivec_splice_empty() {
    let mut v = MiniVec::<i32>::new();
    let replace = mini_vec![1, 2, 3];

    let u: MiniVec<_> = v.splice(.., replace.iter().cloned()).collect();

    assert!(u.is_empty());
    assert_eq!(v, &[1, 2, 3]);
}

#[test]
fn minivec_splice() {
    let mut v = mini_vec![1, 2, 3];
    let new = [7, 8];
    let u: MiniVec<_> = v.splice(..2, new.iter().cloned()).collect();
    assert_eq!(v, &[7, 8, 3]);
    assert_eq!(u, &[1, 2]);

    let mut v = mini_vec![1, 2, 3, 4, 5, 6];
    let new = [7, 8];
    let u: MiniVec<_> = v.splice(1..4, new.iter().cloned()).collect();
    assert_eq!(v, &[1, 7, 8, 5, 6]);
    assert_eq!(u, &[2, 3, 4]);

    let mut v = mini_vec![1, 2, 3];
    let new = [7, 8, 9, 10, 11];
    let u: MiniVec<_> = v.splice(..2, new.iter().cloned()).collect();
    assert_eq!(v, &[7, 8, 9, 10, 11, 3]);
    assert_eq!(u, &[1, 2]);

    let mut v = mini_vec![1, 2, 3];
    let new = [7, 8];
    let u: MiniVec<_> = v.splice(2..2, new.iter().cloned()).collect();
    assert_eq!(v, &[1, 2, 7, 8, 3]);
    assert_eq!(u, &[]);

    let mut v = mini_vec![1, 2, 3, 4, 5, 6];
    let new = [7, 8, 9, 10, 11];
    let mut iter = v.splice(..4, new.iter().cloned());
    let mut u = MiniVec::new();

    u.push(iter.next().unwrap());
    u.push(iter.next().unwrap());

    std::mem::drop(iter);

    assert_eq!(v, &[7, 8, 9, 10, 11, 5, 6]);
    assert_eq!(u, &[1, 2]);
}

#[test]
fn minivec_splice_raii() {
    let mut v = mini_vec![1.to_string(), 2.to_string(), 3.to_string()];
    let new = [7.to_string(), 8.to_string()];
    let u: MiniVec<_> = v.splice(..2, new.iter().cloned()).collect();
    assert_eq!(v, &[7.to_string(), 8.to_string(), 3.to_string()]);
    assert_eq!(u, &[1.to_string(), 2.to_string()]);

    let mut v = mini_vec![
        1.to_string(),
        2.to_string(),
        3.to_string(),
        4.to_string(),
        5.to_string(),
        6.to_string()
    ];

    let new = [7.to_string(), 8.to_string()];
    let u: MiniVec<_> = v.splice(1..4, new.iter().cloned()).collect();

    assert_eq!(
        v,
        &[
            1.to_string(),
            7.to_string(),
            8.to_string(),
            5.to_string(),
            6.to_string()
        ]
    );

    assert_eq!(u, &[2.to_string(), 3.to_string(), 4.to_string()]);

    let mut v = mini_vec![1.to_string(), 2.to_string(), 3.to_string()];
    let new = [
        7.to_string(),
        8.to_string(),
        9.to_string(),
        10.to_string(),
        11.to_string(),
    ];

    let u: MiniVec<_> = v.splice(..2, new.iter().cloned()).collect();
    assert_eq!(
        v,
        &[
            7.to_string(),
            8.to_string(),
            9.to_string(),
            10.to_string(),
            11.to_string(),
            3.to_string()
        ]
    );

    assert_eq!(u, &[1.to_string(), 2.to_string()]);

    let mut v = mini_vec![1.to_string(), 2.to_string(), 3.to_string()];
    let new = [7.to_string(), 8.to_string()];
    let u: MiniVec<_> = v.splice(2..2, new.iter().cloned()).collect();

    assert_eq!(
        v,
        &[
            1.to_string(),
            2.to_string(),
            7.to_string(),
            8.to_string(),
            3.to_string()
        ]
    );

    assert_eq!(u, &[]);

    let mut v = mini_vec![
        1.to_string(),
        2.to_string(),
        3.to_string(),
        4.to_string(),
        5.to_string(),
        6.to_string()
    ];

    let new = [
        7.to_string(),
        8.to_string(),
        9.to_string(),
        10.to_string(),
        11.to_string(),
    ];

    let mut iter = v.splice(..4, new.iter().cloned());
    let mut u = MiniVec::new();

    u.push(iter.next().unwrap());
    u.push(iter.next().unwrap());

    std::mem::drop(iter);

    assert_eq!(
        v,
        &[
            7.to_string(),
            8.to_string(),
            9.to_string(),
            10.to_string(),
            11.to_string(),
            5.to_string(),
            6.to_string()
        ]
    );

    assert_eq!(u, &[1.to_string(), 2.to_string()]);
}

#[test]
fn minivec_truncate() {
    let mut vec = mini_vec![1, 2, 3, 4];
    vec.truncate(vec.len());

    assert_eq!(vec, [1, 2, 3, 4]);
}

#[test]
fn minivec_macro() {
    let vec: MiniVec<i32> = mini_vec!();
    assert_eq!(vec.as_ptr(), core::ptr::null());
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 0);

    let vec: MiniVec<i32> = mini_vec![1337; 4096];
    assert_eq!(vec.len(), 4096);
    assert_eq!(vec[0], 1337);
}

#[test]
fn minivec_spare_capacity_mut() {
    let capacity = 128;
    let mut vec = MiniVec::<i32>::with_capacity(capacity);

    assert!(vec.is_empty());
    assert!(vec.capacity() >= capacity);

    let buf = vec.spare_capacity_mut();
    buf.iter_mut().enumerate().for_each(|(idx, byte)| {
        *byte = core::mem::MaybeUninit::<i32>::new(idx as i32);
    });

    let len = capacity;
    unsafe { vec.set_len(len) };

    let first = Some(0);
    let succ = |n: &i32| -> Option<i32> { Some(n + 1) };
    let iter = core::iter::successors(first, succ).take(capacity);
    let expected: MiniVec<i32> = iter.collect();

    assert_eq!(vec, expected);
}

#[allow(clippy::many_single_char_names, clippy::float_cmp, clippy::identity_op)]
#[cfg(target_feature = "avx")]
#[test]
fn minivec_with_alignment() {
    assert!(MiniVec::<i32>::with_alignment(24, 1).is_err());
    assert!(MiniVec::<i32>::with_alignment(24, 9).is_err());
    assert!(MiniVec::<i32>::with_alignment(24, core::mem::align_of::<*const ()>()).is_ok());

    let alignment = 32;
    let num_elems = 64;

    assert!(num_elems % alignment == 0);

    let mut vec = MiniVec::<f32>::with_alignment(num_elems, alignment).unwrap();

    let p = vec.as_ptr();
    assert!(p as usize % alignment == 0);

    let s = vec.spare_capacity_mut();
    assert_eq!(s.len(), num_elems);

    s.iter_mut().enumerate().for_each(|(idx, x)| {
        *x = core::mem::MaybeUninit::new(idx as f32);
    });

    unsafe {
        vec.set_len(num_elems);
    }

    let p = vec.as_mut_ptr();
    for idx in 0..(num_elems / 8) {
        unsafe {
            let x = core::arch::x86_64::_mm256_load_ps(p.add(idx * 8));
            let y = core::arch::x86_64::_mm256_set_ps(8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
            let z = core::arch::x86_64::_mm256_add_ps(x, y);
            core::arch::x86_64::_mm256_store_ps(p.add(idx * 8), z);
        }
    }

    for idx in 0..(num_elems / 8) {
        assert_eq!(vec[idx * 8 + 0], (idx * 8 + 0) as f32 + 1.0);
        assert_eq!(vec[idx * 8 + 1], (idx * 8 + 1) as f32 + 2.0);
        assert_eq!(vec[idx * 8 + 2], (idx * 8 + 2) as f32 + 3.0);
        assert_eq!(vec[idx * 8 + 3], (idx * 8 + 3) as f32 + 4.0);
        assert_eq!(vec[idx * 8 + 4], (idx * 8 + 4) as f32 + 5.0);
        assert_eq!(vec[idx * 8 + 5], (idx * 8 + 5) as f32 + 6.0);
        assert_eq!(vec[idx * 8 + 6], (idx * 8 + 6) as f32 + 7.0);
        assert_eq!(vec[idx * 8 + 7], (idx * 8 + 7) as f32 + 8.0);
    }
}

#[test]
fn minivec_into_raw_parts() {
    let vec = minivec::mini_vec![1, 2, 3, 4, 5];

    let (old_len, old_cap) = (vec.len(), vec.capacity());

    let (ptr, len, cap) = vec.into_raw_parts();

    assert_eq!(len, old_len);
    assert_eq!(cap, old_cap);

    let vec = unsafe { minivec::MiniVec::from_raw_parts(ptr, len, cap) };
    assert_eq!(vec, [1, 2, 3, 4, 5]);
}

#[test]
fn minivec_page_aligned() {
    let capacity = 1024;
    let alignment = 4096;
    let mut vec = minivec::MiniVec::<i32>::with_alignment(capacity, alignment).unwrap();
    assert_eq!(vec.as_mut_ptr() as usize % alignment, 0);
}
