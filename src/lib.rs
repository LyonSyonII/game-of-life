pub struct Chunk([u32; 32]); // 1024 cells

impl Chunk {
    pub const fn new() -> Self {
        Chunk([0; 32])
    }

    pub fn get(&self, i: usize, j: usize) -> bool {
        self.0.get(i).copied().unwrap_or_default() & (1 << (31-j)) > 0
    }

    pub fn toggle(&mut self, i: usize, j: usize) {
        if let Some(row) = self.0.get_mut(i) {
            *row ^= 1 << (31-j);
        }
    }

    pub fn set(&mut self, i: usize, j: usize) {
        if let Some(row) = self.0.get_mut(i) {
            *row |= 1 << (31-j);
        }
    }

    pub fn unset(&mut self, i: usize, j: usize) {
        if let Some(row) = self.0.get_mut(i) {
            *row &= u32::MAX ^ (1 << (31-j));
        }
    }

    pub fn to_u32_array(&self) -> [u32; 32*32] {
        let mut arr = [0; 32*32];
        for i in 0..self.0.len() {
            for j in 0..self.0.len() {
                arr[i * j] = self.get(i, j) as u32;
            }
        }
        arr
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            writeln!(f, "{row:032b}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Chunk;

    #[test]
    fn debug() {
        let mut chunk = Chunk::new();
        chunk.set(0, 0);
        dbg!(chunk.get(0, 0));

        chunk.unset(0, 0);
        dbg!(chunk.get(0, 0));

        chunk.toggle(0, 0);
        dbg!(chunk.get(0, 0));

        chunk.toggle(0, 0);
        dbg!(chunk.get(0, 0));

        chunk.toggle(31, 31);
        chunk.toggle(30, 30);
        chunk.toggle(29, 29);
        println!("{chunk:?}");
    }
}