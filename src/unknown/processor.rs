//
// Sysinfo
//
// Copyright (c) 2015 Guillaume Gomez
//

use std::default::Default;

use LoadAvg;
use ProcessorExt;

/// Dummy struct that represents a processor.
pub struct Processor {}

impl Processor {
    pub(crate) fn new() -> Processor {
        Processor {}
    }
}

impl ProcessorExt for Processor {
    fn get_cpu_usage(&self) -> f32 {
        0.0
    }

    fn get_name(&self) -> &str {
        ""
    }

    fn get_frequency(&self) -> u64 {
        0
    }

    fn get_vendor_id(&self) -> &str {
        ""
    }

    fn get_brand(&self) -> &str {
        ""
    }
}

pub fn get_cpu_frequency() -> u64 {
    0
}

/// get_avg_load returns the system load average value.
pub fn get_avg_load() -> LoadAvg {
    LoadAvg::default()
}
