use failure;

pub(crate) enum Response {
    Connect {},
}

impl Response {
    pub(super) fn parse(buf: &[u8]) -> Result<Self, failure::Error> {
        let r = Response {};
        Ok(r)
    }
}
