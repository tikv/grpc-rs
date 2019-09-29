use super::*;

use std::time::Duration;

impl gpr_timespec {
    pub fn inf_future() -> gpr_timespec {
        unsafe { gpr_inf_future(gpr_clock_type::GPR_CLOCK_REALTIME) }
    }
}

impl Copy for gpr_timespec {}

impl Clone for gpr_timespec {
    fn clone(&self) -> Self {
        gpr_timespec {
            tv_sec: self.tv_sec,
            tv_nsec: self.tv_nsec,
            clock_type: self.clock_type,
        }
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
