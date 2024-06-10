use crate::buffer::ByteBuf;

pub trait FromNetwork<T> {
    fn from_network(buf: &mut ByteBuf) -> T;
}

pub trait ToNetwork<T> {
    fn to_network(&self, buf: &mut ByteBuf);
}
