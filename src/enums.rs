use std::fmt;

pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeUnit::Seconds => {
                write!(f, "Seconds")
            }
            TimeUnit::Minutes => {
                write!(f, "Minutes")
            }
            TimeUnit::Hours => {
                write!(f, "Hours")
            }
            TimeUnit::Days => {
                write!(f, "Days")
            }
        }
    }
}

pub enum RoughTime {
    InThePast(u32, u32),
    JustNow,
    InTheFuture(u32, u32),
}

impl fmt::Display for RoughTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoughTime::InThePast(a, b) => {
                write!(f, "InThePast ({}, {})", a, b)
            }
            RoughTime::JustNow => {
                write!(f, "JustNow")
            }
            RoughTime::InTheFuture(a, b) => {
                write!(f, "InTheFuture ({}, {})", a, b)
            }
        }
    }
}

pub enum Cooltime {
    SomeTime(TimeUnit, u32),
    LastTime(TimeUnit, u32),
    NextTime(TimeUnit, u32),
}

impl fmt::Display for Cooltime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cooltime::SomeTime(a, b) => {
                write!(f, "SomeTime ({}, {})", a, b)
            }
            Cooltime::LastTime(a, b) => {
                write!(f, "LastTime ({}, {})", a, b)
            }
            Cooltime::NextTime(a, b) => {
                write!(f, "LastTime ({}, {})", a, b)
            }
        }
    }
}
