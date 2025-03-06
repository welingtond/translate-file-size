// Title: File Size Formatter

// Objective: Create a program that takes a file size in bytes and returns a human-readable format.

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
    terabytes: String,
}

impl Sizes {
    fn new(bytes: String) -> Self {
        let mut literal = bytes.trim().split(" ").collect::<Vec<&str>>();
        let bytes = literal[0].parse::<u64>().unwrap();
        let descr: &str = literal[1];
        let bytes = match descr {
            "b" | "B" => bytes,
            "kb" | "KB" => bytes * 1024,
            "mb" | "MB" => bytes * 1_024_000,
            "gb" | "GB" => bytes * 1_024_000_000,
            "tb" | "TB" => bytes * 1_024_000_000_000,
            _ => panic!("Invalid unit"),
        };

        Sizes {
            bytes: format!("{}  bytes", bytes),
            kilobytes: format!("{:.2} kilobytes", bytes as f64 / 1024.0),
            megabytes: format!("{:.2} megabytes", bytes as f64 / 1_024_000.0),
            gigabytes: format!("{:.2} gigabytes", bytes as f64 / 1_024_000_000.0),
            terabytes: format!("{:.2} terabytes", bytes as f64 / 1_024_000_000_000.0),
        }
    }
}

/*     fn format_size(size: u64) -> String {
        let filesize = match size {
            0..=1023 => FileSize::Bytes(size),
            1024..=1_023_999 => FileSize::Kilobytes(size as f64 / 1024.0),
            1_024_000..=1_023_999_999_999 => FileSize::Megabytes(size as f64 / 1_024_000.0),
            1_024_000_000..=1_023_999_999_999_999 => FileSize::Gigabytes(size as f64 / 1_024_000_000.0),
            _ => FileSize::Terabytes(size as f64 / 1_024_000_000_000.0),
        };

        match filesize {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
            FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
            FileSize::Terabytes(tb) => format!("{:.2} TB", tb),
        }
    }
 */


fn main() {
    let result = std::env::args().skip(1).collect::<Vec<String>>()[0].clone();
    let size = Sizes::new(result);
    println!("{:?}", size);

}