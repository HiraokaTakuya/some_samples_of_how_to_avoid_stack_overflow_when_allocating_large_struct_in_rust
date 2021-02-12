#![feature(new_uninit)]

const N: usize = 512;

#[derive(Debug, Clone, Copy)]
struct X<T: Copy>([T; N]);
impl<T: Copy> X<T> {
    fn new(item: T) -> Self {
        Self([item; N])
    }
}
type A = X<i8>;
type B = X<A>;
type C = X<B>;

// This function causes stack overflow.
#[allow(dead_code)]
fn new() -> C {
    C::new(B::new(A::new(1)))
}

// This function causes stack overflow.
#[allow(dead_code)]
fn new_with_box() -> Box<C> {
    Box::new(C::new(B::new(A::new(1))))
}

#[allow(dead_code)]
fn new_with_box_new_uninit() -> Box<C> {
    // Box::new_uninit() is a nightly-only API.
    let mut c = Box::<C>::new_uninit();
    unsafe {
        let c_ptr = c.as_mut_ptr();
        for b in &mut (*c_ptr).0 {
            for a in &mut b.0 {
                *a = A::new(1);
            }
        }
        c.assume_init()
    }
}

#[allow(dead_code)]
fn new_with_ptr() -> Box<C> {
    unsafe {
        let c_ptr = std::alloc::alloc(std::alloc::Layout::new::<C>()) as *mut C;
        for b in &mut (*c_ptr).0 {
            for a in &mut b.0 {
                *a = A::new(1);
            }
        }
        Box::from_raw(c_ptr)
    }
}

// This function causes stack overflow.
#[allow(dead_code)]
fn new_with_thread() -> C {
    const STACK_SIZE: usize = 512 * 1024 * 1024;
    std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(|| C::new(B::new(A::new(1))))
        .unwrap()
        .join()
        .unwrap()
}

#[allow(dead_code)]
fn new_with_box_in_thread() -> Box<C> {
    const STACK_SIZE: usize = 512 * 1024 * 1024;
    std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(|| Box::new(C::new(B::new(A::new(1)))))
        .unwrap()
        .join()
        .unwrap()
}

#[derive(Debug, Clone)]
struct XVec<T: Clone>(Vec<T>);
impl<T: Clone> XVec<T> {
    fn new(item: T) -> Self {
        Self(vec![item; N])
    }
}
type AVec = XVec<i8>;
type BVec = XVec<AVec>;
type CVec = XVec<BVec>;

#[allow(dead_code)]
fn new_with_vec() -> CVec {
    CVec::new(BVec::new(AVec::new(1)))
}

fn main() {}
