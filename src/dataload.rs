use std::fs::File;
use std::io::{self, BufReader, Read};

pub struct FileReader {
    buf_reader: BufReader<File>,
    batch_buffer: Vec<Vec<u8>>,
}

impl FileReader {
    pub fn new(file_path: &str) -> io::Result<FileReader> {
        let file = std::fs::File::open(file_path)?;
        let buf_reader = std::io::BufReader::new(file);
        Ok(FileReader {
            buf_reader,
            batch_buffer: Vec::new(),
        })
    }

    fn read_chunk(&mut self) -> io::Result<Vec<u8>> {
        let mut length_bytes = [0; 8];

        self.buf_reader.read_exact(&mut length_bytes)?;
        
        let length = u64::from_le_bytes(length_bytes).count_ones() as usize;
        let mut buffer = Vec::with_capacity(length + 8);

        buffer.extend_from_slice(&length_bytes);
        buffer.resize(length + 8, 0);

        self.buf_reader.read_exact(&mut buffer[8..])?;

        Ok(buffer)
    }

    pub fn read_batch(&mut self, batch_size: usize) -> io::Result<Vec<Vec<u8>>> {
        let mut chunks = Vec::with_capacity(batch_size);

        for _ in 0..batch_size {
            let chunk = match self.read_chunk() {
                Ok(chunk) => chunk,
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    // End of file, break the loop
                    break;
                }
                Err(e) => return Err(e),
            };

            chunks.push(chunk);
        }

        Ok(chunks)
    }

    fn fill_buffer(&mut self, batch_size: usize) -> io::Result<()> {
        self.batch_buffer.clear();
        self.batch_buffer.reserve(batch_size); // Reserve capacity for the batch

        for _ in 0..batch_size {
            let chunk = match self.read_chunk() {
                Ok(chunk) => chunk,
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    // End of file, break the loop
                    break;
                }
                Err(e) => return Err(e),
            };

            self.batch_buffer.push(chunk);
        }
        // Reverse the order of elements in the buffer
        self.batch_buffer.reverse();

        Ok(())
    }

    fn next_chunk(&mut self) -> Option<Vec<u8>> {
        if self.batch_buffer.is_empty() {
            // Buffer is empty, fill it again
            if let Err(err) = self.fill_buffer(32000) {
                eprintln!("Error filling buffer: {}", err);
                return None;
            }
        }

        // Retrieve and remove the next chunk
        self.batch_buffer.pop()
    }

    pub fn read_all_batches(&mut self, batch_size: usize) -> io::Result<Vec<Vec<u8>>> {
        let mut all_chunks = Vec::new();

        while let Ok(batch) = self.read_batch(batch_size) {
            if batch.is_empty() {
                break; // End of file
            }

            all_chunks.extend(batch);
        }

        Ok(all_chunks)
    }

    pub fn read_all_chunks(&mut self) -> io::Result<Vec<Vec<u8>>> {
        let mut chunks = Vec::new();

        while let Ok(chunk) = self.read_chunk() {
            if chunk.is_empty() {
                // End of file
                break;
            }
            chunks.push(chunk);
        }

        Ok(chunks)
    }

}

impl Iterator for FileReader {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_chunk()
    }
}