use std::cell::RefCell;

thread_local! {
    static CHAT: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn send_msg(m: String) {
    CHAT.with(|chat| chat.borrow_mut().push(m))
}

#[ic_cdk::query]
fn get_all()  -> Vec<String> {
    CHAT.with(|chat| chat.borrow().clone())
}

