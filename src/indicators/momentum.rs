use crate::market_values::MarketValue;

pub fn momentum<M: MarketValue>(first: M, last: M) -> M {
    last - first
}

pub fn moving_momentum<M: MarketValue>(instances: &[M], interval: usize) -> Vec<M> {
    if interval == 0 || instances.len() < interval { return Vec::with_capacity(0); }
    let mut momentum_vec = Vec::with_capacity(instances.len() - interval + 1);

    for i in interval..=instances.len() {
        let m = momentum(instances[i - interval], instances[i - 1]);
        momentum_vec.push(m);
    }
    momentum_vec
}
