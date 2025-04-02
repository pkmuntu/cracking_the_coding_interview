fn validate_packet_structure(data: Vec<i32>) -> bool {
    // Number of bytes in the current packet
    let mut number_of_bytes_to_process = 0;

    // Masks to check two most significant bits in a byte.
    let mask1 = 1 << 7;
    let mask2 = 1 << 6;

    // For each integer in the data array.
    for i in 0..data.len() {
        // If this is the case then we are to start processing a new packet.
        if number_of_bytes_to_process == 0 {
            let mut mask = 1 << 7;
            while (mask & data[i]) != 0 {
                number_of_bytes_to_process += 1;
                mask = mask >> 1;
            }

            // 1 byte packets
            if number_of_bytes_to_process == 0 {
                continue;
            }

            // Invalid scenarios according to the rules of the problem.
            if number_of_bytes_to_process > 4 || number_of_bytes_to_process == 1 {
                return false;
            }
        } else {
            // data[i] should have most significant bit set and
            // second most significant bit unset. So, we use the two masks
            // to make sure this is the case.
            if !((data[i] & mask1) != 0 && (mask2 & data[i]) == 0) {
                return false;
            }
        }

        // We reduce the number of bytes to process by 1 after each integer.
        number_of_bytes_to_process -= 1;
    }

    // This is for the case where we might not have the complete data for
    // a particular packet.
    return number_of_bytes_to_process == 0;
}
fn main() {
    // driver code

    // Example - 1
    let mut data: Vec<i32> = vec![193, 129, 3];
    println!("{:}", validate_packet_structure(data));

    // Example - 2
    data = vec![230, 129, 7];
    println!("{:}", validate_packet_structure(data));

    // Example - 3

    data = vec![248, 129, 129, 129, 129];
    println!("{:}", validate_packet_structure(data));
}

//  Time complexity = O(n)
//  Space complexity = o(1)
