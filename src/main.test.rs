#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping() {
        let ctx = Context::default();
        let msg = Message {
            timestamp: Timestamp::from_secs(0),
            ..Default::default()
        };
        let result = ping(&ctx, &msg).unwrap();
        assert_eq!(result, ());
    }

    #[test]
    fn test_help() {
        let ctx = Context::default();
        let msg = Message {
            timestamp: Timestamp::from_secs(0),
            channel_id: ChannelId(0),
            ..Default::default()
        };
        let result = help(&ctx, &msg).unwrap();
        assert_eq!(result, ());
    }

    #[test]
    fn test_infos() {
        let ctx = Context::default();
        let msg = Message {
            timestamp: Timestamp::from_secs(0),
            channel_id: ChannelId(0),
            ..Default::default()
        };
        let result = infos(&ctx, &msg).unwrap();
        assert_eq!(result, ());
    }

    #[test]
    fn test_github() {
        let ctx = Context::default();
        let msg = Message {
            timestamp: Timestamp::from_secs(0),
            ..Default::default()
        };
        let result = github(&ctx, &msg).unwrap();
        assert_eq!(result, ());
    }

    #[test]
    fn test_rust() {
        let ctx = Context::default();
        let msg = Message {
            timestamp: Timestamp::from_secs(0),
            ..Default::default()
        };
        let result = rust(&ctx, &msg).unwrap();
        assert_eq!(result, ());
    }
}
