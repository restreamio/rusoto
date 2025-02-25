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
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl MarketplaceEntitlementClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "aws-marketplace", &self.region, request_uri);
        request.set_endpoint_prefix("entitlement.marketplace".to_string());

        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request
    }

    async fn sign_and_dispatch<E>(
        &self,
        request: SignedRequest,
        from_response: fn(BufferedHttpResponse) -> RusotoError<E>,
    ) -> Result<HttpResponse, RusotoError<E>> {
        let mut response = self.client.sign_and_dispatch(request).await?;
        if !response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            return Err(from_response(response));
        }

        Ok(response)
    }
}

use serde_json;
/// <p>An entitlement represents capacity in a product owned by the customer. For example, a customer might own some number of users or seats in an SaaS application or some amount of data capacity in a multi-tenant database.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Entitlement {
    /// <p>The customer identifier is a handle to each unique customer in an application. Customer identifiers are obtained through the ResolveCustomer operation in AWS Marketplace Metering Service.</p>
    #[serde(rename = "customerIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_identifier: Option<String>,
    /// <p>The dimension for which the given entitlement applies. Dimensions represent categories of capacity in a product and are specified when the product is listed in AWS Marketplace.</p>
    #[serde(rename = "dimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    /// <p>The expiration date represents the minimum date through which this entitlement is expected to remain valid. For contractual products listed on AWS Marketplace, the expiration date is the date at which the customer will renew or cancel their contract. Customers who are opting to renew their contract will still have entitlements with an expiration date.</p>
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    /// <p>The product code for which the given entitlement applies. Product codes are provided by AWS Marketplace when the product listing is created.</p>
    #[serde(rename = "productCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// <p>The EntitlementValue represents the amount of capacity that the customer is entitled to for the product.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<EntitlementValue>,
}

/// <p>The EntitlementValue represents the amount of capacity that the customer is entitled to for the product.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EntitlementValue {
    /// <p>The BooleanValue field will be populated with a boolean value when the entitlement is a boolean type. Otherwise, the field will not be set.</p>
    #[serde(rename = "booleanValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    /// <p>The DoubleValue field will be populated with a double value when the entitlement is a double type. Otherwise, the field will not be set.</p>
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    /// <p>The IntegerValue field will be populated with an integer value when the entitlement is an integer type. Otherwise, the field will not be set.</p>
    #[serde(rename = "integerValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_value: Option<i64>,
    /// <p>The StringValue field will be populated with a string value when the entitlement is a string type. Otherwise, the field will not be set.</p>
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

/// <p>The GetEntitlementsRequest contains parameters for the GetEntitlements operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEntitlementsRequest {
    /// <p>Filter is used to return entitlements for a specific customer or for a specific dimension. Filters are described as keys mapped to a lists of values. Filtered requests are <i>unioned</i> for each value in the value list, and then <i>intersected</i> for each filter key.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The maximum number of items to retrieve from the GetEntitlements operation. For pagination, use the NextToken field in subsequent calls to GetEntitlements.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For paginated calls to GetEntitlements, pass the NextToken from the previous GetEntitlementsResult.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code will be provided by AWS Marketplace when the product listing is created.</p>
    #[serde(rename = "productCode")]
    pub product_code: String,
}

/// <p>The GetEntitlementsRequest contains results from the GetEntitlements operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEntitlementsResult {
    /// <p>The set of entitlements found through the GetEntitlements operation. If the result contains an empty set of entitlements, NextToken might still be present and should be used.</p>
    #[serde(rename = "entitlements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<Entitlement>>,
    /// <p>For paginated results, use NextToken in subsequent calls to GetEntitlements. If the result contains an empty set of entitlements, NextToken might still be present and should be used.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// Errors returned by GetEntitlements
#[derive(Debug, PartialEq)]
pub enum GetEntitlementsError {
    /// <p>An internal error has occurred. Retry your request. If the problem persists, post a message with details on the AWS forums.</p>
    InternalServiceError(String),
    /// <p>One or more parameters in your request was invalid.</p>
    InvalidParameter(String),
    /// <p>The calls to the GetEntitlements API are throttled.</p>
    Throttling(String),
}

impl GetEntitlementsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEntitlementsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceErrorException" => {
                    return RusotoError::Service(GetEntitlementsError::InternalServiceError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetEntitlementsError::InvalidParameter(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetEntitlementsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEntitlementsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEntitlementsError::InternalServiceError(ref cause) => write!(f, "{}", cause),
            GetEntitlementsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetEntitlementsError::Throttling(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEntitlementsError {}
/// Trait representing the capabilities of the AWS Marketplace Entitlement Service API. AWS Marketplace Entitlement Service clients implement this trait.
#[async_trait]
pub trait MarketplaceEntitlement {
    /// <p>GetEntitlements retrieves entitlement values for a given product. The results can be filtered based on customer identifier or product dimensions.</p>
    async fn get_entitlements(
        &self,
        input: GetEntitlementsRequest,
    ) -> Result<GetEntitlementsResult, RusotoError<GetEntitlementsError>>;
}
/// A client for the AWS Marketplace Entitlement Service API.
#[derive(Clone)]
pub struct MarketplaceEntitlementClient {
    client: Client,
    region: region::Region,
}

impl MarketplaceEntitlementClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MarketplaceEntitlementClient {
        MarketplaceEntitlementClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MarketplaceEntitlementClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MarketplaceEntitlementClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MarketplaceEntitlementClient {
        MarketplaceEntitlementClient { client, region }
    }
}

#[async_trait]
impl MarketplaceEntitlement for MarketplaceEntitlementClient {
    /// <p>GetEntitlements retrieves entitlement values for a given product. The results can be filtered based on customer identifier or product dimensions.</p>
    async fn get_entitlements(
        &self,
        input: GetEntitlementsRequest,
    ) -> Result<GetEntitlementsResult, RusotoError<GetEntitlementsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSMPEntitlementService.GetEntitlements");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetEntitlementsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetEntitlementsResult, _>()
    }
}
