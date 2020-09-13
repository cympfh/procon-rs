/// Time Limited Loop
#[macro_export]
macro_rules! loop_timeout_ms {
    ( $milli_seconds:expr; $body:expr ) => {
        let now = std::time::SystemTime::now();
        loop {
            match now.elapsed() {
                Ok(elapsed) => {
                    if elapsed.as_millis() > $milli_seconds {
                        break;
                    }
                    $body
                }
                Err(e) => {
                    eprintln!("Err, {:?}", e);
                }
            }
        }
    };
}

#[cfg(test)]
mod test_timed_loop {
    #[test]
    fn test_timed_loop() {
        loop_timeout_ms!(10; {
            eprintln!("y");
        });
    }
}
