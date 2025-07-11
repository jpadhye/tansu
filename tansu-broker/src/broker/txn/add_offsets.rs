// Copyright ⓒ 2024 Peter Morgan <peter.james.morgan@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use tansu_sans_io::Body;
use tansu_sans_io::add_offsets_to_txn_response::AddOffsetsToTxnResponse;
use tansu_storage::Storage;
use tracing::debug;

use crate::Result;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AddOffsets<S> {
    storage: S,
}

impl<S> AddOffsets<S>
where
    S: Storage,
{
    pub fn with_storage(storage: S) -> Self {
        Self { storage }
    }

    pub async fn response(
        &mut self,
        transaction_id: &str,
        producer_id: i64,
        producer_epoch: i16,
        group_id: &str,
    ) -> Result<Body> {
        debug!(?transaction_id, ?producer_id, ?producer_epoch, ?group_id);

        self.storage
            .txn_add_offsets(transaction_id, producer_id, producer_epoch, group_id)
            .await
            .map_err(Into::into)
            .map(|error_code| {
                AddOffsetsToTxnResponse::default()
                    .throttle_time_ms(0)
                    .error_code(error_code.into())
                    .into()
            })
    }
}
