use crate::{Dimension, QuantityNum};

pub type Length<V> = L1<V>;
pub struct L1<V>
where V: QuantityNum {
    value: V
}

impl<V: QuantityNum> Dimension for L1<V> {
    type ValueType = V;

    fn get_raw(&self) -> &Self::ValueType {
        &self.value
    }
}
