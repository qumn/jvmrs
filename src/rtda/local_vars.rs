use std::ops::{Deref, DerefMut};

use super::slot::SlotVec;

#[derive(Debug)]
pub(crate) struct LocalVars(SlotVec);
impl LocalVars {
    pub(crate) fn new(max_locals: usize) -> Self {
        LocalVars(SlotVec::new(max_locals))
    }
}

impl Deref for LocalVars {
    type Target = SlotVec;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LocalVars {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn local_vars_should_work() {
        let mut local_vars = super::LocalVars::new(10);
        local_vars.set_int(0, 100);
        local_vars.set_int(1, -100);
        local_vars.set_long(2, 2997924580);
        local_vars.set_long(4, -2997924580);
        local_vars.set_float(6, 3.1415926);
        local_vars.set_double(7, 2.71828182845);
        local_vars.set_ref(9, std::ptr::null_mut());
        assert_eq!(local_vars.get_int(0), 100);
        assert_eq!(local_vars.get_int(1), -100);
        assert_eq!(local_vars.get_long(2), 2997924580);
        assert_eq!(local_vars.get_long(4), -2997924580);
        assert_eq!(local_vars.get_float(6), 3.1415926);
        assert_eq!(local_vars.get_double(7), 2.71828182845);
        assert!(local_vars.get_ref(9).is_null());
    }
}
