use std::fmt;
use std::time::{Duration, Instant};
pub use tree_buf::prelude::*;
pub use tree_buf::{encode_options, options, Decodable, Encodable};

pub type Timer = Stat<Duration>;
pub type Sizer = Stat<usize>;

pub fn time_it<T>(
    op: &'static str,
    stats: &mut Timer,
    mut f: impl FnMut() -> T,
    count: usize,
) -> T {
    // Warmup
    let start = Instant::now();
    while Instant::now() - start < Duration::from_secs(5) {
        f();
    }

    let mut min_time = Duration::from_secs(std::u64::MAX);
    let mut counter = count;
    let t = loop {
        let start = Instant::now();
        let t = f();
        let end = Instant::now();
        let time = end - start;
        if time < min_time {
            min_time = time;
            counter = count;
        }
        // t is intentionally accessible here to not count cost of drop
        {
            counter -= 1;
            if counter == 0 {
                break t;
            }
        }
    };
    stats.push(op, min_time);
    t
}

pub struct Stats {
    pub decode: Timer,
    pub encode: Timer,
    pub size: Sizer,
    pub count: usize,
}

impl Stats {
    pub fn new(count: usize) -> Self {
        Self {
            decode: Timer::new("Decode CPU Time"),
            encode: Timer::new("Encode CPU Time"),
            size: Sizer::new("Size in Bytes"),
            count,
        }
    }

    pub fn profile<T: PartialEq + fmt::Debug>(
        &mut self,
        key: &'static str,
        expected_size: usize,
        data: &T,
        mut encode: impl FnMut(&T) -> Vec<u8>,
        mut decode: impl FnMut(&[u8]) -> T,
    ) -> Vec<u8> {
        let bytes = time_it(key, &mut self.encode, || encode(data), self.count);
        // FIXME: Sizes in Tree-Buf aren't deterministic. In fact, they can vary a lot.
        // This can depend on, for example, iteration of a HashMap order
        // TODO: Have a size checking type that allows for tuples of possible sizes
        //assert_eq!(expected_size, bytes.len());
        self.size.push(key, bytes.len());
        let _result = time_it(key, &mut self.decode, || decode(&bytes), self.count);
        // TODO: Re-enable this. It depends on whether lossy compression is allowed
        //assert_eq!(data, &result);
        bytes
    }
}

#[derive(Default)]
pub struct Stat<T> {
    header: &'static str,
    kvps: Vec<(&'static str, T)>,
}

impl<T> Stat<T> {
    pub fn push(&mut self, key: &'static str, value: T) {
        self.kvps.push((key, value));
    }
    pub fn new(header: &'static str) -> Self {
        Self {
            header,
            kvps: Vec::with_capacity(16),
        }
    }
}

struct Padding(usize);

impl fmt::Display for Padding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.0 {
            write!(f, " ")?;
        }
        Ok(())
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut round_trip = Timer::new("Round Trip CPU Time");
        //assert_eq!(self.decode.kvps.len(), self.encode.kvps.len());
        for (key, time) in self.decode.kvps.iter() {
            if let Some((_, v)) = self.encode.kvps.iter().find(|(k, _)| k == key) {
                let time = *time + *v;
                round_trip.push(key, time)
            }
        }

        writeln!(f, "{}", self.size)?;
        writeln!(f, "{}", round_trip)?;
        writeln!(f, "{}", self.decode)?;
        writeln!(f, "{}", self.encode)?;
        Ok(())
    }
}

impl<T: fmt::Debug + Ord + Clone> fmt::Display for Stat<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Sort by the value of the stats
        let mut kvps = self.kvps.clone();
        kvps.sort_by(|l, r| l.1.cmp(&r.1));

        // Format the values
        let kvps = kvps
            .into_iter()
            .map(|(k, v)| (k, format!("{:?}", v)))
            .collect::<Vec<_>>();

        // Get the longest kvp pair
        let max_len_k = kvps.iter().map(|(k, _)| k.chars().count()).max();
        let max_len_v = kvps.iter().map(|(_, v)| v.chars().count()).max();

        // Encode the header
        writeln!(f, "")?;
        writeln!(f, "* {}", self.header)?;

        if let (Some(max_len_k), Some(max_len_v)) = (max_len_k, max_len_v) {
            for (k, v) in kvps.into_iter() {
                let padding_k = Padding(max_len_k - k.chars().count());
                let padding_v = Padding(max_len_v - v.chars().count());
                writeln!(f, "    {}: {}{} {}", k, padding_k, padding_v, v)?;
            }
        }
        Ok(())
    }
}
