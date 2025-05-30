// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Structure to represent get test generation request.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetTestGenerationInput {
    /// Test generation job group name
    pub test_generation_job_group_name: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub test_generation_job_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub profile_arn: ::std::option::Option<::std::string::String>,
}
impl GetTestGenerationInput {
    /// Test generation job group name
    pub fn test_generation_job_group_name(&self) -> ::std::option::Option<&str> {
        self.test_generation_job_group_name.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn test_generation_job_id(&self) -> ::std::option::Option<&str> {
        self.test_generation_job_id.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn profile_arn(&self) -> ::std::option::Option<&str> {
        self.profile_arn.as_deref()
    }
}
impl GetTestGenerationInput {
    /// Creates a new builder-style object to manufacture
    /// [`GetTestGenerationInput`](crate::operation::get_test_generation::GetTestGenerationInput).
    pub fn builder() -> crate::operation::get_test_generation::builders::GetTestGenerationInputBuilder {
        crate::operation::get_test_generation::builders::GetTestGenerationInputBuilder::default()
    }
}

/// A builder for
/// [`GetTestGenerationInput`](crate::operation::get_test_generation::GetTestGenerationInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetTestGenerationInputBuilder {
    pub(crate) test_generation_job_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) test_generation_job_id: ::std::option::Option<::std::string::String>,
    pub(crate) profile_arn: ::std::option::Option<::std::string::String>,
}
impl GetTestGenerationInputBuilder {
    /// Test generation job group name
    /// This field is required.
    pub fn test_generation_job_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.test_generation_job_group_name = ::std::option::Option::Some(input.into());
        self
    }

    /// Test generation job group name
    pub fn set_test_generation_job_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.test_generation_job_group_name = input;
        self
    }

    /// Test generation job group name
    pub fn get_test_generation_job_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.test_generation_job_group_name
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn test_generation_job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.test_generation_job_id = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_test_generation_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.test_generation_job_id = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_test_generation_job_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.test_generation_job_id
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn profile_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.profile_arn = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_profile_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.profile_arn = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_profile_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.profile_arn
    }

    /// Consumes the builder and constructs a
    /// [`GetTestGenerationInput`](crate::operation::get_test_generation::GetTestGenerationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_test_generation::GetTestGenerationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_test_generation::GetTestGenerationInput {
            test_generation_job_group_name: self.test_generation_job_group_name,
            test_generation_job_id: self.test_generation_job_id,
            profile_arn: self.profile_arn,
        })
    }
}
