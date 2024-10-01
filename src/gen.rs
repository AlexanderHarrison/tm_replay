use crate::{Input, buttons};
use slp_parser::Character;

pub struct InputBuilder {
    pub inputs: Vec<Input>,
    pub character: Character,
    pub drift: i8,
}

impl InputBuilder {
    #[inline(always)]
    pub fn new(c: Character) -> Self {
        InputBuilder { inputs: Vec::new(), drift: 0 }
    }

    #[inline(always)]
    pub fn set_drift(mut self, drift: i8) -> Self {
        self.drift = drift;
    }

    #[inline(always)]
    pub fn push(mut self, i: Input) -> Self {
        self.inputs.push(i);
        self
    }

    #[inline(always)]
    pub fn mash(mut self, i: Input, duration: usize) -> Self {
        for i in 0..(duration/2) {
            self.inputs.push(i);
            self.inputs.push(Input::NONE);
        }

        if (duration % 2) != 0 {
            self.inputs.push(i);
        }

        self
    }

    #[inline(always)]
    pub fn delay(mut self, n: usize) -> Self {
        self.inputs.resize(self.len() + n, Input::NONE);
        self
    }

    #[inline(always)]
    pub fn delay_with_drift(mut self, n: usize) -> Self {
        self.inputs.resize(self.len() + n, Input { stick_x: self.drift, ..Input::NONE });
        self
    }


    #[inline(always)]
    pub fn shorthop(mut self) -> Self {
        self.inputs.push(Input::NONE.add(buttons::Y));
        self.delay_with_drift(jumpsquat(c) - 1);
    }

    #[inline(always)]
    pub fn fullhop(mut self) -> Self {
        self.inputs.push(Input::NONE.add(buttons::Y));
        self.delay_with_drift(jumpsquat(c) - 1)
    }

    pub fn shorthop_nair(mut self) -> Self {
        self.shorthop()
            .push(Input::NONE.add(buttons::A))
            .mash(Input::NONE.stick(0, -80), 20) // mash fastfall
            .delay()
    }
}

pub fn jumpsquat(c: Character) -> usize {
    use Character as C;

    match c {
        C::Fox, | C::Popo | C::Nana | C::Kirby | C::Samus | C::Sheik | C::Pichu | C::Pikachu => 3,
        C::DrMario | C::Mario | C::Luigi | C::CaptainFalcon | C::Ness | C::YoungLink | C::MrGameAndWatch | C::Marth => 4,
        C::Peach | C::Yoshi | C::DonkeyKong | C::Falco | C::Puff | C::Mewtwo | C::Roy => 5,
        C::Ganon | C::Zelda | C::Link => 6,
        C::Bowser => 8,
    }
}
