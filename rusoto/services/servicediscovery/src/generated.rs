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

impl ServiceDiscoveryClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "servicediscovery", &self.region, request_uri);

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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateHttpNamespaceRequest {
    /// <p>A unique string that identifies the request and that allows failed <code>CreateHttpNamespace</code> requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string (for example, a date/time stamp).</p>
    #[serde(rename = "creatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>A description for the namespace.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name that you want to assign to this namespace.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The tags to add to the namespace. Each tag consists of a key and an optional value that you define. Tags keys can be up to 128 characters in length, and tag values can be up to 256 characters in length.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateHttpNamespaceResponse {
    /// <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_GetOperation.html">GetOperation</a>.</p>
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePrivateDnsNamespaceRequest {
    /// <p>A unique string that identifies the request and that allows failed <code>CreatePrivateDnsNamespace</code> requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string (for example, a date/timestamp).</p>
    #[serde(rename = "creatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>A description for the namespace.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name that you want to assign to this namespace. When you create a private DNS namespace, AWS Cloud Map automatically creates an Amazon Route 53 private hosted zone that has the same name as the namespace.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The tags to add to the namespace. Each tag consists of a key and an optional value that you define. Tags keys can be up to 128 characters in length, and tag values can be up to 256 characters in length.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The ID of the Amazon VPC that you want to associate the namespace with.</p>
    #[serde(rename = "vpc")]
    pub vpc: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePrivateDnsNamespaceResponse {
    /// <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_GetOperation.html">GetOperation</a>.</p>
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePublicDnsNamespaceRequest {
    /// <p>A unique string that identifies the request and that allows failed <code>CreatePublicDnsNamespace</code> requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string (for example, a date/timestamp).</p>
    #[serde(rename = "creatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>A description for the namespace.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name that you want to assign to this namespace.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The tags to add to the namespace. Each tag consists of a key and an optional value that you define. Tags keys can be up to 128 characters in length, and tag values can be up to 256 characters in length.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePublicDnsNamespaceResponse {
    /// <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_GetOperation.html">GetOperation</a>.</p>
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateServiceRequest {
    /// <p>A unique string that identifies the request and that allows failed <code>CreateService</code> requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string (for example, a date/timestamp).</p>
    #[serde(rename = "creatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>A description for the service.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A complex type that contains information about the Amazon Route 53 records that you want AWS Cloud Map to create when you register an instance. </p>
    #[serde(rename = "dnsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<DnsConfig>,
    /// <p> <i>Public DNS and HTTP namespaces only.</i> A complex type that contains settings for an optional Route 53 health check. If you specify settings for a health check, AWS Cloud Map associates the health check with all the Route 53 DNS records that you specify in <code>DnsConfig</code>.</p> <important> <p>If you specify a health check configuration, you can specify either <code>HealthCheckCustomConfig</code> or <code>HealthCheckConfig</code> but not both.</p> </important> <p>For information about the charges for health checks, see <a href="http://aws.amazon.com/cloud-map/pricing/">AWS Cloud Map Pricing</a>.</p>
    #[serde(rename = "healthCheckConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<HealthCheckConfig>,
    /// <p>A complex type that contains information about an optional custom health check.</p> <important> <p>If you specify a health check configuration, you can specify either <code>HealthCheckCustomConfig</code> or <code>HealthCheckConfig</code> but not both.</p> </important> <p>You can't add, update, or delete a <code>HealthCheckCustomConfig</code> configuration from an existing service.</p>
    #[serde(rename = "healthCheckCustomConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_custom_config: Option<HealthCheckCustomConfig>,
    /// <p><p>The name that you want to assign to the service.</p> <p>If you want AWS Cloud Map to create an <code>SRV</code> record when you register an instance and you&#39;re using a system that requires a specific <code>SRV</code> format, such as <a href="http://www.haproxy.org/">HAProxy</a>, specify the following for <code>Name</code>:</p> <ul> <li> <p>Start the name with an underscore (<em>), such as <code></em>exampleservice</code>.</p> </li> <li> <p>End the name with <i>.<em>protocol</i>, such as <code>.</em>tcp</code>.</p> </li> </ul> <p>When you register an instance, AWS Cloud Map creates an <code>SRV</code> record and assigns a name to the record by concatenating the service name and the namespace name (for example,</p> <p> <code><em>exampleservice.</em>tcp.example.com</code>).</p> <note> <p>For services that are accessible by DNS queries, you can&#39;t create multiple services with names that differ only by case (such as EXAMPLE and example). Otherwise, these services have the same DNS name and can&#39;t be distinguished. However, if you use a namespace that&#39;s only accessible by API calls, then you can create services that with names that differ only by case.</p> </note></p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ID of the namespace that you want to use to create the service. The namespace ID must be specified, but it can be specified either here or in the <code>DnsConfig</code> object.</p>
    #[serde(rename = "namespaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    /// <p>The tags to add to the service. Each tag consists of a key and an optional value that you define. Tags keys can be up to 128 characters in length, and tag values can be up to 256 characters in length.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>If present, specifies that the service instances are only discoverable using the <code>DiscoverInstances</code> API operation. No DNS records is registered for the service instances. The only valid value is <code>HTTP</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateServiceResponse {
    /// <p>A complex type that contains information about the new service.</p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteNamespaceRequest {
    /// <p>The ID of the namespace that you want to delete.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteNamespaceResponse {
    /// <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_GetOperation.html">GetOperation</a>.</p>
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteServiceRequest {
    /// <p>The ID of the service that you want to delete.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteServiceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeregisterInstanceRequest {
    /// <p>The value that you specified for <code>Id</code> in the <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html">RegisterInstance</a> request.</p>
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    /// <p>The ID of the service that the instance is associated with.</p>
    #[serde(rename = "serviceId")]
    pub service_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeregisterInstanceResponse {
    /// <p>A value that you can use to determine whether the request completed successfully. For more information, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_GetOperation.html">GetOperation</a>.</p>
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DiscoverInstancesRequest {
    /// <p><p>The health status of the instances that you want to discover. This parameter is ignored for services that don&#39;t have a health check configured, and all instances are returned.</p> <dl> <dt>HEALTHY</dt> <dd> <p>Returns healthy instances.</p> </dd> <dt>UNHEALTHY</dt> <dd> <p>Returns unhealthy instances.</p> </dd> <dt>ALL</dt> <dd> <p>Returns all instances.</p> </dd> <dt>HEALTHY<em>OR</em>ELSE_ALL</dt> <dd> <p>Returns healthy instances, unless none are reporting a healthy state. In that case, return all instances. This is also called failing open.</p> </dd> </dl></p>
    #[serde(rename = "healthStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    /// <p>The maximum number of instances that you want AWS Cloud Map to return in the response to a <code>DiscoverInstances</code> request. If you don't specify a value for <code>MaxResults</code>, AWS Cloud Map returns up to 100 instances.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>HttpName</code> name of the namespace. It's found in the <code>HttpProperties</code> member of the <code>Properties</code> member of the namespace.</p>
    #[serde(rename = "namespaceName")]
    pub namespace_name: String,
    /// <p>Opportunistic filters to scope the results based on custom attributes. If there are instances that match both the filters specified in both the <code>QueryParameters</code> parameter and this parameter, all of these instances are returned. Otherwise, the filters are ignored, and only instances that match the filters that are specified in the <code>QueryParameters</code> parameter are returned.</p>
    #[serde(rename = "optionalParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Filters to scope the results based on custom attributes for the instance (for example, <code>{version=v1, az=1a}</code>). Only instances that match all the specified key-value pairs are returned.</p>
    #[serde(rename = "queryParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the service that you specified when you registered the instance.</p>
    #[serde(rename = "serviceName")]
    pub service_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DiscoverInstancesResponse {
    /// <p>A complex type that contains one <code>HttpInstanceSummary</code> for each registered instance.</p>
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<HttpInstanceSummary>>,
}

/// <p>A complex type that contains information about the Amazon Route 53 DNS records that you want AWS Cloud Map to create when you register an instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DnsConfig {
    /// <p>An array that contains one <code>DnsRecord</code> object for each Route 53 DNS record that you want AWS Cloud Map to create when you register an instance.</p>
    #[serde(rename = "dnsRecords")]
    pub dns_records: Vec<DnsRecord>,
    /// <p><p>The routing policy that you want to apply to all Route 53 DNS records that AWS Cloud Map creates when you register an instance and specify this service.</p> <note> <p>If you want to use this service to register instances that create alias records, specify <code>WEIGHTED</code> for the routing policy.</p> </note> <p>You can specify the following values:</p> <dl> <dt>MULTIVALUE</dt> <dd> <p>If you define a health check for the service and the health check is healthy, Route 53 returns the applicable value for up to eight instances.</p> <p>For example, suppose that the service includes configurations for one <code>A</code> record and a health check. You use the service to register 10 instances. Route 53 responds to DNS queries with IP addresses for up to eight healthy instances. If fewer than eight instances are healthy, Route 53 responds to every DNS query with the IP addresses for all of the healthy instances.</p> <p>If you don&#39;t define a health check for the service, Route 53 assumes that all instances are healthy and returns the values for up to eight instances.</p> <p>For more information about the multivalue routing policy, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-policy.html#routing-policy-multivalue">Multivalue Answer Routing</a> in the <i>Route 53 Developer Guide</i>.</p> </dd> <dt>WEIGHTED</dt> <dd> <p>Route 53 returns the applicable value from one randomly selected instance from among the instances that you registered using the same service. Currently, all records have the same weight, so you can&#39;t route more or less traffic to any instances.</p> <p>For example, suppose that the service includes configurations for one <code>A</code> record and a health check. You use the service to register 10 instances. Route 53 responds to DNS queries with the IP address for one randomly selected instance from among the healthy instances. If no instances are healthy, Route 53 responds to DNS queries as if all of the instances were healthy.</p> <p>If you don&#39;t define a health check for the service, Route 53 assumes that all instances are healthy and returns the applicable value for one randomly selected instance.</p> <p>For more information about the weighted routing policy, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-policy.html#routing-policy-weighted">Weighted Routing</a> in the <i>Route 53 Developer Guide</i>.</p> </dd> </dl></p>
    #[serde(rename = "routingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy: Option<String>,
}

/// <p>A complex type that contains information about changes to the Route 53 DNS records that AWS Cloud Map creates when you register an instance.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DnsConfigChange {
    /// <p>An array that contains one <code>DnsRecord</code> object for each Route 53 record that you want AWS Cloud Map to create when you register an instance.</p>
    #[serde(rename = "dnsRecords")]
    pub dns_records: Vec<DnsRecord>,
}

/// <p>A complex type that contains the ID for the Route 53 hosted zone that AWS Cloud Map creates when you create a namespace.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DnsProperties {
    /// <p>The ID for the Route 53 hosted zone that AWS Cloud Map creates when you create a namespace.</p>
    #[serde(rename = "hostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
}

/// <p>A complex type that contains information about the Route 53 DNS records that you want AWS Cloud Map to create when you register an instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DnsRecord {
    /// <p><p>The amount of time, in seconds, that you want DNS resolvers to cache the settings for this record.</p> <note> <p>Alias records don&#39;t include a TTL because Route 53 uses the TTL for the AWS resource that an alias record routes traffic to. If you include the <code>AWS<em>ALIAS</em>DNS<em>NAME</code> attribute when you submit a &lt;a href=&quot;https://docs.aws.amazon.com/cloud-map/latest/api/API</em>RegisterInstance.html&quot;&gt;RegisterInstance</a> request, the <code>TTL</code> value is ignored. Always specify a TTL for the service; you can use a service to register instances that create either alias or non-alias records.</p> </note></p>
    #[serde(rename = "tTL")]
    pub ttl: i64,
    /// <p><p>The type of the resource, which indicates the type of value that Route 53 returns in response to DNS queries. You can specify values for <code>Type</code> in the following combinations:</p> <ul> <li> <p> <b> <code>A</code> </b> </p> </li> <li> <p> <b> <code>AAAA</code> </b> </p> </li> <li> <p> <b> <code>A</code> </b> and <b> <code>AAAA</code> </b> </p> </li> <li> <p> <b> <code>SRV</code> </b> </p> </li> <li> <p> <b> <code>CNAME</code> </b> </p> </li> </ul> <p>If you want AWS Cloud Map to create a Route 53 alias record when you register an instance, specify <code>A</code> or <code>AAAA</code> for <code>Type</code>.</p> <p>You specify other settings, such as the IP address for <code>A</code> and <code>AAAA</code> records, when you register an instance. For more information, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html">RegisterInstance</a>.</p> <p>The following values are supported:</p> <dl> <dt>A</dt> <dd> <p>Route 53 returns the IP address of the resource in IPv4 format, such as 192.0.2.44.</p> </dd> <dt>AAAA</dt> <dd> <p>Route 53 returns the IP address of the resource in IPv6 format, such as 2001:0db8:85a3:0000:0000:abcd:0001:2345.</p> </dd> <dt>CNAME</dt> <dd> <p>Route 53 returns the domain name of the resource, such as www.example.com. Note the following:</p> <ul> <li> <p>You specify the domain name that you want to route traffic to when you register an instance. For more information, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html#cloudmap-RegisterInstance-request-Attributes">Attributes</a> in the topic <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html">RegisterInstance</a>.</p> </li> <li> <p>You must specify <code>WEIGHTED</code> for the value of <code>RoutingPolicy</code>.</p> </li> <li> <p>You can&#39;t specify both <code>CNAME</code> for <code>Type</code> and settings for <code>HealthCheckConfig</code>. If you do, the request will fail with an <code>InvalidInput</code> error.</p> </li> </ul> </dd> <dt>SRV</dt> <dd> <p>Route 53 returns the value for an <code>SRV</code> record. The value for an <code>SRV</code> record uses the following values:</p> <p> <code>priority weight port service-hostname</code> </p> <p>Note the following about the values:</p> <ul> <li> <p>The values of <code>priority</code> and <code>weight</code> are both set to <code>1</code> and can&#39;t be changed. </p> </li> <li> <p>The value of <code>port</code> comes from the value that you specify for the <code>AWS<em>INSTANCE</em>PORT</code> attribute when you submit a <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html">RegisterInstance</a> request. </p> </li> <li> <p>The value of <code>service-hostname</code> is a concatenation of the following values:</p> <ul> <li> <p>The value that you specify for <code>InstanceId</code> when you register an instance.</p> </li> <li> <p>The name of the service.</p> </li> <li> <p>The name of the namespace. </p> </li> </ul> <p>For example, if the value of <code>InstanceId</code> is <code>test</code>, the name of the service is <code>backend</code>, and the name of the namespace is <code>example.com</code>, the value of <code>service-hostname</code> is the following:</p> <p> <code>test.backend.example.com</code> </p> </li> </ul> <p>If you specify settings for an <code>SRV</code> record, note the following:</p> <ul> <li> <p>If you specify values for <code>AWS<em>INSTANCE</em>IPV4</code>, <code>AWS<em>INSTANCE</em>IPV6</code>, or both in the <code>RegisterInstance</code> request, AWS Cloud Map automatically creates <code>A</code> and/or <code>AAAA</code> records that have the same name as the value of <code>service-hostname</code> in the <code>SRV</code> record. You can ignore these records.</p> </li> <li> <p>If you&#39;re using a system that requires a specific <code>SRV</code> format, such as HAProxy, see the <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_CreateService.html#cloudmap-CreateService-request-Name">Name</a> element in the documentation about <code>CreateService</code> for information about how to specify the correct name format.</p> </li> </ul> </dd> </dl></p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInstanceRequest {
    /// <p>The ID of the instance that you want to get information about.</p>
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    /// <p>The ID of the service that the instance is associated with.</p>
    #[serde(rename = "serviceId")]
    pub service_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInstanceResponse {
    /// <p>A complex type that contains information about a specified instance.</p>
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<Instance>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetInstancesHealthStatusRequest {
    /// <p><p>An array that contains the IDs of all the instances that you want to get the health status for.</p> <p>If you omit <code>Instances</code>, AWS Cloud Map returns the health status for all the instances that are associated with the specified service.</p> <note> <p>To get the IDs for the instances that you&#39;ve registered by using a specified service, submit a <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_ListInstances.html">ListInstances</a> request.</p> </note></p>
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,
    /// <p>The maximum number of instances that you want AWS Cloud Map to return in the response to a <code>GetInstancesHealthStatus</code> request. If you don't specify a value for <code>MaxResults</code>, AWS Cloud Map returns up to 100 instances.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For the first <code>GetInstancesHealthStatus</code> request, omit this value.</p> <p>If more than <code>MaxResults</code> instances match the specified criteria, you can submit another <code>GetInstancesHealthStatus</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the service that the instance is associated with.</p>
    #[serde(rename = "serviceId")]
    pub service_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetInstancesHealthStatusResponse {
    /// <p>If more than <code>MaxResults</code> instances match the specified criteria, you can submit another <code>GetInstancesHealthStatus</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A complex type that contains the IDs and the health status of the instances that you specified in the <code>GetInstancesHealthStatus</code> request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetNamespaceRequest {
    /// <p>The ID of the namespace that you want to get information about.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetNamespaceResponse {
    /// <p>A complex type that contains information about the specified namespace.</p>
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetOperationRequest {
    /// <p>The ID of the operation that you want to get more information about.</p>
    #[serde(rename = "operationId")]
    pub operation_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetOperationResponse {
    /// <p>A complex type that contains information about the operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetServiceRequest {
    /// <p>The ID of the service that you want to get settings for.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetServiceResponse {
    /// <p>A complex type that contains information about the service.</p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

/// <p><p> <i>Public DNS and HTTP namespaces only.</i> A complex type that contains settings for an optional health check. If you specify settings for a health check, AWS Cloud Map associates the health check with the records that you specify in <code>DnsConfig</code>.</p> <important> <p>If you specify a health check configuration, you can specify either <code>HealthCheckCustomConfig</code> or <code>HealthCheckConfig</code> but not both.</p> </important> <p>Health checks are basic Route 53 health checks that monitor an AWS endpoint. For information about pricing for health checks, see <a href="http://aws.amazon.com/route53/pricing/">Amazon Route 53 Pricing</a>.</p> <p>Note the following about configuring health checks.</p> <dl> <dt>A and AAAA records</dt> <dd> <p>If <code>DnsConfig</code> includes configurations for both <code>A</code> and <code>AAAA</code> records, AWS Cloud Map creates a health check that uses the IPv4 address to check the health of the resource. If the endpoint tthat&#39;s specified by the IPv4 address is unhealthy, Route 53 considers both the <code>A</code> and <code>AAAA</code> records to be unhealthy. </p> </dd> <dt>CNAME records</dt> <dd> <p>You can&#39;t specify settings for <code>HealthCheckConfig</code> when the <code>DNSConfig</code> includes <code>CNAME</code> for the value of <code>Type</code>. If you do, the <code>CreateService</code> request will fail with an <code>InvalidInput</code> error.</p> </dd> <dt>Request interval</dt> <dd> <p>A Route 53 health checker in each health-checking AWS Region sends a health check request to an endpoint every 30 seconds. On average, your endpoint receives a health check request about every two seconds. However, health checkers don&#39;t coordinate with one another. Therefore, you might sometimes see several requests in one second that&#39;s followed by a few seconds with no health checks at all.</p> </dd> <dt>Health checking regions</dt> <dd> <p>Health checkers perform checks from all Route 53 health-checking Regions. For a list of the current Regions, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_HealthCheckConfig.html#Route53-Type-HealthCheckConfig-Regions">Regions</a>.</p> </dd> <dt>Alias records</dt> <dd> <p>When you register an instance, if you include the <code>AWS<em>ALIAS</em>DNS<em>NAME</code> attribute, AWS Cloud Map creates a Route 53 alias record. Note the following:</p> <ul> <li> <p>Route 53 automatically sets <code>EvaluateTargetHealth</code> to true for alias records. When <code>EvaluateTargetHealth</code> is true, the alias record inherits the health of the referenced AWS resource. such as an ELB load balancer. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/Route53/latest/APIReference/API</em>AliasTarget.html#Route53-Type-AliasTarget-EvaluateTargetHealth&quot;&gt;EvaluateTargetHealth</a>.</p> </li> <li> <p>If you include <code>HealthCheckConfig</code> and then use the service to register an instance that creates an alias record, Route 53 doesn&#39;t create the health check.</p> </li> </ul> </dd> <dt>Charges for health checks</dt> <dd> <p>Health checks are basic Route 53 health checks that monitor an AWS endpoint. For information about pricing for health checks, see <a href="http://aws.amazon.com/route53/pricing/">Amazon Route 53 Pricing</a>.</p> </dd> </dl></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HealthCheckConfig {
    /// <p>The number of consecutive health checks that an endpoint must pass or fail for Route 53 to change the current status of the endpoint from unhealthy to healthy or the other way around. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-determining-health-of-endpoints.html">How Route 53 Determines Whether an Endpoint Is Healthy</a> in the <i>Route 53 Developer Guide</i>.</p>
    #[serde(rename = "failureThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i64>,
    /// <p>The path that you want Route 53 to request when performing health checks. The path can be any value that your endpoint returns an HTTP status code of a 2xx or 3xx format for when the endpoint is healthy. An example file is <code>/docs/route53-health-check.html</code>. Route 53 automatically adds the DNS name for the service. If you don't specify a value for <code>ResourcePath</code>, the default value is <code>/</code>.</p> <p>If you specify <code>TCP</code> for <code>Type</code>, you must <i>not</i> specify a value for <code>ResourcePath</code>.</p>
    #[serde(rename = "resourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,
    /// <p>The type of health check that you want to create, which indicates how Route 53 determines whether an endpoint is healthy.</p> <important> <p>You can't change the value of <code>Type</code> after you create a health check.</p> </important> <p>You can create the following types of health checks:</p> <ul> <li> <p> <b>HTTP</b>: Route 53 tries to establish a TCP connection. If successful, Route 53 submits an HTTP request and waits for an HTTP status code of 200 or greater and less than 400.</p> </li> <li> <p> <b>HTTPS</b>: Route 53 tries to establish a TCP connection. If successful, Route 53 submits an HTTPS request and waits for an HTTP status code of 200 or greater and less than 400.</p> <important> <p>If you specify HTTPS for the value of <code>Type</code>, the endpoint must support TLS v1.0 or later.</p> </important> </li> <li> <p> <b>TCP</b>: Route 53 tries to establish a TCP connection.</p> <p>If you specify <code>TCP</code> for <code>Type</code>, don't specify a value for <code>ResourcePath</code>.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-failover-determining-health-of-endpoints.html">How Route 53 Determines Whether an Endpoint Is Healthy</a> in the <i>Route 53 Developer Guide</i>.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p><p>A complex type that contains information about an optional custom health check. A custom health check, which requires that you use a third-party health checker to evaluate the health of your resources, is useful in the following circumstances:</p> <ul> <li> <p>You can&#39;t use a health check that&#39;s defined by <code>HealthCheckConfig</code> because the resource isn&#39;t available over the internet. For example, you can use a custom health check when the instance is in an Amazon VPC. (To check the health of resources in a VPC, the health checker must also be in the VPC.)</p> </li> <li> <p>You want to use a third-party health checker regardless of where your resources are located.</p> </li> </ul> <important> <p>If you specify a health check configuration, you can specify either <code>HealthCheckCustomConfig</code> or <code>HealthCheckConfig</code> but not both.</p> </important> <p>To change the status of a custom health check, submit an <code>UpdateInstanceCustomHealthStatus</code> request. AWS Cloud Map doesn&#39;t monitor the status of the resource, it just keeps a record of the status specified in the most recent <code>UpdateInstanceCustomHealthStatus</code> request.</p> <p>Here&#39;s how custom health checks work:</p> <ol> <li> <p>You create a service.</p> </li> <li> <p>You register an instance.</p> </li> <li> <p>You configure a third-party health checker to monitor the resource that&#39;s associated with the new instance. </p> <note> <p>AWS Cloud Map doesn&#39;t check the health of the resource directly. </p> </note> </li> <li> <p>The third-party health-checker determines that the resource is unhealthy and notifies your application.</p> </li> <li> <p>Your application submits an <code>UpdateInstanceCustomHealthStatus</code> request.</p> </li> <li> <p>AWS Cloud Map waits for 30 seconds.</p> </li> <li> <p>If another <code>UpdateInstanceCustomHealthStatus</code> request doesn&#39;t arrive during that time to change the status back to healthy, AWS Cloud Map stops routing traffic to the resource.</p> </li> </ol></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HealthCheckCustomConfig {}

/// <p>In a response to a <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_DiscoverInstances.html">DiscoverInstances</a> request, <code>HttpInstanceSummary</code> contains information about one instance that matches the values that you specified in the request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HttpInstanceSummary {
    /// <p>If you included any attributes when you registered the instance, the values of those attributes.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>If you configured health checking in the service, the current health status of the service instance.</p>
    #[serde(rename = "healthStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    /// <p>The ID of an instance that matches the values that you specified in the request.</p>
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// <p> <code/> <code/> <code/> </p> <p>The <code>HttpName</code> name of the namespace. It's found in the <code>HttpProperties</code> member of the <code>Properties</code> member of the namespace.</p>
    #[serde(rename = "namespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// <p>The name of the service that you specified when you registered the instance.</p>
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

/// <p>A complex type that contains the name of an HTTP namespace.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HttpProperties {
    /// <p>The name of an HTTP namespace.</p>
    #[serde(rename = "httpName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_name: Option<String>,
}

/// <p>A complex type that contains information about an instance that AWS Cloud Map creates when you submit a <code>RegisterInstance</code> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Instance {
    /// <p><p>A string map that contains the following information for the service that you specify in <code>ServiceId</code>:</p> <ul> <li> <p>The attributes that apply to the records that are defined in the service. </p> </li> <li> <p>For each attribute, the applicable value.</p> </li> </ul> <p>Supported attribute keys include the following:</p> <dl> <dt>AWS<em>ALIAS</em>DNS<em>NAME</dt> <dd> <p>If you want AWS Cloud Map to create a Route 53 alias record that routes traffic to an Elastic Load Balancing load balancer, specify the DNS name that&#39;s associated with the load balancer. For information about how to get the DNS name, see &lt;a href=&quot;https://docs.aws.amazon.com/Route53/latest/APIReference/API</em>AliasTarget.html#Route53-Type-AliasTarget-DNSName&quot;&gt;AliasTarget-&gt;DNSName</a> in the <i>Route 53 API Reference</i>.</p> <p>Note the following:</p> <ul> <li> <p>The configuration for the service that&#39;s specified by <code>ServiceId</code> must include settings for an <code>A</code> record, an <code>AAAA</code> record, or both.</p> </li> <li> <p>In the service that&#39;s specified by <code>ServiceId</code>, the value of <code>RoutingPolicy</code> must be <code>WEIGHTED</code>.</p> </li> <li> <p>If the service that&#39;s specified by <code>ServiceId</code> includes <code>HealthCheckConfig</code> settings, AWS Cloud Map creates the health check, but it won&#39;t associate the health check with the alias record.</p> </li> <li> <p>Auto naming currently doesn&#39;t support creating alias records that route traffic to AWS resources other than ELB load balancers.</p> </li> <li> <p>If you specify a value for <code>AWS<em>ALIAS</em>DNS<em>NAME</code>, don&#39;t specify values for any of the <code>AWS</em>INSTANCE</code> attributes.</p> </li> </ul> </dd> <dt>AWS<em>EC2</em>INSTANCE<em>ID</dt> <dd> <p> <i>HTTP namespaces only.</i> The Amazon EC2 instance ID for the instance. The <code>AWS</em>INSTANCE<em>IPV4</code> attribute contains the primary private IPv4 address.</p> </dd> <dt>AWS</em>INIT<em>HEALTH</em>STATUS</dt> <dd> <p>If the service configuration includes <code>HealthCheckCustomConfig</code>, you can optionally use <code>AWS<em>INIT</em>HEALTH<em>STATUS</code> to specify the initial status of the custom health check, <code>HEALTHY</code> or <code>UNHEALTHY</code>. If you don&#39;t specify a value for <code>AWS</em>INIT<em>HEALTH</em>STATUS</code>, the initial status is <code>HEALTHY</code>.</p> </dd> <dt>AWS<em>INSTANCE</em>CNAME</dt> <dd> <p>If the service configuration includes a <code>CNAME</code> record, the domain name that you want Route 53 to return in response to DNS queries (for example, <code>example.com</code>).</p> <p>This value is required if the service specified by <code>ServiceId</code> includes settings for an <code>CNAME</code> record.</p> </dd> <dt>AWS<em>INSTANCE</em>IPV4</dt> <dd> <p>If the service configuration includes an <code>A</code> record, the IPv4 address that you want Route 53 to return in response to DNS queries (for example, <code>192.0.2.44</code>).</p> <p>This value is required if the service specified by <code>ServiceId</code> includes settings for an <code>A</code> record. If the service includes settings for an <code>SRV</code> record, you must specify a value for <code>AWS<em>INSTANCE</em>IPV4</code>, <code>AWS<em>INSTANCE</em>IPV6</code>, or both.</p> </dd> <dt>AWS<em>INSTANCE</em>IPV6</dt> <dd> <p>If the service configuration includes an <code>AAAA</code> record, the IPv6 address that you want Route 53 to return in response to DNS queries (for example, <code>2001:0db8:85a3:0000:0000:abcd:0001:2345</code>).</p> <p>This value is required if the service specified by <code>ServiceId</code> includes settings for an <code>AAAA</code> record. If the service includes settings for an <code>SRV</code> record, you must specify a value for <code>AWS<em>INSTANCE</em>IPV4</code>, <code>AWS<em>INSTANCE</em>IPV6</code>, or both.</p> </dd> <dt>AWS<em>INSTANCE</em>PORT</dt> <dd> <p>If the service includes an <code>SRV</code> record, the value that you want Route 53 to return for the port.</p> <p>If the service includes <code>HealthCheckConfig</code>, the port on the endpoint that you want Route 53 to send requests to. </p> <p>This value is required if you specified settings for an <code>SRV</code> record or a Route 53 health check when you created the service.</p> </dd> </dl></p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique string that identifies the request and that allows failed <code>RegisterInstance</code> requests to be retried without the risk of executing the operation twice. You must use a unique <code>CreatorRequestId</code> string every time you submit a <code>RegisterInstance</code> request if you're registering additional instances for the same namespace and service. <code>CreatorRequestId</code> can be any unique string (for example, a date/time stamp).</p>
    #[serde(rename = "creatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p><p>An identifier that you want to associate with the instance. Note the following:</p> <ul> <li> <p>If the service that&#39;s specified by <code>ServiceId</code> includes settings for an <code>SRV</code> record, the value of <code>InstanceId</code> is automatically included as part of the value for the <code>SRV</code> record. For more information, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_DnsRecord.html#cloudmap-Type-DnsRecord-Type">DnsRecord &gt; Type</a>.</p> </li> <li> <p>You can use this value to update an existing instance.</p> </li> <li> <p>To register a new instance, you must specify a value that&#39;s unique among instances that you register by using the same service. </p> </li> <li> <p>If you specify an existing <code>InstanceId</code> and <code>ServiceId</code>, AWS Cloud Map updates the existing DNS records. If there&#39;s also an existing health check, AWS Cloud Map deletes the old health check and creates a new one. </p> <note> <p>The health check isn&#39;t deleted immediately, so it will still appear for a while if you submit a <code>ListHealthChecks</code> request, for example.</p> </note> </li> </ul></p>
    #[serde(rename = "id")]
    pub id: String,
}

/// <p>A complex type that contains information about the instances that you registered by using a specified service.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InstanceSummary {
    /// <p><p>A string map that contains the following information:</p> <ul> <li> <p>The attributes that are associated with the instance. </p> </li> <li> <p>For each attribute, the applicable value.</p> </li> </ul> <p>Supported attribute keys include the following:</p> <dl> <dt>AWS<em>ALIAS</em>DNS<em>NAME</dt> <dd> <p>For an alias record that routes traffic to an Elastic Load Balancing load balancer, the DNS name that&#39;s associated with the load balancer. </p> </dd> <dt>AWS</em>EC2<em>INSTANCE</em>ID (HTTP namespaces only)</dt> <dd> <p>The Amazon EC2 instance ID for the instance. When the <code>AWS<em>EC2</em>INSTANCE<em>ID</code> attribute is specified, then the <code>AWS</em>INSTANCE<em>IPV4</code> attribute contains the primary private IPv4 address.</p> </dd> <dt>AWS</em>INIT<em>HEALTH</em>STATUS</dt> <dd> <p>If the service configuration includes <code>HealthCheckCustomConfig</code>, you can optionally use <code>AWS<em>INIT</em>HEALTH<em>STATUS</code> to specify the initial status of the custom health check, <code>HEALTHY</code> or <code>UNHEALTHY</code>. If you don&#39;t specify a value for <code>AWS</em>INIT<em>HEALTH</em>STATUS</code>, the initial status is <code>HEALTHY</code>.</p> </dd> <dt>AWS<em>INSTANCE</em>CNAME</dt> <dd> <p>For a <code>CNAME</code> record, the domain name that Route 53 returns in response to DNS queries (for example, <code>example.com</code>).</p> </dd> <dt>AWS<em>INSTANCE</em>IPV4</dt> <dd> <p>For an <code>A</code> record, the IPv4 address that Route 53 returns in response to DNS queries (for example, <code>192.0.2.44</code>).</p> </dd> <dt>AWS<em>INSTANCE</em>IPV6</dt> <dd> <p>For an <code>AAAA</code> record, the IPv6 address that Route 53 returns in response to DNS queries (for example, <code>2001:0db8:85a3:0000:0000:abcd:0001:2345</code>).</p> </dd> <dt>AWS<em>INSTANCE</em>PORT</dt> <dd> <p>For an <code>SRV</code> record, the value that Route 53 returns for the port. In addition, if the service includes <code>HealthCheckConfig</code>, the port on the endpoint that Route 53 sends requests to.</p> </dd> </dl></p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The ID for an instance that you created by using a specified service.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInstancesRequest {
    /// <p>The maximum number of instances that you want AWS Cloud Map to return in the response to a <code>ListInstances</code> request. If you don't specify a value for <code>MaxResults</code>, AWS Cloud Map returns up to 100 instances.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>For the first <code>ListInstances</code> request, omit this value.</p> <p>If more than <code>MaxResults</code> instances match the specified criteria, you can submit another <code>ListInstances</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the service that you want to list instances for.</p>
    #[serde(rename = "serviceId")]
    pub service_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInstancesResponse {
    /// <p>Summary information about the instances that are associated with the specified service.</p>
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<InstanceSummary>>,
    /// <p>If more than <code>MaxResults</code> instances match the specified criteria, you can submit another <code>ListInstances</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListNamespacesRequest {
    /// <p>A complex type that contains specifications for the namespaces that you want to list.</p> <p>If you specify more than one filter, a namespace must match all filters to be returned by <code>ListNamespaces</code>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<NamespaceFilter>>,
    /// <p>The maximum number of namespaces that you want AWS Cloud Map to return in the response to a <code>ListNamespaces</code> request. If you don't specify a value for <code>MaxResults</code>, AWS Cloud Map returns up to 100 namespaces.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>For the first <code>ListNamespaces</code> request, omit this value.</p> <p>If the response contains <code>NextToken</code>, submit another <code>ListNamespaces</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p> <note> <p>AWS Cloud Map gets <code>MaxResults</code> namespaces and then filters them based on the specified criteria. It&#39;s possible that no namespaces in the first <code>MaxResults</code> namespaces matched the specified criteria but that subsequent groups of <code>MaxResults</code> namespaces do contain namespaces that match the criteria.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListNamespacesResponse {
    /// <p>An array that contains one <code>NamespaceSummary</code> object for each namespace that matches the specified filter criteria.</p>
    #[serde(rename = "namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<NamespaceSummary>>,
    /// <p><p>If the response contains <code>NextToken</code>, submit another <code>ListNamespaces</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p> <note> <p>AWS Cloud Map gets <code>MaxResults</code> namespaces and then filters them based on the specified criteria. It&#39;s possible that no namespaces in the first <code>MaxResults</code> namespaces matched the specified criteria but that subsequent groups of <code>MaxResults</code> namespaces do contain namespaces that match the criteria.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOperationsRequest {
    /// <p>A complex type that contains specifications for the operations that you want to list, for example, operations that you started between a specified start date and end date.</p> <p>If you specify more than one filter, an operation must match all filters to be returned by <code>ListOperations</code>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<OperationFilter>>,
    /// <p>The maximum number of items that you want AWS Cloud Map to return in the response to a <code>ListOperations</code> request. If you don't specify a value for <code>MaxResults</code>, AWS Cloud Map returns up to 100 operations.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>For the first <code>ListOperations</code> request, omit this value.</p> <p>If the response contains <code>NextToken</code>, submit another <code>ListOperations</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p> <note> <p>AWS Cloud Map gets <code>MaxResults</code> operations and then filters them based on the specified criteria. It&#39;s possible that no operations in the first <code>MaxResults</code> operations matched the specified criteria but that subsequent groups of <code>MaxResults</code> operations do contain operations that match the criteria.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOperationsResponse {
    /// <p><p>If the response contains <code>NextToken</code>, submit another <code>ListOperations</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p> <note> <p>AWS Cloud Map gets <code>MaxResults</code> operations and then filters them based on the specified criteria. It&#39;s possible that no operations in the first <code>MaxResults</code> operations matched the specified criteria but that subsequent groups of <code>MaxResults</code> operations do contain operations that match the criteria.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Summary information about the operations that match the specified criteria.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<OperationSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListServicesRequest {
    /// <p>A complex type that contains specifications for the namespaces that you want to list services for. </p> <p>If you specify more than one filter, an operation must match all filters to be returned by <code>ListServices</code>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ServiceFilter>>,
    /// <p>The maximum number of services that you want AWS Cloud Map to return in the response to a <code>ListServices</code> request. If you don't specify a value for <code>MaxResults</code>, AWS Cloud Map returns up to 100 services.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>For the first <code>ListServices</code> request, omit this value.</p> <p>If the response contains <code>NextToken</code>, submit another <code>ListServices</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p> <note> <p>AWS Cloud Map gets <code>MaxResults</code> services and then filters them based on the specified criteria. It&#39;s possible that no services in the first <code>MaxResults</code> services matched the specified criteria but that subsequent groups of <code>MaxResults</code> services do contain services that match the criteria.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListServicesResponse {
    /// <p><p>If the response contains <code>NextToken</code>, submit another <code>ListServices</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p> <note> <p>AWS Cloud Map gets <code>MaxResults</code> services and then filters them based on the specified criteria. It&#39;s possible that no services in the first <code>MaxResults</code> services matched the specified criteria but that subsequent groups of <code>MaxResults</code> services do contain services that match the criteria.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array that contains one <code>ServiceSummary</code> object for each service that matches the specified filter criteria.</p>
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServiceSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to retrieve tags for.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags that are assigned to the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>A complex type that contains information about a specified namespace.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Namespace {
    /// <p>The Amazon Resource Name (ARN) that AWS Cloud Map assigns to the namespace when you create it.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date that the namespace was created, in Unix date/time format and Coordinated Universal Time (UTC). The value of <code>CreateDate</code> is accurate to milliseconds. For example, the value <code>1516925490.087</code> represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "createDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of running an operation twice. </p>
    #[serde(rename = "creatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>The description that you specify for the namespace when you create it.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of a namespace.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the namespace, such as <code>example.com</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A complex type that contains information that's specific to the type of the namespace.</p>
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
    /// <p>The number of services that are associated with the namespace.</p>
    #[serde(rename = "serviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_count: Option<i64>,
    /// <p><p>The type of the namespace. The methods for discovering instances depends on the value that you specify:</p> <dl> <dt>HTTP</dt> <dd> <p>Instances can be discovered only programmatically, using the AWS Cloud Map <code>DiscoverInstances</code> API.</p> </dd> <dt>DNS<em>PUBLIC</dt> <dd> <p>Instances can be discovered using public DNS queries and using the <code>DiscoverInstances</code> API.</p> </dd> <dt>DNS</em>PRIVATE</dt> <dd> <p>Instances can be discovered using DNS queries in VPCs and using the <code>DiscoverInstances</code> API.</p> </dd> </dl></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A complex type that identifies the namespaces that you want to list. You can choose to list public or private namespaces.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NamespaceFilter {
    /// <p><p>The operator that you want to use to determine whether <code>ListNamespaces</code> returns a namespace. Valid values for <code>condition</code> include:</p> <dl> <dt>EQ</dt> <dd> <p>When you specify <code>EQ</code> for the condition, you can choose to list only public namespaces or private namespaces, but not both. <code>EQ</code> is the default condition and can be omitted.</p> </dd> <dt>IN</dt> <dd> <p>When you specify <code>IN</code> for the condition, you can choose to list public namespaces, private namespaces, or both. </p> </dd> <dt>BETWEEN</dt> <dd> <p> Not applicable</p> </dd> </dl></p>
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// <p>Specify <code>TYPE</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>If you specify <code>EQ</code> for <code>Condition</code>, specify either <code>DNS_PUBLIC</code> or <code>DNS_PRIVATE</code>.</p> <p>If you specify <code>IN</code> for <code>Condition</code>, you can specify <code>DNS_PUBLIC</code>, <code>DNS_PRIVATE</code>, or both.</p>
    #[serde(rename = "values")]
    pub values: Vec<String>,
}

/// <p>A complex type that contains information that's specific to the namespace type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NamespaceProperties {
    /// <p>A complex type that contains the ID for the Route 53 hosted zone that AWS Cloud Map creates when you create a namespace.</p>
    #[serde(rename = "dnsProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_properties: Option<DnsProperties>,
    /// <p>A complex type that contains the name of an HTTP namespace.</p>
    #[serde(rename = "httpProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_properties: Option<HttpProperties>,
}

/// <p>A complex type that contains information about a namespace.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NamespaceSummary {
    /// <p>The Amazon Resource Name (ARN) that AWS Cloud Map assigns to the namespace when you create it.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the namespace was created.</p>
    #[serde(rename = "createDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    /// <p>A description for the namespace.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the namespace.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the namespace. When you create a namespace, AWS Cloud Map automatically creates a Route 53 hosted zone that has the same name as the namespace.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The properties of the namespace.</p>
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
    /// <p>The number of services that were created using the namespace.</p>
    #[serde(rename = "serviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_count: Option<i64>,
    /// <p>The type of the namespace, either public or private.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A complex type that contains information about a specified operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Operation {
    /// <p>The date and time that the request was submitted, in Unix date/time format and Coordinated Universal Time (UTC). The value of <code>CreateDate</code> is accurate to milliseconds. For example, the value <code>1516925490.087</code> represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "createDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    /// <p><p>The code associated with <code>ErrorMessage</code>. Values for <code>ErrorCode</code> include the following:</p> <ul> <li> <p> <code>ACCESS<em>DENIED</code> </p> </li> <li> <p> <code>CANNOT</em>CREATE<em>HOSTED</em>ZONE</code> </p> </li> <li> <p> <code>EXPIRED<em>TOKEN</code> </p> </li> <li> <p> <code>HOSTED</em>ZONE<em>NOT</em>FOUND</code> </p> </li> <li> <p> <code>INTERNAL<em>FAILURE</code> </p> </li> <li> <p> <code>INVALID</em>CHANGE<em>BATCH</code> </p> </li> <li> <p> <code>THROTTLED</em>REQUEST</code> </p> </li> </ul></p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>If the value of <code>Status</code> is <code>FAIL</code>, the reason that the operation failed.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the operation that you want to get information about.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p><p>The status of the operation. Values include the following:</p> <dl> <dt>SUBMITTED</dt> <dd> <p>This is the initial state that occurs immediately after you submit a request.</p> </dd> <dt>PENDING</dt> <dd> <p>AWS Cloud Map is performing the operation.</p> </dd> <dt>SUCCESS</dt> <dd> <p>The operation succeeded.</p> </dd> <dt>FAIL</dt> <dd> <p>The operation failed. For the failure reason, see <code>ErrorMessage</code>.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>The name of the target entity that&#39;s associated with the operation:</p> <dl> <dt>NAMESPACE</dt> <dd> <p>The namespace ID is returned in the <code>ResourceId</code> property.</p> </dd> <dt>SERVICE</dt> <dd> <p>The service ID is returned in the <code>ResourceId</code> property.</p> </dd> <dt>INSTANCE</dt> <dd> <p>The instance ID is returned in the <code>ResourceId</code> property.</p> </dd> </dl></p>
    #[serde(rename = "targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<::std::collections::HashMap<String, String>>,
    /// <p>The name of the operation that's associated with the specified ID.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The date and time that the value of <code>Status</code> changed to the current value, in Unix date/time format and Coordinated Universal Time (UTC). The value of <code>UpdateDate</code> is accurate to milliseconds. For example, the value <code>1516925490.087</code> represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "updateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<f64>,
}

/// <p>A complex type that lets you select the operations that you want to list.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OperationFilter {
    /// <p><p>The operator that you want to use to determine whether an operation matches the specified value. Valid values for condition include:</p> <ul> <li> <p> <code>EQ</code>: When you specify <code>EQ</code> for the condition, you can specify only one value. <code>EQ</code> is supported for <code>NAMESPACE<em>ID</code>, <code>SERVICE</em>ID</code>, <code>STATUS</code>, and <code>TYPE</code>. <code>EQ</code> is the default condition and can be omitted.</p> </li> <li> <p> <code>IN</code>: When you specify <code>IN</code> for the condition, you can specify a list of one or more values. <code>IN</code> is supported for <code>STATUS</code> and <code>TYPE</code>. An operation must match one of the specified values to be returned in the response.</p> </li> <li> <p> <code>BETWEEN</code>: Specify a start date and an end date in Unix date/time format and Coordinated Universal Time (UTC). The start date must be the first value. <code>BETWEEN</code> is supported for <code>UPDATE_DATE</code>. </p> </li> </ul></p>
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// <p><p>Specify the operations that you want to get:</p> <ul> <li> <p> <b>NAMESPACE<em>ID</b>: Gets operations related to specified namespaces.</p> </li> <li> <p> <b>SERVICE</em>ID</b>: Gets operations related to specified services.</p> </li> <li> <p> <b>STATUS</b>: Gets operations based on the status of the operations: <code>SUBMITTED</code>, <code>PENDING</code>, <code>SUCCEED</code>, or <code>FAIL</code>.</p> </li> <li> <p> <b>TYPE</b>: Gets specified types of operation.</p> </li> <li> <p> <b>UPDATE_DATE</b>: Gets operations that changed status during a specified date/time range. </p> </li> </ul></p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p>Specify values that are applicable to the value that you specify for <code>Name</code>: </p> <ul> <li> <p> <b>NAMESPACE<em>ID</b>: Specify one namespace ID.</p> </li> <li> <p> <b>SERVICE</em>ID</b>: Specify one service ID.</p> </li> <li> <p> <b>STATUS</b>: Specify one or more statuses: <code>SUBMITTED</code>, <code>PENDING</code>, <code>SUCCEED</code>, or <code>FAIL</code>.</p> </li> <li> <p> <b>TYPE</b>: Specify one or more of the following types: <code>CREATE<em>NAMESPACE</code>, <code>DELETE</em>NAMESPACE</code>, <code>UPDATE<em>SERVICE</code>, <code>REGISTER</em>INSTANCE</code>, or <code>DEREGISTER<em>INSTANCE</code>.</p> </li> <li> <p> <b>UPDATE</em>DATE</b>: Specify a start date and an end date in Unix date/time format and Coordinated Universal Time (UTC). The start date must be the first value.</p> </li> </ul></p>
    #[serde(rename = "values")]
    pub values: Vec<String>,
}

/// <p>A complex type that contains information about an operation that matches the criteria that you specified in a <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_ListOperations.html">ListOperations</a> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OperationSummary {
    /// <p>The ID for an operation.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p><p>The status of the operation. Values include the following:</p> <ul> <li> <p> <b>SUBMITTED</b>: This is the initial state immediately after you submit a request.</p> </li> <li> <p> <b>PENDING</b>: AWS Cloud Map is performing the operation.</p> </li> <li> <p> <b>SUCCESS</b>: The operation succeeded.</p> </li> <li> <p> <b>FAIL</b>: The operation failed. For the failure reason, see <code>ErrorMessage</code>.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegisterInstanceRequest {
    /// <p><p>A string map that contains the following information for the service that you specify in <code>ServiceId</code>:</p> <ul> <li> <p>The attributes that apply to the records that are defined in the service. </p> </li> <li> <p>For each attribute, the applicable value.</p> </li> </ul> <p>Supported attribute keys include the following:</p> <dl> <dt>AWS<em>ALIAS</em>DNS<em>NAME</dt> <dd> <p>If you want AWS Cloud Map to create an Amazon Route 53 alias record that routes traffic to an Elastic Load Balancing load balancer, specify the DNS name that&#39;s associated with the load balancer. For information about how to get the DNS name, see &quot;DNSName&quot; in the topic &lt;a href=&quot;https://docs.aws.amazon.com/Route53/latest/APIReference/API</em>AliasTarget.html&quot;&gt;AliasTarget</a> in the <i>Route 53 API Reference</i>.</p> <p>Note the following:</p> <ul> <li> <p>The configuration for the service that&#39;s specified by <code>ServiceId</code> must include settings for an <code>A</code> record, an <code>AAAA</code> record, or both.</p> </li> <li> <p>In the service that&#39;s specified by <code>ServiceId</code>, the value of <code>RoutingPolicy</code> must be <code>WEIGHTED</code>.</p> </li> <li> <p>If the service that&#39;s specified by <code>ServiceId</code> includes <code>HealthCheckConfig</code> settings, AWS Cloud Map will create the Route 53 health check, but it doesn&#39;t associate the health check with the alias record.</p> </li> <li> <p>Auto naming currently doesn&#39;t support creating alias records that route traffic to AWS resources other than Elastic Load Balancing load balancers.</p> </li> <li> <p>If you specify a value for <code>AWS<em>ALIAS</em>DNS<em>NAME</code>, don&#39;t specify values for any of the <code>AWS</em>INSTANCE</code> attributes.</p> </li> </ul> </dd> <dt>AWS<em>EC2</em>INSTANCE<em>ID</dt> <dd> <p> <i>HTTP namespaces only.</i> The Amazon EC2 instance ID for the instance. If the <code>AWS</em>EC2<em>INSTANCE</em>ID</code> attribute is specified, then the only other attribute that can be specified is <code>AWS<em>INIT</em>HEALTH<em>STATUS</code>. When the <code>AWS</em>EC2<em>INSTANCE</em>ID</code> attribute is specified, then the <code>AWS<em>INSTANCE</em>IPV4</code> attribute will be filled out with the primary private IPv4 address.</p> </dd> <dt>AWS<em>INIT</em>HEALTH<em>STATUS</dt> <dd> <p>If the service configuration includes <code>HealthCheckCustomConfig</code>, you can optionally use <code>AWS</em>INIT<em>HEALTH</em>STATUS</code> to specify the initial status of the custom health check, <code>HEALTHY</code> or <code>UNHEALTHY</code>. If you don&#39;t specify a value for <code>AWS<em>INIT</em>HEALTH<em>STATUS</code>, the initial status is <code>HEALTHY</code>.</p> </dd> <dt>AWS</em>INSTANCE<em>CNAME</dt> <dd> <p>If the service configuration includes a <code>CNAME</code> record, the domain name that you want Route 53 to return in response to DNS queries (for example, <code>example.com</code>).</p> <p>This value is required if the service specified by <code>ServiceId</code> includes settings for an <code>CNAME</code> record.</p> </dd> <dt>AWS</em>INSTANCE<em>IPV4</dt> <dd> <p>If the service configuration includes an <code>A</code> record, the IPv4 address that you want Route 53 to return in response to DNS queries (for example, <code>192.0.2.44</code>).</p> <p>This value is required if the service specified by <code>ServiceId</code> includes settings for an <code>A</code> record. If the service includes settings for an <code>SRV</code> record, you must specify a value for <code>AWS</em>INSTANCE<em>IPV4</code>, <code>AWS</em>INSTANCE<em>IPV6</code>, or both.</p> </dd> <dt>AWS</em>INSTANCE<em>IPV6</dt> <dd> <p>If the service configuration includes an <code>AAAA</code> record, the IPv6 address that you want Route 53 to return in response to DNS queries (for example, <code>2001:0db8:85a3:0000:0000:abcd:0001:2345</code>).</p> <p>This value is required if the service specified by <code>ServiceId</code> includes settings for an <code>AAAA</code> record. If the service includes settings for an <code>SRV</code> record, you must specify a value for <code>AWS</em>INSTANCE<em>IPV4</code>, <code>AWS</em>INSTANCE<em>IPV6</code>, or both.</p> </dd> <dt>AWS</em>INSTANCE_PORT</dt> <dd> <p>If the service includes an <code>SRV</code> record, the value that you want Route 53 to return for the port.</p> <p>If the service includes <code>HealthCheckConfig</code>, the port on the endpoint that you want Route 53 to send requests to. </p> <p>This value is required if you specified settings for an <code>SRV</code> record or a Route 53 health check when you created the service.</p> </dd> <dt>Custom attributes</dt> <dd> <p>You can add up to 30 custom attributes. For each key-value pair, the maximum length of the attribute name is 255 characters, and the maximum length of the attribute value is 1,024 characters. The total size of all provided attributes (sum of all keys and values) must not exceed 5,000 characters.</p> </dd> </dl></p>
    #[serde(rename = "attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    /// <p>A unique string that identifies the request and that allows failed <code>RegisterInstance</code> requests to be retried without the risk of executing the operation twice. You must use a unique <code>CreatorRequestId</code> string every time you submit a <code>RegisterInstance</code> request if you're registering additional instances for the same namespace and service. <code>CreatorRequestId</code> can be any unique string (for example, a date/time stamp).</p>
    #[serde(rename = "creatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p><p>An identifier that you want to associate with the instance. Note the following:</p> <ul> <li> <p>If the service that&#39;s specified by <code>ServiceId</code> includes settings for an <code>SRV</code> record, the value of <code>InstanceId</code> is automatically included as part of the value for the <code>SRV</code> record. For more information, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_DnsRecord.html#cloudmap-Type-DnsRecord-Type">DnsRecord &gt; Type</a>.</p> </li> <li> <p>You can use this value to update an existing instance.</p> </li> <li> <p>To register a new instance, you must specify a value that&#39;s unique among instances that you register by using the same service. </p> </li> <li> <p>If you specify an existing <code>InstanceId</code> and <code>ServiceId</code>, AWS Cloud Map updates the existing DNS records, if any. If there&#39;s also an existing health check, AWS Cloud Map deletes the old health check and creates a new one. </p> <note> <p>The health check isn&#39;t deleted immediately, so it will still appear for a while if you submit a <code>ListHealthChecks</code> request, for example.</p> </note> </li> </ul></p>
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    /// <p>The ID of the service that you want to use for settings for the instance.</p>
    #[serde(rename = "serviceId")]
    pub service_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterInstanceResponse {
    /// <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_GetOperation.html">GetOperation</a>.</p>
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

/// <p>A complex type that contains information about the specified service.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Service {
    /// <p>The Amazon Resource Name (ARN) that AWS Cloud Map assigns to the service when you create it.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the service was created, in Unix format and Coordinated Universal Time (UTC). The value of <code>CreateDate</code> is accurate to milliseconds. For example, the value <code>1516925490.087</code> represents Friday, January 26, 2018 12:11:30.087 AM.</p>
    #[serde(rename = "createDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    /// <p>A unique string that identifies the request and that allows failed requests to be retried without the risk of running the operation twice. <code>CreatorRequestId</code> can be any unique string (for example, a date/timestamp).</p>
    #[serde(rename = "creatorRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    /// <p>The description of the service.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A complex type that contains information about the Route 53 DNS records that you want AWS Cloud Map to create when you register an instance.</p>
    #[serde(rename = "dnsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<DnsConfig>,
    /// <p> <i>Public DNS and HTTP namespaces only.</i> A complex type that contains settings for an optional health check. If you specify settings for a health check, AWS Cloud Map associates the health check with the records that you specify in <code>DnsConfig</code>.</p> <p>For information about the charges for health checks, see <a href="http://aws.amazon.com/route53/pricing/">Amazon Route 53 Pricing</a>.</p>
    #[serde(rename = "healthCheckConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<HealthCheckConfig>,
    /// <p><p>A complex type that contains information about an optional custom health check.</p> <important> <p>If you specify a health check configuration, you can specify either <code>HealthCheckCustomConfig</code> or <code>HealthCheckConfig</code> but not both.</p> </important></p>
    #[serde(rename = "healthCheckCustomConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_custom_config: Option<HealthCheckCustomConfig>,
    /// <p>The ID that AWS Cloud Map assigned to the service when you created it.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The number of instances that are currently associated with the service. Instances that were previously associated with the service but that are deleted aren't included in the count. The count might not reflect pending registrations and deregistrations.</p>
    #[serde(rename = "instanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i64>,
    /// <p>The name of the service.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ID of the namespace that was used to create the service.</p>
    #[serde(rename = "namespaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    /// <p><p>Describes the systems that can be used to discover the service instances.</p> <dl> <dt>DNS_HTTP</dt> <dd> <p>The service instances can be discovered using either DNS queries or the <code>DiscoverInstances</code> API operation.</p> </dd> <dt>HTTP</dt> <dd> <p>The service instances can only be discovered using the <code>DiscoverInstances</code> API operation.</p> </dd> <dt>DNS</dt> <dd> <p>Reserved.</p> </dd> </dl></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A complex type that contains changes to an existing service.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ServiceChange {
    /// <p>A description for the service.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Information about the Route 53 DNS records that you want AWS Cloud Map to create when you register an instance.</p>
    #[serde(rename = "dnsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<DnsConfigChange>,
    /// <p> <i>Public DNS and HTTP namespaces only.</i> Settings for an optional health check. If you specify settings for a health check, AWS Cloud Map associates the health check with the records that you specify in <code>DnsConfig</code>.</p>
    #[serde(rename = "healthCheckConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<HealthCheckConfig>,
}

/// <p>A complex type that lets you specify the namespaces that you want to list services for.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ServiceFilter {
    /// <p><p>The operator that you want to use to determine whether a service is returned by <code>ListServices</code>. Valid values for <code>Condition</code> include the following:</p> <ul> <li> <p> <code>EQ</code>: When you specify <code>EQ</code>, specify one namespace ID for <code>Values</code>. <code>EQ</code> is the default condition and can be omitted.</p> </li> <li> <p> <code>IN</code>: When you specify <code>IN</code>, specify a list of the IDs for the namespaces that you want <code>ListServices</code> to return a list of services for.</p> </li> <li> <p> <code>BETWEEN</code>: Not applicable.</p> </li> </ul></p>
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// <p>Specify <code>NAMESPACE_ID</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The values that are applicable to the value that you specify for <code>Condition</code> to filter the list of services.</p>
    #[serde(rename = "values")]
    pub values: Vec<String>,
}

/// <p>A complex type that contains information about a specified service.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ServiceSummary {
    /// <p>The Amazon Resource Name (ARN) that AWS Cloud Map assigns to the service when you create it.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time that the service was created.</p>
    #[serde(rename = "createDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    /// <p>The description that you specify when you create the service.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Information about the Route 53 DNS records that you want AWS Cloud Map to create when you register an instance.</p>
    #[serde(rename = "dnsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<DnsConfig>,
    /// <p> <i>Public DNS and HTTP namespaces only.</i> Settings for an optional health check. If you specify settings for a health check, AWS Cloud Map associates the health check with the records that you specify in <code>DnsConfig</code>.</p>
    #[serde(rename = "healthCheckConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<HealthCheckConfig>,
    /// <p><p>Information about an optional custom health check. A custom health check, which requires that you use a third-party health checker to evaluate the health of your resources, is useful in the following circumstances:</p> <ul> <li> <p>You can&#39;t use a health check that&#39;s defined by <code>HealthCheckConfig</code> because the resource isn&#39;t available over the internet. For example, you can use a custom health check when the instance is in an Amazon VPC. (To check the health of resources in a VPC, the health checker must also be in the VPC.)</p> </li> <li> <p>You want to use a third-party health checker regardless of where your resources are located.</p> </li> </ul> <important> <p>If you specify a health check configuration, you can specify either <code>HealthCheckCustomConfig</code> or <code>HealthCheckConfig</code> but not both.</p> </important></p>
    #[serde(rename = "healthCheckCustomConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_custom_config: Option<HealthCheckCustomConfig>,
    /// <p>The ID that AWS Cloud Map assigned to the service when you created it.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The number of instances that are currently associated with the service. Instances that were previously associated with the service but that are deleted aren't included in the count. The count might not reflect pending registrations and deregistrations.</p>
    #[serde(rename = "instanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i64>,
    /// <p>The name of the service.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>Describes the systems that can be used to discover the service instances.</p> <dl> <dt>DNS_HTTP</dt> <dd> <p>The service instances can be discovered using either DNS queries or the <code>DiscoverInstances</code> API operation.</p> </dd> <dt>HTTP</dt> <dd> <p>The service instances can only be discovered using the <code>DiscoverInstances</code> API operation.</p> </dd> <dt>DNS</dt> <dd> <p>Reserved.</p> </dd> </dl></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A custom key-value pair that's associated with a resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The key identifier, or name, of the tag.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The string value that's associated with the key of the tag. You can set the value of a tag to an empty string, but you can't set the value of a tag to null.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to retrieve tags for.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>The tags to add to the specified resource. Specifying the tag key is required. You can set the value of a tag to an empty string, but you can't set the value of a tag to null.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to retrieve tags for.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>The tag keys to remove from the specified resource.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateInstanceCustomHealthStatusRequest {
    /// <p>The ID of the instance that you want to change the health status for.</p>
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    /// <p>The ID of the service that includes the configuration for the custom health check that you want to change the status for.</p>
    #[serde(rename = "serviceId")]
    pub service_id: String,
    /// <p>The new status of the instance, <code>HEALTHY</code> or <code>UNHEALTHY</code>.</p>
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateServiceRequest {
    /// <p>The ID of the service that you want to update.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>A complex type that contains the new settings for the service.</p>
    #[serde(rename = "service")]
    pub service: ServiceChange,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateServiceResponse {
    /// <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_GetOperation.html">GetOperation</a>.</p>
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

/// Errors returned by CreateHttpNamespace
#[derive(Debug, PartialEq)]
pub enum CreateHttpNamespaceError {
    /// <p>The operation is already in progress.</p>
    DuplicateRequest(String),
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>The namespace that you're trying to create already exists.</p>
    NamespaceAlreadyExists(String),
    /// <p>The resource can't be created because you've reached the quota on the number of resources.</p>
    ResourceLimitExceeded(String),
    /// <p>The list of tags on the resource is over the quota. The maximum number of tags that can be applied to a resource is 50.</p>
    TooManyTags(String),
}

impl CreateHttpNamespaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateHttpNamespaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateRequest" => {
                    return RusotoError::Service(CreateHttpNamespaceError::DuplicateRequest(
                        err.msg,
                    ))
                }
                "InvalidInput" => {
                    return RusotoError::Service(CreateHttpNamespaceError::InvalidInput(err.msg))
                }
                "NamespaceAlreadyExists" => {
                    return RusotoError::Service(CreateHttpNamespaceError::NamespaceAlreadyExists(
                        err.msg,
                    ))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateHttpNamespaceError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreateHttpNamespaceError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateHttpNamespaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateHttpNamespaceError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            CreateHttpNamespaceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateHttpNamespaceError::NamespaceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateHttpNamespaceError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateHttpNamespaceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateHttpNamespaceError {}
/// Errors returned by CreatePrivateDnsNamespace
#[derive(Debug, PartialEq)]
pub enum CreatePrivateDnsNamespaceError {
    /// <p>The operation is already in progress.</p>
    DuplicateRequest(String),
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>The namespace that you're trying to create already exists.</p>
    NamespaceAlreadyExists(String),
    /// <p>The resource can't be created because you've reached the quota on the number of resources.</p>
    ResourceLimitExceeded(String),
    /// <p>The list of tags on the resource is over the quota. The maximum number of tags that can be applied to a resource is 50.</p>
    TooManyTags(String),
}

impl CreatePrivateDnsNamespaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePrivateDnsNamespaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateRequest" => {
                    return RusotoError::Service(CreatePrivateDnsNamespaceError::DuplicateRequest(
                        err.msg,
                    ))
                }
                "InvalidInput" => {
                    return RusotoError::Service(CreatePrivateDnsNamespaceError::InvalidInput(
                        err.msg,
                    ))
                }
                "NamespaceAlreadyExists" => {
                    return RusotoError::Service(
                        CreatePrivateDnsNamespaceError::NamespaceAlreadyExists(err.msg),
                    )
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(
                        CreatePrivateDnsNamespaceError::ResourceLimitExceeded(err.msg),
                    )
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreatePrivateDnsNamespaceError::TooManyTags(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePrivateDnsNamespaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePrivateDnsNamespaceError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            CreatePrivateDnsNamespaceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreatePrivateDnsNamespaceError::NamespaceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePrivateDnsNamespaceError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePrivateDnsNamespaceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePrivateDnsNamespaceError {}
/// Errors returned by CreatePublicDnsNamespace
#[derive(Debug, PartialEq)]
pub enum CreatePublicDnsNamespaceError {
    /// <p>The operation is already in progress.</p>
    DuplicateRequest(String),
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>The namespace that you're trying to create already exists.</p>
    NamespaceAlreadyExists(String),
    /// <p>The resource can't be created because you've reached the quota on the number of resources.</p>
    ResourceLimitExceeded(String),
    /// <p>The list of tags on the resource is over the quota. The maximum number of tags that can be applied to a resource is 50.</p>
    TooManyTags(String),
}

impl CreatePublicDnsNamespaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePublicDnsNamespaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateRequest" => {
                    return RusotoError::Service(CreatePublicDnsNamespaceError::DuplicateRequest(
                        err.msg,
                    ))
                }
                "InvalidInput" => {
                    return RusotoError::Service(CreatePublicDnsNamespaceError::InvalidInput(
                        err.msg,
                    ))
                }
                "NamespaceAlreadyExists" => {
                    return RusotoError::Service(
                        CreatePublicDnsNamespaceError::NamespaceAlreadyExists(err.msg),
                    )
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(
                        CreatePublicDnsNamespaceError::ResourceLimitExceeded(err.msg),
                    )
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreatePublicDnsNamespaceError::TooManyTags(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePublicDnsNamespaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePublicDnsNamespaceError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            CreatePublicDnsNamespaceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreatePublicDnsNamespaceError::NamespaceAlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePublicDnsNamespaceError::ResourceLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreatePublicDnsNamespaceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePublicDnsNamespaceError {}
/// Errors returned by CreateService
#[derive(Debug, PartialEq)]
pub enum CreateServiceError {
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>No namespace exists with the specified ID.</p>
    NamespaceNotFound(String),
    /// <p>The resource can't be created because you've reached the quota on the number of resources.</p>
    ResourceLimitExceeded(String),
    /// <p>The service can't be created because a service with the same name already exists.</p>
    ServiceAlreadyExists(String),
    /// <p>The list of tags on the resource is over the quota. The maximum number of tags that can be applied to a resource is 50.</p>
    TooManyTags(String),
}

impl CreateServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateServiceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(CreateServiceError::InvalidInput(err.msg))
                }
                "NamespaceNotFound" => {
                    return RusotoError::Service(CreateServiceError::NamespaceNotFound(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(CreateServiceError::ResourceLimitExceeded(err.msg))
                }
                "ServiceAlreadyExists" => {
                    return RusotoError::Service(CreateServiceError::ServiceAlreadyExists(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreateServiceError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateServiceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateServiceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            CreateServiceError::NamespaceNotFound(ref cause) => write!(f, "{}", cause),
            CreateServiceError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateServiceError::ServiceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateServiceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateServiceError {}
/// Errors returned by DeleteNamespace
#[derive(Debug, PartialEq)]
pub enum DeleteNamespaceError {
    /// <p>The operation is already in progress.</p>
    DuplicateRequest(String),
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>No namespace exists with the specified ID.</p>
    NamespaceNotFound(String),
    /// <p>The specified resource can't be deleted because it contains other resources. For example, you can't delete a service that contains any instances.</p>
    ResourceInUse(String),
}

impl DeleteNamespaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteNamespaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateRequest" => {
                    return RusotoError::Service(DeleteNamespaceError::DuplicateRequest(err.msg))
                }
                "InvalidInput" => {
                    return RusotoError::Service(DeleteNamespaceError::InvalidInput(err.msg))
                }
                "NamespaceNotFound" => {
                    return RusotoError::Service(DeleteNamespaceError::NamespaceNotFound(err.msg))
                }
                "ResourceInUse" => {
                    return RusotoError::Service(DeleteNamespaceError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteNamespaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteNamespaceError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            DeleteNamespaceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteNamespaceError::NamespaceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteNamespaceError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteNamespaceError {}
/// Errors returned by DeleteService
#[derive(Debug, PartialEq)]
pub enum DeleteServiceError {
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>The specified resource can't be deleted because it contains other resources. For example, you can't delete a service that contains any instances.</p>
    ResourceInUse(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
}

impl DeleteServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteServiceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(DeleteServiceError::InvalidInput(err.msg))
                }
                "ResourceInUse" => {
                    return RusotoError::Service(DeleteServiceError::ResourceInUse(err.msg))
                }
                "ServiceNotFound" => {
                    return RusotoError::Service(DeleteServiceError::ServiceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteServiceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteServiceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeleteServiceError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteServiceError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteServiceError {}
/// Errors returned by DeregisterInstance
#[derive(Debug, PartialEq)]
pub enum DeregisterInstanceError {
    /// <p>The operation is already in progress.</p>
    DuplicateRequest(String),
    /// <p>No instance exists with the specified ID, or the instance was recently registered, and information about the instance hasn't propagated yet.</p>
    InstanceNotFound(String),
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>The specified resource can't be deleted because it contains other resources. For example, you can't delete a service that contains any instances.</p>
    ResourceInUse(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
}

impl DeregisterInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateRequest" => {
                    return RusotoError::Service(DeregisterInstanceError::DuplicateRequest(err.msg))
                }
                "InstanceNotFound" => {
                    return RusotoError::Service(DeregisterInstanceError::InstanceNotFound(err.msg))
                }
                "InvalidInput" => {
                    return RusotoError::Service(DeregisterInstanceError::InvalidInput(err.msg))
                }
                "ResourceInUse" => {
                    return RusotoError::Service(DeregisterInstanceError::ResourceInUse(err.msg))
                }
                "ServiceNotFound" => {
                    return RusotoError::Service(DeregisterInstanceError::ServiceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeregisterInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeregisterInstanceError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            DeregisterInstanceError::InstanceNotFound(ref cause) => write!(f, "{}", cause),
            DeregisterInstanceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DeregisterInstanceError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeregisterInstanceError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeregisterInstanceError {}
/// Errors returned by DiscoverInstances
#[derive(Debug, PartialEq)]
pub enum DiscoverInstancesError {
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>No namespace exists with the specified ID.</p>
    NamespaceNotFound(String),
    /// <p>The operation can't be completed because you've reached the quota for the number of requests. For more information, see <a href="https://docs.aws.amazon.com/cloud-map/latest/dg/throttling.html">AWS Cloud Map API request throttling quota</a> in the <i>AWS Cloud Map Developer Guide</i>.</p>
    RequestLimitExceeded(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
}

impl DiscoverInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DiscoverInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(DiscoverInstancesError::InvalidInput(err.msg))
                }
                "NamespaceNotFound" => {
                    return RusotoError::Service(DiscoverInstancesError::NamespaceNotFound(err.msg))
                }
                "RequestLimitExceeded" => {
                    return RusotoError::Service(DiscoverInstancesError::RequestLimitExceeded(
                        err.msg,
                    ))
                }
                "ServiceNotFound" => {
                    return RusotoError::Service(DiscoverInstancesError::ServiceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DiscoverInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DiscoverInstancesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            DiscoverInstancesError::NamespaceNotFound(ref cause) => write!(f, "{}", cause),
            DiscoverInstancesError::RequestLimitExceeded(ref cause) => write!(f, "{}", cause),
            DiscoverInstancesError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DiscoverInstancesError {}
/// Errors returned by GetInstance
#[derive(Debug, PartialEq)]
pub enum GetInstanceError {
    /// <p>No instance exists with the specified ID, or the instance was recently registered, and information about the instance hasn't propagated yet.</p>
    InstanceNotFound(String),
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
}

impl GetInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InstanceNotFound" => {
                    return RusotoError::Service(GetInstanceError::InstanceNotFound(err.msg))
                }
                "InvalidInput" => {
                    return RusotoError::Service(GetInstanceError::InvalidInput(err.msg))
                }
                "ServiceNotFound" => {
                    return RusotoError::Service(GetInstanceError::ServiceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInstanceError::InstanceNotFound(ref cause) => write!(f, "{}", cause),
            GetInstanceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInstanceError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInstanceError {}
/// Errors returned by GetInstancesHealthStatus
#[derive(Debug, PartialEq)]
pub enum GetInstancesHealthStatusError {
    /// <p>No instance exists with the specified ID, or the instance was recently registered, and information about the instance hasn't propagated yet.</p>
    InstanceNotFound(String),
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
}

impl GetInstancesHealthStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstancesHealthStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InstanceNotFound" => {
                    return RusotoError::Service(GetInstancesHealthStatusError::InstanceNotFound(
                        err.msg,
                    ))
                }
                "InvalidInput" => {
                    return RusotoError::Service(GetInstancesHealthStatusError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceNotFound" => {
                    return RusotoError::Service(GetInstancesHealthStatusError::ServiceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetInstancesHealthStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetInstancesHealthStatusError::InstanceNotFound(ref cause) => write!(f, "{}", cause),
            GetInstancesHealthStatusError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetInstancesHealthStatusError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetInstancesHealthStatusError {}
/// Errors returned by GetNamespace
#[derive(Debug, PartialEq)]
pub enum GetNamespaceError {
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>No namespace exists with the specified ID.</p>
    NamespaceNotFound(String),
}

impl GetNamespaceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetNamespaceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(GetNamespaceError::InvalidInput(err.msg))
                }
                "NamespaceNotFound" => {
                    return RusotoError::Service(GetNamespaceError::NamespaceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetNamespaceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetNamespaceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetNamespaceError::NamespaceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetNamespaceError {}
/// Errors returned by GetOperation
#[derive(Debug, PartialEq)]
pub enum GetOperationError {
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>No operation exists with the specified ID.</p>
    OperationNotFound(String),
}

impl GetOperationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOperationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(GetOperationError::InvalidInput(err.msg))
                }
                "OperationNotFound" => {
                    return RusotoError::Service(GetOperationError::OperationNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetOperationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetOperationError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetOperationError::OperationNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetOperationError {}
/// Errors returned by GetService
#[derive(Debug, PartialEq)]
pub enum GetServiceError {
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
}

impl GetServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetServiceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(GetServiceError::InvalidInput(err.msg))
                }
                "ServiceNotFound" => {
                    return RusotoError::Service(GetServiceError::ServiceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetServiceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetServiceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            GetServiceError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetServiceError {}
/// Errors returned by ListInstances
#[derive(Debug, PartialEq)]
pub enum ListInstancesError {
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
}

impl ListInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(ListInstancesError::InvalidInput(err.msg))
                }
                "ServiceNotFound" => {
                    return RusotoError::Service(ListInstancesError::ServiceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInstancesError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListInstancesError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInstancesError {}
/// Errors returned by ListNamespaces
#[derive(Debug, PartialEq)]
pub enum ListNamespacesError {
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
}

impl ListNamespacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListNamespacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(ListNamespacesError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListNamespacesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListNamespacesError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListNamespacesError {}
/// Errors returned by ListOperations
#[derive(Debug, PartialEq)]
pub enum ListOperationsError {
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
}

impl ListOperationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOperationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(ListOperationsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListOperationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOperationsError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListOperationsError {}
/// Errors returned by ListServices
#[derive(Debug, PartialEq)]
pub enum ListServicesError {
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
}

impl ListServicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListServicesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(ListServicesError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListServicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListServicesError::InvalidInput(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListServicesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>The operation can't be completed because the resource was not found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by RegisterInstance
#[derive(Debug, PartialEq)]
pub enum RegisterInstanceError {
    /// <p>The operation is already in progress.</p>
    DuplicateRequest(String),
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>The specified resource can't be deleted because it contains other resources. For example, you can't delete a service that contains any instances.</p>
    ResourceInUse(String),
    /// <p>The resource can't be created because you've reached the quota on the number of resources.</p>
    ResourceLimitExceeded(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
}

impl RegisterInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateRequest" => {
                    return RusotoError::Service(RegisterInstanceError::DuplicateRequest(err.msg))
                }
                "InvalidInput" => {
                    return RusotoError::Service(RegisterInstanceError::InvalidInput(err.msg))
                }
                "ResourceInUse" => {
                    return RusotoError::Service(RegisterInstanceError::ResourceInUse(err.msg))
                }
                "ResourceLimitExceeded" => {
                    return RusotoError::Service(RegisterInstanceError::ResourceLimitExceeded(
                        err.msg,
                    ))
                }
                "ServiceNotFound" => {
                    return RusotoError::Service(RegisterInstanceError::ServiceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RegisterInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterInstanceError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            RegisterInstanceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            RegisterInstanceError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            RegisterInstanceError::ResourceLimitExceeded(ref cause) => write!(f, "{}", cause),
            RegisterInstanceError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterInstanceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>The operation can't be completed because the resource was not found.</p>
    ResourceNotFound(String),
    /// <p>The list of tags on the resource is over the quota. The maximum number of tags that can be applied to a resource is 50.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(TagResourceError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(TagResourceError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>The operation can't be completed because the resource was not found.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidInput" => {
                    return RusotoError::Service(UntagResourceError::InvalidInput(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateInstanceCustomHealthStatus
#[derive(Debug, PartialEq)]
pub enum UpdateInstanceCustomHealthStatusError {
    /// <p>The health check for the instance that's specified by <code>ServiceId</code> and <code>InstanceId</code> isn't a custom health check. </p>
    CustomHealthNotFound(String),
    /// <p>No instance exists with the specified ID, or the instance was recently registered, and information about the instance hasn't propagated yet.</p>
    InstanceNotFound(String),
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
}

impl UpdateInstanceCustomHealthStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateInstanceCustomHealthStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomHealthNotFound" => {
                    return RusotoError::Service(
                        UpdateInstanceCustomHealthStatusError::CustomHealthNotFound(err.msg),
                    )
                }
                "InstanceNotFound" => {
                    return RusotoError::Service(
                        UpdateInstanceCustomHealthStatusError::InstanceNotFound(err.msg),
                    )
                }
                "InvalidInput" => {
                    return RusotoError::Service(
                        UpdateInstanceCustomHealthStatusError::InvalidInput(err.msg),
                    )
                }
                "ServiceNotFound" => {
                    return RusotoError::Service(
                        UpdateInstanceCustomHealthStatusError::ServiceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateInstanceCustomHealthStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateInstanceCustomHealthStatusError::CustomHealthNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateInstanceCustomHealthStatusError::InstanceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateInstanceCustomHealthStatusError::InvalidInput(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateInstanceCustomHealthStatusError::ServiceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateInstanceCustomHealthStatusError {}
/// Errors returned by UpdateService
#[derive(Debug, PartialEq)]
pub enum UpdateServiceError {
    /// <p>The operation is already in progress.</p>
    DuplicateRequest(String),
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(String),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(String),
}

impl UpdateServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateServiceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DuplicateRequest" => {
                    return RusotoError::Service(UpdateServiceError::DuplicateRequest(err.msg))
                }
                "InvalidInput" => {
                    return RusotoError::Service(UpdateServiceError::InvalidInput(err.msg))
                }
                "ServiceNotFound" => {
                    return RusotoError::Service(UpdateServiceError::ServiceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateServiceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateServiceError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            UpdateServiceError::InvalidInput(ref cause) => write!(f, "{}", cause),
            UpdateServiceError::ServiceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateServiceError {}
/// Trait representing the capabilities of the ServiceDiscovery API. ServiceDiscovery clients implement this trait.
#[async_trait]
pub trait ServiceDiscovery {
    /// <p>Creates an HTTP namespace. Service instances registered using an HTTP namespace can be discovered using a <code>DiscoverInstances</code> request but can't be discovered using DNS.</p> <p>For the current quota on the number of namespaces that you can create using the same AWS account, see <a href="https://docs.aws.amazon.com/cloud-map/latest/dg/cloud-map-limits.html">AWS Cloud Map quotas</a> in the <i>AWS Cloud Map Developer Guide</i>.</p>
    async fn create_http_namespace(
        &self,
        input: CreateHttpNamespaceRequest,
    ) -> Result<CreateHttpNamespaceResponse, RusotoError<CreateHttpNamespaceError>>;

    /// <p>Creates a private namespace based on DNS, which is visible only inside a specified Amazon VPC. The namespace defines your service naming scheme. For example, if you name your namespace <code>example.com</code> and name your service <code>backend</code>, the resulting DNS name for the service is <code>backend.example.com</code>. Service instances that are registered using a private DNS namespace can be discovered using either a <code>DiscoverInstances</code> request or using DNS. For the current quota on the number of namespaces that you can create using the same AWS account, see <a href="https://docs.aws.amazon.com/cloud-map/latest/dg/cloud-map-limits.html">AWS Cloud Map Limits</a> in the <i>AWS Cloud Map Developer Guide</i>.</p>
    async fn create_private_dns_namespace(
        &self,
        input: CreatePrivateDnsNamespaceRequest,
    ) -> Result<CreatePrivateDnsNamespaceResponse, RusotoError<CreatePrivateDnsNamespaceError>>;

    /// <p>Creates a public namespace based on DNS, which is visible on the internet. The namespace defines your service naming scheme. For example, if you name your namespace <code>example.com</code> and name your service <code>backend</code>, the resulting DNS name for the service is <code>backend.example.com</code>. You can discover instances that were registered with a public DNS namespace by using either a <code>DiscoverInstances</code> request or using DNS. For the current quota on the number of namespaces that you can create using the same AWS account, see <a href="https://docs.aws.amazon.com/cloud-map/latest/dg/cloud-map-limits.html">AWS Cloud Map Quotas</a>in the <i>AWS Cloud Map Developer Guide</i>.</p>
    async fn create_public_dns_namespace(
        &self,
        input: CreatePublicDnsNamespaceRequest,
    ) -> Result<CreatePublicDnsNamespaceResponse, RusotoError<CreatePublicDnsNamespaceError>>;

    /// <p>Creates a service. This action defines the configuration for the following entities:</p> <ul> <li> <p>For public and private DNS namespaces, one of the following combinations of DNS records in Amazon Route 53:</p> <ul> <li> <p> <code>A</code> </p> </li> <li> <p> <code>AAAA</code> </p> </li> <li> <p> <code>A</code> and <code>AAAA</code> </p> </li> <li> <p> <code>SRV</code> </p> </li> <li> <p> <code>CNAME</code> </p> </li> </ul> </li> <li> <p>Optionally, a health check</p> </li> </ul> <p>After you create the service, you can submit a <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html">RegisterInstance</a> request, and AWS Cloud Map uses the values in the configuration to create the specified entities.</p> <p>For the current quota on the number of instances that you can register using the same namespace and using the same service, see <a href="https://docs.aws.amazon.com/cloud-map/latest/dg/cloud-map-limits.html">AWS Cloud Map Limits</a> in the <i>AWS Cloud Map Developer Guide</i>.</p>
    async fn create_service(
        &self,
        input: CreateServiceRequest,
    ) -> Result<CreateServiceResponse, RusotoError<CreateServiceError>>;

    /// <p>Deletes a namespace from the current account. If the namespace still contains one or more services, the request fails.</p>
    async fn delete_namespace(
        &self,
        input: DeleteNamespaceRequest,
    ) -> Result<DeleteNamespaceResponse, RusotoError<DeleteNamespaceError>>;

    /// <p>Deletes a specified service. If the service still contains one or more registered instances, the request fails.</p>
    async fn delete_service(
        &self,
        input: DeleteServiceRequest,
    ) -> Result<DeleteServiceResponse, RusotoError<DeleteServiceError>>;

    /// <p>Deletes the Amazon Route 53 DNS records and health check, if any, that AWS Cloud Map created for the specified instance.</p>
    async fn deregister_instance(
        &self,
        input: DeregisterInstanceRequest,
    ) -> Result<DeregisterInstanceResponse, RusotoError<DeregisterInstanceError>>;

    /// <p>Discovers registered instances for a specified namespace and service. You can use <code>DiscoverInstances</code> to discover instances for any type of namespace. For public and private DNS namespaces, you can also use DNS queries to discover instances.</p>
    async fn discover_instances(
        &self,
        input: DiscoverInstancesRequest,
    ) -> Result<DiscoverInstancesResponse, RusotoError<DiscoverInstancesError>>;

    /// <p>Gets information about a specified instance.</p>
    async fn get_instance(
        &self,
        input: GetInstanceRequest,
    ) -> Result<GetInstanceResponse, RusotoError<GetInstanceError>>;

    /// <p><p>Gets the current health status (<code>Healthy</code>, <code>Unhealthy</code>, or <code>Unknown</code>) of one or more instances that are associated with a specified service.</p> <note> <p>There&#39;s a brief delay between when you register an instance and when the health status for the instance is available. </p> </note></p>
    async fn get_instances_health_status(
        &self,
        input: GetInstancesHealthStatusRequest,
    ) -> Result<GetInstancesHealthStatusResponse, RusotoError<GetInstancesHealthStatusError>>;

    /// <p>Gets information about a namespace.</p>
    async fn get_namespace(
        &self,
        input: GetNamespaceRequest,
    ) -> Result<GetNamespaceResponse, RusotoError<GetNamespaceError>>;

    /// <p><p>Gets information about any operation that returns an operation ID in the response, such as a <code>CreateService</code> request.</p> <note> <p>To get a list of operations that match specified criteria, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_ListOperations.html">ListOperations</a>.</p> </note></p>
    async fn get_operation(
        &self,
        input: GetOperationRequest,
    ) -> Result<GetOperationResponse, RusotoError<GetOperationError>>;

    /// <p>Gets the settings for a specified service.</p>
    async fn get_service(
        &self,
        input: GetServiceRequest,
    ) -> Result<GetServiceResponse, RusotoError<GetServiceError>>;

    /// <p>Lists summary information about the instances that you registered by using a specified service.</p>
    async fn list_instances(
        &self,
        input: ListInstancesRequest,
    ) -> Result<ListInstancesResponse, RusotoError<ListInstancesError>>;

    /// <p>Lists summary information about the namespaces that were created by the current AWS account.</p>
    async fn list_namespaces(
        &self,
        input: ListNamespacesRequest,
    ) -> Result<ListNamespacesResponse, RusotoError<ListNamespacesError>>;

    /// <p>Lists operations that match the criteria that you specify.</p>
    async fn list_operations(
        &self,
        input: ListOperationsRequest,
    ) -> Result<ListOperationsResponse, RusotoError<ListOperationsError>>;

    /// <p>Lists summary information for all the services that are associated with one or more specified namespaces.</p>
    async fn list_services(
        &self,
        input: ListServicesRequest,
    ) -> Result<ListServicesResponse, RusotoError<ListServicesError>>;

    /// <p>Lists tags for the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Creates or updates one or more records and, optionally, creates a health check based on the settings in a specified service. When you submit a <code>RegisterInstance</code> request, the following occurs:</p> <ul> <li> <p>For each DNS record that you define in the service that's specified by <code>ServiceId</code>, a record is created or updated in the hosted zone that's associated with the corresponding namespace.</p> </li> <li> <p>If the service includes <code>HealthCheckConfig</code>, a health check is created based on the settings in the health check configuration.</p> </li> <li> <p>The health check, if any, is associated with each of the new or updated records.</p> </li> </ul> <important> <p>One <code>RegisterInstance</code> request must complete before you can submit another request and specify the same service ID and instance ID.</p> </important> <p>For more information, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_CreateService.html">CreateService</a>.</p> <p>When AWS Cloud Map receives a DNS query for the specified DNS name, it returns the applicable value:</p> <ul> <li> <p> <b>If the health check is healthy</b>: returns all the records</p> </li> <li> <p> <b>If the health check is unhealthy</b>: returns the applicable value for the last healthy instance</p> </li> <li> <p> <b>If you didn't specify a health check configuration</b>: returns all the records</p> </li> </ul> <p>For the current quota on the number of instances that you can register using the same namespace and using the same service, see <a href="https://docs.aws.amazon.com/cloud-map/latest/dg/cloud-map-limits.html">AWS Cloud Map Limits</a> in the <i>AWS Cloud Map Developer Guide</i>.</p>
    async fn register_instance(
        &self,
        input: RegisterInstanceRequest,
    ) -> Result<RegisterInstanceResponse, RusotoError<RegisterInstanceError>>;

    /// <p>Adds one or more tags to the specified resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes one or more tags from the specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Submits a request to change the health status of a custom health check to healthy or unhealthy.</p> <p>You can use <code>UpdateInstanceCustomHealthStatus</code> to change the status only for custom health checks, which you define using <code>HealthCheckCustomConfig</code> when you create a service. You can't use it to change the status for Route 53 health checks, which you define using <code>HealthCheckConfig</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_HealthCheckCustomConfig.html">HealthCheckCustomConfig</a>.</p>
    async fn update_instance_custom_health_status(
        &self,
        input: UpdateInstanceCustomHealthStatusRequest,
    ) -> Result<(), RusotoError<UpdateInstanceCustomHealthStatusError>>;

    /// <p>Submits a request to perform the following operations:</p> <ul> <li> <p>Update the TTL setting for existing <code>DnsRecords</code> configurations</p> </li> <li> <p>Add, update, or delete <code>HealthCheckConfig</code> for a specified service</p> <note> <p>You can't add, update, or delete a <code>HealthCheckCustomConfig</code> configuration.</p> </note> </li> </ul> <p>For public and private DNS namespaces, note the following:</p> <ul> <li> <p>If you omit any existing <code>DnsRecords</code> or <code>HealthCheckConfig</code> configurations from an <code>UpdateService</code> request, the configurations are deleted from the service.</p> </li> <li> <p>If you omit an existing <code>HealthCheckCustomConfig</code> configuration from an <code>UpdateService</code> request, the configuration isn't deleted from the service.</p> </li> </ul> <p>When you update settings for a service, AWS Cloud Map also updates the corresponding settings in all the records and health checks that were created by using the specified service.</p>
    async fn update_service(
        &self,
        input: UpdateServiceRequest,
    ) -> Result<UpdateServiceResponse, RusotoError<UpdateServiceError>>;
}
/// A client for the ServiceDiscovery API.
#[derive(Clone)]
pub struct ServiceDiscoveryClient {
    client: Client,
    region: region::Region,
}

impl ServiceDiscoveryClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ServiceDiscoveryClient {
        ServiceDiscoveryClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ServiceDiscoveryClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ServiceDiscoveryClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ServiceDiscoveryClient {
        ServiceDiscoveryClient { client, region }
    }
}

#[async_trait]
impl ServiceDiscovery for ServiceDiscoveryClient {
    /// <p>Creates an HTTP namespace. Service instances registered using an HTTP namespace can be discovered using a <code>DiscoverInstances</code> request but can't be discovered using DNS.</p> <p>For the current quota on the number of namespaces that you can create using the same AWS account, see <a href="https://docs.aws.amazon.com/cloud-map/latest/dg/cloud-map-limits.html">AWS Cloud Map quotas</a> in the <i>AWS Cloud Map Developer Guide</i>.</p>
    async fn create_http_namespace(
        &self,
        input: CreateHttpNamespaceRequest,
    ) -> Result<CreateHttpNamespaceResponse, RusotoError<CreateHttpNamespaceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.CreateHttpNamespace",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateHttpNamespaceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateHttpNamespaceResponse, _>()
    }

    /// <p>Creates a private namespace based on DNS, which is visible only inside a specified Amazon VPC. The namespace defines your service naming scheme. For example, if you name your namespace <code>example.com</code> and name your service <code>backend</code>, the resulting DNS name for the service is <code>backend.example.com</code>. Service instances that are registered using a private DNS namespace can be discovered using either a <code>DiscoverInstances</code> request or using DNS. For the current quota on the number of namespaces that you can create using the same AWS account, see <a href="https://docs.aws.amazon.com/cloud-map/latest/dg/cloud-map-limits.html">AWS Cloud Map Limits</a> in the <i>AWS Cloud Map Developer Guide</i>.</p>
    async fn create_private_dns_namespace(
        &self,
        input: CreatePrivateDnsNamespaceRequest,
    ) -> Result<CreatePrivateDnsNamespaceResponse, RusotoError<CreatePrivateDnsNamespaceError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.CreatePrivateDnsNamespace",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreatePrivateDnsNamespaceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreatePrivateDnsNamespaceResponse, _>()
    }

    /// <p>Creates a public namespace based on DNS, which is visible on the internet. The namespace defines your service naming scheme. For example, if you name your namespace <code>example.com</code> and name your service <code>backend</code>, the resulting DNS name for the service is <code>backend.example.com</code>. You can discover instances that were registered with a public DNS namespace by using either a <code>DiscoverInstances</code> request or using DNS. For the current quota on the number of namespaces that you can create using the same AWS account, see <a href="https://docs.aws.amazon.com/cloud-map/latest/dg/cloud-map-limits.html">AWS Cloud Map Quotas</a>in the <i>AWS Cloud Map Developer Guide</i>.</p>
    async fn create_public_dns_namespace(
        &self,
        input: CreatePublicDnsNamespaceRequest,
    ) -> Result<CreatePublicDnsNamespaceResponse, RusotoError<CreatePublicDnsNamespaceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.CreatePublicDnsNamespace",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreatePublicDnsNamespaceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreatePublicDnsNamespaceResponse, _>()
    }

    /// <p>Creates a service. This action defines the configuration for the following entities:</p> <ul> <li> <p>For public and private DNS namespaces, one of the following combinations of DNS records in Amazon Route 53:</p> <ul> <li> <p> <code>A</code> </p> </li> <li> <p> <code>AAAA</code> </p> </li> <li> <p> <code>A</code> and <code>AAAA</code> </p> </li> <li> <p> <code>SRV</code> </p> </li> <li> <p> <code>CNAME</code> </p> </li> </ul> </li> <li> <p>Optionally, a health check</p> </li> </ul> <p>After you create the service, you can submit a <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_RegisterInstance.html">RegisterInstance</a> request, and AWS Cloud Map uses the values in the configuration to create the specified entities.</p> <p>For the current quota on the number of instances that you can register using the same namespace and using the same service, see <a href="https://docs.aws.amazon.com/cloud-map/latest/dg/cloud-map-limits.html">AWS Cloud Map Limits</a> in the <i>AWS Cloud Map Developer Guide</i>.</p>
    async fn create_service(
        &self,
        input: CreateServiceRequest,
    ) -> Result<CreateServiceResponse, RusotoError<CreateServiceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.CreateService");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateServiceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateServiceResponse, _>()
    }

    /// <p>Deletes a namespace from the current account. If the namespace still contains one or more services, the request fails.</p>
    async fn delete_namespace(
        &self,
        input: DeleteNamespaceRequest,
    ) -> Result<DeleteNamespaceResponse, RusotoError<DeleteNamespaceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.DeleteNamespace",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteNamespaceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteNamespaceResponse, _>()
    }

    /// <p>Deletes a specified service. If the service still contains one or more registered instances, the request fails.</p>
    async fn delete_service(
        &self,
        input: DeleteServiceRequest,
    ) -> Result<DeleteServiceResponse, RusotoError<DeleteServiceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.DeleteService");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteServiceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteServiceResponse, _>()
    }

    /// <p>Deletes the Amazon Route 53 DNS records and health check, if any, that AWS Cloud Map created for the specified instance.</p>
    async fn deregister_instance(
        &self,
        input: DeregisterInstanceRequest,
    ) -> Result<DeregisterInstanceResponse, RusotoError<DeregisterInstanceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.DeregisterInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeregisterInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeregisterInstanceResponse, _>()
    }

    /// <p>Discovers registered instances for a specified namespace and service. You can use <code>DiscoverInstances</code> to discover instances for any type of namespace. For public and private DNS namespaces, you can also use DNS queries to discover instances.</p>
    async fn discover_instances(
        &self,
        input: DiscoverInstancesRequest,
    ) -> Result<DiscoverInstancesResponse, RusotoError<DiscoverInstancesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.DiscoverInstances",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DiscoverInstancesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DiscoverInstancesResponse, _>()
    }

    /// <p>Gets information about a specified instance.</p>
    async fn get_instance(
        &self,
        input: GetInstanceRequest,
    ) -> Result<GetInstanceResponse, RusotoError<GetInstanceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.GetInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetInstanceResponse, _>()
    }

    /// <p><p>Gets the current health status (<code>Healthy</code>, <code>Unhealthy</code>, or <code>Unknown</code>) of one or more instances that are associated with a specified service.</p> <note> <p>There&#39;s a brief delay between when you register an instance and when the health status for the instance is available. </p> </note></p>
    async fn get_instances_health_status(
        &self,
        input: GetInstancesHealthStatusRequest,
    ) -> Result<GetInstancesHealthStatusResponse, RusotoError<GetInstancesHealthStatusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.GetInstancesHealthStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetInstancesHealthStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetInstancesHealthStatusResponse, _>()
    }

    /// <p>Gets information about a namespace.</p>
    async fn get_namespace(
        &self,
        input: GetNamespaceRequest,
    ) -> Result<GetNamespaceResponse, RusotoError<GetNamespaceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.GetNamespace");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetNamespaceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetNamespaceResponse, _>()
    }

    /// <p><p>Gets information about any operation that returns an operation ID in the response, such as a <code>CreateService</code> request.</p> <note> <p>To get a list of operations that match specified criteria, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_ListOperations.html">ListOperations</a>.</p> </note></p>
    async fn get_operation(
        &self,
        input: GetOperationRequest,
    ) -> Result<GetOperationResponse, RusotoError<GetOperationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.GetOperation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetOperationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetOperationResponse, _>()
    }

    /// <p>Gets the settings for a specified service.</p>
    async fn get_service(
        &self,
        input: GetServiceRequest,
    ) -> Result<GetServiceResponse, RusotoError<GetServiceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.GetService");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetServiceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetServiceResponse, _>()
    }

    /// <p>Lists summary information about the instances that you registered by using a specified service.</p>
    async fn list_instances(
        &self,
        input: ListInstancesRequest,
    ) -> Result<ListInstancesResponse, RusotoError<ListInstancesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.ListInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListInstancesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListInstancesResponse, _>()
    }

    /// <p>Lists summary information about the namespaces that were created by the current AWS account.</p>
    async fn list_namespaces(
        &self,
        input: ListNamespacesRequest,
    ) -> Result<ListNamespacesResponse, RusotoError<ListNamespacesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.ListNamespaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListNamespacesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListNamespacesResponse, _>()
    }

    /// <p>Lists operations that match the criteria that you specify.</p>
    async fn list_operations(
        &self,
        input: ListOperationsRequest,
    ) -> Result<ListOperationsResponse, RusotoError<ListOperationsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.ListOperations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListOperationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListOperationsResponse, _>()
    }

    /// <p>Lists summary information for all the services that are associated with one or more specified namespaces.</p>
    async fn list_services(
        &self,
        input: ListServicesRequest,
    ) -> Result<ListServicesResponse, RusotoError<ListServicesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.ListServices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListServicesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListServicesResponse, _>()
    }

    /// <p>Lists tags for the specified resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Creates or updates one or more records and, optionally, creates a health check based on the settings in a specified service. When you submit a <code>RegisterInstance</code> request, the following occurs:</p> <ul> <li> <p>For each DNS record that you define in the service that's specified by <code>ServiceId</code>, a record is created or updated in the hosted zone that's associated with the corresponding namespace.</p> </li> <li> <p>If the service includes <code>HealthCheckConfig</code>, a health check is created based on the settings in the health check configuration.</p> </li> <li> <p>The health check, if any, is associated with each of the new or updated records.</p> </li> </ul> <important> <p>One <code>RegisterInstance</code> request must complete before you can submit another request and specify the same service ID and instance ID.</p> </important> <p>For more information, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_CreateService.html">CreateService</a>.</p> <p>When AWS Cloud Map receives a DNS query for the specified DNS name, it returns the applicable value:</p> <ul> <li> <p> <b>If the health check is healthy</b>: returns all the records</p> </li> <li> <p> <b>If the health check is unhealthy</b>: returns the applicable value for the last healthy instance</p> </li> <li> <p> <b>If you didn't specify a health check configuration</b>: returns all the records</p> </li> </ul> <p>For the current quota on the number of instances that you can register using the same namespace and using the same service, see <a href="https://docs.aws.amazon.com/cloud-map/latest/dg/cloud-map-limits.html">AWS Cloud Map Limits</a> in the <i>AWS Cloud Map Developer Guide</i>.</p>
    async fn register_instance(
        &self,
        input: RegisterInstanceRequest,
    ) -> Result<RegisterInstanceResponse, RusotoError<RegisterInstanceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.RegisterInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RegisterInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RegisterInstanceResponse, _>()
    }

    /// <p>Adds one or more tags to the specified resource.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Removes one or more tags from the specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p>Submits a request to change the health status of a custom health check to healthy or unhealthy.</p> <p>You can use <code>UpdateInstanceCustomHealthStatus</code> to change the status only for custom health checks, which you define using <code>HealthCheckCustomConfig</code> when you create a service. You can't use it to change the status for Route 53 health checks, which you define using <code>HealthCheckConfig</code>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_HealthCheckCustomConfig.html">HealthCheckCustomConfig</a>.</p>
    async fn update_instance_custom_health_status(
        &self,
        input: UpdateInstanceCustomHealthStatusRequest,
    ) -> Result<(), RusotoError<UpdateInstanceCustomHealthStatusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Route53AutoNaming_v20170314.UpdateInstanceCustomHealthStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                UpdateInstanceCustomHealthStatusError::from_response,
            )
            .await?;
        std::mem::drop(response);
        Ok(())
    }

    /// <p>Submits a request to perform the following operations:</p> <ul> <li> <p>Update the TTL setting for existing <code>DnsRecords</code> configurations</p> </li> <li> <p>Add, update, or delete <code>HealthCheckConfig</code> for a specified service</p> <note> <p>You can't add, update, or delete a <code>HealthCheckCustomConfig</code> configuration.</p> </note> </li> </ul> <p>For public and private DNS namespaces, note the following:</p> <ul> <li> <p>If you omit any existing <code>DnsRecords</code> or <code>HealthCheckConfig</code> configurations from an <code>UpdateService</code> request, the configurations are deleted from the service.</p> </li> <li> <p>If you omit an existing <code>HealthCheckCustomConfig</code> configuration from an <code>UpdateService</code> request, the configuration isn't deleted from the service.</p> </li> </ul> <p>When you update settings for a service, AWS Cloud Map also updates the corresponding settings in all the records and health checks that were created by using the specified service.</p>
    async fn update_service(
        &self,
        input: UpdateServiceRequest,
    ) -> Result<UpdateServiceResponse, RusotoError<UpdateServiceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Route53AutoNaming_v20170314.UpdateService");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateServiceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateServiceResponse, _>()
    }
}
