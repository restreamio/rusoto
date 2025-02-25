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

impl ResourceGroupsTaggingApiClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "tagging", &self.region, request_uri);

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
/// <p>Information that shows whether a resource is compliant with the effective tag policy, including details on any noncompliant tag keys.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ComplianceDetails {
    /// <p>Whether a resource is compliant with the effective tag policy.</p>
    #[serde(rename = "complianceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<bool>,
    /// <p>These are keys defined in the effective policy that are on the resource with either incorrect case treatment or noncompliant values. </p>
    #[serde(rename = "keysWithNoncompliantValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys_with_noncompliant_values: Option<Vec<String>>,
    /// <p>These tag keys on the resource are noncompliant with the effective tag policy.</p>
    #[serde(rename = "noncompliantKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncompliant_keys: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReportCreationInput {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReportCreationOutput {
    /// <p>Details of the common errors that all operations return.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The path to the Amazon S3 bucket where the report was stored on creation.</p>
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<String>,
    /// <p><p>Reports the status of the operation.</p> <p>The operation status can be one of the following:</p> <ul> <li> <p> <code>RUNNING</code> - Report creation is in progress.</p> </li> <li> <p> <code>SUCCEEDED</code> - Report creation is complete. You can open the report from the Amazon S3 bucket that you specified when you ran <code>StartReportCreation</code>.</p> </li> <li> <p> <code>FAILED</code> - Report creation timed out or the Amazon S3 bucket is not accessible. </p> </li> <li> <p> <code>NO REPORT</code> - No report was generated in the last 90 days.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about the errors that are returned for each failed resource. This information can include <code>InternalServiceException</code> and <code>InvalidParameterException</code> errors. It can also include any valid error code returned by the AWS service that hosts the resource that the ARN key represents.</p> <p>The following are common error codes that you might receive from other AWS services:</p> <ul> <li> <p> <b>InternalServiceException</b> – This can mean that the Resource Groups Tagging API didn't receive a response from another AWS service. It can also mean the the resource type in the request is not supported by the Resource Groups Tagging API. In these cases, it's safe to retry the request and then call <a href="http://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/API_GetResources.html">GetResources</a> to verify the changes.</p> </li> <li> <p> <b>AccessDeniedException</b> – This can mean that you need permission to calling tagging operations in the AWS service that contains the resource. For example, to use the Resource Groups Tagging API to tag a CloudWatch alarm resource, you need permission to call <a href="http://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/API_TagResources.html"> <code>TagResources</code> </a> <i>and</i> <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_TagResource.html"> <code>TagResource</code> </a> in the CloudWatch API. </p> </li> </ul> <p>For more information on errors that are generated from other AWS services, see the documentation for that service. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailureInfo {
    /// <p>The code of the common error. Valid values include <code>InternalServiceException</code>, <code>InvalidParameterException</code>, and any valid error code returned by the AWS service that hosts the resource that you want to tag.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The message of the common error.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The HTTP status code of the common error.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetComplianceSummaryInput {
    /// <p>Specifies a list of attributes to group the counts of noncompliant resources by. If supplied, the counts are sorted by those attributes.</p>
    #[serde(rename = "groupBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<String>>,
    /// <p>Specifies the maximum number of results to be returned in each page. A query can return fewer than this maximum, even if there are more results still to return. You should always check the <code>PaginationToken</code> response value to see if there are more results. You can specify a minimum of 1 and a maximum value of 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Specifies a <code>PaginationToken</code> response value from a previous request to indicate that you want the next page of results. Leave this parameter empty in your initial request.</p>
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>Specifies a list of AWS Regions to limit the output by. If you use this parameter, the count of returned noncompliant resources includes only resources in the specified Regions.</p>
    #[serde(rename = "regionFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_filters: Option<Vec<String>>,
    /// <p>Specifies that you want the response to include information for only resources of the specified types. The format of each resource type is <code>service[:resourceType]</code>. For example, specifying a resource type of <code>ec2</code> returns all Amazon EC2 resources (which includes EC2 instances). Specifying a resource type of <code>ec2:instance</code> returns only EC2 instances. </p> <p>The string for each service name and resource type is the same as that embedded in a resource's Amazon Resource Name (ARN). Consult the <i>AWS General Reference</i> for the following:</p> <ul> <li> <p>For a list of service name strings, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a>.</p> </li> <li> <p>For resource type strings, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arns-syntax">Example ARNs</a>.</p> </li> <li> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p> </li> </ul> <p>You can specify multiple resource types by using a comma separated array. The array can include up to 100 items. Note that the length constraint requirement applies to each resource type filter. </p>
    #[serde(rename = "resourceTypeFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_filters: Option<Vec<String>>,
    /// <p>Specifies that you want the response to include information for only resources that have tags with the specified tag keys. If you use this parameter, the count of returned noncompliant resources includes only resources that have the specified tag keys.</p>
    #[serde(rename = "tagKeyFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key_filters: Option<Vec<String>>,
    /// <p>Specifies target identifiers (usually, specific account IDs) to limit the output by. If you use this parameter, the count of returned noncompliant resources includes only resources with the specified target IDs.</p>
    #[serde(rename = "targetIdFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id_filters: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetComplianceSummaryOutput {
    /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>A table that shows counts of noncompliant resources.</p>
    #[serde(rename = "summaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_list: Option<Vec<Summary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetResourcesInput {
    /// <p>Specifies whether to exclude resources that are compliant with the tag policy. Set this to <code>true</code> if you are interested in retrieving information on noncompliant resources only.</p> <p>You can use this parameter only if the <code>IncludeComplianceDetails</code> parameter is also set to <code>true</code>.</p>
    #[serde(rename = "excludeCompliantResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_compliant_resources: Option<bool>,
    /// <p>Specifies whether to include details regarding the compliance with the effective tag policy. Set this to <code>true</code> to determine whether resources are compliant with the tag policy and to get details.</p>
    #[serde(rename = "includeComplianceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_compliance_details: Option<bool>,
    /// <p>Specifies a <code>PaginationToken</code> response value from a previous request to indicate that you want the next page of results. Leave this parameter empty in your initial request.</p>
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>Specifies a list of ARNs of resources for which you want to retrieve tag data. You can't specify both this parameter and any of the pagination parameters (<code>ResourcesPerPage</code>, <code>TagsPerPage</code>, <code>PaginationToken</code>) in the same request. If you specify both, you get an <code>Invalid Parameter</code> exception.</p> <p>If a resource specified by this parameter doesn't exist, it doesn't generate an error; it simply isn't included in the response.</p> <p>An ARN (Amazon Resource Name) uniquely identifies a resource. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "resourceARNList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn_list: Option<Vec<String>>,
    /// <p>Specifies the resource types that you want included in the response. The format of each resource type is <code>service[:resourceType]</code>. For example, specifying a resource type of <code>ec2</code> returns all Amazon EC2 resources (which includes EC2 instances). Specifying a resource type of <code>ec2:instance</code> returns only EC2 instances. </p> <p>The string for each service name and resource type is the same as that embedded in a resource's Amazon Resource Name (ARN). Consult the <i>AWS General Reference</i> for the following:</p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p> <p>You can specify multiple resource types by using an array. The array can include up to 100 items. Note that the length constraint requirement applies to each resource type filter. </p>
    #[serde(rename = "resourceTypeFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_filters: Option<Vec<String>>,
    /// <p>Specifies the maximum number of results to be returned in each page. A query can return fewer than this maximum, even if there are more results still to return. You should always check the <code>PaginationToken</code> response value to see if there are more results. You can specify a minimum of 1 and a maximum value of 100.</p>
    #[serde(rename = "resourcesPerPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_per_page: Option<i64>,
    /// <p><p>Specifies a list of TagFilters (keys and values) to restrict the output to only those resources that have the specified tag and, if included, the specified value. Each <code>TagFilter</code> must contain a key with values optional. A request can include up to 50 keys, and each key can include up to 20 values. </p> <p>Note the following when deciding how to use TagFilters:</p> <ul> <li> <p>If you <i>don&#39;t</i> specify a <code>TagFilter</code>, the response includes all resources that are currently tagged or ever had a tag. Resources that currently don&#39;t have tags are shown with an empty tag set, like this: <code>&quot;Tags&quot;: []</code>.</p> </li> <li> <p>If you specify more than one filter in a single request, the response returns only those resources that satisfy all filters.</p> </li> <li> <p>If you specify a filter that contains more than one value for a key, the response returns resources that match any of the specified values for that key.</p> </li> <li> <p>If you don&#39;t specify any values for a key, the response returns resources that are tagged with that key and any or no value.</p> <p>For example, for the following filters: <code>filter1= {keyA,{value1}}</code>, <code>filter2={keyB,{value2,value3,value4}}</code>, <code>filter3= {keyC}</code>:</p> <ul> <li> <p> <code>GetResources({filter1})</code> returns resources tagged with <code>key1=value1</code> </p> </li> <li> <p> <code>GetResources({filter2})</code> returns resources tagged with <code>key2=value2</code> or <code>key2=value3</code> or <code>key2=value4</code> </p> </li> <li> <p> <code>GetResources({filter3})</code> returns resources tagged with any tag with the key <code>key3</code>, and with any or no value</p> </li> <li> <p> <code>GetResources({filter1,filter2,filter3})</code> returns resources tagged with <code>(key1=value1) and (key2=value2 or key2=value3 or key2=value4) and (key3, any or no value)</code> </p> </li> </ul> </li> </ul></p>
    #[serde(rename = "tagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
    /// <p>AWS recommends using <code>ResourcesPerPage</code> instead of this parameter.</p> <p>A limit that restricts the number of tags (key and value pairs) returned by <code>GetResources</code> in paginated output. A resource with no tags is counted as having one tag (one key and value pair).</p> <p> <code>GetResources</code> does not split a resource and its associated tags across pages. If the specified <code>TagsPerPage</code> would cause such a break, a <code>PaginationToken</code> is returned in place of the affected resource and its tags. Use that token in another request to get the remaining data. For example, if you specify a <code>TagsPerPage</code> of <code>100</code> and the account has 22 resources with 10 tags each (meaning that each resource has 10 key and value pairs), the output will consist of three pages. The first page displays the first 10 resources, each with its 10 tags. The second page displays the next 10 resources, each with its 10 tags. The third page displays the remaining 2 resources, each with its 10 tags.</p> <p>You can set <code>TagsPerPage</code> to a minimum of 100 items up to a maximum of 500 items.</p>
    #[serde(rename = "tagsPerPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_per_page: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetResourcesOutput {
    /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>A list of resource ARNs and the tags (keys and values) associated with those ARNs.</p>
    #[serde(rename = "resourceTagMappingList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tag_mapping_list: Option<Vec<ResourceTagMapping>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTagKeysInput {
    /// <p>Specifies a <code>PaginationToken</code> response value from a previous request to indicate that you want the next page of results. Leave this parameter empty in your initial request.</p>
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTagKeysOutput {
    /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>A list of all tag keys in the AWS account.</p>
    #[serde(rename = "tagKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetTagValuesInput {
    /// <p>Specifies the tag key for which you want to list all existing values that are currently used in the specified AWS Region for the calling AWS account.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>Specifies a <code>PaginationToken</code> response value from a previous request to indicate that you want the next page of results. Leave this parameter empty in your initial request.</p>
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetTagValuesOutput {
    /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    #[serde(rename = "paginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>A list of all tag values for the specified key currently used in the specified AWS Region for the calling AWS account.</p>
    #[serde(rename = "tagValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

/// <p>A list of resource ARNs and the tags (keys and values) that are associated with each.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceTagMapping {
    /// <p>Information that shows whether a resource is compliant with the effective tag policy, including details on any noncompliant tag keys.</p>
    #[serde(rename = "complianceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_details: Option<ComplianceDetails>,
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "resourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The tags that have been applied to one or more AWS resources.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartReportCreationInput {
    /// <p>The name of the Amazon S3 bucket where the report will be stored; for example:</p> <p> <code>awsexamplebucket</code> </p> <p>For more information on S3 bucket requirements, including an example bucket policy, see the example S3 bucket policy on this page.</p>
    #[serde(rename = "s3Bucket")]
    pub s3_bucket: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartReportCreationOutput {}

/// <p>A count of noncompliant resources.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Summary {
    /// <p>The timestamp that shows when this summary was generated in this Region. </p>
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// <p>The count of noncompliant resources.</p>
    #[serde(rename = "nonCompliantResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_resources: Option<i64>,
    /// <p>The AWS Region that the summary applies to.</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The AWS resource type.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The account identifier or the root identifier of the organization. If you don't know the root ID, you can call the AWS Organizations <a href="http://docs.aws.amazon.com/organizations/latest/APIReference/API_ListRoots.html">ListRoots</a> API.</p>
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// <p>Whether the target is an account, an OU, or the organization root.</p>
    #[serde(rename = "targetIdType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id_type: Option<String>,
}

/// <p>The metadata that you apply to AWS resources to help you categorize and organize them. Each tag consists of a key and a value, both of which you define. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging AWS Resources</a> in the <i>AWS General Reference</i>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Tag {
    /// <p>One part of a key-value pair that makes up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>One part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). The value can be empty or null.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>A list of tags (keys and values) that are used to specify the associated resources.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagFilter {
    /// <p>One part of a key-value pair that makes up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>One part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key). The value can be empty or null.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourcesInput {
    /// <p>Specifies the list of ARNs of the resources that you want to apply tags to.</p> <p>An ARN (Amazon Resource Name) uniquely identifies a resource. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "resourceARNList")]
    pub resource_arn_list: Vec<String>,
    /// <p>Specifies a list of tags that you want to add to the specified resources. A tag consists of a key and a value that you define.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourcesOutput {
    /// <p>A map containing a key-value pair for each failed item that couldn't be tagged. The key is the ARN of the failed resource. The value is a <code>FailureInfo</code> object that contains an error code, a status code, and an error message. If there are no errors, the <code>FailedResourcesMap</code> is empty.</p>
    #[serde(rename = "failedResourcesMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_resources_map: Option<::std::collections::HashMap<String, FailureInfo>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourcesInput {
    /// <p>Specifies a list of ARNs of the resources that you want to remove tags from.</p> <p>An ARN (Amazon Resource Name) uniquely identifies a resource. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "resourceARNList")]
    pub resource_arn_list: Vec<String>,
    /// <p>Specifies a list of tag keys that you want to remove from the specified resources.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourcesOutput {
    /// <p>A map containing a key-value pair for each failed item that couldn't be untagged. The key is the ARN of the failed resource. The value is a <code>FailureInfo</code> object that contains an error code, a status code, and an error message. If there are no errors, the <code>FailedResourcesMap</code> is empty.</p>
    #[serde(rename = "failedResourcesMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_resources_map: Option<::std::collections::HashMap<String, FailureInfo>>,
}

/// Errors returned by DescribeReportCreation
#[derive(Debug, PartialEq)]
pub enum DescribeReportCreationError {
    /// <p><p>The request was denied because performing this operation violates a constraint. </p> <p>Some of the reasons in the following list might not apply to this specific operation.</p> <ul> <li> <p>You must meet the prerequisites for using tag policies. For information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html">Prerequisites and Permissions for Using Tag Policies</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>You must enable the tag policies service principal (<code>tagpolicies.tag.amazonaws.com</code>) to integrate with AWS Organizations For information, see <a href="http://docs.aws.amazon.com/organizations/latest/APIReference/API_EnableAWSServiceAccess.html">EnableAWSServiceAccess</a>.</p> </li> <li> <p>You must have a tag policy attached to the organization root, an OU, or an account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl DescribeReportCreationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeReportCreationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConstraintViolationException" => {
                    return RusotoError::Service(DescribeReportCreationError::ConstraintViolation(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(DescribeReportCreationError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeReportCreationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ThrottledException" => {
                    return RusotoError::Service(DescribeReportCreationError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeReportCreationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReportCreationError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            DescribeReportCreationError::InternalService(ref cause) => write!(f, "{}", cause),
            DescribeReportCreationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeReportCreationError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeReportCreationError {}
/// Errors returned by GetComplianceSummary
#[derive(Debug, PartialEq)]
pub enum GetComplianceSummaryError {
    /// <p><p>The request was denied because performing this operation violates a constraint. </p> <p>Some of the reasons in the following list might not apply to this specific operation.</p> <ul> <li> <p>You must meet the prerequisites for using tag policies. For information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html">Prerequisites and Permissions for Using Tag Policies</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>You must enable the tag policies service principal (<code>tagpolicies.tag.amazonaws.com</code>) to integrate with AWS Organizations For information, see <a href="http://docs.aws.amazon.com/organizations/latest/APIReference/API_EnableAWSServiceAccess.html">EnableAWSServiceAccess</a>.</p> </li> <li> <p>You must have a tag policy attached to the organization root, an OU, or an account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl GetComplianceSummaryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetComplianceSummaryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConstraintViolationException" => {
                    return RusotoError::Service(GetComplianceSummaryError::ConstraintViolation(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(GetComplianceSummaryError::InternalService(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetComplianceSummaryError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetComplianceSummaryError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetComplianceSummaryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetComplianceSummaryError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            GetComplianceSummaryError::InternalService(ref cause) => write!(f, "{}", cause),
            GetComplianceSummaryError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetComplianceSummaryError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetComplianceSummaryError {}
/// Errors returned by GetResources
#[derive(Debug, PartialEq)]
pub enum GetResourcesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>A <code>PaginationToken</code> is valid for a maximum of 15 minutes. Your request was denied because the specified <code>PaginationToken</code> has expired.</p>
    PaginationTokenExpired(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl GetResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetResourcesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetResourcesError::InvalidParameter(err.msg))
                }
                "PaginationTokenExpiredException" => {
                    return RusotoError::Service(GetResourcesError::PaginationTokenExpired(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetResourcesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetResourcesError::InternalService(ref cause) => write!(f, "{}", cause),
            GetResourcesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetResourcesError::PaginationTokenExpired(ref cause) => write!(f, "{}", cause),
            GetResourcesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetResourcesError {}
/// Errors returned by GetTagKeys
#[derive(Debug, PartialEq)]
pub enum GetTagKeysError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>A <code>PaginationToken</code> is valid for a maximum of 15 minutes. Your request was denied because the specified <code>PaginationToken</code> has expired.</p>
    PaginationTokenExpired(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl GetTagKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTagKeysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetTagKeysError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetTagKeysError::InvalidParameter(err.msg))
                }
                "PaginationTokenExpiredException" => {
                    return RusotoError::Service(GetTagKeysError::PaginationTokenExpired(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetTagKeysError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTagKeysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTagKeysError::InternalService(ref cause) => write!(f, "{}", cause),
            GetTagKeysError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetTagKeysError::PaginationTokenExpired(ref cause) => write!(f, "{}", cause),
            GetTagKeysError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTagKeysError {}
/// Errors returned by GetTagValues
#[derive(Debug, PartialEq)]
pub enum GetTagValuesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>A <code>PaginationToken</code> is valid for a maximum of 15 minutes. Your request was denied because the specified <code>PaginationToken</code> has expired.</p>
    PaginationTokenExpired(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl GetTagValuesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTagValuesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetTagValuesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetTagValuesError::InvalidParameter(err.msg))
                }
                "PaginationTokenExpiredException" => {
                    return RusotoError::Service(GetTagValuesError::PaginationTokenExpired(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetTagValuesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetTagValuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetTagValuesError::InternalService(ref cause) => write!(f, "{}", cause),
            GetTagValuesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetTagValuesError::PaginationTokenExpired(ref cause) => write!(f, "{}", cause),
            GetTagValuesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetTagValuesError {}
/// Errors returned by StartReportCreation
#[derive(Debug, PartialEq)]
pub enum StartReportCreationError {
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModification(String),
    /// <p><p>The request was denied because performing this operation violates a constraint. </p> <p>Some of the reasons in the following list might not apply to this specific operation.</p> <ul> <li> <p>You must meet the prerequisites for using tag policies. For information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html">Prerequisites and Permissions for Using Tag Policies</a> in the <i>AWS Organizations User Guide.</i> </p> </li> <li> <p>You must enable the tag policies service principal (<code>tagpolicies.tag.amazonaws.com</code>) to integrate with AWS Organizations For information, see <a href="http://docs.aws.amazon.com/organizations/latest/APIReference/API_EnableAWSServiceAccess.html">EnableAWSServiceAccess</a>.</p> </li> <li> <p>You must have a tag policy attached to the organization root, an OU, or an account.</p> </li> </ul></p>
    ConstraintViolation(String),
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl StartReportCreationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartReportCreationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(StartReportCreationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "ConstraintViolationException" => {
                    return RusotoError::Service(StartReportCreationError::ConstraintViolation(
                        err.msg,
                    ))
                }
                "InternalServiceException" => {
                    return RusotoError::Service(StartReportCreationError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartReportCreationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ThrottledException" => {
                    return RusotoError::Service(StartReportCreationError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartReportCreationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartReportCreationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            StartReportCreationError::ConstraintViolation(ref cause) => write!(f, "{}", cause),
            StartReportCreationError::InternalService(ref cause) => write!(f, "{}", cause),
            StartReportCreationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartReportCreationError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartReportCreationError {}
/// Errors returned by TagResources
#[derive(Debug, PartialEq)]
pub enum TagResourcesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl TagResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(TagResourcesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourcesError::InvalidParameter(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(TagResourcesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourcesError::InternalService(ref cause) => write!(f, "{}", cause),
            TagResourcesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourcesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourcesError {}
/// Errors returned by UntagResources
#[derive(Debug, PartialEq)]
pub enum UntagResourcesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p><p>This error indicates one of the following:</p> <ul> <li> <p>A parameter is missing.</p> </li> <li> <p>A malformed string was supplied for the request parameter.</p> </li> <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> <li> <p>The target ID is invalid, unsupported, or doesn&#39;t exist.</p> </li> <li> <p>You can&#39;t access the Amazon S3 bucket for report storage. For more information, see <a href="http://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul></p>
    InvalidParameter(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl UntagResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UntagResourcesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourcesError::InvalidParameter(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(UntagResourcesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourcesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourcesError::InternalService(ref cause) => write!(f, "{}", cause),
            UntagResourcesError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourcesError::Throttled(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourcesError {}
/// Trait representing the capabilities of the AWS Resource Groups Tagging API API. AWS Resource Groups Tagging API clients implement this trait.
#[async_trait]
pub trait ResourceGroupsTaggingApi {
    /// <p>Describes the status of the <code>StartReportCreation</code> operation. </p> <p>You can call this operation only from the organization's management account and from the us-east-1 Region.</p>
    async fn describe_report_creation(
        &self,
    ) -> Result<DescribeReportCreationOutput, RusotoError<DescribeReportCreationError>>;

    /// <p>Returns a table that shows counts of resources that are noncompliant with their tag policies.</p> <p>For more information on tag policies, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">Tag Policies</a> in the <i>AWS Organizations User Guide.</i> </p> <p>You can call this operation only from the organization's management account and from the us-east-1 Region.</p> <p>This operation supports pagination, where the response can be sent in multiple pages. You should check the <code>PaginationToken</code> response parameter to determine if there are additional results available to return. Repeat the query, passing the <code>PaginationToken</code> response parameter value as an input to the next request until you recieve a <code>null</code> value. A null value for <code>PaginationToken</code> indicates that there are no more results waiting to be returned.</p>
    async fn get_compliance_summary(
        &self,
        input: GetComplianceSummaryInput,
    ) -> Result<GetComplianceSummaryOutput, RusotoError<GetComplianceSummaryError>>;

    /// <p>Returns all the tagged or previously tagged resources that are located in the specified Region for the AWS account.</p> <p>Depending on what information you want returned, you can also specify the following:</p> <ul> <li> <p> <i>Filters</i> that specify what tags and resource types you want returned. The response includes all tags that are associated with the requested resources.</p> </li> <li> <p>Information about compliance with the account's effective tag policy. For more information on tag policies, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">Tag Policies</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul> <p>This operation supports pagination, where the response can be sent in multiple pages. You should check the <code>PaginationToken</code> response parameter to determine if there are additional results available to return. Repeat the query, passing the <code>PaginationToken</code> response parameter value as an input to the next request until you recieve a <code>null</code> value. A null value for <code>PaginationToken</code> indicates that there are no more results waiting to be returned.</p>
    async fn get_resources(
        &self,
        input: GetResourcesInput,
    ) -> Result<GetResourcesOutput, RusotoError<GetResourcesError>>;

    /// <p>Returns all tag keys currently in use in the specified Region for the calling AWS account.</p> <p>This operation supports pagination, where the response can be sent in multiple pages. You should check the <code>PaginationToken</code> response parameter to determine if there are additional results available to return. Repeat the query, passing the <code>PaginationToken</code> response parameter value as an input to the next request until you recieve a <code>null</code> value. A null value for <code>PaginationToken</code> indicates that there are no more results waiting to be returned.</p>
    async fn get_tag_keys(
        &self,
        input: GetTagKeysInput,
    ) -> Result<GetTagKeysOutput, RusotoError<GetTagKeysError>>;

    /// <p>Returns all tag values for the specified key that are used in the specified AWS Region for the calling AWS account.</p> <p>This operation supports pagination, where the response can be sent in multiple pages. You should check the <code>PaginationToken</code> response parameter to determine if there are additional results available to return. Repeat the query, passing the <code>PaginationToken</code> response parameter value as an input to the next request until you recieve a <code>null</code> value. A null value for <code>PaginationToken</code> indicates that there are no more results waiting to be returned.</p>
    async fn get_tag_values(
        &self,
        input: GetTagValuesInput,
    ) -> Result<GetTagValuesOutput, RusotoError<GetTagValuesError>>;

    /// <p>Generates a report that lists all tagged resources in the accounts across your organization and tells whether each resource is compliant with the effective tag policy. Compliance data is refreshed daily. The report is generated asynchronously.</p> <p>The generated report is saved to the following location:</p> <p> <code>s3://example-bucket/AwsTagPolicies/o-exampleorgid/YYYY-MM-ddTHH:mm:ssZ/report.csv</code> </p> <p>You can call this operation only from the organization's management account and from the us-east-1 Region.</p>
    async fn start_report_creation(
        &self,
        input: StartReportCreationInput,
    ) -> Result<StartReportCreationOutput, RusotoError<StartReportCreationError>>;

    /// <p><p>Applies one or more tags to the specified resources. Note the following:</p> <ul> <li> <p>Not all resources can have tags. For a list of services with resources that support tagging using this operation, see <a href="https://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/supported-services.html">Services that support the Resource Groups Tagging API</a>.</p> </li> <li> <p>Each resource can have up to 50 tags. For other limits, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html#tag-conventions">Tag Naming and Usage Conventions</a> in the <i>AWS General Reference.</i> </p> </li> <li> <p>You can only tag resources that are located in the specified AWS Region for the AWS account.</p> </li> <li> <p>To add tags to a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for adding tags. For more information, see the documentation for each service.</p> </li> </ul> <important> <p>Do not store personally identifiable information (PII) or other confidential or sensitive information in tags. We use tags to provide you with billing and administration services. Tags are not intended to be used for private or sensitive data.</p> </important></p>
    async fn tag_resources(
        &self,
        input: TagResourcesInput,
    ) -> Result<TagResourcesOutput, RusotoError<TagResourcesError>>;

    /// <p><p>Removes the specified tags from the specified resources. When you specify a tag key, the action removes both that key and its associated value. The operation succeeds even if you attempt to remove tags from a resource that were already removed. Note the following:</p> <ul> <li> <p>To remove tags from a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for removing tags. For more information, see the documentation for the service whose resource you want to untag.</p> </li> <li> <p>You can only tag resources that are located in the specified AWS Region for the calling AWS account.</p> </li> </ul></p>
    async fn untag_resources(
        &self,
        input: UntagResourcesInput,
    ) -> Result<UntagResourcesOutput, RusotoError<UntagResourcesError>>;
}
/// A client for the AWS Resource Groups Tagging API API.
#[derive(Clone)]
pub struct ResourceGroupsTaggingApiClient {
    client: Client,
    region: region::Region,
}

impl ResourceGroupsTaggingApiClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ResourceGroupsTaggingApiClient {
        ResourceGroupsTaggingApiClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ResourceGroupsTaggingApiClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ResourceGroupsTaggingApiClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(
        client: Client,
        region: region::Region,
    ) -> ResourceGroupsTaggingApiClient {
        ResourceGroupsTaggingApiClient { client, region }
    }
}

#[async_trait]
impl ResourceGroupsTaggingApi for ResourceGroupsTaggingApiClient {
    /// <p>Describes the status of the <code>StartReportCreation</code> operation. </p> <p>You can call this operation only from the organization's management account and from the us-east-1 Region.</p>
    async fn describe_report_creation(
        &self,
    ) -> Result<DescribeReportCreationOutput, RusotoError<DescribeReportCreationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.DescribeReportCreation",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DescribeReportCreationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeReportCreationOutput, _>()
    }

    /// <p>Returns a table that shows counts of resources that are noncompliant with their tag policies.</p> <p>For more information on tag policies, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">Tag Policies</a> in the <i>AWS Organizations User Guide.</i> </p> <p>You can call this operation only from the organization's management account and from the us-east-1 Region.</p> <p>This operation supports pagination, where the response can be sent in multiple pages. You should check the <code>PaginationToken</code> response parameter to determine if there are additional results available to return. Repeat the query, passing the <code>PaginationToken</code> response parameter value as an input to the next request until you recieve a <code>null</code> value. A null value for <code>PaginationToken</code> indicates that there are no more results waiting to be returned.</p>
    async fn get_compliance_summary(
        &self,
        input: GetComplianceSummaryInput,
    ) -> Result<GetComplianceSummaryOutput, RusotoError<GetComplianceSummaryError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.GetComplianceSummary",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetComplianceSummaryError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetComplianceSummaryOutput, _>()
    }

    /// <p>Returns all the tagged or previously tagged resources that are located in the specified Region for the AWS account.</p> <p>Depending on what information you want returned, you can also specify the following:</p> <ul> <li> <p> <i>Filters</i> that specify what tags and resource types you want returned. The response includes all tags that are associated with the requested resources.</p> </li> <li> <p>Information about compliance with the account's effective tag policy. For more information on tag policies, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies.html">Tag Policies</a> in the <i>AWS Organizations User Guide.</i> </p> </li> </ul> <p>This operation supports pagination, where the response can be sent in multiple pages. You should check the <code>PaginationToken</code> response parameter to determine if there are additional results available to return. Repeat the query, passing the <code>PaginationToken</code> response parameter value as an input to the next request until you recieve a <code>null</code> value. A null value for <code>PaginationToken</code> indicates that there are no more results waiting to be returned.</p>
    async fn get_resources(
        &self,
        input: GetResourcesInput,
    ) -> Result<GetResourcesOutput, RusotoError<GetResourcesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.GetResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetResourcesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetResourcesOutput, _>()
    }

    /// <p>Returns all tag keys currently in use in the specified Region for the calling AWS account.</p> <p>This operation supports pagination, where the response can be sent in multiple pages. You should check the <code>PaginationToken</code> response parameter to determine if there are additional results available to return. Repeat the query, passing the <code>PaginationToken</code> response parameter value as an input to the next request until you recieve a <code>null</code> value. A null value for <code>PaginationToken</code> indicates that there are no more results waiting to be returned.</p>
    async fn get_tag_keys(
        &self,
        input: GetTagKeysInput,
    ) -> Result<GetTagKeysOutput, RusotoError<GetTagKeysError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.GetTagKeys",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetTagKeysError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetTagKeysOutput, _>()
    }

    /// <p>Returns all tag values for the specified key that are used in the specified AWS Region for the calling AWS account.</p> <p>This operation supports pagination, where the response can be sent in multiple pages. You should check the <code>PaginationToken</code> response parameter to determine if there are additional results available to return. Repeat the query, passing the <code>PaginationToken</code> response parameter value as an input to the next request until you recieve a <code>null</code> value. A null value for <code>PaginationToken</code> indicates that there are no more results waiting to be returned.</p>
    async fn get_tag_values(
        &self,
        input: GetTagValuesInput,
    ) -> Result<GetTagValuesOutput, RusotoError<GetTagValuesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.GetTagValues",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetTagValuesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetTagValuesOutput, _>()
    }

    /// <p>Generates a report that lists all tagged resources in the accounts across your organization and tells whether each resource is compliant with the effective tag policy. Compliance data is refreshed daily. The report is generated asynchronously.</p> <p>The generated report is saved to the following location:</p> <p> <code>s3://example-bucket/AwsTagPolicies/o-exampleorgid/YYYY-MM-ddTHH:mm:ssZ/report.csv</code> </p> <p>You can call this operation only from the organization's management account and from the us-east-1 Region.</p>
    async fn start_report_creation(
        &self,
        input: StartReportCreationInput,
    ) -> Result<StartReportCreationOutput, RusotoError<StartReportCreationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.StartReportCreation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartReportCreationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StartReportCreationOutput, _>()
    }

    /// <p><p>Applies one or more tags to the specified resources. Note the following:</p> <ul> <li> <p>Not all resources can have tags. For a list of services with resources that support tagging using this operation, see <a href="https://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/supported-services.html">Services that support the Resource Groups Tagging API</a>.</p> </li> <li> <p>Each resource can have up to 50 tags. For other limits, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html#tag-conventions">Tag Naming and Usage Conventions</a> in the <i>AWS General Reference.</i> </p> </li> <li> <p>You can only tag resources that are located in the specified AWS Region for the AWS account.</p> </li> <li> <p>To add tags to a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for adding tags. For more information, see the documentation for each service.</p> </li> </ul> <important> <p>Do not store personally identifiable information (PII) or other confidential or sensitive information in tags. We use tags to provide you with billing and administration services. Tags are not intended to be used for private or sensitive data.</p> </important></p>
    async fn tag_resources(
        &self,
        input: TagResourcesInput,
    ) -> Result<TagResourcesOutput, RusotoError<TagResourcesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.TagResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourcesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourcesOutput, _>()
    }

    /// <p><p>Removes the specified tags from the specified resources. When you specify a tag key, the action removes both that key and its associated value. The operation succeeds even if you attempt to remove tags from a resource that were already removed. Note the following:</p> <ul> <li> <p>To remove tags from a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for removing tags. For more information, see the documentation for the service whose resource you want to untag.</p> </li> <li> <p>You can only tag resources that are located in the specified AWS Region for the calling AWS account.</p> </li> </ul></p>
    async fn untag_resources(
        &self,
        input: UntagResourcesInput,
    ) -> Result<UntagResourcesOutput, RusotoError<UntagResourcesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.UntagResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourcesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourcesOutput, _>()
    }
}
