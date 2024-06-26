fn main() {}

impl std::ops::AddAssign for () {
    //~^ ERROR only traits defined in the current crate can be implemented for arbitrary types
    fn add_assign(&self, other: ()) -> () { //~ ERROR incompatible type
        ()
    }
}

impl std::ops::AddAssign for [(); 1] {
    //~^ ERROR only traits defined in the current crate can be implemented for arbitrary types
    fn add_assign(&self, other: [(); 1]) -> [(); 1] { //~ ERROR incompatible type
        [()]
    }
}

impl std::ops::AddAssign for &[u8] {
    //~^ ERROR only traits defined in the current crate can be implemented for arbitrary types
    fn add_assign(&self, other: &[u8]) -> &[u8] { //~ ERROR incompatible type
        self
    }
}
