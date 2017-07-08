/*
 * Address: H ( H ( CKey + Root + Index ) )
 * Tag: Any
 * Message: [ L<NextRoot + Message> + Nonce + Signature + Hashes ]
 *
 * Encryption Key: H^i ( CKey + Root + Index )
 */
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
