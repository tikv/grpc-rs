// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use super::*;

use std::time::Duration;

impl gpr_timespec {
    pub fn inf_future() -> gpr_timespec {
        unsafe { gpr_inf_future(gpr_clock_type::GPR_CLOCK_REALTIME) }
    }
}

impl From<Duration> for gpr_timespec {
    fn from(dur: Duration) -> gpr_timespec {
        gpr_timespec {
            tv_sec: dur.as_secs() as i64,
            tv_nsec: dur.subsec_nanos() as i32,
            clock_type: gpr_clock_type::GPR_TIMESPAN,
        }
    }
}
