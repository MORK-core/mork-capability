use bitfield::bitfield;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum CapType {
    CNode = 0,
    Thread,
    PageTable,
    Frame
}

impl CapType {
    pub fn from_word(label: u128) -> Self {
        unsafe {
            core::mem::transmute::<u8, CapType>(label as u8)
        }
    }
}

#[derive(Copy, Clone)]
pub union Cap {
    raw: u128,
    pub cnode_cap: CNodeCap,
    pub thread_cap: ThreadCap,
    pub page_table_cap: PageTableCap,
    pub frame_cap: FrameCap,
}

impl Cap {
    pub fn get_type(&self) -> CapType {
        unsafe {
            CapType::from_word(self.cnode_cap.type_tag())
        }
    }
}

impl Default for Cap {
    fn default() -> Self {
        Self {
            raw: 0,
        }
    }
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct CNodeCap(u128);
    impl Debug;
    pub type_tag, set_type : 7, 0;
    pub base_ptr, set_base_ptr: 64, 13;
}

impl CNodeCap {
    pub fn new(base_ptr: usize) -> Self {
        let mut cap = CNodeCap(0);
        cap.set_type(CapType::CNode as usize as u128);
        cap.set_base_ptr(base_ptr as u128 >> 12);
        cap
    }

    pub fn into_cap(self) -> Cap {
        Cap {
            cnode_cap: self
        }
    }
}


bitfield! {
    #[derive(Copy, Clone)]
    pub struct ThreadCap(u128);
    impl Debug;
    pub type_tag, set_type : 7, 0;
    pub base_ptr, set_base_ptr: 64, 13;
}

impl ThreadCap {
    pub fn new(base_ptr: usize) -> Self {
        let mut cap = ThreadCap(0);
        cap.set_type(CapType::Thread as usize as u128);
        cap.set_base_ptr(base_ptr as u128 >> 12);
        cap
    }

    pub fn into_cap(self) -> Cap {
        Cap {
            thread_cap: self,
        }
    }
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct PageTableCap(u128);
    impl Debug;
    pub type_tag, set_type : 7, 0;
    pub base_ptr, set_base_ptr: 64, 13;
}

impl PageTableCap {
    pub fn new(base_ptr: usize) -> Self {
        let mut cap = PageTableCap(0);
        cap.set_type(CapType::PageTable as usize as u128);
        cap.set_base_ptr(base_ptr as u128 >> 12);
        cap
    }

    pub fn into_cap(self) -> Cap {
        Cap {
            page_table_cap: self,
        }
    }
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct FrameCap(u128);
    impl Debug;
    pub type_tag, set_type : 7, 0;
    pub base_ptr, set_base_ptr: 64, 13;
}

impl FrameCap {
    pub fn new(base_ptr: usize) -> Self {
        let mut cap = FrameCap(0);
        cap.set_type(CapType::Frame as usize as u128);
        cap.set_base_ptr(base_ptr as u128 >> 12);
        cap
    }

    pub fn into_cap(self) -> Cap {
        Cap {
            frame_cap: self,
        }
    }
}

