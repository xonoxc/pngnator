use crate::{
    chunk::Chunk,
    cli::{EncodeArgs, MutateArgs},
    png::Png,
};

#[rustfmt::skip]
pub fn encode(args: EncodeArgs) {
    let file_bytes = std::fs::read(&args.file_path)
        .expect("Failed to read file");

    let mut png = Png::try_from(file_bytes.as_ref())
        .expect("Failed to parse PNG");

    let chunk = Chunk::new(
        args.chunk_type.clone(), args.message.into_bytes()
    );

    let iend = png.remove_first_chunk("IEND").ok();
    png.append_chunk(chunk);
    if let Some(iend_chunk) = iend {
        png.append_chunk(iend_chunk);
    }

    std::fs::write(&args.output_file, png.as_bytes())
        .expect("Failed to write output");

    println!("Encoded message into {}", args.output_file)
}

#[rustfmt::skip]
pub fn decode(args: MutateArgs) {
    let file_bytes = std::fs::read(&args.file_path)
        .expect("Failed to read file");

    let png = Png::try_from(file_bytes.as_ref()).expect("Failed to parse PNG");

    let chunk = png
        .chunk_by_type(&args.chunk_type.to_string())
        .expect("Chunk type not found");

    println!("{}", chunk.data_as_string().expect("Invalid UTF-8 data"));
}

#[rustfmt::skip]
pub fn remove(args: MutateArgs) {
    let file_bytes = std::fs::read(&args.file_path)
        .expect("Failed to read file");

    let mut png = Png::try_from(file_bytes.as_ref()).expect("Failed to parse PNG");

    png.remove_first_chunk(&args.chunk_type.to_string())
        .expect("Chunk type not found");

    std::fs::write(&args.file_path, png.as_bytes())
        .expect("Failed to write output");

    println!("Removed chunk {} from {}", args.chunk_type, args.file_path)
}

#[rustfmt::skip]
pub fn print(file_path: String) {
    let file_bytes = std::fs::read(&file_path)
        .expect("Failed to read file");

    let png = Png::try_from(file_bytes.as_ref()).expect("Failed to parse PNG");

    for chunk in png.chunks() {
        println!("{}", chunk);
    }
}
