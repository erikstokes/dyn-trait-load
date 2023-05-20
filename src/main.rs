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

pub mod test32 {
    type DataType = u32;

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
}

pub mod test64 {
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
}

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

#[typetag::serde]
impl test64::TestTag for A {}


#[typetag::serde]
impl test64::TestTag for B {}

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

fn run_tests<A: Test<D> + ?Sized, D:Data>(array: &Vec<Box<A>>, data: &D)
{
    for test in array {
        test.test(data);
    }
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

   match args.size {
        32 => {
            let tests = serde_yaml::from_str::<test32::TestArray>(&data).unwrap();
            run_tests(&tests.tests, &123);
        },

       64 => {
           let tests = serde_yaml::from_str::<test64::TestArray>(&data).unwrap();
           run_tests(&tests.tests, &321);
       }
        _ => panic!("bad size"),
    };
}
