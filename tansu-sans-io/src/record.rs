//! Kafka Record
//!
//! Constructing a [`Record`] using a builder:
//!
//! ```
//! use bytes::Bytes;
//! use tansu_sans_io::record::{Header, Record};
//!
//! let record = Record::builder()
//!     .key(Some(Bytes::from_static(b"message")))
//!     .value(Some(Bytes::from_static(b"hello world!")))
//!     .header(
//!         Header::builder()
//!             .key(Bytes::from_static(b"format"))
//!             .value(Bytes::from_static(b"text")),
//!     )
//!     .header(
//!         Header::builder()
//!             .key(Bytes::from_static(b"importance"))
//!             .value(Bytes::from_static(b"high")),
//!     );
//!
//! ```
//!
//! An [`inflated`] Kafka record batch must be [`deflated`] before it can be sent using [`Compression`](super::Compression) with a [`ProduceRequest`](super::ProduceRequest):
//!
//! ```
//! # use tansu_sans_io::Error;
//! # pub fn main() -> Result<(), Error> {
//! #
//! use bytes::Bytes;
//! use tansu_sans_io::{
//!     ApiKey as _, BatchAttribute, Compression, Frame, Header, ProduceRequest,
//!     produce_request::{PartitionProduceData, TopicProduceData},
//!     record::{self, deflated, inflated},
//! };
//!
//! let batch = inflated::Batch::builder()
//!     .attributes(
//!         BatchAttribute::default()
//!             .compression(Compression::Lz4)
//!             .into(),
//!     )
//!     .record(record::Record::builder().value(Bytes::from_static(b"hello world!").into()))
//!     .build()
//!     .and_then(deflated::Batch::try_from)?;
//!
//! let produce_request = ProduceRequest::default()
//!     .topic_data(Some(
//!         [TopicProduceData::default()
//!             .name("test".into())
//!             .partition_data(Some(
//!                 [PartitionProduceData::default()
//!                     .index(0)
//!                     .records(Some(deflated::Frame {
//!                         batches: vec![batch],
//!                     }))]
//!                 .into(),
//!             ))]
//!         .into(),
//!     ))
//!     .into();
//!
//! let correlation_id = 12321;
//!
//! let request = Frame::request(
//!     Header::Request {
//!         api_key: ProduceRequest::KEY,
//!         api_version: 6,
//!         correlation_id,
//!         client_id: Some("tansu".into()),
//!     },
//!     produce_request,
//! )?;
//! # Ok(())
//! # }
//! ```
//!
//! A [`deflated`] Kafka record batch from a [`FetchRequest`](super::FetchRequest) must be [`inflated`] before it can be used:
//!
//! ```
//! # use tansu_sans_io::Error;
//! # pub fn main() -> Result<(), Error> {
//! #
//! use bytes::Bytes;
//! use tansu_sans_io::{ApiKey as _, FetchResponse, Frame, record::inflated};
//!
//! let api_key = FetchResponse::KEY;
//! let api_version = 16;
//!
//! let v = vec![
//!     0, 0, 0, 186, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 28, 205, 172, 195, 142, 19,
//!     71, 71, 182, 128, 13, 18, 65, 142, 210, 222, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//!     0, 0, 0, 0, 0, 0, 0, 1, 255, 255, 255, 255, 255, 255, 255, 255, 1, 0, 0, 0, 0, 74, 0, 0, 0,
//!     0, 0, 0, 0, 0, 0, 0, 0, 61, 255, 255, 255, 255, 2, 153, 143, 24, 144, 0, 0, 0, 0, 0, 0, 0,
//!     0, 1, 144, 238, 148, 84, 54, 0, 0, 1, 144, 238, 148, 84, 54, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
//!     0, 0, 0, 0, 0, 0, 0, 1, 22, 0, 0, 0, 1, 10, 112, 111, 105, 117, 121, 0, 3, 0, 13, 255, 255,
//!     255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
//!     13, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0,
//! ];
//!
//! let fetch_response = Frame::response_from_bytes(&v[..], api_key, api_version)
//!     .and_then(|message_frame| FetchResponse::try_from(message_frame.body))?;
//!
//! // first batch, from the first partition of the first topic
//! // in the response:
//!
//! let deflated = fetch_response
//!     .responses
//!     .as_ref()
//!     .and_then(|topics| topics.first())
//!     .and_then(|topic| topic.partitions.as_ref())
//!     .and_then(|partitions| partitions.first())
//!     .and_then(|partition| partition.records.as_ref())
//!     .map(|record_frame| record_frame.batches.as_slice())
//!     .and_then(|batches| batches.first())
//!     .expect("deflated batch");
//!
//! // we just have raw record data at this point:
//! assert_eq!(12, deflated.record_data.len());
//!
//! // inflate the batch:
//! let inflated = inflated::Batch::try_from(deflated)?;
//!
//! // first record in the inflated batch:
//! assert_eq!(
//!     Some(Bytes::from_static(b"poiuy")),
//!     inflated
//!         .records
//!         .first()
//!         .and_then(|first| first.value.clone())
//! );
//! # Ok(())
//! # }
//! ```
pub(crate) mod codec;
pub mod deflated;
pub mod header;
pub mod inflated;

use crate::{
    Result,
    primitive::{
        ByteSize,
        varint::{LongVarInt, VarInt},
    },
};
use bytes::Bytes;
use codec::{Octets, VarIntSequence};
pub use header::Header;
use serde::{
    Deserialize, Serialize, Serializer,
    ser::{self, SerializeSeq},
};

/// A Kafka API Record.
///
/// Note that is structure uses the same variant encoding as protobuf.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Record {
    #[serde(serialize_with = "VarInt::serialize")]
    #[serde(deserialize_with = "VarInt::deserialize")]
    pub length: i32,

    pub attributes: u8,

    #[serde(serialize_with = "LongVarInt::serialize")]
    #[serde(deserialize_with = "LongVarInt::deserialize")]
    pub timestamp_delta: i64,

    #[serde(serialize_with = "VarInt::serialize")]
    #[serde(deserialize_with = "VarInt::deserialize")]
    pub offset_delta: i32,

    #[serde(serialize_with = "Octets::serialize")]
    #[serde(deserialize_with = "Octets::deserialize")]
    pub key: Option<Bytes>,

    #[serde(serialize_with = "Octets::serialize")]
    #[serde(deserialize_with = "Octets::deserialize")]
    pub value: Option<Bytes>,

    #[serde(serialize_with = "VarIntSequence::<Header>::serialize")]
    #[serde(deserialize_with = "VarIntSequence::<Header>::deserialize")]
    pub headers: Vec<Header>,
}

impl Record {
    #[must_use]
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn key(&self) -> Option<Bytes> {
        self.key.clone()
    }

    pub fn value(&self) -> Option<Bytes> {
        self.value.clone()
    }

    pub fn is_tombstone(&self) -> bool {
        self.key.is_some() && self.value.is_none()
    }
}

impl TryFrom<Builder> for Record {
    type Error = crate::Error;

    fn try_from(builder: Builder) -> Result<Self, Self::Error> {
        Ok(Self {
            length: builder
                .size_in_bytes()
                .and_then(|sz| i32::try_from(sz).map_err(Into::into))?,
            attributes: builder.attributes,
            timestamp_delta: *builder.timestamp_delta,
            offset_delta: *builder.offset_delta,
            key: builder.key.into(),
            value: builder.value.into(),
            headers: builder.headers.0.into_iter().map(Into::into).collect(),
        })
    }
}

impl From<Record> for Builder {
    fn from(value: Record) -> Self {
        Self {
            attributes: value.attributes,
            timestamp_delta: value.timestamp_delta.into(),
            offset_delta: value.offset_delta.into(),
            key: value.key.into(),
            value: value.value.into(),
            headers: value.headers.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Builder {
    attributes: u8,
    timestamp_delta: LongVarInt,
    offset_delta: VarInt,
    key: Octets,
    value: Octets,
    headers: VarIntSequence<header::Builder>,
}

impl Serialize for Builder {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_seq(None).and_then(|mut s| {
            self.size_in_bytes()
                .map_err(|e| ser::Error::custom(e.to_string()))
                .map(|size| {
                    VarInt::try_from(size)
                        .map_err(|e| ser::Error::custom(e.to_string()))
                        .and_then(|v| s.serialize_element(&v))
                })
                .and(s.serialize_element(&self.attributes))
                .and(s.serialize_element(&self.timestamp_delta))
                .and(s.serialize_element(&self.offset_delta))
                .and(s.serialize_element(&self.key))
                .and(s.serialize_element(&self.value))
                .and(s.serialize_element(&self.headers))
                .and(s.end())
        })
    }
}

impl ByteSize for Builder {
    fn size_in_bytes(&self) -> Result<usize> {
        Ok(size_of::<u8>()
            + self.timestamp_delta.size_in_bytes()?
            + self.offset_delta.size_in_bytes()?
            + self.key.size_in_bytes()?
            + self.value.size_in_bytes()?
            + self.headers.size_in_bytes()?)
    }
}

impl Builder {
    #[must_use]
    pub fn attributes(self, attributes: u8) -> Self {
        Self { attributes, ..self }
    }

    #[must_use]
    pub fn timestamp_delta(self, timestamp_delta: i64) -> Self {
        Self {
            timestamp_delta: timestamp_delta.into(),
            ..self
        }
    }

    #[must_use]
    pub fn offset_delta(self, offset_delta: i32) -> Self {
        Self {
            offset_delta: offset_delta.into(),
            ..self
        }
    }

    #[must_use]
    pub fn key(self, key: Option<Bytes>) -> Self {
        Self {
            key: key.into(),
            ..self
        }
    }

    #[must_use]
    pub fn value(self, value: Option<Bytes>) -> Self {
        Self {
            value: value.into(),
            ..self
        }
    }

    #[must_use]
    pub fn header(mut self, header: header::Builder) -> Self {
        self.headers.0.extend(vec![header]);
        self
    }

    pub fn build(self) -> Result<Record> {
        Record::try_from(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Result, ser::Encoder};
    use codec::Sequence;
    use std::io::Cursor;

    #[test]
    fn bytes_size() -> Result<()> {
        assert_eq!(4, Octets::from(Some(vec![100, 101, 102])).size_in_bytes()?);
        Ok(())
    }

    #[test]
    fn record_size() -> Result<()> {
        assert_eq!(
            9,
            Record::builder()
                .value(Some(Bytes::from(vec![100, 101, 102])))
                .size_in_bytes()?
        );
        Ok(())
    }

    #[test]
    fn serialize_record_builder() -> Result<()> {
        let rb = Record::builder().value(Some(Bytes::from(vec![100, 101, 102])));

        let mut c = Cursor::new(vec![]);
        let mut e = Encoder::new(&mut c);
        rb.serialize(&mut e)?;

        assert_eq!(vec![18, 0, 0, 0, 1, 6, 100, 101, 102, 0], c.into_inner());
        Ok(())
    }

    #[test]
    fn try_from_record_builder() -> Result<()> {
        let record =
            Record::try_from(Record::builder().value(Some(Bytes::from(vec![100, 101, 102]))))?;
        assert_eq!(9, record.length);
        Ok(())
    }

    #[test]
    fn sequence_of_record_builder() -> Result<()> {
        let rb = Record::builder().value(Some(Bytes::from(vec![100, 101, 102])));
        let records = Sequence::from(vec![rb.clone()]);
        assert_eq!(14, records.size_in_bytes()?);

        let mut c = Cursor::new(vec![]);
        let mut e = Encoder::new(&mut c);
        records.serialize(&mut e)?;

        assert_eq!(
            vec![0, 0, 0, 1, 18, 0, 0, 0, 1, 6, 100, 101, 102, 0],
            c.into_inner()
        );

        Ok(())
    }

    #[test]
    fn crc_check() {
        use crc::CRC_32_ISCSI;
        use crc::Crc;

        let b = [
            0, 0, 0, 0, 0, 0, 0, 0, 1, 141, 116, 152, 137, 53, 0, 0, 1, 141, 116, 152, 137, 53, 0,
            0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 18, 0, 0, 0, 1, 6, 100, 101, 102, 0,
        ];

        let crc = Crc::<u32>::new(&CRC_32_ISCSI);
        let mut digester = crc.digest();
        digester.update(&b);
        assert_eq!(1_126_819_645, digester.finalize());
    }
}
