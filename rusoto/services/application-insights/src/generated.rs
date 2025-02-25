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

impl ApplicationInsightsClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(
            http_method,
            "applicationinsights",
            &self.region,
            request_uri,
        );

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
/// <p>Describes a standalone resource or similarly grouped resources that the application is made up of.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationComponent {
    /// <p>The name of the component.</p>
    #[serde(rename = "componentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    /// <p> If logging is supported for the resource type, indicates whether the component has configured logs to be monitored. </p>
    #[serde(rename = "componentRemarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_remarks: Option<String>,
    /// <p> Workloads detected in the application component. </p>
    #[serde(rename = "detectedWorkload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_workload:
        Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>>,
    /// <p>Indicates whether the application component is monitored. </p>
    #[serde(rename = "monitor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<bool>,
    /// <p> The operating system of the component. </p>
    #[serde(rename = "osType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// <p>The resource type. Supported resource types include EC2 instances, Auto Scaling group, Classic ELB, Application ELB, and SQS Queue.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The stack tier of the application component.</p>
    #[serde(rename = "tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

/// <p>Describes the status of the application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationInfo {
    /// <p> Indicates whether Application Insights can listen to CloudWatch events for the application resources, such as <code>instance terminated</code>, <code>failed deployment</code>, and others. </p>
    #[serde(rename = "cWEMonitorEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwe_monitor_enabled: Option<bool>,
    /// <p>The lifecycle of the application. </p>
    #[serde(rename = "lifeCycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_cycle: Option<String>,
    /// <p> Indicates whether Application Insights will create opsItems for any problem detected by Application Insights for an application. </p>
    #[serde(rename = "opsCenterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_center_enabled: Option<bool>,
    /// <p> The SNS topic provided to Application Insights that is associated to the created opsItems to receive SNS notifications for opsItem updates. </p>
    #[serde(rename = "opsItemSNSTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_sns_topic_arn: Option<String>,
    /// <p><p>The issues on the user side that block Application Insights from successfully monitoring an application. Example remarks include:</p> <ul> <li> <p>“Configuring application, detected 1 Errors, 3 Warnings”</p> </li> <li> <p>“Configuring application, detected 1 Unconfigured Components”</p> </li> </ul></p>
    #[serde(rename = "remarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    /// <p>The name of the resource group used for the application.</p>
    #[serde(rename = "resourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

/// <p> The event information. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigurationEvent {
    /// <p> The details of the event in plain text. </p>
    #[serde(rename = "eventDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_detail: Option<String>,
    /// <p> The name of the resource Application Insights attempted to configure. </p>
    #[serde(rename = "eventResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_resource_name: Option<String>,
    /// <p> The resource type that Application Insights attempted to configure, for example, CLOUDWATCH_ALARM. </p>
    #[serde(rename = "eventResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_resource_type: Option<String>,
    /// <p> The status of the configuration update event. Possible values include INFO, WARN, and ERROR. </p>
    #[serde(rename = "eventStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_status: Option<String>,
    /// <p> The timestamp of the event. </p>
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<f64>,
    /// <p> The resource monitored by Application Insights. </p>
    #[serde(rename = "monitoredResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitored_resource_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApplicationRequest {
    /// <p> Indicates whether Application Insights can listen to CloudWatch events for the application resources, such as <code>instance terminated</code>, <code>failed deployment</code>, and others. </p>
    #[serde(rename = "cWEMonitorEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwe_monitor_enabled: Option<bool>,
    /// <p> When set to <code>true</code>, creates opsItems for any problems detected on an application. </p>
    #[serde(rename = "opsCenterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_center_enabled: Option<bool>,
    /// <p> The SNS topic provided to Application Insights that is associated to the created opsItem. Allows you to receive notifications for updates to the opsItem. </p>
    #[serde(rename = "opsItemSNSTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_sns_topic_arn: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    /// <p>List of tags to add to the application. tag key (<code>Key</code>) and an associated tag value (<code>Value</code>). The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateApplicationResponse {
    /// <p>Information about the application.</p>
    #[serde(rename = "applicationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_info: Option<ApplicationInfo>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateComponentRequest {
    /// <p>The name of the component.</p>
    #[serde(rename = "componentName")]
    pub component_name: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    /// <p>The list of resource ARNs that belong to the component.</p>
    #[serde(rename = "resourceList")]
    pub resource_list: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateComponentResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLogPatternRequest {
    /// <p>The log pattern. The pattern must be DFA compatible. Patterns that utilize forward lookahead or backreference constructions are not supported.</p>
    #[serde(rename = "pattern")]
    pub pattern: String,
    /// <p>The name of the log pattern.</p>
    #[serde(rename = "patternName")]
    pub pattern_name: String,
    /// <p>The name of the log pattern set.</p>
    #[serde(rename = "patternSetName")]
    pub pattern_set_name: String,
    /// <p>Rank of the log pattern. Must be a value between <code>1</code> and <code>1,000,000</code>. The patterns are sorted by rank, so we recommend that you set your highest priority patterns with the lowest rank. A pattern of rank <code>1</code> will be the first to get matched to a log line. A pattern of rank <code>1,000,000</code> will be last to get matched. When you configure custom log patterns from the console, a <code>Low</code> severity pattern translates to a <code>750,000</code> rank. A <code>Medium</code> severity pattern translates to a <code>500,000</code> rank. And a <code>High</code> severity pattern translates to a <code>250,000</code> rank. Rank values less than <code>1</code> or greater than <code>1,000,000</code> are reserved for AWS-provided patterns. </p>
    #[serde(rename = "rank")]
    pub rank: i64,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLogPatternResponse {
    /// <p>The successfully created log pattern.</p>
    #[serde(rename = "logPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_pattern: Option<LogPattern>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApplicationRequest {
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApplicationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteComponentRequest {
    /// <p>The name of the component.</p>
    #[serde(rename = "componentName")]
    pub component_name: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteComponentResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLogPatternRequest {
    /// <p>The name of the log pattern.</p>
    #[serde(rename = "patternName")]
    pub pattern_name: String,
    /// <p>The name of the log pattern set.</p>
    #[serde(rename = "patternSetName")]
    pub pattern_set_name: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLogPatternResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeApplicationRequest {
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeApplicationResponse {
    /// <p>Information about the application.</p>
    #[serde(rename = "applicationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_info: Option<ApplicationInfo>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeComponentConfigurationRecommendationRequest {
    /// <p>The name of the component.</p>
    #[serde(rename = "componentName")]
    pub component_name: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    /// <p>The tier of the application component. Supported tiers include <code>DOT_NET_CORE</code>, <code>DOT_NET_WORKER</code>, <code>DOT_NET_WEB</code>, <code>SQL_SERVER</code>, and <code>DEFAULT</code>.</p>
    #[serde(rename = "tier")]
    pub tier: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeComponentConfigurationRecommendationResponse {
    /// <p>The recommended configuration settings of the component. The value is the escaped JSON of the configuration.</p>
    #[serde(rename = "componentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_configuration: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeComponentConfigurationRequest {
    /// <p>The name of the component.</p>
    #[serde(rename = "componentName")]
    pub component_name: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeComponentConfigurationResponse {
    /// <p>The configuration settings of the component. The value is the escaped JSON of the configuration.</p>
    #[serde(rename = "componentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_configuration: Option<String>,
    /// <p>Indicates whether the application component is monitored.</p>
    #[serde(rename = "monitor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<bool>,
    /// <p>The tier of the application component. Supported tiers include <code>DOT_NET_CORE</code>, <code>DOT_NET_WORKER</code>, <code>DOT_NET_WEB</code>, <code>SQL_SERVER</code>, and <code>DEFAULT</code> </p>
    #[serde(rename = "tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeComponentRequest {
    /// <p>The name of the component.</p>
    #[serde(rename = "componentName")]
    pub component_name: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeComponentResponse {
    #[serde(rename = "applicationComponent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_component: Option<ApplicationComponent>,
    /// <p>The list of resource ARNs that belong to the component.</p>
    #[serde(rename = "resourceList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_list: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLogPatternRequest {
    /// <p>The name of the log pattern.</p>
    #[serde(rename = "patternName")]
    pub pattern_name: String,
    /// <p>The name of the log pattern set.</p>
    #[serde(rename = "patternSetName")]
    pub pattern_set_name: String,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLogPatternResponse {
    /// <p>The successfully created log pattern.</p>
    #[serde(rename = "logPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_pattern: Option<LogPattern>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeObservationRequest {
    /// <p>The ID of the observation.</p>
    #[serde(rename = "observationId")]
    pub observation_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeObservationResponse {
    /// <p>Information about the observation.</p>
    #[serde(rename = "observation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation: Option<Observation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProblemObservationsRequest {
    /// <p>The ID of the problem.</p>
    #[serde(rename = "problemId")]
    pub problem_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProblemObservationsResponse {
    /// <p>Observations related to the problem.</p>
    #[serde(rename = "relatedObservations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_observations: Option<RelatedObservations>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeProblemRequest {
    /// <p>The ID of the problem.</p>
    #[serde(rename = "problemId")]
    pub problem_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeProblemResponse {
    /// <p>Information about the problem. </p>
    #[serde(rename = "problem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem: Option<Problem>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListApplicationsRequest {
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListApplicationsResponse {
    /// <p>The list of applications.</p>
    #[serde(rename = "applicationInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_info_list: Option<Vec<ApplicationInfo>>,
    /// <p>The token used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListComponentsRequest {
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListComponentsResponse {
    /// <p>The list of application components.</p>
    #[serde(rename = "applicationComponentList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_component_list: Option<Vec<ApplicationComponent>>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListConfigurationHistoryRequest {
    /// <p>The end time of the event.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The status of the configuration update event. Possible values include INFO, WARN, and ERROR.</p>
    #[serde(rename = "eventStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_status: Option<String>,
    /// <p> The maximum number of results returned by <code>ListConfigurationHistory</code> in paginated output. When this parameter is used, <code>ListConfigurationHistory</code> returns only <code>MaxResults</code> in a single page along with a <code>NextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListConfigurationHistory</code> request with the returned <code>NextToken</code> value. If this parameter is not used, then <code>ListConfigurationHistory</code> returns all results. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>NextToken</code> value returned from a previous paginated <code>ListConfigurationHistory</code> request where <code>MaxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>NextToken</code> value. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Resource group to which the application belongs. </p>
    #[serde(rename = "resourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    /// <p>The start time of the event. </p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConfigurationHistoryResponse {
    /// <p> The list of configuration events and their corresponding details. </p>
    #[serde(rename = "eventList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_list: Option<Vec<ConfigurationEvent>>,
    /// <p>The <code>NextToken</code> value to include in a future <code>ListConfigurationHistory</code> request. When the results of a <code>ListConfigurationHistory</code> request exceed <code>MaxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLogPatternSetsRequest {
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLogPatternSetsResponse {
    /// <p>The list of log pattern sets.</p>
    #[serde(rename = "logPatternSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_pattern_sets: Option<Vec<String>>,
    /// <p>The token used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLogPatternsRequest {
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the log pattern set.</p>
    #[serde(rename = "patternSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_set_name: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLogPatternsResponse {
    /// <p>The list of log patterns.</p>
    #[serde(rename = "logPatterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_patterns: Option<Vec<LogPattern>>,
    /// <p>The token used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProblemsRequest {
    /// <p>The time when the problem ended, in epoch seconds. If not specified, problems within the past seven days are returned.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    /// <p>The time when the problem was detected, in epoch seconds. If you don't specify a time frame for the request, problems within the past seven days are returned.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProblemsResponse {
    /// <p>The token used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of problems. </p>
    #[serde(rename = "problemList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_list: Option<Vec<Problem>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the application that you want to retrieve tag information for.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>An array that lists all the tags that are associated with the application. Each tag consists of a required tag key (<code>Key</code>) and an associated tag value (<code>Value</code>).</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>An object that defines the log patterns that belongs to a <code>LogPatternSet</code>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LogPattern {
    /// <p>A regular expression that defines the log pattern. A log pattern can contain as many as 50 characters, and it cannot be empty. The pattern must be DFA compatible. Patterns that utilize forward lookahead or backreference constructions are not supported.</p>
    #[serde(rename = "pattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// <p>The name of the log pattern. A log pattern name can contain as many as 50 characters, and it cannot be empty. The characters can be Unicode letters, digits, or one of the following symbols: period, dash, underscore.</p>
    #[serde(rename = "patternName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_name: Option<String>,
    /// <p>The name of the log pattern. A log pattern name can contain as many as 30 characters, and it cannot be empty. The characters can be Unicode letters, digits, or one of the following symbols: period, dash, underscore.</p>
    #[serde(rename = "patternSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_set_name: Option<String>,
    /// <p>Rank of the log pattern. Must be a value between <code>1</code> and <code>1,000,000</code>. The patterns are sorted by rank, so we recommend that you set your highest priority patterns with the lowest rank. A pattern of rank <code>1</code> will be the first to get matched to a log line. A pattern of rank <code>1,000,000</code> will be last to get matched. When you configure custom log patterns from the console, a <code>Low</code> severity pattern translates to a <code>750,000</code> rank. A <code>Medium</code> severity pattern translates to a <code>500,000</code> rank. And a <code>High</code> severity pattern translates to a <code>250,000</code> rank. Rank values less than <code>1</code> or greater than <code>1,000,000</code> are reserved for AWS-provided patterns. </p>
    #[serde(rename = "rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

/// <p>Describes an anomaly or error with the application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Observation {
    /// <p> The detail type of the CloudWatch Event-based observation, for example, <code>EC2 Instance State-change Notification</code>. </p>
    #[serde(rename = "cloudWatchEventDetailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_event_detail_type: Option<String>,
    /// <p> The ID of the CloudWatch Event-based observation related to the detected problem. </p>
    #[serde(rename = "cloudWatchEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_event_id: Option<String>,
    /// <p> The source of the CloudWatch Event. </p>
    #[serde(rename = "cloudWatchEventSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_event_source: Option<String>,
    /// <p> The CodeDeploy application to which the deployment belongs. </p>
    #[serde(rename = "codeDeployApplication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_deploy_application: Option<String>,
    /// <p> The deployment group to which the CodeDeploy deployment belongs. </p>
    #[serde(rename = "codeDeployDeploymentGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_deploy_deployment_group: Option<String>,
    /// <p> The deployment ID of the CodeDeploy-based observation related to the detected problem. </p>
    #[serde(rename = "codeDeployDeploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_deploy_deployment_id: Option<String>,
    /// <p> The instance group to which the CodeDeploy instance belongs. </p>
    #[serde(rename = "codeDeployInstanceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_deploy_instance_group_id: Option<String>,
    /// <p> The status of the CodeDeploy deployment, for example <code>SUCCESS</code> or <code> FAILURE</code>. </p>
    #[serde(rename = "codeDeployState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_deploy_state: Option<String>,
    /// <p> The cause of an EBS CloudWatch event. </p>
    #[serde(rename = "ebsCause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_cause: Option<String>,
    /// <p> The type of EBS CloudWatch event, such as <code>createVolume</code>, <code>deleteVolume</code> or <code>attachVolume</code>. </p>
    #[serde(rename = "ebsEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_event: Option<String>,
    /// <p> The request ID of an EBS CloudWatch event. </p>
    #[serde(rename = "ebsRequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_request_id: Option<String>,
    /// <p> The result of an EBS CloudWatch event, such as <code>failed</code> or <code>succeeded</code>. </p>
    #[serde(rename = "ebsResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_result: Option<String>,
    /// <p> The state of the instance, such as <code>STOPPING</code> or <code>TERMINATING</code>. </p>
    #[serde(rename = "ec2State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_state: Option<String>,
    /// <p>The time when the observation ended, in epoch seconds.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p> The Amazon Resource Name (ARN) of the AWS Health Event-based observation.</p>
    #[serde(rename = "healthEventArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_event_arn: Option<String>,
    /// <p> The description of the AWS Health event provided by the service, such as Amazon EC2. </p>
    #[serde(rename = "healthEventDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_event_description: Option<String>,
    /// <p> The category of the AWS Health event, such as <code>issue</code>. </p>
    #[serde(rename = "healthEventTypeCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_event_type_category: Option<String>,
    /// <p> The type of the AWS Health event, for example, <code>AWS_EC2_POWER_CONNECTIVITY_ISSUE</code>. </p>
    #[serde(rename = "healthEventTypeCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_event_type_code: Option<String>,
    /// <p> The service to which the AWS Health Event belongs, such as EC2. </p>
    #[serde(rename = "healthService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_service: Option<String>,
    /// <p>The ID of the observation type.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The timestamp in the CloudWatch Logs that specifies when the matched line occurred.</p>
    #[serde(rename = "lineTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_time: Option<f64>,
    /// <p>The log filter of the observation.</p>
    #[serde(rename = "logFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_filter: Option<String>,
    /// <p>The log group name.</p>
    #[serde(rename = "logGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
    /// <p>The log text of the observation.</p>
    #[serde(rename = "logText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_text: Option<String>,
    /// <p>The name of the observation metric.</p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>The namespace of the observation metric.</p>
    #[serde(rename = "metricNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    /// <p> The category of an RDS event. </p>
    #[serde(rename = "rdsEventCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_event_categories: Option<String>,
    /// <p> The message of an RDS event. </p>
    #[serde(rename = "rdsEventMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_event_message: Option<String>,
    /// <p> The name of the S3 CloudWatch Event-based observation. </p>
    #[serde(rename = "s3EventName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_event_name: Option<String>,
    /// <p>The source resource ARN of the observation.</p>
    #[serde(rename = "sourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    /// <p>The source type of the observation.</p>
    #[serde(rename = "sourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>The time when the observation was first detected, in epoch seconds.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p> The Amazon Resource Name (ARN) of the step function-based observation. </p>
    #[serde(rename = "statesArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states_arn: Option<String>,
    /// <p> The Amazon Resource Name (ARN) of the step function execution-based observation. </p>
    #[serde(rename = "statesExecutionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states_execution_arn: Option<String>,
    /// <p> The input to the step function-based observation. </p>
    #[serde(rename = "statesInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states_input: Option<String>,
    /// <p> The status of the step function-related observation. </p>
    #[serde(rename = "statesStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states_status: Option<String>,
    /// <p>The unit of the source observation metric.</p>
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// <p>The value of the source observation metric.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    /// <p> The X-Ray request error percentage for this node. </p>
    #[serde(rename = "xRayErrorPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ray_error_percent: Option<i64>,
    /// <p> The X-Ray request fault percentage for this node. </p>
    #[serde(rename = "xRayFaultPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ray_fault_percent: Option<i64>,
    /// <p> The name of the X-Ray node. </p>
    #[serde(rename = "xRayNodeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ray_node_name: Option<String>,
    /// <p> The type of the X-Ray node. </p>
    #[serde(rename = "xRayNodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ray_node_type: Option<String>,
    /// <p> The X-Ray node request average latency for this node. </p>
    #[serde(rename = "xRayRequestAverageLatency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ray_request_average_latency: Option<i64>,
    /// <p> The X-Ray request count for this node. </p>
    #[serde(rename = "xRayRequestCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ray_request_count: Option<i64>,
    /// <p> The X-Ray request throttle percentage for this node. </p>
    #[serde(rename = "xRayThrottlePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_ray_throttle_percent: Option<i64>,
}

/// <p>Describes a problem that is detected by correlating observations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Problem {
    /// <p>The resource affected by the problem.</p>
    #[serde(rename = "affectedResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_resource: Option<String>,
    /// <p>The time when the problem ended, in epoch seconds.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>Feedback provided by the user about the problem.</p>
    #[serde(rename = "feedback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<::std::collections::HashMap<String, String>>,
    /// <p>The ID of the problem.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A detailed analysis of the problem using machine learning.</p>
    #[serde(rename = "insights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights: Option<String>,
    /// <p>The name of the resource group affected by the problem.</p>
    #[serde(rename = "resourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    /// <p>A measure of the level of impact of the problem.</p>
    #[serde(rename = "severityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_level: Option<String>,
    /// <p>The time when the problem started, in epoch seconds.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the problem.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The name of the problem.</p>
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// <p>Describes observations related to the problem.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RelatedObservations {
    /// <p>The list of observations related to the problem.</p>
    #[serde(rename = "observationList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation_list: Option<Vec<Observation>>,
}

/// <p><p>An object that defines the tags associated with an application. A <i>tag</i> is a label that you optionally define and associate with an application. Tags can help you categorize and manage resources in different ways, such as by purpose, owner, environment, or other criteria.</p> <p>Each tag consists of a required <i>tag key</i> and an associated <i>tag value</i>, both of which you define. A tag key is a general label that acts as a category for a more specific tag value. A tag value acts as a descriptor within a tag key. A tag key can contain as many as 128 characters. A tag value can contain as many as 256 characters. The characters can be Unicode letters, digits, white space, or one of the following symbols: _ . : / = + -. The following additional restrictions apply to tags:</p> <ul> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>For each associated resource, each tag key must be unique and it can have only one value.</p> </li> <li> <p>The <code>aws:</code> prefix is reserved for use by AWS; you can’t use it in any tag keys or values that you define. In addition, you can&#39;t edit or remove tag keys or values that use this prefix. </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>One part of a key-value pair that defines a tag. The maximum length of a tag key is 128 characters. The minimum length is 1 character.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The optional part of a key-value pair that defines a tag. The maximum length of a tag value is 256 characters. The minimum length is 0 characters. If you don't want an application to have a specific tag value, don't specify a value for this parameter.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the application that you want to add one or more tags to.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>A list of tags that to add to the application. A tag consists of a required tag key (<code>Key</code>) and an associated tag value (<code>Value</code>). The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the application that you want to remove one or more tags from.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>The tags (tag keys) that you want to remove from the resource. When you specify a tag key, the action removes both that key and its associated tag value.</p> <p>To remove more than one tag from the application, append the <code>TagKeys</code> parameter and argument for each additional tag to remove, separated by an ampersand. </p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApplicationRequest {
    /// <p> Indicates whether Application Insights can listen to CloudWatch events for the application resources, such as <code>instance terminated</code>, <code>failed deployment</code>, and others. </p>
    #[serde(rename = "cWEMonitorEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwe_monitor_enabled: Option<bool>,
    /// <p> When set to <code>true</code>, creates opsItems for any problems detected on an application. </p>
    #[serde(rename = "opsCenterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_center_enabled: Option<bool>,
    /// <p> The SNS topic provided to Application Insights that is associated to the created opsItem. Allows you to receive notifications for updates to the opsItem.</p>
    #[serde(rename = "opsItemSNSTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_sns_topic_arn: Option<String>,
    /// <p> Disassociates the SNS topic from the opsItem created for detected problems.</p>
    #[serde(rename = "removeSNSTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_sns_topic: Option<bool>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApplicationResponse {
    /// <p>Information about the application. </p>
    #[serde(rename = "applicationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_info: Option<ApplicationInfo>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateComponentConfigurationRequest {
    /// <p>The configuration settings of the component. The value is the escaped JSON of the configuration. For more information about the JSON format, see <a href="https://docs.aws.amazon.com/sdk-for-javascript/v2/developer-guide/working-with-json.html">Working with JSON</a>. You can send a request to <code>DescribeComponentConfigurationRecommendation</code> to see the recommended configuration for a component. For the complete format of the component configuration file, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/component-config.html">Component Configuration</a>.</p>
    #[serde(rename = "componentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_configuration: Option<String>,
    /// <p>The name of the component.</p>
    #[serde(rename = "componentName")]
    pub component_name: String,
    /// <p>Indicates whether the application component is monitored.</p>
    #[serde(rename = "monitor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<bool>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    /// <p>The tier of the application component. Supported tiers include <code>DOT_NET_WORKER</code>, <code>DOT_NET_WEB</code>, <code>DOT_NET_CORE</code>, <code>SQL_SERVER</code>, and <code>DEFAULT</code>.</p>
    #[serde(rename = "tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateComponentConfigurationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateComponentRequest {
    /// <p>The name of the component.</p>
    #[serde(rename = "componentName")]
    pub component_name: String,
    /// <p>The new name of the component.</p>
    #[serde(rename = "newComponentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_component_name: Option<String>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    /// <p>The list of resource ARNs that belong to the component.</p>
    #[serde(rename = "resourceList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_list: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateComponentResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateLogPatternRequest {
    /// <p>The log pattern. The pattern must be DFA compatible. Patterns that utilize forward lookahead or backreference constructions are not supported.</p>
    #[serde(rename = "pattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// <p>The name of the log pattern.</p>
    #[serde(rename = "patternName")]
    pub pattern_name: String,
    /// <p>The name of the log pattern set.</p>
    #[serde(rename = "patternSetName")]
    pub pattern_set_name: String,
    /// <p>Rank of the log pattern. Must be a value between <code>1</code> and <code>1,000,000</code>. The patterns are sorted by rank, so we recommend that you set your highest priority patterns with the lowest rank. A pattern of rank <code>1</code> will be the first to get matched to a log line. A pattern of rank <code>1,000,000</code> will be last to get matched. When you configure custom log patterns from the console, a <code>Low</code> severity pattern translates to a <code>750,000</code> rank. A <code>Medium</code> severity pattern translates to a <code>500,000</code> rank. And a <code>High</code> severity pattern translates to a <code>250,000</code> rank. Rank values less than <code>1</code> or greater than <code>1,000,000</code> are reserved for AWS-provided patterns. </p>
    #[serde(rename = "rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateLogPatternResponse {
    /// <p>The successfully created log pattern.</p>
    #[serde(rename = "logPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_pattern: Option<LogPattern>,
    /// <p>The name of the resource group.</p>
    #[serde(rename = "resourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
}

/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    /// <p> User does not have permissions to perform this action. </p>
    AccessDenied(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource is already created or in use.</p>
    ResourceInUse(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
    /// <p>Tags are already registered for the specified application ARN.</p>
    TagsAlreadyExist(String),
}

impl CreateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateApplicationError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CreateApplicationError::InternalServer(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateApplicationError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateApplicationError::ResourceNotFound(err.msg))
                }
                "TagsAlreadyExistException" => {
                    return RusotoError::Service(CreateApplicationError::TagsAlreadyExist(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateApplicationError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::TagsAlreadyExist(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateApplicationError {}
/// Errors returned by CreateComponent
#[derive(Debug, PartialEq)]
pub enum CreateComponentError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource is already created or in use.</p>
    ResourceInUse(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl CreateComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateComponentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateComponentError::InternalServer(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateComponentError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateComponentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateComponentError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateComponentError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateComponentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateComponentError {}
/// Errors returned by CreateLogPattern
#[derive(Debug, PartialEq)]
pub enum CreateLogPatternError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource is already created or in use.</p>
    ResourceInUse(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl CreateLogPatternError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLogPatternError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateLogPatternError::InternalServer(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateLogPatternError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateLogPatternError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateLogPatternError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLogPatternError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateLogPatternError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateLogPatternError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLogPatternError {}
/// Errors returned by DeleteApplication
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationError {
    /// <p>The request is not understood by the server.</p>
    BadRequest(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DeleteApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteApplicationError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteApplicationError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteApplicationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApplicationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApplicationError {}
/// Errors returned by DeleteComponent
#[derive(Debug, PartialEq)]
pub enum DeleteComponentError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DeleteComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteComponentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteComponentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteComponentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteComponentError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteComponentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteComponentError {}
/// Errors returned by DeleteLogPattern
#[derive(Debug, PartialEq)]
pub enum DeleteLogPatternError {
    /// <p>The request is not understood by the server.</p>
    BadRequest(String),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DeleteLogPatternError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLogPatternError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteLogPatternError::BadRequest(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(DeleteLogPatternError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteLogPatternError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLogPatternError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLogPatternError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteLogPatternError::InternalServer(ref cause) => write!(f, "{}", cause),
            DeleteLogPatternError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLogPatternError {}
/// Errors returned by DescribeApplication
#[derive(Debug, PartialEq)]
pub enum DescribeApplicationError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeApplicationError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeApplicationError::ResourceNotFound(
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
impl fmt::Display for DescribeApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeApplicationError {}
/// Errors returned by DescribeComponent
#[derive(Debug, PartialEq)]
pub enum DescribeComponentError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeComponentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeComponentError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeComponentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeComponentError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeComponentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeComponentError {}
/// Errors returned by DescribeComponentConfiguration
#[derive(Debug, PartialEq)]
pub enum DescribeComponentConfigurationError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeComponentConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeComponentConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeComponentConfigurationError::InternalServer(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeComponentConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeComponentConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeComponentConfigurationError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeComponentConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeComponentConfigurationError {}
/// Errors returned by DescribeComponentConfigurationRecommendation
#[derive(Debug, PartialEq)]
pub enum DescribeComponentConfigurationRecommendationError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeComponentConfigurationRecommendationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeComponentConfigurationRecommendationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(
                        DescribeComponentConfigurationRecommendationError::InternalServer(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeComponentConfigurationRecommendationError::ResourceNotFound(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeComponentConfigurationRecommendationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeComponentConfigurationRecommendationError::InternalServer(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeComponentConfigurationRecommendationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeComponentConfigurationRecommendationError {}
/// Errors returned by DescribeLogPattern
#[derive(Debug, PartialEq)]
pub enum DescribeLogPatternError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeLogPatternError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLogPatternError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeLogPatternError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeLogPatternError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeLogPatternError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLogPatternError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeLogPatternError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLogPatternError {}
/// Errors returned by DescribeObservation
#[derive(Debug, PartialEq)]
pub enum DescribeObservationError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeObservationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeObservationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeObservationError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeObservationError::ResourceNotFound(
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
impl fmt::Display for DescribeObservationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeObservationError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeObservationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeObservationError {}
/// Errors returned by DescribeProblem
#[derive(Debug, PartialEq)]
pub enum DescribeProblemError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeProblemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeProblemError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeProblemError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeProblemError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProblemError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProblemError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeProblemError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProblemError {}
/// Errors returned by DescribeProblemObservations
#[derive(Debug, PartialEq)]
pub enum DescribeProblemObservationsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl DescribeProblemObservationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeProblemObservationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeProblemObservationsError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeProblemObservationsError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeProblemObservationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeProblemObservationsError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeProblemObservationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeProblemObservationsError {}
/// Errors returned by ListApplications
#[derive(Debug, PartialEq)]
pub enum ListApplicationsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
}

impl ListApplicationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApplicationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListApplicationsError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListApplicationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListApplicationsError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListApplicationsError {}
/// Errors returned by ListComponents
#[derive(Debug, PartialEq)]
pub enum ListComponentsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl ListComponentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListComponentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListComponentsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListComponentsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListComponentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListComponentsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListComponentsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListComponentsError {}
/// Errors returned by ListConfigurationHistory
#[derive(Debug, PartialEq)]
pub enum ListConfigurationHistoryError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl ListConfigurationHistoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConfigurationHistoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListConfigurationHistoryError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListConfigurationHistoryError::ResourceNotFound(
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
impl fmt::Display for ListConfigurationHistoryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListConfigurationHistoryError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListConfigurationHistoryError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListConfigurationHistoryError {}
/// Errors returned by ListLogPatternSets
#[derive(Debug, PartialEq)]
pub enum ListLogPatternSetsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl ListLogPatternSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLogPatternSetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListLogPatternSetsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListLogPatternSetsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLogPatternSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLogPatternSetsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListLogPatternSetsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLogPatternSetsError {}
/// Errors returned by ListLogPatterns
#[derive(Debug, PartialEq)]
pub enum ListLogPatternsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl ListLogPatternsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLogPatternsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListLogPatternsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListLogPatternsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLogPatternsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLogPatternsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListLogPatternsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLogPatternsError {}
/// Errors returned by ListProblems
#[derive(Debug, PartialEq)]
pub enum ListProblemsError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl ListProblemsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListProblemsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListProblemsError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListProblemsError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProblemsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProblemsError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListProblemsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListProblemsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
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
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
    /// <p>The number of the provided tags is beyond the limit, or the number of total tags you are trying to attach to the specified resource exceeds the limit.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
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
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
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
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateApplication
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl UpdateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateApplicationError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateApplicationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApplicationError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApplicationError {}
/// Errors returned by UpdateComponent
#[derive(Debug, PartialEq)]
pub enum UpdateComponentError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource is already created or in use.</p>
    ResourceInUse(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl UpdateComponentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateComponentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateComponentError::InternalServer(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateComponentError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateComponentError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateComponentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateComponentError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateComponentError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateComponentError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateComponentError {}
/// Errors returned by UpdateComponentConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateComponentConfigurationError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl UpdateComponentConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateComponentConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateComponentConfigurationError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateComponentConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateComponentConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateComponentConfigurationError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateComponentConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateComponentConfigurationError {}
/// Errors returned by UpdateLogPattern
#[derive(Debug, PartialEq)]
pub enum UpdateLogPatternError {
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServer(String),
    /// <p>The resource is already created or in use.</p>
    ResourceInUse(String),
    /// <p>The resource does not exist in the customer account.</p>
    ResourceNotFound(String),
}

impl UpdateLogPatternError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateLogPatternError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UpdateLogPatternError::InternalServer(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateLogPatternError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateLogPatternError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateLogPatternError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateLogPatternError::InternalServer(ref cause) => write!(f, "{}", cause),
            UpdateLogPatternError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateLogPatternError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateLogPatternError {}
/// Trait representing the capabilities of the Application Insights API. Application Insights clients implement this trait.
#[async_trait]
pub trait ApplicationInsights {
    /// <p>Adds an application that is created from a resource group.</p>
    async fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> Result<CreateApplicationResponse, RusotoError<CreateApplicationError>>;

    /// <p>Creates a custom component by grouping similar standalone instances to monitor.</p>
    async fn create_component(
        &self,
        input: CreateComponentRequest,
    ) -> Result<CreateComponentResponse, RusotoError<CreateComponentError>>;

    /// <p>Adds an log pattern to a <code>LogPatternSet</code>.</p>
    async fn create_log_pattern(
        &self,
        input: CreateLogPatternRequest,
    ) -> Result<CreateLogPatternResponse, RusotoError<CreateLogPatternError>>;

    /// <p>Removes the specified application from monitoring. Does not delete the application.</p>
    async fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> Result<DeleteApplicationResponse, RusotoError<DeleteApplicationError>>;

    /// <p>Ungroups a custom component. When you ungroup custom components, all applicable monitors that are set up for the component are removed and the instances revert to their standalone status.</p>
    async fn delete_component(
        &self,
        input: DeleteComponentRequest,
    ) -> Result<DeleteComponentResponse, RusotoError<DeleteComponentError>>;

    /// <p>Removes the specified log pattern from a <code>LogPatternSet</code>.</p>
    async fn delete_log_pattern(
        &self,
        input: DeleteLogPatternRequest,
    ) -> Result<DeleteLogPatternResponse, RusotoError<DeleteLogPatternError>>;

    /// <p>Describes the application.</p>
    async fn describe_application(
        &self,
        input: DescribeApplicationRequest,
    ) -> Result<DescribeApplicationResponse, RusotoError<DescribeApplicationError>>;

    /// <p>Describes a component and lists the resources that are grouped together in a component.</p>
    async fn describe_component(
        &self,
        input: DescribeComponentRequest,
    ) -> Result<DescribeComponentResponse, RusotoError<DescribeComponentError>>;

    /// <p>Describes the monitoring configuration of the component.</p>
    async fn describe_component_configuration(
        &self,
        input: DescribeComponentConfigurationRequest,
    ) -> Result<
        DescribeComponentConfigurationResponse,
        RusotoError<DescribeComponentConfigurationError>,
    >;

    /// <p>Describes the recommended monitoring configuration of the component.</p>
    async fn describe_component_configuration_recommendation(
        &self,
        input: DescribeComponentConfigurationRecommendationRequest,
    ) -> Result<
        DescribeComponentConfigurationRecommendationResponse,
        RusotoError<DescribeComponentConfigurationRecommendationError>,
    >;

    /// <p>Describe a specific log pattern from a <code>LogPatternSet</code>.</p>
    async fn describe_log_pattern(
        &self,
        input: DescribeLogPatternRequest,
    ) -> Result<DescribeLogPatternResponse, RusotoError<DescribeLogPatternError>>;

    /// <p>Describes an anomaly or error with the application.</p>
    async fn describe_observation(
        &self,
        input: DescribeObservationRequest,
    ) -> Result<DescribeObservationResponse, RusotoError<DescribeObservationError>>;

    /// <p>Describes an application problem.</p>
    async fn describe_problem(
        &self,
        input: DescribeProblemRequest,
    ) -> Result<DescribeProblemResponse, RusotoError<DescribeProblemError>>;

    /// <p>Describes the anomalies or errors associated with the problem.</p>
    async fn describe_problem_observations(
        &self,
        input: DescribeProblemObservationsRequest,
    ) -> Result<DescribeProblemObservationsResponse, RusotoError<DescribeProblemObservationsError>>;

    /// <p>Lists the IDs of the applications that you are monitoring. </p>
    async fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> Result<ListApplicationsResponse, RusotoError<ListApplicationsError>>;

    /// <p>Lists the auto-grouped, standalone, and custom components of the application.</p>
    async fn list_components(
        &self,
        input: ListComponentsRequest,
    ) -> Result<ListComponentsResponse, RusotoError<ListComponentsError>>;

    /// <p><p> Lists the INFO, WARN, and ERROR events for periodic configuration updates performed by Application Insights. Examples of events represented are: </p> <ul> <li> <p>INFO: creating a new alarm or updating an alarm threshold.</p> </li> <li> <p>WARN: alarm not created due to insufficient data points used to predict thresholds.</p> </li> <li> <p>ERROR: alarm not created due to permission errors or exceeding quotas. </p> </li> </ul></p>
    async fn list_configuration_history(
        &self,
        input: ListConfigurationHistoryRequest,
    ) -> Result<ListConfigurationHistoryResponse, RusotoError<ListConfigurationHistoryError>>;

    /// <p>Lists the log pattern sets in the specific application.</p>
    async fn list_log_pattern_sets(
        &self,
        input: ListLogPatternSetsRequest,
    ) -> Result<ListLogPatternSetsResponse, RusotoError<ListLogPatternSetsError>>;

    /// <p>Lists the log patterns in the specific log <code>LogPatternSet</code>.</p>
    async fn list_log_patterns(
        &self,
        input: ListLogPatternsRequest,
    ) -> Result<ListLogPatternsResponse, RusotoError<ListLogPatternsError>>;

    /// <p>Lists the problems with your application.</p>
    async fn list_problems(
        &self,
        input: ListProblemsRequest,
    ) -> Result<ListProblemsResponse, RusotoError<ListProblemsError>>;

    /// <p>Retrieve a list of the tags (keys and values) that are associated with a specified application. A <i>tag</i> is a label that you optionally define and associate with an application. Each tag consists of a required <i>tag key</i> and an optional associated <i>tag value</i>. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Add one or more tags (keys and values) to a specified application. A <i>tag</i> is a label that you optionally define and associate with an application. Tags can help you categorize and manage application in different ways, such as by purpose, owner, environment, or other criteria. </p> <p>Each tag consists of a required <i>tag key</i> and an associated <i>tag value</i>, both of which you define. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Remove one or more tags (keys and values) from a specified application.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates the application.</p>
    async fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> Result<UpdateApplicationResponse, RusotoError<UpdateApplicationError>>;

    /// <p>Updates the custom component name and/or the list of resources that make up the component.</p>
    async fn update_component(
        &self,
        input: UpdateComponentRequest,
    ) -> Result<UpdateComponentResponse, RusotoError<UpdateComponentError>>;

    /// <p>Updates the monitoring configurations for the component. The configuration input parameter is an escaped JSON of the configuration and should match the schema of what is returned by <code>DescribeComponentConfigurationRecommendation</code>. </p>
    async fn update_component_configuration(
        &self,
        input: UpdateComponentConfigurationRequest,
    ) -> Result<UpdateComponentConfigurationResponse, RusotoError<UpdateComponentConfigurationError>>;

    /// <p>Adds a log pattern to a <code>LogPatternSet</code>.</p>
    async fn update_log_pattern(
        &self,
        input: UpdateLogPatternRequest,
    ) -> Result<UpdateLogPatternResponse, RusotoError<UpdateLogPatternError>>;
}
/// A client for the Application Insights API.
#[derive(Clone)]
pub struct ApplicationInsightsClient {
    client: Client,
    region: region::Region,
}

impl ApplicationInsightsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ApplicationInsightsClient {
        ApplicationInsightsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ApplicationInsightsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ApplicationInsightsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ApplicationInsightsClient {
        ApplicationInsightsClient { client, region }
    }
}

#[async_trait]
impl ApplicationInsights for ApplicationInsightsClient {
    /// <p>Adds an application that is created from a resource group.</p>
    async fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> Result<CreateApplicationResponse, RusotoError<CreateApplicationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.CreateApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateApplicationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateApplicationResponse, _>()
    }

    /// <p>Creates a custom component by grouping similar standalone instances to monitor.</p>
    async fn create_component(
        &self,
        input: CreateComponentRequest,
    ) -> Result<CreateComponentResponse, RusotoError<CreateComponentError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.CreateComponent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateComponentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateComponentResponse, _>()
    }

    /// <p>Adds an log pattern to a <code>LogPatternSet</code>.</p>
    async fn create_log_pattern(
        &self,
        input: CreateLogPatternRequest,
    ) -> Result<CreateLogPatternResponse, RusotoError<CreateLogPatternError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.CreateLogPattern");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateLogPatternError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateLogPatternResponse, _>()
    }

    /// <p>Removes the specified application from monitoring. Does not delete the application.</p>
    async fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> Result<DeleteApplicationResponse, RusotoError<DeleteApplicationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.DeleteApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteApplicationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteApplicationResponse, _>()
    }

    /// <p>Ungroups a custom component. When you ungroup custom components, all applicable monitors that are set up for the component are removed and the instances revert to their standalone status.</p>
    async fn delete_component(
        &self,
        input: DeleteComponentRequest,
    ) -> Result<DeleteComponentResponse, RusotoError<DeleteComponentError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.DeleteComponent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteComponentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteComponentResponse, _>()
    }

    /// <p>Removes the specified log pattern from a <code>LogPatternSet</code>.</p>
    async fn delete_log_pattern(
        &self,
        input: DeleteLogPatternRequest,
    ) -> Result<DeleteLogPatternResponse, RusotoError<DeleteLogPatternError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.DeleteLogPattern");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteLogPatternError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteLogPatternResponse, _>()
    }

    /// <p>Describes the application.</p>
    async fn describe_application(
        &self,
        input: DescribeApplicationRequest,
    ) -> Result<DescribeApplicationResponse, RusotoError<DescribeApplicationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.DescribeApplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeApplicationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeApplicationResponse, _>()
    }

    /// <p>Describes a component and lists the resources that are grouped together in a component.</p>
    async fn describe_component(
        &self,
        input: DescribeComponentRequest,
    ) -> Result<DescribeComponentResponse, RusotoError<DescribeComponentError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.DescribeComponent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeComponentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeComponentResponse, _>()
    }

    /// <p>Describes the monitoring configuration of the component.</p>
    async fn describe_component_configuration(
        &self,
        input: DescribeComponentConfigurationRequest,
    ) -> Result<
        DescribeComponentConfigurationResponse,
        RusotoError<DescribeComponentConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.DescribeComponentConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeComponentConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeComponentConfigurationResponse, _>()
    }

    /// <p>Describes the recommended monitoring configuration of the component.</p>
    async fn describe_component_configuration_recommendation(
        &self,
        input: DescribeComponentConfigurationRecommendationRequest,
    ) -> Result<
        DescribeComponentConfigurationRecommendationResponse,
        RusotoError<DescribeComponentConfigurationRecommendationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.DescribeComponentConfigurationRecommendation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DescribeComponentConfigurationRecommendationError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeComponentConfigurationRecommendationResponse, _>()
    }

    /// <p>Describe a specific log pattern from a <code>LogPatternSet</code>.</p>
    async fn describe_log_pattern(
        &self,
        input: DescribeLogPatternRequest,
    ) -> Result<DescribeLogPatternResponse, RusotoError<DescribeLogPatternError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.DescribeLogPattern");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeLogPatternError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeLogPatternResponse, _>()
    }

    /// <p>Describes an anomaly or error with the application.</p>
    async fn describe_observation(
        &self,
        input: DescribeObservationRequest,
    ) -> Result<DescribeObservationResponse, RusotoError<DescribeObservationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.DescribeObservation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeObservationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeObservationResponse, _>()
    }

    /// <p>Describes an application problem.</p>
    async fn describe_problem(
        &self,
        input: DescribeProblemRequest,
    ) -> Result<DescribeProblemResponse, RusotoError<DescribeProblemError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.DescribeProblem");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeProblemError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeProblemResponse, _>()
    }

    /// <p>Describes the anomalies or errors associated with the problem.</p>
    async fn describe_problem_observations(
        &self,
        input: DescribeProblemObservationsRequest,
    ) -> Result<DescribeProblemObservationsResponse, RusotoError<DescribeProblemObservationsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.DescribeProblemObservations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeProblemObservationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeProblemObservationsResponse, _>()
    }

    /// <p>Lists the IDs of the applications that you are monitoring. </p>
    async fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> Result<ListApplicationsResponse, RusotoError<ListApplicationsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.ListApplications");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListApplicationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListApplicationsResponse, _>()
    }

    /// <p>Lists the auto-grouped, standalone, and custom components of the application.</p>
    async fn list_components(
        &self,
        input: ListComponentsRequest,
    ) -> Result<ListComponentsResponse, RusotoError<ListComponentsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.ListComponents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListComponentsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListComponentsResponse, _>()
    }

    /// <p><p> Lists the INFO, WARN, and ERROR events for periodic configuration updates performed by Application Insights. Examples of events represented are: </p> <ul> <li> <p>INFO: creating a new alarm or updating an alarm threshold.</p> </li> <li> <p>WARN: alarm not created due to insufficient data points used to predict thresholds.</p> </li> <li> <p>ERROR: alarm not created due to permission errors or exceeding quotas. </p> </li> </ul></p>
    async fn list_configuration_history(
        &self,
        input: ListConfigurationHistoryRequest,
    ) -> Result<ListConfigurationHistoryResponse, RusotoError<ListConfigurationHistoryError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.ListConfigurationHistory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListConfigurationHistoryError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListConfigurationHistoryResponse, _>()
    }

    /// <p>Lists the log pattern sets in the specific application.</p>
    async fn list_log_pattern_sets(
        &self,
        input: ListLogPatternSetsRequest,
    ) -> Result<ListLogPatternSetsResponse, RusotoError<ListLogPatternSetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.ListLogPatternSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListLogPatternSetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListLogPatternSetsResponse, _>()
    }

    /// <p>Lists the log patterns in the specific log <code>LogPatternSet</code>.</p>
    async fn list_log_patterns(
        &self,
        input: ListLogPatternsRequest,
    ) -> Result<ListLogPatternsResponse, RusotoError<ListLogPatternsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.ListLogPatterns");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListLogPatternsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListLogPatternsResponse, _>()
    }

    /// <p>Lists the problems with your application.</p>
    async fn list_problems(
        &self,
        input: ListProblemsRequest,
    ) -> Result<ListProblemsResponse, RusotoError<ListProblemsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.ListProblems");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListProblemsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListProblemsResponse, _>()
    }

    /// <p>Retrieve a list of the tags (keys and values) that are associated with a specified application. A <i>tag</i> is a label that you optionally define and associate with an application. Each tag consists of a required <i>tag key</i> and an optional associated <i>tag value</i>. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.ListTagsForResource",
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

    /// <p>Add one or more tags (keys and values) to a specified application. A <i>tag</i> is a label that you optionally define and associate with an application. Tags can help you categorize and manage application in different ways, such as by purpose, owner, environment, or other criteria. </p> <p>Each tag consists of a required <i>tag key</i> and an associated <i>tag value</i>, both of which you define. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Remove one or more tags (keys and values) from a specified application.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p>Updates the application.</p>
    async fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> Result<UpdateApplicationResponse, RusotoError<UpdateApplicationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.UpdateApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateApplicationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateApplicationResponse, _>()
    }

    /// <p>Updates the custom component name and/or the list of resources that make up the component.</p>
    async fn update_component(
        &self,
        input: UpdateComponentRequest,
    ) -> Result<UpdateComponentResponse, RusotoError<UpdateComponentError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.UpdateComponent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateComponentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateComponentResponse, _>()
    }

    /// <p>Updates the monitoring configurations for the component. The configuration input parameter is an escaped JSON of the configuration and should match the schema of what is returned by <code>DescribeComponentConfigurationRecommendation</code>. </p>
    async fn update_component_configuration(
        &self,
        input: UpdateComponentConfigurationRequest,
    ) -> Result<UpdateComponentConfigurationResponse, RusotoError<UpdateComponentConfigurationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "EC2WindowsBarleyService.UpdateComponentConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateComponentConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateComponentConfigurationResponse, _>()
    }

    /// <p>Adds a log pattern to a <code>LogPatternSet</code>.</p>
    async fn update_log_pattern(
        &self,
        input: UpdateLogPatternRequest,
    ) -> Result<UpdateLogPatternResponse, RusotoError<UpdateLogPatternError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "EC2WindowsBarleyService.UpdateLogPattern");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateLogPatternError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateLogPatternResponse, _>()
    }
}
