/*
 *
 * Unit tests from libieeep1788 for elementary interval functions
 * (Original author: Marco Nehmeier)
 * converted into portable ITL format by Oliver Heimlich.
 *
 * Copyright 2013-2015 Marco Nehmeier (nehmeier@informatik.uni-wuerzburg.de)
 * Copyright 2015-2017 Oliver Heimlich (oheim@posteo.de)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
//Language imports
#![rustfmt::skip]
#![allow(unused_attributes, unused_imports)]

//Test library imports

//Arithmetic library imports

//Preamble
use crate::util::*;
use hexf::*;
type D = inari::Decoration;
type DI = inari::DecoratedInterval;
type I = inari::Interval;

#[test]
fn minimal_neg_test() {
    assert_eq!(-n2i(1.0, 2.0), n2i(-2.0, -1.0));
    assert_eq!(-I::EMPTY, I::EMPTY);
    assert_eq!(-I::ENTIRE, I::ENTIRE);
    assert_eq!(-n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(-n2i(f64::NEG_INFINITY, 1.0), n2i(-1.0, f64::INFINITY));
    assert_eq!(-n2i(0.0, 2.0), n2i(-2.0, 0.0));
    assert_eq!(-n2i(-0.0, 2.0), n2i(-2.0, 0.0));
    assert_eq!(-n2i(-2.0, 0.0), n2i(0.0, 2.0));
    assert_eq!(-n2i(-2.0, -0.0), n2i(0.0, 2.0));
    assert_eq!(-n2i(0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(-n2i(-0.0, -0.0), n2i(0.0, 0.0));
}

#[test]
fn minimal_neg_dec_test() {
    assert!((-DI::NAI).is_nai());
    assert_eq!(-DI::EMPTY, DI::EMPTY);
    assert_eq!(-nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq!(-nd2di(1.0, 2.0, D::Com), nd2di(-2.0, -1.0, D::Com));
}

#[test]
fn minimal_add_test() {
    assert_eq!(I::EMPTY + I::EMPTY, I::EMPTY);
    assert_eq!(n2i(-1.0, 1.0) + I::EMPTY, I::EMPTY);
    assert_eq!(I::EMPTY + n2i(-1.0, 1.0), I::EMPTY);
    assert_eq!(I::EMPTY + I::ENTIRE, I::EMPTY);
    assert_eq!(I::ENTIRE + I::EMPTY, I::EMPTY);
    assert_eq!(I::ENTIRE + n2i(f64::NEG_INFINITY, 1.0), I::ENTIRE);
    assert_eq!(I::ENTIRE + n2i(-1.0, 1.0), I::ENTIRE);
    assert_eq!(I::ENTIRE + n2i(-1.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(I::ENTIRE + I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 1.0) + I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-1.0, 1.0) + I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY) + I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0) + n2i(f64::NEG_INFINITY, 4.0), n2i(f64::NEG_INFINITY, 6.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0) + n2i(3.0, 4.0), n2i(f64::NEG_INFINITY, 6.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0) + n2i(3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(1.0, 2.0) + n2i(f64::NEG_INFINITY, 4.0), n2i(f64::NEG_INFINITY, 6.0));
    assert_eq!(n2i(1.0, 2.0) + n2i(3.0, 4.0), n2i(4.0, 6.0));
    assert_eq!(n2i(1.0, 2.0) + n2i(3.0, f64::INFINITY), n2i(4.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY) + n2i(f64::NEG_INFINITY, 4.0), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY) + n2i(3.0, 4.0), n2i(4.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY) + n2i(3.0, f64::INFINITY), n2i(4.0, f64::INFINITY));
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")) + n2i(3.0, 4.0), n2i(4.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), 2.0) + n2i(-3.0, 4.0), n2i(f64::NEG_INFINITY, 6.0));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), 2.0) + n2i(-3.0, hexf64!("0x1.fffffffffffffp+1023")), I::ENTIRE);
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")) + n2i(0.0, 0.0), n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")) + n2i(-0.0, -0.0), n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(0.0, 0.0) + n2i(-3.0, 4.0), n2i(-3.0, 4.0));
    assert_eq!(n2i(-0.0, -0.0) + n2i(-3.0, hexf64!("0x1.fffffffffffffp+1023")), n2i(-3.0, hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(hexf64!("0x1.ffffffffffff0p+0"), hexf64!("0x1.ffffffffffff0p+0")) + n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")), n2i(hexf64!("0x1.0ccccccccccc4p+1"), hexf64!("0x1.0ccccccccccc5p+1")));
    assert_eq!(n2i(hexf64!("0x1.ffffffffffff0p+0"), hexf64!("0x1.ffffffffffff0p+0")) + n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("-0x1.999999999999ap-4")), n2i(hexf64!("0x1.e666666666656p+0"), hexf64!("0x1.e666666666657p+0")));
    assert_eq!(n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("0x1.ffffffffffff0p+0")) + n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")), n2i(hexf64!("-0x1.e666666666657p+0"), hexf64!("0x1.0ccccccccccc5p+1")));
}

#[test]
fn minimal_add_dec_test() {
    assert_eq!(nd2di(1.0, 2.0, D::Com) + nd2di(5.0, 7.0, D::Com), nd2di(6.0, 9.0, D::Com));
    assert_eq!(nd2di(1.0, 2.0, D::Com) + nd2di(5.0, 7.0, D::Def), nd2di(6.0, 9.0, D::Def));
    assert_eq!(nd2di(1.0, 2.0, D::Com) + nd2di(5.0, hexf64!("0x1.fffffffffffffp+1023"), D::Com), nd2di(6.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), 2.0, D::Com) + nd2di(-0.1, 5.0, D::Com), nd2di(f64::NEG_INFINITY, 7.0, D::Dac));
    assert_eq!(nd2di(1.0, 2.0, D::Trv) + DI::EMPTY, DI::EMPTY);
    assert!((DI::NAI + nd2di(1.0, 2.0, D::Trv)).is_nai());
}

#[test]
fn minimal_sub_test() {
    assert_eq!(I::EMPTY - I::EMPTY, I::EMPTY);
    assert_eq!(n2i(-1.0, 1.0) - I::EMPTY, I::EMPTY);
    assert_eq!(I::EMPTY - n2i(-1.0, 1.0), I::EMPTY);
    assert_eq!(I::EMPTY - I::ENTIRE, I::EMPTY);
    assert_eq!(I::ENTIRE - I::EMPTY, I::EMPTY);
    assert_eq!(I::ENTIRE - n2i(f64::NEG_INFINITY, 1.0), I::ENTIRE);
    assert_eq!(I::ENTIRE - n2i(-1.0, 1.0), I::ENTIRE);
    assert_eq!(I::ENTIRE - n2i(-1.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(I::ENTIRE - I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 1.0) - I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-1.0, 1.0) - I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY) - I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0) - n2i(f64::NEG_INFINITY, 4.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0) - n2i(3.0, 4.0), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.0) - n2i(3.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(1.0, 2.0) - n2i(f64::NEG_INFINITY, 4.0), n2i(-3.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 2.0) - n2i(3.0, 4.0), n2i(-3.0, -1.0));
    assert_eq!(n2i(1.0, 2.0) - n2i(3.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(1.0, f64::INFINITY) - n2i(f64::NEG_INFINITY, 4.0), n2i(-3.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY) - n2i(3.0, 4.0), n2i(-3.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY) - n2i(3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")) - n2i(-3.0, 4.0), n2i(-3.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), 2.0) - n2i(3.0, 4.0), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), 2.0) - n2i(hexf64!("-0x1.fffffffffffffp+1023"), 4.0), I::ENTIRE);
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")) - n2i(0.0, 0.0), n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")) - n2i(-0.0, -0.0), n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(0.0, 0.0) - n2i(-3.0, 4.0), n2i(-4.0, 3.0));
    assert_eq!(n2i(-0.0, -0.0) - n2i(-3.0, hexf64!("0x1.fffffffffffffp+1023")), n2i(hexf64!("-0x1.fffffffffffffp+1023"), 3.0));
    assert_eq!(n2i(hexf64!("0x1.ffffffffffff0p+0"), hexf64!("0x1.ffffffffffff0p+0")) - n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")), n2i(hexf64!("0x1.e666666666656p+0"), hexf64!("0x1.e666666666657p+0")));
    assert_eq!(n2i(hexf64!("0x1.ffffffffffff0p+0"), hexf64!("0x1.ffffffffffff0p+0")) - n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("-0x1.999999999999ap-4")), n2i(hexf64!("0x1.0ccccccccccc4p+1"), hexf64!("0x1.0ccccccccccc5p+1")));
    assert_eq!(n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("0x1.ffffffffffff0p+0")) - n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")), n2i(hexf64!("-0x1.0ccccccccccc5p+1"), hexf64!("0x1.e666666666657p+0")));
}

#[test]
fn minimal_sub_dec_test() {
    assert_eq!(nd2di(1.0, 2.0, D::Com) - nd2di(5.0, 7.0, D::Com), nd2di(-6.0, -3.0, D::Com));
    assert_eq!(nd2di(1.0, 2.0, D::Com) - nd2di(5.0, 7.0, D::Def), nd2di(-6.0, -3.0, D::Def));
    assert_eq!(nd2di(-1.0, 2.0, D::Com) - nd2di(5.0, hexf64!("0x1.fffffffffffffp+1023"), D::Com), nd2di(f64::NEG_INFINITY, -3.0, D::Dac));
    assert_eq!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), 2.0, D::Com) - nd2di(-1.0, 5.0, D::Com), nd2di(f64::NEG_INFINITY, 3.0, D::Dac));
    assert_eq!(nd2di(1.0, 2.0, D::Trv) - DI::EMPTY, DI::EMPTY);
    assert!((DI::NAI - nd2di(1.0, 2.0, D::Trv)).is_nai());
}

#[test]
fn minimal_mul_test() {
    assert_eq!(I::EMPTY * I::EMPTY, I::EMPTY);
    assert_eq!(n2i(-1.0, 1.0) * I::EMPTY, I::EMPTY);
    assert_eq!(I::EMPTY * n2i(-1.0, 1.0), I::EMPTY);
    assert_eq!(I::EMPTY * I::ENTIRE, I::EMPTY);
    assert_eq!(I::ENTIRE * I::EMPTY, I::EMPTY);
    assert_eq!(n2i(0.0, 0.0) * I::EMPTY, I::EMPTY);
    assert_eq!(I::EMPTY * n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0) * I::EMPTY, I::EMPTY);
    assert_eq!(I::EMPTY * n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(I::ENTIRE * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(I::ENTIRE * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(I::ENTIRE * n2i(-5.0, -1.0), I::ENTIRE);
    assert_eq!(I::ENTIRE * n2i(-5.0, 3.0), I::ENTIRE);
    assert_eq!(I::ENTIRE * n2i(1.0, 3.0), I::ENTIRE);
    assert_eq!(I::ENTIRE * n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE);
    assert_eq!(I::ENTIRE * n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(I::ENTIRE * n2i(-5.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(I::ENTIRE * n2i(1.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(I::ENTIRE * I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(-5.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(1.0, 3.0), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(-5.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY) * n2i(1.0, f64::INFINITY), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY) * I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 5.0));
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(-5.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(1.0, 3.0), n2i(-3.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(-5.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY) * n2i(1.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY) * I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(-5.0, -1.0), n2i(-15.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(-5.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 9.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(-5.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * n2i(1.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0) * I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(-5.0, -1.0), n2i(3.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(-5.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, -3.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(f64::NEG_INFINITY, -1.0), n2i(3.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(-5.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -3.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0) * I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(-5.0, -1.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(-5.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(1.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(f64::NEG_INFINITY, -1.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(f64::NEG_INFINITY, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(-5.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * n2i(1.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) * I::ENTIRE, n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(-5.0, -1.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(-5.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(1.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(f64::NEG_INFINITY, -1.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(f64::NEG_INFINITY, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(-5.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * n2i(1.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) * I::ENTIRE, n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(-5.0, -1.0), n2i(-25.0, -1.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(-5.0, 3.0), n2i(-25.0, 15.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(1.0, 3.0), n2i(1.0, 15.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 15.0));
    assert_eq!(n2i(1.0, 5.0) * n2i(-5.0, f64::INFINITY), n2i(-25.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0) * n2i(1.0, f64::INFINITY), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0) * I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-1.0, 5.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-1.0, 5.0) * n2i(-5.0, -1.0), n2i(-25.0, 5.0));
    //min max
    assert_eq!(n2i(-1.0, 5.0) * n2i(-5.0, 3.0), n2i(-25.0, 15.0));
    assert_eq!(n2i(-10.0, 2.0) * n2i(-5.0, 3.0), n2i(-30.0, 50.0));
    assert_eq!(n2i(-1.0, 5.0) * n2i(-1.0, 10.0), n2i(-10.0, 50.0));
    assert_eq!(n2i(-2.0, 2.0) * n2i(-5.0, 3.0), n2i(-10.0, 10.0));
    //end min max
    assert_eq!(n2i(-1.0, 5.0) * n2i(1.0, 3.0), n2i(-3.0, 15.0));
    assert_eq!(n2i(-1.0, 5.0) * n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0) * n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0) * n2i(-5.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0) * n2i(1.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0) * I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0) * n2i(0.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-10.0, -5.0) * n2i(-0.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-10.0, -5.0) * n2i(-5.0, -1.0), n2i(5.0, 50.0));
    assert_eq!(n2i(-10.0, -5.0) * n2i(-5.0, 3.0), n2i(-30.0, 50.0));
    assert_eq!(n2i(-10.0, -5.0) * n2i(1.0, 3.0), n2i(-30.0, -5.0));
    assert_eq!(n2i(-10.0, -5.0) * n2i(f64::NEG_INFINITY, -1.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0) * n2i(f64::NEG_INFINITY, 3.0), n2i(-30.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0) * n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 50.0));
    assert_eq!(n2i(-10.0, -5.0) * n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(-10.0, -5.0) * I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.ffffffffffff0p+0")) * n2i(hexf64!("-0x1.ffffffffffff0p+0"), f64::INFINITY), n2i(hexf64!("-0x1.fffffffffffe1p+1"), f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("0x1.ffffffffffff0p+0")) * n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("-0x1.999999999999ap-4")), n2i(hexf64!("-0x1.fffffffffffe1p+1"), hexf64!("0x1.999999999998ep-3")));
    assert_eq!(n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")) * n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("0x1.999999999999ap-4")), n2i(hexf64!("-0x1.999999999998ep-3"), hexf64!("0x1.999999999998ep-3")));
    assert_eq!(n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("-0x1.999999999999ap-4")) * n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.ffffffffffff0p+0")), n2i(hexf64!("-0x1.fffffffffffe1p+1"), hexf64!("-0x1.47ae147ae147bp-7")));
}

#[test]
fn minimal_mul_dec_test() {
    assert_eq!(nd2di(1.0, 2.0, D::Com) * nd2di(5.0, 7.0, D::Com), nd2di(5.0, 14.0, D::Com));
    assert_eq!(nd2di(1.0, 2.0, D::Com) * nd2di(5.0, 7.0, D::Def), nd2di(5.0, 14.0, D::Def));
    assert_eq!(nd2di(1.0, 2.0, D::Com) * nd2di(5.0, hexf64!("0x1.fffffffffffffp+1023"), D::Com), nd2di(5.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), 2.0, D::Com) * nd2di(-1.0, 5.0, D::Com), nd2di(f64::NEG_INFINITY, hexf64!("0x1.fffffffffffffp+1023"), D::Dac));
    assert_eq!(nd2di(1.0, 2.0, D::Trv) * DI::EMPTY, DI::EMPTY);
    assert!((DI::NAI * nd2di(1.0, 2.0, D::Trv)).is_nai());
}

#[test]
fn minimal_div_test() {
    assert_eq!(I::EMPTY / I::EMPTY, I::EMPTY);
    assert_eq!(n2i(-1.0, 1.0) / I::EMPTY, I::EMPTY);
    assert_eq!(I::EMPTY / n2i(-1.0, 1.0), I::EMPTY);
    assert_eq!(I::EMPTY / n2i(0.1, 1.0), I::EMPTY);
    assert_eq!(I::EMPTY / n2i(-1.0, -0.1), I::EMPTY);
    assert_eq!(I::EMPTY / I::ENTIRE, I::EMPTY);
    assert_eq!(I::ENTIRE / I::EMPTY, I::EMPTY);
    assert_eq!(n2i(0.0, 0.0) / I::EMPTY, I::EMPTY);
    assert_eq!(I::EMPTY / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0) / I::EMPTY, I::EMPTY);
    assert_eq!(I::EMPTY / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(I::ENTIRE / n2i(-5.0, -3.0), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(3.0, 5.0), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(f64::NEG_INFINITY, -3.0), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(I::ENTIRE / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(I::ENTIRE / n2i(-3.0, 0.0), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(-3.0, -0.0), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(0.0, 3.0), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(f64::NEG_INFINITY, 0.0), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(-0.0, 3.0), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(f64::NEG_INFINITY, -0.0), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(0.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(I::ENTIRE / n2i(-0.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(I::ENTIRE / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-30.0, -15.0) / n2i(-5.0, -3.0), n2i(3.0, 10.0));
    assert_eq!(n2i(-30.0, -15.0) / n2i(3.0, 5.0), n2i(-10.0, -3.0));
    assert_eq!(n2i(-30.0, -15.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, 10.0));
    assert_eq!(n2i(-30.0, -15.0) / n2i(3.0, f64::INFINITY), n2i(-10.0, 0.0));
    assert_eq!(n2i(-30.0, -15.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(-30.0, -15.0) / n2i(-3.0, 0.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -15.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(-30.0, -15.0) / n2i(-3.0, -0.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -15.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(-30.0, -15.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(-30.0, -15.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -15.0) / n2i(-0.0, 3.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(-30.0, -15.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -15.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(-30.0, -15.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-30.0, -15.0) / n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, -15.0) / n2i(-0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, -15.0) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-30.0, 15.0) / n2i(-5.0, -3.0), n2i(-5.0, 10.0));
    assert_eq!(n2i(-30.0, 15.0) / n2i(3.0, 5.0), n2i(-10.0, 5.0));
    assert_eq!(n2i(-30.0, 15.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(-5.0, 10.0));
    assert_eq!(n2i(-30.0, 15.0) / n2i(3.0, f64::INFINITY), n2i(-10.0, 5.0));
    assert_eq!(n2i(-30.0, 15.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(-30.0, 15.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(-30.0, 15.0) / n2i(-3.0, 0.0), I::ENTIRE);
    assert_eq!(n2i(-30.0, 15.0) / n2i(-3.0, -0.0), I::ENTIRE);
    assert_eq!(n2i(-30.0, 15.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(-30.0, 15.0) / n2i(0.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(-30.0, 15.0) / n2i(f64::NEG_INFINITY, 0.0), I::ENTIRE);
    assert_eq!(n2i(-30.0, 15.0) / n2i(-0.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(-30.0, 15.0) / n2i(f64::NEG_INFINITY, -0.0), I::ENTIRE);
    assert_eq!(n2i(-30.0, 15.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(-30.0, 15.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-30.0, 15.0) / n2i(0.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-30.0, 15.0) / n2i(-0.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-30.0, 15.0) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(15.0, 30.0) / n2i(-5.0, -3.0), n2i(-10.0, -3.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(3.0, 5.0), n2i(3.0, 10.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(-10.0, 0.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 10.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(15.0, 30.0) / n2i(-3.0, 0.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(15.0, 30.0) / n2i(-3.0, -0.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(15.0, 30.0) / n2i(0.0, 3.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(15.0, 30.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(-0.0, 3.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(15.0, 30.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(15.0, 30.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(15.0, 30.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(15.0, 30.0) / n2i(0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(15.0, 30.0) / n2i(-0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(15.0, 30.0) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0) / n2i(-5.0, -3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(3.0, 5.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0) / n2i(-3.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0) / n2i(-3.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(-3.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(0.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(-0.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(f64::NEG_INFINITY, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(-3.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(0.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / n2i(-0.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0) / I::ENTIRE, n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(-5.0, -3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(3.0, 5.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0) / n2i(-3.0, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0) / n2i(-3.0, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(-3.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(0.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(-0.0, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(f64::NEG_INFINITY, 3.0), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(-3.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(0.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / n2i(-0.0, f64::INFINITY), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0) / I::ENTIRE, n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-5.0, -3.0), n2i(3.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(3.0, 5.0), n2i(f64::NEG_INFINITY, -3.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(3.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-3.0, 0.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-3.0, -0.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-0.0, 3.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / n2i(-0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -15.0) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-5.0, -3.0), n2i(-5.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(3.0, 5.0), n2i(f64::NEG_INFINITY, 5.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(-5.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(3.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 5.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-3.0, 0.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-3.0, -0.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(0.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(f64::NEG_INFINITY, 0.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-0.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(f64::NEG_INFINITY, -0.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(0.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / n2i(-0.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 15.0) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-5.0, -3.0), n2i(f64::NEG_INFINITY, 5.0));
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(3.0, 5.0), n2i(-5.0, f64::INFINITY));
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -3.0), n2i(f64::NEG_INFINITY, 5.0));
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(3.0, f64::INFINITY), n2i(-5.0, f64::INFINITY));
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-3.0, 0.0), I::ENTIRE);
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-3.0, -0.0), I::ENTIRE);
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(0.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 0.0), I::ENTIRE);
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-0.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -0.0), I::ENTIRE);
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(0.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-15.0, f64::INFINITY) / n2i(-0.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-15.0, f64::INFINITY) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-5.0, -3.0), n2i(f64::NEG_INFINITY, -3.0));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(3.0, 5.0), n2i(3.0, f64::INFINITY));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(3.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-3.0, 0.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-3.0, -0.0), n2i(f64::NEG_INFINITY, -5.0));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(0.0, 3.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-0.0, 3.0), n2i(5.0, f64::INFINITY));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(15.0, f64::INFINITY) / n2i(-0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(15.0, f64::INFINITY) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-30.0, 0.0) / n2i(-5.0, -3.0), n2i(0.0, 10.0));
    assert_eq!(n2i(-30.0, 0.0) / n2i(3.0, 5.0), n2i(-10.0, 0.0));
    assert_eq!(n2i(-30.0, 0.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, 10.0));
    assert_eq!(n2i(-30.0, 0.0) / n2i(3.0, f64::INFINITY), n2i(-10.0, 0.0));
    assert_eq!(n2i(-30.0, 0.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(-30.0, 0.0) / n2i(-3.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, 0.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(-30.0, 0.0) / n2i(-3.0, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, 0.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(-30.0, 0.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, 0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, 0.0) / n2i(-0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, 0.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, 0.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(-30.0, 0.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-30.0, 0.0) / n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, 0.0) / n2i(-0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, 0.0) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-30.0, -0.0) / n2i(-5.0, -3.0), n2i(0.0, 10.0));
    assert_eq!(n2i(-30.0, -0.0) / n2i(3.0, 5.0), n2i(-10.0, 0.0));
    assert_eq!(n2i(-30.0, -0.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, 10.0));
    assert_eq!(n2i(-30.0, -0.0) / n2i(3.0, f64::INFINITY), n2i(-10.0, 0.0));
    assert_eq!(n2i(-30.0, -0.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(-30.0, -0.0) / n2i(-3.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -0.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(-30.0, -0.0) / n2i(-3.0, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -0.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(-30.0, -0.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, -0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -0.0) / n2i(-0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, -0.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-30.0, -0.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(-30.0, -0.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-30.0, -0.0) / n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, -0.0) / n2i(-0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-30.0, -0.0) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(0.0, 30.0) / n2i(-5.0, -3.0), n2i(-10.0, 0.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(3.0, 5.0), n2i(0.0, 10.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(-10.0, 0.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 10.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(0.0, 30.0) / n2i(-3.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(0.0, 30.0) / n2i(-3.0, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(0.0, 30.0) / n2i(0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 30.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(-0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 30.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, 30.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(0.0, 30.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(0.0, 30.0) / n2i(0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 30.0) / n2i(-0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 30.0) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-0.0, 30.0) / n2i(-5.0, -3.0), n2i(-10.0, 0.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(3.0, 5.0), n2i(0.0, 10.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(-10.0, 0.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(3.0, f64::INFINITY), n2i(0.0, 10.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(-0.0, 30.0) / n2i(-3.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(-0.0, 30.0) / n2i(-3.0, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(-0.0, 30.0) / n2i(0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, 30.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(-0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, 30.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, 30.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(-0.0, 30.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-0.0, 30.0) / n2i(0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, 30.0) / n2i(-0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, 30.0) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-5.0, -3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(3.0, 5.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(3.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-3.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-3.0, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / n2i(-0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-5.0, -3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(3.0, 5.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(f64::NEG_INFINITY, -3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(3.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-3.0, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-3.0, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(f64::NEG_INFINITY, 0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-0.0, 3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(f64::NEG_INFINITY, -0.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / n2i(-0.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-5.0, -3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(3.0, 5.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(3.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-3.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-3.0, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) / n2i(-0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-5.0, -3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(3.0, 5.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -3.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(3.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(0.0, 0.0), I::EMPTY);
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-3.0, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-0.0, -0.0), I::EMPTY);
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-3.0, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-3.0, 3.0), I::ENTIRE);
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-0.0, 3.0), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, -0.0), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE);
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-3.0, f64::INFINITY), I::ENTIRE);
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY) / n2i(-0.0, f64::INFINITY), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY) / I::ENTIRE, I::ENTIRE);
    assert_eq!(n2i(-2.0, -1.0) / n2i(-10.0, -3.0), n2i(hexf64!("0x1.9999999999999p-4"), hexf64!("0x1.5555555555556p-1")));
    assert_eq!(n2i(-2.0, -1.0) / n2i(0.0, 10.0), n2i(f64::NEG_INFINITY, hexf64!("-0x1.9999999999999p-4")));
    assert_eq!(n2i(-2.0, -1.0) / n2i(-0.0, 10.0), n2i(f64::NEG_INFINITY, hexf64!("-0x1.9999999999999p-4")));
    assert_eq!(n2i(-1.0, 2.0) / n2i(10.0, f64::INFINITY), n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-3")));
    assert_eq!(n2i(1.0, 3.0) / n2i(f64::NEG_INFINITY, -10.0), n2i(hexf64!("-0x1.3333333333334p-2"), 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -1.0) / n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, hexf64!("-0x1.5555555555555p-2")));
}

#[test]
fn minimal_div_dec_test() {
    assert_eq!(nd2di(-2.0, -1.0, D::Com) / nd2di(-10.0, -3.0, D::Com), nd2di(hexf64!("0x1.9999999999999p-4"), hexf64!("0x1.5555555555556p-1"), D::Com));
    assert_eq!(nd2di(-200.0, -1.0, D::Com) / nd2di(hexf64!("0x0.0000000000001p-1022"), 10.0, D::Com), nd2di(f64::NEG_INFINITY, hexf64!("-0x1.9999999999999p-4"), D::Dac));
    assert_eq!(nd2di(-2.0, -1.0, D::Com) / nd2di(0.0, 10.0, D::Com), nd2di(f64::NEG_INFINITY, hexf64!("-0x1.9999999999999p-4"), D::Trv));
    assert_eq!(nd2di(1.0, 3.0, D::Def) / nd2di(f64::NEG_INFINITY, -10.0, D::Dac), nd2di(hexf64!("-0x1.3333333333334p-2"), 0.0, D::Def));
    assert_eq!(nd2di(1.0, 2.0, D::Trv) / DI::EMPTY, DI::EMPTY);
    assert!((DI::NAI / nd2di(1.0, 2.0, D::Trv)).is_nai());
}

#[test]
fn minimal_recip_test() {
    assert_eq!(n2i(-50.0, -10.0).recip(), n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("-0x1.47ae147ae147ap-6")));
    assert_eq!(n2i(10.0, 50.0).recip(), n2i(hexf64!("0x1.47ae147ae147ap-6"), hexf64!("0x1.999999999999ap-4")));
    assert_eq!(n2i(f64::NEG_INFINITY, -10.0).recip(), n2i(hexf64!("-0x1.999999999999ap-4"), 0.0));
    assert_eq!(n2i(10.0, f64::INFINITY).recip(), n2i(0.0, hexf64!("0x1.999999999999ap-4")));
    assert_eq!(n2i(0.0, 0.0).recip(), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).recip(), I::EMPTY);
    assert_eq!(n2i(-10.0, 0.0).recip(), n2i(f64::NEG_INFINITY, hexf64!("-0x1.9999999999999p-4")));
    assert_eq!(n2i(-10.0, -0.0).recip(), n2i(f64::NEG_INFINITY, hexf64!("-0x1.9999999999999p-4")));
    assert_eq!(n2i(-10.0, 10.0).recip(), I::ENTIRE);
    assert_eq!(n2i(0.0, 10.0).recip(), n2i(hexf64!("0x1.9999999999999p-4"), f64::INFINITY));
    assert_eq!(n2i(-0.0, 10.0).recip(), n2i(hexf64!("0x1.9999999999999p-4"), f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).recip(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).recip(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 10.0).recip(), I::ENTIRE);
    assert_eq!(n2i(-10.0, f64::INFINITY).recip(), I::ENTIRE);
    assert_eq!(n2i(0.0, f64::INFINITY).recip(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).recip(), n2i(0.0, f64::INFINITY));
    assert_eq!(I::ENTIRE.recip(), I::ENTIRE);
}

#[test]
fn minimal_recip_dec_test() {
    assert_eq!(nd2di(10.0, 50.0, D::Com).recip(), nd2di(hexf64!("0x1.47ae147ae147ap-6"), hexf64!("0x1.999999999999ap-4"), D::Com));
    assert_eq!(nd2di(f64::NEG_INFINITY, -10.0, D::Dac).recip(), nd2di(hexf64!("-0x1.999999999999ap-4"), 0.0, D::Dac));
    assert_eq!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x0.0000000000001p-1022"), D::Def).recip(), nd2di(f64::NEG_INFINITY, hexf64!("-0x0.4000000000000p-1022"), D::Def));
    assert_eq!(nd2di(0.0, 0.0, D::Com).recip(), DI::EMPTY);
    assert_eq!(nd2di(-10.0, 0.0, D::Com).recip(), nd2di(f64::NEG_INFINITY, hexf64!("-0x1.9999999999999p-4"), D::Trv));
    assert_eq!(nd2di(-10.0, f64::INFINITY, D::Dac).recip(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(-0.0, f64::INFINITY, D::Dac).recip(), nd2di(0.0, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).recip(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
}

#[test]
fn minimal_sqr_test() {
    assert_eq!(I::EMPTY.sqr(), I::EMPTY);
    assert_eq!(I::ENTIRE.sqr(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, hexf64!("-0x0.0000000000001p-1022")).sqr(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, 1.0).sqr(), n2i(0.0, 1.0));
    assert_eq!(n2i(0.0, 1.0).sqr(), n2i(0.0, 1.0));
    assert_eq!(n2i(-0.0, 1.0).sqr(), n2i(0.0, 1.0));
    assert_eq!(n2i(-5.0, 3.0).sqr(), n2i(0.0, 25.0));
    assert_eq!(n2i(-5.0, 0.0).sqr(), n2i(0.0, 25.0));
    assert_eq!(n2i(-5.0, -0.0).sqr(), n2i(0.0, 25.0));
    assert_eq!(n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")).sqr(), n2i(hexf64!("0x1.47ae147ae147bp-7"), hexf64!("0x1.47ae147ae147cp-7")));
    assert_eq!(n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("0x1.999999999999ap-4")).sqr(), n2i(0.0, hexf64!("0x1.fffffffffffe1p+1")));
    assert_eq!(n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("-0x1.ffffffffffff0p+0")).sqr(), n2i(hexf64!("0x1.fffffffffffe0p+1"), hexf64!("0x1.fffffffffffe1p+1")));
}

#[test]
fn minimal_sqr_dec_test() {
    assert_eq!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x0.0000000000001p-1022"), D::Com).sqr(), nd2di(0.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(-1.0, 1.0, D::Def).sqr(), nd2di(0.0, 1.0, D::Def));
    assert_eq!(nd2di(-5.0, 3.0, D::Com).sqr(), nd2di(0.0, 25.0, D::Com));
    assert_eq!(nd2di(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4"), D::Com).sqr(), nd2di(hexf64!("0x1.47ae147ae147bp-7"), hexf64!("0x1.47ae147ae147cp-7"), D::Com));
}

#[test]
fn minimal_sqrt_test() {
    assert_eq!(I::EMPTY.sqrt(), I::EMPTY);
    assert_eq!(I::ENTIRE.sqrt(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, hexf64!("-0x0.0000000000001p-1022")).sqrt(), I::EMPTY);
    assert_eq!(n2i(-1.0, 1.0).sqrt(), n2i(0.0, 1.0));
    assert_eq!(n2i(0.0, 1.0).sqrt(), n2i(0.0, 1.0));
    assert_eq!(n2i(-0.0, 1.0).sqrt(), n2i(0.0, 1.0));
    assert_eq!(n2i(-5.0, 25.0).sqrt(), n2i(0.0, 5.0));
    assert_eq!(n2i(0.0, 25.0).sqrt(), n2i(0.0, 5.0));
    assert_eq!(n2i(-0.0, 25.0).sqrt(), n2i(0.0, 5.0));
    assert_eq!(n2i(-5.0, f64::INFINITY).sqrt(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")).sqrt(), n2i(hexf64!("0x1.43d136248490fp-2"), hexf64!("0x1.43d1362484910p-2")));
    assert_eq!(n2i(hexf64!("-0x1.ffffffffffff0p+0"), hexf64!("0x1.999999999999ap-4")).sqrt(), n2i(0.0, hexf64!("0x1.43d1362484910p-2")));
    assert_eq!(n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.ffffffffffff0p+0")).sqrt(), n2i(hexf64!("0x1.43d136248490fp-2"), hexf64!("0x1.6a09e667f3bc7p+0")));
}

#[test]
fn minimal_sqrt_dec_test() {
    assert_eq!(nd2di(1.0, 4.0, D::Com).sqrt(), nd2di(1.0, 2.0, D::Com));
    assert_eq!(nd2di(-5.0, 25.0, D::Com).sqrt(), nd2di(0.0, 5.0, D::Trv));
    assert_eq!(nd2di(0.0, 25.0, D::Def).sqrt(), nd2di(0.0, 5.0, D::Def));
    assert_eq!(nd2di(-5.0, f64::INFINITY, D::Dac).sqrt(), nd2di(0.0, f64::INFINITY, D::Trv));
}

#[test]
fn minimal_fma_test() {
    assert_eq!(I::EMPTY.mul_add(I::EMPTY, I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, 1.0).mul_add(I::EMPTY, I::EMPTY), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(-1.0, 1.0), I::EMPTY), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(I::EMPTY, I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(I::EMPTY, I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::EMPTY, I::EMPTY), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(n2i(1.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(1.0, 5.0).mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    //min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    //end min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-1.0, 5.0).mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-10.0, -5.0).mul_add(I::ENTIRE, I::EMPTY), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(I::EMPTY, n2i(f64::NEG_INFINITY, 2.0)), I::EMPTY);
    assert_eq!(n2i(-1.0, 1.0).mul_add(I::EMPTY, n2i(f64::NEG_INFINITY, 2.0)), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(-1.0, 1.0), n2i(f64::NEG_INFINITY, 2.0)), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(I::EMPTY, n2i(f64::NEG_INFINITY, 2.0)), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(I::EMPTY, n2i(f64::NEG_INFINITY, 2.0)), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::EMPTY, n2i(f64::NEG_INFINITY, 2.0)), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(I::ENTIRE.mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 7.0));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 11.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 17.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 17.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 17.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 7.0));
    //min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 17.0));
    assert_eq!(n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 52.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 52.0));
    assert_eq!(n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 12.0));
    //end min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 17.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 52.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 52.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, -3.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, 52.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(f64::NEG_INFINITY, 2.0)), n2i(f64::NEG_INFINITY, -3.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(I::ENTIRE, n2i(f64::NEG_INFINITY, 2.0)), I::ENTIRE);
    assert_eq!(I::EMPTY.mul_add(I::EMPTY, n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq!(n2i(-1.0, 1.0).mul_add(I::EMPTY, n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(-1.0, 1.0), n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(I::ENTIRE, n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(I::EMPTY, n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(I::EMPTY, n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::EMPTY, n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(I::ENTIRE.mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(I::ENTIRE, n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(I::ENTIRE, n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, 7.0));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(-5.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(I::ENTIRE, n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(-17.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, 11.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(I::ENTIRE, n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, -1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(I::ENTIRE, n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 0.0).mul_add(I::ENTIRE, n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::ENTIRE, n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(-27.0, 1.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), n2i(-27.0, 17.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(-1.0, 17.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, 1.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, 17.0));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(-27.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(I::ENTIRE, n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(-27.0, 7.0));
    //min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), n2i(-27.0, 17.0));
    assert_eq!(n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), n2i(-32.0, 52.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), n2i(-2.0, 2.0)), n2i(-12.0, 52.0));
    assert_eq!(n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), n2i(-12.0, 12.0));
    //end min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(-5.0, 17.0));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(I::ENTIRE, n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, 2.0)), n2i(-2.0, 2.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, 2.0)), n2i(3.0, 52.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, 2.0)), n2i(-32.0, 52.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, 2.0)), n2i(-32.0, -3.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, 2.0)), n2i(3.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, 2.0)), n2i(-32.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, 52.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, 2.0)), n2i(f64::NEG_INFINITY, -3.0));
    assert_eq!(n2i(-10.0, -5.0).mul_add(I::ENTIRE, n2i(-2.0, 2.0)), I::ENTIRE);
    assert_eq!(I::EMPTY.mul_add(I::EMPTY, n2i(-2.0, f64::INFINITY)), I::EMPTY);
    assert_eq!(n2i(-1.0, 1.0).mul_add(I::EMPTY, n2i(-2.0, f64::INFINITY)), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(-1.0, 1.0), n2i(-2.0, f64::INFINITY)), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(I::EMPTY, n2i(-2.0, f64::INFINITY)), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(I::EMPTY, n2i(-2.0, f64::INFINITY)), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::EMPTY, n2i(-2.0, f64::INFINITY)), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(I::ENTIRE.mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-5.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), n2i(-17.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), n2i(-27.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-27.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), n2i(-27.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 5.0).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), n2i(-27.0, f64::INFINITY));
    //min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-27.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-32.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), n2i(-2.0, f64::INFINITY)), n2i(-12.0, f64::INFINITY));
    assert_eq!(n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-12.0, f64::INFINITY));
    //end min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-5.0, f64::INFINITY));
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), n2i(-2.0, f64::INFINITY)), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), n2i(-2.0, f64::INFINITY)), n2i(3.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-32.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-32.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), n2i(-2.0, f64::INFINITY)), n2i(3.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-2.0, f64::INFINITY)), n2i(-32.0, f64::INFINITY));
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(I::ENTIRE, n2i(-2.0, f64::INFINITY)), I::ENTIRE);
    assert_eq!(I::EMPTY.mul_add(I::EMPTY, I::ENTIRE), I::EMPTY);
    assert_eq!(n2i(-1.0, 1.0).mul_add(I::EMPTY, I::ENTIRE), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(-1.0, 1.0), I::ENTIRE), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(I::ENTIRE, I::ENTIRE), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(I::EMPTY, I::ENTIRE), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).mul_add(I::EMPTY, I::ENTIRE), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::EMPTY, I::ENTIRE), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(0.0, 0.0), I::ENTIRE), I::EMPTY);
    assert_eq!(I::EMPTY.mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::EMPTY);
    assert_eq!(I::ENTIRE.mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(I::ENTIRE.mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, f64::INFINITY).mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, f64::INFINITY).mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 3.0).mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -3.0).mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-0.0, -0.0).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-0.0, -0.0).mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(1.0, 5.0).mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, -1.0), I::ENTIRE), I::ENTIRE);
    //min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-10.0, 2.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-1.0, 10.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-2.0, 2.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    //end min max
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-1.0, 5.0).mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(0.0, 0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-0.0, -0.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, -1.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(f64::NEG_INFINITY, 3.0), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(-5.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(n2i(1.0, f64::INFINITY), I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(-10.0, -5.0).mul_add(I::ENTIRE, I::ENTIRE), I::ENTIRE);
    assert_eq!(n2i(0.1, 0.5).mul_add(n2i(-5.0, 3.0), n2i(-0.1, 0.1)), n2i(hexf64!("-0x1.4cccccccccccdp+1"), hexf64!("0x1.999999999999ap+0")));
    assert_eq!(n2i(-0.5, 0.2).mul_add(n2i(-5.0, 3.0), n2i(-0.1, 0.1)), n2i(hexf64!("-0x1.999999999999ap+0"), hexf64!("0x1.4cccccccccccdp+1")));
    assert_eq!(n2i(-0.5, -0.1).mul_add(n2i(2.0, 3.0), n2i(-0.1, 0.1)), n2i(hexf64!("-0x1.999999999999ap+0"), hexf64!("-0x1.999999999999ap-4")));
    assert_eq!(n2i(-0.5, -0.1).mul_add(n2i(f64::NEG_INFINITY, 3.0), n2i(-0.1, 0.1)), n2i(hexf64!("-0x1.999999999999ap+0"), f64::INFINITY));
}

#[test]
fn minimal_fma_dec_test() {
    assert_eq!(nd2di(-0.5, -0.1, D::Com).mul_add(nd2di(f64::NEG_INFINITY, 3.0, D::Dac), nd2di(-0.1, 0.1, D::Com)), nd2di(hexf64!("-0x1.999999999999ap+0"), f64::INFINITY, D::Dac));
    assert_eq!(nd2di(1.0, 2.0, D::Com).mul_add(nd2di(1.0, hexf64!("0x1.fffffffffffffp+1023"), D::Com), nd2di(0.0, 1.0, D::Com)), nd2di(1.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(1.0, 2.0, D::Com).mul_add(nd2di(1.0, 2.0, D::Com), nd2di(2.0, 5.0, D::Com)), nd2di(3.0, 9.0, D::Com));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_pown_test() {
    assert_eq!(I::EMPTY.pown(0), I::EMPTY);
    assert_eq!(I::ENTIRE.pown(0), n2i(1.0, 1.0));
    assert_eq!(n2i(0.0, 0.0).pown(0), n2i(1.0, 1.0));
    assert_eq!(n2i(-0.0, -0.0).pown(0), n2i(1.0, 1.0));
    assert_eq!(n2i(13.1, 13.1).pown(0), n2i(1.0, 1.0));
    assert_eq!(n2i(-7451.145, -7451.145).pown(0), n2i(1.0, 1.0));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).pown(0), n2i(1.0, 1.0));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")).pown(0), n2i(1.0, 1.0));
    assert_eq!(n2i(0.0, f64::INFINITY).pown(0), n2i(1.0, 1.0));
    assert_eq!(n2i(-0.0, f64::INFINITY).pown(0), n2i(1.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).pown(0), n2i(1.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).pown(0), n2i(1.0, 1.0));
    assert_eq!(n2i(-324.3, 2.5).pown(0), n2i(1.0, 1.0));
    assert_eq!(I::EMPTY.pown(2), I::EMPTY);
    assert_eq!(I::ENTIRE.pown(2), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).pown(2), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).pown(2), n2i(0.0, 0.0));
    assert_eq!(n2i(13.1, 13.1).pown(2), n2i(hexf64!("0x1.573851eb851ebp+7"), hexf64!("0x1.573851eb851ecp+7")));
    assert_eq!(n2i(-7451.145, -7451.145).pown(2), n2i(hexf64!("0x1.a794a4e7cfaadp+25"), hexf64!("0x1.a794a4e7cfaaep+25")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).pown(2), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")).pown(2), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY).pown(2), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).pown(2), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).pown(2), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).pown(2), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-324.3, 2.5).pown(2), n2i(0.0, hexf64!("0x1.9ad27d70a3d72p+16")));
    assert_eq!(n2i(0.01, 2.33).pown(2), n2i(hexf64!("0x1.a36e2eb1c432cp-14"), hexf64!("0x1.5b7318fc50482p+2")));
    assert_eq!(n2i(-1.9, -0.33).pown(2), n2i(hexf64!("0x1.be0ded288ce70p-4"), hexf64!("0x1.ce147ae147ae1p+1")));
    assert_eq!(I::EMPTY.pown(8), I::EMPTY);
    assert_eq!(I::ENTIRE.pown(8), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).pown(8), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).pown(8), n2i(0.0, 0.0));
    assert_eq!(n2i(13.1, 13.1).pown(8), n2i(hexf64!("0x1.9d8fd495853f5p+29"), hexf64!("0x1.9d8fd495853f6p+29")));
    assert_eq!(n2i(-7451.145, -7451.145).pown(8), n2i(hexf64!("0x1.dfb1bb622e70dp+102"), hexf64!("0x1.dfb1bb622e70ep+102")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).pown(8), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")).pown(8), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY).pown(8), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).pown(8), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).pown(8), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).pown(8), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-324.3, 2.5).pown(8), n2i(0.0, hexf64!("0x1.a875871096550p+66")));
    assert_eq!(n2i(0.01, 2.33).pown(8), n2i(hexf64!("0x1.cd2b297d889bdp-54"), hexf64!("0x1.b253d9f33ce4dp+9")));
    assert_eq!(n2i(-1.9, -0.33).pown(8), n2i(hexf64!("0x1.26f1fcdd502a3p-13"), hexf64!("0x1.53abd7bfc4fc6p+7")));
    assert_eq!(I::EMPTY.pown(1), I::EMPTY);
    assert_eq!(I::ENTIRE.pown(1), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).pown(1), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).pown(1), n2i(0.0, 0.0));
    assert_eq!(n2i(13.1, 13.1).pown(1), n2i(13.1, 13.1));
    assert_eq!(n2i(-7451.145, -7451.145).pown(1), n2i(-7451.145, -7451.145));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).pown(1), n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")).pown(1), n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(0.0, f64::INFINITY).pown(1), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).pown(1), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).pown(1), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).pown(1), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-324.3, 2.5).pown(1), n2i(-324.3, 2.5));
    assert_eq!(n2i(0.01, 2.33).pown(1), n2i(0.01, 2.33));
    assert_eq!(n2i(-1.9, -0.33).pown(1), n2i(-1.9, -0.33));
    assert_eq!(I::EMPTY.pown(3), I::EMPTY);
    assert_eq!(I::ENTIRE.pown(3), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).pown(3), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).pown(3), n2i(0.0, 0.0));
    assert_eq!(n2i(13.1, 13.1).pown(3), n2i(hexf64!("0x1.1902e978d4fdep+11"), hexf64!("0x1.1902e978d4fdfp+11")));
    assert_eq!(n2i(-7451.145, -7451.145).pown(3), n2i(hexf64!("-0x1.81460637b9a3dp+38"), hexf64!("-0x1.81460637b9a3cp+38")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).pown(3), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")).pown(3), n2i(f64::NEG_INFINITY, hexf64!("-0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(0.0, f64::INFINITY).pown(3), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).pown(3), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).pown(3), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).pown(3), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-324.3, 2.5).pown(3), n2i(hexf64!("-0x1.0436d2f418938p+25"), hexf64!("0x1.f400000000000p+3")));
    assert_eq!(n2i(0.01, 2.33).pown(3), n2i(hexf64!("0x1.0c6f7a0b5ed8dp-20"), hexf64!("0x1.94c75e6362a60p+3")));
    assert_eq!(n2i(-1.9, -0.33).pown(3), n2i(hexf64!("-0x1.b6f9db22d0e55p+2"), hexf64!("-0x1.266559f6ec5b1p-5")));
    assert_eq!(I::EMPTY.pown(7), I::EMPTY);
    assert_eq!(I::ENTIRE.pown(7), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).pown(7), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).pown(7), n2i(0.0, 0.0));
    assert_eq!(n2i(13.1, 13.1).pown(7), n2i(hexf64!("0x1.f91d1b185493bp+25"), hexf64!("0x1.f91d1b185493cp+25")));
    assert_eq!(n2i(-7451.145, -7451.145).pown(7), n2i(hexf64!("-0x1.07b1da32f9b59p+90"), hexf64!("-0x1.07b1da32f9b58p+90")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).pown(7), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")).pown(7), n2i(f64::NEG_INFINITY, hexf64!("-0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(0.0, f64::INFINITY).pown(7), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).pown(7), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).pown(7), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).pown(7), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-324.3, 2.5).pown(7), n2i(hexf64!("-0x1.4f109959e6d7fp+58"), hexf64!("0x1.312d000000000p+9")));
    assert_eq!(n2i(0.01, 2.33).pown(7), n2i(hexf64!("0x1.6849b86a12b9bp-47"), hexf64!("0x1.74d0373c76313p+8")));
    assert_eq!(n2i(-1.9, -0.33).pown(7), n2i(hexf64!("-0x1.658c775099757p+6"), hexf64!("-0x1.bee30301bf47ap-12")));
    assert_eq!(I::EMPTY.pown(-2), I::EMPTY);
    assert_eq!(I::ENTIRE.pown(-2), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).pown(-2), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).pown(-2), I::EMPTY);
    assert_eq!(n2i(13.1, 13.1).pown(-2), n2i(hexf64!("0x1.7de3a077d1568p-8"), hexf64!("0x1.7de3a077d1569p-8")));
    assert_eq!(n2i(-7451.145, -7451.145).pown(-2), n2i(hexf64!("0x1.3570290cd6e14p-26"), hexf64!("0x1.3570290cd6e15p-26")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).pown(-2), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")).pown(-2), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(0.0, f64::INFINITY).pown(-2), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).pown(-2), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).pown(-2), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).pown(-2), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-324.3, 2.5).pown(-2), n2i(hexf64!("0x1.3f0c482c977c9p-17"), f64::INFINITY));
    assert_eq!(n2i(0.01, 2.33).pown(-2), n2i(hexf64!("0x1.793d85ef38e47p-3"), hexf64!("0x1.3880000000000p+13")));
    assert_eq!(n2i(-1.9, -0.33).pown(-2), n2i(hexf64!("0x1.1ba81104f6c80p-2"), hexf64!("0x1.25d8fa1f801e1p+3")));
    assert_eq!(I::EMPTY.pown(-8), I::EMPTY);
    assert_eq!(I::ENTIRE.pown(-8), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).pown(-8), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).pown(-8), I::EMPTY);
    assert_eq!(n2i(13.1, 13.1).pown(-8), n2i(hexf64!("0x1.3cef39247ca6dp-30"), hexf64!("0x1.3cef39247ca6ep-30")));
    assert_eq!(n2i(-7451.145, -7451.145).pown(-8), n2i(hexf64!("0x1.113d9ef0a99acp-103"), hexf64!("0x1.113d9ef0a99adp-103")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).pown(-8), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")).pown(-8), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(0.0, f64::INFINITY).pown(-8), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).pown(-8), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).pown(-8), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).pown(-8), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-324.3, 2.5).pown(-8), n2i(hexf64!("0x1.34cc3764d1e0cp-67"), f64::INFINITY));
    assert_eq!(n2i(0.01, 2.33).pown(-8), n2i(hexf64!("0x1.2dc80db11ab7cp-10"), hexf64!("0x1.1c37937e08000p+53")));
    assert_eq!(n2i(-1.9, -0.33).pown(-8), n2i(hexf64!("0x1.81e104e61630dp-8"), hexf64!("0x1.bc64f21560e34p+12")));
    assert_eq!(I::EMPTY.pown(-1), I::EMPTY);
    assert_eq!(I::ENTIRE.pown(-1), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).pown(-1), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).pown(-1), I::EMPTY);
    assert_eq!(n2i(13.1, 13.1).pown(-1), n2i(hexf64!("0x1.38abf82ee6986p-4"), hexf64!("0x1.38abf82ee6987p-4")));
    assert_eq!(n2i(-7451.145, -7451.145).pown(-1), n2i(hexf64!("-0x1.197422c9048bfp-13"), hexf64!("-0x1.197422c9048bep-13")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).pown(-1), n2i(hexf64!("0x0.4000000000000p-1022"), hexf64!("0x0.4000000000001p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")).pown(-1), n2i(hexf64!("-0x0.4000000000001p-1022"), hexf64!("-0x0.4000000000000p-1022")));
    assert_eq!(n2i(0.0, f64::INFINITY).pown(-1), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).pown(-1), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).pown(-1), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).pown(-1), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-324.3, 2.5).pown(-1), I::ENTIRE);
    assert_eq!(n2i(0.01, 2.33).pown(-1), n2i(hexf64!("0x1.b77c278dbbe13p-2"), hexf64!("0x1.9000000000000p+6")));
    assert_eq!(n2i(-1.9, -0.33).pown(-1), n2i(hexf64!("-0x1.83e0f83e0f83ep+1"), hexf64!("-0x1.0d79435e50d79p-1")));
    assert_eq!(I::EMPTY.pown(-3), I::EMPTY);
    assert_eq!(I::ENTIRE.pown(-3), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).pown(-3), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).pown(-3), I::EMPTY);
    assert_eq!(n2i(13.1, 13.1).pown(-3), n2i(hexf64!("0x1.d26df4d8b1831p-12"), hexf64!("0x1.d26df4d8b1832p-12")));
    assert_eq!(n2i(-7451.145, -7451.145).pown(-3), n2i(hexf64!("-0x1.54347ded91b19p-39"), hexf64!("-0x1.54347ded91b18p-39")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).pown(-3), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")).pown(-3), n2i(hexf64!("-0x0.0000000000001p-1022"), hexf64!("-0x0.0p+0")));
    assert_eq!(n2i(0.0, f64::INFINITY).pown(-3), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).pown(-3), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).pown(-3), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).pown(-3), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-324.3, 2.5).pown(-3), I::ENTIRE);
    assert_eq!(n2i(0.01, 2.33).pown(-3), n2i(hexf64!("0x1.43cfba61aacabp-4"), hexf64!("0x1.e848000000000p+19")));
    assert_eq!(n2i(-1.9, -0.33).pown(-3), n2i(hexf64!("-0x1.bd393ce9e8e7cp+4"), hexf64!("-0x1.2a95f6f7c066cp-3")));
    assert_eq!(I::EMPTY.pown(-7), I::EMPTY);
    assert_eq!(I::ENTIRE.pown(-7), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).pown(-7), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).pown(-7), I::EMPTY);
    assert_eq!(n2i(13.1, 13.1).pown(-7), n2i(hexf64!("0x1.037d76c912dbcp-26"), hexf64!("0x1.037d76c912dbdp-26")));
    assert_eq!(n2i(-7451.145, -7451.145).pown(-7), n2i(hexf64!("-0x1.f10f41fb8858fp-91"), hexf64!("-0x1.f10f41fb8858ep-91")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")).pown(-7), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")).pown(-7), n2i(hexf64!("-0x0.0000000000001p-1022"), hexf64!("-0x0.0p+0")));
    assert_eq!(n2i(0.0, f64::INFINITY).pown(-7), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).pown(-7), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).pown(-7), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).pown(-7), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-324.3, 2.5).pown(-7), I::ENTIRE);
    assert_eq!(n2i(0.01, 2.33).pown(-7), n2i(hexf64!("0x1.5f934d64162a9p-9"), hexf64!("0x1.6bcc41e900000p+46")));
    assert_eq!(n2i(-1.9, -0.33).pown(-7), n2i(hexf64!("-0x1.254cdd3711ddbp+11"), hexf64!("-0x1.6e95c4a761e19p-7")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_pown_dec_test() {
    assert_eq!(nd2di(-5.0, 10.0, D::Com).pown(0), nd2di(1.0, 1.0, D::Com));
    assert_eq!(nd2di(f64::NEG_INFINITY, 15.0, D::Dac).pown(0), nd2di(1.0, 1.0, D::Dac));
    assert_eq!(nd2di(-3.0, 5.0, D::Def).pown(2), nd2di(0.0, 25.0, D::Def));
    assert_eq!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), 2.0, D::Com).pown(2), nd2di(0.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(-3.0, 5.0, D::Dac).pown(3), nd2di(-27.0, 125.0, D::Dac));
    assert_eq!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), 2.0, D::Com).pown(3), nd2di(f64::NEG_INFINITY, 8.0, D::Dac));
    assert_eq!(nd2di(3.0, 5.0, D::Com).pown(-2), nd2di(hexf64!("0x1.47ae147ae147ap-5"), hexf64!("0x1.c71c71c71c71dp-4"), D::Com));
    assert_eq!(nd2di(-5.0, -3.0, D::Def).pown(-2), nd2di(hexf64!("0x1.47ae147ae147ap-5"), hexf64!("0x1.c71c71c71c71dp-4"), D::Def));
    assert_eq!(nd2di(-5.0, 3.0, D::Com).pown(-2), nd2di(hexf64!("0x1.47ae147ae147ap-5"), f64::INFINITY, D::Trv));
    assert_eq!(nd2di(3.0, 5.0, D::Dac).pown(-3), nd2di(hexf64!("0x1.0624dd2f1a9fbp-7"), hexf64!("0x1.2f684bda12f69p-5"), D::Dac));
    assert_eq!(nd2di(-3.0, 5.0, D::Com).pown(-3), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_exp_test() {
    assert_eq!(I::EMPTY.exp(), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).exp(), n2i(0.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).exp(), n2i(0.0, 1.0));
    assert_eq!(n2i(0.0, f64::INFINITY).exp(), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).exp(), n2i(1.0, f64::INFINITY));
    assert_eq!(I::ENTIRE.exp(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, hexf64!("0x1.62e42fefa39f0p+9")).exp(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("0x1.62e42fefa39f0p+9"), hexf64!("0x1.62e42fefa39f0p+9")).exp(), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(n2i(0.0, hexf64!("0x1.62e42fefa39e0p+9")).exp(), n2i(1.0, hexf64!("0x1.fffffffffc32bp+1023")));
    assert_eq!(n2i(-0.0, hexf64!("0x1.62e42fefa39e0p+9")).exp(), n2i(1.0, hexf64!("0x1.fffffffffc32bp+1023")));
    assert_eq!(n2i(hexf64!("-0x1.6232bdd7abcd3p+9"), hexf64!("0x1.62e42fefa39e0p+9")).exp(), n2i(hexf64!("0x0.ffffffffffe7bp-1022"), hexf64!("0x1.fffffffffc32bp+1023")));
    assert_eq!(n2i(hexf64!("-0x1.6232bdd7abcd3p+8"), hexf64!("0x1.62e42fefa39e0p+9")).exp(), n2i(hexf64!("0x1.ffffffffffe7bp-512"), hexf64!("0x1.fffffffffc32bp+1023")));
    assert_eq!(n2i(hexf64!("-0x1.6232bdd7abcd3p+8"), 0.0).exp(), n2i(hexf64!("0x1.ffffffffffe7bp-512"), 1.0));
    assert_eq!(n2i(hexf64!("-0x1.6232bdd7abcd3p+8"), -0.0).exp(), n2i(hexf64!("0x1.ffffffffffe7bp-512"), 1.0));
    assert_eq!(n2i(hexf64!("-0x1.6232bdd7abcd3p+8"), 1.0).exp(), n2i(hexf64!("0x1.ffffffffffe7bp-512"), hexf64!("0x1.5bf0a8b14576ap+1")));
    assert_eq!(n2i(1.0, 5.0).exp(), n2i(hexf64!("0x1.5bf0a8b145769p+1"), hexf64!("0x1.28d3899703390p+7")));
    assert_eq!(n2i(hexf64!("-0x1.a934f0979a372p+1"), hexf64!("0x1.ceaecfea8085ap+0")).exp(), n2i(hexf64!("0x1.2797f0a337a5fp-5"), hexf64!("0x1.86091cc9095c5p+2")));
    assert_eq!(n2i(hexf64!("0x1.87f42b972949cp-1"), hexf64!("0x1.8b55484710029p+6")).exp(), n2i(hexf64!("0x1.1337e9e45812ap+1"), hexf64!("0x1.805a5c88021b6p+142")));
    assert_eq!(n2i(hexf64!("0x1.78025c8b3fd39p+3"), hexf64!("0x1.9fd8eef3fa79bp+4")).exp(), n2i(hexf64!("0x1.ef461a783114cp+16"), hexf64!("0x1.691d36c6b008cp+37")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_exp_dec_test() {
    assert_eq!(nd2di(hexf64!("0x1.62e42fefa39f0p+9"), hexf64!("0x1.62e42fefa39f0p+9"), D::Com).exp(), nd2di(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY, D::Dac));
    assert_eq!(nd2di(0.0, hexf64!("0x1.62e42fefa39e0p+9"), D::Def).exp(), nd2di(1.0, hexf64!("0x1.fffffffffc32bp+1023"), D::Def));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_exp2_test() {
    assert_eq!(I::EMPTY.exp2(), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).exp2(), n2i(0.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).exp2(), n2i(0.0, 1.0));
    assert_eq!(n2i(0.0, f64::INFINITY).exp2(), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).exp2(), n2i(1.0, f64::INFINITY));
    assert_eq!(I::ENTIRE.exp2(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 1024.0).exp2(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(1024.0, 1024.0).exp2(), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(n2i(0.0, 1023.0).exp2(), n2i(1.0, hexf64!("0x1.0000000000000p+1023")));
    assert_eq!(n2i(-0.0, 1023.0).exp2(), n2i(1.0, hexf64!("0x1.0000000000000p+1023")));
    assert_eq!(n2i(-1022.0, 1023.0).exp2(), n2i(hexf64!("0x1.0000000000000p-1022"), hexf64!("0x1.0000000000000p+1023")));
    assert_eq!(n2i(-1022.0, 0.0).exp2(), n2i(hexf64!("0x1.0000000000000p-1022"), 1.0));
    assert_eq!(n2i(-1022.0, -0.0).exp2(), n2i(hexf64!("0x1.0000000000000p-1022"), 1.0));
    assert_eq!(n2i(-1022.0, 1.0).exp2(), n2i(hexf64!("0x1.0000000000000p-1022"), 2.0));
    assert_eq!(n2i(1.0, 5.0).exp2(), n2i(2.0, 32.0));
    assert_eq!(n2i(hexf64!("-0x1.a934f0979a372p+1"), hexf64!("0x1.ceaecfea8085ap+0")).exp2(), n2i(hexf64!("0x1.9999999999998p-4"), hexf64!("0x1.c000000000001p+1")));
    assert_eq!(n2i(hexf64!("0x1.87f42b972949cp-1"), hexf64!("0x1.8b55484710029p+6")).exp2(), n2i(hexf64!("0x1.b333333333332p+0"), hexf64!("0x1.c81fd88228b4fp+98")));
    assert_eq!(n2i(hexf64!("0x1.78025c8b3fd39p+3"), hexf64!("0x1.9fd8eef3fa79bp+4")).exp2(), n2i(hexf64!("0x1.aea0000721857p+11"), hexf64!("0x1.fca0555555559p+25")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_exp2_dec_test() {
    assert_eq!(nd2di(1024.0, 1024.0, D::Com).exp2(), nd2di(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY, D::Dac));
    assert_eq!(nd2di(hexf64!("0x1.87f42b972949cp-1"), hexf64!("0x1.8b55484710029p+6"), D::Def).exp2(), nd2di(hexf64!("0x1.b333333333332p+0"), hexf64!("0x1.c81fd88228b4fp+98"), D::Def));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_exp10_test() {
    assert_eq!(I::EMPTY.exp10(), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).exp10(), n2i(0.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).exp10(), n2i(0.0, 1.0));
    assert_eq!(n2i(0.0, f64::INFINITY).exp10(), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).exp10(), n2i(1.0, f64::INFINITY));
    assert_eq!(I::ENTIRE.exp10(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, hexf64!("0x1.34413509f79ffp+8")).exp10(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("0x1.34413509f79ffp+8"), hexf64!("0x1.34413509f79ffp+8")).exp10(), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(n2i(0.0, hexf64!("0x1.34413509f79fep+8")).exp10(), n2i(1.0, hexf64!("0x1.ffffffffffba1p+1023")));
    assert_eq!(n2i(-0.0, hexf64!("0x1.34413509f79fep+8")).exp10(), n2i(1.0, hexf64!("0x1.ffffffffffba1p+1023")));
    assert_eq!(n2i(hexf64!("-0x1.33a7146f72a42p+8"), hexf64!("0x1.34413509f79fep+8")).exp10(), n2i(hexf64!("0x0.fffffffffffe3p-1022"), hexf64!("0x1.ffffffffffba1p+1023")));
    assert_eq!(n2i(hexf64!("-0x1.2200000000000p+7"), hexf64!("0x1.34413509f79fep+8")).exp10(), n2i(hexf64!("0x1.3faac3e3fa1f3p-482"), hexf64!("0x1.ffffffffffba1p+1023")));
    assert_eq!(n2i(hexf64!("-0x1.2200000000000p+7"), 0.0).exp10(), n2i(hexf64!("0x1.3faac3e3fa1f3p-482"), 1.0));
    assert_eq!(n2i(hexf64!("-0x1.2200000000000p+7"), -0.0).exp10(), n2i(hexf64!("0x1.3faac3e3fa1f3p-482"), 1.0));
    assert_eq!(n2i(hexf64!("-0x1.2200000000000p+7"), 1.0).exp10(), n2i(hexf64!("0x1.3faac3e3fa1f3p-482"), 10.0));
    assert_eq!(n2i(1.0, 5.0).exp10(), n2i(10.0, 100000.0));
    assert_eq!(n2i(hexf64!("-0x1.a934f0979a372p+1"), hexf64!("0x1.ceaecfea8085ap+0")).exp10(), n2i(hexf64!("0x1.f3a8254311f9ap-12"), hexf64!("0x1.00b18ad5b7d56p+6")));
    assert_eq!(n2i(hexf64!("0x1.87f42b972949cp-1"), hexf64!("0x1.8b55484710029p+6")).exp10(), n2i(hexf64!("0x1.75014b7296807p+2"), hexf64!("0x1.3eec1d47dfb2bp+328")));
    assert_eq!(n2i(hexf64!("0x1.78025c8b3fd39p+3"), hexf64!("0x1.9fd8eef3fa79bp+4")).exp10(), n2i(hexf64!("0x1.0608d2279a811p+39"), hexf64!("0x1.43af5d4271cb8p+86")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_exp10_dec_test() {
    assert_eq!(nd2di(hexf64!("0x1.34413509f79ffp+8"), hexf64!("0x1.34413509f79ffp+8"), D::Com).exp10(), nd2di(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY, D::Dac));
    assert_eq!(nd2di(hexf64!("0x1.87f42b972949cp-1"), hexf64!("0x1.8b55484710029p+6"), D::Def).exp10(), nd2di(hexf64!("0x1.75014b7296807p+2"), hexf64!("0x1.3eec1d47dfb2bp+328"), D::Def));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_log_test() {
    assert_eq!(I::EMPTY.log(), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).log(), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).log(), I::EMPTY);
    assert_eq!(n2i(0.0, 1.0).log(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, 1.0).log(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(1.0, f64::INFINITY).log(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY).log(), I::ENTIRE);
    assert_eq!(n2i(-0.0, f64::INFINITY).log(), I::ENTIRE);
    assert_eq!(I::ENTIRE.log(), I::ENTIRE);
    assert_eq!(n2i(0.0, hexf64!("0x1.fffffffffffffp+1023")).log(), n2i(f64::NEG_INFINITY, hexf64!("0x1.62e42fefa39f0p+9")));
    assert_eq!(n2i(-0.0, hexf64!("0x1.fffffffffffffp+1023")).log(), n2i(f64::NEG_INFINITY, hexf64!("0x1.62e42fefa39f0p+9")));
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")).log(), n2i(0.0, hexf64!("0x1.62e42fefa39f0p+9")));
    assert_eq!(n2i(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x1.fffffffffffffp+1023")).log(), n2i(hexf64!("-0x1.74385446d71c4p+9"), hexf64!("0x1.62e42fefa39f0p+9")));
    assert_eq!(n2i(hexf64!("0x0.0000000000001p-1022"), 1.0).log(), n2i(hexf64!("-0x1.74385446d71c4p+9"), 0.0));
    assert_eq!(n2i(hexf64!("0x1.5bf0a8b145769p+1"), hexf64!("0x1.5bf0a8b145769p+1")).log(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.5bf0a8b14576ap+1"), hexf64!("0x1.5bf0a8b14576ap+1")).log(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x1.5bf0a8b14576ap+1")).log(), n2i(hexf64!("-0x1.74385446d71c4p+9"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("0x1.5bf0a8b145769p+1"), 32.0).log(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.bb9d3beb8c86cp+1")));
    assert_eq!(n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.c000000000000p+1")).log(), n2i(hexf64!("-0x1.26bb1bbb55516p+1"), hexf64!("0x1.40b512eb53d60p+0")));
    assert_eq!(n2i(hexf64!("0x1.b333333333333p+0"), hexf64!("0x1.c81fd88228b2fp+98")).log(), n2i(hexf64!("0x1.0fae81914a990p-1"), hexf64!("0x1.120627f6ae7f1p+6")));
    assert_eq!(n2i(hexf64!("0x1.aea0000721861p+11"), hexf64!("0x1.fca055555554cp+25")).log(), n2i(hexf64!("0x1.04a1363db1e63p+3"), hexf64!("0x1.203e52c0256b5p+4")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_log_dec_test() {
    assert_eq!(nd2di(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x1.fffffffffffffp+1023"), D::Com).log(), nd2di(hexf64!("-0x1.74385446d71c4p+9"), hexf64!("0x1.62e42fefa39f0p+9"), D::Com));
    assert_eq!(nd2di(0.0, 1.0, D::Com).log(), nd2di(f64::NEG_INFINITY, 0.0, D::Trv));
    assert_eq!(nd2di(hexf64!("0x1.5bf0a8b14576ap+1"), hexf64!("0x1.5bf0a8b14576ap+1"), D::Def).log(), nd2di(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0"), D::Def));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_log2_test() {
    assert_eq!(I::EMPTY.log2(), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).log2(), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).log2(), I::EMPTY);
    assert_eq!(n2i(0.0, 1.0).log2(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, 1.0).log2(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(1.0, f64::INFINITY).log2(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY).log2(), I::ENTIRE);
    assert_eq!(n2i(-0.0, f64::INFINITY).log2(), I::ENTIRE);
    assert_eq!(I::ENTIRE.log2(), I::ENTIRE);
    assert_eq!(n2i(0.0, hexf64!("0x1.fffffffffffffp+1023")).log2(), n2i(f64::NEG_INFINITY, 1024.0));
    assert_eq!(n2i(-0.0, hexf64!("0x1.fffffffffffffp+1023")).log2(), n2i(f64::NEG_INFINITY, 1024.0));
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")).log2(), n2i(0.0, 1024.0));
    assert_eq!(n2i(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x1.fffffffffffffp+1023")).log2(), n2i(-1074.0, 1024.0));
    assert_eq!(n2i(hexf64!("0x0.0000000000001p-1022"), 1.0).log2(), n2i(-1074.0, 0.0));
    assert_eq!(n2i(hexf64!("0x0.0000000000001p-1022"), 2.0).log2(), n2i(-1074.0, 1.0));
    assert_eq!(n2i(2.0, 32.0).log2(), n2i(1.0, 5.0));
    assert_eq!(n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.c000000000000p+1")).log2(), n2i(hexf64!("-0x1.a934f0979a372p+1"), hexf64!("0x1.ceaecfea8085ap+0")));
    assert_eq!(n2i(hexf64!("0x1.b333333333333p+0"), hexf64!("0x1.c81fd88228b2fp+98")).log2(), n2i(hexf64!("0x1.87f42b972949cp-1"), hexf64!("0x1.8b55484710029p+6")));
    assert_eq!(n2i(hexf64!("0x1.aea0000721861p+11"), hexf64!("0x1.fca055555554cp+25")).log2(), n2i(hexf64!("0x1.78025c8b3fd39p+3"), hexf64!("0x1.9fd8eef3fa79bp+4")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_log2_dec_test() {
    assert_eq!(nd2di(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x1.fffffffffffffp+1023"), D::Com).log2(), nd2di(-1074.0, 1024.0, D::Com));
    assert_eq!(nd2di(hexf64!("0x0.0000000000001p-1022"), f64::INFINITY, D::Dac).log2(), nd2di(-1074.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(2.0, 32.0, D::Def).log2(), nd2di(1.0, 5.0, D::Def));
    assert_eq!(nd2di(0.0, hexf64!("0x1.fffffffffffffp+1023"), D::Com).log2(), nd2di(f64::NEG_INFINITY, 1024.0, D::Trv));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_log10_test() {
    assert_eq!(I::EMPTY.log10(), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).log10(), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).log10(), I::EMPTY);
    assert_eq!(n2i(0.0, 1.0).log10(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(-0.0, 1.0).log10(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(1.0, f64::INFINITY).log10(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(0.0, f64::INFINITY).log10(), I::ENTIRE);
    assert_eq!(n2i(-0.0, f64::INFINITY).log10(), I::ENTIRE);
    assert_eq!(I::ENTIRE.log10(), I::ENTIRE);
    assert_eq!(n2i(0.0, hexf64!("0x1.fffffffffffffp+1023")).log10(), n2i(f64::NEG_INFINITY, hexf64!("0x1.34413509f79ffp+8")));
    assert_eq!(n2i(-0.0, hexf64!("0x1.fffffffffffffp+1023")).log10(), n2i(f64::NEG_INFINITY, hexf64!("0x1.34413509f79ffp+8")));
    assert_eq!(n2i(1.0, hexf64!("0x1.fffffffffffffp+1023")).log10(), n2i(0.0, hexf64!("0x1.34413509f79ffp+8")));
    assert_eq!(n2i(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x1.fffffffffffffp+1023")).log10(), n2i(hexf64!("-0x1.434e6420f4374p+8"), hexf64!("0x1.34413509f79ffp+8")));
    assert_eq!(n2i(hexf64!("0x0.0000000000001p-1022"), 1.0).log10(), n2i(hexf64!("-0x1.434e6420f4374p+8"), 0.0));
    assert_eq!(n2i(hexf64!("0x0.0000000000001p-1022"), 10.0).log10(), n2i(hexf64!("-0x1.434e6420f4374p+8"), 1.0));
    assert_eq!(n2i(10.0, 100000.0).log10(), n2i(1.0, 5.0));
    assert_eq!(n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.c000000000000p+1")).log10(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.1690163290f40p-1")));
    assert_eq!(n2i(hexf64!("0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")).log10(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.fffffffffffffp-1")));
    assert_eq!(n2i(hexf64!("0x1.b333333333333p+0"), hexf64!("0x1.c81fd88228b2fp+98")).log10(), n2i(hexf64!("0x1.d7f59aa5becb9p-3"), hexf64!("0x1.dc074d84e5aabp+4")));
    assert_eq!(n2i(hexf64!("0x1.aea0000721861p+11"), hexf64!("0x1.fca055555554cp+25")).log10(), n2i(hexf64!("0x1.c4c29dd829191p+1"), hexf64!("0x1.f4baebba4fa40p+2")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_log10_dec_test() {
    assert_eq!(nd2di(hexf64!("0x0.0000000000001p-1022"), hexf64!("0x1.fffffffffffffp+1023"), D::Com).log10(), nd2di(hexf64!("-0x1.434e6420f4374p+8"), hexf64!("0x1.34413509f79ffp+8"), D::Com));
    assert_eq!(nd2di(0.0, hexf64!("0x1.fffffffffffffp+1023"), D::Dac).log10(), nd2di(f64::NEG_INFINITY, hexf64!("0x1.34413509f79ffp+8"), D::Trv));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_sin_test() {
    assert_eq!(I::EMPTY.sin(), I::EMPTY);
    assert_eq!(n2i(0.0, f64::INFINITY).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-0.0, f64::INFINITY).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).sin(), n2i(-1.0, 1.0));
    assert_eq!(I::ENTIRE.sin(), n2i(-1.0, 1.0));
    assert_eq!(n2i(0.0, 0.0).sin(), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).sin(), n2i(0.0, 0.0));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d18p+0")).sin(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")).sin(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")).sin(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(0.0, hexf64!("0x1.921fb54442d18p+0")).sin(), n2i(0.0, hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(-0.0, hexf64!("0x1.921fb54442d18p+0")).sin(), n2i(0.0, hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(0.0, hexf64!("0x1.921fb54442d19p+0")).sin(), n2i(0.0, hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(-0.0, hexf64!("0x1.921fb54442d19p+0")).sin(), n2i(0.0, hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d18p+1")).sin(), n2i(hexf64!("0x1.1a62633145c06p-53"), hexf64!("0x1.1a62633145c07p-53")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")).sin(), n2i(hexf64!("-0x1.72cece675d1fdp-52"), hexf64!("-0x1.72cece675d1fcp-52")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")).sin(), n2i(hexf64!("-0x1.72cece675d1fdp-52"), hexf64!("0x1.1a62633145c07p-53")));
    assert_eq!(n2i(0.0, hexf64!("0x1.921fb54442d18p+1")).sin(), n2i(0.0, 1.0));
    assert_eq!(n2i(-0.0, hexf64!("0x1.921fb54442d18p+1")).sin(), n2i(0.0, 1.0));
    assert_eq!(n2i(0.0, hexf64!("0x1.921fb54442d19p+1")).sin(), n2i(hexf64!("-0x1.72cece675d1fdp-52"), 1.0));
    assert_eq!(n2i(-0.0, hexf64!("0x1.921fb54442d19p+1")).sin(), n2i(hexf64!("-0x1.72cece675d1fdp-52"), 1.0));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d18p+1")).sin(), n2i(hexf64!("0x1.1a62633145c06p-53"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1")).sin(), n2i(hexf64!("-0x1.72cece675d1fdp-52"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d18p+1")).sin(), n2i(hexf64!("0x1.1a62633145c06p-53"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+1")).sin(), n2i(hexf64!("-0x1.72cece675d1fdp-52"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+0"), hexf64!("-0x1.921fb54442d18p+0")).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.fffffffffffffp-1")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d19p+0")).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.fffffffffffffp-1")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.fffffffffffffp-1")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+0"), 0.0).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), 0.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+0"), -0.0).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), 0.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+0"), 0.0).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), 0.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+0"), -0.0).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), 0.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+1"), hexf64!("-0x1.921fb54442d18p+1")).sin(), n2i(hexf64!("-0x1.1a62633145c07p-53"), hexf64!("-0x1.1a62633145c06p-53")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("-0x1.921fb54442d19p+1")).sin(), n2i(hexf64!("0x1.72cece675d1fcp-52"), hexf64!("0x1.72cece675d1fdp-52")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("-0x1.921fb54442d18p+1")).sin(), n2i(hexf64!("-0x1.1a62633145c07p-53"), hexf64!("0x1.72cece675d1fdp-52")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+1"), 0.0).sin(), n2i(-1.0, 0.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+1"), -0.0).sin(), n2i(-1.0, 0.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+1"), 0.0).sin(), n2i(-1.0, hexf64!("0x1.72cece675d1fdp-52")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+1"), -0.0).sin(), n2i(-1.0, hexf64!("0x1.72cece675d1fdp-52")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+1"), hexf64!("-0x1.921fb54442d18p+0")).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.1a62633145c06p-53")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("-0x1.921fb54442d18p+0")).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.72cece675d1fdp-52")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+1"), hexf64!("-0x1.921fb54442d19p+0")).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.1a62633145c06p-53")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("-0x1.921fb54442d19p+0")).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.72cece675d1fdp-52")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d18p+0")).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d18p+0")).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(-0.7, 0.1).sin(), n2i(hexf64!("-0x1.49d6e694619b9p-1"), hexf64!("0x1.98eaecb8bcb2dp-4")));
    assert_eq!(n2i(1.0, 2.0).sin(), n2i(hexf64!("0x1.aed548f090ceep-1"), 1.0));
    assert_eq!(n2i(-3.2, -2.9).sin(), n2i(hexf64!("-0x1.e9fb8d64830e3p-3"), hexf64!("0x1.de33739e82d33p-5")));
    assert_eq!(n2i(2.0, 3.0).sin(), n2i(hexf64!("0x1.210386db6d55bp-3"), hexf64!("0x1.d18f6ead1b446p-1")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_sin_dec_test() {
    assert_eq!(nd2di(hexf64!("-0x1.921fb54442d18p+1"), hexf64!("-0x1.921fb54442d18p+0"), D::Def).sin(), nd2di(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.1a62633145c06p-53"), D::Def));
    assert_eq!(nd2di(f64::NEG_INFINITY, -0.0, D::Trv).sin(), nd2di(-1.0, 1.0, D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).sin(), nd2di(-1.0, 1.0, D::Dac));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_cos_test() {
    assert_eq!(I::EMPTY.cos(), I::EMPTY);
    assert_eq!(n2i(0.0, f64::INFINITY).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-0.0, f64::INFINITY).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(I::ENTIRE.cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(0.0, 0.0).cos(), n2i(1.0, 1.0));
    assert_eq!(n2i(-0.0, -0.0).cos(), n2i(1.0, 1.0));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d18p+0")).cos(), n2i(hexf64!("0x1.1a62633145c06p-54"), hexf64!("0x1.1a62633145c07p-54")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")).cos(), n2i(hexf64!("-0x1.72cece675d1fdp-53"), hexf64!("-0x1.72cece675d1fcp-53")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")).cos(), n2i(hexf64!("-0x1.72cece675d1fdp-53"), hexf64!("0x1.1a62633145c07p-54")));
    assert_eq!(n2i(0.0, hexf64!("0x1.921fb54442d18p+0")).cos(), n2i(hexf64!("0x1.1a62633145c06p-54"), 1.0));
    assert_eq!(n2i(-0.0, hexf64!("0x1.921fb54442d18p+0")).cos(), n2i(hexf64!("0x1.1a62633145c06p-54"), 1.0));
    assert_eq!(n2i(0.0, hexf64!("0x1.921fb54442d19p+0")).cos(), n2i(hexf64!("-0x1.72cece675d1fdp-53"), 1.0));
    assert_eq!(n2i(-0.0, hexf64!("0x1.921fb54442d19p+0")).cos(), n2i(hexf64!("-0x1.72cece675d1fdp-53"), 1.0));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d18p+1")).cos(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.fffffffffffffp-1")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")).cos(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.fffffffffffffp-1")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")).cos(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.fffffffffffffp-1")));
    assert_eq!(n2i(0.0, hexf64!("0x1.921fb54442d18p+1")).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-0.0, hexf64!("0x1.921fb54442d18p+1")).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(0.0, hexf64!("0x1.921fb54442d19p+1")).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-0.0, hexf64!("0x1.921fb54442d19p+1")).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d18p+1")).cos(), n2i(-1.0, hexf64!("0x1.1a62633145c07p-54")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1")).cos(), n2i(-1.0, hexf64!("0x1.1a62633145c07p-54")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d18p+1")).cos(), n2i(-1.0, hexf64!("-0x1.72cece675d1fcp-53")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+1")).cos(), n2i(-1.0, hexf64!("-0x1.72cece675d1fcp-53")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+0"), hexf64!("-0x1.921fb54442d18p+0")).cos(), n2i(hexf64!("0x1.1a62633145c06p-54"), hexf64!("0x1.1a62633145c07p-54")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d19p+0")).cos(), n2i(hexf64!("-0x1.72cece675d1fdp-53"), hexf64!("-0x1.72cece675d1fcp-53")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")).cos(), n2i(hexf64!("-0x1.72cece675d1fdp-53"), hexf64!("0x1.1a62633145c07p-54")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+0"), 0.0).cos(), n2i(hexf64!("0x1.1a62633145c06p-54"), 1.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+0"), -0.0).cos(), n2i(hexf64!("0x1.1a62633145c06p-54"), 1.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+0"), 0.0).cos(), n2i(hexf64!("-0x1.72cece675d1fdp-53"), 1.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+0"), -0.0).cos(), n2i(hexf64!("-0x1.72cece675d1fdp-53"), 1.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+1"), hexf64!("-0x1.921fb54442d18p+1")).cos(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.fffffffffffffp-1")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("-0x1.921fb54442d19p+1")).cos(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.fffffffffffffp-1")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("-0x1.921fb54442d18p+1")).cos(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.fffffffffffffp-1")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+1"), 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+1"), -0.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+1"), 0.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+1"), -0.0).cos(), n2i(-1.0, 1.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+1"), hexf64!("-0x1.921fb54442d18p+0")).cos(), n2i(-1.0, hexf64!("0x1.1a62633145c07p-54")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("-0x1.921fb54442d18p+0")).cos(), n2i(-1.0, hexf64!("0x1.1a62633145c07p-54")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+1"), hexf64!("-0x1.921fb54442d19p+0")).cos(), n2i(-1.0, hexf64!("-0x1.72cece675d1fcp-53")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("-0x1.921fb54442d19p+0")).cos(), n2i(-1.0, hexf64!("-0x1.72cece675d1fcp-53")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d18p+0")).cos(), n2i(hexf64!("0x1.1a62633145c06p-54"), 1.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")).cos(), n2i(hexf64!("-0x1.72cece675d1fdp-53"), 1.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d18p+0")).cos(), n2i(hexf64!("-0x1.72cece675d1fdp-53"), 1.0));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")).cos(), n2i(hexf64!("-0x1.72cece675d1fdp-53"), 1.0));
    assert_eq!(n2i(-0.7, 0.1).cos(), n2i(hexf64!("0x1.87996529f9d92p-1"), 1.0));
    assert_eq!(n2i(1.0, 2.0).cos(), n2i(hexf64!("-0x1.aa22657537205p-2"), hexf64!("0x1.14a280fb5068cp-1")));
    assert_eq!(n2i(-3.2, -2.9).cos(), n2i(-1.0, hexf64!("-0x1.f1216dba340c8p-1")));
    assert_eq!(n2i(2.0, 3.0).cos(), n2i(hexf64!("-0x1.fae04be85e5d3p-1"), hexf64!("-0x1.aa22657537204p-2")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_cos_dec_test() {
    assert_eq!(nd2di(hexf64!("-0x1.921fb54442d18p+0"), hexf64!("-0x1.921fb54442d18p+0"), D::Trv).cos(), nd2di(hexf64!("0x1.1a62633145c06p-54"), hexf64!("0x1.1a62633145c07p-54"), D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, -0.0, D::Def).cos(), nd2di(-1.0, 1.0, D::Def));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).cos(), nd2di(-1.0, 1.0, D::Dac));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_tan_test() {
    assert_eq!(I::EMPTY.tan(), I::EMPTY);
    assert_eq!(n2i(0.0, f64::INFINITY).tan(), I::ENTIRE);
    assert_eq!(n2i(-0.0, f64::INFINITY).tan(), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).tan(), I::ENTIRE);
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).tan(), I::ENTIRE);
    assert_eq!(I::ENTIRE.tan(), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).tan(), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).tan(), n2i(0.0, 0.0));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d18p+0")).tan(), n2i(hexf64!("0x1.d02967c31cdb4p+53"), hexf64!("0x1.d02967c31cdb5p+53")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")).tan(), n2i(hexf64!("-0x1.617a15494767bp+52"), hexf64!("-0x1.617a15494767ap+52")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")).tan(), I::ENTIRE);
    assert_eq!(n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d18p+1")).tan(), n2i(hexf64!("-0x1.1a62633145c07p-53"), hexf64!("-0x1.1a62633145c06p-53")));
    assert_eq!(n2i(hexf64!("0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")).tan(), n2i(hexf64!("0x1.72cece675d1fcp-52"), hexf64!("0x1.72cece675d1fdp-52")));
    assert_eq!(n2i(0.0, hexf64!("0x1.921fb54442d18p+0")).tan(), n2i(0.0, hexf64!("0x1.d02967c31cdb5p+53")));
    assert_eq!(n2i(-0.0, hexf64!("0x1.921fb54442d18p+0")).tan(), n2i(0.0, hexf64!("0x1.d02967c31cdb5p+53")));
    assert_eq!(n2i(0.0, hexf64!("0x1.921fb54442d19p+0")).tan(), I::ENTIRE);
    assert_eq!(n2i(-0.0, hexf64!("0x1.921fb54442d19p+0")).tan(), I::ENTIRE);
    assert_eq!(n2i(0.0, hexf64!("0x1.921fb54442d18p+1")).tan(), I::ENTIRE);
    assert_eq!(n2i(-0.0, hexf64!("0x1.921fb54442d18p+1")).tan(), I::ENTIRE);
    assert_eq!(n2i(0.0, hexf64!("0x1.921fb54442d19p+1")).tan(), I::ENTIRE);
    assert_eq!(n2i(-0.0, hexf64!("0x1.921fb54442d19p+1")).tan(), I::ENTIRE);
    assert_eq!(n2i(hexf64!("0x1.0000000000000p-51"), hexf64!("0x1.921fb54442d18p+1")).tan(), I::ENTIRE);
    assert_eq!(n2i(hexf64!("0x1.0000000000000p-51"), hexf64!("0x1.921fb54442d19p+1")).tan(), I::ENTIRE);
    assert_eq!(n2i(hexf64!("0x1.0000000000000p-52"), hexf64!("0x1.921fb54442d18p+1")).tan(), I::ENTIRE);
    assert_eq!(n2i(hexf64!("0x1.0000000000000p-52"), hexf64!("0x1.921fb54442d19p+1")).tan(), I::ENTIRE);
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d18p+0")).tan(), n2i(hexf64!("-0x1.d02967c31cdb5p+53"), hexf64!("0x1.d02967c31cdb5p+53")));
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")).tan(), I::ENTIRE);
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d18p+0")).tan(), I::ENTIRE);
    assert_eq!(n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")).tan(), I::ENTIRE);
    assert_eq!(n2i(hexf64!("-0x1.555475a31a4bep-2"), hexf64!("0x1.999999999999ap-4")).tan(), n2i(hexf64!("-0x1.628f4fd931fefp-2"), hexf64!("0x1.9af8877430b81p-4")));
    assert_eq!(n2i(hexf64!("0x1.4e18e147ae148p+12"), hexf64!("0x1.4e2028f5c28f6p+12")).tan(), n2i(hexf64!("-0x1.d6d67b035b6b4p+2"), hexf64!("-0x1.7e42b0760e3f3p+0")));
    assert_eq!(n2i(hexf64!("0x1.4e18e147ae148p+12"), hexf64!("0x1.546028f5c28f6p+12")).tan(), I::ENTIRE);
    assert_eq!(n2i(hexf64!("0x1.fae147ae147aep-1"), hexf64!("0x1.028f5c28f5c29p+0")).tan(), n2i(hexf64!("0x1.860fadcc59064p+0"), hexf64!("0x1.979ad0628469dp+0")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_tan_dec_test() {
    assert_eq!(DI::EMPTY.tan(), DI::EMPTY);
    assert_eq!(nd2di(0.0, f64::INFINITY, D::Dac).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(-0.0, f64::INFINITY, D::Def).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, 0.0, D::Trv).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, -0.0, D::Dac).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(0.0, 0.0, D::Com).tan(), nd2di(0.0, 0.0, D::Com));
    assert_eq!(nd2di(-0.0, -0.0, D::Def).tan(), nd2di(0.0, 0.0, D::Def));
    assert_eq!(nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d18p+0"), D::Com).tan(), nd2di(hexf64!("0x1.d02967c31cdb4p+53"), hexf64!("0x1.d02967c31cdb5p+53"), D::Com));
    assert_eq!(nd2di(hexf64!("0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Def).tan(), nd2di(hexf64!("-0x1.617a15494767bp+52"), hexf64!("-0x1.617a15494767ap+52"), D::Def));
    assert_eq!(nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Dac).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d18p+1"), D::Trv).tan(), nd2di(hexf64!("-0x1.1a62633145c07p-53"), hexf64!("-0x1.1a62633145c06p-53"), D::Trv));
    assert_eq!(nd2di(hexf64!("0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Com).tan(), nd2di(hexf64!("0x1.72cece675d1fcp-52"), hexf64!("0x1.72cece675d1fdp-52"), D::Com));
    assert_eq!(nd2di(0.0, hexf64!("0x1.921fb54442d18p+0"), D::Dac).tan(), nd2di(0.0, hexf64!("0x1.d02967c31cdb5p+53"), D::Dac));
    assert_eq!(nd2di(-0.0, hexf64!("0x1.921fb54442d18p+0"), D::Com).tan(), nd2di(0.0, hexf64!("0x1.d02967c31cdb5p+53"), D::Com));
    assert_eq!(nd2di(0.0, hexf64!("0x1.921fb54442d19p+0"), D::Trv).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(-0.0, hexf64!("0x1.921fb54442d19p+0"), D::Def).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(0.0, hexf64!("0x1.921fb54442d18p+1"), D::Dac).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(-0.0, hexf64!("0x1.921fb54442d18p+1"), D::Com).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Trv).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(-0.0, hexf64!("0x1.921fb54442d19p+1"), D::Def).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(hexf64!("0x1.0000000000000p-51"), hexf64!("0x1.921fb54442d18p+1"), D::Dac).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(hexf64!("0x1.0000000000000p-51"), hexf64!("0x1.921fb54442d19p+1"), D::Com).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(hexf64!("0x1.0000000000000p-52"), hexf64!("0x1.921fb54442d18p+1"), D::Trv).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(hexf64!("0x1.0000000000000p-52"), hexf64!("0x1.921fb54442d19p+1"), D::Def).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(hexf64!("-0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d18p+0"), D::Com).tan(), nd2di(hexf64!("-0x1.d02967c31cdb5p+53"), hexf64!("0x1.d02967c31cdb5p+53"), D::Com));
    assert_eq!(nd2di(hexf64!("-0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d18p+0"), D::Def).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Dac).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(hexf64!("-0x1.555475a31a4bep-2"), hexf64!("0x1.999999999999ap-4"), D::Com).tan(), nd2di(hexf64!("-0x1.628f4fd931fefp-2"), hexf64!("0x1.9af8877430b81p-4"), D::Com));
    assert_eq!(nd2di(hexf64!("0x1.4e18e147ae148p+12"), hexf64!("0x1.4e2028f5c28f6p+12"), D::Dac).tan(), nd2di(hexf64!("-0x1.d6d67b035b6b4p+2"), hexf64!("-0x1.7e42b0760e3f3p+0"), D::Dac));
    assert_eq!(nd2di(hexf64!("0x1.4e18e147ae148p+12"), hexf64!("0x1.546028f5c28f6p+12"), D::Def).tan(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(hexf64!("0x1.fae147ae147aep-1"), hexf64!("0x1.028f5c28f5c29p+0"), D::Trv).tan(), nd2di(hexf64!("0x1.860fadcc59064p+0"), hexf64!("0x1.979ad0628469dp+0"), D::Trv));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_asin_test() {
    assert_eq!(I::EMPTY.asin(), I::EMPTY);
    assert_eq!(n2i(0.0, f64::INFINITY).asin(), n2i(0.0, hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-0.0, f64::INFINITY).asin(), n2i(0.0, hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).asin(), n2i(hexf64!("-0x1.921fb54442d19p+0"), 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).asin(), n2i(hexf64!("-0x1.921fb54442d19p+0"), 0.0));
    assert_eq!(I::ENTIRE.asin(), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-1.0, 1.0).asin(), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(f64::NEG_INFINITY, -1.0).asin(), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(1.0, f64::INFINITY).asin(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-1.0, -1.0).asin(), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(1.0, 1.0).asin(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.0, 0.0).asin(), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).asin(), n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, hexf64!("-0x1.0000000000001p+0")).asin(), I::EMPTY);
    assert_eq!(n2i(hexf64!("0x1.0000000000001p+0"), f64::INFINITY).asin(), I::EMPTY);
    assert_eq!(n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")).asin(), n2i(hexf64!("-0x1.9a49276037885p-4"), hexf64!("0x1.9a49276037885p-4")));
    assert_eq!(n2i(hexf64!("-0x1.51eb851eb851fp-2"), hexf64!("0x1.fffffffffffffp-1")).asin(), n2i(hexf64!("-0x1.585ff6e341c3fp-2"), hexf64!("0x1.921fb50442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp-1"), hexf64!("0x1.fffffffffffffp-1")).asin(), n2i(hexf64!("-0x1.921fb50442d19p+0"), hexf64!("0x1.921fb50442d19p+0")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_asin_dec_test() {
    assert_eq!(nd2di(0.0, f64::INFINITY, D::Dac).asin(), nd2di(0.0, hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, 0.0, D::Def).asin(), nd2di(hexf64!("-0x1.921fb54442d19p+0"), 0.0, D::Trv));
    assert_eq!(nd2di(-1.0, 1.0, D::Com).asin(), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Com));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).asin(), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(hexf64!("-0x1.51eb851eb851fp-2"), hexf64!("0x1.fffffffffffffp-1"), D::Def).asin(), nd2di(hexf64!("-0x1.585ff6e341c3fp-2"), hexf64!("0x1.921fb50442d19p+0"), D::Def));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_acos_test() {
    assert_eq!(I::EMPTY.acos(), I::EMPTY);
    assert_eq!(n2i(0.0, f64::INFINITY).acos(), n2i(0.0, hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-0.0, f64::INFINITY).acos(), n2i(0.0, hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(I::ENTIRE.acos(), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-1.0, 1.0).acos(), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(f64::NEG_INFINITY, -1.0).acos(), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(1.0, f64::INFINITY).acos(), n2i(0.0, 0.0));
    assert_eq!(n2i(-1.0, -1.0).acos(), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(1.0, 1.0).acos(), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-0.0, -0.0).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(f64::NEG_INFINITY, hexf64!("-0x1.0000000000001p+0")).acos(), I::EMPTY);
    assert_eq!(n2i(hexf64!("0x1.0000000000001p+0"), f64::INFINITY).acos(), I::EMPTY);
    assert_eq!(n2i(hexf64!("-0x1.999999999999ap-4"), hexf64!("0x1.999999999999ap-4")).acos(), n2i(hexf64!("0x1.787b22ce3f590p+0"), hexf64!("0x1.abc447ba464a1p+0")));
    assert_eq!(n2i(hexf64!("-0x1.51eb851eb851fp-2"), hexf64!("0x1.fffffffffffffp-1")).acos(), n2i(hexf64!("0x1.0000000000000p-26"), hexf64!("0x1.e837b2fd13428p+0")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp-1"), hexf64!("0x1.fffffffffffffp-1")).acos(), n2i(hexf64!("0x1.0000000000000p-26"), hexf64!("0x1.921fb52442d19p+1")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_acos_dec_test() {
    assert_eq!(nd2di(0.0, f64::INFINITY, D::Dac).acos(), nd2di(0.0, hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, 0.0, D::Def).acos(), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-1.0, 1.0, D::Com).acos(), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Com));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).acos(), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(hexf64!("-0x1.51eb851eb851fp-2"), hexf64!("0x1.fffffffffffffp-1"), D::Def).acos(), nd2di(hexf64!("0x1.0000000000000p-26"), hexf64!("0x1.e837b2fd13428p+0"), D::Def));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_atan_test() {
    assert_eq!(I::EMPTY.atan(), I::EMPTY);
    assert_eq!(n2i(0.0, f64::INFINITY).atan(), n2i(0.0, hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-0.0, f64::INFINITY).atan(), n2i(0.0, hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).atan(), n2i(hexf64!("-0x1.921fb54442d19p+0"), 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).atan(), n2i(hexf64!("-0x1.921fb54442d19p+0"), 0.0));
    assert_eq!(I::ENTIRE.atan(), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.0, 0.0).atan(), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).atan(), n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, hexf64!("0x1.4c2463567c5acp+25")).atan(), n2i(hexf64!("0x1.921fb54442d18p-1"), hexf64!("0x1.921fb4e19abd7p+0")));
    assert_eq!(n2i(hexf64!("-0x1.fd219490eaac1p+38"), hexf64!("-0x1.1af1c9d74f06dp+9")).atan(), n2i(hexf64!("-0x1.921fb54440cebp+0"), hexf64!("-0x1.91abe5c1e4c6dp+0")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_atan_dec_test() {
    assert_eq!(nd2di(0.0, f64::INFINITY, D::Dac).atan(), nd2di(0.0, hexf64!("0x1.921fb54442d19p+0"), D::Dac));
    assert_eq!(nd2di(f64::NEG_INFINITY, 0.0, D::Def).atan(), nd2di(hexf64!("-0x1.921fb54442d19p+0"), 0.0, D::Def));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan(), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Dac));
    assert_eq!(nd2di(1.0, hexf64!("0x1.4c2463567c5acp+25"), D::Trv).atan(), nd2di(hexf64!("0x1.921fb54442d18p-1"), hexf64!("0x1.921fb4e19abd7p+0"), D::Trv));
    assert_eq!(nd2di(hexf64!("-0x1.fd219490eaac1p+38"), hexf64!("-0x1.1af1c9d74f06dp+9"), D::Com).atan(), nd2di(hexf64!("-0x1.921fb54440cebp+0"), hexf64!("-0x1.91abe5c1e4c6dp+0"), D::Com));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_atan2_test() {
    assert_eq!(I::EMPTY.atan2(I::EMPTY), I::EMPTY);
    assert_eq!(I::EMPTY.atan2(I::ENTIRE), I::EMPTY);
    assert_eq!(I::EMPTY.atan2(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq!(I::EMPTY.atan2(n2i(-0.0, 0.0)), I::EMPTY);
    assert_eq!(I::EMPTY.atan2(n2i(0.0, -0.0)), I::EMPTY);
    assert_eq!(I::EMPTY.atan2(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq!(I::EMPTY.atan2(n2i(-2.0, -0.1)), I::EMPTY);
    assert_eq!(I::EMPTY.atan2(n2i(-2.0, 0.0)), I::EMPTY);
    assert_eq!(I::EMPTY.atan2(n2i(-2.0, -0.0)), I::EMPTY);
    assert_eq!(I::EMPTY.atan2(n2i(-2.0, 1.0)), I::EMPTY);
    assert_eq!(I::EMPTY.atan2(n2i(0.0, 1.0)), I::EMPTY);
    assert_eq!(I::EMPTY.atan2(n2i(-0.0, 1.0)), I::EMPTY);
    assert_eq!(I::EMPTY.atan2(n2i(0.1, 1.0)), I::EMPTY);
    assert_eq!(I::ENTIRE.atan2(I::EMPTY), I::EMPTY);
    assert_eq!(I::ENTIRE.atan2(I::ENTIRE), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(I::ENTIRE.atan2(n2i(0.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(I::ENTIRE.atan2(n2i(0.0, -0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(I::ENTIRE.atan2(n2i(-0.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(I::ENTIRE.atan2(n2i(-0.0, -0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(I::ENTIRE.atan2(n2i(-2.0, -0.1)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(I::ENTIRE.atan2(n2i(-2.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(I::ENTIRE.atan2(n2i(-2.0, -0.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(I::ENTIRE.atan2(n2i(-2.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(I::ENTIRE.atan2(n2i(0.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(I::ENTIRE.atan2(n2i(-0.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(I::ENTIRE.atan2(n2i(0.1, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.0, 0.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).atan2(I::ENTIRE), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, 0.0).atan2(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).atan2(n2i(-0.0, 0.0)), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).atan2(n2i(0.0, -0.0)), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).atan2(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq!(n2i(0.0, 0.0).atan2(n2i(-2.0, -0.1)), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, 0.0).atan2(n2i(-2.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, 0.0).atan2(n2i(-2.0, -0.0)), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, 0.0).atan2(n2i(-2.0, 1.0)), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, 0.0).atan2(n2i(0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0).atan2(n2i(-0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, 0.0).atan2(n2i(0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, 0.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-0.0, 0.0).atan2(I::ENTIRE), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, 0.0).atan2(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq!(n2i(-0.0, 0.0).atan2(n2i(-0.0, 0.0)), I::EMPTY);
    assert_eq!(n2i(-0.0, 0.0).atan2(n2i(0.0, -0.0)), I::EMPTY);
    assert_eq!(n2i(-0.0, 0.0).atan2(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq!(n2i(-0.0, 0.0).atan2(n2i(-2.0, -0.1)), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, 0.0).atan2(n2i(-2.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, 0.0).atan2(n2i(-2.0, -0.0)), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, 0.0).atan2(n2i(-2.0, 1.0)), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, 0.0).atan2(n2i(0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, 0.0).atan2(n2i(-0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, 0.0).atan2(n2i(0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, -0.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.0, -0.0).atan2(I::ENTIRE), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, -0.0).atan2(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq!(n2i(0.0, -0.0).atan2(n2i(-0.0, 0.0)), I::EMPTY);
    assert_eq!(n2i(0.0, -0.0).atan2(n2i(0.0, -0.0)), I::EMPTY);
    assert_eq!(n2i(0.0, -0.0).atan2(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq!(n2i(0.0, -0.0).atan2(n2i(-2.0, -0.1)), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, -0.0).atan2(n2i(-2.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, -0.0).atan2(n2i(-2.0, -0.0)), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, -0.0).atan2(n2i(-2.0, 1.0)), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, -0.0).atan2(n2i(0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, -0.0).atan2(n2i(-0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(0.0, -0.0).atan2(n2i(0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).atan2(I::ENTIRE), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, -0.0).atan2(n2i(0.0, 0.0)), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).atan2(n2i(-0.0, 0.0)), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).atan2(n2i(0.0, -0.0)), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).atan2(n2i(-0.0, -0.0)), I::EMPTY);
    assert_eq!(n2i(-0.0, -0.0).atan2(n2i(-2.0, -0.1)), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, -0.0).atan2(n2i(-2.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, -0.0).atan2(n2i(-2.0, -0.0)), n2i(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, -0.0).atan2(n2i(-2.0, 1.0)), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, -0.0).atan2(n2i(0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).atan2(n2i(-0.0, 1.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).atan2(n2i(0.1, 1.0)), n2i(0.0, 0.0));
    assert_eq!(n2i(-2.0, -0.1).atan2(I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-2.0, -0.1).atan2(I::ENTIRE), n2i(hexf64!("-0x1.921fb54442d19p+1"), 0.0));
    assert_eq!(n2i(-2.0, -0.1).atan2(n2i(0.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(-2.0, -0.1).atan2(n2i(0.0, -0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(-2.0, -0.1).atan2(n2i(-0.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(-2.0, -0.1).atan2(n2i(0.0, -0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(-2.0, -0.1).atan2(n2i(-2.0, -0.1)), n2i(hexf64!("-0x1.8bbaabde5e29cp+1"), hexf64!("-0x1.9ee9c8100c211p+0")));
    assert_eq!(n2i(-2.0, -0.1).atan2(n2i(-2.0, 0.0)), n2i(hexf64!("-0x1.8bbaabde5e29cp+1"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(-2.0, -0.1).atan2(n2i(-2.0, -0.0)), n2i(hexf64!("-0x1.8bbaabde5e29cp+1"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(-2.0, -0.1).atan2(n2i(-2.0, 1.0)), n2i(hexf64!("-0x1.8bbaabde5e29cp+1"), hexf64!("-0x1.983e282e2cc4cp-4")));
    assert_eq!(n2i(-2.0, -0.1).atan2(n2i(0.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.983e282e2cc4cp-4")));
    assert_eq!(n2i(-2.0, -0.1).atan2(n2i(-0.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.983e282e2cc4cp-4")));
    assert_eq!(n2i(-2.0, -0.1).atan2(n2i(0.1, 1.0)), n2i(hexf64!("-0x1.8555a27879820p+0"), hexf64!("-0x1.983e282e2cc4cp-4")));
    assert_eq!(n2i(-2.0, 0.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-2.0, 0.0).atan2(I::ENTIRE), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, 0.0).atan2(n2i(0.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(-2.0, 0.0).atan2(n2i(-0.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(-2.0, 0.0).atan2(n2i(0.0, -0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(-2.0, 0.0).atan2(n2i(-0.0, -0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(-2.0, 0.0).atan2(n2i(-2.0, -0.1)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, 0.0).atan2(n2i(-2.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, 0.0).atan2(n2i(-2.0, -0.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, 0.0).atan2(n2i(-2.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, 0.0).atan2(n2i(0.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), 0.0));
    assert_eq!(n2i(-2.0, 0.0).atan2(n2i(-0.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), 0.0));
    assert_eq!(n2i(-2.0, 0.0).atan2(n2i(0.1, 1.0)), n2i(hexf64!("-0x1.8555a27879820p+0"), 0.0));
    assert_eq!(n2i(-2.0, -0.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-2.0, -0.0).atan2(I::ENTIRE), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, -0.0).atan2(n2i(0.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(-2.0, -0.0).atan2(n2i(-0.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(-2.0, -0.0).atan2(n2i(0.0, -0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(-2.0, -0.0).atan2(n2i(-0.0, -0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0")));
    assert_eq!(n2i(-2.0, -0.0).atan2(n2i(-2.0, -0.1)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, -0.0).atan2(n2i(-2.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, -0.0).atan2(n2i(-2.0, -0.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, -0.0).atan2(n2i(-2.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, -0.0).atan2(n2i(0.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), 0.0));
    assert_eq!(n2i(-2.0, -0.0).atan2(n2i(-0.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), 0.0));
    assert_eq!(n2i(-2.0, -0.0).atan2(n2i(0.1, 1.0)), n2i(hexf64!("-0x1.8555a27879820p+0"), 0.0));
    assert_eq!(n2i(-2.0, 1.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-2.0, 1.0).atan2(I::ENTIRE), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, 1.0).atan2(n2i(0.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-2.0, 1.0).atan2(n2i(-0.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-2.0, 1.0).atan2(n2i(0.0, -0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-2.0, 1.0).atan2(n2i(-0.0, -0.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-2.0, 1.0).atan2(n2i(-2.0, -0.1)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, 1.0).atan2(n2i(-2.0, 0.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, 1.0).atan2(n2i(-2.0, -0.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, 1.0).atan2(n2i(-2.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-2.0, 1.0).atan2(n2i(0.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-2.0, 1.0).atan2(n2i(-0.0, 1.0)), n2i(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-2.0, 1.0).atan2(n2i(0.1, 1.0)), n2i(hexf64!("-0x1.8555a27879820p+0"), hexf64!("0x1.789bd2c160054p+0")));
    assert_eq!(n2i(-0.0, 1.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq!(n2i(-0.0, 1.0).atan2(I::ENTIRE), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, 1.0).atan2(n2i(0.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-0.0, 1.0).atan2(n2i(-0.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-0.0, 1.0).atan2(n2i(0.0, -0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-0.0, 1.0).atan2(n2i(-0.0, -0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-0.0, 1.0).atan2(n2i(-2.0, -0.1)), n2i(hexf64!("0x1.aba397c7259ddp+0"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, 1.0).atan2(n2i(-2.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, 1.0).atan2(n2i(-2.0, -0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, 1.0).atan2(n2i(-2.0, 1.0)), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(-0.0, 1.0).atan2(n2i(0.0, 1.0)), n2i(0.0, hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-0.0, 1.0).atan2(n2i(-0.0, 1.0)), n2i(0.0, hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(-0.0, 1.0).atan2(n2i(0.1, 1.0)), n2i(0.0, hexf64!("0x1.789bd2c160054p+0")));
    assert_eq!(n2i(0.0, 1.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.0, 1.0).atan2(I::ENTIRE), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, 1.0).atan2(n2i(0.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.0, 1.0).atan2(n2i(-0.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.0, 1.0).atan2(n2i(0.0, -0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.0, 1.0).atan2(n2i(-0.0, -0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.0, 1.0).atan2(n2i(-2.0, -0.1)), n2i(hexf64!("0x1.aba397c7259ddp+0"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, 1.0).atan2(n2i(-2.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, 1.0).atan2(n2i(-2.0, -0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, 1.0).atan2(n2i(-2.0, 1.0)), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.0, 1.0).atan2(n2i(0.0, 1.0)), n2i(0.0, hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.0, 1.0).atan2(n2i(-0.0, 1.0)), n2i(0.0, hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.0, 1.0).atan2(n2i(0.1, 1.0)), n2i(0.0, hexf64!("0x1.789bd2c160054p+0")));
    assert_eq!(n2i(0.1, 1.0).atan2(I::EMPTY), I::EMPTY);
    assert_eq!(n2i(0.1, 1.0).atan2(I::ENTIRE), n2i(0.0, hexf64!("0x1.921fb54442d19p+1")));
    assert_eq!(n2i(0.1, 1.0).atan2(n2i(0.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.1, 1.0).atan2(n2i(-0.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.1, 1.0).atan2(n2i(0.0, -0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.1, 1.0).atan2(n2i(-0.0, -0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.1, 1.0).atan2(n2i(-2.0, -0.1)), n2i(hexf64!("0x1.aba397c7259ddp+0"), hexf64!("0x1.8bbaabde5e29cp+1")));
    assert_eq!(n2i(0.1, 1.0).atan2(n2i(-2.0, 0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.8bbaabde5e29cp+1")));
    assert_eq!(n2i(0.1, 1.0).atan2(n2i(-2.0, -0.0)), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.8bbaabde5e29cp+1")));
    assert_eq!(n2i(0.1, 1.0).atan2(n2i(-2.0, 1.0)), n2i(hexf64!("0x1.983e282e2cc4cp-4"), hexf64!("0x1.8bbaabde5e29cp+1")));
    assert_eq!(n2i(0.1, 1.0).atan2(n2i(0.0, 1.0)), n2i(hexf64!("0x1.983e282e2cc4cp-4"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.1, 1.0).atan2(n2i(-0.0, 1.0)), n2i(hexf64!("0x1.983e282e2cc4cp-4"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(0.1, 1.0).atan2(n2i(0.1, 1.0)), n2i(hexf64!("0x1.983e282e2cc4cp-4"), hexf64!("0x1.789bd2c160054p+0")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_atan2_dec_test() {
    assert_eq!(DI::EMPTY.atan2(DI::EMPTY), DI::EMPTY);
    assert_eq!(DI::EMPTY.atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), DI::EMPTY);
    assert_eq!(DI::EMPTY.atan2(nd2di(0.0, 0.0, D::Com)), DI::EMPTY);
    assert_eq!(DI::EMPTY.atan2(nd2di(-0.0, 0.0, D::Dac)), DI::EMPTY);
    assert_eq!(DI::EMPTY.atan2(nd2di(0.0, -0.0, D::Def)), DI::EMPTY);
    assert_eq!(DI::EMPTY.atan2(nd2di(-0.0, -0.0, D::Trv)), DI::EMPTY);
    assert_eq!(DI::EMPTY.atan2(nd2di(-2.0, -0.1, D::Com)), DI::EMPTY);
    assert_eq!(DI::EMPTY.atan2(nd2di(-2.0, 0.0, D::Dac)), DI::EMPTY);
    assert_eq!(DI::EMPTY.atan2(nd2di(-2.0, -0.0, D::Def)), DI::EMPTY);
    assert_eq!(DI::EMPTY.atan2(nd2di(-2.0, 1.0, D::Trv)), DI::EMPTY);
    assert_eq!(DI::EMPTY.atan2(nd2di(0.0, 1.0, D::Com)), DI::EMPTY);
    assert_eq!(DI::EMPTY.atan2(nd2di(-0.0, 1.0, D::Dac)), DI::EMPTY);
    assert_eq!(DI::EMPTY.atan2(nd2di(0.1, 1.0, D::Def)), DI::EMPTY);
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(0.0, 0.0, D::Com)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(0.0, -0.0, D::Def)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(-0.0, 0.0, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(-0.0, -0.0, D::Trv)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(-2.0, -0.1, D::Com)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Def));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(-2.0, 0.0, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(-2.0, -0.0, D::Def)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(-2.0, 1.0, D::Trv)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(0.0, 1.0, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(-0.0, 1.0, D::Def)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).atan2(nd2di(0.1, 1.0, D::Com)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Dac));
    assert_eq!(nd2di(0.0, 0.0, D::Com).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq!(nd2di(0.0, 0.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(0.0, 0.0, D::Def).atan2(nd2di(0.0, 0.0, D::Trv)), DI::EMPTY);
    assert_eq!(nd2di(0.0, 0.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Dac)), DI::EMPTY);
    assert_eq!(nd2di(0.0, 0.0, D::Com).atan2(nd2di(0.0, -0.0, D::Def)), DI::EMPTY);
    assert_eq!(nd2di(0.0, 0.0, D::Dac).atan2(nd2di(-0.0, -0.0, D::Com)), DI::EMPTY);
    assert_eq!(nd2di(0.0, 0.0, D::Com).atan2(nd2di(-2.0, -0.1, D::Dac)), nd2di(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Dac));
    assert_eq!(nd2di(0.0, 0.0, D::Trv).atan2(nd2di(-2.0, 0.0, D::Com)), nd2di(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(0.0, 0.0, D::Com).atan2(nd2di(-2.0, -0.0, D::Trv)), nd2di(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(0.0, 0.0, D::Dac).atan2(nd2di(-2.0, 1.0, D::Def)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(0.0, 0.0, D::Def).atan2(nd2di(0.0, 1.0, D::Com)), nd2di(0.0, 0.0, D::Trv));
    assert_eq!(nd2di(0.0, 0.0, D::Trv).atan2(nd2di(-0.0, 1.0, D::Dac)), nd2di(0.0, 0.0, D::Trv));
    assert_eq!(nd2di(0.0, 0.0, D::Com).atan2(nd2di(0.1, 1.0, D::Com)), nd2di(0.0, 0.0, D::Com));
    assert_eq!(nd2di(-0.0, 0.0, D::Com).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq!(nd2di(-0.0, 0.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-0.0, 0.0, D::Def).atan2(nd2di(0.0, 0.0, D::Com)), DI::EMPTY);
    assert_eq!(nd2di(-0.0, 0.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Trv)), DI::EMPTY);
    assert_eq!(nd2di(-0.0, 0.0, D::Com).atan2(nd2di(0.0, -0.0, D::Def)), DI::EMPTY);
    assert_eq!(nd2di(-0.0, 0.0, D::Dac).atan2(nd2di(-0.0, -0.0, D::Com)), DI::EMPTY);
    assert_eq!(nd2di(-0.0, 0.0, D::Dac).atan2(nd2di(-2.0, -0.1, D::Dac)), nd2di(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Dac));
    assert_eq!(nd2di(-0.0, 0.0, D::Trv).atan2(nd2di(-2.0, 0.0, D::Com)), nd2di(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-0.0, 0.0, D::Com).atan2(nd2di(-2.0, -0.0, D::Trv)), nd2di(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-0.0, 0.0, D::Dac).atan2(nd2di(-2.0, 1.0, D::Def)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-0.0, 0.0, D::Com).atan2(nd2di(0.0, 1.0, D::Com)), nd2di(0.0, 0.0, D::Trv));
    assert_eq!(nd2di(-0.0, 0.0, D::Def).atan2(nd2di(-0.0, 1.0, D::Dac)), nd2di(0.0, 0.0, D::Trv));
    assert_eq!(nd2di(-0.0, 0.0, D::Com).atan2(nd2di(0.1, 1.0, D::Com)), nd2di(0.0, 0.0, D::Com));
    assert_eq!(nd2di(0.0, -0.0, D::Com).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq!(nd2di(0.0, -0.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(0.0, -0.0, D::Def).atan2(nd2di(0.0, 0.0, D::Dac)), DI::EMPTY);
    assert_eq!(nd2di(0.0, -0.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Com)), DI::EMPTY);
    assert_eq!(nd2di(0.0, -0.0, D::Dac).atan2(nd2di(0.0, -0.0, D::Def)), DI::EMPTY);
    assert_eq!(nd2di(0.0, -0.0, D::Com).atan2(nd2di(-0.0, -0.0, D::Trv)), DI::EMPTY);
    assert_eq!(nd2di(0.0, -0.0, D::Com).atan2(nd2di(-2.0, -0.1, D::Dac)), nd2di(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Dac));
    assert_eq!(nd2di(0.0, -0.0, D::Def).atan2(nd2di(-2.0, 0.0, D::Com)), nd2di(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(0.0, -0.0, D::Com).atan2(nd2di(-2.0, -0.0, D::Dac)), nd2di(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(0.0, -0.0, D::Dac).atan2(nd2di(-2.0, 1.0, D::Com)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(0.0, -0.0, D::Com).atan2(nd2di(0.0, 1.0, D::Trv)), nd2di(0.0, 0.0, D::Trv));
    assert_eq!(nd2di(0.0, -0.0, D::Def).atan2(nd2di(-0.0, 1.0, D::Com)), nd2di(0.0, 0.0, D::Trv));
    assert_eq!(nd2di(0.0, -0.0, D::Com).atan2(nd2di(0.1, 1.0, D::Def)), nd2di(0.0, 0.0, D::Def));
    assert_eq!(nd2di(-0.0, -0.0, D::Com).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq!(nd2di(-0.0, -0.0, D::Def).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-0.0, -0.0, D::Dac).atan2(nd2di(0.0, 0.0, D::Com)), DI::EMPTY);
    assert_eq!(nd2di(-0.0, -0.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Def)), DI::EMPTY);
    assert_eq!(nd2di(-0.0, -0.0, D::Com).atan2(nd2di(0.0, -0.0, D::Trv)), DI::EMPTY);
    assert_eq!(nd2di(-0.0, -0.0, D::Dac).atan2(nd2di(-0.0, -0.0, D::Com)), DI::EMPTY);
    assert_eq!(nd2di(-0.0, -0.0, D::Def).atan2(nd2di(-2.0, -0.1, D::Dac)), nd2di(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Def));
    assert_eq!(nd2di(-0.0, -0.0, D::Trv).atan2(nd2di(-2.0, 0.0, D::Def)), nd2di(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-0.0, -0.0, D::Dac).atan2(nd2di(-2.0, -0.0, D::Trv)), nd2di(hexf64!("0x1.921fb54442d18p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-0.0, -0.0, D::Def).atan2(nd2di(-2.0, 1.0, D::Com)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-0.0, -0.0, D::Com).atan2(nd2di(0.0, 1.0, D::Com)), nd2di(0.0, 0.0, D::Trv));
    assert_eq!(nd2di(-0.0, -0.0, D::Trv).atan2(nd2di(-0.0, 1.0, D::Dac)), nd2di(0.0, 0.0, D::Trv));
    assert_eq!(nd2di(-0.0, -0.0, D::Com).atan2(nd2di(0.1, 1.0, D::Com)), nd2di(0.0, 0.0, D::Com));
    assert_eq!(nd2di(-2.0, -0.1, D::Dac).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq!(nd2di(-2.0, -0.1, D::Def).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), 0.0, D::Def));
    assert_eq!(nd2di(-2.0, -0.1, D::Trv).atan2(nd2di(0.0, 0.0, D::Com)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, -0.1, D::Com).atan2(nd2di(0.0, -0.0, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0"), D::Dac));
    assert_eq!(nd2di(-2.0, -0.1, D::Dac).atan2(nd2di(-0.0, 0.0, D::Def)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0"), D::Def));
    assert_eq!(nd2di(-2.0, -0.1, D::Com).atan2(nd2di(0.0, -0.0, D::Trv)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, -0.1, D::Def).atan2(nd2di(-2.0, -0.1, D::Com)), nd2di(hexf64!("-0x1.8bbaabde5e29cp+1"), hexf64!("-0x1.9ee9c8100c211p+0"), D::Def));
    assert_eq!(nd2di(-2.0, -0.1, D::Com).atan2(nd2di(-2.0, 0.0, D::Def)), nd2di(hexf64!("-0x1.8bbaabde5e29cp+1"), hexf64!("-0x1.921fb54442d18p+0"), D::Def));
    assert_eq!(nd2di(-2.0, -0.1, D::Trv).atan2(nd2di(-2.0, -0.0, D::Dac)), nd2di(hexf64!("-0x1.8bbaabde5e29cp+1"), hexf64!("-0x1.921fb54442d18p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, -0.1, D::Def).atan2(nd2di(-2.0, 1.0, D::Trv)), nd2di(hexf64!("-0x1.8bbaabde5e29cp+1"), hexf64!("-0x1.983e282e2cc4cp-4"), D::Trv));
    assert_eq!(nd2di(-2.0, -0.1, D::Com).atan2(nd2di(0.0, 1.0, D::Def)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.983e282e2cc4cp-4"), D::Def));
    assert_eq!(nd2di(-2.0, -0.1, D::Dac).atan2(nd2di(-0.0, 1.0, D::Com)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.983e282e2cc4cp-4"), D::Dac));
    assert_eq!(nd2di(-2.0, -0.1, D::Com).atan2(nd2di(0.1, 1.0, D::Com)), nd2di(hexf64!("-0x1.8555a27879820p+0"), hexf64!("-0x1.983e282e2cc4cp-4"), D::Com));
    assert_eq!(nd2di(-2.0, 0.0, D::Def).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq!(nd2di(-2.0, 0.0, D::Def).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-2.0, 0.0, D::Dac).atan2(nd2di(0.0, 0.0, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, 0.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Com)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, 0.0, D::Com).atan2(nd2di(0.0, -0.0, D::Trv)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, 0.0, D::Def).atan2(nd2di(-0.0, -0.0, D::Def)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, 0.0, D::Com).atan2(nd2di(-2.0, -0.1, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Def));
    assert_eq!(nd2di(-2.0, 0.0, D::Dac).atan2(nd2di(-2.0, 0.0, D::Com)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-2.0, 0.0, D::Com).atan2(nd2di(-2.0, -0.0, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-2.0, 0.0, D::Trv).atan2(nd2di(-2.0, 1.0, D::Com)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-2.0, 0.0, D::Def).atan2(nd2di(0.0, 1.0, D::Def)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), 0.0, D::Trv));
    assert_eq!(nd2di(-2.0, 0.0, D::Com).atan2(nd2di(-0.0, 1.0, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), 0.0, D::Trv));
    assert_eq!(nd2di(-2.0, 0.0, D::Com).atan2(nd2di(0.1, 1.0, D::Com)), nd2di(hexf64!("-0x1.8555a27879820p+0"), 0.0, D::Com));
    assert_eq!(nd2di(-2.0, -0.0, D::Trv).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq!(nd2di(-2.0, -0.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-2.0, -0.0, D::Com).atan2(nd2di(0.0, 0.0, D::Com)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, -0.0, D::Def).atan2(nd2di(-0.0, 0.0, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, -0.0, D::Dac).atan2(nd2di(0.0, -0.0, D::Def)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, -0.0, D::Com).atan2(nd2di(-0.0, -0.0, D::Trv)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("-0x1.921fb54442d18p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, -0.0, D::Def).atan2(nd2di(-2.0, -0.1, D::Com)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Def));
    assert_eq!(nd2di(-2.0, -0.0, D::Com).atan2(nd2di(-2.0, 0.0, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-2.0, -0.0, D::Dac).atan2(nd2di(-2.0, -0.0, D::Def)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-2.0, -0.0, D::Def).atan2(nd2di(-2.0, 1.0, D::Trv)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-2.0, -0.0, D::Trv).atan2(nd2di(0.0, 1.0, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), 0.0, D::Trv));
    assert_eq!(nd2di(-2.0, -0.0, D::Com).atan2(nd2di(-0.0, 1.0, D::Com)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), 0.0, D::Trv));
    assert_eq!(nd2di(-2.0, -0.0, D::Com).atan2(nd2di(0.1, 1.0, D::Com)), nd2di(hexf64!("-0x1.8555a27879820p+0"), 0.0, D::Com));
    assert_eq!(nd2di(-2.0, 1.0, D::Def).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq!(nd2di(-2.0, 1.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-2.0, 1.0, D::Def).atan2(nd2di(0.0, 0.0, D::Def)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, 1.0, D::Com).atan2(nd2di(-0.0, 0.0, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, 1.0, D::Trv).atan2(nd2di(0.0, -0.0, D::Trv)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, 1.0, D::Com).atan2(nd2di(-0.0, -0.0, D::Com)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, 1.0, D::Dac).atan2(nd2di(-2.0, -0.1, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Def));
    assert_eq!(nd2di(-2.0, 1.0, D::Def).atan2(nd2di(-2.0, 0.0, D::Def)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-2.0, 1.0, D::Trv).atan2(nd2di(-2.0, -0.0, D::Trv)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-2.0, 1.0, D::Dac).atan2(nd2di(-2.0, 1.0, D::Com)), nd2di(hexf64!("-0x1.921fb54442d19p+1"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-2.0, 1.0, D::Com).atan2(nd2di(0.0, 1.0, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, 1.0, D::Trv).atan2(nd2di(-0.0, 1.0, D::Dac)), nd2di(hexf64!("-0x1.921fb54442d19p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(-2.0, 1.0, D::Com).atan2(nd2di(0.1, 1.0, D::Com)), nd2di(hexf64!("-0x1.8555a27879820p+0"), hexf64!("0x1.789bd2c160054p+0"), D::Com));
    assert_eq!(nd2di(-0.0, 1.0, D::Com).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq!(nd2di(-0.0, 1.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-0.0, 1.0, D::Def).atan2(nd2di(0.0, 0.0, D::Def)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(-0.0, 1.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Trv)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(-0.0, 1.0, D::Dac).atan2(nd2di(0.0, -0.0, D::Dac)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(-0.0, 1.0, D::Com).atan2(nd2di(-0.0, -0.0, D::Def)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(-0.0, 1.0, D::Com).atan2(nd2di(-2.0, -0.1, D::Com)), nd2di(hexf64!("0x1.aba397c7259ddp+0"), hexf64!("0x1.921fb54442d19p+1"), D::Dac));
    assert_eq!(nd2di(-0.0, 1.0, D::Def).atan2(nd2di(-2.0, 0.0, D::Com)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-0.0, 1.0, D::Def).atan2(nd2di(-2.0, -0.0, D::Def)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-0.0, 1.0, D::Dac).atan2(nd2di(-2.0, 1.0, D::Dac)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(-0.0, 1.0, D::Dac).atan2(nd2di(0.0, 1.0, D::Dac)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(-0.0, 1.0, D::Trv).atan2(nd2di(-0.0, 1.0, D::Com)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(-0.0, 1.0, D::Trv).atan2(nd2di(0.1, 1.0, D::Com)), nd2di(0.0, hexf64!("0x1.789bd2c160054p+0"), D::Trv));
    assert_eq!(nd2di(0.0, 1.0, D::Def).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq!(nd2di(0.0, 1.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(0.0, 1.0, D::Dac).atan2(nd2di(0.0, 0.0, D::Def)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(0.0, 1.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Trv)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(0.0, 1.0, D::Trv).atan2(nd2di(0.0, -0.0, D::Dac)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(0.0, 1.0, D::Def).atan2(nd2di(-0.0, -0.0, D::Com)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(0.0, 1.0, D::Dac).atan2(nd2di(-2.0, -0.1, D::Dac)), nd2di(hexf64!("0x1.aba397c7259ddp+0"), hexf64!("0x1.921fb54442d19p+1"), D::Dac));
    assert_eq!(nd2di(0.0, 1.0, D::Def).atan2(nd2di(-2.0, 0.0, D::Trv)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(0.0, 1.0, D::Dac).atan2(nd2di(-2.0, -0.0, D::Dac)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(0.0, 1.0, D::Dac).atan2(nd2di(-2.0, 1.0, D::Def)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Trv));
    assert_eq!(nd2di(0.0, 1.0, D::Dac).atan2(nd2di(0.0, 1.0, D::Trv)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(0.0, 1.0, D::Dac).atan2(nd2di(-0.0, 1.0, D::Def)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(0.0, 1.0, D::Com).atan2(nd2di(0.1, 1.0, D::Com)), nd2di(0.0, hexf64!("0x1.789bd2c160054p+0"), D::Com));
    assert_eq!(nd2di(0.1, 1.0, D::Dac).atan2(DI::EMPTY), DI::EMPTY);
    assert_eq!(nd2di(0.1, 1.0, D::Dac).atan2(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac)), nd2di(0.0, hexf64!("0x1.921fb54442d19p+1"), D::Dac));
    assert_eq!(nd2di(0.1, 1.0, D::Def).atan2(nd2di(0.0, 0.0, D::Com)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Def));
    assert_eq!(nd2di(0.1, 1.0, D::Trv).atan2(nd2di(-0.0, 0.0, D::Def)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(0.1, 1.0, D::Trv).atan2(nd2di(0.0, -0.0, D::Def)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Trv));
    assert_eq!(nd2di(0.1, 1.0, D::Dac).atan2(nd2di(-0.0, -0.0, D::Def)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0"), D::Def));
    assert_eq!(nd2di(0.1, 1.0, D::Com).atan2(nd2di(-2.0, -0.1, D::Trv)), nd2di(hexf64!("0x1.aba397c7259ddp+0"), hexf64!("0x1.8bbaabde5e29cp+1"), D::Trv));
    assert_eq!(nd2di(0.1, 1.0, D::Com).atan2(nd2di(-2.0, 0.0, D::Dac)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.8bbaabde5e29cp+1"), D::Dac));
    assert_eq!(nd2di(0.1, 1.0, D::Com).atan2(nd2di(-2.0, -0.0, D::Dac)), nd2di(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.8bbaabde5e29cp+1"), D::Dac));
    assert_eq!(nd2di(0.1, 1.0, D::Def).atan2(nd2di(-2.0, 1.0, D::Dac)), nd2di(hexf64!("0x1.983e282e2cc4cp-4"), hexf64!("0x1.8bbaabde5e29cp+1"), D::Def));
    assert_eq!(nd2di(0.1, 1.0, D::Def).atan2(nd2di(0.0, 1.0, D::Def)), nd2di(hexf64!("0x1.983e282e2cc4cp-4"), hexf64!("0x1.921fb54442d19p+0"), D::Def));
    assert_eq!(nd2di(0.1, 1.0, D::Dac).atan2(nd2di(-0.0, 1.0, D::Def)), nd2di(hexf64!("0x1.983e282e2cc4cp-4"), hexf64!("0x1.921fb54442d19p+0"), D::Def));
    assert_eq!(nd2di(0.1, 1.0, D::Dac).atan2(nd2di(0.1, 1.0, D::Def)), nd2di(hexf64!("0x1.983e282e2cc4cp-4"), hexf64!("0x1.789bd2c160054p+0"), D::Def));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_sinh_test() {
    assert_eq!(I::EMPTY.sinh(), I::EMPTY);
    assert_eq!(n2i(0.0, f64::INFINITY).sinh(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).sinh(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).sinh(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).sinh(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(I::ENTIRE.sinh(), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).sinh(), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).sinh(), n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, hexf64!("0x1.2c903022dd7aap+8")).sinh(), n2i(hexf64!("0x1.2cd9fc44eb982p+0"), hexf64!("0x1.89bca168970c6p+432")));
    assert_eq!(n2i(hexf64!("-0x1.fd219490eaac1p+38"), hexf64!("-0x1.1af1c9d74f06dp+9")).sinh(), n2i(f64::NEG_INFINITY, hexf64!("-0x1.53045b4f849dep+815")));
    assert_eq!(n2i(hexf64!("-0x1.199999999999ap+0"), hexf64!("0x1.2666666666666p+1")).sinh(), n2i(hexf64!("-0x1.55ecfe1b2b215p+0"), hexf64!("0x1.3bf72ea61af1bp+2")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_sinh_dec_test() {
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).sinh(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(0.0, f64::INFINITY, D::Dac).sinh(), nd2di(0.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(f64::NEG_INFINITY, 0.0, D::Def).sinh(), nd2di(f64::NEG_INFINITY, 0.0, D::Def));
    assert_eq!(nd2di(1.0, hexf64!("0x1.2c903022dd7aap+8"), D::Com).sinh(), nd2di(hexf64!("0x1.2cd9fc44eb982p+0"), hexf64!("0x1.89bca168970c6p+432"), D::Com));
    assert_eq!(nd2di(hexf64!("-0x1.fd219490eaac1p+38"), hexf64!("-0x1.1af1c9d74f06dp+9"), D::Com).sinh(), nd2di(f64::NEG_INFINITY, hexf64!("-0x1.53045b4f849dep+815"), D::Dac));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_cosh_test() {
    assert_eq!(I::EMPTY.cosh(), I::EMPTY);
    assert_eq!(n2i(0.0, f64::INFINITY).cosh(), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).cosh(), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).cosh(), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).cosh(), n2i(1.0, f64::INFINITY));
    assert_eq!(I::ENTIRE.cosh(), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(0.0, 0.0).cosh(), n2i(1.0, 1.0));
    assert_eq!(n2i(-0.0, -0.0).cosh(), n2i(1.0, 1.0));
    assert_eq!(n2i(1.0, hexf64!("0x1.2c903022dd7aap+8")).cosh(), n2i(hexf64!("0x1.8b07551d9f550p+0"), hexf64!("0x1.89bca168970c6p+432")));
    assert_eq!(n2i(hexf64!("-0x1.fd219490eaac1p+38"), hexf64!("-0x1.1af1c9d74f06dp+9")).cosh(), n2i(hexf64!("0x1.53045b4f849dep+815"), f64::INFINITY));
    assert_eq!(n2i(hexf64!("-0x1.199999999999ap+0"), hexf64!("0x1.2666666666666p+1")).cosh(), n2i(1.0, hexf64!("0x1.4261d2b7d6181p+2")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_cosh_dec_test() {
    assert_eq!(nd2di(0.0, f64::INFINITY, D::Dac).cosh(), nd2di(1.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(f64::NEG_INFINITY, 0.0, D::Def).cosh(), nd2di(1.0, f64::INFINITY, D::Def));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Def).cosh(), nd2di(1.0, f64::INFINITY, D::Def));
    assert_eq!(nd2di(1.0, hexf64!("0x1.2c903022dd7aap+8"), D::Def).cosh(), nd2di(hexf64!("0x1.8b07551d9f550p+0"), hexf64!("0x1.89bca168970c6p+432"), D::Def));
    assert_eq!(nd2di(hexf64!("-0x1.fd219490eaac1p+38"), hexf64!("-0x1.1af1c9d74f06dp+9"), D::Com).cosh(), nd2di(hexf64!("0x1.53045b4f849dep+815"), f64::INFINITY, D::Dac));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_tanh_test() {
    assert_eq!(I::EMPTY.tanh(), I::EMPTY);
    assert_eq!(n2i(0.0, f64::INFINITY).tanh(), n2i(0.0, 1.0));
    assert_eq!(n2i(-0.0, f64::INFINITY).tanh(), n2i(0.0, 1.0));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).tanh(), n2i(-1.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).tanh(), n2i(-1.0, 0.0));
    assert_eq!(I::ENTIRE.tanh(), n2i(-1.0, 1.0));
    assert_eq!(n2i(0.0, 0.0).tanh(), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).tanh(), n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, hexf64!("0x1.2c903022dd7aap+8")).tanh(), n2i(hexf64!("0x1.85efab514f394p-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.fd219490eaac1p+38"), hexf64!("-0x1.1af1c9d74f06dp+9")).tanh(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.fffffffffffffp-1")));
    assert_eq!(n2i(hexf64!("-0x1.199999999999ap+0"), hexf64!("0x1.2666666666666p+1")).tanh(), n2i(hexf64!("-0x1.99db01fde2406p-1"), hexf64!("0x1.f5cf31e1c8103p-1")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_tanh_dec_test() {
    assert_eq!(nd2di(0.0, f64::INFINITY, D::Dac).tanh(), nd2di(0.0, 1.0, D::Dac));
    assert_eq!(nd2di(f64::NEG_INFINITY, 0.0, D::Def).tanh(), nd2di(-1.0, 0.0, D::Def));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).tanh(), nd2di(-1.0, 1.0, D::Dac));
    assert_eq!(nd2di(1.0, hexf64!("0x1.2c903022dd7aap+8"), D::Com).tanh(), nd2di(hexf64!("0x1.85efab514f394p-1"), hexf64!("0x1.0000000000000p+0"), D::Com));
    assert_eq!(nd2di(hexf64!("-0x1.fd219490eaac1p+38"), hexf64!("-0x1.1af1c9d74f06dp+9"), D::Trv).tanh(), nd2di(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.fffffffffffffp-1"), D::Trv));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_asinh_test() {
    assert_eq!(I::EMPTY.asinh(), I::EMPTY);
    assert_eq!(n2i(0.0, f64::INFINITY).asinh(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).asinh(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).asinh(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).asinh(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(I::ENTIRE.asinh(), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).asinh(), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).asinh(), n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, hexf64!("0x1.2c903022dd7aap+8")).asinh(), n2i(hexf64!("0x1.c34366179d426p-1"), hexf64!("0x1.9986127438a87p+2")));
    assert_eq!(n2i(hexf64!("-0x1.fd219490eaac1p+38"), hexf64!("-0x1.1af1c9d74f06dp+9")).asinh(), n2i(hexf64!("-0x1.bb86380a6cc45p+4"), hexf64!("-0x1.c204d8eb20827p+2")));
    assert_eq!(n2i(hexf64!("-0x1.199999999999ap+0"), hexf64!("0x1.2666666666666p+1")).asinh(), n2i(hexf64!("-0x1.e693df6edf1e7p-1"), hexf64!("0x1.91fdc64de0e51p+0")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_asinh_dec_test() {
    assert_eq!(nd2di(0.0, f64::INFINITY, D::Dac).asinh(), nd2di(0.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(f64::NEG_INFINITY, 0.0, D::Trv).asinh(), nd2di(f64::NEG_INFINITY, 0.0, D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).asinh(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(1.0, hexf64!("0x1.2c903022dd7aap+8"), D::Com).asinh(), nd2di(hexf64!("0x1.c34366179d426p-1"), hexf64!("0x1.9986127438a87p+2"), D::Com));
    assert_eq!(nd2di(hexf64!("-0x1.fd219490eaac1p+38"), hexf64!("-0x1.1af1c9d74f06dp+9"), D::Def).asinh(), nd2di(hexf64!("-0x1.bb86380a6cc45p+4"), hexf64!("-0x1.c204d8eb20827p+2"), D::Def));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_acosh_test() {
    assert_eq!(I::EMPTY.acosh(), I::EMPTY);
    assert_eq!(n2i(0.0, f64::INFINITY).acosh(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).acosh(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY).acosh(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 1.0).acosh(), n2i(0.0, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, hexf64!("0x1.fffffffffffffp-1")).acosh(), I::EMPTY);
    assert_eq!(I::ENTIRE.acosh(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 1.0).acosh(), n2i(0.0, 0.0));
    assert_eq!(n2i(1.0, hexf64!("0x1.2c903022dd7aap+8")).acosh(), n2i(0.0, hexf64!("0x1.9985fb3d532afp+2")));
    assert_eq!(n2i(hexf64!("0x1.199999999999ap+0"), hexf64!("0x1.2666666666666p+1")).acosh(), n2i(hexf64!("0x1.c636c1a882f2cp-2"), hexf64!("0x1.799c88e79140dp+0")));
    assert_eq!(n2i(hexf64!("0x1.14d4e82b2b26fp+15"), hexf64!("0x1.72dbe91c837b5p+29")).acosh(), n2i(hexf64!("0x1.656510b4baec3p+3"), hexf64!("0x1.52a415ee8455ap+4")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_acosh_dec_test() {
    assert_eq!(nd2di(0.0, f64::INFINITY, D::Dac).acosh(), nd2di(0.0, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(1.0, f64::INFINITY, D::Dac).acosh(), nd2di(0.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).acosh(), nd2di(0.0, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(1.0, 1.0, D::Com).acosh(), nd2di(0.0, 0.0, D::Com));
    assert_eq!(nd2di(0.9, 1.0, D::Com).acosh(), nd2di(0.0, 0.0, D::Trv));
    assert_eq!(nd2di(1.0, hexf64!("0x1.2c903022dd7aap+8"), D::Dac).acosh(), nd2di(0.0, hexf64!("0x1.9985fb3d532afp+2"), D::Dac));
    assert_eq!(nd2di(0.9, hexf64!("0x1.2c903022dd7aap+8"), D::Com).acosh(), nd2di(0.0, hexf64!("0x1.9985fb3d532afp+2"), D::Trv));
    assert_eq!(nd2di(hexf64!("0x1.14d4e82b2b26fp+15"), hexf64!("0x1.72dbe91c837b5p+29"), D::Def).acosh(), nd2di(hexf64!("0x1.656510b4baec3p+3"), hexf64!("0x1.52a415ee8455ap+4"), D::Def));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_atanh_test() {
    assert_eq!(I::EMPTY.atanh(), I::EMPTY);
    assert_eq!(n2i(0.0, f64::INFINITY).atanh(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(-0.0, f64::INFINITY).atanh(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(1.0, f64::INFINITY).atanh(), I::EMPTY);
    assert_eq!(n2i(f64::NEG_INFINITY, 0.0).atanh(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -0.0).atanh(), n2i(f64::NEG_INFINITY, 0.0));
    assert_eq!(n2i(f64::NEG_INFINITY, -1.0).atanh(), I::EMPTY);
    assert_eq!(n2i(-1.0, 1.0).atanh(), I::ENTIRE);
    assert_eq!(n2i(0.0, 0.0).atanh(), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).atanh(), n2i(0.0, 0.0));
    assert_eq!(n2i(-1.0, -1.0).atanh(), I::EMPTY);
    assert_eq!(n2i(1.0, 1.0).atanh(), I::EMPTY);
    assert_eq!(I::ENTIRE.atanh(), I::ENTIRE);
    assert_eq!(n2i(hexf64!("0x1.4c0420f6f08ccp-2"), hexf64!("0x1.fffffffffffffp-1")).atanh(), n2i(hexf64!("0x1.5871dd2df9102p-2"), hexf64!("0x1.2b708872320e2p+4")));
    assert_eq!(n2i(hexf64!("-0x1.ffb88e9eb6307p-1"), hexf64!("0x1.999999999999ap-4")).atanh(), n2i(hexf64!("-0x1.06a3a97d7979cp+2"), hexf64!("0x1.9af93cd234413p-4")));
}

#[cfg(feature = "gmp")]
#[test]
fn minimal_atanh_dec_test() {
    assert_eq!(nd2di(0.0, f64::INFINITY, D::Dac).atanh(), nd2di(0.0, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, 0.0, D::Def).atanh(), nd2di(f64::NEG_INFINITY, 0.0, D::Trv));
    assert_eq!(nd2di(-1.0, 1.0, D::Com).atanh(), nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(0.0, 0.0, D::Com).atanh(), nd2di(0.0, 0.0, D::Com));
    assert_eq!(nd2di(1.0, 1.0, D::Def).atanh(), DI::EMPTY);
    assert_eq!(nd2di(hexf64!("0x1.4c0420f6f08ccp-2"), hexf64!("0x1.fffffffffffffp-1"), D::Com).atanh(), nd2di(hexf64!("0x1.5871dd2df9102p-2"), hexf64!("0x1.2b708872320e2p+4"), D::Com));
    assert_eq!(nd2di(-1.0, hexf64!("0x1.fffffffffffffp-1"), D::Com).atanh(), nd2di(f64::NEG_INFINITY, hexf64!("0x1.2b708872320e2p+4"), D::Trv));
    assert_eq!(nd2di(hexf64!("-0x1.ffb88e9eb6307p-1"), hexf64!("0x1.999999999999ap-4"), D::Def).atanh(), nd2di(hexf64!("-0x1.06a3a97d7979cp+2"), hexf64!("0x1.9af93cd234413p-4"), D::Def));
    assert_eq!(nd2di(hexf64!("-0x1.ffb88e9eb6307p-1"), 1.0, D::Com).atanh(), nd2di(hexf64!("-0x1.06a3a97d7979cp+2"), f64::INFINITY, D::Trv));
}

#[test]
fn minimal_sign_test() {
    assert_eq!(I::EMPTY.sign(), I::EMPTY);
    assert_eq!(n2i(1.0, 2.0).sign(), n2i(1.0, 1.0));
    assert_eq!(n2i(-1.0, 2.0).sign(), n2i(-1.0, 1.0));
    assert_eq!(n2i(-1.0, 0.0).sign(), n2i(-1.0, 0.0));
    assert_eq!(n2i(0.0, 2.0).sign(), n2i(0.0, 1.0));
    assert_eq!(n2i(-0.0, 2.0).sign(), n2i(0.0, 1.0));
    assert_eq!(n2i(-5.0, -2.0).sign(), n2i(-1.0, -1.0));
    assert_eq!(n2i(0.0, 0.0).sign(), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, -0.0).sign(), n2i(0.0, 0.0));
    assert_eq!(n2i(-0.0, 0.0).sign(), n2i(0.0, 0.0));
    assert_eq!(I::ENTIRE.sign(), n2i(-1.0, 1.0));
}

#[test]
fn minimal_sign_dec_test() {
    assert_eq!(nd2di(1.0, 2.0, D::Com).sign(), nd2di(1.0, 1.0, D::Com));
    assert_eq!(nd2di(-1.0, 2.0, D::Com).sign(), nd2di(-1.0, 1.0, D::Def));
    assert_eq!(nd2di(-1.0, 0.0, D::Com).sign(), nd2di(-1.0, 0.0, D::Def));
    assert_eq!(nd2di(0.0, 2.0, D::Com).sign(), nd2di(0.0, 1.0, D::Def));
    assert_eq!(nd2di(-0.0, 2.0, D::Def).sign(), nd2di(0.0, 1.0, D::Def));
    assert_eq!(nd2di(-5.0, -2.0, D::Trv).sign(), nd2di(-1.0, -1.0, D::Trv));
    assert_eq!(nd2di(0.0, 0.0, D::Dac).sign(), nd2di(0.0, 0.0, D::Dac));
}

#[test]
fn minimal_ceil_test() {
    assert_eq!(I::EMPTY.ceil(), I::EMPTY);
    assert_eq!(I::ENTIRE.ceil(), I::ENTIRE);
    assert_eq!(n2i(1.1, 2.0).ceil(), n2i(2.0, 2.0));
    assert_eq!(n2i(-1.1, 2.0).ceil(), n2i(-1.0, 2.0));
    assert_eq!(n2i(-1.1, 0.0).ceil(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.1, -0.0).ceil(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.1, -0.4).ceil(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.9, 2.2).ceil(), n2i(-1.0, 3.0));
    assert_eq!(n2i(-1.0, 2.2).ceil(), n2i(-1.0, 3.0));
    assert_eq!(n2i(0.0, 2.2).ceil(), n2i(0.0, 3.0));
    assert_eq!(n2i(-0.0, 2.2).ceil(), n2i(0.0, 3.0));
    assert_eq!(n2i(-1.5, f64::INFINITY).ceil(), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY).ceil(), n2i(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.2).ceil(), n2i(f64::NEG_INFINITY, 3.0));
    assert_eq!(n2i(f64::NEG_INFINITY, hexf64!("-0x1.fffffffffffffp+1023")).ceil(), n2i(f64::NEG_INFINITY, hexf64!("-0x1.fffffffffffffp+1023")));
}

#[test]
fn minimal_ceil_dec_test() {
    assert_eq!(nd2di(1.1, 2.0, D::Com).ceil(), nd2di(2.0, 2.0, D::Dac));
    assert_eq!(nd2di(-1.1, 2.0, D::Com).ceil(), nd2di(-1.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.1, 0.0, D::Dac).ceil(), nd2di(-1.0, 0.0, D::Def));
    assert_eq!(nd2di(-1.1, -0.0, D::Trv).ceil(), nd2di(-1.0, 0.0, D::Trv));
    assert_eq!(nd2di(-1.1, -0.4, D::Dac).ceil(), nd2di(-1.0, 0.0, D::Def));
    assert_eq!(nd2di(-1.9, 2.2, D::Com).ceil(), nd2di(-1.0, 3.0, D::Def));
    assert_eq!(nd2di(-1.0, 2.2, D::Dac).ceil(), nd2di(-1.0, 3.0, D::Def));
    assert_eq!(nd2di(0.0, 2.2, D::Trv).ceil(), nd2di(0.0, 3.0, D::Trv));
    assert_eq!(nd2di(-0.0, 2.2, D::Def).ceil(), nd2di(0.0, 3.0, D::Def));
    assert_eq!(nd2di(-1.5, f64::INFINITY, D::Trv).ceil(), nd2di(-1.0, f64::INFINITY, D::Trv));
    assert_eq!(nd2di(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY, D::Dac).ceil(), nd2di(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY, D::Def));
    assert_eq!(nd2di(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Com).ceil(), nd2di(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Dac));
    assert_eq!(nd2di(f64::NEG_INFINITY, 2.2, D::Trv).ceil(), nd2di(f64::NEG_INFINITY, 3.0, D::Trv));
    assert_eq!(nd2di(f64::NEG_INFINITY, hexf64!("-0x1.fffffffffffffp+1023"), D::Dac).ceil(), nd2di(f64::NEG_INFINITY, hexf64!("-0x1.fffffffffffffp+1023"), D::Def));
}

#[test]
fn minimal_floor_test() {
    assert_eq!(I::EMPTY.floor(), I::EMPTY);
    assert_eq!(I::ENTIRE.floor(), I::ENTIRE);
    assert_eq!(n2i(1.1, 2.0).floor(), n2i(1.0, 2.0));
    assert_eq!(n2i(-1.1, 2.0).floor(), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.1, 0.0).floor(), n2i(-2.0, 0.0));
    assert_eq!(n2i(-1.1, -0.0).floor(), n2i(-2.0, 0.0));
    assert_eq!(n2i(-1.1, -0.4).floor(), n2i(-2.0, -1.0));
    assert_eq!(n2i(-1.9, 2.2).floor(), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.0, 2.2).floor(), n2i(-1.0, 2.0));
    assert_eq!(n2i(0.0, 2.2).floor(), n2i(0.0, 2.0));
    assert_eq!(n2i(-0.0, 2.2).floor(), n2i(0.0, 2.0));
    assert_eq!(n2i(-1.5, f64::INFINITY).floor(), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.2).floor(), n2i(f64::NEG_INFINITY, 2.0));
}

#[test]
fn minimal_floor_dec_test() {
    assert_eq!(nd2di(1.1, 2.0, D::Com).floor(), nd2di(1.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.1, 2.0, D::Def).floor(), nd2di(-2.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.1, 0.0, D::Dac).floor(), nd2di(-2.0, 0.0, D::Def));
    assert_eq!(nd2di(-1.2, -1.1, D::Com).floor(), nd2di(-2.0, -2.0, D::Com));
    assert_eq!(nd2di(-1.1, -0.4, D::Def).floor(), nd2di(-2.0, -1.0, D::Def));
    assert_eq!(nd2di(-1.9, 2.2, D::Com).floor(), nd2di(-2.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.0, 2.2, D::Trv).floor(), nd2di(-1.0, 2.0, D::Trv));
    assert_eq!(nd2di(0.0, 2.2, D::Trv).floor(), nd2di(0.0, 2.0, D::Trv));
    assert_eq!(nd2di(-0.0, 2.2, D::Com).floor(), nd2di(0.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.5, f64::INFINITY, D::Dac).floor(), nd2di(-2.0, f64::INFINITY, D::Def));
    assert_eq!(nd2di(f64::NEG_INFINITY, 2.2, D::Trv).floor(), nd2di(f64::NEG_INFINITY, 2.0, D::Trv));
    assert_eq!(nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023"), D::Com).floor(), nd2di(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023"), D::Dac));
}

#[test]
fn minimal_trunc_test() {
    assert_eq!(I::EMPTY.trunc(), I::EMPTY);
    assert_eq!(I::ENTIRE.trunc(), I::ENTIRE);
    assert_eq!(n2i(1.1, 2.1).trunc(), n2i(1.0, 2.0));
    assert_eq!(n2i(-1.1, 2.0).trunc(), n2i(-1.0, 2.0));
    assert_eq!(n2i(-1.1, 0.0).trunc(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.1, -0.0).trunc(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.1, -0.4).trunc(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.9, 2.2).trunc(), n2i(-1.0, 2.0));
    assert_eq!(n2i(-1.0, 2.2).trunc(), n2i(-1.0, 2.0));
    assert_eq!(n2i(0.0, 2.2).trunc(), n2i(0.0, 2.0));
    assert_eq!(n2i(-0.0, 2.2).trunc(), n2i(0.0, 2.0));
    assert_eq!(n2i(-1.5, f64::INFINITY).trunc(), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.2).trunc(), n2i(f64::NEG_INFINITY, 2.0));
}

#[test]
fn minimal_trunc_dec_test() {
    assert_eq!(nd2di(1.1, 2.1, D::Com).trunc(), nd2di(1.0, 2.0, D::Def));
    assert_eq!(nd2di(1.1, 1.9, D::Com).trunc(), nd2di(1.0, 1.0, D::Com));
    assert_eq!(nd2di(-1.1, 2.0, D::Dac).trunc(), nd2di(-1.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.1, 0.0, D::Trv).trunc(), nd2di(-1.0, 0.0, D::Trv));
    assert_eq!(nd2di(-1.1, -0.0, D::Def).trunc(), nd2di(-1.0, 0.0, D::Def));
    assert_eq!(nd2di(-1.1, -0.4, D::Com).trunc(), nd2di(-1.0, 0.0, D::Def));
    assert_eq!(nd2di(-1.9, 2.2, D::Def).trunc(), nd2di(-1.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.0, 2.2, D::Dac).trunc(), nd2di(-1.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.5, f64::INFINITY, D::Dac).trunc(), nd2di(-1.0, f64::INFINITY, D::Def));
    assert_eq!(nd2di(f64::NEG_INFINITY, 2.2, D::Trv).trunc(), nd2di(f64::NEG_INFINITY, 2.0, D::Trv));
    assert_eq!(nd2di(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Com).trunc(), nd2di(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023"), D::Dac));
    assert_eq!(nd2di(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY, D::Dac).trunc(), nd2di(hexf64!("0x1.fffffffffffffp+1023"), f64::INFINITY, D::Def));
}

#[test]
fn minimal_round_ties_to_even_test() {
    assert_eq!(I::EMPTY.round_ties_to_even(), I::EMPTY);
    assert_eq!(I::ENTIRE.round_ties_to_even(), I::ENTIRE);
    assert_eq!(n2i(1.1, 2.1).round_ties_to_even(), n2i(1.0, 2.0));
    assert_eq!(n2i(-1.1, 2.0).round_ties_to_even(), n2i(-1.0, 2.0));
    assert_eq!(n2i(-1.1, -0.4).round_ties_to_even(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.1, 0.0).round_ties_to_even(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.1, -0.0).round_ties_to_even(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.9, 2.2).round_ties_to_even(), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.0, 2.2).round_ties_to_even(), n2i(-1.0, 2.0));
    assert_eq!(n2i(1.5, 2.1).round_ties_to_even(), n2i(2.0, 2.0));
    assert_eq!(n2i(-1.5, 2.0).round_ties_to_even(), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.1, -0.5).round_ties_to_even(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.9, 2.5).round_ties_to_even(), n2i(-2.0, 2.0));
    assert_eq!(n2i(0.0, 2.5).round_ties_to_even(), n2i(0.0, 2.0));
    assert_eq!(n2i(-0.0, 2.5).round_ties_to_even(), n2i(0.0, 2.0));
    assert_eq!(n2i(-1.5, 2.5).round_ties_to_even(), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.5, f64::INFINITY).round_ties_to_even(), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.2).round_ties_to_even(), n2i(f64::NEG_INFINITY, 2.0));
}

#[test]
fn minimal_round_ties_to_even_dec_test() {
    assert_eq!(nd2di(1.1, 2.1, D::Com).round_ties_to_even(), nd2di(1.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.1, 2.0, D::Trv).round_ties_to_even(), nd2di(-1.0, 2.0, D::Trv));
    assert_eq!(nd2di(-1.6, -1.5, D::Com).round_ties_to_even(), nd2di(-2.0, -2.0, D::Dac));
    assert_eq!(nd2di(-1.6, -1.4, D::Com).round_ties_to_even(), nd2di(-2.0, -1.0, D::Def));
    assert_eq!(nd2di(-1.5, f64::INFINITY, D::Dac).round_ties_to_even(), nd2di(-2.0, f64::INFINITY, D::Def));
    assert_eq!(nd2di(f64::NEG_INFINITY, 2.2, D::Trv).round_ties_to_even(), nd2di(f64::NEG_INFINITY, 2.0, D::Trv));
}

#[test]
fn minimal_round_ties_to_away_test() {
    assert_eq!(I::EMPTY.round_ties_to_away(), I::EMPTY);
    assert_eq!(I::ENTIRE.round_ties_to_away(), I::ENTIRE);
    assert_eq!(n2i(1.1, 2.1).round_ties_to_away(), n2i(1.0, 2.0));
    assert_eq!(n2i(-1.1, 2.0).round_ties_to_away(), n2i(-1.0, 2.0));
    assert_eq!(n2i(-1.1, 0.0).round_ties_to_away(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.1, -0.0).round_ties_to_away(), n2i(-1.0, -0.0));
    assert_eq!(n2i(-1.1, -0.4).round_ties_to_away(), n2i(-1.0, 0.0));
    assert_eq!(n2i(-1.9, 2.2).round_ties_to_away(), n2i(-2.0, 2.0));
    assert_eq!(n2i(-1.0, 2.2).round_ties_to_away(), n2i(-1.0, 2.0));
    assert_eq!(n2i(0.5, 2.1).round_ties_to_away(), n2i(1.0, 2.0));
    assert_eq!(n2i(-2.5, 2.0).round_ties_to_away(), n2i(-3.0, 2.0));
    assert_eq!(n2i(-1.1, -0.5).round_ties_to_away(), n2i(-1.0, -1.0));
    assert_eq!(n2i(-1.9, 2.5).round_ties_to_away(), n2i(-2.0, 3.0));
    assert_eq!(n2i(-1.5, 2.5).round_ties_to_away(), n2i(-2.0, 3.0));
    assert_eq!(n2i(0.0, 2.5).round_ties_to_away(), n2i(0.0, 3.0));
    assert_eq!(n2i(-0.0, 2.5).round_ties_to_away(), n2i(0.0, 3.0));
    assert_eq!(n2i(-1.5, f64::INFINITY).round_ties_to_away(), n2i(-2.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, 2.2).round_ties_to_away(), n2i(f64::NEG_INFINITY, 2.0));
}

#[test]
fn minimal_round_ties_to_away_dec_test() {
    assert_eq!(nd2di(1.1, 2.1, D::Com).round_ties_to_away(), nd2di(1.0, 2.0, D::Def));
    assert_eq!(nd2di(-1.9, 2.2, D::Com).round_ties_to_away(), nd2di(-2.0, 2.0, D::Def));
    assert_eq!(nd2di(1.9, 2.2, D::Com).round_ties_to_away(), nd2di(2.0, 2.0, D::Com));
    assert_eq!(nd2di(-1.0, 2.2, D::Trv).round_ties_to_away(), nd2di(-1.0, 2.0, D::Trv));
    assert_eq!(nd2di(2.5, 2.6, D::Com).round_ties_to_away(), nd2di(3.0, 3.0, D::Dac));
    assert_eq!(nd2di(-1.5, f64::INFINITY, D::Dac).round_ties_to_away(), nd2di(-2.0, f64::INFINITY, D::Def));
    assert_eq!(nd2di(f64::NEG_INFINITY, 2.2, D::Def).round_ties_to_away(), nd2di(f64::NEG_INFINITY, 2.0, D::Def));
}

#[test]
fn minimal_abs_test() {
    assert_eq!(I::EMPTY.abs(), I::EMPTY);
    assert_eq!(I::ENTIRE.abs(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(1.1, 2.1).abs(), n2i(1.1, 2.1));
    assert_eq!(n2i(-1.1, 2.0).abs(), n2i(0.0, 2.0));
    assert_eq!(n2i(-1.1, 0.0).abs(), n2i(0.0, 1.1));
    assert_eq!(n2i(-1.1, -0.0).abs(), n2i(0.0, 1.1));
    assert_eq!(n2i(-1.1, -0.4).abs(), n2i(0.4, 1.1));
    assert_eq!(n2i(-1.9, 0.2).abs(), n2i(0.0, 1.9));
    assert_eq!(n2i(0.0, 0.2).abs(), n2i(0.0, 0.2));
    assert_eq!(n2i(-0.0, 0.2).abs(), n2i(0.0, 0.2));
    assert_eq!(n2i(-1.5, f64::INFINITY).abs(), n2i(0.0, f64::INFINITY));
    assert_eq!(n2i(f64::NEG_INFINITY, -2.2).abs(), n2i(2.2, f64::INFINITY));
}

#[test]
fn minimal_abs_dec_test() {
    assert_eq!(nd2di(-1.1, 2.0, D::Com).abs(), nd2di(0.0, 2.0, D::Com));
    assert_eq!(nd2di(-1.1, 0.0, D::Dac).abs(), nd2di(0.0, 1.1, D::Dac));
    assert_eq!(nd2di(-1.1, -0.0, D::Def).abs(), nd2di(0.0, 1.1, D::Def));
    assert_eq!(nd2di(-1.1, -0.4, D::Trv).abs(), nd2di(0.4, 1.1, D::Trv));
    assert_eq!(nd2di(-1.9, 0.2, D::Dac).abs(), nd2di(0.0, 1.9, D::Dac));
    assert_eq!(nd2di(0.0, 0.2, D::Def).abs(), nd2di(0.0, 0.2, D::Def));
    assert_eq!(nd2di(-0.0, 0.2, D::Com).abs(), nd2di(0.0, 0.2, D::Com));
    assert_eq!(nd2di(-1.5, f64::INFINITY, D::Dac).abs(), nd2di(0.0, f64::INFINITY, D::Dac));
}

#[test]
fn minimal_min_test() {
    assert_eq!(I::EMPTY.min(n2i(1.0, 2.0)), I::EMPTY);
    assert_eq!(n2i(1.0, 2.0).min(I::EMPTY), I::EMPTY);
    assert_eq!(I::EMPTY.min(I::EMPTY), I::EMPTY);
    assert_eq!(I::ENTIRE.min(n2i(1.0, 2.0)), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(n2i(1.0, 2.0).min(I::ENTIRE), n2i(f64::NEG_INFINITY, 2.0));
    assert_eq!(I::ENTIRE.min(I::ENTIRE), I::ENTIRE);
    assert_eq!(I::EMPTY.min(I::ENTIRE), I::EMPTY);
    assert_eq!(n2i(1.0, 5.0).min(n2i(2.0, 4.0)), n2i(1.0, 4.0));
    assert_eq!(n2i(0.0, 5.0).min(n2i(2.0, 4.0)), n2i(0.0, 4.0));
    assert_eq!(n2i(-0.0, 5.0).min(n2i(2.0, 4.0)), n2i(0.0, 4.0));
    assert_eq!(n2i(1.0, 5.0).min(n2i(2.0, 8.0)), n2i(1.0, 5.0));
    assert_eq!(n2i(1.0, 5.0).min(I::ENTIRE), n2i(f64::NEG_INFINITY, 5.0));
    assert_eq!(n2i(-7.0, -5.0).min(n2i(2.0, 4.0)), n2i(-7.0, -5.0));
    assert_eq!(n2i(-7.0, 0.0).min(n2i(2.0, 4.0)), n2i(-7.0, 0.0));
    assert_eq!(n2i(-7.0, -0.0).min(n2i(2.0, 4.0)), n2i(-7.0, 0.0));
}

#[test]
fn minimal_min_dec_test() {
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).min(nd2di(1.0, 2.0, D::Com)), nd2di(f64::NEG_INFINITY, 2.0, D::Dac));
    assert_eq!(nd2di(-7.0, -5.0, D::Trv).min(nd2di(2.0, 4.0, D::Def)), nd2di(-7.0, -5.0, D::Trv));
    assert_eq!(nd2di(-7.0, 0.0, D::Dac).min(nd2di(2.0, 4.0, D::Def)), nd2di(-7.0, 0.0, D::Def));
    assert_eq!(nd2di(-7.0, -0.0, D::Com).min(nd2di(2.0, 4.0, D::Com)), nd2di(-7.0, 0.0, D::Com));
}

#[test]
fn minimal_max_test() {
    assert_eq!(I::EMPTY.max(n2i(1.0, 2.0)), I::EMPTY);
    assert_eq!(n2i(1.0, 2.0).max(I::EMPTY), I::EMPTY);
    assert_eq!(I::EMPTY.max(I::EMPTY), I::EMPTY);
    assert_eq!(I::ENTIRE.max(n2i(1.0, 2.0)), n2i(1.0, f64::INFINITY));
    assert_eq!(n2i(1.0, 2.0).max(I::ENTIRE), n2i(1.0, f64::INFINITY));
    assert_eq!(I::ENTIRE.max(I::ENTIRE), I::ENTIRE);
    assert_eq!(I::EMPTY.max(I::ENTIRE), I::EMPTY);
    assert_eq!(n2i(1.0, 5.0).max(n2i(2.0, 4.0)), n2i(2.0, 5.0));
    assert_eq!(n2i(1.0, 5.0).max(n2i(2.0, 8.0)), n2i(2.0, 8.0));
    assert_eq!(n2i(-1.0, 5.0).max(I::ENTIRE), n2i(-1.0, f64::INFINITY));
    assert_eq!(n2i(-7.0, -5.0).max(n2i(2.0, 4.0)), n2i(2.0, 4.0));
    assert_eq!(n2i(-7.0, -5.0).max(n2i(0.0, 4.0)), n2i(0.0, 4.0));
    assert_eq!(n2i(-7.0, -5.0).max(n2i(-0.0, 4.0)), n2i(0.0, 4.0));
    assert_eq!(n2i(-7.0, -5.0).max(n2i(-2.0, 0.0)), n2i(-2.0, 0.0));
    assert_eq!(n2i(-7.0, -5.0).max(n2i(-2.0, -0.0)), n2i(-2.0, 0.0));
}

#[test]
fn minimal_max_dec_test() {
    assert_eq!(nd2di(f64::NEG_INFINITY, f64::INFINITY, D::Dac).max(nd2di(1.0, 2.0, D::Com)), nd2di(1.0, f64::INFINITY, D::Dac));
    assert_eq!(nd2di(-7.0, -5.0, D::Trv).max(nd2di(2.0, 4.0, D::Def)), nd2di(2.0, 4.0, D::Trv));
    assert_eq!(nd2di(-7.0, 5.0, D::Dac).max(nd2di(2.0, 4.0, D::Def)), nd2di(2.0, 5.0, D::Def));
    assert_eq!(nd2di(3.0, 3.5, D::Com).max(nd2di(2.0, 4.0, D::Com)), nd2di(3.0, 4.0, D::Com));
}
