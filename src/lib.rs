mod cpu;

#[cfg(test)]
mod tests {
    use crate::cpu::Flags;

    #[test]
    fn test_flags() {
        // B: Binary, u8
        // O: Object, struct

        assert!(u8::from(Flags::from(0b10000000 as u8)) == 0b10000000, "[Flags; B->O->B]: Zero flag isn't translated correctly!");
        assert!(u8::from(Flags::from(0b01000000 as u8)) == 0b01000000, "[Flags; B->O->B]: Subtract flag isn't translated correctly!");
        assert!(u8::from(Flags::from(0b00100000 as u8)) == 0b00100000, "[Flags; B->O->B]: Half carry flag isn't translated correctly!");
        assert!(u8::from(Flags::from(0b00010000 as u8)) == 0b00010000, "[Flags; B->O->B]: Carry flag isn't translated correctly!");
        assert!(u8::from(Flags::from(0b11110000 as u8)) == 0b11110000, "[Flags; B->O->B]: Z, S, H and C flag aren't translated correctly!");

        
        assert!(Flags::from(0b10000000 as u8).zero, "[Flags; B->O]: Zero flag isn't translated correctly!");
        assert!(Flags::from(0b01000000 as u8).subtract, "[Flags; B->O]: Subtract flag isn't translated correctly!");
        assert!(Flags::from(0b00100000 as u8).half_carry, "[Flags; B->O]: Half carry flag isn't translated correctly!");
        assert!(Flags::from(0b00010000 as u8).carry, "[Flags; B->O]: Carry flag isn't translated correctly!");
        
        let flags = Flags::from(0b11110000 as u8);
        assert!(flags.zero && flags.subtract && flags.half_carry && flags.carry, "[Flags; B->O]: Z, S, H and C flag aren't double translated correctly!");

        assert!(u8::from(Flags::new().set_zero()) == 0b10000000, "[Flags; O->]: Zero flag isn't translated correctly!");
        assert!(u8::from(Flags::new().set_subtract()) == 0b01000000, "[Flags; O->]: Subtract flag isn't translated correctly!");
        assert!(u8::from(Flags::new().set_half_carry()) == 0b00100000, "[Flags; O->]: Half carry flag isn't translated correctly!");
        assert!(u8::from(Flags::new().set_carry()) == 0b00010000, "[Flags; O->]: Carry flag isn't translated correctly!");
        assert!(u8::from(Flags::new().set_zero().set_subtract().set_half_carry().set_carry()) == 0b11110000, "[Flags; O->]: Z, S, H and C flag aren't translated correctly!");
    }
}