use crate::{
    multiplexer::Multiplexer,
    types::{ErrorResponse, Request, Response},
};
use eth2::types::EthSpec;

impl<E: EthSpec> Multiplexer<E> {
    pub async fn handle_syncing(&self, request: Request) -> Result<Response, ErrorResponse> {
        // TODO: actually check EL status, maybe with a cache
        let (id, ()) = request.parse_as()?;
        Response::new(id, false)
    }
}