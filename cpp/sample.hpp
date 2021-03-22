// File: sample.hpp
// Project: cpp
// Created Date: 22/03/2021
// Author: Shun Suzuki
// -----
// Last Modified: 22/03/2021
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#pragma once

#include <cstdint>

class Sample {
 public:
  Sample(int32_t n);
  ~Sample();
  void print();

 private:
  int32_t _n;
};
