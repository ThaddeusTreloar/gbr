use std::marker::PhantomData;

pub struct LittleEndian;
pub struct BigEndian;

pub struct CombinedRegister<E> {
    left: u8,
    right: u8,
    endieness: PhantomData<E>,
}

impl<T> CombinedRegister<T> {
    pub fn new() -> Self {
        CombinedRegister {
            left: u8::default(),
            right: u8::default(),
            endieness: PhantomData,
        }
    }

    pub fn set_left(&mut self, value: u8) {
        self.left = value;
    }

    pub fn set_right(&mut self, value: u8) {
        self.right = value;
    }

    pub fn get_left(&self) -> &u8 {
        &self.left
    }

    pub fn get_right(&self) -> &u8 {
        &self.right
    }
}

impl<T> Default for CombinedRegister<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl CombinedRegister<LittleEndian> {
    pub fn get_combined(&self) -> u16 {
        (self.left as u16) << 8 | (self.right as u16)
    }

    pub fn set_combined(&mut self, value: u16) {
        self.left = (value >> 8) as u8;
        self.right = value as u8;
    }
}

impl CombinedRegister<BigEndian> {
    pub fn get_combined(&self) -> u16 {
        (self.right as u16) << 8 | (self.left as u16)
    }

    pub fn set_combined(&mut self, value: u16) {
        self.right = (value >> 8) as u8;
        self.left = value as u8;
    }
}

pub struct Registers<T> {
    pub ab: CombinedRegister<T>,
    pub bc: CombinedRegister<T>,
    pub de: CombinedRegister<T>,
    pub hl: CombinedRegister<T>,
    pub sp: u16,
    pub pc: u16,
}

impl<T> Registers<T> {
    pub fn new() -> Self {
        Registers {
            ab: CombinedRegister::<T>::default(),
            bc: CombinedRegister::<T>::default(),
            de: CombinedRegister::<T>::default(),
            hl: CombinedRegister::<T>::default(),
            sp: 0,
            pc: 0,
        }
    }
}

impl<T> Default for Registers<T> {
    fn default() -> Self {
        Self::new()
    }
}

mod register_tests {
    #[test]
    fn test_combinator() {
        use crate::registers::{BigEndian, LittleEndian, Registers};

        let mut reg = Registers::<LittleEndian>::default();

        reg.ab.set_combined(0x1234);

        assert_eq!(reg.ab.get_left(), &0x12);
        assert_eq!(reg.ab.get_right(), &0x34);

        assert_eq!(reg.ab.get_combined(), 0x1234);

        reg.ab.set_left(0x56);

        assert_eq!(reg.ab.get_combined(), 0x5634);

        reg.ab.set_right(0x78);

        assert_eq!(reg.ab.get_combined(), 0x5678);

        let mut reg = Registers::<BigEndian>::new();

        reg.ab.set_combined(0x1234);

        assert_eq!(reg.ab.get_left(), &0x34);
        assert_eq!(reg.ab.get_right(), &0x12);

        assert_eq!(reg.ab.get_combined(), 0x1234);

        reg.ab.set_left(0x56);

        assert_eq!(reg.ab.get_combined(), 0x1256);

        reg.ab.set_right(0x78);

        assert_eq!(reg.ab.get_combined(), 0x7856);
    }
}
