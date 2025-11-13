pub(crate) fn absolute(wait: i64) -> i64 {
    wait
}

pub(crate) fn relative(wait: i64) -> i64 {
    -wait
}

pub(crate) fn nanoseconds(nanos: i64) -> i64 {
    nanos / 100
}

pub(crate) fn microseconds(micros: i64) -> i64 {
    micros * nanoseconds(1000)
}

pub(crate) fn milliseconds(milli: i64) -> i64 {
    milli * microseconds(1000)
}

pub(crate) fn seconds(seconds: i64) -> i64 {
    seconds * milliseconds(1000)
}
