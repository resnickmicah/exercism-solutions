/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    create_buffer(0)
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let fib_size = 5;
    let mut buffer = create_buffer(fib_size);
    for i in 0..fib_size {
        if i == 0 || i == 1 {
            buffer[i] = 1;
        } else {
            buffer[i] = buffer[i-1] + buffer[i-2];
        }
    }
    buffer
}
