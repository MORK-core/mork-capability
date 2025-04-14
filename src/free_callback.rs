use alloc::boxed::Box;
use lazy_init::LazyInit;
use crate::cap::{CNodeCap, FrameCap, NotificationCap, PageTableCap, ThreadCap};

pub static CALLBACK_MANAGER: LazyInit<FreeCallbackManager> = LazyInit::new();

pub trait CallbackHandler: Send + Sync {
    fn free_cnode(&self, cap: CNodeCap);

    fn free_frame(&self, cap: FrameCap);

    fn free_page_table(&self, cap: PageTableCap);

    fn free_task(&self, cap: ThreadCap);

    fn free_notification(&self, cap: NotificationCap);
}

pub struct FreeCallbackManager {
    pub handler: Option<Box<dyn CallbackHandler>>
}

impl FreeCallbackManager {
    pub fn new(handler: Box<dyn CallbackHandler>) -> Self {
        Self { handler: Some(handler) }
    }
}

pub fn register_handler(handler: Box<dyn CallbackHandler>) {
    CALLBACK_MANAGER.init_by(FreeCallbackManager::new(handler));
}