// Title: File Size Formatter

// Objective: Create a program that takes a file size in bytes and returns a human-readable format.

#[derive(Debug)]
struct Sizes {
    bytes: u64,
    kilobytes: f64,
    megabytes: f64,
    gigabytes: f64,
    terabytes: f64,
}

impl Sizes {
    fn new(bytes: String) -> Self {
        let bytes = bytes.trim().split(" ").collect::<Vec<&str>>();
        let bytes = bytes[0].parse::<u64>().unwrap();
        Sizes {
            bytes,
            kilobytes: bytes as f64 * 1024.0,
            megabytes: bytes as f64 * 1_024_000.0,
            gigabytes: bytes as f64 * 1_024_000_000.0,
            terabytes: bytes as f64 * 1_024_000_000_000.0,
        }
    }
}
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
}

impl FileSize {

    fn translate(linha: String) -> Self {
        let mut linha = linha.trim().split_whitespace();
        let size = linha.next().unwrap().parse::<u64>().unwrap();
        let unit = linha.next().unwrap();

        match unit {
            "B" => FileSize::Bytes(size),
            "KB" => FileSize::Kilobytes(size as f64),
            "MB" => FileSize::Megabytes(size as f64),
            "GB" => FileSize::Gigabytes(size as f64),
            "TB" => FileSize::Terabytes(size as f64),
            _ => panic!("Invalid unit"),
        }
    }

    fn format_size(size: u64) -> String {
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
}


fn main() {
    let result = std::env::args().skip(1).collect::<Vec<String>>()[0].clone();
    let size = Sizes::new(result);
    println!("{:?}", size);

}