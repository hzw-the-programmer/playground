pub struct S;

impl volo_gen::volo::example::ItemService for S {
    async fn get_item(
        &self,
        _req: ::volo_grpc::Request<volo_gen::volo::example::GetItemRequest>,
    ) -> ::std::result::Result<
        ::volo_grpc::Response<volo_gen::volo::example::GetItemResponse>,
        ::volo_grpc::Status,
    > {
        Ok(::volo_grpc::Response::new(Default::default()))
    }
}
