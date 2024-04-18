use ic_cdk_macros::update;

use crate::{
    guards::authenticated_guard,
    icrc37_types::{
        ApproveCollectionArg, ApproveCollectionResult, ApproveTokenArg, ApproveTokenResult,
    },
    state::STATE,
};

#[update(guard = "authenticated_guard")]
pub fn icrc37_approve_tokens(args: Vec<ApproveTokenArg>) -> Vec<Option<ApproveTokenResult>> {
    let caller = ic_cdk::caller();
    STATE.with(|s| s.borrow_mut().approve(&caller, args))
}

#[update(guard = "authenticated_guard")]
pub fn icrc37_approve_collection(
    args: Vec<ApproveCollectionArg>,
) -> Vec<Option<ApproveCollectionResult>> {
    let caller = ic_cdk::caller();

    STATE.with(|s| s.borrow_mut().collection_approve(&caller, args))
}