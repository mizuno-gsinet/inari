/*
 *
 * Unit tests from FI_LIB version 1.2
 * (Original authors: Werner Hofschuster and Walter Kraemer)
 * converted into portable ITL format by Oliver Heimlich.
 *
 * Copyright 1997-2000 Institut fuer Wissenschaftliches Rechnen
 *                     und Mathematische Modellbildung (IWRMM)
 *                                      and
 *                     Institut fuer Angewandte Mathematik
 *                     Universitaet Karlsruhe, Germany
 * Copyright 2000-2005 Wiss. Rechnen/Softwaretechnologie
 *                     Universitaet Wuppertal, Germany
 * Copyright 2015-2016 Oliver Heimlich (oheim@posteo.de)
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
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

#[test]
fn fi_lib_addii() {
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")) + n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) + n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) + n2i(hexf64!("-0x1.0000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) + n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) + n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) + n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")), n2i(hexf64!("0x0.0000000000fd0p-1022"), hexf64!("0x0.0000000000fd0p-1022")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) + n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) + n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")), n2i(hexf64!("-0x0.0000000000fd0p-1022"), hexf64!("-0x0.0000000000fd0p-1022")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) + n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")) + n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x1.ffffffffffffep+1023"), hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")) + n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.ffffffffffffep+1023")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.0000000000000p+0")) + n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")) + n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")) + n2i(hexf64!("0x1.8000000000000p+1"), hexf64!("0x1.0000000000000p+2")), n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.8000000000000p+2")));
    assert_eq!(n2i(hexf64!("0x1.8000000000000p+1"), hexf64!("0x1.0000000000000p+2")) + n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.8000000000000p+2")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x0.0p+0")) + n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("-0x1.8000000000000p+1")), n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("-0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("-0x1.8000000000000p+1")) + n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("-0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("-0x1.0000000000000p+2")) + n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.4000000000000p+2")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.4000000000000p+2")) + n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("-0x1.0000000000000p+2")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
}

#[test]
fn fi_lib_subii() {
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")) - n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) - n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) - n2i(hexf64!("-0x1.0000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) - n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.0000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) - n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000000p+1"), hexf64!("-0x1.0000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) - n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) - n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.fffffffffffffp-1")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) - n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) - n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000001p+0"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")) - n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x1.ffffffffffffep+1023"), hexf64!("0x1.fffffffffffffp+1023")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")) - n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.ffffffffffffep+1023")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.0000000000000p+0")) - n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("-0x1.0000000000000p+1"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")) - n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.0000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")) - n2i(hexf64!("0x1.8000000000000p+1"), hexf64!("0x1.0000000000000p+2")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.8000000000000p+1"), hexf64!("0x1.0000000000000p+2")) - n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x0.0p+0")) - n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("-0x1.8000000000000p+1")), n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.0000000000000p+2")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("-0x1.8000000000000p+1")) - n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("-0x1.0000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("-0x1.0000000000000p+2")) - n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.4000000000000p+2")), n2i(hexf64!("-0x1.4000000000000p+3"), hexf64!("-0x1.0000000000000p+3")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.4000000000000p+2")) - n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("-0x1.0000000000000p+2")), n2i(hexf64!("0x1.0000000000000p+3"), hexf64!("0x1.4000000000000p+3")));
}

#[test]
fn fi_lib_mulii() {
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")) * n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) * n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) * n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")) * n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")) * n2i(hexf64!("0x1.0000000000000p-1"), hexf64!("0x1.0000000000000p-1")), n2i(hexf64!("0x1.fffffffffffffp+1022"), hexf64!("0x1.fffffffffffffp+1022")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")) * n2i(hexf64!("0x1.0000000000000p-1"), hexf64!("0x1.0000000000000p-1")), n2i(hexf64!("-0x1.fffffffffffffp+1022"), hexf64!("-0x1.fffffffffffffp+1022")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) * n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) * n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.2000000000000p+3")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")) * n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.2000000000000p+3"), hexf64!("-0x1.0000000000000p+2")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("-0x1.2000000000000p+3"), hexf64!("-0x1.0000000000000p+2")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("0x1.0000000000000p+2"), hexf64!("0x1.2000000000000p+3")));
    assert_eq!(n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("0x1.0000000000000p+1")) * n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.e000000000000p+3"), hexf64!("0x1.4000000000000p+4")));
    assert_eq!(n2i(hexf64!("-0x1.4000000000000p+2"), hexf64!("0x1.0000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.0000000000000p+3")), n2i(hexf64!("-0x1.4000000000000p+5"), hexf64!("0x1.0000000000000p+4")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+1"), hexf64!("0x1.4000000000000p+2")) * n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.4000000000000p+4"), hexf64!("0x1.e000000000000p+3")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("0x1.4000000000000p+2")) * n2i(hexf64!("-0x1.0000000000000p+2"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.4000000000000p+4"), hexf64!("0x1.0000000000000p+4")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.2000000000000p+3")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")) * n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.2000000000000p+3"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.2000000000000p+3")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")) * n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.2000000000000p+3"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) * n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("-0x1.2000000000000p+3"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x1.8000000000000p+1")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.2000000000000p+3")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")), n2i(hexf64!("-0x1.2000000000000p+3"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")), n2i(hexf64!("-0x1.2000000000000p+3"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.0000000000000p+1")) * n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")), n2i(hexf64!("-0x1.8000000000000p+2"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) * n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) * n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) * n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) * n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")), n2i(hexf64!("-0x0.0000000000001p-1022"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")) * n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) * n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
}

#[test]
fn fi_lib_divii() {
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) / n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")) / n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")) / n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")) / n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) / n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) / n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.fffffffffffffp+1023"), hexf64!("0x1.fffffffffffffp+1023")) / n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("0x1.fffffffffffffp+1022"), hexf64!("0x1.fffffffffffffp+1022")));
    assert_eq!(n2i(hexf64!("-0x1.fffffffffffffp+1023"), hexf64!("-0x1.fffffffffffffp+1023")) / n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.0000000000000p+1")), n2i(hexf64!("-0x1.fffffffffffffp+1022"), hexf64!("-0x1.fffffffffffffp+1022")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) / n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) / n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")), n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")));
    assert_eq!(n2i(hexf64!("0x0.00000000007e8p-1022"), hexf64!("0x0.00000000007e8p-1022")) / n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("-0x1.0000000000000p+0")), n2i(hexf64!("-0x0.00000000007e8p-1022"), hexf64!("-0x0.00000000007e8p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")) / n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("0x1.5555555555555p-1"), hexf64!("0x1.8000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")) / n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+0"), hexf64!("-0x1.5555555555555p-1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) / n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("-0x1.0000000000000p-1"), hexf64!("0x1.0000000000000p-1")));
    assert_eq!(n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")) / n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.0000000000000p-1"), hexf64!("0x1.0000000000000p-1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")) / n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+0"), hexf64!("-0x1.5555555555555p-1")));
    assert_eq!(n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")) / n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("0x1.5555555555555p-1"), hexf64!("0x1.8000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")) / n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("0x0.0p+0")) / n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+1")) / n2i(hexf64!("-0x1.8000000000000p+1"), hexf64!("-0x1.0000000000000p+1")), n2i(hexf64!("-0x1.8000000000000p+0"), hexf64!("0x0.0p+0")));
    assert_eq!(n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+1")) / n2i(hexf64!("0x1.0000000000000p+1"), hexf64!("0x1.8000000000000p+1")), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.8000000000000p+0")));
}

#[cfg(feature = "gmp")]
#[test]
fn fi_lib_unary_functions_gmp() {
    assert_eq!(n2i(hexf64!("-0x1.4c8993b11d519p-149"), hexf64!("-0x1.b1d8f24f24de3p-941")).sin(), n2i(hexf64!("-0x1.4c8993b11d519p-149"), hexf64!("-0x1.b1d8f24f24de2p-941")));
    assert_eq!(n2i(hexf64!("-0x1.9ee1a9db994f5p-436"), hexf64!("-0x1.b6451c0720bfbp-622")).sin(), n2i(hexf64!("-0x1.9ee1a9db994f5p-436"), hexf64!("-0x1.b6451c0720bfap-622")));
    assert_eq!(n2i(hexf64!("-0x1.59415fcfbff18p+6"), hexf64!("-0x1.1b0be7ac0af65p-959")).sin(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.975299ccb0e08p-372"), hexf64!("0x1.77d8fa6b68b60p-585")).sin(), n2i(hexf64!("-0x1.975299ccb0e08p-372"), hexf64!("0x1.77d8fa6b68b60p-585")));
    assert_eq!(n2i(hexf64!("-0x1.a8e9c46a3d769p-355"), hexf64!("-0x1.c3a9cd7025105p-564")).sin(), n2i(hexf64!("-0x1.a8e9c46a3d769p-355"), hexf64!("-0x1.c3a9cd7025104p-564")));
    assert_eq!(n2i(hexf64!("-0x1.9b66c64d19ee1p-329"), hexf64!("-0x1.0b8dbebdff270p-411")).sin(), n2i(hexf64!("-0x1.9b66c64d19ee1p-329"), hexf64!("-0x1.0b8dbebdff26fp-411")));
    assert_eq!(n2i(hexf64!("-0x1.32690aac2472dp-40"), hexf64!("-0x1.0e0dd7b9e7391p-789")).sin(), n2i(hexf64!("-0x1.32690aac2472dp-40"), hexf64!("-0x1.0e0dd7b9e7390p-789")));
    assert_eq!(n2i(hexf64!("-0x1.40516bc314cc9p-198"), hexf64!("-0x1.7ad0659980c2bp-203")).sin(), n2i(hexf64!("-0x1.40516bc314cc9p-198"), hexf64!("-0x1.7ad0659980c2ap-203")));
    assert_eq!(n2i(hexf64!("0x1.e80ef8fd19ad4p-265"), hexf64!("0x1.ba9a1304c562dp-79")).sin(), n2i(hexf64!("0x1.e80ef8fd19ad3p-265"), hexf64!("0x1.ba9a1304c562dp-79")));
    assert_eq!(n2i(hexf64!("-0x1.764bf1b176ff7p-373"), hexf64!("-0x1.00bd1997cd82ep-915")).sin(), n2i(hexf64!("-0x1.764bf1b176ff7p-373"), hexf64!("-0x1.00bd1997cd82dp-915")));
    assert_eq!(n2i(hexf64!("-0x1.b28f9484ffbfcp-589"), hexf64!("0x1.c77c9276b791dp-13")).sin(), n2i(hexf64!("-0x1.b28f9484ffbfcp-589"), hexf64!("0x1.c77c923aa2e2ep-13")));
    assert_eq!(n2i(hexf64!("-0x1.388c331648e3ep-333"), hexf64!("-0x1.65ed85df2d4b7p-576")).sin(), n2i(hexf64!("-0x1.388c331648e3ep-333"), hexf64!("-0x1.65ed85df2d4b6p-576")));
    assert_eq!(n2i(hexf64!("-0x1.96433e013eda2p-935"), hexf64!("0x1.8620faa09eadbp-386")).sin(), n2i(hexf64!("-0x1.96433e013eda2p-935"), hexf64!("0x1.8620faa09eadbp-386")));
    assert_eq!(n2i(hexf64!("-0x1.ab077c8e23ef5p-491"), hexf64!("0x1.504d993745eafp-18")).sin(), n2i(hexf64!("-0x1.ab077c8e23ef5p-491"), hexf64!("0x1.504d99373fdf5p-18")));
    assert_eq!(n2i(hexf64!("-0x1.06d7f9ae94dadp-301"), hexf64!("-0x1.9610758986a88p-835")).sin(), n2i(hexf64!("-0x1.06d7f9ae94dadp-301"), hexf64!("-0x1.9610758986a87p-835")));
    assert_eq!(n2i(hexf64!("-0x1.af3b21180e563p-498"), hexf64!("-0x1.068b13da99666p-760")).sin(), n2i(hexf64!("-0x1.af3b21180e563p-498"), hexf64!("-0x1.068b13da99665p-760")));
    assert_eq!(n2i(hexf64!("-0x1.2789c2d583bcdp-568"), hexf64!("-0x1.f2bd89dad0665p-780")).sin(), n2i(hexf64!("-0x1.2789c2d583bcdp-568"), hexf64!("-0x1.f2bd89dad0664p-780")));
    assert_eq!(n2i(hexf64!("-0x1.9fc9d1b0afc7cp-545"), hexf64!("0x1.580844b9dc45cp-780")).sin(), n2i(hexf64!("-0x1.9fc9d1b0afc7cp-545"), hexf64!("0x1.580844b9dc45cp-780")));
    assert_eq!(n2i(hexf64!("-0x1.8a11a53596037p-49"), hexf64!("0x1.b1e6b793078ddp-664")).sin(), n2i(hexf64!("-0x1.8a11a53596037p-49"), hexf64!("0x1.b1e6b793078ddp-664")));
    assert_eq!(n2i(hexf64!("-0x1.425eef071014fp-121"), hexf64!("-0x1.bb2efb4f70837p-547")).sin(), n2i(hexf64!("-0x1.425eef071014fp-121"), hexf64!("-0x1.bb2efb4f70836p-547")));
    assert_eq!(n2i(hexf64!("-0x1.8e96354bf7e11p-894"), hexf64!("-0x1.039e2518cf503p-1008")).sin(), n2i(hexf64!("-0x1.8e96354bf7e11p-894"), hexf64!("-0x1.039e2518cf502p-1008")));
    assert_eq!(n2i(hexf64!("-0x1.7ba62e3fbdd83p-165"), hexf64!("0x1.069e434ee9e0fp-740")).sin(), n2i(hexf64!("-0x1.7ba62e3fbdd83p-165"), hexf64!("0x1.069e434ee9e0fp-740")));
    assert_eq!(n2i(hexf64!("-0x1.e7802992ba99dp-775"), hexf64!("-0x1.7883a587654e5p-928")).sin(), n2i(hexf64!("-0x1.e7802992ba99dp-775"), hexf64!("-0x1.7883a587654e4p-928")));
    assert_eq!(n2i(hexf64!("0x1.455801d3d2b63p-704"), hexf64!("0x1.d2648abc1e83dp-27")).sin(), n2i(hexf64!("0x1.455801d3d2b62p-704"), hexf64!("0x1.d2648abc1e83dp-27")));
    assert_eq!(n2i(hexf64!("-0x1.04be837a6f1f1p-375"), hexf64!("0x1.173ab0fec92afp-771")).sin(), n2i(hexf64!("-0x1.04be837a6f1f1p-375"), hexf64!("0x1.173ab0fec92afp-771")));
    assert_eq!(n2i(hexf64!("-0x1.7e13dbb66e5a3p-84"), hexf64!("-0x1.af23d175aa3d2p-538")).sin(), n2i(hexf64!("-0x1.7e13dbb66e5a3p-84"), hexf64!("-0x1.af23d175aa3d1p-538")));
    assert_eq!(n2i(hexf64!("-0x1.07d4317cb3695p-274"), hexf64!("-0x1.ef8b7bcbab211p-495")).sin(), n2i(hexf64!("-0x1.07d4317cb3695p-274"), hexf64!("-0x1.ef8b7bcbab210p-495")));
    assert_eq!(n2i(hexf64!("-0x1.fb31317bb132bp-326"), hexf64!("-0x1.70170edbd047bp-875")).sin(), n2i(hexf64!("-0x1.fb31317bb132bp-326"), hexf64!("-0x1.70170edbd047ap-875")));
    assert_eq!(n2i(hexf64!("-0x1.8378f49913a88p-253"), hexf64!("0x1.53fab12968e9ap-607")).sin(), n2i(hexf64!("-0x1.8378f49913a88p-253"), hexf64!("0x1.53fab12968e9ap-607")));
    assert_eq!(n2i(hexf64!("-0x1.79ca1af65e50dp-233"), hexf64!("-0x1.cd61131067370p-429")).sin(), n2i(hexf64!("-0x1.79ca1af65e50dp-233"), hexf64!("-0x1.cd6113106736fp-429")));
    assert_eq!(n2i(hexf64!("-0x1.4c8993b11d519p-149"), hexf64!("-0x1.b1d8f24f24de3p-941")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.9ee1a9db994f5p-436"), hexf64!("-0x1.b6451c0720bfbp-622")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.59415fcfbff18p+6"), hexf64!("-0x1.1b0be7ac0af65p-959")).cos(), n2i(hexf64!("-0x1.0000000000000p+0"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.975299ccb0e08p-372"), hexf64!("0x1.77d8fa6b68b60p-585")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.a8e9c46a3d769p-355"), hexf64!("-0x1.c3a9cd7025105p-564")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.9b66c64d19ee1p-329"), hexf64!("-0x1.0b8dbebdff270p-411")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.32690aac2472dp-40"), hexf64!("-0x1.0e0dd7b9e7391p-789")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.40516bc314cc9p-198"), hexf64!("-0x1.7ad0659980c2bp-203")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.e80ef8fd19ad4p-265"), hexf64!("0x1.ba9a1304c562dp-79")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.764bf1b176ff7p-373"), hexf64!("-0x1.00bd1997cd82ep-915")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.b28f9484ffbfcp-589"), hexf64!("0x1.c77c9276b791dp-13")).cos(), n2i(hexf64!("0x1.ffffff3564fcbp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.388c331648e3ep-333"), hexf64!("-0x1.65ed85df2d4b7p-576")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.96433e013eda2p-935"), hexf64!("0x1.8620faa09eadbp-386")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.ab077c8e23ef5p-491"), hexf64!("0x1.504d993745eafp-18")).cos(), n2i(hexf64!("0x1.ffffffffe4634p-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.06d7f9ae94dadp-301"), hexf64!("-0x1.9610758986a88p-835")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.af3b21180e563p-498"), hexf64!("-0x1.068b13da99666p-760")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.2789c2d583bcdp-568"), hexf64!("-0x1.f2bd89dad0665p-780")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.9fc9d1b0afc7cp-545"), hexf64!("0x1.580844b9dc45cp-780")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.8a11a53596037p-49"), hexf64!("0x1.b1e6b793078ddp-664")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.425eef071014fp-121"), hexf64!("-0x1.bb2efb4f70837p-547")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.8e96354bf7e11p-894"), hexf64!("-0x1.039e2518cf503p-1008")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.7ba62e3fbdd83p-165"), hexf64!("0x1.069e434ee9e0fp-740")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.e7802992ba99dp-775"), hexf64!("-0x1.7883a587654e5p-928")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.455801d3d2b63p-704"), hexf64!("0x1.d2648abc1e83dp-27")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.04be837a6f1f1p-375"), hexf64!("0x1.173ab0fec92afp-771")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.7e13dbb66e5a3p-84"), hexf64!("-0x1.af23d175aa3d2p-538")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.07d4317cb3695p-274"), hexf64!("-0x1.ef8b7bcbab211p-495")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.fb31317bb132bp-326"), hexf64!("-0x1.70170edbd047bp-875")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.8378f49913a88p-253"), hexf64!("0x1.53fab12968e9ap-607")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("-0x1.79ca1af65e50dp-233"), hexf64!("-0x1.cd61131067370p-429")).cos(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.1abe4c698badcp+6"), hexf64!("0x1.1b03626b3e0fcp+6")).tan(), n2i(hexf64!("-0x1.0ae5296e191d2p+18"), hexf64!("-0x1.d98fdff87377dp+3")));
    assert_eq!(n2i(hexf64!("0x1.37048632cf1d3p+7"), hexf64!("0x1.3706ca82d6631p+7")).tan(), n2i(hexf64!("-0x1.2a2ec2193fe9ep+31"), hexf64!("-0x1.c3b9d7a157425p+7")));
    assert_eq!(n2i(hexf64!("0x1.c463adbcd6522p+3"), hexf64!("0x1.c4642ab313fabp+3")).tan(), n2i(hexf64!("-0x1.1a74cc770ca81p+20"), hexf64!("-0x1.02797f0002d0cp+14")));
    assert_eq!(n2i(hexf64!("0x1.d0f4ca2bb579cp+5"), hexf64!("0x1.d0fb1659ec586p+5")).tan(), n2i(hexf64!("-0x1.f6ddf31d42d10p+13"), hexf64!("-0x1.3ec2e09b0b82dp+8")));
    assert_eq!(n2i(hexf64!("0x1.141144b19d322p+10"), hexf64!("0x1.1411451f7d15cp+10")).tan(), n2i(hexf64!("-0x1.8bd033e5794e1p+31"), hexf64!("-0x1.2a3a40ea27098p+15")));
    assert_eq!(n2i(hexf64!("0x1.f6a7a3099ec00p+2"), hexf64!("0x1.5998e051427cap+3")).tan(), n2i(hexf64!("-0x1.19c4db4f71995p+23"), hexf64!("0x1.42e8a6723dd8fp+2")));
    assert_eq!(n2i(hexf64!("0x1.9953d2adc130ep+15"), hexf64!("0x1.9953fd66bd1a5p+15")).tan(), n2i(hexf64!("-0x1.281ad1a684049p+19"), hexf64!("-0x1.7e9968792c573p+3")));
    assert_eq!(n2i(hexf64!("0x1.7b58e9870d616p+9"), hexf64!("0x1.7b58f56c876b7p+9")).tan(), n2i(hexf64!("-0x1.1e211833a60a2p+20"), hexf64!("-0x1.577fab846bd1dp+11")));
    assert_eq!(n2i(hexf64!("0x1.7866ee95918a2p+11"), hexf64!("0x1.7866f1f2c8170p+11")).tan(), n2i(hexf64!("-0x1.dc74d19acf53bp+13"), hexf64!("-0x1.06780cb6101efp+11")));
    assert_eq!(n2i(hexf64!("0x1.2d97c809d003bp+2"), hexf64!("0x1.2d97da429140fp+2")).tan(), n2i(hexf64!("-0x1.6a35693eedaa4p+25"), hexf64!("-0x1.bf6798fccd6e2p+17")));
    assert_eq!(n2i(hexf64!("0x1.e305163ba6c53p+9"), hexf64!("0x1.e3053b5947216p+9")).tan(), n2i(hexf64!("-0x1.9471e22608da2p+25"), hexf64!("-0x1.b96c2d98deb30p+9")));
    assert_eq!(n2i(hexf64!("0x1.f6a7a2977cedfp+2"), hexf64!("0x1.f6aa73f91519bp+2")).tan(), n2i(hexf64!("-0x1.d9b11082e8fc3p+28"), hexf64!("-0x1.6b6333e88c1dfp+12")));
    assert_eq!(n2i(hexf64!("0x1.d0f4a998ad811p+5"), hexf64!("0x1.d1b127e4dc260p+5")).tan(), n2i(hexf64!("-0x1.24694a629d8bfp+26"), hexf64!("-0x1.5ab37594d2be9p+3")));
    assert_eq!(n2i(hexf64!("0x1.f6a7a2ac6dedfp+2"), hexf64!("0x1.1c94579773116p+3")).tan(), n2i(hexf64!("-0x1.62956c2a2d62ap+25"), hexf64!("-0x1.2d2399b72ea3dp-1")));
    assert_eq!(n2i(hexf64!("0x1.521e0f04ac063p+10"), hexf64!("0x1.521e18ea2831ep+10")).tan(), n2i(hexf64!("-0x1.1d19765e3fc7ap+6"), hexf64!("-0x1.115461c99eca5p+6")));
    assert_eq!(n2i(hexf64!("0x1.ab41b772619c2p+4"), hexf64!("0x1.ab96c295e1a23p+4")).tan(), n2i(hexf64!("-0x1.2aef4fa14f93bp+17"), hexf64!("-0x1.8121ce094b034p+5")));
    assert_eq!(n2i(hexf64!("0x1.87e20c8b7ccc1p+16"), hexf64!("0x1.87e20e060934fp+16")).tan(), n2i(hexf64!("-0x1.d7deec3154c47p+30"), hexf64!("-0x1.5a3ebabe6e61dp+7")));
    assert_eq!(n2i(hexf64!("0x1.2dd2b14821824p+17"), hexf64!("0x1.2dd2d30836efap+17")).tan(), n2i(hexf64!("-0x1.2f78f24545a36p+6"), hexf64!("-0x1.c276ac2300da6p+1")));
    assert_eq!(n2i(hexf64!("0x1.a9af913ee27cfp+8"), hexf64!("0x1.a9afa6d06e549p+8")).tan(), n2i(hexf64!("-0x1.65a2a934e5caap+17"), hexf64!("-0x1.759cdc5d55213p+11")));
    assert_eq!(n2i(hexf64!("0x1.d933d00e94652p+10"), hexf64!("0x1.d9344e7c644b9p+10")).tan(), n2i(hexf64!("-0x1.4e3dc1208291fp+30"), hexf64!("-0x1.032d0c507703dp+7")));
    assert_eq!(n2i(hexf64!("0x1.050dec23e2e9cp+18"), hexf64!("0x1.050e689fa1b54p+18")).tan(), n2i(hexf64!("-0x1.3adb30f7a9b56p+24"), hexf64!("0x1.9231d192a3e68p-2")));
    assert_eq!(n2i(hexf64!("0x1.3a28c59dd7dbep+5"), hexf64!("0x1.3a28c6f2d3060p+5")).tan(), n2i(hexf64!("-0x1.f1c7421b66cc8p+27"), hexf64!("-0x1.7fd15910ff62ep+18")));
    assert_eq!(n2i(hexf64!("0x1.dd85a7816a0ffp+4"), hexf64!("0x1.dd89849dfdf09p+4")).tan(), n2i(hexf64!("-0x1.fd2e45e8d33f6p+21"), hexf64!("-0x1.08f65ac0e1d44p+10")));
    assert_eq!(n2i(hexf64!("0x1.ab41b29964887p+4"), hexf64!("0x1.b5cbd4ed0d1ddp+4")).tan(), n2i(hexf64!("-0x1.ff22d5dee5cc4p+18"), hexf64!("-0x1.4ab8a7bb81552p+0")));
    assert_eq!(n2i(hexf64!("0x1.19454e95bc804p+12"), hexf64!("0x1.194551e9ad95ap+12")).tan(), n2i(hexf64!("-0x1.2a53ecf4c9b1ap+12"), hexf64!("-0x1.e93f83a616d4ap+9")));
    assert_eq!(n2i(hexf64!("0x1.31208f4b0c340p+10"), hexf64!("0x1.3120b7ab8e573p+10")).tan(), n2i(hexf64!("-0x1.fded0d28fcd51p+30"), hexf64!("-0x1.95c659f53b09bp+8")));
    assert_eq!(n2i(hexf64!("0x1.dd85baab9baf9p+4"), hexf64!("0x1.dd8660423baacp+4")).tan(), n2i(hexf64!("-0x1.a5ea670ea3939p+15"), hexf64!("-0x1.623d682415be0p+12")));
    assert_eq!(n2i(hexf64!("0x1.7bcc5d9a9b348p+17"), hexf64!("0x1.7bcd1b1984619p+17")).tan(), n2i(hexf64!("-0x1.7c60d60ad4e67p+17"), hexf64!("-0x1.731b7acf94603p-4")));
    assert_eq!(n2i(hexf64!("0x1.78fdb9f143616p+4"), hexf64!("0x1.78fdb9f143616p+4")).tan(), n2i(hexf64!("-0x1.939fac71fc143p+27"), hexf64!("-0x1.939fac71fc142p+27")));
    assert_eq!(n2i(hexf64!("0x1.78fdb9f143616p+4"), hexf64!("0x1.79367e02d4eaap+4")).tan(), n2i(hexf64!("-0x1.939fac71fc143p+27"), hexf64!("-0x1.209afe017139ap+6")));
    assert_eq!(n2i(hexf64!("-0x1.88d184a3af3b1p-163"), hexf64!("-0x1.86f08605c2aa0p-677")).asin(), n2i(hexf64!("-0x1.88d184a3af3b2p-163"), hexf64!("-0x1.86f08605c2aa0p-677")));
    assert_eq!(n2i(hexf64!("-0x1.5f745e909234ep-278"), hexf64!("-0x1.44c7e079c37bap-639")).asin(), n2i(hexf64!("-0x1.5f745e909234fp-278"), hexf64!("-0x1.44c7e079c37bap-639")));
    assert_eq!(n2i(hexf64!("-0x1.db07b9afeac94p-293"), hexf64!("-0x1.62f61fba0f40fp-764")).asin(), n2i(hexf64!("-0x1.db07b9afeac95p-293"), hexf64!("-0x1.62f61fba0f40fp-764")));
    assert_eq!(n2i(hexf64!("-0x1.67712a1e64c2cp-944"), hexf64!("-0x1.c0102c4d258efp-976")).asin(), n2i(hexf64!("-0x1.67712a1e64c2dp-944"), hexf64!("-0x1.c0102c4d258efp-976")));
    assert_eq!(n2i(hexf64!("0x1.71ecc8d742334p-727"), hexf64!("0x1.92c3c728ccf4ap-612")).asin(), n2i(hexf64!("0x1.71ecc8d742334p-727"), hexf64!("0x1.92c3c728ccf4bp-612")));
    assert_eq!(n2i(hexf64!("-0x1.bcd3feb3b0175p-640"), hexf64!("0x1.bebe69e3bf3c2p-536")).asin(), n2i(hexf64!("-0x1.bcd3feb3b0176p-640"), hexf64!("0x1.bebe69e3bf3c3p-536")));
    assert_eq!(n2i(hexf64!("-0x1.2469575189327p-372"), hexf64!("-0x1.d47030e7d6293p-1006")).asin(), n2i(hexf64!("-0x1.2469575189328p-372"), hexf64!("-0x1.d47030e7d6293p-1006")));
    assert_eq!(n2i(hexf64!("-0x1.c4d163a6cccd9p-336"), hexf64!("-0x1.3bee6dab70397p-796")).asin(), n2i(hexf64!("-0x1.c4d163a6cccdap-336"), hexf64!("-0x1.3bee6dab70397p-796")));
    assert_eq!(n2i(hexf64!("-0x1.07d72ef4864c6p-895"), hexf64!("-0x1.103cbbbf6120cp-975")).asin(), n2i(hexf64!("-0x1.07d72ef4864c7p-895"), hexf64!("-0x1.103cbbbf6120cp-975")));
    assert_eq!(n2i(hexf64!("-0x1.08c248c37e53bp-816"), hexf64!("0x1.464f82772ef42p-947")).asin(), n2i(hexf64!("-0x1.08c248c37e53cp-816"), hexf64!("0x1.464f82772ef43p-947")));
    assert_eq!(n2i(hexf64!("-0x1.00012a1580a3ap-227"), hexf64!("0x1.26acf90bdfba6p-795")).asin(), n2i(hexf64!("-0x1.00012a1580a3bp-227"), hexf64!("0x1.26acf90bdfba7p-795")));
    assert_eq!(n2i(hexf64!("0x1.ba8067112e534p-841"), hexf64!("0x1.d11146fe4675ep-554")).asin(), n2i(hexf64!("0x1.ba8067112e534p-841"), hexf64!("0x1.d11146fe4675fp-554")));
    assert_eq!(n2i(hexf64!("-0x1.0448c580b4caep-63"), hexf64!("0x1.9fe1322863725p-267")).asin(), n2i(hexf64!("-0x1.0448c580b4cafp-63"), hexf64!("0x1.9fe1322863726p-267")));
    assert_eq!(n2i(hexf64!("-0x1.3364adec6bb8bp-387"), hexf64!("0x1.7e16b310f878ap-232")).asin(), n2i(hexf64!("-0x1.3364adec6bb8cp-387"), hexf64!("0x1.7e16b310f878bp-232")));
    assert_eq!(n2i(hexf64!("-0x1.ece335e985bbap-255"), hexf64!("-0x1.2a30c3d9e32dap-454")).asin(), n2i(hexf64!("-0x1.ece335e985bbbp-255"), hexf64!("-0x1.2a30c3d9e32dap-454")));
    assert_eq!(n2i(hexf64!("-0x1.aa045ccb15aedp-804"), hexf64!("0x1.a8a188e64cac2p-21")).asin(), n2i(hexf64!("-0x1.aa045ccb15aeep-804"), hexf64!("0x1.a8a188e64cdcdp-21")));
    assert_eq!(n2i(hexf64!("-0x1.11d6fd2b8fe1ep-343"), hexf64!("-0x1.30d1074dc059ep-868")).asin(), n2i(hexf64!("-0x1.11d6fd2b8fe1fp-343"), hexf64!("-0x1.30d1074dc059ep-868")));
    assert_eq!(n2i(hexf64!("-0x1.efde0d25f9c44p-67"), hexf64!("-0x1.2a278e6c91f21p-838")).asin(), n2i(hexf64!("-0x1.efde0d25f9c45p-67"), hexf64!("-0x1.2a278e6c91f21p-838")));
    assert_eq!(n2i(hexf64!("0x1.2b060c8a4ba6ep-493"), hexf64!("0x1.633b2978352afp-407")).asin(), n2i(hexf64!("0x1.2b060c8a4ba6ep-493"), hexf64!("0x1.633b2978352b0p-407")));
    assert_eq!(n2i(hexf64!("-0x1.9c5fc40761841p-303"), hexf64!("-0x1.dec661df94dabp-510")).asin(), n2i(hexf64!("-0x1.9c5fc40761842p-303"), hexf64!("-0x1.dec661df94dabp-510")));
    assert_eq!(n2i(hexf64!("-0x1.56df81b91c381p-43"), hexf64!("0x1.98dc940c3ae1ep-564")).asin(), n2i(hexf64!("-0x1.56df81b91c382p-43"), hexf64!("0x1.98dc940c3ae1fp-564")));
    assert_eq!(n2i(hexf64!("-0x1.3929e7122ce96p-326"), hexf64!("0x1.451605cb3a73bp-853")).asin(), n2i(hexf64!("-0x1.3929e7122ce97p-326"), hexf64!("0x1.451605cb3a73cp-853")));
    assert_eq!(n2i(hexf64!("-0x1.a034fa1ee4476p-230"), hexf64!("-0x1.1efaa10962372p-519")).asin(), n2i(hexf64!("-0x1.a034fa1ee4477p-230"), hexf64!("-0x1.1efaa10962372p-519")));
    assert_eq!(n2i(hexf64!("-0x1.4e54c309c46f8p-480"), hexf64!("0x1.28fd3055907f3p-685")).asin(), n2i(hexf64!("-0x1.4e54c309c46f9p-480"), hexf64!("0x1.28fd3055907f4p-685")));
    assert_eq!(n2i(hexf64!("-0x1.13b101113d36fp-807"), hexf64!("0x1.c53e9ba64fadfp-768")).asin(), n2i(hexf64!("-0x1.13b101113d370p-807"), hexf64!("0x1.c53e9ba64fae0p-768")));
    assert_eq!(n2i(hexf64!("-0x1.2392d35ee9b74p-210"), hexf64!("-0x1.4b35284c1064bp-548")).asin(), n2i(hexf64!("-0x1.2392d35ee9b75p-210"), hexf64!("-0x1.4b35284c1064bp-548")));
    assert_eq!(n2i(hexf64!("-0x1.ae295c6cffac1p-247"), hexf64!("-0x1.57c346b295c33p-911")).asin(), n2i(hexf64!("-0x1.ae295c6cffac2p-247"), hexf64!("-0x1.57c346b295c33p-911")));
    assert_eq!(n2i(hexf64!("0x1.6938cc5ee183ap-692"), hexf64!("0x1.7ef4b0758702dp-661")).asin(), n2i(hexf64!("0x1.6938cc5ee183ap-692"), hexf64!("0x1.7ef4b0758702ep-661")));
    assert_eq!(n2i(hexf64!("-0x1.b459af91d9283p-559"), hexf64!("0x1.3f39248da0a27p-301")).asin(), n2i(hexf64!("-0x1.b459af91d9284p-559"), hexf64!("0x1.3f39248da0a28p-301")));
    assert_eq!(n2i(hexf64!("-0x1.57bce16d0a1d4p-513"), hexf64!("0x1.5dbb6adfb81fdp-1019")).asin(), n2i(hexf64!("-0x1.57bce16d0a1d5p-513"), hexf64!("0x1.5dbb6adfb81fep-1019")));
    assert_eq!(n2i(hexf64!("-0x1.88d184a3af3b1p-163"), hexf64!("-0x1.86f08605c2aa0p-677")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.5f745e909234ep-278"), hexf64!("-0x1.44c7e079c37bap-639")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.db07b9afeac94p-293"), hexf64!("-0x1.62f61fba0f40fp-764")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.67712a1e64c2cp-944"), hexf64!("-0x1.c0102c4d258efp-976")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.71ecc8d742334p-727"), hexf64!("0x1.92c3c728ccf4ap-612")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.bcd3feb3b0175p-640"), hexf64!("0x1.bebe69e3bf3c2p-536")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.2469575189327p-372"), hexf64!("-0x1.d47030e7d6293p-1006")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.c4d163a6cccd9p-336"), hexf64!("-0x1.3bee6dab70397p-796")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.07d72ef4864c6p-895"), hexf64!("-0x1.103cbbbf6120cp-975")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.08c248c37e53bp-816"), hexf64!("0x1.464f82772ef42p-947")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.00012a1580a3ap-227"), hexf64!("0x1.26acf90bdfba6p-795")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.ba8067112e534p-841"), hexf64!("0x1.d11146fe4675ep-554")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0448c580b4caep-63"), hexf64!("0x1.9fe1322863725p-267")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.3364adec6bb8bp-387"), hexf64!("0x1.7e16b310f878ap-232")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.ece335e985bbap-255"), hexf64!("-0x1.2a30c3d9e32dap-454")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.aa045ccb15aedp-804"), hexf64!("0x1.a8a188e64cac2p-21")).acos(), n2i(hexf64!("0x1.921fa7ff368a5p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.11d6fd2b8fe1ep-343"), hexf64!("-0x1.30d1074dc059ep-868")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.efde0d25f9c44p-67"), hexf64!("-0x1.2a278e6c91f21p-838")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.2b060c8a4ba6ep-493"), hexf64!("0x1.633b2978352afp-407")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.9c5fc40761841p-303"), hexf64!("-0x1.dec661df94dabp-510")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.56df81b91c381p-43"), hexf64!("0x1.98dc940c3ae1ep-564")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442fc7p+0")));
    assert_eq!(n2i(hexf64!("-0x1.3929e7122ce96p-326"), hexf64!("0x1.451605cb3a73bp-853")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.a034fa1ee4476p-230"), hexf64!("-0x1.1efaa10962372p-519")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.4e54c309c46f8p-480"), hexf64!("0x1.28fd3055907f3p-685")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.13b101113d36fp-807"), hexf64!("0x1.c53e9ba64fadfp-768")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.2392d35ee9b74p-210"), hexf64!("-0x1.4b35284c1064bp-548")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.ae295c6cffac1p-247"), hexf64!("-0x1.57c346b295c33p-911")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.6938cc5ee183ap-692"), hexf64!("0x1.7ef4b0758702dp-661")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.b459af91d9283p-559"), hexf64!("0x1.3f39248da0a27p-301")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("-0x1.57bce16d0a1d4p-513"), hexf64!("0x1.5dbb6adfb81fdp-1019")).acos(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.7a77bfccf5a9ep-232"), hexf64!("0x1.422bde014cc37p+113")).atan(), n2i(hexf64!("0x1.7a77bfccf5a9dp-232"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.e2f6187ddb464p-507"), hexf64!("0x1.81702ec6f2dccp-218")).atan(), n2i(hexf64!("0x1.e2f6187ddb463p-507"), hexf64!("0x1.81702ec6f2dccp-218")));
    assert_eq!(n2i(hexf64!("0x1.c41fe837a300bp-201"), hexf64!("0x1.4d84c8cdfcecfp+273")).atan(), n2i(hexf64!("0x1.c41fe837a300ap-201"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.03aefcc5923c6p-455"), hexf64!("0x1.1503444763fc5p-416")).atan(), n2i(hexf64!("0x1.03aefcc5923c5p-455"), hexf64!("0x1.1503444763fc5p-416")));
    assert_eq!(n2i(hexf64!("0x1.5444e676976f1p+252"), hexf64!("0x1.3e0327b49491ap+293")).atan(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.3a01905e36f84p+0"), hexf64!("0x1.3b2f21561c420p+334")).atan(), n2i(hexf64!("0x1.c60c5c686a64fp-1"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.202722624e322p-439"), hexf64!("0x1.ccbb6adfd8294p+238")).atan(), n2i(hexf64!("0x1.202722624e321p-439"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.63a15e999eb64p-344"), hexf64!("0x1.9498f3e13bce2p+174")).atan(), n2i(hexf64!("0x1.63a15e999eb63p-344"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.a507d1c29c01dp-754"), hexf64!("0x1.4171976a1ca54p-288")).atan(), n2i(hexf64!("0x1.a507d1c29c01cp-754"), hexf64!("0x1.4171976a1ca54p-288")));
    assert_eq!(n2i(hexf64!("0x1.de287d1d68c1bp-562"), hexf64!("0x1.1a6ec74d2e55cp-74")).atan(), n2i(hexf64!("0x1.de287d1d68c1ap-562"), hexf64!("0x1.1a6ec74d2e55cp-74")));
    assert_eq!(n2i(hexf64!("0x1.ba04d452bbb35p+180"), hexf64!("0x1.f0d19adcb5d74p+312")).atan(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.355e0aabcd959p-47"), hexf64!("0x1.70236daaa01bep+257")).atan(), n2i(hexf64!("0x1.355e0aabcd958p-47"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.052f533db3da1p+11"), hexf64!("0x1.c678939eb70f2p+655")).atan(), n2i(hexf64!("0x1.920057e3a66f1p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.a1b6ff47840cap+49"), hexf64!("0x1.534d4b4bb97efp+689")).atan(), n2i(hexf64!("0x1.921fb54442d13p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.8f96439899147p-170"), hexf64!("0x1.7aaa15ebbd3f2p+8")).atan(), n2i(hexf64!("0x1.8f96439899146p-170"), hexf64!("0x1.9172a3136eb8dp+0")));
    assert_eq!(n2i(hexf64!("0x1.ab3e906363e4bp-542"), hexf64!("0x1.47c9025e2020fp+989")).atan(), n2i(hexf64!("0x1.ab3e906363e4ap-542"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.5dab859ab6365p-439"), hexf64!("0x1.5802285246ac3p-147")).atan(), n2i(hexf64!("0x1.5dab859ab6364p-439"), hexf64!("0x1.5802285246ac3p-147")));
    assert_eq!(n2i(hexf64!("0x1.8826528c34186p-509"), hexf64!("0x1.9e7ddbbe00f75p+352")).atan(), n2i(hexf64!("0x1.8826528c34185p-509"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.9a5e124a8c193p+190"), hexf64!("0x1.abc408b9dd2e4p+239")).atan(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.f7a98177b25f3p-314"), hexf64!("0x1.5bd629b25aa23p-236")).atan(), n2i(hexf64!("0x1.f7a98177b25f2p-314"), hexf64!("0x1.5bd629b25aa23p-236")));
    assert_eq!(n2i(hexf64!("0x1.a0bd399ecd9ebp+427"), hexf64!("0x1.752884a66e610p+567")).atan(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.9975f82891ddep-54"), hexf64!("0x1.d70ee09d32965p+169")).atan(), n2i(hexf64!("0x1.9975f82891dddp-54"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.4ad560048c7b2p-342"), hexf64!("0x1.44f68aaa2b029p-65")).atan(), n2i(hexf64!("0x1.4ad560048c7b1p-342"), hexf64!("0x1.44f68aaa2b029p-65")));
    assert_eq!(n2i(hexf64!("0x1.8dad9877389a2p-234"), hexf64!("0x1.d68a6ba7e617fp+12")).atan(), n2i(hexf64!("0x1.8dad9877389a1p-234"), hexf64!("0x1.921700d14cfe5p+0")));
    assert_eq!(n2i(hexf64!("0x1.eb0e1ab78f314p-480"), hexf64!("0x1.98ef0c6a8bd66p+132")).atan(), n2i(hexf64!("0x1.eb0e1ab78f313p-480"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.d33c64c4c7e99p-190"), hexf64!("0x1.c08152cc09416p+220")).atan(), n2i(hexf64!("0x1.d33c64c4c7e98p-190"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.7036c237d5b00p-672"), hexf64!("0x1.bdc3836934ae8p-138")).atan(), n2i(hexf64!("0x1.7036c237d5affp-672"), hexf64!("0x1.bdc3836934ae8p-138")));
    assert_eq!(n2i(hexf64!("0x1.d283cf8f05665p+252"), hexf64!("0x1.649a33c01908cp+327")).atan(), n2i(hexf64!("0x1.921fb54442d18p+0"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.d07742228e496p-323"), hexf64!("0x1.f9926ff4661c8p+651")).atan(), n2i(hexf64!("0x1.d07742228e495p-323"), hexf64!("0x1.921fb54442d19p+0")));
    assert_eq!(n2i(hexf64!("0x1.8bca641025a83p-124"), hexf64!("0x1.7ddd664e50062p-118")).atan(), n2i(hexf64!("0x1.8bca641025a82p-124"), hexf64!("0x1.7ddd664e50062p-118")));
    assert_eq!(n2i(hexf64!("-0x1.a9ee4a4b6f050p-159"), hexf64!("-0x1.850fc1c21d837p-957")).sinh(), n2i(hexf64!("-0x1.a9ee4a4b6f051p-159"), hexf64!("-0x1.850fc1c21d837p-957")));
    assert_eq!(n2i(hexf64!("-0x1.cf3637dcbcc9ap-452"), hexf64!("-0x1.79211918bfccfp-634")).sinh(), n2i(hexf64!("-0x1.cf3637dcbcc9bp-452"), hexf64!("-0x1.79211918bfccfp-634")));
    assert_eq!(n2i(hexf64!("-0x1.0362421843787p+0"), hexf64!("-0x1.9e96677b4c52dp-971")).sinh(), n2i(hexf64!("-0x1.32197576f3697p+0"), hexf64!("-0x1.9e96677b4c52dp-971")));
    assert_eq!(n2i(hexf64!("-0x1.494a24a7585d1p-380"), hexf64!("0x1.a0790a9e3013ep-604")).sinh(), n2i(hexf64!("-0x1.494a24a7585d2p-380"), hexf64!("0x1.a0790a9e3013fp-604")));
    assert_eq!(n2i(hexf64!("-0x1.5b25e2f4ccc33p-367"), hexf64!("-0x1.e0c6608222185p-569")).sinh(), n2i(hexf64!("-0x1.5b25e2f4ccc34p-367"), hexf64!("-0x1.e0c6608222185p-569")));
    assert_eq!(n2i(hexf64!("-0x1.d5bd4bd3fb615p-334"), hexf64!("-0x1.3d028e4d2ccd5p-414")).sinh(), n2i(hexf64!("-0x1.d5bd4bd3fb616p-334"), hexf64!("-0x1.3d028e4d2ccd5p-414")));
    assert_eq!(n2i(hexf64!("-0x1.41c6785752b91p-45"), hexf64!("-0x1.1927ca3847669p-808")).sinh(), n2i(hexf64!("-0x1.41c6785752b92p-45"), hexf64!("-0x1.1927ca3847669p-808")));
    assert_eq!(n2i(hexf64!("-0x1.1bbbd6fe8b950p-208"), hexf64!("-0x1.463a32dba649dp-220")).sinh(), n2i(hexf64!("-0x1.1bbbd6fe8b951p-208"), hexf64!("-0x1.463a32dba649dp-220")));
    assert_eq!(n2i(hexf64!("0x1.4a43fa124554cp-266"), hexf64!("0x1.8c9af520c22c3p-96")).sinh(), n2i(hexf64!("0x1.4a43fa124554cp-266"), hexf64!("0x1.8c9af520c22c4p-96")));
    assert_eq!(n2i(hexf64!("-0x1.73999632a55dbp-383"), hexf64!("-0x1.4363967367f55p-932")).sinh(), n2i(hexf64!("-0x1.73999632a55dcp-383"), hexf64!("-0x1.4363967367f55p-932")));
    assert_eq!(n2i(hexf64!("-0x1.55ebb1d70a46ep-592"), hexf64!("0x1.1ccfe9451a14cp-18")).sinh(), n2i(hexf64!("-0x1.55ebb1d70a46fp-592"), hexf64!("0x1.1ccfe9451dc0ep-18")));
    assert_eq!(n2i(hexf64!("-0x1.1b4b8388a3d92p-340"), hexf64!("-0x1.aa3a9479c9892p-597")).sinh(), n2i(hexf64!("-0x1.1b4b8388a3d93p-340"), hexf64!("-0x1.aa3a9479c9892p-597")));
    assert_eq!(n2i(hexf64!("-0x1.086dcdd16f130p-950"), hexf64!("0x1.bc81b0724787cp-401")).sinh(), n2i(hexf64!("-0x1.086dcdd16f131p-950"), hexf64!("0x1.bc81b0724787dp-401")));
    assert_eq!(n2i(hexf64!("-0x1.73d14fa7da1cbp-504"), hexf64!("0x1.5b3afeeb17a85p-28")).sinh(), n2i(hexf64!("-0x1.73d14fa7da1ccp-504"), hexf64!("0x1.5b3afeeb17a86p-28")));
    assert_eq!(n2i(hexf64!("-0x1.9e69a4cbef833p-319"), hexf64!("-0x1.606c7bce75819p-852")).sinh(), n2i(hexf64!("-0x1.9e69a4cbef834p-319"), hexf64!("-0x1.606c7bce75819p-852")));
    assert_eq!(n2i(hexf64!("-0x1.8a6ad4adcb2e9p-513"), hexf64!("-0x1.33b9a95c55513p-772")).sinh(), n2i(hexf64!("-0x1.8a6ad4adcb2eap-513"), hexf64!("-0x1.33b9a95c55513p-772")));
    assert_eq!(n2i(hexf64!("-0x1.7ea21e54298f3p-586"), hexf64!("-0x1.5b83ba0d8aa3cp-799")).sinh(), n2i(hexf64!("-0x1.7ea21e54298f4p-586"), hexf64!("-0x1.5b83ba0d8aa3cp-799")));
    assert_eq!(n2i(hexf64!("-0x1.4a3b2325572ddp-547"), hexf64!("0x1.01cd385f009b5p-782")).sinh(), n2i(hexf64!("-0x1.4a3b2325572dep-547"), hexf64!("0x1.01cd385f009b6p-782")));
    assert_eq!(n2i(hexf64!("-0x1.f48d580b61d55p-65"), hexf64!("0x1.efa89f34f4188p-684")).sinh(), n2i(hexf64!("-0x1.f48d580b61d56p-65"), hexf64!("0x1.efa89f34f4189p-684")));
    assert_eq!(n2i(hexf64!("-0x1.329119640b5f5p-121"), hexf64!("-0x1.4a00c068d5157p-555")).sinh(), n2i(hexf64!("-0x1.329119640b5f6p-121"), hexf64!("-0x1.4a00c068d5157p-555")));
    assert_eq!(n2i(hexf64!("-0x1.fcecff2bc9670p-911"), hexf64!("-0x1.f807e4671256ep-1015")).sinh(), n2i(hexf64!("-0x1.fcecff2bc9671p-911"), hexf64!("-0x1.f807e4671256ep-1015")));
    assert_eq!(n2i(hexf64!("-0x1.f568a3be035acp-174"), hexf64!("0x1.8ba571ff9655bp-753")).sinh(), n2i(hexf64!("-0x1.f568a3be035adp-174"), hexf64!("0x1.8ba571ff9655cp-753")));
    assert_eq!(n2i(hexf64!("-0x1.6d772aa08698ap-782"), hexf64!("-0x1.48532232c10fdp-940")).sinh(), n2i(hexf64!("-0x1.6d772aa08698bp-782"), hexf64!("-0x1.48532232c10fdp-940")));
    assert_eq!(n2i(hexf64!("0x1.c088689d581bcp-719"), hexf64!("0x1.3ba1170c4c0e1p-43")).sinh(), n2i(hexf64!("0x1.c088689d581bcp-719"), hexf64!("0x1.3ba1170c4c0e2p-43")));
    assert_eq!(n2i(hexf64!("-0x1.4ad562c0b5178p-380"), hexf64!("0x1.e759a7a0754ddp-791")).sinh(), n2i(hexf64!("-0x1.4ad562c0b5179p-380"), hexf64!("0x1.e759a7a0754dep-791")));
    assert_eq!(n2i(hexf64!("-0x1.fb40bbd5b902cp-90"), hexf64!("-0x1.8c80275a696b0p-552")).sinh(), n2i(hexf64!("-0x1.fb40bbd5b902dp-90"), hexf64!("-0x1.8c80275a696b0p-552")));
    assert_eq!(n2i(hexf64!("-0x1.56fbe834fc822p-296"), hexf64!("-0x1.b3020aac5d654p-515")).sinh(), n2i(hexf64!("-0x1.56fbe834fc823p-296"), hexf64!("-0x1.b3020aac5d654p-515")));
    assert_eq!(n2i(hexf64!("-0x1.911493dbf33e3p-338"), hexf64!("-0x1.4fa3b55da84cdp-894")).sinh(), n2i(hexf64!("-0x1.911493dbf33e4p-338"), hexf64!("-0x1.4fa3b55da84cdp-894")));
    assert_eq!(n2i(hexf64!("-0x1.125eb7db4b38fp-265"), hexf64!("0x1.28d88e1763b5dp-611")).sinh(), n2i(hexf64!("-0x1.125eb7db4b390p-265"), hexf64!("0x1.28d88e1763b5ep-611")));
    assert_eq!(n2i(hexf64!("-0x1.1e1711af70a94p-242"), hexf64!("-0x1.ba5c062de8f00p-432")).sinh(), n2i(hexf64!("-0x1.1e1711af70a95p-242"), hexf64!("-0x1.ba5c062de8f00p-432")));
    assert_eq!(n2i(hexf64!("-0x1.a9ee4a4b6f050p-159"), hexf64!("-0x1.850fc1c21d837p-957")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.cf3637dcbcc9ap-452"), hexf64!("-0x1.79211918bfccfp-634")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.0362421843787p+0"), hexf64!("-0x1.9e96677b4c52dp-971")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.8f0a39674b193p+0")));
    assert_eq!(n2i(hexf64!("-0x1.494a24a7585d1p-380"), hexf64!("0x1.a0790a9e3013ep-604")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.5b25e2f4ccc33p-367"), hexf64!("-0x1.e0c6608222185p-569")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.d5bd4bd3fb615p-334"), hexf64!("-0x1.3d028e4d2ccd5p-414")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.41c6785752b91p-45"), hexf64!("-0x1.1927ca3847669p-808")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.1bbbd6fe8b950p-208"), hexf64!("-0x1.463a32dba649dp-220")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("0x1.4a43fa124554cp-266"), hexf64!("0x1.8c9af520c22c3p-96")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.73999632a55dbp-383"), hexf64!("-0x1.4363967367f55p-932")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.55ebb1d70a46ep-592"), hexf64!("0x1.1ccfe9451a14cp-18")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000009e6fp+0")));
    assert_eq!(n2i(hexf64!("-0x1.1b4b8388a3d92p-340"), hexf64!("-0x1.aa3a9479c9892p-597")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.086dcdd16f130p-950"), hexf64!("0x1.bc81b0724787cp-401")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.73d14fa7da1cbp-504"), hexf64!("0x1.5b3afeeb17a85p-28")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.9e69a4cbef833p-319"), hexf64!("-0x1.606c7bce75819p-852")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.8a6ad4adcb2e9p-513"), hexf64!("-0x1.33b9a95c55513p-772")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.7ea21e54298f3p-586"), hexf64!("-0x1.5b83ba0d8aa3cp-799")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.4a3b2325572ddp-547"), hexf64!("0x1.01cd385f009b5p-782")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.f48d580b61d55p-65"), hexf64!("0x1.efa89f34f4188p-684")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.329119640b5f5p-121"), hexf64!("-0x1.4a00c068d5157p-555")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.fcecff2bc9670p-911"), hexf64!("-0x1.f807e4671256ep-1015")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.f568a3be035acp-174"), hexf64!("0x1.8ba571ff9655bp-753")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.6d772aa08698ap-782"), hexf64!("-0x1.48532232c10fdp-940")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("0x1.c088689d581bcp-719"), hexf64!("0x1.3ba1170c4c0e1p-43")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.4ad562c0b5178p-380"), hexf64!("0x1.e759a7a0754ddp-791")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.fb40bbd5b902cp-90"), hexf64!("-0x1.8c80275a696b0p-552")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.56fbe834fc822p-296"), hexf64!("-0x1.b3020aac5d654p-515")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.911493dbf33e3p-338"), hexf64!("-0x1.4fa3b55da84cdp-894")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.125eb7db4b38fp-265"), hexf64!("0x1.28d88e1763b5dp-611")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("-0x1.1e1711af70a94p-242"), hexf64!("-0x1.ba5c062de8f00p-432")).cosh(), n2i(hexf64!("0x1.0000000000000p+0"), hexf64!("0x1.0000000000001p+0")));
    assert_eq!(n2i(hexf64!("0x1.7a77bfccf5a9ep-232"), hexf64!("0x1.422bde014cc37p+113")).tanh(), n2i(hexf64!("0x1.7a77bfccf5a9dp-232"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.e2f6187ddb464p-507"), hexf64!("0x1.81702ec6f2dccp-218")).tanh(), n2i(hexf64!("0x1.e2f6187ddb463p-507"), hexf64!("0x1.81702ec6f2dccp-218")));
    assert_eq!(n2i(hexf64!("0x1.c41fe837a300bp-201"), hexf64!("0x1.4d84c8cdfcecfp+273")).tanh(), n2i(hexf64!("0x1.c41fe837a300ap-201"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.03aefcc5923c6p-455"), hexf64!("0x1.1503444763fc5p-416")).tanh(), n2i(hexf64!("0x1.03aefcc5923c5p-455"), hexf64!("0x1.1503444763fc5p-416")));
    assert_eq!(n2i(hexf64!("0x1.5444e676976f1p+252"), hexf64!("0x1.3e0327b49491ap+293")).tanh(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.3a01905e36f84p+0"), hexf64!("0x1.3b2f21561c420p+334")).tanh(), n2i(hexf64!("0x1.aee466bcc4973p-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.202722624e322p-439"), hexf64!("0x1.ccbb6adfd8294p+238")).tanh(), n2i(hexf64!("0x1.202722624e321p-439"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.63a15e999eb64p-344"), hexf64!("0x1.9498f3e13bce2p+174")).tanh(), n2i(hexf64!("0x1.63a15e999eb63p-344"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.a507d1c29c01dp-754"), hexf64!("0x1.4171976a1ca54p-288")).tanh(), n2i(hexf64!("0x1.a507d1c29c01cp-754"), hexf64!("0x1.4171976a1ca54p-288")));
    assert_eq!(n2i(hexf64!("0x1.de287d1d68c1bp-562"), hexf64!("0x1.1a6ec74d2e55cp-74")).tanh(), n2i(hexf64!("0x1.de287d1d68c1ap-562"), hexf64!("0x1.1a6ec74d2e55cp-74")));
    assert_eq!(n2i(hexf64!("0x1.ba04d452bbb35p+180"), hexf64!("0x1.f0d19adcb5d74p+312")).tanh(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.355e0aabcd959p-47"), hexf64!("0x1.70236daaa01bep+257")).tanh(), n2i(hexf64!("0x1.355e0aabcd958p-47"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.052f533db3da1p+11"), hexf64!("0x1.c678939eb70f2p+655")).tanh(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.a1b6ff47840cap+49"), hexf64!("0x1.534d4b4bb97efp+689")).tanh(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.8f96439899147p-170"), hexf64!("0x1.7aaa15ebbd3f2p+8")).tanh(), n2i(hexf64!("0x1.8f96439899146p-170"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.ab3e906363e4bp-542"), hexf64!("0x1.47c9025e2020fp+989")).tanh(), n2i(hexf64!("0x1.ab3e906363e4ap-542"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.5dab859ab6365p-439"), hexf64!("0x1.5802285246ac3p-147")).tanh(), n2i(hexf64!("0x1.5dab859ab6364p-439"), hexf64!("0x1.5802285246ac3p-147")));
    assert_eq!(n2i(hexf64!("0x1.8826528c34186p-509"), hexf64!("0x1.9e7ddbbe00f75p+352")).tanh(), n2i(hexf64!("0x1.8826528c34185p-509"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.9a5e124a8c193p+190"), hexf64!("0x1.abc408b9dd2e4p+239")).tanh(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.f7a98177b25f3p-314"), hexf64!("0x1.5bd629b25aa23p-236")).tanh(), n2i(hexf64!("0x1.f7a98177b25f2p-314"), hexf64!("0x1.5bd629b25aa23p-236")));
    assert_eq!(n2i(hexf64!("0x1.a0bd399ecd9ebp+427"), hexf64!("0x1.752884a66e610p+567")).tanh(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.9975f82891ddep-54"), hexf64!("0x1.d70ee09d32965p+169")).tanh(), n2i(hexf64!("0x1.9975f82891dddp-54"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.4ad560048c7b2p-342"), hexf64!("0x1.44f68aaa2b029p-65")).tanh(), n2i(hexf64!("0x1.4ad560048c7b1p-342"), hexf64!("0x1.44f68aaa2b029p-65")));
    assert_eq!(n2i(hexf64!("0x1.8dad9877389a2p-234"), hexf64!("0x1.d68a6ba7e617fp+12")).tanh(), n2i(hexf64!("0x1.8dad9877389a1p-234"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.eb0e1ab78f314p-480"), hexf64!("0x1.98ef0c6a8bd66p+132")).tanh(), n2i(hexf64!("0x1.eb0e1ab78f313p-480"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.d33c64c4c7e99p-190"), hexf64!("0x1.c08152cc09416p+220")).tanh(), n2i(hexf64!("0x1.d33c64c4c7e98p-190"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.7036c237d5b00p-672"), hexf64!("0x1.bdc3836934ae8p-138")).tanh(), n2i(hexf64!("0x1.7036c237d5affp-672"), hexf64!("0x1.bdc3836934ae8p-138")));
    assert_eq!(n2i(hexf64!("0x1.d283cf8f05665p+252"), hexf64!("0x1.649a33c01908cp+327")).tanh(), n2i(hexf64!("0x1.fffffffffffffp-1"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.d07742228e496p-323"), hexf64!("0x1.f9926ff4661c8p+651")).tanh(), n2i(hexf64!("0x1.d07742228e495p-323"), hexf64!("0x1.0000000000000p+0")));
    assert_eq!(n2i(hexf64!("0x1.8bca641025a83p-124"), hexf64!("0x1.7ddd664e50062p-118")).tanh(), n2i(hexf64!("0x1.8bca641025a82p-124"), hexf64!("0x1.7ddd664e50062p-118")));
    assert_eq!(n2i(hexf64!("0x1.7a77bfccf5a9ep-232"), hexf64!("0x1.422bde014cc37p+113")).asinh(), n2i(hexf64!("0x1.7a77bfccf5a9dp-232"), hexf64!("0x1.3cfea73418196p+6")));
    assert_eq!(n2i(hexf64!("0x1.e2f6187ddb464p-507"), hexf64!("0x1.81702ec6f2dccp-218")).asinh(), n2i(hexf64!("0x1.e2f6187ddb463p-507"), hexf64!("0x1.81702ec6f2dccp-218")));
    assert_eq!(n2i(hexf64!("0x1.c41fe837a300bp-201"), hexf64!("0x1.4d84c8cdfcecfp+273")).asinh(), n2i(hexf64!("0x1.c41fe837a300ap-201"), hexf64!("0x1.7c5faaa9c6e7cp+7")));
    assert_eq!(n2i(hexf64!("0x1.03aefcc5923c6p-455"), hexf64!("0x1.1503444763fc5p-416")).asinh(), n2i(hexf64!("0x1.03aefcc5923c5p-455"), hexf64!("0x1.1503444763fc5p-416")));
    assert_eq!(n2i(hexf64!("0x1.5444e676976f1p+252"), hexf64!("0x1.3e0327b49491ap+293")).asinh(), n2i(hexf64!("0x1.5f4d3526a4a27p+7"), hexf64!("0x1.98011e3b75072p+7")));
    assert_eq!(n2i(hexf64!("0x1.3a01905e36f84p+0"), hexf64!("0x1.3b2f21561c420p+334")).asinh(), n2i(hexf64!("0x1.086affae230b6p+0"), hexf64!("0x1.d0d31703c4556p+7")));
    assert_eq!(n2i(hexf64!("0x1.202722624e322p-439"), hexf64!("0x1.ccbb6adfd8294p+238")).asinh(), n2i(hexf64!("0x1.202722624e321p-439"), hexf64!("0x1.4c7fe7cfad6e6p+7")));
    assert_eq!(n2i(hexf64!("0x1.63a15e999eb64p-344"), hexf64!("0x1.9498f3e13bce2p+174")).asinh(), n2i(hexf64!("0x1.63a15e999eb63p-344"), hexf64!("0x1.e708acd6c73b8p+6")));
    assert_eq!(n2i(hexf64!("0x1.a507d1c29c01dp-754"), hexf64!("0x1.4171976a1ca54p-288")).asinh(), n2i(hexf64!("0x1.a507d1c29c01cp-754"), hexf64!("0x1.4171976a1ca54p-288")));
    assert_eq!(n2i(hexf64!("0x1.de287d1d68c1bp-562"), hexf64!("0x1.1a6ec74d2e55cp-74")).asinh(), n2i(hexf64!("0x1.de287d1d68c1ap-562"), hexf64!("0x1.1a6ec74d2e55cp-74")));
    assert_eq!(n2i(hexf64!("0x1.ba04d452bbb35p+180"), hexf64!("0x1.f0d19adcb5d74p+312")).asinh(), n2i(hexf64!("0x1.f805f45086bb5p+6"), hexf64!("0x1.b33c799e851aep+7")));
    assert_eq!(n2i(hexf64!("0x1.355e0aabcd959p-47"), hexf64!("0x1.70236daaa01bep+257")).asinh(), n2i(hexf64!("0x1.355e0aabcd958p-47"), hexf64!("0x1.6663f857841cep+7")));
    //asinh [0X8.297A99ED9ED08P+8, 0XE.33C49CF5B8790P+652] = [0X8.567B3095B8380P+0, 0X1.C7474C3E00682P+8];
    //asinh [0X3.436DFE8F08194P+48, 0X2.A69A969772FDEP+688] = [0X2.325A084AF897EP+4, 0X1.DE8DA2F064858P+8];
    assert_eq!(n2i(hexf64!("0x1.8f96439899147p-170"), hexf64!("0x1.7aaa15ebbd3f2p+8")).asinh(), n2i(hexf64!("0x1.8f96439899146p-170"), hexf64!("0x1.a84ea179951e2p+2")));
    assert_eq!(n2i(hexf64!("0x1.ab3e906363e4bp-542"), hexf64!("0x1.47c9025e2020fp+989")).asinh(), n2i(hexf64!("0x1.ab3e906363e4ap-542"), hexf64!("0x1.573b3ff919870p+9")));
    assert_eq!(n2i(hexf64!("0x1.5dab859ab6365p-439"), hexf64!("0x1.5802285246ac3p-147")).asinh(), n2i(hexf64!("0x1.5dab859ab6364p-439"), hexf64!("0x1.5802285246ac3p-147")));
    assert_eq!(n2i(hexf64!("0x1.8826528c34186p-509"), hexf64!("0x1.9e7ddbbe00f75p+352")).asinh(), n2i(hexf64!("0x1.8826528c34185p-509"), hexf64!("0x1.ea535e77020fcp+7")));
    assert_eq!(n2i(hexf64!("0x1.9a5e124a8c193p+190"), hexf64!("0x1.abc408b9dd2e4p+239")).asinh(), n2i(hexf64!("0x1.09b9d970a5c3dp+7"), hexf64!("0x1.4dbcc91540344p+7")));
    assert_eq!(n2i(hexf64!("0x1.f7a98177b25f3p-314"), hexf64!("0x1.5bd629b25aa23p-236")).asinh(), n2i(hexf64!("0x1.f7a98177b25f2p-314"), hexf64!("0x1.5bd629b25aa23p-236")));
    //asinh [0XD.05E9CCF66CF58P+424, 0XB.A944253373080P+564] = [0X1.29277EA798036P+8, 0X1.8A159CCBD552AP+8];
    assert_eq!(n2i(hexf64!("0x1.9975f82891ddep-54"), hexf64!("0x1.d70ee09d32965p+169")).asinh(), n2i(hexf64!("0x1.9975f82891dddp-54"), hexf64!("0x1.d9c78021da20cp+6")));
    assert_eq!(n2i(hexf64!("0x1.4ad560048c7b2p-342"), hexf64!("0x1.44f68aaa2b029p-65")).asinh(), n2i(hexf64!("0x1.4ad560048c7b1p-342"), hexf64!("0x1.44f68aaa2b029p-65")));
    assert_eq!(n2i(hexf64!("0x1.8dad9877389a2p-234"), hexf64!("0x1.d68a6ba7e617fp+12")).asinh(), n2i(hexf64!("0x1.8dad9877389a1p-234"), hexf64!("0x1.33d3e9eb4ac11p+3")));
    assert_eq!(n2i(hexf64!("0x1.eb0e1ab78f314p-480"), hexf64!("0x1.98ef0c6a8bd66p+132")).asinh(), n2i(hexf64!("0x1.eb0e1ab78f313p-480"), hexf64!("0x1.72a0b7b6cf4c8p+6")));
    assert_eq!(n2i(hexf64!("0x1.d33c64c4c7e99p-190"), hexf64!("0x1.c08152cc09416p+220")).asinh(), n2i(hexf64!("0x1.d33c64c4c7e98p-190"), hexf64!("0x1.337e170ecd30ap+7")));
    assert_eq!(n2i(hexf64!("0x1.7036c237d5b00p-672"), hexf64!("0x1.bdc3836934ae8p-138")).asinh(), n2i(hexf64!("0x1.7036c237d5affp-672"), hexf64!("0x1.bdc3836934ae8p-138")));
    assert_eq!(n2i(hexf64!("0x1.d283cf8f05665p+252"), hexf64!("0x1.649a33c01908cp+327")).asinh(), n2i(hexf64!("0x1.5feec551661d4p+7"), hexf64!("0x1.c75e106d65eb6p+7")));
    //asinh [0X3.A0EE84451C92CP-324, 0XF.CC937FA330E40P+648] = [0X3.A0EE84451C92AP-324, 0X1.C49CCB25BBD50P+8];
    assert_eq!(n2i(hexf64!("0x1.8bca641025a83p-124"), hexf64!("0x1.7ddd664e50062p-118")).asinh(), n2i(hexf64!("0x1.8bca641025a82p-124"), hexf64!("0x1.7ddd664e50062p-118")));
    assert_eq!(n2i(hexf64!("0x1.6b626b453771dp+274"), hexf64!("0x1.3762e98b3852fp+861")).acosh(), n2i(hexf64!("0x1.7dee77e85833bp+7"), hexf64!("0x1.2ad828325b615p+9")));
    assert_eq!(n2i(hexf64!("0x1.2915f0d619250p+346"), hexf64!("0x1.1d6f7daa5e024p+445")).acosh(), n2i(hexf64!("0x1.e1578227b635ep+7"), hexf64!("0x1.3540a29bc6b5ep+8")));
    assert_eq!(n2i(hexf64!("0x1.a50722e541b64p+58"), hexf64!("0x1.d88ec041d74acp+745")).acosh(), n2i(hexf64!("0x1.4b2548cbdfc19p+5"), hexf64!("0x1.02d9b31fd7329p+9")));
    assert_eq!(n2i(hexf64!("0x1.09a940a083ee3p+132"), hexf64!("0x1.047fd514adf08p+384")).acosh(), n2i(hexf64!("0x1.70e708b5eaa42p+6"), hexf64!("0x1.0ae10bdf7fe81p+8")));
    assert_eq!(n2i(hexf64!("0x1.dc41d02dc6835p+258"), hexf64!("0x1.f80f31a2ad7f4p+563")).acosh(), n2i(hexf64!("0x1.684ab36b9e69dp+7"), hexf64!("0x1.879cce87ee3a1p+8")));
    assert_eq!(n2i(hexf64!("0x1.1b90d00fb822bp+565"), hexf64!("0x1.38c577e63fa52p+731")).acosh(), n2i(hexf64!("0x1.886c6f1dddff7p+8"), hexf64!("0x1.fb958311209bfp+8")));
    assert_eq!(n2i(hexf64!("0x1.bc1b514af73bcp+46"), hexf64!("0x1.2412c989b35dep+521")).acosh(), n2i(hexf64!("0x1.0907cade8fb83p+5"), hexf64!("0x1.69f464545899ap+8")));
    assert_eq!(n2i(hexf64!("0x1.74c9b52c220f6p+78"), hexf64!("0x1.77175c5a113adp+528")).acosh(), n2i(hexf64!("0x1.b9135f80abc98p+5"), hexf64!("0x1.6f0e8dbf98710p+8")));
    assert_eq!(n2i(hexf64!("0x1.05adfe119d4c2p+296"), hexf64!("0x1.35e6fa8702f1ap+1021")).acosh(), n2i(hexf64!("0x1.9bc5f7cf1fd85p+7"), hexf64!("0x1.624b3322cf0d2p+9")));
    assert_eq!(n2i(hexf64!("0x1.4feba4ab7024dp+411"), hexf64!("0x1.9d1eddc132b36p+864")).acosh(), n2i(hexf64!("0x1.1dd92c82827a1p+8"), hexf64!("0x1.2c06830395f90p+9")));
    assert_eq!(n2i(hexf64!("0x1.9f7c9c58da150p+487"), hexf64!("0x1.37dd6f705f0ccp+870")).acosh(), n2i(hexf64!("0x1.52bd770642386p+8"), hexf64!("0x1.2df6dbf1bb70cp+9")));
    assert_eq!(n2i(hexf64!("0x1.6482065f2e014p+383"), hexf64!("0x1.04a65b06b2920p+640")).acosh(), n2i(hexf64!("0x1.0a7fec190cceep+8"), hexf64!("0x1.bc5349b021815p+8")));
    assert_eq!(n2i(hexf64!("0x1.7c81fabd7e2edp+610"), hexf64!("0x1.5790f8df215dcp+651")).acosh(), n2i(hexf64!("0x1.a7e8c5069ea32p+8"), hexf64!("0x1.c439e5e8a511cp+8")));
    assert_eq!(n2i(hexf64!("0x1.bcd62c46adfd7p+16"), hexf64!("0x1.59a51366bd9a0p+567")).acosh(), n2i(hexf64!("0x1.8ac0c581882a3p+3"), hexf64!("0x1.8a0201556d85ap+8")));
    assert_eq!(n2i(hexf64!("0x1.94d2ee90282d1p+226"), hexf64!("0x1.8de35856e91fbp+452")).acosh(), n2i(hexf64!("0x1.3b9af9818fff7p+7"), hexf64!("0x1.3a6fc95911674p+8")));
    assert_eq!(n2i(hexf64!("0x1.627ac8097a724p+509"), hexf64!("0x1.1854765a9a205p+688")).acosh(), n2i(hexf64!("0x1.61d49df92ab79p+8"), hexf64!("0x1.ddab5081e5a0ep+8")));
    assert_eq!(n2i(hexf64!("0x1.0df120a458316p+47"), hexf64!("0x1.fc3ac96b8f036p+221")).acosh(), n2i(hexf64!("0x1.0a97be96df95ep+5"), hexf64!("0x1.3520fcf8e02d2p+7")));
    assert_eq!(n2i(hexf64!("0x1.255206b4af12ep+127"), hexf64!("0x1.35ee42dca8b75p+608")).acosh(), n2i(hexf64!("0x1.636f8a9fa5a64p+6"), hexf64!("0x1.a6515b9ecf2f1p+8")));
    assert_eq!(n2i(hexf64!("0x1.49904457e1b4ep+206"), hexf64!("0x1.ef8cf42039961p+254")).acosh(), n2i(hexf64!("0x1.1f77d677098e1p+7"), hexf64!("0x1.62d377c2627edp+7")));
    assert_eq!(n2i(hexf64!("0x1.50e8dee5437b2p+75"), hexf64!("0x1.5aaf8a68c5548p+874")).acosh(), n2i(hexf64!("0x1.a7a16dfe0725ap+5"), hexf64!("0x1.2f674c9163c4ap+9")));
    assert_eq!(n2i(hexf64!("0x1.7a12e7fee0800p+227"), hexf64!("0x1.17ecd5de84e9ap+977")).acosh(), n2i(hexf64!("0x1.3cdadd40c660ep+7"), hexf64!("0x1.52fe5f6e8af96p+9")));
    assert_eq!(n2i(hexf64!("0x1.582baa3b6fb3ep+539"), hexf64!("0x1.7207a70831d7ap+796")).acosh(), n2i(hexf64!("0x1.76986e964982dp+8"), hexf64!("0x1.1467423fb4ee5p+9")));
    assert_eq!(n2i(hexf64!("0x1.0a02fb1e4479fp+182"), hexf64!("0x1.ad3c47a1bb28ep+794")).acosh(), n2i(hexf64!("0x1.fb898511b757ep+6"), hexf64!("0x1.13c8cfa479610p+9")));
    assert_eq!(n2i(hexf64!("0x1.a59133a4b0587p+469"), hexf64!("0x1.4a0b40c3be99bp+811")).acosh(), n2i(hexf64!("0x1.4647297977e4ep+8"), hexf64!("0x1.198b76ab804eap+9")));
    assert_eq!(n2i(hexf64!("0x1.1c1fe13eaf272p+757"), hexf64!("0x1.623a725959795p+922")).acosh(), n2i(hexf64!("0x1.06c14110011acp+9"), hexf64!("0x1.400cc014caa86p+9")));
    assert_eq!(n2i(hexf64!("0x1.20e762c8aab55p+72"), hexf64!("0x1.dbb7b8fe35847p+960")).acosh(), n2i(hexf64!("0x1.95c3e9be4f57dp+5"), hexf64!("0x1.4d5df6953bf31p+9")));
    assert_eq!(n2i(hexf64!("0x1.1265074e9e3dfp+792"), hexf64!("0x1.114d942d384e0p+921")).acosh(), n2i(hexf64!("0x1.12de17e43b75ep+9"), hexf64!("0x1.3f92d44ff0b73p+9")));
    assert_eq!(n2i(hexf64!("0x1.054dcdef21349p+436"), hexf64!("0x1.618994d07d9a7p+636")).acosh(), n2i(hexf64!("0x1.2eed02d819a15p+8"), hexf64!("0x1.b9db8970cc5ecp+8")));
    assert_eq!(n2i(hexf64!("0x1.16f83669ffc33p+462"), hexf64!("0x1.37962af8b1aa5p+569")).acosh(), n2i(hexf64!("0x1.4103588c86ab6p+8"), hexf64!("0x1.8b4a572e180d4p+8")));
    assert_eq!(n2i(hexf64!("0x1.37b1734ff6cc6p+83"), hexf64!("0x1.56afc22c64391p+769")).acosh(), n2i(hexf64!("0x1.d35ea0b7e67b8p+5"), hexf64!("0x1.0b01ea76e59b5p+9")));
    assert_eq!(n2i(hexf64!("-0x1.88d184a3af3b1p-163"), hexf64!("-0x1.86f08605c2aa0p-677")).atanh(), n2i(hexf64!("-0x1.88d184a3af3b2p-163"), hexf64!("-0x1.86f08605c2aa0p-677")));
    assert_eq!(n2i(hexf64!("-0x1.5f745e909234ep-278"), hexf64!("-0x1.44c7e079c37bap-639")).atanh(), n2i(hexf64!("-0x1.5f745e909234fp-278"), hexf64!("-0x1.44c7e079c37bap-639")));
    assert_eq!(n2i(hexf64!("-0x1.db07b9afeac94p-293"), hexf64!("-0x1.62f61fba0f40fp-764")).atanh(), n2i(hexf64!("-0x1.db07b9afeac95p-293"), hexf64!("-0x1.62f61fba0f40fp-764")));
    assert_eq!(n2i(hexf64!("-0x1.67712a1e64c2cp-944"), hexf64!("-0x1.c0102c4d258efp-976")).atanh(), n2i(hexf64!("-0x1.67712a1e64c2dp-944"), hexf64!("-0x1.c0102c4d258efp-976")));
    assert_eq!(n2i(hexf64!("0x1.71ecc8d742334p-727"), hexf64!("0x1.92c3c728ccf4ap-612")).atanh(), n2i(hexf64!("0x1.71ecc8d742334p-727"), hexf64!("0x1.92c3c728ccf4bp-612")));
    assert_eq!(n2i(hexf64!("-0x1.bcd3feb3b0175p-640"), hexf64!("0x1.bebe69e3bf3c2p-536")).atanh(), n2i(hexf64!("-0x1.bcd3feb3b0176p-640"), hexf64!("0x1.bebe69e3bf3c3p-536")));
    assert_eq!(n2i(hexf64!("-0x1.2469575189327p-372"), hexf64!("-0x1.d47030e7d6293p-1006")).atanh(), n2i(hexf64!("-0x1.2469575189328p-372"), hexf64!("-0x1.d47030e7d6293p-1006")));
    assert_eq!(n2i(hexf64!("-0x1.c4d163a6cccd9p-336"), hexf64!("-0x1.3bee6dab70397p-796")).atanh(), n2i(hexf64!("-0x1.c4d163a6cccdap-336"), hexf64!("-0x1.3bee6dab70397p-796")));
    assert_eq!(n2i(hexf64!("-0x1.07d72ef4864c6p-895"), hexf64!("-0x1.103cbbbf6120cp-975")).atanh(), n2i(hexf64!("-0x1.07d72ef4864c7p-895"), hexf64!("-0x1.103cbbbf6120cp-975")));
    assert_eq!(n2i(hexf64!("-0x1.08c248c37e53bp-816"), hexf64!("0x1.464f82772ef42p-947")).atanh(), n2i(hexf64!("-0x1.08c248c37e53cp-816"), hexf64!("0x1.464f82772ef43p-947")));
    assert_eq!(n2i(hexf64!("-0x1.00012a1580a3ap-227"), hexf64!("0x1.26acf90bdfba6p-795")).atanh(), n2i(hexf64!("-0x1.00012a1580a3bp-227"), hexf64!("0x1.26acf90bdfba7p-795")));
    assert_eq!(n2i(hexf64!("0x1.ba8067112e534p-841"), hexf64!("0x1.d11146fe4675ep-554")).atanh(), n2i(hexf64!("0x1.ba8067112e534p-841"), hexf64!("0x1.d11146fe4675fp-554")));
    assert_eq!(n2i(hexf64!("-0x1.0448c580b4caep-63"), hexf64!("0x1.9fe1322863725p-267")).atanh(), n2i(hexf64!("-0x1.0448c580b4cafp-63"), hexf64!("0x1.9fe1322863726p-267")));
    assert_eq!(n2i(hexf64!("-0x1.3364adec6bb8bp-387"), hexf64!("0x1.7e16b310f878ap-232")).atanh(), n2i(hexf64!("-0x1.3364adec6bb8cp-387"), hexf64!("0x1.7e16b310f878bp-232")));
    assert_eq!(n2i(hexf64!("-0x1.ece335e985bbap-255"), hexf64!("-0x1.2a30c3d9e32dap-454")).atanh(), n2i(hexf64!("-0x1.ece335e985bbbp-255"), hexf64!("-0x1.2a30c3d9e32dap-454")));
    assert_eq!(n2i(hexf64!("-0x1.aa045ccb15aedp-804"), hexf64!("0x1.a8a188e64cac2p-21")).atanh(), n2i(hexf64!("-0x1.aa045ccb15aeep-804"), hexf64!("0x1.a8a188e64d0d8p-21")));
    assert_eq!(n2i(hexf64!("-0x1.11d6fd2b8fe1ep-343"), hexf64!("-0x1.30d1074dc059ep-868")).atanh(), n2i(hexf64!("-0x1.11d6fd2b8fe1fp-343"), hexf64!("-0x1.30d1074dc059ep-868")));
    assert_eq!(n2i(hexf64!("-0x1.efde0d25f9c44p-67"), hexf64!("-0x1.2a278e6c91f21p-838")).atanh(), n2i(hexf64!("-0x1.efde0d25f9c45p-67"), hexf64!("-0x1.2a278e6c91f21p-838")));
    assert_eq!(n2i(hexf64!("0x1.2b060c8a4ba6ep-493"), hexf64!("0x1.633b2978352afp-407")).atanh(), n2i(hexf64!("0x1.2b060c8a4ba6ep-493"), hexf64!("0x1.633b2978352b0p-407")));
    assert_eq!(n2i(hexf64!("-0x1.9c5fc40761841p-303"), hexf64!("-0x1.dec661df94dabp-510")).atanh(), n2i(hexf64!("-0x1.9c5fc40761842p-303"), hexf64!("-0x1.dec661df94dabp-510")));
    assert_eq!(n2i(hexf64!("-0x1.56df81b91c381p-43"), hexf64!("0x1.98dc940c3ae1ep-564")).atanh(), n2i(hexf64!("-0x1.56df81b91c382p-43"), hexf64!("0x1.98dc940c3ae1fp-564")));
    assert_eq!(n2i(hexf64!("-0x1.3929e7122ce96p-326"), hexf64!("0x1.451605cb3a73bp-853")).atanh(), n2i(hexf64!("-0x1.3929e7122ce97p-326"), hexf64!("0x1.451605cb3a73cp-853")));
    assert_eq!(n2i(hexf64!("-0x1.a034fa1ee4476p-230"), hexf64!("-0x1.1efaa10962372p-519")).atanh(), n2i(hexf64!("-0x1.a034fa1ee4477p-230"), hexf64!("-0x1.1efaa10962372p-519")));
    assert_eq!(n2i(hexf64!("-0x1.4e54c309c46f8p-480"), hexf64!("0x1.28fd3055907f3p-685")).atanh(), n2i(hexf64!("-0x1.4e54c309c46f9p-480"), hexf64!("0x1.28fd3055907f4p-685")));
    assert_eq!(n2i(hexf64!("-0x1.13b101113d36fp-807"), hexf64!("0x1.c53e9ba64fadfp-768")).atanh(), n2i(hexf64!("-0x1.13b101113d370p-807"), hexf64!("0x1.c53e9ba64fae0p-768")));
    assert_eq!(n2i(hexf64!("-0x1.2392d35ee9b74p-210"), hexf64!("-0x1.4b35284c1064bp-548")).atanh(), n2i(hexf64!("-0x1.2392d35ee9b75p-210"), hexf64!("-0x1.4b35284c1064bp-548")));
    assert_eq!(n2i(hexf64!("-0x1.ae295c6cffac1p-247"), hexf64!("-0x1.57c346b295c33p-911")).atanh(), n2i(hexf64!("-0x1.ae295c6cffac2p-247"), hexf64!("-0x1.57c346b295c33p-911")));
    assert_eq!(n2i(hexf64!("0x1.6938cc5ee183ap-692"), hexf64!("0x1.7ef4b0758702dp-661")).atanh(), n2i(hexf64!("0x1.6938cc5ee183ap-692"), hexf64!("0x1.7ef4b0758702ep-661")));
    assert_eq!(n2i(hexf64!("-0x1.b459af91d9283p-559"), hexf64!("0x1.3f39248da0a27p-301")).atanh(), n2i(hexf64!("-0x1.b459af91d9284p-559"), hexf64!("0x1.3f39248da0a28p-301")));
    assert_eq!(n2i(hexf64!("-0x1.57bce16d0a1d4p-513"), hexf64!("0x1.5dbb6adfb81fdp-1019")).atanh(), n2i(hexf64!("-0x1.57bce16d0a1d5p-513"), hexf64!("0x1.5dbb6adfb81fep-1019")));
}

#[test]
fn fi_lib_unary_functions() {
    assert_eq!(n2i(hexf64!("0x1.89694dd6d675ep-261"), hexf64!("0x1.5ba589837c966p+95")).sqrt(), n2i(hexf64!("0x1.c0ce46227e574p-131"), hexf64!("0x1.a5e5061f52645p+47")));
    assert_eq!(n2i(hexf64!("0x1.3109f10d8a44ap-515"), hexf64!("0x1.9981bc265797dp-222")).sqrt(), n2i(hexf64!("0x1.8b322d746dd92p-258"), hexf64!("0x1.43c7c708713a9p-111")));
    assert_eq!(n2i(hexf64!("0x1.21fe595436609p-237"), hexf64!("0x1.95f14b9ba7449p+236")).sqrt(), n2i(hexf64!("0x1.8153a5585df9bp-119"), hexf64!("0x1.425e465111a84p+118")));
    assert_eq!(n2i(hexf64!("0x1.1c55b51bf7d27p-489"), hexf64!("0x1.b3e59df05d8a3p-451")).sqrt(), n2i(hexf64!("0x1.7d8c77fc21799p-245"), hexf64!("0x1.d86b2273c5e01p-226")));
    assert_eq!(n2i(hexf64!("0x1.394270bbcba7ep+196"), hexf64!("0x1.092ede17e8b48p+227")).sqrt(), n2i(hexf64!("0x1.1b2facb63d9c6p+98"), hexf64!("0x1.7079918cd6185p+113")));
    assert_eq!(n2i(hexf64!("0x1.a800284a0e694p-42"), hexf64!("0x1.c36e2ca1ca0adp+277")).sqrt(), n2i(hexf64!("0x1.4975dcfe90520p-21"), hexf64!("0x1.e0c3235697cf6p+138")));
    assert_eq!(n2i(hexf64!("0x1.3c84e4f9c80cep-476"), hexf64!("0x1.23d287387686bp+199")).sqrt(), n2i(hexf64!("0x1.1ca7df0d1338cp-238"), hexf64!("0x1.828a33acd2ffep+99")));
    assert_eq!(n2i(hexf64!("0x1.083a5b6db26a9p-373"), hexf64!("0x1.76e7a53c8c81ap+141")).sqrt(), n2i(hexf64!("0x1.6fcf8a53263bdp-187"), hexf64!("0x1.b61f599c226f6p+70")));
    assert_eq!(n2i(hexf64!("0x1.2c18feebcaeaep-768"), hexf64!("0x1.c369e759df5e3p-328")).sqrt(), n2i(hexf64!("0x1.152c585eddb6ap-384"), hexf64!("0x1.53f1a81caa4a0p-164")));
    assert_eq!(n2i(hexf64!("0x1.3653a458674fcp-578"), hexf64!("0x1.4fb657b5aa89fp-131")).sqrt(), n2i(hexf64!("0x1.19db8426bc0bdp-289"), hexf64!("0x1.9e96fbc95e70dp-66")));
    assert_eq!(n2i(hexf64!("0x1.4e68960e85562p+117"), hexf64!("0x1.dfbf07296e0d5p+277")).sqrt(), n2i(hexf64!("0x1.9dc8b22366176p+58"), hexf64!("0x1.ef9c5cc91699ap+138")));
    assert_eq!(n2i(hexf64!("0x1.9b6f2e2aaca77p-99"), hexf64!("0x1.d0460177b1553p+204")).sqrt(), n2i(hexf64!("0x1.caf89228c2349p-50"), hexf64!("0x1.58c0892d42f3ep+102")));
    assert_eq!(n2i(hexf64!("0x1.2f500c5cdbdeap+6"), hexf64!("0x1.f4217679238f6p+595")).sqrt(), n2i(hexf64!("0x1.16a76c242f69fp+3"), hexf64!("0x1.fa07d2a534cf1p+297")));
    assert_eq!(n2i(hexf64!("0x1.4712c6d9e227cp+21"), hexf64!("0x1.8d276f38cd260p+629")).sqrt(), n2i(hexf64!("0x1.99387d9205e4cp+10"), hexf64!("0x1.c2ef801ba24d3p+314")));
    assert_eq!(n2i(hexf64!("0x1.19ca879c7c187p-175"), hexf64!("0x1.6f80e21866c06p-22")).sqrt(), n2i(hexf64!("0x1.7bd69462cdad2p-88"), hexf64!("0x1.32b9dfd924778p-11")));
    assert_eq!(n2i(hexf64!("0x1.92781ef99ab46p-559"), hexf64!("0x1.cce4ab046843dp+923")).sqrt(), n2i(hexf64!("0x1.c5f168118c2b1p-280"), hexf64!("0x1.e5c6656b17688p+461")));
    assert_eq!(n2i(hexf64!("0x1.6ab931cee2fd4p-467"), hexf64!("0x1.95cf42aa171cdp-160")).sqrt(), n2i(hexf64!("0x1.aef2280db324cp-234"), hexf64!("0x1.4250c275a7b2bp-80")));
    assert_eq!(n2i(hexf64!("0x1.1c1680a0c0b6cp-530"), hexf64!("0x1.72b9a0e0a4b86p+325")).sqrt(), n2i(hexf64!("0x1.0dadb347d442cp-265"), hexf64!("0x1.b3ac5fd5bc6d4p+162")));
    assert_eq!(n2i(hexf64!("0x1.36e874875bcc2p+146"), hexf64!("0x1.b33cc462bc32dp+197")).sqrt(), n2i(hexf64!("0x1.1a1f10af59132p+73"), hexf64!("0x1.d80f99fc38774p+98")));
    assert_eq!(n2i(hexf64!("0x1.27e9687937dd0p-326"), hexf64!("0x1.11b85141b78f6p-240")).sqrt(), n2i(hexf64!("0x1.133bbe271b45fp-163"), hexf64!("0x1.08b63617a4210p-120")));
    assert_eq!(n2i(hexf64!("0x1.419bd3b802b61p+363"), hexf64!("0x1.f33b09f0c558bp+527")).sqrt(), n2i(hexf64!("0x1.95c9a66e614f0p+181"), hexf64!("0x1.f99333020215ap+263")));
    assert_eq!(n2i(hexf64!("0x1.a211040e355aep-106"), hexf64!("0x1.9eb2e04c9c099p+137")).sqrt(), n2i(hexf64!("0x1.4725936450707p-53"), hexf64!("0x1.ccc9c68e6b873p+68")));
    assert_eq!(n2i(hexf64!("0x1.c5d872ac1544dp-354"), hexf64!("0x1.92efd09488689p-76")).sqrt(), n2i(hexf64!("0x1.54dbc003c350fp-177"), hexf64!("0x1.412c2d0ac617dp-38")));
    assert_eq!(n2i(hexf64!("0x1.88217909d04f2p-250"), hexf64!("0x1.719ba76049d42p-33")).sqrt(), n2i(hexf64!("0x1.3cd62fbb21703p-125"), hexf64!("0x1.b30435c147e51p-17")));
    assert_eq!(n2i(hexf64!("0x1.7ebe381f4672fp-501"), hexf64!("0x1.2493763542c73p+74")).sqrt(), n2i(hexf64!("0x1.baadbfcd8a3fep-251"), hexf64!("0x1.11ad7b2551299p+37")));
    assert_eq!(n2i(hexf64!("0x1.778fc6fd65a71p-213"), hexf64!("0x1.cfe27bb53debbp+192")).sqrt(), n2i(hexf64!("0x1.b6818c65b2cb8p-107"), hexf64!("0x1.589b93c7cc280p+96")));
    assert_eq!(n2i(hexf64!("0x1.e139dd116f868p-688"), hexf64!("0x1.a4a8a68dad1fap-145")).sqrt(), n2i(hexf64!("0x1.5efd65c23f515p-344"), hexf64!("0x1.d0166d0139468p-73")));
    assert_eq!(n2i(hexf64!("0x1.7064732544856p+193"), hexf64!("0x1.3544c696554d2p+261")).sqrt(), n2i(hexf64!("0x1.b24cebb3d4b84p+96"), hexf64!("0x1.8ded33641a18dp+130")));
    assert_eq!(n2i(hexf64!("0x1.671190bb5e2f2p-338"), hexf64!("0x1.18c6f488cbc30p+590")).sqrt(), n2i(hexf64!("0x1.2f2f89ad0042ep-169"), hexf64!("0x1.0c1a3e03e351bp+295")));
    assert_eq!(n2i(hexf64!("0x1.f80bd5021bfd1p-177"), hexf64!("0x1.40b109f7dabe9p-149")).sqrt(), n2i(hexf64!("0x1.fc01ee5cd905fp-89"), hexf64!("0x1.95356c567e5d8p-75")));
    assert_eq!(n2i(hexf64!("-0x1.4c8993b11d519p-149"), hexf64!("-0x1.b1d8f24f24de3p-941")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.aff52102ccd1ep-298")));
    assert_eq!(n2i(hexf64!("-0x1.9ee1a9db994f5p-436"), hexf64!("-0x1.b6451c0720bfbp-622")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.502f54272449ep-871")));
    assert_eq!(n2i(hexf64!("-0x1.59415fcfbff18p+6"), hexf64!("-0x1.1b0be7ac0af65p-959")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.d1a144efbeb44p+12")));
    assert_eq!(n2i(hexf64!("-0x1.975299ccb0e08p-372"), hexf64!("0x1.77d8fa6b68b60p-585")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.440bdfd7def8cp-743")));
    assert_eq!(n2i(hexf64!("-0x1.a8e9c46a3d769p-355"), hexf64!("-0x1.c3a9cd7025105p-564")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.60a3980b85d7cp-709")));
    assert_eq!(n2i(hexf64!("-0x1.9b66c64d19ee1p-329"), hexf64!("-0x1.0b8dbebdff270p-411")).sqr(), n2i(hexf64!("0x1.17a0fa5bfda4fp-822"), hexf64!("0x1.4a9194ff18274p-657")));
    assert_eq!(n2i(hexf64!("-0x1.32690aac2472dp-40"), hexf64!("-0x1.0e0dd7b9e7391p-789")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.6ebf489d48ca5p-80")));
    assert_eq!(n2i(hexf64!("-0x1.40516bc314cc9p-198"), hexf64!("-0x1.7ad0659980c2bp-203")).sqr(), n2i(hexf64!("0x1.18460ad749015p-405"), hexf64!("0x1.90cba74d12cf4p-396")));
    assert_eq!(n2i(hexf64!("0x1.e80ef8fd19ad4p-265"), hexf64!("0x1.ba9a1304c562dp-79")).sqr(), n2i(hexf64!("0x1.d13c8b128fdf5p-529"), hexf64!("0x1.7e9c3333ae604p-157")));
    assert_eq!(n2i(hexf64!("-0x1.764bf1b176ff7p-373"), hexf64!("-0x1.00bd1997cd82ep-915")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.11a0fe5d04eeep-745")));
    assert_eq!(n2i(hexf64!("-0x1.b28f9484ffbfcp-589"), hexf64!("0x1.c77c9276b791dp-13")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.953606a01baafp-25")));
    assert_eq!(n2i(hexf64!("-0x1.388c331648e3ep-333"), hexf64!("-0x1.65ed85df2d4b7p-576")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.7d96094e3c3d1p-666")));
    assert_eq!(n2i(hexf64!("-0x1.96433e013eda2p-935"), hexf64!("0x1.8620faa09eadbp-386")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.29443ff0807f8p-771")));
    assert_eq!(n2i(hexf64!("-0x1.ab077c8e23ef5p-491"), hexf64!("0x1.504d993745eafp-18")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.b9cbc9b69e7bfp-36")));
    assert_eq!(n2i(hexf64!("-0x1.06d7f9ae94dadp-301"), hexf64!("-0x1.9610758986a88p-835")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.0ddec946af730p-602")));
    assert_eq!(n2i(hexf64!("-0x1.af3b21180e563p-498"), hexf64!("-0x1.068b13da99666p-760")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.6b34138ba2d21p-995")));
    assert_eq!(n2i(hexf64!("-0x1.2789c2d583bcdp-568"), hexf64!("-0x1.f2bd89dad0665p-780")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.9fc9d1b0afc7cp-545"), hexf64!("0x1.580844b9dc45cp-780")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.8a11a53596037p-49"), hexf64!("0x1.b1e6b793078ddp-664")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.2f4d28e026bf5p-97")));
    assert_eq!(n2i(hexf64!("-0x1.425eef071014fp-121"), hexf64!("-0x1.bb2efb4f70837p-547")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.95f2f4822cd2bp-242")));
    assert_eq!(n2i(hexf64!("-0x1.8e96354bf7e11p-894"), hexf64!("-0x1.039e2518cf503p-1008")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("-0x1.7ba62e3fbdd83p-165"), hexf64!("0x1.069e434ee9e0fp-740")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.1982bc685f91bp-329")));
    assert_eq!(n2i(hexf64!("-0x1.e7802992ba99dp-775"), hexf64!("-0x1.7883a587654e5p-928")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x0.0000000000001p-1022")));
    assert_eq!(n2i(hexf64!("0x1.455801d3d2b63p-704"), hexf64!("0x1.d2648abc1e83dp-27")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.a8d91848c69f9p-53")));
    assert_eq!(n2i(hexf64!("-0x1.04be837a6f1f1p-375"), hexf64!("0x1.173ab0fec92afp-771")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.099388d81efaap-750")));
    assert_eq!(n2i(hexf64!("-0x1.7e13dbb66e5a3p-84"), hexf64!("-0x1.af23d175aa3d2p-538")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.1d1fa29f657dap-167")));
    assert_eq!(n2i(hexf64!("-0x1.07d4317cb3695p-274"), hexf64!("-0x1.ef8b7bcbab211p-495")).sqr(), n2i(hexf64!("0x1.df9e5a5ea5698p-989"), hexf64!("0x1.0fe5ad9038bc9p-548")));
    assert_eq!(n2i(hexf64!("-0x1.fb31317bb132bp-326"), hexf64!("-0x1.70170edbd047bp-875")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.f66df1b9f564ap-651")));
    assert_eq!(n2i(hexf64!("-0x1.8378f49913a88p-253"), hexf64!("0x1.53fab12968e9ap-607")).sqr(), n2i(hexf64!("0x0.0p+0"), hexf64!("0x1.253b765685531p-505")));
    assert_eq!(n2i(hexf64!("-0x1.79ca1af65e50dp-233"), hexf64!("-0x1.cd61131067370p-429")).sqr(), n2i(hexf64!("0x1.9fc361bc43cadp-857"), hexf64!("0x1.16c2717c18b6fp-465")));
}
