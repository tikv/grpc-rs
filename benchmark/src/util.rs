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


use std::time::{Duration, Instant};
use std::{f64, mem};

use grpc_proto::testing::stats::HistogramData;
use grpc_sys;
use libc::{self, rusage};

pub struct CpuRecorder {
    usage_snap: rusage,
    last_reset_time: Instant,
}

impl CpuRecorder {
    pub fn new() -> CpuRecorder {
        let mut usage = new_rusage();
        unsafe { assert_eq!(libc::getrusage(libc::RUSAGE_SELF, &mut usage), 0) };
        CpuRecorder {
            usage_snap: usage,
            last_reset_time: Instant::now(),
        }
    }

    pub fn cpu_time(&mut self, reset: bool) -> (f64, f64, f64) {
        let now = Instant::now();
        let mut latest = new_rusage();
        unsafe { assert_eq!(libc::getrusage(libc::RUSAGE_SELF, &mut latest), 0) };

        let user_sec = latest.ru_utime.tv_sec - self.usage_snap.ru_utime.tv_sec;
        let user_usec = latest.ru_utime.tv_usec - self.usage_snap.ru_utime.tv_usec;
        let sys_sec = latest.ru_stime.tv_sec - self.usage_snap.ru_stime.tv_sec;
        let sys_usec = latest.ru_stime.tv_usec - self.usage_snap.ru_stime.tv_usec;
        let user_time = user_sec as f64 + user_usec as f64 / 1_000_000_000f64;
        let sys_time = sys_sec as f64 + sys_usec as f64 / 1_000_000_000f64;

        let elapsed = now - self.last_reset_time;
        let real_time = dur_to_secs(elapsed);

        if reset {
            self.usage_snap = latest;
            self.last_reset_time = now;
        }

        (real_time, user_time, sys_time)
    }
}

pub fn new_rusage() -> rusage {
    // hack: so we don't need to list all the field of rusage.
    // Note: compiler will complain if the size is not correct.
    let data = [0u8; 144];
    unsafe { mem::transmute(data) }
}

#[inline]
pub fn cpu_num_cores() -> usize {
    unsafe { grpc_sys::gpr_cpu_num_cores() }
}

#[inline]
pub fn dur_to_secs(dur: Duration) -> f64 {
    dur.as_secs() as f64 + dur.subsec_nanos() as f64 / 1_000_000_000f64
}

// Histogram accumulates values in the form of a histogram with
// exponentially increased bucket sizes. See also: grpc/support/histogram.h.
pub struct Histogram {
    count: u32,
    sum: f64,
    sum_of_squares: f64,
    min: f64,
    max: f64,
    buckets: Vec<u32>,
    one_on_log_multiplier: f64,
    max_val: f64,
}

impl Histogram {
    pub fn new(resolution: f64, max_val: f64) -> Histogram {
        let multiplier = 1.0 + resolution;
        let one_on_log_multiplier = 1.0 / multiplier.ln();

        let mut his = Histogram {
            count: 0,
            sum: 0f64,
            sum_of_squares: 0f64,
            min: f64::MAX,
            max: f64::MIN,
            buckets: vec![],
            one_on_log_multiplier: one_on_log_multiplier,
            max_val: max_val,
        };

        let bucket_size = his.find_bucket(max_val);
        his.buckets.resize(bucket_size + 1, 0);

        his
    }

    pub fn observe(&mut self, value: f64) {
        self.count += 1;
        self.sum += value;
        self.sum_of_squares += value * value;
        if self.min > value {
            self.min = value;
        }
        if self.max < value {
            self.max = value;
        }
        let bucket_idx = self.find_bucket(value);
        self.buckets[bucket_idx] += 1;
    }

    #[inline]
    fn find_bucket(&self, mut value: f64) -> usize {
        value = if value < 1.0 { 1.0 } else { value };
        if value > self.max_val {
            value = self.max_val;
        }
        (value.ln() * self.one_on_log_multiplier) as usize
    }

    pub fn report(&mut self, reset: bool) -> HistogramData {
        let mut data = HistogramData::new();
        data.set_count(self.count as f64);
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
