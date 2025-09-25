pub const DESCRIPTOR_ID: u8 = 0x52;

#[derive(Debug, Clone)]
pub struct StreamIdentifier {
    /// Identifies the component stream for associating it with a description given in a component descriptor.
    pub component_tag: u8,
}

impl StreamIdentifier {
    pub fn from_buf(buf: &[u8]) -> StreamIdentifier {
        let component_tag = buf[0];

        StreamIdentifier { component_tag }
    }
}
