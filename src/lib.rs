#![feature(iterator_fold_self)]
#![feature(destructuring_assignment)]
#![feature(str_split_once)]
#![feature(drain_filter)]
#![feature(array_methods)]
#![feature(array_map)]
use aoc_runner_derive::aoc_lib;

pub mod point3d;
pub mod point4d;
pub mod pointnd;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;

aoc_lib! { year = 2020 }
