#[derive(Clone)]
pub struct Observability<L, M> {
    pub logger: L,
    pub metrics: M,
}

impl<L, M> Observability<L, M> {
    pub fn new(logger: L, metrics: M) -> Self {
        Self {
            logger,
            metrics,
        }
    }
}