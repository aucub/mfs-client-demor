use rsocket_rust::utils::Writeable;
use bytes::{Buf, BufMut, Bytes, BytesMut};

const MAX_TOKEN_TAG_LEN: usize = 0x7F;

#[derive(Debug, Clone)]
pub struct TOKENMetadata {
    Authentication: Vec<String>,
}

pub struct TOKENMetadataBuilder {
    inner: TOKENMetadata,
}

impl TOKENMetadataBuilder {
    pub fn push_str(self, Authentication: &str) -> Self {
        self.push(String::from(Authentication))
    }
    pub fn push(mut self, Authentication: String) -> Self {
       /* assert!(
            Authentication.len() <= MAX_TOKEN_TAG_LEN,
            "exceeded maximum routing tag length!"
        );*/
        self.inner.Authentication.push(Authentication);
        self
    }
    pub fn build(self) -> TOKENMetadata {
        self.inner
    }
}

impl TOKENMetadata {
    pub fn builder() -> TOKENMetadataBuilder {
        TOKENMetadataBuilder {
            inner: TOKENMetadata { Authentication: vec![] },
        }
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.Authentication
    }
}

impl Writeable for TOKENMetadata {
    fn write_to(&self, bf: &mut BytesMut) {
        for tag in &self.Authentication {
            let size = tag.len() as u8;
            bf.put_u8(129);
            bf.put_slice(tag.as_bytes());
        }
    }

    fn len(&self) -> usize {
        let mut n = 0;
        for tag in &self.Authentication {
            n += 1 + tag.as_bytes().len();
        }
        n
    }
}
