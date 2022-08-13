use time_fn::time_fn;

#[test]
#[time_fn]
fn test_sleep_5000() {
    std::thread::sleep(std::time::Duration::from_millis(5000));
}

#[test]
#[time_fn]
fn test_simple_maths() {
    let add = 5 + 10;
}

#[test]
fn test_return_time() {
    #[time_fn(return_time = true)]
    fn instant_return() {}

    let time = instant_return();

    println!("Instant return took: {}", time);
}
