#[cfg(test)]
mod tests {
    use bincake_core::*;
    use bincake_derive::Serialize;

    #[derive(Serialize, Debug, PartialEq, Eq)]
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    #[derive(Serialize, Debug, PartialEq, Eq)]
    struct Player {
        name: String,
        position: Point,
        health: u32,
        active: bool,
    }

    #[derive(Serialize, Debug, PartialEq, Eq)]
    enum Instruction {
        Nop,
        Push(u32),
        Pop,
        Add,
        Load { addr: u32, size: u8 },
    }

    #[test]
    fn test_struct_serialization() {
        let point = Point {
            x: 10,
            y: -20,
            z: 30,
        };
        let mut buffer = Vec::new();

        buffer.write(&point).expect("Failed to write Point");

        let mut stream = buffer.to_tape();
        let decoded = stream.read::<Point>().expect("Failed to read Point");

        assert_eq!(point, decoded);
    }

    #[test]
    fn test_nested_struct_serialization() {
        let player = Player {
            name: "Rustacean".to_string(),
            position: Point { x: 1, y: 2, z: 3 },
            health: 100,
            active: true,
        };
        let mut buffer = Vec::new();

        buffer.write(&player).expect("Failed to write Player");

        let mut stream = buffer.to_tape();
        let decoded = stream.read::<Player>().expect("Failed to read Player");

        assert_eq!(player, decoded);
    }

    #[test]
    fn test_enum_variants() {
        let instructions = vec![
            Instruction::Nop,
            Instruction::Push(42),
            Instruction::Load {
                addr: 0xFF,
                size: 4,
            },
            Instruction::Pop,
        ];

        for instr in instructions {
            let mut buffer = Vec::new();
            buffer.write(&instr).expect("Failed to write Instruction");

            let mut stream = buffer.to_tape();
            let decoded = stream
                .read::<Instruction>()
                .expect("Failed to read Instruction");

            assert_eq!(instr, decoded);
        }
    }
}
