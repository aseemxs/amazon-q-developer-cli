// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_feature_evaluations_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_feature_evaluations::ListFeatureEvaluationsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.user_context {
        #[allow(unused_mut)]
        let mut object_2 = object.key("userContext").start_object();
        crate::protocol_serde::shape_user_context::ser_user_context(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.profile_arn {
        object.key("profileArn").string(var_3.as_str());
    }
    Ok(())
}
