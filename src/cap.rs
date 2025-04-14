use bitfield::bitfield;
use crate::free_callback::CALLBACK_MANAGER;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum CapType {
    UnInit = 0,
    CNode,
    Thread,
    PageTable,
    Frame,
    Notification
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
    pub notification_cap: NotificationCap
}

impl Cap {
    pub fn get_type(&self) -> CapType {
        unsafe {
            CapType::from_word(self.cnode_cap.type_tag())
        }
    }

    pub fn derive(&self) -> Self {
        unsafe {
            let new_cap_raw = self.raw | (1 << 7);
            Self {
                raw: new_cap_raw,
            }
        }
    }

    pub fn is_derived(&self) -> bool {
        unsafe {
            (self.raw & (1 << 7)) != 0
        }
    }

    pub fn free(&self) {
        if self.is_derived() {
            return;
        }
        if let Some(handler) = CALLBACK_MANAGER.handler.as_ref() {
            match self.get_type() {
                CapType::UnInit => {}
                CapType::CNode => {
                    handler.free_cnode(unsafe {self.cnode_cap});
                }
                CapType::Thread => {
                    handler.free_task(unsafe {self.thread_cap});
                }
                CapType::PageTable => {
                    handler.free_page_table(unsafe {self.page_table_cap});
                }
                CapType::Frame => {
                    handler.free_frame(unsafe {self.frame_cap} );
                }
                CapType::Notification => {
                    handler.free_notification(unsafe {self.notification_cap} );
                }
            }
        } else {
            panic!("free-handler is null!");
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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CNodeCap(u128);
    impl Debug;
    pub type_tag, set_type : 6, 0;
    pub base_ptr, set_base_ptr: 64, 13;
}

bitfield! {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ThreadCap(u128);
    impl Debug;
    pub type_tag, set_type : 6, 0;
    pub base_ptr, set_base_ptr: 64, 13;
}

bitfield! {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PageTableCap(u128);
    impl Debug;
    pub type_tag, set_type : 6, 0;
    pub base_ptr, set_base_ptr: 64, 13;
    pub mapped_addr, set_mapped_addr: 116, 65;
    pub mapped_level, set_level: 126, 119;
    pub is_mapped, set_mapped: 127, 127;
}

bitfield! {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct FrameCap(u128);
    impl Debug;
    pub type_tag, set_type : 6, 0;
    pub base_ptr, set_base_ptr: 64, 13;
    pub mapped_addr, set_mapped_addr: 116, 65;
    pub level, set_level: 126, 119;
    pub is_mapped, set_mapped: 127, 127;
}

bitfield! {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct NotificationCap(u128);
    impl Debug;
    pub type_tag, set_type : 6, 0;
    pub base_ptr, set_base_ptr: 64, 13;
    pub badge, set_badge: 70, 65;
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

impl FrameCap {
    pub fn new(base_ptr: usize, level: usize) -> Self {
        let mut cap = FrameCap(0);
        cap.set_type(CapType::Frame as usize as u128);
        cap.set_level(level as u128);
        cap.set_base_ptr(base_ptr as u128 >> 12);
        cap
    }

    pub fn into_cap(self) -> Cap {
        Cap {
            frame_cap: self,
        }
    }
}

impl NotificationCap {
    pub fn new(base_ptr: usize) -> Self {
        let mut cap = NotificationCap(0);
        cap.set_type(CapType::Notification as usize as u128);
        cap.set_base_ptr(base_ptr as u128 >> 12);
        cap
    }

    pub fn into_cap(self) -> Cap {
        Cap {
            notification_cap: self,
        }
    }
}

