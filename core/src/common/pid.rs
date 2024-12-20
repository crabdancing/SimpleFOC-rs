use core::ops::Deref;
use esp_backtrace as _;

use embedded_time::{clock::Error as ClockError, Clock, Instant};

pub enum PIDError {
    Clock(ClockError),
    NegativeTimeDelta,
}

impl From<ClockError> for PIDError {
    fn from(value: ClockError) -> Self {
        Self::Clock(value)
    }
}

/// the p, i and d in pid
pub struct PIDParam(f32);
impl Deref for PIDParam {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct PID<C: Clock> {
    pub p: PIDParam,
    pub i: PIDParam,
    pub d: PIDParam,
    pub output_ramp: Option<f32>,
    pub upper_limit: f32,
    lookback: PIDLookBack<C>,
}

struct PIDLookBack<C: Clock> {
    prev_error: f32,
    prev_output: f32,
    prev_integral: PIDParam,
    prev_timestamp: Instant<C>,
}

impl<C: Clock> PIDLookBack<C> {
    fn new(clock: &C) -> Self {
        Self {
            prev_error: 0.0,
            prev_output: 0.0,
            prev_integral: PIDParam(0.0),
            prev_timestamp: clock.try_now().unwrap(),
        }
    }

    fn reset(&mut self) {
        self.prev_error = 0.0;
        self.prev_output = 0.0;
        self.prev_integral = PIDParam(0.0);
    }
}

impl<C: Clock> PID<C> {
    pub fn init(
        p: PIDParam,
        i: PIDParam,
        d: PIDParam,
        output_ramp: Option<f32>,
        upper_limit: f32,
        clock: &C,
    ) -> Self {
        Self {
            p,
            i,
            d,
            output_ramp,
            upper_limit,
            lookback: PIDLookBack::new(clock),
        }
    }
    pub fn run(&mut self, clock: &C, _error: f32) -> Result<f32, PIDError> {
        let now = clock.try_now()?;
        let _delta = now
            .checked_duration_since(&self.lookback.prev_timestamp)
            .ok_or(PIDError::NegativeTimeDelta)?;
        let _proportional = self.p.deref() * self.lookback.prev_error;

        // let integral = {
        //     let unclamped = self.lookback.prev_integral.deref()
        //         + self.i.deref() * (delta as f32) * 0.5 * (error + self.lookback.prev_error);
        //     unclamped.clamp(self.upper_limit, -self.upper_limit)
        // };

        // let derivitive = self.d.deref() * (error - self.lookback.prev_error) / (delta as f32);

        // let mut output = {
        //     let unclamped = proportional + integral + derivitive;
        //     unclamped.clamp(self.upper_limit, -self.upper_limit)
        // };
        // if let Some(ramp) = self.output_ramp {
        //     let output_rate = (output - self.lookback.prev_output) / (delta as f32);
        //     if output_rate > ramp {
        //         output = self.lookback.prev_output + ramp * (delta as f32);
        //     } else if output_rate < -ramp {
        //         output = self.lookback.prev_output - ramp * (delta as f32);
        //     }
        // }

        // self.lookback.prev_integral = PIDParam(integral);
        // self.lookback.prev_output = output;
        // self.lookback.prev_error = error;
        // self.lookback.prev_timestamp = now;
        // Ok(output)
        todo!("embeded clocks are hard. work out how to make cross-platform, hal'd timekeeping compatable with these functions");
    }
    pub fn reset(&mut self) {
        self.lookback.reset();
    }
}
