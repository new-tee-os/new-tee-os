use elf_loader::ElfReader;
use hal::edge::EdgeFile;

pub struct EdgeElfFile(pub EdgeFile);

impl ElfReader for EdgeElfFile {
    fn read(&mut self, buf: &mut [u8]) -> usize {
        self.0.read(buf)
    }

    fn seek(&mut self, pos: u64) {
        self.0.seek(pos);
    }
}
