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
use std::time::{Duration, Instant};

use grpc_proto::testing::stats::HistogramData;
use grpc_sys;

#[path = "../../examples/log_util.rs"]
pub mod log_util;

pub struct Sample {
    pub real_time: f64,
    pub user_time: f64,
    pub sys_time: f64,
    pub total_cpu: u64,
    pub idle_cpu: u64,
}

pub struct CpuRecorder {
    user_time: f64,
    sys_time: f64,
    total_cpu: u64,
    idle_cpu: u64,
    last_reset_time: Instant,
}

impl CpuRecorder {
    pub fn new() -> CpuRecorder {
        let (total_cpu, idle_cpu) = sys_util::get_cpu_usage();
        let (sys_time, user_time) = sys_util::get_resource_usage();
        let last_reset_time = Instant::now();

        CpuRecorder {
            user_time,
            sys_time,
            total_cpu,
            idle_cpu,
            last_reset_time,
        }
    }

    pub fn cpu_time(&mut self, reset: bool) -> Sample {
        let now = Instant::now();
        let (total_cpu, idle_cpu) = sys_util::get_cpu_usage();
        let (sys_time, user_time) = sys_util::get_resource_usage();

        let total_cpu_diff = total_cpu - self.total_cpu;
        let idle_cpu_diff = idle_cpu - self.idle_cpu;

        let sys_time_diff = sys_time - self.sys_time;
        let user_time_diff = user_time - self.user_time;

        let elapsed = now - self.last_reset_time;
        let real_time = dur_to_secs(elapsed);

        if reset {
            self.user_time = user_time;
            self.sys_time = sys_time;
            self.last_reset_time = now;
            self.total_cpu = total_cpu;
            self.idle_cpu = idle_cpu;
        }

        Sample {
            real_time,
            user_time: user_time_diff,
            sys_time: sys_time_diff,
            total_cpu: total_cpu_diff,
            idle_cpu: idle_cpu_diff,
        }
    }
}

#[cfg(target_os = "linux")]
mod sys_util {
    use std::fs::File;
    use std::io::Read;
    use std::mem;

    use libc::{self, timeval};

    pub fn get_resource_usage() -> (f64, f64) {
        fn timeval_to_seconds(tv: &timeval) -> f64 {
            tv.tv_sec as f64 + tv.tv_usec as f64 * 10e-6
        }

        unsafe {
            let mut usage = mem::zeroed();
            assert_eq!(libc::getrusage(libc::RUSAGE_SELF, &mut usage), 0);
            (
                timeval_to_seconds(&usage.ru_stime),
                timeval_to_seconds(&usage.ru_utime),
            )
        }
    }

    pub fn get_cpu_usage() -> (u64, u64) {
        let mut f = File::open("/proc/stat").unwrap();
        let mut usages = String::default();
        let (mut total_usage, mut idle_usage) = (0, 0);
        f.read_to_string(&mut usages).unwrap();
        for (idx, usage) in usages[5..].split_whitespace().take(10).enumerate() {
            total_usage += usage.parse::<u64>().unwrap();
            if idx == 3 {
                idle_usage = usage.parse::<u64>().unwrap();
            }
        }
        (total_usage, idle_usage)
    }
}

#[cfg(not(target_os = "linux"))]
mod sys_util {
    pub fn get_resource_usage() -> (f64, f64) {
        (0f64, 0f64)
    }

    pub fn get_cpu_usage() -> (u64, u64) {
        (0, 0)
    }
}

#[inline]
pub fn cpu_num_cores() -> usize {
    unsafe { grpc_sys::gpr_cpu_num_cores() as usize }
}

#[inline]
pub fn dur_to_secs(dur: Duration) -> f64 {
    dur.as_secs() as f64 + f64::from(dur.subsec_nanos()) / 1_000_000_000f64
}

#[inline]
pub fn dur_to_nanos(dur: Duration) -> f64 {
    dur.as_secs() as f64 * 1_000_000_000f64 + f64::from(dur.subsec_nanos())
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
            one_on_log_multiplier,
            max_val,
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
        let mut data = HistogramData::default();
        data.set_count(f64::from(self.count));
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
