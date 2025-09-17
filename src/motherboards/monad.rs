use crate::motherboards::Monarch64Motherboard;

pub struct MonadMotherboard {
    pub cpu: Box<dyn crate::cpus::Monarch64CPU>,
}

impl Monarch64Motherboard for MonadMotherboard {
    fn get_cpu(&self) -> &dyn crate::cpus::Monarch64CPU {
        &*self.cpu
    }

    fn get_cpu_mut(&mut self) -> &mut dyn crate::cpus::Monarch64CPU {
        &mut *self.cpu
    }

    fn set_cpu(&mut self, cpu: Box<dyn crate::cpus::Monarch64CPU>) {
        self.cpu = cpu;
    }
}