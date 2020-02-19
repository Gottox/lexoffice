use crate::model::voucher_list::{VoucherStatusEnum, VoucherTypeEnum};
use crate::model::VoucherList;
use crate::request::impls::ById;
use crate::request::impls::Paginated;
use crate::request::Endpoint;
use crate::request::Request;
use std::marker::PhantomData;

pub trait Void {}
impl Void for () {}

// Not implementing the into trait here as this must not be public.
fn into<O, T, S>(
    request: Request<VoucherList, O>,
) -> Request<VoucherList, (T, S)> {
    Request {
        client: request.client,
        url: request.url,
        target: request.target,
        state: PhantomData,
    }
}

pub type VoucherListRequest =
    Request<VoucherList, (VoucherTypeEnum, VoucherStatusEnum)>;
pub type UnstartedVoucherListRequest =
    Request<VoucherList, ()>;
pub type IncompleteVoucherListRequest<T, S> =
    Request<VoucherList, (T, S)>;

impl Endpoint for VoucherListRequest {
    const ENDPOINT: &'static str = "voucherlist";
}

impl UnstartedVoucherListRequest {
    pub fn type_(
        self,
        voucher_type: VoucherTypeEnum,
    ) -> IncompleteVoucherListRequest<VoucherTypeEnum, ()> {
        into::<_, (), ()>(self).type_(voucher_type)
    }
    pub fn status(
        self,
        voucher_status: VoucherStatusEnum,
    ) -> IncompleteVoucherListRequest<(), VoucherStatusEnum> {
        into::<_, (), ()>(self).status(voucher_status)
    }
}

impl<T, S> IncompleteVoucherListRequest<T, S> {
    pub fn type_(
        mut self,
        voucher_type: VoucherTypeEnum,
    ) -> IncompleteVoucherListRequest<VoucherTypeEnum, S>
    where
        T: Void,
    {
        self.url.query_pairs_mut().append_pair(
            "voucherType",
            &serde_plain::to_string(&voucher_type).unwrap(),
        );
        into(self)
    }
    pub fn status(
        mut self,
        voucher_status: VoucherStatusEnum,
    ) -> IncompleteVoucherListRequest<T, VoucherStatusEnum>
    where
        S: Void,
    {
        self.url.query_pairs_mut().append_pair(
            "voucherStatus",
            &serde_plain::to_string(&voucher_status).unwrap(),
        );
        into(self)
    }
}

impl ById for VoucherListRequest {}

impl Paginated for VoucherListRequest {}
