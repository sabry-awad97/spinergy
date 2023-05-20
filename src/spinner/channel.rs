use crossbeam::channel::{unbounded, Receiver, Sender, TryRecvError, TrySendError};

#[derive(Clone)]
pub struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Channel<T>
where
    T: Send + 'static + Clone,
{
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self {
            sender: tx,
            receiver: rx,
        }
    }

    pub fn try_send(&self, message: T) -> Result<(), TrySendError<T>> {
        self.sender.try_send(message)
    }

    pub fn try_receive(&self) -> Result<T, TryRecvError> {
        self.receiver.try_recv()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_receive() {
        let channel = Channel::new();
        let message = "Hello, world!".to_string();
        channel.try_send(message.clone()).unwrap();
        let received = channel.try_receive().unwrap();
        assert_eq!(received, message);
    }

    #[test]
    fn test_receive_empty() {
        let channel = Channel::<String>::new();
        let result = channel.try_receive();
        assert!(result.is_err());
    }
}
