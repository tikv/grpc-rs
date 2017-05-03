// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.


use std::f64;

use grpc_proto::testing::stats::HistogramData;

// Histogram accumulates values in the form of a histogram with
// exponentially increased bucket sizes. See also: grpc/support/histogram.h.
pub struct Histogram {
    count: u64,
    sum: f64,
    sum_of_squares: f64,
    min: f64,
    max: f64,
    buckets: Vec<u64>,
    multiplier: f64,
    one_on_log_multiplier: f64,
    max_val: f64,
}

impl Histogram {
    pub fn new(resolution: f64, max_val: f64) -> Histogram {
        let multiplier = 1.0 + resolution;
        let one_on_log_multiplier = 1.0 / multiplier.log();
        let bucket_size = (max_val.log() * one_on_log_multiplier) as usize;
        
        Histogram {
            count: 0,
            sum: 0f64,
            sum_of_squares: 0f64,
            min: f64::MAX,
            max: f64::MIN,
            buckets: vec![0; bucket_size],
            multiplier: multiplier,
            one_on_log_multiplier: one_on_log_multiplier,
            max_val: max_val,
        }
    }

    pub fn observe(&mut self, value: f64) {
        self.count += 1;
        self.sum += value;
        self.sum_of_squares += value * value;
        self.min = cmp::min(self.min, value);
        self.max = cmp::max(self.max, value);
        let bucket_idx = self.find_bucket(value);
        self.buckets[bucket_idx] += 1;
    }

    #[inline]
    fn find_bucket(&self, mut value: f64) -> usize {
        value = cmp::max(value, 1.0);
        value = cmp::min(value, self.max_val);
        (value.log() * self.one_on_log_multiplier) as usize
    }

    pub fn report(&mut self, reset: bool) -> HistogramData {
        let mut data = HistogramData::new();
        data.set_count(self.count);
        data.set_sum(self.sum);
        data.set_sum_of_squares(self.sum_of_squares);
        data.set_min_seen(self.min);
        data.set_max_seen(self.max);
        data.set_bucket(self.buckets.clone());

        if reset {
            self.clear();
        }

        data
    }

    fn clear(&mut self) {
        self.count = 0;
        self.sum = 0f64;
        self.sum_of_squares = 0f64;
        self.min = f64::MAX;
        self.max = f64::MIN;
        for b in &mut self.buckets {
            *b = 0;
        }
    }
}
