pub(crate) fn mem_dump(raw_memory: &Vec<u8>) {
    print_nonzero_regions(&raw_memory, 1);
}

fn print_nonzero_regions(data: &[u8], context: usize) {
    let mut i = 0;
    let len = data.len();

    while i < len {
        if data[i] != 0 {
            let start = i.saturating_sub(context);
            let mut end = i + 1;

            while end < len && (data[end] != 0 || end < i + context) {
                end += 1;
            }

            print!("Region [{:02X}..{:02X}]:", start, end);
            for byte in &data[start..end] {
                print!(" {:02X}", byte);
            }
            println!();

            i = end;
        } else {
            i += 1;
        }
    }
}

