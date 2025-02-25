// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InvokeEndpointInput {
    /// <p>The desired MIME type of the inference in the response.</p>
    #[serde(rename = "accept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    /// <p>Provides input data, in the format specified in the <code>ContentType</code> request header. Amazon SageMaker passes all of the data in the body to the model. </p> <p>For information about the format of the request body, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/cdf-inference.html">Common Data Formats-Inference</a>.</p>
    #[serde(rename = "body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub body: bytes::Bytes,
    /// <p>The MIME type of the input data in the request body.</p>
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>Provides additional information about a request for an inference submitted to a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to provide an ID that you can use to track a request or to provide other metadata that a service endpoint was programmed to process. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://tools.ietf.org/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). </p> <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID:</code> in your post-processing function.</p> <p>This feature is currently supported in the AWS SDKs but not in the Amazon SageMaker Python SDK.</p>
    #[serde(rename = "customAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<String>,
    /// <p>The name of the endpoint that you specified when you created the endpoint using the <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/API_CreateEndpoint.html">CreateEndpoint</a> API. </p>
    #[serde(rename = "endpointName")]
    pub endpoint_name: String,
    /// <p>If you provide a value, it is added to the captured data when you enable data capture on the endpoint. For information about data capture, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-monitor-data-capture.html">Capture Data</a>.</p>
    #[serde(rename = "inferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_id: Option<String>,
    /// <p>If the endpoint hosts multiple containers and is configured to use direct invocation, this parameter specifies the host name of the container to invoke.</p>
    #[serde(rename = "targetContainerHostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_container_hostname: Option<String>,
    /// <p>The model to request for inference when invoking a multi-model endpoint.</p>
    #[serde(rename = "targetModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_model: Option<String>,
    /// <p>Specify the production variant to send the inference request to when invoking an endpoint that is running two or more variants. Note that this parameter overrides the default behavior for the endpoint, which is to distribute the invocation traffic based on the variant weights.</p> <p>For information about how to use variant targeting to perform a/b testing, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/model-ab-testing.html">Test models in production</a> </p>
    #[serde(rename = "targetVariant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_variant: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InvokeEndpointOutput {
    /// <p>Includes the inference provided by the model.</p> <p>For information about the format of the response body, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/cdf-inference.html">Common Data Formats-Inference</a>.</p>
    pub body: bytes::Bytes,
    /// <p>The MIME type of the inference returned in the response body.</p>
    pub content_type: Option<String>,
    /// <p>Provides additional information in the response about the inference returned by a model hosted at an Amazon SageMaker endpoint. The information is an opaque value that is forwarded verbatim. You could use this value, for example, to return an ID received in the <code>CustomAttributes</code> header of a request or other metadata that a service endpoint was programmed to produce. The value must consist of no more than 1024 visible US-ASCII characters as specified in <a href="https://tools.ietf.org/html/rfc7230#section-3.2.6">Section 3.3.6. Field Value Components</a> of the Hypertext Transfer Protocol (HTTP/1.1). If the customer wants the custom attribute returned, the model must set the custom attribute to be included on the way back. </p> <p>The code in your model is responsible for setting or updating any custom attributes in the response. If your code does not set this value in the response, an empty value is returned. For example, if a custom attribute represents the trace ID, your model can prepend the custom attribute with <code>Trace ID:</code> in your post-processing function.</p> <p>This feature is currently supported in the AWS SDKs but not in the Amazon SageMaker Python SDK.</p>
    pub custom_attributes: Option<String>,
    /// <p>Identifies the production variant that was invoked.</p>
    pub invoked_production_variant: Option<String>,
}

/// Errors returned by InvokeEndpoint
#[derive(Debug, PartialEq)]
pub enum InvokeEndpointError {
    /// <p> An internal failure occurred. </p>
    InternalFailure(String),
    /// <p> Model (owned by the customer in the container) returned 4xx or 5xx error code. </p>
    ModelError(String),
    /// <p> The service is unavailable. Try your call again. </p>
    ServiceUnavailable(String),
    /// <p> Inspect your request and try again. </p>
    ValidationError(String),
}

impl InvokeEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InvokeEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailure" => {
                    return RusotoError::Service(InvokeEndpointError::InternalFailure(err.msg))
                }
                "ModelError" => {
                    return RusotoError::Service(InvokeEndpointError::ModelError(err.msg))
                }
                "ServiceUnavailable" => {
                    return RusotoError::Service(InvokeEndpointError::ServiceUnavailable(err.msg))
                }
                "ValidationError" => {
                    return RusotoError::Service(InvokeEndpointError::ValidationError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for InvokeEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvokeEndpointError::InternalFailure(ref cause) => write!(f, "{}", cause),
            InvokeEndpointError::ModelError(ref cause) => write!(f, "{}", cause),
            InvokeEndpointError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            InvokeEndpointError::ValidationError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InvokeEndpointError {}
/// Trait representing the capabilities of the Amazon SageMaker Runtime API. Amazon SageMaker Runtime clients implement this trait.
#[async_trait]
pub trait SageMakerRuntime {
    /// <p><p>After you deploy a model into production using Amazon SageMaker hosting services, your client applications use this API to get inferences from the model hosted at the specified endpoint. </p> <p>For an overview of Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p> <p>Amazon SageMaker strips all POST headers except those supported by the API. Amazon SageMaker might add additional headers. You should not rely on the behavior of headers outside those enumerated in the request syntax. </p> <p>Calls to <code>InvokeEndpoint</code> are authenticated by using AWS Signature Version 4. For information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html">Authenticating Requests (AWS Signature Version 4)</a> in the <i>Amazon S3 API Reference</i>.</p> <p>A customer&#39;s model containers must respond to requests within 60 seconds. The model itself can have a maximum processing time of 60 seconds before responding to invocations. If your model is going to take 50-60 seconds of processing time, the SDK socket timeout should be set to be 70 seconds.</p> <note> <p>Endpoints are scoped to an individual account, and are not public. The URL does not contain the account ID, but Amazon SageMaker determines the account ID from the authentication token that is supplied by the caller.</p> </note></p>
    async fn invoke_endpoint(
        &self,
        input: InvokeEndpointInput,
    ) -> Result<InvokeEndpointOutput, RusotoError<InvokeEndpointError>>;
}
/// A client for the Amazon SageMaker Runtime API.
#[derive(Clone)]
pub struct SageMakerRuntimeClient {
    client: Client,
    region: region::Region,
}

impl SageMakerRuntimeClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> SageMakerRuntimeClient {
        SageMakerRuntimeClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> SageMakerRuntimeClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        SageMakerRuntimeClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> SageMakerRuntimeClient {
        SageMakerRuntimeClient { client, region }
    }
}

#[async_trait]
impl SageMakerRuntime for SageMakerRuntimeClient {
    /// <p><p>After you deploy a model into production using Amazon SageMaker hosting services, your client applications use this API to get inferences from the model hosted at the specified endpoint. </p> <p>For an overview of Amazon SageMaker, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/how-it-works.html">How It Works</a>. </p> <p>Amazon SageMaker strips all POST headers except those supported by the API. Amazon SageMaker might add additional headers. You should not rely on the behavior of headers outside those enumerated in the request syntax. </p> <p>Calls to <code>InvokeEndpoint</code> are authenticated by using AWS Signature Version 4. For information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html">Authenticating Requests (AWS Signature Version 4)</a> in the <i>Amazon S3 API Reference</i>.</p> <p>A customer&#39;s model containers must respond to requests within 60 seconds. The model itself can have a maximum processing time of 60 seconds before responding to invocations. If your model is going to take 50-60 seconds of processing time, the SDK socket timeout should be set to be 70 seconds.</p> <note> <p>Endpoints are scoped to an individual account, and are not public. The URL does not contain the account ID, but Amazon SageMaker determines the account ID from the authentication token that is supplied by the caller.</p> </note></p>
    #[allow(unused_mut)]
    async fn invoke_endpoint(
        &self,
        input: InvokeEndpointInput,
    ) -> Result<InvokeEndpointOutput, RusotoError<InvokeEndpointError>> {
        let request_uri = format!(
            "/endpoints/{endpoint_name}/invocations",
            endpoint_name = input.endpoint_name
        );

        let mut request = SignedRequest::new("POST", "sagemaker", &self.region, &request_uri);
        if input.content_type.is_none() {
            request.set_content_type("application/x-amz-json-1.1".to_owned());
        }

        request.set_endpoint_prefix("runtime.sagemaker".to_string());
        let encoded = Some(input.body.to_owned());
        request.set_payload(encoded);
        request.add_optional_header("Accept", input.accept.as_ref());
        request.add_optional_header("Content-Type", input.content_type.as_ref());
        request.add_optional_header(
            "X-Amzn-SageMaker-Custom-Attributes",
            input.custom_attributes.as_ref(),
        );
        request.add_optional_header("X-Amzn-SageMaker-Inference-Id", input.inference_id.as_ref());
        request.add_optional_header(
            "X-Amzn-SageMaker-Target-Container-Hostname",
            input.target_container_hostname.as_ref(),
        );
        request.add_optional_header("X-Amzn-SageMaker-Target-Model", input.target_model.as_ref());
        request.add_optional_header(
            "X-Amzn-SageMaker-Target-Variant",
            input.target_variant.as_ref(),
        );

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = InvokeEndpointOutput::default();
            result.body = response.body;

            result.content_type = response.headers.remove("Content-Type");
            result.custom_attributes = response
                .headers
                .remove("X-Amzn-SageMaker-Custom-Attributes");
            result.invoked_production_variant =
                response.headers.remove("x-Amzn-Invoked-Production-Variant");

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(InvokeEndpointError::from_response(response))
        }
    }
}
