pub struct Monarch64System {
    pub motherboard: Box<dyn crate::motherboards::Monarch64Motherboard>,
}

impl Monarch64System {
    pub fn new() -> Self {
        
    }
}