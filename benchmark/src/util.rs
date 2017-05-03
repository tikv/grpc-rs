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


use std::time::Instant;

use libc::rusage;

pub struct CpuRecorder {
    usage_snap: rusage,
    last_reset_time: Instant,
}

impl CpuRecorder {
    pub fn new() -> CpuRecorder {
        let mut usage = rusage::default();
        assert_eq!(libc::getrusage(libc::RUSAGE_SELF, &mut usage), 0);
        CpuRecorder {
            usage_snap: usage,
            last_reset_time: Instant::now(),
        }
    }

    pub fn cpu_time(&mut self, reset: bool) -> (f64, f64, f64) {
        let now = Instant::now();
        let mut latest = rusage::default();
        assert_eq!(libc::getrusage(libc::RUSAGE_SELF, &mut usage), 0);

        let user_sec = latest.ru_utime.tv_sec - self.usage_snap.ru_utime.tv_sec;
        let user_usec = latest.ru_utime.tv_usec - self.usage_snap.ru_utime.tv_usec;
        let sys_sec = latest.ru_stime.tv_sec - self.usage_snap.ru_stime.tv_sec;
        let sys_usec = latest.ru_stime.tv_usec - self.usage_snap.ru_stime.tv_usec;
        let user_time = user_sec as f64 + user_usec as f64 / 1_000_000_000f64;
        let sys_time = sys_sec as f64 + sys_usec as f64 / 1_000_000_000f64;

        let elapsed = now - self.last_reset_time;
        let real_time = elapsed.as_sec() as f64 + elapsed.as_subsec() as f64 / 1_000_000_000f64;

        if reset {
            self.usage_snap = latest;
            self.last_reset_time = now;
        }

        (real_time, user_time, sys_time)
    }
}
