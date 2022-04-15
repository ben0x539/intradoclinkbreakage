pub trait Tr {
    /// [`cr::Cr`], [`cr::Cr` inline link][cr::Cr], [`cr::Cr` other link]!
    ///
    /// [`cr::Cr`] after first line
    ///
    /// [`cr::Cr` other link]: cr::Cr
    fn t(&self);
}

pub struct TrS;
impl Tr for TrS {
    fn t(&self) {}
}
