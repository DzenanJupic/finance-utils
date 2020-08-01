use crate::market_values::MarketValue;

pub fn simple_average<M: MarketValue>(instances: &[M]) -> M {
    if instances.len() == 0 { return M::from(0.0); } else if instances.len() == 1 { return instances[0]; }

    instances
        .iter()
        .skip(1)
        .fold(
            instances[0],
            |prev, &curr| prev + curr)
        .div(instances.len() as f64)
}

pub fn simple_moving_average<M: MarketValue>(instances: &[M], interval: usize) -> Vec<M> {
    if interval == 0 || instances.len() < interval { return Vec::with_capacity(0); }
    let mut averages = Vec::with_capacity(instances.len() - interval + 1);

    for i in interval..=instances.len() {
        let average = simple_average(&instances[i - interval..i]);
        averages.push(average);
    }
    averages
}

pub fn exponential_average_multiplier(smoothing: f64, interval: usize) -> f64 {
    smoothing / (interval as f64 + 1.0)
}

pub fn exponential_average<M: MarketValue>(instance: M, previous_ea: M, multiplier: f64) -> M {
    (instance - previous_ea) * multiplier + previous_ea
}

pub fn exponential_moving_average<M: MarketValue>(instances: &[M], smoothing: f64, interval: usize) -> Vec<M> {
    if interval == 0 || instances.len() <= interval { return Vec::with_capacity(0); }
    let mut averages = Vec::with_capacity(instances.len() - interval);

    let mut previous = simple_average(&instances[..interval]);
    let multiplier = exponential_average_multiplier(smoothing, interval);

    for i in interval..instances.len() {
        let average = exponential_average(instances[i], previous, multiplier);
        previous = average;
        averages.push(average);
    }
    averages
}

pub fn triangular_moving_average<M: MarketValue>(instances: &[M], interval: usize) -> Vec<M> {
    if interval == 0 || instances.len() < interval { return Vec::with_capacity(0); }
    let mut averages = Vec::with_capacity(instances.len() - interval + 1);

    // calculating the sma
    for i in interval..=instances.len() {
        let sa = simple_average(&instances[i - interval..i]);
        averages.push(sa);
    }
    // calculating the sma of the sma (tma)
    for i in interval..=averages.len() {
        let ta = simple_average(&averages[i - interval..i]);
        averages[i - interval] = ta;
    }

    if interval != 1 {
        averages.pop();
    }

    averages
}

// F takes the index of the current element
pub fn weighted_average<M: MarketValue, F: Fn(usize) -> f64>(instances: &[M], weight: F) -> M {
    if instances.len() == 0 { return M::from(0.0); } else if instances.len() == 1 { return instances[0] * weight(0); }

    instances
        .iter()
        .enumerate()
        .fold(
            M::from(0.0),
            |prev, (i, &curr)| {
                let weight = weight(i);
                prev + curr * weight
            },
        )
        .div(instances.len() as f64)
}

pub fn weighted_moving_average<M: MarketValue, F: Fn(usize) -> f64>(instances: &[M], weight: F, interval: usize) -> Vec<M> {
    if interval == 0 || instances.len() < interval { return Vec::with_capacity(0); }
    let mut averages = Vec::with_capacity(instances.len() - interval + 1);

    for i in interval..=instances.len() {
        let average = weighted_average(&instances[i - interval..i], &weight);
        averages.push(average);
    }
    averages
}
