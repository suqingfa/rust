use std::time::Duration;

use tokio::time::Instant;
use tokio::{join, test, time::sleep};

async fn do_something(i: i32) -> i32 {
    sleep(Duration::from_millis(100)).await;
    i
}

#[test]
async fn test_async() {
    let future1 = do_something(10);
    let future2 = do_something(20);
    let future3 = do_something(30);

    let instant = Instant::now();

    let (x1, x2, x3) = join!(future1, future2, future3);

    let instant = Instant::now() - instant;
    assert!(instant.as_millis() < 110);

    assert_eq!(10, x1);
    assert_eq!(20, x2);
    assert_eq!(30, x3);
}
