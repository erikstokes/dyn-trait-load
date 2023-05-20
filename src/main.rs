use std::io::Read;

use serde::{Deserialize, Serialize};
use clap::Parser;

trait Data {}

// If T implements Data, so does &'a T
impl<'a, T: ?Sized> Data for &'a T where T: Data {}

impl Data for u32 {}
impl Data for i64 {}


trait Test<D: Data>: std::fmt::Debug {
    fn test(&self, data: &D) -> bool;
}

trait GenericTestExecutor
{
    fn generic_run_tests<D: Data>(&self, data: &D) {
        let results: Vec<bool> = self.generic_tests().iter()
                                    .map(|t| t.test(data))
                                    .collect();
        dbg!(results);
    }
    fn generic_tests<D: Data>(&self) -> Vec<Box<dyn Test<D>>>;
}


impl<'a, T: ?Sized + std::iter::Iterator> GenericTestExecutor for Box<T>
where
    T: GenericTestExecutor,
{
    fn generic_run_tests<D: Data>(&self, data: &D) {
        (**self).generic_run_tests(data)
    }

    fn generic_tests<D:Data>(&self) -> Vec<Box<dyn Test<D>>> {
        (**self).generic_tests()
}

}

trait TestExecutor {
    fn run_tests(&self, data: &dyn Data);
    fn tests(&self) -> Vec<Box<dyn Test<dyn Data>>>;
}

impl GenericTestExecutor for dyn TestExecutor {
    fn generic_run_tests<D: Data>(&self, data: &D) {
        self.run_tests(data)
    }

    fn generic_tests<D:Data>(&self) -> Vec<Box<dyn Test<D>>> {
        self.tests()
    }
}

pub mod test32 {
    type DataType = i64;

    #[typetag::serde(tag = "NAME")]
    pub(super) trait TestTag: std::fmt::Debug + super::Test<DataType>  {}

    #[derive(Debug, super::Deserialize)]
    pub(super) struct TestArray
    {
        pub(super) tests: Vec<Box<dyn TestTag>>,

    }

    impl IntoIterator for TestArray {
        type Item = Box<dyn TestTag>;

        type IntoIter = ::std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.tests.into_iter()
        }
    }

    impl super::GenericTestExecutor for TestArray {
        fn generic_tests<D:Data>(&self) -> Vec<Box<dyn super::Test<DataType>>> {
            let out = vec![];

            out
        }
    }

}

// pub mod test64 {
//     type DataType = i64;

//     #[typetag::serde(tag = "NAME")]
//     pub(super) trait TestTag: std::fmt::Debug + super::Test<DataType>  {}

//     #[derive(Debug, super::Deserialize)]
//     pub(super) struct TestArray
//     {
//         pub(super) tests: Vec<Box<dyn TestTag>>,

//     }

//     impl IntoIterator for TestArray {
//         type Item = Box<dyn TestTag>;

//         type IntoIter = ::std::vec::IntoIter<Self::Item>;

//         fn into_iter(self) -> Self::IntoIter {
//             self.tests.into_iter()
//         }
//     }

//     impl super::GenericTestExecutor for TestArray {}
// }

// Actual structs used to test

#[derive(Serialize, Deserialize, Debug)]
struct A {
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct B {
    name: String,
    size: usize,
}

// boilerplate for loading things from input file

#[typetag::serde]
impl test32::TestTag for A {}


#[typetag::serde]
impl test32::TestTag for B {}

// #[typetag::serde]
// impl test64::TestTag for A {}


// #[typetag::serde]
// impl test64::TestTag for B {}

// Implementations for the various tests

impl Test<u32> for A {
    fn test(&self, data: &u32) -> bool {
println!("Testing A 32");
        true
    }
}

impl Test<i64> for A {
    fn test(&self, _data: &i64) -> bool {
        println!("Testing A 64");
        true
    }
}

impl Test<i64> for B {
    fn test(&self, _data: &i64) -> bool {
        println!("Testing B 64");
        true
    }
}

impl Test<u32> for B {
    fn test(&self, _data: &u32) -> bool {
        println!("Testing B 32");
        false
    }
}

#[derive(clap::Parser)]
struct Args {
    /// The size of input to test. Either 32 or 64
    size: u32,
}

fn main() {
    let args = Args::parse();
    let data = {
        let mut data = "".to_string();
        std::fs::File::open("test.yaml")
            .unwrap()
            .read_to_string(&mut data)
            .unwrap();
        data
    };

    // let tests: &dyn TestExecutor = match args.size {
    //     32 => &serde_yaml::from_str::<test32::TestArray>(&data).unwrap(),
    //     64 => &serde_yaml::from_str::<test64::TestArray>(&data).unwrap(),
    //     _ => panic!("bad size"),
    // };

    // // let test: Box<dyn Runner> = Box::new(A { x:1, y:2});
    // // let t = test.as_runner();
    // for test in tests.tests.iter() {
    //     println!("test, {:?} {}", test, test.test(&123));
    // }
}
