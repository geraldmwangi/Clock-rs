pub struct LedMatrix {
    pub width: usize,
    pub height: usize,
    pub data: [[bool; 3]; 5],
}

impl LedMatrix {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: [[false; 3]; 5],
        }
    }

    pub fn from_digit(digit: u32) -> Self {
        let patterns = [
            [
                [true, true, true],
                [true, false, true],
                [true, false, true],
                [true, false, true],
                [true, true, true],
            ],
            [
                [false, true, false],
                [true, true, false],
                [false, true, false],
                [false, true, false],
                [true, true, true],
            ],
            [
                [true, true, true],
                [false, false, true],
                [true, true, true],
                [true, false, false],
                [true, true, true],
            ],
            [
                [true, true, true],
                [false, false, true],
                [true, true, true],
                [false, false, true],
                [true, true, true],
            ],
            [
                [true, false, true],
                [true, false, true],
                [true, true, true],
                [false, false, true],
                [false, false, true],
            ],
            [
                [true, true, true],
                [true, false, false],
                [true, true, true],
                [false, false, true],
                [true, true, true],
            ],
            [
                [true, true, true],
                [true, false, false],
                [true, true, true],
                [true, false, true],
                [true, true, true],
            ],
            [
                [true, true, true],
                [false, false, true],
                [false, false, true],
                [false, false, true],
                [false, false, true],
            ],
            [
                [true, true, true],
                [true, false, true],
                [true, true, true],
                [true, false, true],
                [true, true, true],
            ],
            [
                [true, true, true],
                [true, false, true],
                [true, true, true],
                [false, false, true],
                [true, true, true],
            ],
        ];

        let pattern = patterns[digit as usize];

        Self {
            width: 3,
            height: 5,
            data: pattern,
        }
    }
}
