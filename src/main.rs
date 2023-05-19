use std::{io::Read, marker::PhantomData};

use serde::{Deserialize, Serialize};

trait Data {}

// If T implements Data, so does &'a T
impl<'a, T: ?Sized> Data for &'a T where T: Data {}

impl Data for u32 {}
impl Data for i64 {}

// trait AsRunner {
//     fn as_runner(&self) -> &(dyn Runner + 'static);
// }

trait Test: std::fmt::Debug {
    type Data;
    fn test(&self, data: &Self::Data) -> bool;
}

#[typetag::serde(tag = "NAME")]
trait TestTag: std::fmt::Debug + Test<Data=u32>  {}

#[derive(Debug, Deserialize)]
struct TestArray
{
    tests: Vec<Box<dyn TestTag>>,

}

// trait GenericRunner {
//     fn generic_test<D: Data>(&self, data: D) -> bool;
// }

// impl<'a, T: ?Sized> GenericRunner for Box<T>
// where
//     T: GenericRunner,
// {
//     fn generic_test<D: Data>(&self, data: D) -> bool {
//         (**self).generic_test(data)
//     }
// }

// trait Runner {
//     fn do_test(&self, data: &dyn Data) -> bool;
// }

// impl GenericRunner for dyn Runner
// {
//     fn generic_test<D: Data>(&self, data: D) -> bool {
//         self.do_test(&data)
//     }
// }

// impl<T> Runner for T
// where
//     T: GenericRunner
// {
//     fn do_test(&self, data: &dyn Data) -> bool {
//         self.generic_test(data)
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
struct A<D: Data> {
    x: u32,
    y: u32,
    #[serde(skip_serializing, default)]
    phantom: PhantomData<D>,
}

#[derive(Serialize, Deserialize, Debug)]
struct B {
    name: String,
    size: usize,
}

#[typetag::serde]
impl TestTag for A<u32> {}

impl Test for A<u32> {
    type Data = u32;
    fn test(&self, data: &u32) -> bool {
        self.self_test(data)
    }
}

impl Test for A<i64> {
    type Data = i64;
    fn test(&self, _data: &i64) -> bool {
        panic!("this is bad")
    }
}


// impl<T> AsRunner for T
// where T: Runner + 'static
// {
//     fn as_runner<'a>(&'a self) -> &(dyn Runner + 'static) {
//         self
//     }
// }

impl A<u32> {
    fn self_test(&self, data: &u32) -> bool {
        *data <= self.x + self.y
    }
}

// impl GenericRunner for A<u32> {
//     fn generic_test<D: Data>(&self, data: D) -> bool
//     {
//         println!("generic test");
//         true
//     }
// }

#[typetag::serde]
impl TestTag for B {}

impl Test for B {
    type Data = u32;
    fn test(&self, _data: &Self::Data) -> bool {
        false
    }
}



fn main() {
    let data = {
        let mut data = "".to_string();
        std::fs::File::open("test.yaml")
            .unwrap()
            .read_to_string(&mut data)
            .unwrap();
        data
    };
    let tests: TestArray = serde_yaml::from_str(&data).unwrap();
    // let test: Box<dyn Runner> = Box::new(A { x:1, y:2});
    // let t = test.as_runner();
    for test in tests.tests.iter() {
        println!("test, {:?} {}", test, test.test(&123));
    }

}
