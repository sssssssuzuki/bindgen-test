// File: sample.cpp
// Project: cpp
// Created Date: 22/03/2021
// Author: Shun Suzuki
// -----
// Last Modified: 22/03/2021
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#include "sample.hpp"

#include <iostream>

Sample::Sample(int32_t n) : _n(n) {}
Sample::~Sample() = default;
void Sample::print() { std::cout << "Called from C++: " << _n << std::endl; }
