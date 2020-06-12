



/// Functionality used by GenericJoin to extend prefixes with new attributes.
///
/// These methods are used in `GenericJoin`'s `extend` method, and may not be broadly useful elsewhere.
pub trait StreamPrefixExtender<G: Scope, W: Data> {
    /// The type of data to extend.
    type Prefix: Data;
    /// The type of the extentions.
    type Extension: Data;
    /// Updates each prefix with an upper bound on the number of extensions for this relation.
    fn count(&self, Stream<G, (Self::Prefix, u64, u64, W)>, u64) -> Stream<G, (Self::Prefix, u64, u64, W)>;
    /// Proposes each extension from this relation.
    fn propose(&self, Stream<G, (Self::Prefix, W)>) -> Stream<G, (Self::Prefix, Vec<Self::Extension>, W)>;
    /// Restricts proposals by those this relation would propose.
    fn intersect(&self, Stream<G, (Self::Prefix, Vec<Self::Extension>, W)>) -> Stream<G, (Self::Prefix, Vec<Self::Extension>, W)>;
}