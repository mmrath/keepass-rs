
error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        Compression(v: String) {
            description("Decompression error"),
        }
        Crypto {
            description("Cryptography error")
            display("Cryptography error")
        }
        IncorrectKey {
            description("Incorrect key")
        }
        InvalidIdentifier {
            description("Invalid file header - not a .kdbx file?")
        }
        InvalidHeaderEntry(h: u8)  {
            description("Encountered invalid header entry")
        }
        InvalidCipherID {
            description ("Encountered an invalid cipher ID")
        }
        InvalidCompressionSuite {
            description("Encountered an invalid compression suite")
        }
        InvalidInnerRandomStreamId {
            description("Encountered an invalid inner stream cipher")
        }
        BlockHashMismatch {
            description( "Block hash verification failed"),
        }
    }

}


impl ::std::convert::From<::crypto::symmetriccipher::SymmetricCipherError> for self::Error {
    fn from(_ce: ::crypto::symmetriccipher::SymmetricCipherError) -> Self{
        self::ErrorKind::Crypto.into()
    }
}

