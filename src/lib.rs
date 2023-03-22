pub enum Sex {
    Male,
    Female,
}

pub enum HairColor {
    Black,
    Brown,
    Red,
    Blonde,
    Blue,
    Pink,
    Purple,
}

pub enum EyeColor {
    Blue,
    Green,
    Hazel,
    Brown,
}

pub enum SkinColor {
    Light,
    Medium,
    Dark,
}

pub enum Class {
    Fighter,
    Hacker,

}

pub struct Stats {

}

pub struct Character {
    pub name:String,
    pub sex:Sex,
    pub hair_color:HairColor,
    pub eye_color:EyeColor,
    pub skin_color:SkinColor,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
