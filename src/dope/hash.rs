
//Hashing algorithms for different sorts of data

pub trait Hashable {
    fn hash(&self) -> u32;
}

impl Hashable for String {
    fn hash(&self) -> u32 {
        let mut hash: u32 = 0;
        for i in self.chars() {
            let highorder = hash & 0xf8000000;

            hash = hash << 5;
            hash = hash ^ (highorder >> 27);
            hash = hash ^ i as u32;
        }
        hash
    }
}

impl Hashable for [u8; 4] {
    fn hash(&self) -> u32 {
        let mut hash: u32 = 0;
        for i in self {
            let highorder = hash & 0xf8000000;

            hash = hash << 5;
            hash = hash ^ (highorder >> 27);
            hash = hash ^ *i as u32;
        }
        hash
    }

}