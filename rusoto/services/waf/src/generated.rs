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

impl WafClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "waf", &self.region, request_uri);

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
/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>The <code>ActivatedRule</code> object in an <a>UpdateWebACL</a> request specifies a <code>Rule</code> that you want to insert or delete, the priority of the <code>Rule</code> in the <code>WebACL</code>, and the action that you want AWS WAF to take when a web request matches the <code>Rule</code> (<code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>).</p> <p>To specify whether to insert or delete a <code>Rule</code>, use the <code>Action</code> parameter in the <a>WebACLUpdate</a> data type.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ActivatedRule {
    /// <p>Specifies the action that CloudFront or AWS WAF takes when a web request matches the conditions in the <code>Rule</code>. Valid values for <code>Action</code> include the following:</p> <ul> <li> <p> <code>ALLOW</code>: CloudFront responds with the requested object.</p> </li> <li> <p> <code>BLOCK</code>: CloudFront responds with an HTTP 403 (Forbidden) status code.</p> </li> <li> <p> <code>COUNT</code>: AWS WAF increments a counter of requests that match the conditions in the rule and then continues to inspect the web request based on the remaining rules in the web ACL. </p> </li> </ul> <p> <code>ActivatedRule|OverrideAction</code> applies only when updating or adding a <code>RuleGroup</code> to a <code>WebACL</code>. In this case, you do not use <code>ActivatedRule|Action</code>. For all other update requests, <code>ActivatedRule|Action</code> is used instead of <code>ActivatedRule|OverrideAction</code>.</p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<WafAction>,
    /// <p><p>An array of rules to exclude from a rule group. This is applicable only when the <code>ActivatedRule</code> refers to a <code>RuleGroup</code>.</p> <p>Sometimes it is necessary to troubleshoot rule groups that are blocking traffic unexpectedly (false positives). One troubleshooting technique is to identify the specific rule within the rule group that is blocking the legitimate traffic and then disable (exclude) that particular rule. You can exclude rules from both your own rule groups and AWS Marketplace rule groups that have been associated with a web ACL.</p> <p>Specifying <code>ExcludedRules</code> does not remove those rules from the rule group. Rather, it changes the action for the rules to <code>COUNT</code>. Therefore, requests that match an <code>ExcludedRule</code> are counted but not blocked. The <code>RuleGroup</code> owner will receive COUNT metrics for each <code>ExcludedRule</code>.</p> <p>If you want to exclude rules from a rule group that is already associated with a web ACL, perform the following steps:</p> <ol> <li> <p>Use the AWS WAF logs to identify the IDs of the rules that you want to exclude. For more information about the logs, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/logging.html">Logging Web ACL Traffic Information</a>.</p> </li> <li> <p>Submit an <a>UpdateWebACL</a> request that has two actions:</p> <ul> <li> <p>The first action deletes the existing rule group from the web ACL. That is, in the <a>UpdateWebACL</a> request, the first <code>Updates:Action</code> should be <code>DELETE</code> and <code>Updates:ActivatedRule:RuleId</code> should be the rule group that contains the rules that you want to exclude.</p> </li> <li> <p>The second action inserts the same rule group back in, but specifying the rules to exclude. That is, the second <code>Updates:Action</code> should be <code>INSERT</code>, <code>Updates:ActivatedRule:RuleId</code> should be the rule group that you just removed, and <code>ExcludedRules</code> should contain the rules that you want to exclude.</p> </li> </ul> </li> </ol></p>
    #[serde(rename = "excludedRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_rules: Option<Vec<ExcludedRule>>,
    /// <p>Use the <code>OverrideAction</code> to test your <code>RuleGroup</code>.</p> <p>Any rule in a <code>RuleGroup</code> can potentially block a request. If you set the <code>OverrideAction</code> to <code>None</code>, the <code>RuleGroup</code> will block a request if any individual rule in the <code>RuleGroup</code> matches the request and is configured to block that request. However if you first want to test the <code>RuleGroup</code>, set the <code>OverrideAction</code> to <code>Count</code>. The <code>RuleGroup</code> will then override any block action specified by individual rules contained within the group. Instead of blocking matching requests, those requests will be counted. You can view a record of counted requests using <a>GetSampledRequests</a>. </p> <p> <code>ActivatedRule|OverrideAction</code> applies only when updating or adding a <code>RuleGroup</code> to a <code>WebACL</code>. In this case you do not use <code>ActivatedRule|Action</code>. For all other update requests, <code>ActivatedRule|Action</code> is used instead of <code>ActivatedRule|OverrideAction</code>.</p>
    #[serde(rename = "overrideAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_action: Option<WafOverrideAction>,
    /// <p>Specifies the order in which the <code>Rules</code> in a <code>WebACL</code> are evaluated. Rules with a lower value for <code>Priority</code> are evaluated before <code>Rules</code> with a higher value. The value must be a unique integer. If you add multiple <code>Rules</code> to a <code>WebACL</code>, the values don't need to be consecutive.</p>
    #[serde(rename = "priority")]
    pub priority: i64,
    /// <p>The <code>RuleId</code> for a <code>Rule</code>. You use <code>RuleId</code> to get more information about a <code>Rule</code> (see <a>GetRule</a>), update a <code>Rule</code> (see <a>UpdateRule</a>), insert a <code>Rule</code> into a <code>WebACL</code> or delete a one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>Rule</code> from AWS WAF (see <a>DeleteRule</a>).</p> <p> <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    /// <p>The rule type, either <code>REGULAR</code>, as defined by <a>Rule</a>, <code>RATE_BASED</code>, as defined by <a>RateBasedRule</a>, or <code>GROUP</code>, as defined by <a>RuleGroup</a>. The default is REGULAR. Although this field is optional, be aware that if you try to add a RATE_BASED rule to a web ACL without setting the type, the <a>UpdateWebACL</a> request will fail because the request tries to add a REGULAR rule with the specified ID, which does not exist. </p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>In a <a>GetByteMatchSet</a> request, <code>ByteMatchSet</code> is a complex type that contains the <code>ByteMatchSetId</code> and <code>Name</code> of a <code>ByteMatchSet</code>, and the values that you specified when you updated the <code>ByteMatchSet</code>. </p> <p>A complex type that contains <code>ByteMatchTuple</code> objects, which specify the parts of web requests that you want AWS WAF to inspect and the values that you want AWS WAF to search for. If a <code>ByteMatchSet</code> contains more than one <code>ByteMatchTuple</code> object, a request needs to match the settings in only one <code>ByteMatchTuple</code> to be considered a match.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ByteMatchSet {
    /// <p>The <code>ByteMatchSetId</code> for a <code>ByteMatchSet</code>. You use <code>ByteMatchSetId</code> to get information about a <code>ByteMatchSet</code> (see <a>GetByteMatchSet</a>), update a <code>ByteMatchSet</code> (see <a>UpdateByteMatchSet</a>), insert a <code>ByteMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>ByteMatchSet</code> from AWS WAF (see <a>DeleteByteMatchSet</a>).</p> <p> <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>
    #[serde(rename = "byteMatchSetId")]
    pub byte_match_set_id: String,
    /// <p>Specifies the bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to search for in web requests, the location in requests that you want AWS WAF to search, and other settings.</p>
    #[serde(rename = "byteMatchTuples")]
    pub byte_match_tuples: Vec<ByteMatchTuple>,
    /// <p>A friendly name or description of the <a>ByteMatchSet</a>. You can't change <code>Name</code> after you create a <code>ByteMatchSet</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returned by <a>ListByteMatchSets</a>. Each <code>ByteMatchSetSummary</code> object includes the <code>Name</code> and <code>ByteMatchSetId</code> for one <a>ByteMatchSet</a>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ByteMatchSetSummary {
    /// <p>The <code>ByteMatchSetId</code> for a <code>ByteMatchSet</code>. You use <code>ByteMatchSetId</code> to get information about a <code>ByteMatchSet</code>, update a <code>ByteMatchSet</code>, remove a <code>ByteMatchSet</code> from a <code>Rule</code>, and delete a <code>ByteMatchSet</code> from AWS WAF.</p> <p> <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>
    #[serde(rename = "byteMatchSetId")]
    pub byte_match_set_id: String,
    /// <p>A friendly name or description of the <a>ByteMatchSet</a>. You can't change <code>Name</code> after you create a <code>ByteMatchSet</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>In an <a>UpdateByteMatchSet</a> request, <code>ByteMatchSetUpdate</code> specifies whether to insert or delete a <a>ByteMatchTuple</a> and includes the settings for the <code>ByteMatchTuple</code>.</p></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ByteMatchSetUpdate {
    /// <p>Specifies whether to insert or delete a <a>ByteMatchTuple</a>.</p>
    #[serde(rename = "action")]
    pub action: String,
    /// <p>Information about the part of a web request that you want AWS WAF to inspect and the value that you want AWS WAF to search for. If you specify <code>DELETE</code> for the value of <code>Action</code>, the <code>ByteMatchTuple</code> values must exactly match the values in the <code>ByteMatchTuple</code> that you want to delete from the <code>ByteMatchSet</code>.</p>
    #[serde(rename = "byteMatchTuple")]
    pub byte_match_tuple: ByteMatchTuple,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>The bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to search for in web requests, the location in requests that you want AWS WAF to search, and other settings.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ByteMatchTuple {
    /// <p>The part of a web request that you want AWS WAF to search, such as a specified header or a query string. For more information, see <a>FieldToMatch</a>.</p>
    #[serde(rename = "fieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>Within the portion of a web request that you want to search (for example, in the query string, if any), specify where you want AWS WAF to search. Valid values include the following:</p> <p> <b>CONTAINS</b> </p> <p>The specified part of the web request must include the value of <code>TargetString</code>, but the location doesn't matter.</p> <p> <b>CONTAINS_WORD</b> </p> <p>The specified part of the web request must include the value of <code>TargetString</code>, and <code>TargetString</code> must contain only alphanumeric characters or underscore (A-Z, a-z, 0-9, or _). In addition, <code>TargetString</code> must be a word, which means one of the following:</p> <ul> <li> <p> <code>TargetString</code> exactly matches the value of the specified part of the web request, such as the value of a header.</p> </li> <li> <p> <code>TargetString</code> is at the beginning of the specified part of the web request and is followed by a character other than an alphanumeric character or underscore (_), for example, <code>BadBot;</code>.</p> </li> <li> <p> <code>TargetString</code> is at the end of the specified part of the web request and is preceded by a character other than an alphanumeric character or underscore (_), for example, <code>;BadBot</code>.</p> </li> <li> <p> <code>TargetString</code> is in the middle of the specified part of the web request and is preceded and followed by characters other than alphanumeric characters or underscore (_), for example, <code>-BadBot;</code>.</p> </li> </ul> <p> <b>EXACTLY</b> </p> <p>The value of the specified part of the web request must exactly match the value of <code>TargetString</code>.</p> <p> <b>STARTS_WITH</b> </p> <p>The value of <code>TargetString</code> must appear at the beginning of the specified part of the web request.</p> <p> <b>ENDS_WITH</b> </p> <p>The value of <code>TargetString</code> must appear at the end of the specified part of the web request.</p>
    #[serde(rename = "positionalConstraint")]
    pub positional_constraint: String,
    /// <p>The value that you want AWS WAF to search for. AWS WAF searches for the specified string in the part of web requests that you specified in <code>FieldToMatch</code>. The maximum length of the value is 50 bytes.</p> <p>Valid values depend on the values that you specified for <code>FieldToMatch</code>:</p> <ul> <li> <p> <code>HEADER</code>: The value that you want AWS WAF to search for in the request header that you specified in <a>FieldToMatch</a>, for example, the value of the <code>User-Agent</code> or <code>Referer</code> header.</p> </li> <li> <p> <code>METHOD</code>: The HTTP method, which indicates the type of operation specified in the request. CloudFront supports the following methods: <code>DELETE</code>, <code>GET</code>, <code>HEAD</code>, <code>OPTIONS</code>, <code>PATCH</code>, <code>POST</code>, and <code>PUT</code>.</p> </li> <li> <p> <code>QUERY_STRING</code>: The value that you want AWS WAF to search for in the query string, which is the part of a URL that appears after a <code>?</code> character.</p> </li> <li> <p> <code>URI</code>: The value that you want AWS WAF to search for in the part of a URL that identifies a resource, for example, <code>/images/daily-ad.jpg</code>.</p> </li> <li> <p> <code>BODY</code>: The part of a request that contains any additional data that you want to send to your web server as the HTTP request body, such as data from a form. The request body immediately follows the request headers. Note that only the first <code>8192</code> bytes of the request body are forwarded to AWS WAF for inspection. To allow or block requests based on the length of the body, you can create a size constraint set. For more information, see <a>CreateSizeConstraintSet</a>. </p> </li> <li> <p> <code>SINGLE_QUERY_ARG</code>: The parameter in the query string that you will inspect, such as <i>UserName</i> or <i>SalesRegion</i>. The maximum length for <code>SINGLE_QUERY_ARG</code> is 30 characters.</p> </li> <li> <p> <code>ALL_QUERY_ARGS</code>: Similar to <code>SINGLE_QUERY_ARG</code>, but instead of inspecting a single parameter, AWS WAF inspects all parameters within the query string for the value or regex pattern that you specify in <code>TargetString</code>.</p> </li> </ul> <p>If <code>TargetString</code> includes alphabetic characters A-Z and a-z, note that the value is case sensitive.</p> <p> <b>If you're using the AWS WAF API</b> </p> <p>Specify a base64-encoded version of the value. The maximum length of the value before you base64-encode it is 50 bytes.</p> <p>For example, suppose the value of <code>Type</code> is <code>HEADER</code> and the value of <code>Data</code> is <code>User-Agent</code>. If you want to search the <code>User-Agent</code> header for the value <code>BadBot</code>, you base64-encode <code>BadBot</code> using MIME base64-encoding and include the resulting value, <code>QmFkQm90</code>, in the value of <code>TargetString</code>.</p> <p> <b>If you're using the AWS CLI or one of the AWS SDKs</b> </p> <p>The value that you want AWS WAF to search for. The SDK automatically base64 encodes the value.</p>
    #[serde(rename = "targetString")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub target_string: bytes::Bytes,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF. If you specify a transformation, AWS WAF performs the transformation on <code>FieldToMatch</code> before inspecting it for a match.</p> <p>You can only specify a single type of TextTransformation.</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system command line command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \ " ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\f, formfeed, decimal 12</p> </li> <li> <p>\t, tab, decimal 9</p> </li> <li> <p>\n, newline, decimal 10</p> </li> <li> <p>\r, carriage return, decimal 13</p> </li> <li> <p>\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a "less than" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want to perform any text transformations.</p>
    #[serde(rename = "textTransformation")]
    pub text_transformation: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateByteMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>A friendly name or description of the <a>ByteMatchSet</a>. You can't change <code>Name</code> after you create a <code>ByteMatchSet</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateByteMatchSetResponse {
    /// <p>A <a>ByteMatchSet</a> that contains no <code>ByteMatchTuple</code> objects.</p>
    #[serde(rename = "byteMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_match_set: Option<ByteMatchSet>,
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateByteMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGeoMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>A friendly name or description of the <a>GeoMatchSet</a>. You can't change <code>Name</code> after you create the <code>GeoMatchSet</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGeoMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateGeoMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>The <a>GeoMatchSet</a> returned in the <code>CreateGeoMatchSet</code> response. The <code>GeoMatchSet</code> contains no <code>GeoMatchConstraints</code>.</p>
    #[serde(rename = "geoMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_match_set: Option<GeoMatchSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateIPSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>A friendly name or description of the <a>IPSet</a>. You can't change <code>Name</code> after you create the <code>IPSet</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateIPSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateIPSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>The <a>IPSet</a> returned in the <code>CreateIPSet</code> response.</p>
    #[serde(rename = "iPSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_set: Option<IPSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRateBasedRuleRequest {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>A friendly name or description for the metrics for this <code>RateBasedRule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the <code>RateBasedRule</code>.</p>
    #[serde(rename = "metricName")]
    pub metric_name: String,
    /// <p>A friendly name or description of the <a>RateBasedRule</a>. You can't change the name of a <code>RateBasedRule</code> after you create it.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The field that AWS WAF uses to determine if requests are likely arriving from a single source and thus subject to rate monitoring. The only valid value for <code>RateKey</code> is <code>IP</code>. <code>IP</code> indicates that requests that arrive from the same IP address are subject to the <code>RateLimit</code> that is specified in the <code>RateBasedRule</code>.</p>
    #[serde(rename = "rateKey")]
    pub rate_key: String,
    /// <p>The maximum number of requests, which have an identical value in the field that is specified by <code>RateKey</code>, allowed in a five-minute period. If the number of requests exceeds the <code>RateLimit</code> and the other predicates specified in the rule are also met, AWS WAF triggers the action that is specified for this rule.</p>
    #[serde(rename = "rateLimit")]
    pub rate_limit: i64,
    /// <p><p/></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRateBasedRuleResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>The <a>RateBasedRule</a> that is returned in the <code>CreateRateBasedRule</code> response.</p>
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<RateBasedRule>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRegexMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>A friendly name or description of the <a>RegexMatchSet</a>. You can't change <code>Name</code> after you create a <code>RegexMatchSet</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRegexMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRegexMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>A <a>RegexMatchSet</a> that contains no <code>RegexMatchTuple</code> objects.</p>
    #[serde(rename = "regexMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_match_set: Option<RegexMatchSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRegexPatternSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>A friendly name or description of the <a>RegexPatternSet</a>. You can't change <code>Name</code> after you create a <code>RegexPatternSet</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRegexPatternSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRegexPatternSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>A <a>RegexPatternSet</a> that contains no objects.</p>
    #[serde(rename = "regexPatternSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_pattern_set: Option<RegexPatternSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRuleGroupRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>A friendly name or description for the metrics for this <code>RuleGroup</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the <code>RuleGroup</code>.</p>
    #[serde(rename = "metricName")]
    pub metric_name: String,
    /// <p>A friendly name or description of the <a>RuleGroup</a>. You can't change <code>Name</code> after you create a <code>RuleGroup</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p/></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRuleGroupResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRuleGroup</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>An empty <a>RuleGroup</a>.</p>
    #[serde(rename = "ruleGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group: Option<RuleGroup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateRuleRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>A friendly name or description for the metrics for this <code>Rule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the <code>Rule</code>.</p>
    #[serde(rename = "metricName")]
    pub metric_name: String,
    /// <p>A friendly name or description of the <a>Rule</a>. You can't change the name of a <code>Rule</code> after you create it.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p/></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateRuleResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>The <a>Rule</a> returned in the <code>CreateRule</code> response.</p>
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Rule>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSizeConstraintSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>A friendly name or description of the <a>SizeConstraintSet</a>. You can't change <code>Name</code> after you create a <code>SizeConstraintSet</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSizeConstraintSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateSizeConstraintSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>A <a>SizeConstraintSet</a> that contains no <code>SizeConstraint</code> objects.</p>
    #[serde(rename = "sizeConstraintSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_constraint_set: Option<SizeConstraintSet>,
}

/// <p>A request to create a <a>SqlInjectionMatchSet</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSqlInjectionMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>A friendly name or description for the <a>SqlInjectionMatchSet</a> that you're creating. You can't change <code>Name</code> after you create the <code>SqlInjectionMatchSet</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>The response to a <code>CreateSqlInjectionMatchSet</code> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSqlInjectionMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateSqlInjectionMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>A <a>SqlInjectionMatchSet</a>.</p>
    #[serde(rename = "sqlInjectionMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_injection_match_set: Option<SqlInjectionMatchSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateWebACLMigrationStackRequest {
    /// <p>Indicates whether to exclude entities that can't be migrated or to stop the migration. Set this to true to ignore unsupported entities in the web ACL during the migration. Otherwise, if AWS WAF encounters unsupported entities, it stops the process and throws an exception. </p>
    #[serde(rename = "ignoreUnsupportedType")]
    pub ignore_unsupported_type: bool,
    /// <p><p>The name of the Amazon S3 bucket to store the CloudFormation template in. The S3 bucket must be configured as follows for the migration: </p> <ul> <li> <p>The bucket name must start with <code>aws-waf-migration-</code>. For example, <code>aws-waf-migration-my-web-acl</code>.</p> </li> <li> <p>The bucket must be in the Region where you are deploying the template. For example, for a web ACL in us-west-2, you must use an Amazon S3 bucket in us-west-2 and you must deploy the template stack to us-west-2. </p> </li> <li> <p>The bucket policies must permit the migration process to write data. For listings of the bucket policies, see the Examples section. </p> </li> </ul></p>
    #[serde(rename = "s3BucketName")]
    pub s3_bucket_name: String,
    /// <p>The UUID of the WAF Classic web ACL that you want to migrate to WAF v2.</p>
    #[serde(rename = "webACLId")]
    pub web_acl_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateWebACLMigrationStackResponse {
    /// <p>The URL of the template created in Amazon S3. </p>
    #[serde(rename = "s3ObjectUrl")]
    pub s3_object_url: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateWebACLRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The action that you want AWS WAF to take when a request doesn't match the criteria specified in any of the <code>Rule</code> objects that are associated with the <code>WebACL</code>.</p>
    #[serde(rename = "defaultAction")]
    pub default_action: WafAction,
    /// <p>A friendly name or description for the metrics for this <code>WebACL</code>.The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change <code>MetricName</code> after you create the <code>WebACL</code>.</p>
    #[serde(rename = "metricName")]
    pub metric_name: String,
    /// <p>A friendly name or description of the <a>WebACL</a>. You can't change <code>Name</code> after you create the <code>WebACL</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p/></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateWebACLResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateWebACL</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>The <a>WebACL</a> returned in the <code>CreateWebACL</code> response.</p>
    #[serde(rename = "webACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl: Option<WebACL>,
}

/// <p>A request to create an <a>XssMatchSet</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateXssMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>A friendly name or description for the <a>XssMatchSet</a> that you're creating. You can't change <code>Name</code> after you create the <code>XssMatchSet</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>The response to a <code>CreateXssMatchSet</code> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateXssMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>CreateXssMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
    /// <p>An <a>XssMatchSet</a>.</p>
    #[serde(rename = "xssMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xss_match_set: Option<XssMatchSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteByteMatchSetRequest {
    /// <p>The <code>ByteMatchSetId</code> of the <a>ByteMatchSet</a> that you want to delete. <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>
    #[serde(rename = "byteMatchSetId")]
    pub byte_match_set_id: String,
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteByteMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteByteMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGeoMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>GeoMatchSetID</code> of the <a>GeoMatchSet</a> that you want to delete. <code>GeoMatchSetId</code> is returned by <a>CreateGeoMatchSet</a> and by <a>ListGeoMatchSets</a>.</p>
    #[serde(rename = "geoMatchSetId")]
    pub geo_match_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGeoMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteGeoMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteIPSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>IPSetId</code> of the <a>IPSet</a> that you want to delete. <code>IPSetId</code> is returned by <a>CreateIPSet</a> and by <a>ListIPSets</a>.</p>
    #[serde(rename = "iPSetId")]
    pub ip_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteIPSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteIPSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLoggingConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the web ACL from which you want to delete the <a>LoggingConfiguration</a>.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteLoggingConfigurationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePermissionPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the RuleGroup from which you want to delete the policy.</p> <p>The user making the request must be the owner of the RuleGroup.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePermissionPolicyResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRateBasedRuleRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>RuleId</code> of the <a>RateBasedRule</a> that you want to delete. <code>RuleId</code> is returned by <a>CreateRateBasedRule</a> and by <a>ListRateBasedRules</a>.</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRateBasedRuleResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRegexMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>RegexMatchSetId</code> of the <a>RegexMatchSet</a> that you want to delete. <code>RegexMatchSetId</code> is returned by <a>CreateRegexMatchSet</a> and by <a>ListRegexMatchSets</a>.</p>
    #[serde(rename = "regexMatchSetId")]
    pub regex_match_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRegexMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteRegexMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRegexPatternSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>RegexPatternSetId</code> of the <a>RegexPatternSet</a> that you want to delete. <code>RegexPatternSetId</code> is returned by <a>CreateRegexPatternSet</a> and by <a>ListRegexPatternSets</a>.</p>
    #[serde(rename = "regexPatternSetId")]
    pub regex_pattern_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRegexPatternSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteRegexPatternSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRuleGroupRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>RuleGroupId</code> of the <a>RuleGroup</a> that you want to delete. <code>RuleGroupId</code> is returned by <a>CreateRuleGroup</a> and by <a>ListRuleGroups</a>.</p>
    #[serde(rename = "ruleGroupId")]
    pub rule_group_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRuleGroupResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteRuleGroup</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteRuleRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>RuleId</code> of the <a>Rule</a> that you want to delete. <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteRuleResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSizeConstraintSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>SizeConstraintSetId</code> of the <a>SizeConstraintSet</a> that you want to delete. <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>
    #[serde(rename = "sizeConstraintSetId")]
    pub size_constraint_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSizeConstraintSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteSizeConstraintSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

/// <p>A request to delete a <a>SqlInjectionMatchSet</a> from AWS WAF.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSqlInjectionMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>SqlInjectionMatchSetId</code> of the <a>SqlInjectionMatchSet</a> that you want to delete. <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>
    #[serde(rename = "sqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: String,
}

/// <p>The response to a request to delete a <a>SqlInjectionMatchSet</a> from AWS WAF.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSqlInjectionMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteSqlInjectionMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteWebACLRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>WebACLId</code> of the <a>WebACL</a> that you want to delete. <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>
    #[serde(rename = "webACLId")]
    pub web_acl_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteWebACLResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteWebACL</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

/// <p>A request to delete an <a>XssMatchSet</a> from AWS WAF.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteXssMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>XssMatchSetId</code> of the <a>XssMatchSet</a> that you want to delete. <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>
    #[serde(rename = "xssMatchSetId")]
    pub xss_match_set_id: String,
}

/// <p>The response to a request to delete an <a>XssMatchSet</a> from AWS WAF.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteXssMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>DeleteXssMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>The rule to exclude from a rule group. This is applicable only when the <code>ActivatedRule</code> refers to a <code>RuleGroup</code>. The rule must belong to the <code>RuleGroup</code> that is specified by the <code>ActivatedRule</code>. </p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ExcludedRule {
    /// <p>The unique identifier for the rule to exclude from the rule group.</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Specifies where in a web request to look for <code>TargetString</code>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FieldToMatch {
    /// <p>When the value of <code>Type</code> is <code>HEADER</code>, enter the name of the header that you want AWS WAF to search, for example, <code>User-Agent</code> or <code>Referer</code>. The name of the header is not case sensitive.</p> <p>When the value of <code>Type</code> is <code>SINGLE_QUERY_ARG</code>, enter the name of the parameter that you want AWS WAF to search, for example, <code>UserName</code> or <code>SalesRegion</code>. The parameter name is not case sensitive.</p> <p>If the value of <code>Type</code> is any other value, omit <code>Data</code>.</p>
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// <p><p>The part of the web request that you want AWS WAF to search for a specified string. Parts of a request that you can search include the following:</p> <ul> <li> <p> <code>HEADER</code>: A specified request header, for example, the value of the <code>User-Agent</code> or <code>Referer</code> header. If you choose <code>HEADER</code> for the type, specify the name of the header in <code>Data</code>.</p> </li> <li> <p> <code>METHOD</code>: The HTTP method, which indicated the type of operation that the request is asking the origin to perform. Amazon CloudFront supports the following methods: <code>DELETE</code>, <code>GET</code>, <code>HEAD</code>, <code>OPTIONS</code>, <code>PATCH</code>, <code>POST</code>, and <code>PUT</code>.</p> </li> <li> <p> <code>QUERY<em>STRING</code>: A query string, which is the part of a URL that appears after a <code>?</code> character, if any.</p> </li> <li> <p> <code>URI</code>: The part of a web request that identifies a resource, for example, <code>/images/daily-ad.jpg</code>.</p> </li> <li> <p> <code>BODY</code>: The part of a request that contains any additional data that you want to send to your web server as the HTTP request body, such as data from a form. The request body immediately follows the request headers. Note that only the first <code>8192</code> bytes of the request body are forwarded to AWS WAF for inspection. To allow or block requests based on the length of the body, you can create a size constraint set. For more information, see <a>CreateSizeConstraintSet</a>. </p> </li> <li> <p> <code>SINGLE</em>QUERY<em>ARG</code>: The parameter in the query string that you will inspect, such as <i>UserName</i> or <i>SalesRegion</i>. The maximum length for <code>SINGLE</em>QUERY<em>ARG</code> is 30 characters.</p> </li> <li> <p> <code>ALL</em>QUERY<em>ARGS</code>: Similar to <code>SINGLE</em>QUERY_ARG</code>, but rather than inspecting a single parameter, AWS WAF will inspect all parameters within the query for the value or regex pattern that you specify in <code>TargetString</code>.</p> </li> </ul></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>The country from which web requests originate that you want AWS WAF to search for.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GeoMatchConstraint {
    /// <p>The type of geographical area you want AWS WAF to search for. Currently <code>Country</code> is the only valid value.</p>
    #[serde(rename = "type")]
    pub type_: String,
    /// <p>The country that you want AWS WAF to search for.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Contains one or more countries that AWS WAF will search for.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GeoMatchSet {
    /// <p>An array of <a>GeoMatchConstraint</a> objects, which contain the country that you want AWS WAF to search for.</p>
    #[serde(rename = "geoMatchConstraints")]
    pub geo_match_constraints: Vec<GeoMatchConstraint>,
    /// <p>The <code>GeoMatchSetId</code> for an <code>GeoMatchSet</code>. You use <code>GeoMatchSetId</code> to get information about a <code>GeoMatchSet</code> (see <a>GeoMatchSet</a>), update a <code>GeoMatchSet</code> (see <a>UpdateGeoMatchSet</a>), insert a <code>GeoMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>GeoMatchSet</code> from AWS WAF (see <a>DeleteGeoMatchSet</a>).</p> <p> <code>GeoMatchSetId</code> is returned by <a>CreateGeoMatchSet</a> and by <a>ListGeoMatchSets</a>.</p>
    #[serde(rename = "geoMatchSetId")]
    pub geo_match_set_id: String,
    /// <p>A friendly name or description of the <a>GeoMatchSet</a>. You can't change the name of an <code>GeoMatchSet</code> after you create it.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Contains the identifier and the name of the <code>GeoMatchSet</code>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GeoMatchSetSummary {
    /// <p>The <code>GeoMatchSetId</code> for an <a>GeoMatchSet</a>. You can use <code>GeoMatchSetId</code> in a <a>GetGeoMatchSet</a> request to get detailed information about an <a>GeoMatchSet</a>.</p>
    #[serde(rename = "geoMatchSetId")]
    pub geo_match_set_id: String,
    /// <p>A friendly name or description of the <a>GeoMatchSet</a>. You can't change the name of an <code>GeoMatchSet</code> after you create it.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Specifies the type of update to perform to an <a>GeoMatchSet</a> with <a>UpdateGeoMatchSet</a>.</p></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GeoMatchSetUpdate {
    /// <p>Specifies whether to insert or delete a country with <a>UpdateGeoMatchSet</a>.</p>
    #[serde(rename = "action")]
    pub action: String,
    /// <p>The country from which web requests originate that you want AWS WAF to search for.</p>
    #[serde(rename = "geoMatchConstraint")]
    pub geo_match_constraint: GeoMatchConstraint,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetByteMatchSetRequest {
    /// <p>The <code>ByteMatchSetId</code> of the <a>ByteMatchSet</a> that you want to get. <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>
    #[serde(rename = "byteMatchSetId")]
    pub byte_match_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetByteMatchSetResponse {
    /// <p><p>Information about the <a>ByteMatchSet</a> that you specified in the <code>GetByteMatchSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>ByteMatchSet</a>: Contains <code>ByteMatchSetId</code>, <code>ByteMatchTuples</code>, and <code>Name</code> </p> </li> <li> <p> <code>ByteMatchTuples</code>: Contains an array of <a>ByteMatchTuple</a> objects. Each <code>ByteMatchTuple</code> object contains <a>FieldToMatch</a>, <code>PositionalConstraint</code>, <code>TargetString</code>, and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "byteMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_match_set: Option<ByteMatchSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetChangeTokenRequest {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetChangeTokenResponse {
    /// <p>The <code>ChangeToken</code> that you used in the request. Use this value in a <code>GetChangeTokenStatus</code> request to get the current status of the request. </p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetChangeTokenStatusRequest {
    /// <p>The change token for which you want to get the status. This change token was previously returned in the <code>GetChangeToken</code> response.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetChangeTokenStatusResponse {
    /// <p>The status of the change token.</p>
    #[serde(rename = "changeTokenStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token_status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetGeoMatchSetRequest {
    /// <p>The <code>GeoMatchSetId</code> of the <a>GeoMatchSet</a> that you want to get. <code>GeoMatchSetId</code> is returned by <a>CreateGeoMatchSet</a> and by <a>ListGeoMatchSets</a>.</p>
    #[serde(rename = "geoMatchSetId")]
    pub geo_match_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetGeoMatchSetResponse {
    /// <p>Information about the <a>GeoMatchSet</a> that you specified in the <code>GetGeoMatchSet</code> request. This includes the <code>Type</code>, which for a <code>GeoMatchContraint</code> is always <code>Country</code>, as well as the <code>Value</code>, which is the identifier for a specific country.</p>
    #[serde(rename = "geoMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_match_set: Option<GeoMatchSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetIPSetRequest {
    /// <p>The <code>IPSetId</code> of the <a>IPSet</a> that you want to get. <code>IPSetId</code> is returned by <a>CreateIPSet</a> and by <a>ListIPSets</a>.</p>
    #[serde(rename = "iPSetId")]
    pub ip_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetIPSetResponse {
    /// <p><p>Information about the <a>IPSet</a> that you specified in the <code>GetIPSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>IPSet</a>: Contains <code>IPSetDescriptors</code>, <code>IPSetId</code>, and <code>Name</code> </p> </li> <li> <p> <code>IPSetDescriptors</code>: Contains an array of <a>IPSetDescriptor</a> objects. Each <code>IPSetDescriptor</code> object contains <code>Type</code> and <code>Value</code> </p> </li> </ul></p>
    #[serde(rename = "iPSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_set: Option<IPSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLoggingConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the web ACL for which you want to get the <a>LoggingConfiguration</a>.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetLoggingConfigurationResponse {
    /// <p>The <a>LoggingConfiguration</a> for the specified web ACL.</p>
    #[serde(rename = "loggingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPermissionPolicyRequest {
    /// <p>The Amazon Resource Name (ARN) of the RuleGroup for which you want to get the policy.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPermissionPolicyResponse {
    /// <p>The IAM policy attached to the specified RuleGroup.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRateBasedRuleManagedKeysRequest {
    /// <p>A null value and not currently used. Do not include this in your request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>The <code>RuleId</code> of the <a>RateBasedRule</a> for which you want to get a list of <code>ManagedKeys</code>. <code>RuleId</code> is returned by <a>CreateRateBasedRule</a> and by <a>ListRateBasedRules</a>.</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRateBasedRuleManagedKeysResponse {
    /// <p>An array of IP addresses that currently are blocked by the specified <a>RateBasedRule</a>. </p>
    #[serde(rename = "managedKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_keys: Option<Vec<String>>,
    /// <p>A null value and not currently used.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRateBasedRuleRequest {
    /// <p>The <code>RuleId</code> of the <a>RateBasedRule</a> that you want to get. <code>RuleId</code> is returned by <a>CreateRateBasedRule</a> and by <a>ListRateBasedRules</a>.</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRateBasedRuleResponse {
    /// <p>Information about the <a>RateBasedRule</a> that you specified in the <code>GetRateBasedRule</code> request.</p>
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<RateBasedRule>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRegexMatchSetRequest {
    /// <p>The <code>RegexMatchSetId</code> of the <a>RegexMatchSet</a> that you want to get. <code>RegexMatchSetId</code> is returned by <a>CreateRegexMatchSet</a> and by <a>ListRegexMatchSets</a>.</p>
    #[serde(rename = "regexMatchSetId")]
    pub regex_match_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRegexMatchSetResponse {
    /// <p>Information about the <a>RegexMatchSet</a> that you specified in the <code>GetRegexMatchSet</code> request. For more information, see <a>RegexMatchTuple</a>.</p>
    #[serde(rename = "regexMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_match_set: Option<RegexMatchSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRegexPatternSetRequest {
    /// <p>The <code>RegexPatternSetId</code> of the <a>RegexPatternSet</a> that you want to get. <code>RegexPatternSetId</code> is returned by <a>CreateRegexPatternSet</a> and by <a>ListRegexPatternSets</a>.</p>
    #[serde(rename = "regexPatternSetId")]
    pub regex_pattern_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRegexPatternSetResponse {
    /// <p>Information about the <a>RegexPatternSet</a> that you specified in the <code>GetRegexPatternSet</code> request, including the identifier of the pattern set and the regular expression patterns you want AWS WAF to search for. </p>
    #[serde(rename = "regexPatternSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_pattern_set: Option<RegexPatternSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRuleGroupRequest {
    /// <p>The <code>RuleGroupId</code> of the <a>RuleGroup</a> that you want to get. <code>RuleGroupId</code> is returned by <a>CreateRuleGroup</a> and by <a>ListRuleGroups</a>.</p>
    #[serde(rename = "ruleGroupId")]
    pub rule_group_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRuleGroupResponse {
    /// <p>Information about the <a>RuleGroup</a> that you specified in the <code>GetRuleGroup</code> request. </p>
    #[serde(rename = "ruleGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group: Option<RuleGroup>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRuleRequest {
    /// <p>The <code>RuleId</code> of the <a>Rule</a> that you want to get. <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRuleResponse {
    /// <p><p>Information about the <a>Rule</a> that you specified in the <code>GetRule</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>Rule</a>: Contains <code>MetricName</code>, <code>Name</code>, an array of <code>Predicate</code> objects, and <code>RuleId</code> </p> </li> <li> <p> <a>Predicate</a>: Each <code>Predicate</code> object contains <code>DataId</code>, <code>Negated</code>, and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Rule>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSampledRequestsRequest {
    /// <p>The number of requests that you want AWS WAF to return from among the first 5,000 requests that your AWS resource received during the time range. If your resource received fewer requests than the value of <code>MaxItems</code>, <code>GetSampledRequests</code> returns information about all of them. </p>
    #[serde(rename = "maxItems")]
    pub max_items: i64,
    /// <p><p> <code>RuleId</code> is one of three values:</p> <ul> <li> <p>The <code>RuleId</code> of the <code>Rule</code> or the <code>RuleGroupId</code> of the <code>RuleGroup</code> for which you want <code>GetSampledRequests</code> to return a sample of requests.</p> </li> <li> <p> <code>Default_Action</code>, which causes <code>GetSampledRequests</code> to return a sample of the requests that didn&#39;t match any of the rules in the specified <code>WebACL</code>.</p> </li> </ul></p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    /// <p>The start date and time and the end date and time of the range for which you want <code>GetSampledRequests</code> to return a sample of requests. You must specify the times in Coordinated Universal Time (UTC) format. UTC format includes the special designator, <code>Z</code>. For example, <code>"2016-09-27T14:50Z"</code>. You can specify any time range in the previous three hours.</p>
    #[serde(rename = "timeWindow")]
    pub time_window: TimeWindow,
    /// <p>The <code>WebACLId</code> of the <code>WebACL</code> for which you want <code>GetSampledRequests</code> to return a sample of requests.</p>
    #[serde(rename = "webAclId")]
    pub web_acl_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSampledRequestsResponse {
    /// <p>The total number of requests from which <code>GetSampledRequests</code> got a sample of <code>MaxItems</code> requests. If <code>PopulationSize</code> is less than <code>MaxItems</code>, the sample includes every request that your AWS resource received during the specified time range.</p>
    #[serde(rename = "populationSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub population_size: Option<i64>,
    /// <p>A complex type that contains detailed information about each of the requests in the sample.</p>
    #[serde(rename = "sampledRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampled_requests: Option<Vec<SampledHTTPRequest>>,
    /// <p>Usually, <code>TimeWindow</code> is the time range that you specified in the <code>GetSampledRequests</code> request. However, if your AWS resource received more than 5,000 requests during the time range that you specified in the request, <code>GetSampledRequests</code> returns the time range for the first 5,000 requests. Times are in Coordinated Universal Time (UTC) format.</p>
    #[serde(rename = "timeWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window: Option<TimeWindow>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSizeConstraintSetRequest {
    /// <p>The <code>SizeConstraintSetId</code> of the <a>SizeConstraintSet</a> that you want to get. <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>
    #[serde(rename = "sizeConstraintSetId")]
    pub size_constraint_set_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSizeConstraintSetResponse {
    /// <p><p>Information about the <a>SizeConstraintSet</a> that you specified in the <code>GetSizeConstraintSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>SizeConstraintSet</a>: Contains <code>SizeConstraintSetId</code>, <code>SizeConstraints</code>, and <code>Name</code> </p> </li> <li> <p> <code>SizeConstraints</code>: Contains an array of <a>SizeConstraint</a> objects. Each <code>SizeConstraint</code> object contains <a>FieldToMatch</a>, <code>TextTransformation</code>, <code>ComparisonOperator</code>, and <code>Size</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "sizeConstraintSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_constraint_set: Option<SizeConstraintSet>,
}

/// <p>A request to get a <a>SqlInjectionMatchSet</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSqlInjectionMatchSetRequest {
    /// <p>The <code>SqlInjectionMatchSetId</code> of the <a>SqlInjectionMatchSet</a> that you want to get. <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>
    #[serde(rename = "sqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: String,
}

/// <p>The response to a <a>GetSqlInjectionMatchSet</a> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetSqlInjectionMatchSetResponse {
    /// <p><p>Information about the <a>SqlInjectionMatchSet</a> that you specified in the <code>GetSqlInjectionMatchSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>SqlInjectionMatchSet</a>: Contains <code>Name</code>, <code>SqlInjectionMatchSetId</code>, and an array of <code>SqlInjectionMatchTuple</code> objects</p> </li> <li> <p> <a>SqlInjectionMatchTuple</a>: Each <code>SqlInjectionMatchTuple</code> object contains <code>FieldToMatch</code> and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "sqlInjectionMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_injection_match_set: Option<SqlInjectionMatchSet>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetWebACLRequest {
    /// <p>The <code>WebACLId</code> of the <a>WebACL</a> that you want to get. <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>
    #[serde(rename = "webACLId")]
    pub web_acl_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetWebACLResponse {
    /// <p><p>Information about the <a>WebACL</a> that you specified in the <code>GetWebACL</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>WebACL</a>: Contains <code>DefaultAction</code>, <code>MetricName</code>, <code>Name</code>, an array of <code>Rule</code> objects, and <code>WebACLId</code> </p> </li> <li> <p> <code>DefaultAction</code> (Data type is <a>WafAction</a>): Contains <code>Type</code> </p> </li> <li> <p> <code>Rules</code>: Contains an array of <code>ActivatedRule</code> objects, which contain <code>Action</code>, <code>Priority</code>, and <code>RuleId</code> </p> </li> <li> <p> <code>Action</code>: Contains <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "webACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl: Option<WebACL>,
}

/// <p>A request to get an <a>XssMatchSet</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetXssMatchSetRequest {
    /// <p>The <code>XssMatchSetId</code> of the <a>XssMatchSet</a> that you want to get. <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>
    #[serde(rename = "xssMatchSetId")]
    pub xss_match_set_id: String,
}

/// <p>The response to a <a>GetXssMatchSet</a> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetXssMatchSetResponse {
    /// <p><p>Information about the <a>XssMatchSet</a> that you specified in the <code>GetXssMatchSet</code> request. For more information, see the following topics:</p> <ul> <li> <p> <a>XssMatchSet</a>: Contains <code>Name</code>, <code>XssMatchSetId</code>, and an array of <code>XssMatchTuple</code> objects</p> </li> <li> <p> <a>XssMatchTuple</a>: Each <code>XssMatchTuple</code> object contains <code>FieldToMatch</code> and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "xssMatchSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xss_match_set: Option<XssMatchSet>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>The response from a <a>GetSampledRequests</a> request includes an <code>HTTPHeader</code> complex type that appears as <code>Headers</code> in the response syntax. <code>HTTPHeader</code> contains the names and values of all of the headers that appear in one of the web requests that were returned by <code>GetSampledRequests</code>. </p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HTTPHeader {
    /// <p>The name of one of the headers in the sampled web request.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of one of the headers in the sampled web request.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>The response from a <a>GetSampledRequests</a> request includes an <code>HTTPRequest</code> complex type that appears as <code>Request</code> in the response syntax. <code>HTTPRequest</code> contains information about one of the web requests that were returned by <code>GetSampledRequests</code>. </p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HTTPRequest {
    /// <p><p>The IP address that the request originated from. If the <code>WebACL</code> is associated with a CloudFront distribution, this is the value of one of the following fields in CloudFront access logs:</p> <ul> <li> <p> <code>c-ip</code>, if the viewer did not use an HTTP proxy or a load balancer to send the request</p> </li> <li> <p> <code>x-forwarded-for</code>, if the viewer did use an HTTP proxy or a load balancer to send the request</p> </li> </ul></p>
    #[serde(rename = "clientIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    /// <p>The two-letter country code for the country that the request originated from. For a current list of country codes, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2">ISO 3166-1 alpha-2</a>.</p>
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// <p>The HTTP version specified in the sampled web request, for example, <code>HTTP/1.1</code>.</p>
    #[serde(rename = "hTTPVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_version: Option<String>,
    /// <p>A complex type that contains two values for each header in the sampled web request: the name of the header and the value of the header.</p>
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HTTPHeader>>,
    /// <p>The HTTP method specified in the sampled web request. CloudFront supports the following methods: <code>DELETE</code>, <code>GET</code>, <code>HEAD</code>, <code>OPTIONS</code>, <code>PATCH</code>, <code>POST</code>, and <code>PUT</code>. </p>
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// <p>The part of a web request that identifies the resource, for example, <code>/images/daily-ad.jpg</code>.</p>
    #[serde(rename = "uRI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Contains one or more IP addresses or blocks of IP addresses specified in Classless Inter-Domain Routing (CIDR) notation. AWS WAF supports IPv4 address ranges: /8 and any range between /16 through /32. AWS WAF supports IPv6 address ranges: /24, /32, /48, /56, /64, and /128.</p> <p>To specify an individual IP address, you specify the four-part IP address followed by a <code>/32</code>, for example, 192.0.2.0/32. To block a range of IP addresses, you can specify /8 or any range between /16 through /32 (for IPv4) or /24, /32, /48, /56, /64, or /128 (for IPv6). For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>. </p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IPSet {
    /// <p>The IP address type (<code>IPV4</code> or <code>IPV6</code>) and the IP address range (in CIDR notation) that web requests originate from. If the <code>WebACL</code> is associated with a CloudFront distribution and the viewer did not use an HTTP proxy or a load balancer to send the request, this is the value of the c-ip field in the CloudFront access logs.</p>
    #[serde(rename = "iPSetDescriptors")]
    pub ip_set_descriptors: Vec<IPSetDescriptor>,
    /// <p>The <code>IPSetId</code> for an <code>IPSet</code>. You use <code>IPSetId</code> to get information about an <code>IPSet</code> (see <a>GetIPSet</a>), update an <code>IPSet</code> (see <a>UpdateIPSet</a>), insert an <code>IPSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete an <code>IPSet</code> from AWS WAF (see <a>DeleteIPSet</a>).</p> <p> <code>IPSetId</code> is returned by <a>CreateIPSet</a> and by <a>ListIPSets</a>.</p>
    #[serde(rename = "iPSetId")]
    pub ip_set_id: String,
    /// <p>A friendly name or description of the <a>IPSet</a>. You can't change the name of an <code>IPSet</code> after you create it.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Specifies the IP address type (<code>IPV4</code> or <code>IPV6</code>) and the IP address range (in CIDR format) that web requests originate from.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IPSetDescriptor {
    /// <p>Specify <code>IPV4</code> or <code>IPV6</code>.</p>
    #[serde(rename = "type")]
    pub type_: String,
    /// <p><p>Specify an IPv4 address by using CIDR notation. For example:</p> <ul> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from the IP address 192.0.2.44, specify <code>192.0.2.44/32</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from IP addresses from 192.0.2.0 to 192.0.2.255, specify <code>192.0.2.0/24</code>.</p> </li> </ul> <p>For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p> <p>Specify an IPv6 address by using CIDR notation. For example:</p> <ul> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from the IP address 1111:0000:0000:0000:0000:0000:0000:0111, specify <code>1111:0000:0000:0000:0000:0000:0000:0111/128</code>.</p> </li> <li> <p>To configure AWS WAF to allow, block, or count requests that originated from IP addresses 1111:0000:0000:0000:0000:0000:0000:0000 to 1111:0000:0000:0000:ffff:ffff:ffff:ffff, specify <code>1111:0000:0000:0000:0000:0000:0000:0000/64</code>.</p> </li> </ul></p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Contains the identifier and the name of the <code>IPSet</code>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IPSetSummary {
    /// <p>The <code>IPSetId</code> for an <a>IPSet</a>. You can use <code>IPSetId</code> in a <a>GetIPSet</a> request to get detailed information about an <a>IPSet</a>.</p>
    #[serde(rename = "iPSetId")]
    pub ip_set_id: String,
    /// <p>A friendly name or description of the <a>IPSet</a>. You can't change the name of an <code>IPSet</code> after you create it.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Specifies the type of update to perform to an <a>IPSet</a> with <a>UpdateIPSet</a>.</p></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IPSetUpdate {
    /// <p>Specifies whether to insert or delete an IP address with <a>UpdateIPSet</a>.</p>
    #[serde(rename = "action")]
    pub action: String,
    /// <p>The IP address type (<code>IPV4</code> or <code>IPV6</code>) and the IP address range (in CIDR notation) that web requests originate from.</p>
    #[serde(rename = "iPSetDescriptor")]
    pub ip_set_descriptor: IPSetDescriptor,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListActivatedRulesInRuleGroupRequest {
    /// <p>Specifies the number of <code>ActivatedRules</code> that you want AWS WAF to return for this request. If you have more <code>ActivatedRules</code> than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>ActivatedRules</code>.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>ActivatedRules</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>ActivatedRules</code>. For the second and subsequent <code>ListActivatedRulesInRuleGroup</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>ActivatedRules</code>.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>The <code>RuleGroupId</code> of the <a>RuleGroup</a> for which you want to get a list of <a>ActivatedRule</a> objects.</p>
    #[serde(rename = "ruleGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListActivatedRulesInRuleGroupResponse {
    /// <p>An array of <code>ActivatedRules</code> objects.</p>
    #[serde(rename = "activatedRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_rules: Option<Vec<ActivatedRule>>,
    /// <p>If you have more <code>ActivatedRules</code> than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>ActivatedRules</code>, submit another <code>ListActivatedRulesInRuleGroup</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListByteMatchSetsRequest {
    /// <p>Specifies the number of <code>ByteMatchSet</code> objects that you want AWS WAF to return for this request. If you have more <code>ByteMatchSets</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>ByteMatchSet</code> objects.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>ByteMatchSets</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>ByteMatchSets</code>. For the second and subsequent <code>ListByteMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>ByteMatchSets</code>.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListByteMatchSetsResponse {
    /// <p>An array of <a>ByteMatchSetSummary</a> objects.</p>
    #[serde(rename = "byteMatchSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_match_sets: Option<Vec<ByteMatchSetSummary>>,
    /// <p>If you have more <code>ByteMatchSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>ByteMatchSet</code> objects, submit another <code>ListByteMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGeoMatchSetsRequest {
    /// <p>Specifies the number of <code>GeoMatchSet</code> objects that you want AWS WAF to return for this request. If you have more <code>GeoMatchSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>GeoMatchSet</code> objects.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>GeoMatchSet</code>s than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>GeoMatchSet</code> objects. For the second and subsequent <code>ListGeoMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>GeoMatchSet</code> objects.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGeoMatchSetsResponse {
    /// <p>An array of <a>GeoMatchSetSummary</a> objects.</p>
    #[serde(rename = "geoMatchSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_match_sets: Option<Vec<GeoMatchSetSummary>>,
    /// <p>If you have more <code>GeoMatchSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>GeoMatchSet</code> objects, submit another <code>ListGeoMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListIPSetsRequest {
    /// <p>Specifies the number of <code>IPSet</code> objects that you want AWS WAF to return for this request. If you have more <code>IPSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>IPSet</code> objects.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>IPSets</code>. For the second and subsequent <code>ListIPSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>IPSets</code>.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListIPSetsResponse {
    /// <p>An array of <a>IPSetSummary</a> objects.</p>
    #[serde(rename = "iPSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_sets: Option<Vec<IPSetSummary>>,
    /// <p>To list more <code>IPSet</code> objects, submit another <code>ListIPSets</code> request, and in the next request use the <code>NextMarker</code> response value as the <code>NextMarker</code> value.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLoggingConfigurationsRequest {
    /// <p>Specifies the number of <code>LoggingConfigurations</code> that you want AWS WAF to return for this request. If you have more <code>LoggingConfigurations</code> than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>LoggingConfigurations</code>.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>LoggingConfigurations</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>LoggingConfigurations</code>. For the second and subsequent <code>ListLoggingConfigurations</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>ListLoggingConfigurations</code>.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLoggingConfigurationsResponse {
    /// <p>An array of <a>LoggingConfiguration</a> objects.</p>
    #[serde(rename = "loggingConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configurations: Option<Vec<LoggingConfiguration>>,
    /// <p>If you have more <code>LoggingConfigurations</code> than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>LoggingConfigurations</code>, submit another <code>ListLoggingConfigurations</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRateBasedRulesRequest {
    /// <p>Specifies the number of <code>Rules</code> that you want AWS WAF to return for this request. If you have more <code>Rules</code> than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>Rules</code>.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>Rules</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>Rules</code>. For the second and subsequent <code>ListRateBasedRules</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>Rules</code>.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRateBasedRulesResponse {
    /// <p>If you have more <code>Rules</code> than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>Rules</code>, submit another <code>ListRateBasedRules</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>RuleSummary</a> objects.</p>
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RuleSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRegexMatchSetsRequest {
    /// <p>Specifies the number of <code>RegexMatchSet</code> objects that you want AWS WAF to return for this request. If you have more <code>RegexMatchSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>RegexMatchSet</code> objects.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>RegexMatchSet</code> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>ByteMatchSets</code>. For the second and subsequent <code>ListRegexMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>RegexMatchSet</code> objects.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRegexMatchSetsResponse {
    /// <p>If you have more <code>RegexMatchSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>RegexMatchSet</code> objects, submit another <code>ListRegexMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>RegexMatchSetSummary</a> objects.</p>
    #[serde(rename = "regexMatchSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_match_sets: Option<Vec<RegexMatchSetSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRegexPatternSetsRequest {
    /// <p>Specifies the number of <code>RegexPatternSet</code> objects that you want AWS WAF to return for this request. If you have more <code>RegexPatternSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>RegexPatternSet</code> objects.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>RegexPatternSet</code> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>RegexPatternSet</code> objects. For the second and subsequent <code>ListRegexPatternSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>RegexPatternSet</code> objects.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRegexPatternSetsResponse {
    /// <p>If you have more <code>RegexPatternSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>RegexPatternSet</code> objects, submit another <code>ListRegexPatternSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>RegexPatternSetSummary</a> objects.</p>
    #[serde(rename = "regexPatternSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_pattern_sets: Option<Vec<RegexPatternSetSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRuleGroupsRequest {
    /// <p>Specifies the number of <code>RuleGroups</code> that you want AWS WAF to return for this request. If you have more <code>RuleGroups</code> than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>RuleGroups</code>.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>RuleGroups</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>RuleGroups</code>. For the second and subsequent <code>ListRuleGroups</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>RuleGroups</code>.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRuleGroupsResponse {
    /// <p>If you have more <code>RuleGroups</code> than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>RuleGroups</code>, submit another <code>ListRuleGroups</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>RuleGroup</a> objects.</p>
    #[serde(rename = "ruleGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_groups: Option<Vec<RuleGroupSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListRulesRequest {
    /// <p>Specifies the number of <code>Rules</code> that you want AWS WAF to return for this request. If you have more <code>Rules</code> than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>Rules</code>.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>Rules</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>Rules</code>. For the second and subsequent <code>ListRules</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>Rules</code>.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListRulesResponse {
    /// <p>If you have more <code>Rules</code> than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>Rules</code>, submit another <code>ListRules</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>RuleSummary</a> objects.</p>
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RuleSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSizeConstraintSetsRequest {
    /// <p>Specifies the number of <code>SizeConstraintSet</code> objects that you want AWS WAF to return for this request. If you have more <code>SizeConstraintSets</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>SizeConstraintSet</code> objects.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>SizeConstraintSets</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>SizeConstraintSets</code>. For the second and subsequent <code>ListSizeConstraintSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>SizeConstraintSets</code>.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSizeConstraintSetsResponse {
    /// <p>If you have more <code>SizeConstraintSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>SizeConstraintSet</code> objects, submit another <code>ListSizeConstraintSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>SizeConstraintSetSummary</a> objects.</p>
    #[serde(rename = "sizeConstraintSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_constraint_sets: Option<Vec<SizeConstraintSetSummary>>,
}

/// <p>A request to list the <a>SqlInjectionMatchSet</a> objects created by the current AWS account.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSqlInjectionMatchSetsRequest {
    /// <p>Specifies the number of <a>SqlInjectionMatchSet</a> objects that you want AWS WAF to return for this request. If you have more <code>SqlInjectionMatchSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>Rules</code>.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <a>SqlInjectionMatchSet</a> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>SqlInjectionMatchSets</code>. For the second and subsequent <code>ListSqlInjectionMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>SqlInjectionMatchSets</code>.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

/// <p>The response to a <a>ListSqlInjectionMatchSets</a> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSqlInjectionMatchSetsResponse {
    /// <p>If you have more <a>SqlInjectionMatchSet</a> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>SqlInjectionMatchSet</code> objects, submit another <code>ListSqlInjectionMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>SqlInjectionMatchSetSummary</a> objects.</p>
    #[serde(rename = "sqlInjectionMatchSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_injection_match_sets: Option<Vec<SqlInjectionMatchSetSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSubscribedRuleGroupsRequest {
    /// <p>Specifies the number of subscribed rule groups that you want AWS WAF to return for this request. If you have more objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of objects.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>ByteMatchSets</code>subscribed rule groups than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of subscribed rule groups. For the second and subsequent <code>ListSubscribedRuleGroupsRequest</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of subscribed rule groups.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSubscribedRuleGroupsResponse {
    /// <p>If you have more objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more objects, submit another <code>ListSubscribedRuleGroups</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>RuleGroup</a> objects.</p>
    #[serde(rename = "ruleGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_groups: Option<Vec<SubscribedRuleGroupSummary>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p><p/></p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p><p/></p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "tagInfoForResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_info_for_resource: Option<TagInfoForResource>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListWebACLsRequest {
    /// <p>Specifies the number of <code>WebACL</code> objects that you want AWS WAF to return for this request. If you have more <code>WebACL</code> objects than the number that you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>WebACL</code> objects.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <code>WebACL</code> objects than the number that you specify for <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>WebACL</code> objects. For the second and subsequent <code>ListWebACLs</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>WebACL</code> objects.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListWebACLsResponse {
    /// <p>If you have more <code>WebACL</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>WebACL</code> objects, submit another <code>ListWebACLs</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>WebACLSummary</a> objects.</p>
    #[serde(rename = "webACLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_ac_ls: Option<Vec<WebACLSummary>>,
}

/// <p>A request to list the <a>XssMatchSet</a> objects created by the current AWS account.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListXssMatchSetsRequest {
    /// <p>Specifies the number of <a>XssMatchSet</a> objects that you want AWS WAF to return for this request. If you have more <code>XssMatchSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>Rules</code>.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If you specify a value for <code>Limit</code> and you have more <a>XssMatchSet</a> objects than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>XssMatchSets</code>. For the second and subsequent <code>ListXssMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>XssMatchSets</code>.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

/// <p>The response to a <a>ListXssMatchSets</a> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListXssMatchSetsResponse {
    /// <p>If you have more <a>XssMatchSet</a> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>XssMatchSet</code> objects, submit another <code>ListXssMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>An array of <a>XssMatchSetSummary</a> objects.</p>
    #[serde(rename = "xssMatchSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xss_match_sets: Option<Vec<XssMatchSetSummary>>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>The Amazon Kinesis Data Firehose, <code>RedactedFields</code> information, and the web ACL Amazon Resource Name (ARN).</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LoggingConfiguration {
    /// <p>An array of Amazon Kinesis Data Firehose ARNs.</p>
    #[serde(rename = "logDestinationConfigs")]
    pub log_destination_configs: Vec<String>,
    /// <p>The parts of the request that you want redacted from the logs. For example, if you redact the cookie field, the cookie field in the firehose will be <code>xxx</code>. </p>
    #[serde(rename = "redactedFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted_fields: Option<Vec<FieldToMatch>>,
    /// <p>The Amazon Resource Name (ARN) of the web ACL that you want to associate with <code>LogDestinationConfigs</code>.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Specifies the <a>ByteMatchSet</a>, <a>IPSet</a>, <a>SqlInjectionMatchSet</a>, <a>XssMatchSet</a>, <a>RegexMatchSet</a>, <a>GeoMatchSet</a>, and <a>SizeConstraintSet</a> objects that you want to add to a <code>Rule</code> and, for each object, indicates whether you want to negate the settings, for example, requests that do NOT originate from the IP address 192.0.2.44. </p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Predicate {
    /// <p>A unique identifier for a predicate in a <code>Rule</code>, such as <code>ByteMatchSetId</code> or <code>IPSetId</code>. The ID is returned by the corresponding <code>Create</code> or <code>List</code> command.</p>
    #[serde(rename = "dataId")]
    pub data_id: String,
    /// <p>Set <code>Negated</code> to <code>False</code> if you want AWS WAF to allow, block, or count requests based on the settings in the specified <a>ByteMatchSet</a>, <a>IPSet</a>, <a>SqlInjectionMatchSet</a>, <a>XssMatchSet</a>, <a>RegexMatchSet</a>, <a>GeoMatchSet</a>, or <a>SizeConstraintSet</a>. For example, if an <code>IPSet</code> includes the IP address <code>192.0.2.44</code>, AWS WAF will allow or block requests based on that IP address.</p> <p>Set <code>Negated</code> to <code>True</code> if you want AWS WAF to allow or block a request based on the negation of the settings in the <a>ByteMatchSet</a>, <a>IPSet</a>, <a>SqlInjectionMatchSet</a>, <a>XssMatchSet</a>, <a>RegexMatchSet</a>, <a>GeoMatchSet</a>, or <a>SizeConstraintSet</a>. For example, if an <code>IPSet</code> includes the IP address <code>192.0.2.44</code>, AWS WAF will allow, block, or count requests based on all IP addresses <i>except</i> <code>192.0.2.44</code>.</p>
    #[serde(rename = "negated")]
    pub negated: bool,
    /// <p>The type of predicate in a <code>Rule</code>, such as <code>ByteMatch</code> or <code>IPSet</code>.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutLoggingConfigurationRequest {
    /// <p><p>The Amazon Kinesis Data Firehose that contains the inspected traffic information, the redacted fields details, and the Amazon Resource Name (ARN) of the web ACL to monitor.</p> <note> <p>When specifying <code>Type</code> in <code>RedactedFields</code>, you must use one of the following values: <code>URI</code>, <code>QUERY_STRING</code>, <code>HEADER</code>, or <code>METHOD</code>.</p> </note></p>
    #[serde(rename = "loggingConfiguration")]
    pub logging_configuration: LoggingConfiguration,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutLoggingConfigurationResponse {
    /// <p>The <a>LoggingConfiguration</a> that you submitted in the request.</p>
    #[serde(rename = "loggingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<LoggingConfiguration>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutPermissionPolicyRequest {
    /// <p>The policy to attach to the specified RuleGroup.</p>
    #[serde(rename = "policy")]
    pub policy: String,
    /// <p>The Amazon Resource Name (ARN) of the RuleGroup to which you want to attach the policy.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutPermissionPolicyResponse {}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>A <code>RateBasedRule</code> is identical to a regular <a>Rule</a>, with one addition: a <code>RateBasedRule</code> counts the number of requests that arrive from a specified IP address every five minutes. For example, based on recent requests that you&#39;ve seen from an attacker, you might create a <code>RateBasedRule</code> that includes the following conditions: </p> <ul> <li> <p>The requests come from 192.0.2.44.</p> </li> <li> <p>They contain the value <code>BadBot</code> in the <code>User-Agent</code> header.</p> </li> </ul> <p>In the rule, you also define the rate limit as 1,000.</p> <p>Requests that meet both of these conditions and exceed 1,000 requests every five minutes trigger the rule&#39;s action (block or count), which is defined in the web ACL.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RateBasedRule {
    /// <p>The <code>Predicates</code> object contains one <code>Predicate</code> element for each <a>ByteMatchSet</a>, <a>IPSet</a>, or <a>SqlInjectionMatchSet</a> object that you want to include in a <code>RateBasedRule</code>.</p>
    #[serde(rename = "matchPredicates")]
    pub match_predicates: Vec<Predicate>,
    /// <p>A friendly name or description for the metrics for a <code>RateBasedRule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the <code>RateBasedRule</code>.</p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>A friendly name or description for a <code>RateBasedRule</code>. You can't change the name of a <code>RateBasedRule</code> after you create it.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The field that AWS WAF uses to determine if requests are likely arriving from single source and thus subject to rate monitoring. The only valid value for <code>RateKey</code> is <code>IP</code>. <code>IP</code> indicates that requests arriving from the same IP address are subject to the <code>RateLimit</code> that is specified in the <code>RateBasedRule</code>.</p>
    #[serde(rename = "rateKey")]
    pub rate_key: String,
    /// <p>The maximum number of requests, which have an identical value in the field specified by the <code>RateKey</code>, allowed in a five-minute period. If the number of requests exceeds the <code>RateLimit</code> and the other predicates specified in the rule are also met, AWS WAF triggers the action that is specified for this rule.</p>
    #[serde(rename = "rateLimit")]
    pub rate_limit: i64,
    /// <p>A unique identifier for a <code>RateBasedRule</code>. You use <code>RuleId</code> to get more information about a <code>RateBasedRule</code> (see <a>GetRateBasedRule</a>), update a <code>RateBasedRule</code> (see <a>UpdateRateBasedRule</a>), insert a <code>RateBasedRule</code> into a <code>WebACL</code> or delete one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>RateBasedRule</code> from AWS WAF (see <a>DeleteRateBasedRule</a>).</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>In a <a>GetRegexMatchSet</a> request, <code>RegexMatchSet</code> is a complex type that contains the <code>RegexMatchSetId</code> and <code>Name</code> of a <code>RegexMatchSet</code>, and the values that you specified when you updated the <code>RegexMatchSet</code>.</p> <p> The values are contained in a <code>RegexMatchTuple</code> object, which specify the parts of web requests that you want AWS WAF to inspect and the values that you want AWS WAF to search for. If a <code>RegexMatchSet</code> contains more than one <code>RegexMatchTuple</code> object, a request needs to match the settings in only one <code>ByteMatchTuple</code> to be considered a match.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegexMatchSet {
    /// <p>A friendly name or description of the <a>RegexMatchSet</a>. You can't change <code>Name</code> after you create a <code>RegexMatchSet</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The <code>RegexMatchSetId</code> for a <code>RegexMatchSet</code>. You use <code>RegexMatchSetId</code> to get information about a <code>RegexMatchSet</code> (see <a>GetRegexMatchSet</a>), update a <code>RegexMatchSet</code> (see <a>UpdateRegexMatchSet</a>), insert a <code>RegexMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>RegexMatchSet</code> from AWS WAF (see <a>DeleteRegexMatchSet</a>).</p> <p> <code>RegexMatchSetId</code> is returned by <a>CreateRegexMatchSet</a> and by <a>ListRegexMatchSets</a>.</p>
    #[serde(rename = "regexMatchSetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_match_set_id: Option<String>,
    /// <p><p>Contains an array of <a>RegexMatchTuple</a> objects. Each <code>RegexMatchTuple</code> object contains: </p> <ul> <li> <p>The part of a web request that you want AWS WAF to inspect, such as a query string or the value of the <code>User-Agent</code> header. </p> </li> <li> <p>The identifier of the pattern (a regular expression) that you want AWS WAF to look for. For more information, see <a>RegexPatternSet</a>.</p> </li> <li> <p>Whether to perform any conversions on the request, such as converting it to lowercase, before inspecting it for the specified string.</p> </li> </ul></p>
    #[serde(rename = "regexMatchTuples")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_match_tuples: Option<Vec<RegexMatchTuple>>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returned by <a>ListRegexMatchSets</a>. Each <code>RegexMatchSetSummary</code> object includes the <code>Name</code> and <code>RegexMatchSetId</code> for one <a>RegexMatchSet</a>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegexMatchSetSummary {
    /// <p>A friendly name or description of the <a>RegexMatchSet</a>. You can't change <code>Name</code> after you create a <code>RegexMatchSet</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The <code>RegexMatchSetId</code> for a <code>RegexMatchSet</code>. You use <code>RegexMatchSetId</code> to get information about a <code>RegexMatchSet</code>, update a <code>RegexMatchSet</code>, remove a <code>RegexMatchSet</code> from a <code>Rule</code>, and delete a <code>RegexMatchSet</code> from AWS WAF.</p> <p> <code>RegexMatchSetId</code> is returned by <a>CreateRegexMatchSet</a> and by <a>ListRegexMatchSets</a>.</p>
    #[serde(rename = "regexMatchSetId")]
    pub regex_match_set_id: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>In an <a>UpdateRegexMatchSet</a> request, <code>RegexMatchSetUpdate</code> specifies whether to insert or delete a <a>RegexMatchTuple</a> and includes the settings for the <code>RegexMatchTuple</code>.</p></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegexMatchSetUpdate {
    /// <p>Specifies whether to insert or delete a <a>RegexMatchTuple</a>.</p>
    #[serde(rename = "action")]
    pub action: String,
    /// <p>Information about the part of a web request that you want AWS WAF to inspect and the identifier of the regular expression (regex) pattern that you want AWS WAF to search for. If you specify <code>DELETE</code> for the value of <code>Action</code>, the <code>RegexMatchTuple</code> values must exactly match the values in the <code>RegexMatchTuple</code> that you want to delete from the <code>RegexMatchSet</code>.</p>
    #[serde(rename = "regexMatchTuple")]
    pub regex_match_tuple: RegexMatchTuple,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>The regular expression pattern that you want AWS WAF to search for in web requests, the location in requests that you want AWS WAF to search, and other settings. Each <code>RegexMatchTuple</code> object contains: </p> <ul> <li> <p>The part of a web request that you want AWS WAF to inspect, such as a query string or the value of the <code>User-Agent</code> header. </p> </li> <li> <p>The identifier of the pattern (a regular expression) that you want AWS WAF to look for. For more information, see <a>RegexPatternSet</a>. </p> </li> <li> <p>Whether to perform any conversions on the request, such as converting it to lowercase, before inspecting it for the specified string.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RegexMatchTuple {
    /// <p>Specifies where in a web request to look for the <code>RegexPatternSet</code>.</p>
    #[serde(rename = "fieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>The <code>RegexPatternSetId</code> for a <code>RegexPatternSet</code>. You use <code>RegexPatternSetId</code> to get information about a <code>RegexPatternSet</code> (see <a>GetRegexPatternSet</a>), update a <code>RegexPatternSet</code> (see <a>UpdateRegexPatternSet</a>), insert a <code>RegexPatternSet</code> into a <code>RegexMatchSet</code> or delete one from a <code>RegexMatchSet</code> (see <a>UpdateRegexMatchSet</a>), and delete an <code>RegexPatternSet</code> from AWS WAF (see <a>DeleteRegexPatternSet</a>).</p> <p> <code>RegexPatternSetId</code> is returned by <a>CreateRegexPatternSet</a> and by <a>ListRegexPatternSets</a>.</p>
    #[serde(rename = "regexPatternSetId")]
    pub regex_pattern_set_id: String,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF. If you specify a transformation, AWS WAF performs the transformation on <code>RegexPatternSet</code> before inspecting a request for a match.</p> <p>You can only specify a single type of TextTransformation.</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system commandline command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \ " ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\f, formfeed, decimal 12</p> </li> <li> <p>\t, tab, decimal 9</p> </li> <li> <p>\n, newline, decimal 10</p> </li> <li> <p>\r, carriage return, decimal 13</p> </li> <li> <p>\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a "less than" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want to perform any text transformations.</p>
    #[serde(rename = "textTransformation")]
    pub text_transformation: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>The <code>RegexPatternSet</code> specifies the regular expression (regex) pattern that you want AWS WAF to search for, such as <code>B[a@]dB[o0]t</code>. You can then configure AWS WAF to reject those requests.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegexPatternSet {
    /// <p>A friendly name or description of the <a>RegexPatternSet</a>. You can't change <code>Name</code> after you create a <code>RegexPatternSet</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The identifier for the <code>RegexPatternSet</code>. You use <code>RegexPatternSetId</code> to get information about a <code>RegexPatternSet</code>, update a <code>RegexPatternSet</code>, remove a <code>RegexPatternSet</code> from a <code>RegexMatchSet</code>, and delete a <code>RegexPatternSet</code> from AWS WAF.</p> <p> <code>RegexMatchSetId</code> is returned by <a>CreateRegexPatternSet</a> and by <a>ListRegexPatternSets</a>.</p>
    #[serde(rename = "regexPatternSetId")]
    pub regex_pattern_set_id: String,
    /// <p>Specifies the regular expression (regex) patterns that you want AWS WAF to search for, such as <code>B[a@]dB[o0]t</code>.</p>
    #[serde(rename = "regexPatternStrings")]
    pub regex_pattern_strings: Vec<String>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returned by <a>ListRegexPatternSets</a>. Each <code>RegexPatternSetSummary</code> object includes the <code>Name</code> and <code>RegexPatternSetId</code> for one <a>RegexPatternSet</a>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegexPatternSetSummary {
    /// <p>A friendly name or description of the <a>RegexPatternSet</a>. You can't change <code>Name</code> after you create a <code>RegexPatternSet</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The <code>RegexPatternSetId</code> for a <code>RegexPatternSet</code>. You use <code>RegexPatternSetId</code> to get information about a <code>RegexPatternSet</code>, update a <code>RegexPatternSet</code>, remove a <code>RegexPatternSet</code> from a <code>RegexMatchSet</code>, and delete a <code>RegexPatternSet</code> from AWS WAF.</p> <p> <code>RegexPatternSetId</code> is returned by <a>CreateRegexPatternSet</a> and by <a>ListRegexPatternSets</a>.</p>
    #[serde(rename = "regexPatternSetId")]
    pub regex_pattern_set_id: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>In an <a>UpdateRegexPatternSet</a> request, <code>RegexPatternSetUpdate</code> specifies whether to insert or delete a <code>RegexPatternString</code> and includes the settings for the <code>RegexPatternString</code>.</p></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegexPatternSetUpdate {
    /// <p>Specifies whether to insert or delete a <code>RegexPatternString</code>.</p>
    #[serde(rename = "action")]
    pub action: String,
    /// <p>Specifies the regular expression (regex) pattern that you want AWS WAF to search for, such as <code>B[a@]dB[o0]t</code>.</p>
    #[serde(rename = "regexPatternString")]
    pub regex_pattern_string: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>A combination of <a>ByteMatchSet</a>, <a>IPSet</a>, and/or <a>SqlInjectionMatchSet</a> objects that identify the web requests that you want to allow, block, or count. For example, you might create a <code>Rule</code> that includes the following predicates:</p> <ul> <li> <p>An <code>IPSet</code> that causes AWS WAF to search for web requests that originate from the IP address <code>192.0.2.44</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that causes AWS WAF to search for web requests for which the value of the <code>User-Agent</code> header is <code>BadBot</code>.</p> </li> </ul> <p>To match the settings in this <code>Rule</code>, a request must originate from <code>192.0.2.44</code> AND include a <code>User-Agent</code> header for which the value is <code>BadBot</code>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Rule {
    /// <p>A friendly name or description for the metrics for this <code>Rule</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change <code>MetricName</code> after you create the <code>Rule</code>.</p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>The friendly name or description for the <code>Rule</code>. You can't change the name of a <code>Rule</code> after you create it.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The <code>Predicates</code> object contains one <code>Predicate</code> element for each <a>ByteMatchSet</a>, <a>IPSet</a>, or <a>SqlInjectionMatchSet</a> object that you want to include in a <code>Rule</code>.</p>
    #[serde(rename = "predicates")]
    pub predicates: Vec<Predicate>,
    /// <p>A unique identifier for a <code>Rule</code>. You use <code>RuleId</code> to get more information about a <code>Rule</code> (see <a>GetRule</a>), update a <code>Rule</code> (see <a>UpdateRule</a>), insert a <code>Rule</code> into a <code>WebACL</code> or delete a one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>Rule</code> from AWS WAF (see <a>DeleteRule</a>).</p> <p> <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>A collection of predefined rules that you can add to a web ACL.</p> <p>Rule groups are subject to the following limits:</p> <ul> <li> <p>Three rule groups per account. You can request an increase to this limit by contacting customer support.</p> </li> <li> <p>One rule group per web ACL.</p> </li> <li> <p>Ten rules per rule group.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RuleGroup {
    /// <p>A friendly name or description for the metrics for this <code>RuleGroup</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the <code>RuleGroup</code>.</p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>The friendly name or description for the <code>RuleGroup</code>. You can't change the name of a <code>RuleGroup</code> after you create it.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier for a <code>RuleGroup</code>. You use <code>RuleGroupId</code> to get more information about a <code>RuleGroup</code> (see <a>GetRuleGroup</a>), update a <code>RuleGroup</code> (see <a>UpdateRuleGroup</a>), insert a <code>RuleGroup</code> into a <code>WebACL</code> or delete a one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>RuleGroup</code> from AWS WAF (see <a>DeleteRuleGroup</a>).</p> <p> <code>RuleGroupId</code> is returned by <a>CreateRuleGroup</a> and by <a>ListRuleGroups</a>.</p>
    #[serde(rename = "ruleGroupId")]
    pub rule_group_id: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Contains the identifier and the friendly name or description of the <code>RuleGroup</code>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RuleGroupSummary {
    /// <p>A friendly name or description of the <a>RuleGroup</a>. You can't change the name of a <code>RuleGroup</code> after you create it.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A unique identifier for a <code>RuleGroup</code>. You use <code>RuleGroupId</code> to get more information about a <code>RuleGroup</code> (see <a>GetRuleGroup</a>), update a <code>RuleGroup</code> (see <a>UpdateRuleGroup</a>), insert a <code>RuleGroup</code> into a <code>WebACL</code> or delete one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>RuleGroup</code> from AWS WAF (see <a>DeleteRuleGroup</a>).</p> <p> <code>RuleGroupId</code> is returned by <a>CreateRuleGroup</a> and by <a>ListRuleGroups</a>.</p>
    #[serde(rename = "ruleGroupId")]
    pub rule_group_id: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Specifies an <code>ActivatedRule</code> and indicates whether you want to add it to a <code>RuleGroup</code> or delete it from a <code>RuleGroup</code>.</p></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RuleGroupUpdate {
    /// <p>Specify <code>INSERT</code> to add an <code>ActivatedRule</code> to a <code>RuleGroup</code>. Use <code>DELETE</code> to remove an <code>ActivatedRule</code> from a <code>RuleGroup</code>.</p>
    #[serde(rename = "action")]
    pub action: String,
    /// <p>The <code>ActivatedRule</code> object specifies a <code>Rule</code> that you want to insert or delete, the priority of the <code>Rule</code> in the <code>WebACL</code>, and the action that you want AWS WAF to take when a web request matches the <code>Rule</code> (<code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>).</p>
    #[serde(rename = "activatedRule")]
    pub activated_rule: ActivatedRule,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Contains the identifier and the friendly name or description of the <code>Rule</code>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RuleSummary {
    /// <p>A friendly name or description of the <a>Rule</a>. You can't change the name of a <code>Rule</code> after you create it.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A unique identifier for a <code>Rule</code>. You use <code>RuleId</code> to get more information about a <code>Rule</code> (see <a>GetRule</a>), update a <code>Rule</code> (see <a>UpdateRule</a>), insert a <code>Rule</code> into a <code>WebACL</code> or delete one from a <code>WebACL</code> (see <a>UpdateWebACL</a>), or delete a <code>Rule</code> from AWS WAF (see <a>DeleteRule</a>).</p> <p> <code>RuleId</code> is returned by <a>CreateRule</a> and by <a>ListRules</a>.</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Specifies a <code>Predicate</code> (such as an <code>IPSet</code>) and indicates whether you want to add it to a <code>Rule</code> or delete it from a <code>Rule</code>.</p></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RuleUpdate {
    /// <p>Specify <code>INSERT</code> to add a <code>Predicate</code> to a <code>Rule</code>. Use <code>DELETE</code> to remove a <code>Predicate</code> from a <code>Rule</code>.</p>
    #[serde(rename = "action")]
    pub action: String,
    /// <p>The ID of the <code>Predicate</code> (such as an <code>IPSet</code>) that you want to add to a <code>Rule</code>.</p>
    #[serde(rename = "predicate")]
    pub predicate: Predicate,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>The response from a <a>GetSampledRequests</a> request includes a <code>SampledHTTPRequests</code> complex type that appears as <code>SampledRequests</code> in the response syntax. <code>SampledHTTPRequests</code> contains one <code>SampledHTTPRequest</code> object for each web request that is returned by <code>GetSampledRequests</code>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SampledHTTPRequest {
    /// <p>The action for the <code>Rule</code> that the request matched: <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>A complex type that contains detailed information about the request.</p>
    #[serde(rename = "request")]
    pub request: HTTPRequest,
    /// <p>This value is returned if the <code>GetSampledRequests</code> request specifies the ID of a <code>RuleGroup</code> rather than the ID of an individual rule. <code>RuleWithinRuleGroup</code> is the rule within the specified <code>RuleGroup</code> that matched the request listed in the response.</p>
    #[serde(rename = "ruleWithinRuleGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_within_rule_group: Option<String>,
    /// <p>The time at which AWS WAF received the request from your AWS resource, in Unix time format (in seconds).</p>
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    /// <p>A value that indicates how one result in the response relates proportionally to other results in the response. A result that has a weight of <code>2</code> represents roughly twice as many CloudFront web requests as a result that has a weight of <code>1</code>.</p>
    #[serde(rename = "weight")]
    pub weight: i64,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Specifies a constraint on the size of a part of the web request. AWS WAF uses the <code>Size</code>, <code>ComparisonOperator</code>, and <code>FieldToMatch</code> to build an expression in the form of &quot;<code>Size</code> <code>ComparisonOperator</code> size in bytes of <code>FieldToMatch</code>&quot;. If that expression is true, the <code>SizeConstraint</code> is considered to match.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SizeConstraint {
    /// <p>The type of comparison you want AWS WAF to perform. AWS WAF uses this in combination with the provided <code>Size</code> and <code>FieldToMatch</code> to build an expression in the form of "<code>Size</code> <code>ComparisonOperator</code> size in bytes of <code>FieldToMatch</code>". If that expression is true, the <code>SizeConstraint</code> is considered to match.</p> <p> <b>EQ</b>: Used to test if the <code>Size</code> is equal to the size of the <code>FieldToMatch</code> </p> <p> <b>NE</b>: Used to test if the <code>Size</code> is not equal to the size of the <code>FieldToMatch</code> </p> <p> <b>LE</b>: Used to test if the <code>Size</code> is less than or equal to the size of the <code>FieldToMatch</code> </p> <p> <b>LT</b>: Used to test if the <code>Size</code> is strictly less than the size of the <code>FieldToMatch</code> </p> <p> <b>GE</b>: Used to test if the <code>Size</code> is greater than or equal to the size of the <code>FieldToMatch</code> </p> <p> <b>GT</b>: Used to test if the <code>Size</code> is strictly greater than the size of the <code>FieldToMatch</code> </p>
    #[serde(rename = "comparisonOperator")]
    pub comparison_operator: String,
    /// <p>Specifies where in a web request to look for the size constraint.</p>
    #[serde(rename = "fieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>The size in bytes that you want AWS WAF to compare against the size of the specified <code>FieldToMatch</code>. AWS WAF uses this in combination with <code>ComparisonOperator</code> and <code>FieldToMatch</code> to build an expression in the form of "<code>Size</code> <code>ComparisonOperator</code> size in bytes of <code>FieldToMatch</code>". If that expression is true, the <code>SizeConstraint</code> is considered to match.</p> <p>Valid values for size are 0 - 21474836480 bytes (0 - 20 GB).</p> <p>If you specify <code>URI</code> for the value of <code>Type</code>, the / in the URI counts as one character. For example, the URI <code>/logo.jpg</code> is nine characters long.</p>
    #[serde(rename = "size")]
    pub size: i64,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF. If you specify a transformation, AWS WAF performs the transformation on <code>FieldToMatch</code> before inspecting it for a match.</p> <p>You can only specify a single type of TextTransformation.</p> <p>Note that if you choose <code>BODY</code> for the value of <code>Type</code>, you must choose <code>NONE</code> for <code>TextTransformation</code> because CloudFront forwards only the first 8192 bytes for inspection. </p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want to perform any text transformations.</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system command line command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \ " ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\f, formfeed, decimal 12</p> </li> <li> <p>\t, tab, decimal 9</p> </li> <li> <p>\n, newline, decimal 10</p> </li> <li> <p>\r, carriage return, decimal 13</p> </li> <li> <p>\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a "less than" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p>
    #[serde(rename = "textTransformation")]
    pub text_transformation: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>A complex type that contains <code>SizeConstraint</code> objects, which specify the parts of web requests that you want AWS WAF to inspect the size of. If a <code>SizeConstraintSet</code> contains more than one <code>SizeConstraint</code> object, a request only needs to match one constraint to be considered a match.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SizeConstraintSet {
    /// <p>The name, if any, of the <code>SizeConstraintSet</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier for a <code>SizeConstraintSet</code>. You use <code>SizeConstraintSetId</code> to get information about a <code>SizeConstraintSet</code> (see <a>GetSizeConstraintSet</a>), update a <code>SizeConstraintSet</code> (see <a>UpdateSizeConstraintSet</a>), insert a <code>SizeConstraintSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>SizeConstraintSet</code> from AWS WAF (see <a>DeleteSizeConstraintSet</a>).</p> <p> <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>
    #[serde(rename = "sizeConstraintSetId")]
    pub size_constraint_set_id: String,
    /// <p>Specifies the parts of web requests that you want to inspect the size of.</p>
    #[serde(rename = "sizeConstraints")]
    pub size_constraints: Vec<SizeConstraint>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>The <code>Id</code> and <code>Name</code> of a <code>SizeConstraintSet</code>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SizeConstraintSetSummary {
    /// <p>The name of the <code>SizeConstraintSet</code>, if any.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A unique identifier for a <code>SizeConstraintSet</code>. You use <code>SizeConstraintSetId</code> to get information about a <code>SizeConstraintSet</code> (see <a>GetSizeConstraintSet</a>), update a <code>SizeConstraintSet</code> (see <a>UpdateSizeConstraintSet</a>), insert a <code>SizeConstraintSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>SizeConstraintSet</code> from AWS WAF (see <a>DeleteSizeConstraintSet</a>).</p> <p> <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>
    #[serde(rename = "sizeConstraintSetId")]
    pub size_constraint_set_id: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Specifies the part of a web request that you want to inspect the size of and indicates whether you want to add the specification to a <a>SizeConstraintSet</a> or delete it from a <code>SizeConstraintSet</code>.</p></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SizeConstraintSetUpdate {
    /// <p>Specify <code>INSERT</code> to add a <a>SizeConstraintSetUpdate</a> to a <a>SizeConstraintSet</a>. Use <code>DELETE</code> to remove a <code>SizeConstraintSetUpdate</code> from a <code>SizeConstraintSet</code>.</p>
    #[serde(rename = "action")]
    pub action: String,
    /// <p>Specifies a constraint on the size of a part of the web request. AWS WAF uses the <code>Size</code>, <code>ComparisonOperator</code>, and <code>FieldToMatch</code> to build an expression in the form of "<code>Size</code> <code>ComparisonOperator</code> size in bytes of <code>FieldToMatch</code>". If that expression is true, the <code>SizeConstraint</code> is considered to match.</p>
    #[serde(rename = "sizeConstraint")]
    pub size_constraint: SizeConstraint,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>A complex type that contains <code>SqlInjectionMatchTuple</code> objects, which specify the parts of web requests that you want AWS WAF to inspect for snippets of malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header. If a <code>SqlInjectionMatchSet</code> contains more than one <code>SqlInjectionMatchTuple</code> object, a request needs to include snippets of SQL code in only one of the specified parts of the request to be considered a match.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SqlInjectionMatchSet {
    /// <p>The name, if any, of the <code>SqlInjectionMatchSet</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier for a <code>SqlInjectionMatchSet</code>. You use <code>SqlInjectionMatchSetId</code> to get information about a <code>SqlInjectionMatchSet</code> (see <a>GetSqlInjectionMatchSet</a>), update a <code>SqlInjectionMatchSet</code> (see <a>UpdateSqlInjectionMatchSet</a>), insert a <code>SqlInjectionMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>SqlInjectionMatchSet</code> from AWS WAF (see <a>DeleteSqlInjectionMatchSet</a>).</p> <p> <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>
    #[serde(rename = "sqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: String,
    /// <p>Specifies the parts of web requests that you want to inspect for snippets of malicious SQL code.</p>
    #[serde(rename = "sqlInjectionMatchTuples")]
    pub sql_injection_match_tuples: Vec<SqlInjectionMatchTuple>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>The <code>Id</code> and <code>Name</code> of a <code>SqlInjectionMatchSet</code>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SqlInjectionMatchSetSummary {
    /// <p>The name of the <code>SqlInjectionMatchSet</code>, if any, specified by <code>Id</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A unique identifier for a <code>SqlInjectionMatchSet</code>. You use <code>SqlInjectionMatchSetId</code> to get information about a <code>SqlInjectionMatchSet</code> (see <a>GetSqlInjectionMatchSet</a>), update a <code>SqlInjectionMatchSet</code> (see <a>UpdateSqlInjectionMatchSet</a>), insert a <code>SqlInjectionMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete a <code>SqlInjectionMatchSet</code> from AWS WAF (see <a>DeleteSqlInjectionMatchSet</a>).</p> <p> <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>
    #[serde(rename = "sqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Specifies the part of a web request that you want to inspect for snippets of malicious SQL code and indicates whether you want to add the specification to a <a>SqlInjectionMatchSet</a> or delete it from a <code>SqlInjectionMatchSet</code>.</p></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SqlInjectionMatchSetUpdate {
    /// <p>Specify <code>INSERT</code> to add a <a>SqlInjectionMatchSetUpdate</a> to a <a>SqlInjectionMatchSet</a>. Use <code>DELETE</code> to remove a <code>SqlInjectionMatchSetUpdate</code> from a <code>SqlInjectionMatchSet</code>.</p>
    #[serde(rename = "action")]
    pub action: String,
    /// <p>Specifies the part of a web request that you want AWS WAF to inspect for snippets of malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header.</p>
    #[serde(rename = "sqlInjectionMatchTuple")]
    pub sql_injection_match_tuple: SqlInjectionMatchTuple,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Specifies the part of a web request that you want AWS WAF to inspect for snippets of malicious SQL code and, if you want AWS WAF to inspect a header, the name of the header.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SqlInjectionMatchTuple {
    /// <p>Specifies where in a web request to look for snippets of malicious SQL code.</p>
    #[serde(rename = "fieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF. If you specify a transformation, AWS WAF performs the transformation on <code>FieldToMatch</code> before inspecting it for a match.</p> <p>You can only specify a single type of TextTransformation.</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system command line command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \ " ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\f, formfeed, decimal 12</p> </li> <li> <p>\t, tab, decimal 9</p> </li> <li> <p>\n, newline, decimal 10</p> </li> <li> <p>\r, carriage return, decimal 13</p> </li> <li> <p>\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a "less than" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want to perform any text transformations.</p>
    #[serde(rename = "textTransformation")]
    pub text_transformation: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>A summary of the rule groups you are subscribed to.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubscribedRuleGroupSummary {
    /// <p>A friendly name or description for the metrics for this <code>RuleGroup</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change the name of the metric after you create the <code>RuleGroup</code>.</p>
    #[serde(rename = "metricName")]
    pub metric_name: String,
    /// <p>A friendly name or description of the <code>RuleGroup</code>. You can't change the name of a <code>RuleGroup</code> after you create it.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A unique identifier for a <code>RuleGroup</code>.</p>
    #[serde(rename = "ruleGroupId")]
    pub rule_group_id: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>A tag associated with an AWS resource. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. For example, you might set the tag key to &quot;customer&quot; and the value to the customer name or ID. You can specify one or more tags to add to each AWS resource, up to 50 tags for a resource.</p> <p>Tagging is only available through the API, SDKs, and CLI. You can&#39;t manage or view tags through the AWS WAF Classic console. You can tag the AWS resources that you manage through AWS WAF Classic: web ACLs, rule groups, and rules. </p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p><p/></p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p><p/></p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Information for a tag associated with an AWS resource. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. For example, you might set the tag key to &quot;customer&quot; and the value to the customer name or ID. You can specify one or more tags to add to each AWS resource, up to 50 tags for a resource.</p> <p>Tagging is only available through the API, SDKs, and CLI. You can&#39;t manage or view tags through the AWS WAF Classic console. You can tag the AWS resources that you manage through AWS WAF Classic: web ACLs, rule groups, and rules. </p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagInfoForResource {
    /// <p><p/></p>
    #[serde(rename = "resourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "tagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p><p/></p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p><p/></p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>In a <a>GetSampledRequests</a> request, the <code>StartTime</code> and <code>EndTime</code> objects specify the time range for which you want AWS WAF to return a sample of web requests.</p> <p>You must specify the times in Coordinated Universal Time (UTC) format. UTC format includes the special designator, <code>Z</code>. For example, <code>&quot;2016-09-27T14:50Z&quot;</code>. </p> <p>In a <a>GetSampledRequests</a> response, the <code>StartTime</code> and <code>EndTime</code> objects specify the time range for which AWS WAF actually returned a sample of web requests. AWS WAF gets the specified number of requests from among the first 5,000 requests that your AWS resource receives during the specified time period. If your resource receives more than 5,000 requests during that period, AWS WAF stops sampling after the 5,000th request. In that case, <code>EndTime</code> is the time that AWS WAF received the 5,000th request. </p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TimeWindow {
    /// <p>The end of the time range from which you want <code>GetSampledRequests</code> to return a sample of the requests that your AWS resource received. You must specify the date and time in Coordinated Universal Time (UTC) format. UTC format includes the special designator, <code>Z</code>. For example, <code>"2016-09-27T14:50Z"</code>. You can specify any time range in the previous three hours.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The beginning of the time range from which you want <code>GetSampledRequests</code> to return a sample of the requests that your AWS resource received. You must specify the date and time in Coordinated Universal Time (UTC) format. UTC format includes the special designator, <code>Z</code>. For example, <code>"2016-09-27T14:50Z"</code>. You can specify any time range in the previous three hours.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p><p/></p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p><p/></p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateByteMatchSetRequest {
    /// <p>The <code>ByteMatchSetId</code> of the <a>ByteMatchSet</a> that you want to update. <code>ByteMatchSetId</code> is returned by <a>CreateByteMatchSet</a> and by <a>ListByteMatchSets</a>.</p>
    #[serde(rename = "byteMatchSetId")]
    pub byte_match_set_id: String,
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p><p>An array of <code>ByteMatchSetUpdate</code> objects that you want to insert into or delete from a <a>ByteMatchSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>ByteMatchSetUpdate</a>: Contains <code>Action</code> and <code>ByteMatchTuple</code> </p> </li> <li> <p> <a>ByteMatchTuple</a>: Contains <code>FieldToMatch</code>, <code>PositionalConstraint</code>, <code>TargetString</code>, and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "updates")]
    pub updates: Vec<ByteMatchSetUpdate>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateByteMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateByteMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGeoMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>GeoMatchSetId</code> of the <a>GeoMatchSet</a> that you want to update. <code>GeoMatchSetId</code> is returned by <a>CreateGeoMatchSet</a> and by <a>ListGeoMatchSets</a>.</p>
    #[serde(rename = "geoMatchSetId")]
    pub geo_match_set_id: String,
    /// <p><p>An array of <code>GeoMatchSetUpdate</code> objects that you want to insert into or delete from an <a>GeoMatchSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>GeoMatchSetUpdate</a>: Contains <code>Action</code> and <code>GeoMatchConstraint</code> </p> </li> <li> <p> <a>GeoMatchConstraint</a>: Contains <code>Type</code> and <code>Value</code> </p> <p>You can have only one <code>Type</code> and <code>Value</code> per <code>GeoMatchConstraint</code>. To add multiple countries, include multiple <code>GeoMatchSetUpdate</code> objects in your request.</p> </li> </ul></p>
    #[serde(rename = "updates")]
    pub updates: Vec<GeoMatchSetUpdate>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGeoMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateGeoMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateIPSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>IPSetId</code> of the <a>IPSet</a> that you want to update. <code>IPSetId</code> is returned by <a>CreateIPSet</a> and by <a>ListIPSets</a>.</p>
    #[serde(rename = "iPSetId")]
    pub ip_set_id: String,
    /// <p>An array of <code>IPSetUpdate</code> objects that you want to insert into or delete from an <a>IPSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>IPSetUpdate</a>: Contains <code>Action</code> and <code>IPSetDescriptor</code> </p> </li> <li> <p> <a>IPSetDescriptor</a>: Contains <code>Type</code> and <code>Value</code> </p> </li> </ul> <p>You can insert a maximum of 1000 addresses in a single request.</p>
    #[serde(rename = "updates")]
    pub updates: Vec<IPSetUpdate>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateIPSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateIPSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRateBasedRuleRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The maximum number of requests, which have an identical value in the field specified by the <code>RateKey</code>, allowed in a five-minute period. If the number of requests exceeds the <code>RateLimit</code> and the other predicates specified in the rule are also met, AWS WAF triggers the action that is specified for this rule.</p>
    #[serde(rename = "rateLimit")]
    pub rate_limit: i64,
    /// <p>The <code>RuleId</code> of the <code>RateBasedRule</code> that you want to update. <code>RuleId</code> is returned by <code>CreateRateBasedRule</code> and by <a>ListRateBasedRules</a>.</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    /// <p>An array of <code>RuleUpdate</code> objects that you want to insert into or delete from a <a>RateBasedRule</a>. </p>
    #[serde(rename = "updates")]
    pub updates: Vec<RuleUpdate>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRateBasedRuleResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateRateBasedRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRegexMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>RegexMatchSetId</code> of the <a>RegexMatchSet</a> that you want to update. <code>RegexMatchSetId</code> is returned by <a>CreateRegexMatchSet</a> and by <a>ListRegexMatchSets</a>.</p>
    #[serde(rename = "regexMatchSetId")]
    pub regex_match_set_id: String,
    /// <p>An array of <code>RegexMatchSetUpdate</code> objects that you want to insert into or delete from a <a>RegexMatchSet</a>. For more information, see <a>RegexMatchTuple</a>.</p>
    #[serde(rename = "updates")]
    pub updates: Vec<RegexMatchSetUpdate>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRegexMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateRegexMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRegexPatternSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>RegexPatternSetId</code> of the <a>RegexPatternSet</a> that you want to update. <code>RegexPatternSetId</code> is returned by <a>CreateRegexPatternSet</a> and by <a>ListRegexPatternSets</a>.</p>
    #[serde(rename = "regexPatternSetId")]
    pub regex_pattern_set_id: String,
    /// <p>An array of <code>RegexPatternSetUpdate</code> objects that you want to insert into or delete from a <a>RegexPatternSet</a>.</p>
    #[serde(rename = "updates")]
    pub updates: Vec<RegexPatternSetUpdate>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRegexPatternSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateRegexPatternSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRuleGroupRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>RuleGroupId</code> of the <a>RuleGroup</a> that you want to update. <code>RuleGroupId</code> is returned by <a>CreateRuleGroup</a> and by <a>ListRuleGroups</a>.</p>
    #[serde(rename = "ruleGroupId")]
    pub rule_group_id: String,
    /// <p>An array of <code>RuleGroupUpdate</code> objects that you want to insert into or delete from a <a>RuleGroup</a>.</p> <p>You can only insert <code>REGULAR</code> rules into a rule group.</p> <p> <code>ActivatedRule|OverrideAction</code> applies only when updating or adding a <code>RuleGroup</code> to a <code>WebACL</code>. In this case you do not use <code>ActivatedRule|Action</code>. For all other update requests, <code>ActivatedRule|Action</code> is used instead of <code>ActivatedRule|OverrideAction</code>.</p>
    #[serde(rename = "updates")]
    pub updates: Vec<RuleGroupUpdate>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRuleGroupResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateRuleGroup</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateRuleRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>RuleId</code> of the <code>Rule</code> that you want to update. <code>RuleId</code> is returned by <code>CreateRule</code> and by <a>ListRules</a>.</p>
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    /// <p><p>An array of <code>RuleUpdate</code> objects that you want to insert into or delete from a <a>Rule</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>RuleUpdate</a>: Contains <code>Action</code> and <code>Predicate</code> </p> </li> <li> <p> <a>Predicate</a>: Contains <code>DataId</code>, <code>Negated</code>, and <code>Type</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "updates")]
    pub updates: Vec<RuleUpdate>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateRuleResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateRule</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSizeConstraintSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>SizeConstraintSetId</code> of the <a>SizeConstraintSet</a> that you want to update. <code>SizeConstraintSetId</code> is returned by <a>CreateSizeConstraintSet</a> and by <a>ListSizeConstraintSets</a>.</p>
    #[serde(rename = "sizeConstraintSetId")]
    pub size_constraint_set_id: String,
    /// <p><p>An array of <code>SizeConstraintSetUpdate</code> objects that you want to insert into or delete from a <a>SizeConstraintSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>SizeConstraintSetUpdate</a>: Contains <code>Action</code> and <code>SizeConstraint</code> </p> </li> <li> <p> <a>SizeConstraint</a>: Contains <code>FieldToMatch</code>, <code>TextTransformation</code>, <code>ComparisonOperator</code>, and <code>Size</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "updates")]
    pub updates: Vec<SizeConstraintSetUpdate>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSizeConstraintSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateSizeConstraintSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

/// <p>A request to update a <a>SqlInjectionMatchSet</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSqlInjectionMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>The <code>SqlInjectionMatchSetId</code> of the <code>SqlInjectionMatchSet</code> that you want to update. <code>SqlInjectionMatchSetId</code> is returned by <a>CreateSqlInjectionMatchSet</a> and by <a>ListSqlInjectionMatchSets</a>.</p>
    #[serde(rename = "sqlInjectionMatchSetId")]
    pub sql_injection_match_set_id: String,
    /// <p><p>An array of <code>SqlInjectionMatchSetUpdate</code> objects that you want to insert into or delete from a <a>SqlInjectionMatchSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>SqlInjectionMatchSetUpdate</a>: Contains <code>Action</code> and <code>SqlInjectionMatchTuple</code> </p> </li> <li> <p> <a>SqlInjectionMatchTuple</a>: Contains <code>FieldToMatch</code> and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "updates")]
    pub updates: Vec<SqlInjectionMatchSetUpdate>,
}

/// <p>The response to an <a>UpdateSqlInjectionMatchSets</a> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSqlInjectionMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateSqlInjectionMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateWebACLRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p>A default action for the web ACL, either ALLOW or BLOCK. AWS WAF performs the default action if a request doesn't match the criteria in any of the rules in a web ACL.</p>
    #[serde(rename = "defaultAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<WafAction>,
    /// <p><p>An array of updates to make to the <a>WebACL</a>.</p> <p>An array of <code>WebACLUpdate</code> objects that you want to insert into or delete from a <a>WebACL</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>WebACLUpdate</a>: Contains <code>Action</code> and <code>ActivatedRule</code> </p> </li> <li> <p> <a>ActivatedRule</a>: Contains <code>Action</code>, <code>OverrideAction</code>, <code>Priority</code>, <code>RuleId</code>, and <code>Type</code>. <code>ActivatedRule|OverrideAction</code> applies only when updating or adding a <code>RuleGroup</code> to a <code>WebACL</code>. In this case, you do not use <code>ActivatedRule|Action</code>. For all other update requests, <code>ActivatedRule|Action</code> is used instead of <code>ActivatedRule|OverrideAction</code>. </p> </li> <li> <p> <a>WafAction</a>: Contains <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "updates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<WebACLUpdate>>,
    /// <p>The <code>WebACLId</code> of the <a>WebACL</a> that you want to update. <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>
    #[serde(rename = "webACLId")]
    pub web_acl_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateWebACLResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateWebACL</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

/// <p>A request to update an <a>XssMatchSet</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateXssMatchSetRequest {
    /// <p>The value returned by the most recent call to <a>GetChangeToken</a>.</p>
    #[serde(rename = "changeToken")]
    pub change_token: String,
    /// <p><p>An array of <code>XssMatchSetUpdate</code> objects that you want to insert into or delete from an <a>XssMatchSet</a>. For more information, see the applicable data types:</p> <ul> <li> <p> <a>XssMatchSetUpdate</a>: Contains <code>Action</code> and <code>XssMatchTuple</code> </p> </li> <li> <p> <a>XssMatchTuple</a>: Contains <code>FieldToMatch</code> and <code>TextTransformation</code> </p> </li> <li> <p> <a>FieldToMatch</a>: Contains <code>Data</code> and <code>Type</code> </p> </li> </ul></p>
    #[serde(rename = "updates")]
    pub updates: Vec<XssMatchSetUpdate>,
    /// <p>The <code>XssMatchSetId</code> of the <code>XssMatchSet</code> that you want to update. <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>
    #[serde(rename = "xssMatchSetId")]
    pub xss_match_set_id: String,
}

/// <p>The response to an <a>UpdateXssMatchSets</a> request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateXssMatchSetResponse {
    /// <p>The <code>ChangeToken</code> that you used to submit the <code>UpdateXssMatchSet</code> request. You can also use this value to query the status of the request. For more information, see <a>GetChangeTokenStatus</a>.</p>
    #[serde(rename = "changeToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_token: Option<String>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>For the action that is associated with a rule in a <code>WebACL</code>, specifies the action that you want AWS WAF to perform when a web request matches all of the conditions in a rule. For the default action in a <code>WebACL</code>, specifies the action that you want AWS WAF to take when a web request doesn&#39;t match all of the conditions in any of the rules in a <code>WebACL</code>. </p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WafAction {
    /// <p><p>Specifies how you want AWS WAF to respond to requests that match the settings in a <code>Rule</code>. Valid settings include the following:</p> <ul> <li> <p> <code>ALLOW</code>: AWS WAF allows requests</p> </li> <li> <p> <code>BLOCK</code>: AWS WAF blocks requests</p> </li> <li> <p> <code>COUNT</code>: AWS WAF increments a counter of the requests that match all of the conditions in the rule. AWS WAF then continues to inspect the web request based on the remaining rules in the web ACL. You can&#39;t specify <code>COUNT</code> for the default action for a <code>WebACL</code>.</p> </li> </ul></p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>The action to take if any rule within the <code>RuleGroup</code> matches a request. </p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WafOverrideAction {
    /// <p> <code>COUNT</code> overrides the action specified by the individual rule within a <code>RuleGroup</code> . If set to <code>NONE</code>, the rule's action will take place.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Contains the <code>Rules</code> that identify the requests that you want to allow, block, or count. In a <code>WebACL</code>, you also specify a default action (<code>ALLOW</code> or <code>BLOCK</code>), and the action for each <code>Rule</code> that you add to a <code>WebACL</code>, for example, block requests from specified IP addresses or block requests from specified referrers. You also associate the <code>WebACL</code> with a CloudFront distribution to identify the requests that you want AWS WAF to filter. If you add more than one <code>Rule</code> to a <code>WebACL</code>, a request needs to match only one of the specifications to be allowed, blocked, or counted. For more information, see <a>UpdateWebACL</a>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WebACL {
    /// <p>The action to perform if none of the <code>Rules</code> contained in the <code>WebACL</code> match. The action is specified by the <a>WafAction</a> object.</p>
    #[serde(rename = "defaultAction")]
    pub default_action: WafAction,
    /// <p>A friendly name or description for the metrics for this <code>WebACL</code>. The name can contain only alphanumeric characters (A-Z, a-z, 0-9), with maximum length 128 and minimum length one. It can't contain whitespace or metric names reserved for AWS WAF, including "All" and "Default_Action." You can't change <code>MetricName</code> after you create the <code>WebACL</code>.</p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// <p>A friendly name or description of the <code>WebACL</code>. You can't change the name of a <code>WebACL</code> after you create it.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array that contains the action for each <code>Rule</code> in a <code>WebACL</code>, the priority of the <code>Rule</code>, and the ID of the <code>Rule</code>.</p>
    #[serde(rename = "rules")]
    pub rules: Vec<ActivatedRule>,
    /// <p>Tha Amazon Resource Name (ARN) of the web ACL.</p>
    #[serde(rename = "webACLArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_arn: Option<String>,
    /// <p>A unique identifier for a <code>WebACL</code>. You use <code>WebACLId</code> to get information about a <code>WebACL</code> (see <a>GetWebACL</a>), update a <code>WebACL</code> (see <a>UpdateWebACL</a>), and delete a <code>WebACL</code> from AWS WAF (see <a>DeleteWebACL</a>).</p> <p> <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>
    #[serde(rename = "webACLId")]
    pub web_acl_id: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Contains the identifier and the name or description of the <a>WebACL</a>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct WebACLSummary {
    /// <p>A friendly name or description of the <a>WebACL</a>. You can't change the name of a <code>WebACL</code> after you create it.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A unique identifier for a <code>WebACL</code>. You use <code>WebACLId</code> to get information about a <code>WebACL</code> (see <a>GetWebACL</a>), update a <code>WebACL</code> (see <a>UpdateWebACL</a>), and delete a <code>WebACL</code> from AWS WAF (see <a>DeleteWebACL</a>).</p> <p> <code>WebACLId</code> is returned by <a>CreateWebACL</a> and by <a>ListWebACLs</a>.</p>
    #[serde(rename = "webACLId")]
    pub web_acl_id: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Specifies whether to insert a <code>Rule</code> into or delete a <code>Rule</code> from a <code>WebACL</code>.</p></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct WebACLUpdate {
    /// <p>Specifies whether to insert a <code>Rule</code> into or delete a <code>Rule</code> from a <code>WebACL</code>.</p>
    #[serde(rename = "action")]
    pub action: String,
    /// <p>The <code>ActivatedRule</code> object in an <a>UpdateWebACL</a> request specifies a <code>Rule</code> that you want to insert or delete, the priority of the <code>Rule</code> in the <code>WebACL</code>, and the action that you want AWS WAF to take when a web request matches the <code>Rule</code> (<code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>).</p>
    #[serde(rename = "activatedRule")]
    pub activated_rule: ActivatedRule,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>A complex type that contains <code>XssMatchTuple</code> objects, which specify the parts of web requests that you want AWS WAF to inspect for cross-site scripting attacks and, if you want AWS WAF to inspect a header, the name of the header. If a <code>XssMatchSet</code> contains more than one <code>XssMatchTuple</code> object, a request needs to include cross-site scripting attacks in only one of the specified parts of the request to be considered a match.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct XssMatchSet {
    /// <p>The name, if any, of the <code>XssMatchSet</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier for an <code>XssMatchSet</code>. You use <code>XssMatchSetId</code> to get information about an <code>XssMatchSet</code> (see <a>GetXssMatchSet</a>), update an <code>XssMatchSet</code> (see <a>UpdateXssMatchSet</a>), insert an <code>XssMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete an <code>XssMatchSet</code> from AWS WAF (see <a>DeleteXssMatchSet</a>).</p> <p> <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>
    #[serde(rename = "xssMatchSetId")]
    pub xss_match_set_id: String,
    /// <p>Specifies the parts of web requests that you want to inspect for cross-site scripting attacks.</p>
    #[serde(rename = "xssMatchTuples")]
    pub xss_match_tuples: Vec<XssMatchTuple>,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>The <code>Id</code> and <code>Name</code> of an <code>XssMatchSet</code>.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct XssMatchSetSummary {
    /// <p>The name of the <code>XssMatchSet</code>, if any, specified by <code>Id</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A unique identifier for an <code>XssMatchSet</code>. You use <code>XssMatchSetId</code> to get information about a <code>XssMatchSet</code> (see <a>GetXssMatchSet</a>), update an <code>XssMatchSet</code> (see <a>UpdateXssMatchSet</a>), insert an <code>XssMatchSet</code> into a <code>Rule</code> or delete one from a <code>Rule</code> (see <a>UpdateRule</a>), and delete an <code>XssMatchSet</code> from AWS WAF (see <a>DeleteXssMatchSet</a>).</p> <p> <code>XssMatchSetId</code> is returned by <a>CreateXssMatchSet</a> and by <a>ListXssMatchSets</a>.</p>
    #[serde(rename = "xssMatchSetId")]
    pub xss_match_set_id: String,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Specifies the part of a web request that you want to inspect for cross-site scripting attacks and indicates whether you want to add the specification to an <a>XssMatchSet</a> or delete it from an <code>XssMatchSet</code>.</p></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct XssMatchSetUpdate {
    /// <p>Specify <code>INSERT</code> to add an <a>XssMatchSetUpdate</a> to an <a>XssMatchSet</a>. Use <code>DELETE</code> to remove an <code>XssMatchSetUpdate</code> from an <code>XssMatchSet</code>.</p>
    #[serde(rename = "action")]
    pub action: String,
    /// <p>Specifies the part of a web request that you want AWS WAF to inspect for cross-site scripting attacks and, if you want AWS WAF to inspect a header, the name of the header.</p>
    #[serde(rename = "xssMatchTuple")]
    pub xss_match_tuple: XssMatchTuple,
}

/// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Specifies the part of a web request that you want AWS WAF to inspect for cross-site scripting attacks and, if you want AWS WAF to inspect a header, the name of the header.</p></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct XssMatchTuple {
    /// <p>Specifies where in a web request to look for cross-site scripting attacks.</p>
    #[serde(rename = "fieldToMatch")]
    pub field_to_match: FieldToMatch,
    /// <p>Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass AWS WAF. If you specify a transformation, AWS WAF performs the transformation on <code>FieldToMatch</code> before inspecting it for a match.</p> <p>You can only specify a single type of TextTransformation.</p> <p> <b>CMD_LINE</b> </p> <p>When you're concerned that attackers are injecting an operating system command line command and using unusual formatting to disguise some or all of the command, use this option to perform the following transformations:</p> <ul> <li> <p>Delete the following characters: \ " ' ^</p> </li> <li> <p>Delete spaces before the following characters: / (</p> </li> <li> <p>Replace the following characters with a space: , ;</p> </li> <li> <p>Replace multiple spaces with one space</p> </li> <li> <p>Convert uppercase letters (A-Z) to lowercase (a-z)</p> </li> </ul> <p> <b>COMPRESS_WHITE_SPACE</b> </p> <p>Use this option to replace the following characters with a space character (decimal 32):</p> <ul> <li> <p>\f, formfeed, decimal 12</p> </li> <li> <p>\t, tab, decimal 9</p> </li> <li> <p>\n, newline, decimal 10</p> </li> <li> <p>\r, carriage return, decimal 13</p> </li> <li> <p>\v, vertical tab, decimal 11</p> </li> <li> <p>non-breaking space, decimal 160</p> </li> </ul> <p> <code>COMPRESS_WHITE_SPACE</code> also replaces multiple spaces with one space.</p> <p> <b>HTML_ENTITY_DECODE</b> </p> <p>Use this option to replace HTML-encoded characters with unencoded characters. <code>HTML_ENTITY_DECODE</code> performs the following operations:</p> <ul> <li> <p>Replaces <code>(ampersand)quot;</code> with <code>"</code> </p> </li> <li> <p>Replaces <code>(ampersand)nbsp;</code> with a non-breaking space, decimal 160</p> </li> <li> <p>Replaces <code>(ampersand)lt;</code> with a "less than" symbol</p> </li> <li> <p>Replaces <code>(ampersand)gt;</code> with <code>&gt;</code> </p> </li> <li> <p>Replaces characters that are represented in hexadecimal format, <code>(ampersand)#xhhhh;</code>, with the corresponding characters</p> </li> <li> <p>Replaces characters that are represented in decimal format, <code>(ampersand)#nnnn;</code>, with the corresponding characters</p> </li> </ul> <p> <b>LOWERCASE</b> </p> <p>Use this option to convert uppercase letters (A-Z) to lowercase (a-z).</p> <p> <b>URL_DECODE</b> </p> <p>Use this option to decode a URL-encoded value.</p> <p> <b>NONE</b> </p> <p>Specify <code>NONE</code> if you don't want to perform any text transformations.</p>
    #[serde(rename = "textTransformation")]
    pub text_transformation: String,
}

/// Errors returned by CreateByteMatchSet
#[derive(Debug, PartialEq)]
pub enum CreateByteMatchSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateByteMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateByteMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateByteMatchSetError::WAFDisallowedName(
                        err.msg,
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateByteMatchSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(CreateByteMatchSetError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateByteMatchSetError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateByteMatchSetError::WAFLimitsExceeded(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateByteMatchSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateByteMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateByteMatchSetError::WAFDisallowedName(ref cause) => write!(f, "{}", cause),
            CreateByteMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateByteMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            CreateByteMatchSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateByteMatchSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateByteMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateByteMatchSetError {}
/// Errors returned by CreateGeoMatchSet
#[derive(Debug, PartialEq)]
pub enum CreateGeoMatchSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateGeoMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGeoMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateGeoMatchSetError::WAFDisallowedName(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateGeoMatchSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(CreateGeoMatchSetError::WAFInvalidAccount(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateGeoMatchSetError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateGeoMatchSetError::WAFLimitsExceeded(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateGeoMatchSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateGeoMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGeoMatchSetError::WAFDisallowedName(ref cause) => write!(f, "{}", cause),
            CreateGeoMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateGeoMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            CreateGeoMatchSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateGeoMatchSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateGeoMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGeoMatchSetError {}
/// Errors returned by CreateIPSet
#[derive(Debug, PartialEq)]
pub enum CreateIPSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateIPSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateIPSetError::WAFDisallowedName(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateIPSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(CreateIPSetError::WAFInvalidAccount(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateIPSetError::WAFInvalidParameter(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateIPSetError::WAFLimitsExceeded(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateIPSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateIPSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateIPSetError::WAFDisallowedName(ref cause) => write!(f, "{}", cause),
            CreateIPSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateIPSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            CreateIPSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateIPSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateIPSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateIPSetError {}
/// Errors returned by CreateRateBasedRule
#[derive(Debug, PartialEq)]
pub enum CreateRateBasedRuleError {
    /// <p><p/></p>
    WAFBadRequest(String),
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// <p><p/></p>
    WAFTagOperation(String),
    /// <p><p/></p>
    WAFTagOperationInternalError(String),
}

impl CreateRateBasedRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRateBasedRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFBadRequestException" => {
                    return RusotoError::Service(CreateRateBasedRuleError::WAFBadRequest(err.msg))
                }
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateRateBasedRuleError::WAFDisallowedName(
                        err.msg,
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateRateBasedRuleError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateRateBasedRuleError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateRateBasedRuleError::WAFLimitsExceeded(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateRateBasedRuleError::WAFStaleData(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(CreateRateBasedRuleError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(
                        CreateRateBasedRuleError::WAFTagOperationInternalError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRateBasedRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRateBasedRuleError::WAFBadRequest(ref cause) => write!(f, "{}", cause),
            CreateRateBasedRuleError::WAFDisallowedName(ref cause) => write!(f, "{}", cause),
            CreateRateBasedRuleError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateRateBasedRuleError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateRateBasedRuleError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateRateBasedRuleError::WAFStaleData(ref cause) => write!(f, "{}", cause),
            CreateRateBasedRuleError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            CreateRateBasedRuleError::WAFTagOperationInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateRateBasedRuleError {}
/// Errors returned by CreateRegexMatchSet
#[derive(Debug, PartialEq)]
pub enum CreateRegexMatchSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateRegexMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRegexMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateRegexMatchSetError::WAFDisallowedName(
                        err.msg,
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateRegexMatchSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateRegexMatchSetError::WAFLimitsExceeded(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateRegexMatchSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRegexMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRegexMatchSetError::WAFDisallowedName(ref cause) => write!(f, "{}", cause),
            CreateRegexMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateRegexMatchSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateRegexMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRegexMatchSetError {}
/// Errors returned by CreateRegexPatternSet
#[derive(Debug, PartialEq)]
pub enum CreateRegexPatternSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateRegexPatternSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRegexPatternSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateRegexPatternSetError::WAFDisallowedName(
                        err.msg,
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateRegexPatternSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateRegexPatternSetError::WAFLimitsExceeded(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateRegexPatternSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRegexPatternSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRegexPatternSetError::WAFDisallowedName(ref cause) => write!(f, "{}", cause),
            CreateRegexPatternSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateRegexPatternSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateRegexPatternSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRegexPatternSetError {}
/// Errors returned by CreateRule
#[derive(Debug, PartialEq)]
pub enum CreateRuleError {
    /// <p><p/></p>
    WAFBadRequest(String),
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// <p><p/></p>
    WAFTagOperation(String),
    /// <p><p/></p>
    WAFTagOperationInternalError(String),
}

impl CreateRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFBadRequestException" => {
                    return RusotoError::Service(CreateRuleError::WAFBadRequest(err.msg))
                }
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateRuleError::WAFDisallowedName(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateRuleError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateRuleError::WAFInvalidParameter(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateRuleError::WAFLimitsExceeded(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateRuleError::WAFStaleData(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(CreateRuleError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(CreateRuleError::WAFTagOperationInternalError(
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
impl fmt::Display for CreateRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRuleError::WAFBadRequest(ref cause) => write!(f, "{}", cause),
            CreateRuleError::WAFDisallowedName(ref cause) => write!(f, "{}", cause),
            CreateRuleError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateRuleError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateRuleError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateRuleError::WAFStaleData(ref cause) => write!(f, "{}", cause),
            CreateRuleError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            CreateRuleError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRuleError {}
/// Errors returned by CreateRuleGroup
#[derive(Debug, PartialEq)]
pub enum CreateRuleGroupError {
    /// <p><p/></p>
    WAFBadRequest(String),
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// <p><p/></p>
    WAFTagOperation(String),
    /// <p><p/></p>
    WAFTagOperationInternalError(String),
}

impl CreateRuleGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRuleGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFBadRequestException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFBadRequest(err.msg))
                }
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFDisallowedName(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFInternalError(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFLimitsExceeded(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFStaleData(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(CreateRuleGroupError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(
                        CreateRuleGroupError::WAFTagOperationInternalError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateRuleGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateRuleGroupError::WAFBadRequest(ref cause) => write!(f, "{}", cause),
            CreateRuleGroupError::WAFDisallowedName(ref cause) => write!(f, "{}", cause),
            CreateRuleGroupError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateRuleGroupError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateRuleGroupError::WAFStaleData(ref cause) => write!(f, "{}", cause),
            CreateRuleGroupError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            CreateRuleGroupError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateRuleGroupError {}
/// Errors returned by CreateSizeConstraintSet
#[derive(Debug, PartialEq)]
pub enum CreateSizeConstraintSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateSizeConstraintSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSizeConstraintSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateSizeConstraintSetError::WAFDisallowedName(
                        err.msg,
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateSizeConstraintSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(CreateSizeConstraintSetError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateSizeConstraintSetError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateSizeConstraintSetError::WAFLimitsExceeded(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateSizeConstraintSetError::WAFStaleData(
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
impl fmt::Display for CreateSizeConstraintSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSizeConstraintSetError::WAFDisallowedName(ref cause) => write!(f, "{}", cause),
            CreateSizeConstraintSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateSizeConstraintSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            CreateSizeConstraintSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateSizeConstraintSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateSizeConstraintSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSizeConstraintSetError {}
/// Errors returned by CreateSqlInjectionMatchSet
#[derive(Debug, PartialEq)]
pub enum CreateSqlInjectionMatchSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateSqlInjectionMatchSetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateSqlInjectionMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(
                        CreateSqlInjectionMatchSetError::WAFDisallowedName(err.msg),
                    )
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateSqlInjectionMatchSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(
                        CreateSqlInjectionMatchSetError::WAFInvalidAccount(err.msg),
                    )
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        CreateSqlInjectionMatchSetError::WAFInvalidParameter(err.msg),
                    )
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(
                        CreateSqlInjectionMatchSetError::WAFLimitsExceeded(err.msg),
                    )
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateSqlInjectionMatchSetError::WAFStaleData(
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
impl fmt::Display for CreateSqlInjectionMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSqlInjectionMatchSetError::WAFDisallowedName(ref cause) => write!(f, "{}", cause),
            CreateSqlInjectionMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateSqlInjectionMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            CreateSqlInjectionMatchSetError::WAFInvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateSqlInjectionMatchSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateSqlInjectionMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSqlInjectionMatchSetError {}
/// Errors returned by CreateWebACL
#[derive(Debug, PartialEq)]
pub enum CreateWebACLError {
    /// <p><p/></p>
    WAFBadRequest(String),
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// <p><p/></p>
    WAFTagOperation(String),
    /// <p><p/></p>
    WAFTagOperationInternalError(String),
}

impl CreateWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateWebACLError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFBadRequestException" => {
                    return RusotoError::Service(CreateWebACLError::WAFBadRequest(err.msg))
                }
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateWebACLError::WAFDisallowedName(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateWebACLError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(CreateWebACLError::WAFInvalidAccount(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateWebACLError::WAFInvalidParameter(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateWebACLError::WAFLimitsExceeded(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateWebACLError::WAFStaleData(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(CreateWebACLError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(CreateWebACLError::WAFTagOperationInternalError(
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
impl fmt::Display for CreateWebACLError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateWebACLError::WAFBadRequest(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFDisallowedName(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFStaleData(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            CreateWebACLError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateWebACLError {}
/// Errors returned by CreateWebACLMigrationStack
#[derive(Debug, PartialEq)]
pub enum CreateWebACLMigrationStackError {
    /// <p><p>The operation failed due to a problem with the migration. The failure cause is provided in the exception, in the <code>MigrationErrorType</code>: </p> <ul> <li> <p> <code>ENTITY<em>NOT</em>SUPPORTED</code> - The web ACL has an unsupported entity but the <code>IgnoreUnsupportedType</code> is not set to true.</p> </li> <li> <p> <code>ENTITY<em>NOT</em>FOUND</code> - The web ACL doesn&#39;t exist. </p> </li> <li> <p> <code>S3<em>BUCKET</em>NO<em>PERMISSION</code> - You don&#39;t have permission to perform the <code>PutObject</code> action to the specified Amazon S3 bucket.</p> </li> <li> <p> <code>S3</em>BUCKET<em>NOT</em>ACCESSIBLE</code> - The bucket policy doesn&#39;t allow AWS WAF to perform the <code>PutObject</code> action in the bucket.</p> </li> <li> <p> <code>S3<em>BUCKET</em>NOT<em>FOUND</code> - The S3 bucket doesn&#39;t exist. </p> </li> <li> <p> <code>S3</em>BUCKET<em>INVALID</em>REGION</code> - The S3 bucket is not in the same Region as the web ACL.</p> </li> <li> <p> <code>S3<em>INTERNAL</em>ERROR</code> - AWS WAF failed to create the template in the S3 bucket for another reason.</p> </li> </ul></p>
    WAFEntityMigration(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl CreateWebACLMigrationStackError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateWebACLMigrationStackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFEntityMigrationException" => {
                    return RusotoError::Service(
                        CreateWebACLMigrationStackError::WAFEntityMigration(err.msg),
                    )
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateWebACLMigrationStackError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(
                        CreateWebACLMigrationStackError::WAFInvalidOperation(err.msg),
                    )
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        CreateWebACLMigrationStackError::WAFInvalidParameter(err.msg),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        CreateWebACLMigrationStackError::WAFNonexistentItem(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateWebACLMigrationStackError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateWebACLMigrationStackError::WAFEntityMigration(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateWebACLMigrationStackError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateWebACLMigrationStackError::WAFInvalidOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateWebACLMigrationStackError::WAFInvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateWebACLMigrationStackError::WAFNonexistentItem(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateWebACLMigrationStackError {}
/// Errors returned by CreateXssMatchSet
#[derive(Debug, PartialEq)]
pub enum CreateXssMatchSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl CreateXssMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateXssMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(CreateXssMatchSetError::WAFDisallowedName(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(CreateXssMatchSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(CreateXssMatchSetError::WAFInvalidAccount(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(CreateXssMatchSetError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(CreateXssMatchSetError::WAFLimitsExceeded(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(CreateXssMatchSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateXssMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateXssMatchSetError::WAFDisallowedName(ref cause) => write!(f, "{}", cause),
            CreateXssMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            CreateXssMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            CreateXssMatchSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateXssMatchSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            CreateXssMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateXssMatchSetError {}
/// Errors returned by DeleteByteMatchSet
#[derive(Debug, PartialEq)]
pub enum DeleteByteMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteByteMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteByteMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteByteMatchSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteByteMatchSetError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteByteMatchSetError::WAFNonEmptyEntity(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteByteMatchSetError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteByteMatchSetError::WAFReferencedItem(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteByteMatchSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteByteMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteByteMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteByteMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            DeleteByteMatchSetError::WAFNonEmptyEntity(ref cause) => write!(f, "{}", cause),
            DeleteByteMatchSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteByteMatchSetError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            DeleteByteMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteByteMatchSetError {}
/// Errors returned by DeleteGeoMatchSet
#[derive(Debug, PartialEq)]
pub enum DeleteGeoMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteGeoMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGeoMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteGeoMatchSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteGeoMatchSetError::WAFInvalidAccount(err.msg))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteGeoMatchSetError::WAFNonEmptyEntity(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteGeoMatchSetError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteGeoMatchSetError::WAFReferencedItem(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteGeoMatchSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteGeoMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGeoMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteGeoMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            DeleteGeoMatchSetError::WAFNonEmptyEntity(ref cause) => write!(f, "{}", cause),
            DeleteGeoMatchSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteGeoMatchSetError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            DeleteGeoMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGeoMatchSetError {}
/// Errors returned by DeleteIPSet
#[derive(Debug, PartialEq)]
pub enum DeleteIPSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIPSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFInvalidAccount(err.msg))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFNonEmptyEntity(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFNonexistentItem(err.msg))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFReferencedItem(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteIPSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteIPSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteIPSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteIPSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            DeleteIPSetError::WAFNonEmptyEntity(ref cause) => write!(f, "{}", cause),
            DeleteIPSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteIPSetError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            DeleteIPSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteIPSetError {}
/// Errors returned by DeleteLoggingConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteLoggingConfigurationError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteLoggingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteLoggingConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteLoggingConfigurationError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        DeleteLoggingConfigurationError::WAFNonexistentItem(err.msg),
                    )
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteLoggingConfigurationError::WAFStaleData(
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
impl fmt::Display for DeleteLoggingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLoggingConfigurationError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteLoggingConfigurationError::WAFNonexistentItem(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteLoggingConfigurationError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLoggingConfigurationError {}
/// Errors returned by DeletePermissionPolicy
#[derive(Debug, PartialEq)]
pub enum DeletePermissionPolicyError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeletePermissionPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePermissionPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeletePermissionPolicyError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeletePermissionPolicyError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeletePermissionPolicyError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePermissionPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePermissionPolicyError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeletePermissionPolicyError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeletePermissionPolicyError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePermissionPolicyError {}
/// Errors returned by DeleteRateBasedRule
#[derive(Debug, PartialEq)]
pub enum DeleteRateBasedRuleError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// <p><p/></p>
    WAFTagOperation(String),
    /// <p><p/></p>
    WAFTagOperationInternalError(String),
}

impl DeleteRateBasedRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRateBasedRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteRateBasedRuleError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteRateBasedRuleError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteRateBasedRuleError::WAFNonEmptyEntity(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteRateBasedRuleError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteRateBasedRuleError::WAFReferencedItem(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteRateBasedRuleError::WAFStaleData(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(DeleteRateBasedRuleError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(
                        DeleteRateBasedRuleError::WAFTagOperationInternalError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRateBasedRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRateBasedRuleError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteRateBasedRuleError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            DeleteRateBasedRuleError::WAFNonEmptyEntity(ref cause) => write!(f, "{}", cause),
            DeleteRateBasedRuleError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteRateBasedRuleError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            DeleteRateBasedRuleError::WAFStaleData(ref cause) => write!(f, "{}", cause),
            DeleteRateBasedRuleError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            DeleteRateBasedRuleError::WAFTagOperationInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteRateBasedRuleError {}
/// Errors returned by DeleteRegexMatchSet
#[derive(Debug, PartialEq)]
pub enum DeleteRegexMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteRegexMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRegexMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteRegexMatchSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteRegexMatchSetError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteRegexMatchSetError::WAFNonEmptyEntity(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteRegexMatchSetError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteRegexMatchSetError::WAFReferencedItem(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteRegexMatchSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRegexMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRegexMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteRegexMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            DeleteRegexMatchSetError::WAFNonEmptyEntity(ref cause) => write!(f, "{}", cause),
            DeleteRegexMatchSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteRegexMatchSetError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            DeleteRegexMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRegexMatchSetError {}
/// Errors returned by DeleteRegexPatternSet
#[derive(Debug, PartialEq)]
pub enum DeleteRegexPatternSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteRegexPatternSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRegexPatternSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFNonEmptyEntity(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFReferencedItem(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteRegexPatternSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRegexPatternSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRegexPatternSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteRegexPatternSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            DeleteRegexPatternSetError::WAFNonEmptyEntity(ref cause) => write!(f, "{}", cause),
            DeleteRegexPatternSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteRegexPatternSetError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            DeleteRegexPatternSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRegexPatternSetError {}
/// Errors returned by DeleteRule
#[derive(Debug, PartialEq)]
pub enum DeleteRuleError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// <p><p/></p>
    WAFTagOperation(String),
    /// <p><p/></p>
    WAFTagOperationInternalError(String),
}

impl DeleteRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteRuleError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteRuleError::WAFInvalidAccount(err.msg))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteRuleError::WAFNonEmptyEntity(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteRuleError::WAFNonexistentItem(err.msg))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteRuleError::WAFReferencedItem(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteRuleError::WAFStaleData(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(DeleteRuleError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(DeleteRuleError::WAFTagOperationInternalError(
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
impl fmt::Display for DeleteRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRuleError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::WAFNonEmptyEntity(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::WAFStaleData(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            DeleteRuleError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRuleError {}
/// Errors returned by DeleteRuleGroup
#[derive(Debug, PartialEq)]
pub enum DeleteRuleGroupError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// <p><p/></p>
    WAFTagOperation(String),
    /// <p><p/></p>
    WAFTagOperationInternalError(String),
}

impl DeleteRuleGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRuleGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFInternalError(err.msg))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFInvalidOperation(err.msg))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFNonEmptyEntity(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFNonexistentItem(err.msg))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFReferencedItem(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFStaleData(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(DeleteRuleGroupError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(
                        DeleteRuleGroupError::WAFTagOperationInternalError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteRuleGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteRuleGroupError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteRuleGroupError::WAFInvalidOperation(ref cause) => write!(f, "{}", cause),
            DeleteRuleGroupError::WAFNonEmptyEntity(ref cause) => write!(f, "{}", cause),
            DeleteRuleGroupError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteRuleGroupError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            DeleteRuleGroupError::WAFStaleData(ref cause) => write!(f, "{}", cause),
            DeleteRuleGroupError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            DeleteRuleGroupError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteRuleGroupError {}
/// Errors returned by DeleteSizeConstraintSet
#[derive(Debug, PartialEq)]
pub enum DeleteSizeConstraintSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteSizeConstraintSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSizeConstraintSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteSizeConstraintSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteSizeConstraintSetError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteSizeConstraintSetError::WAFNonEmptyEntity(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteSizeConstraintSetError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteSizeConstraintSetError::WAFReferencedItem(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteSizeConstraintSetError::WAFStaleData(
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
impl fmt::Display for DeleteSizeConstraintSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSizeConstraintSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteSizeConstraintSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            DeleteSizeConstraintSetError::WAFNonEmptyEntity(ref cause) => write!(f, "{}", cause),
            DeleteSizeConstraintSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteSizeConstraintSetError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            DeleteSizeConstraintSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSizeConstraintSetError {}
/// Errors returned by DeleteSqlInjectionMatchSet
#[derive(Debug, PartialEq)]
pub enum DeleteSqlInjectionMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteSqlInjectionMatchSetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteSqlInjectionMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteSqlInjectionMatchSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(
                        DeleteSqlInjectionMatchSetError::WAFInvalidAccount(err.msg),
                    )
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(
                        DeleteSqlInjectionMatchSetError::WAFNonEmptyEntity(err.msg),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        DeleteSqlInjectionMatchSetError::WAFNonexistentItem(err.msg),
                    )
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(
                        DeleteSqlInjectionMatchSetError::WAFReferencedItem(err.msg),
                    )
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteSqlInjectionMatchSetError::WAFStaleData(
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
impl fmt::Display for DeleteSqlInjectionMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSqlInjectionMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteSqlInjectionMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            DeleteSqlInjectionMatchSetError::WAFNonEmptyEntity(ref cause) => write!(f, "{}", cause),
            DeleteSqlInjectionMatchSetError::WAFNonexistentItem(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteSqlInjectionMatchSetError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            DeleteSqlInjectionMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSqlInjectionMatchSetError {}
/// Errors returned by DeleteWebACL
#[derive(Debug, PartialEq)]
pub enum DeleteWebACLError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// <p><p/></p>
    WAFTagOperation(String),
    /// <p><p/></p>
    WAFTagOperationInternalError(String),
}

impl DeleteWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteWebACLError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFInvalidAccount(err.msg))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFNonEmptyEntity(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFNonexistentItem(err.msg))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFReferencedItem(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFStaleData(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(DeleteWebACLError::WAFTagOperationInternalError(
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
impl fmt::Display for DeleteWebACLError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteWebACLError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteWebACLError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            DeleteWebACLError::WAFNonEmptyEntity(ref cause) => write!(f, "{}", cause),
            DeleteWebACLError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteWebACLError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            DeleteWebACLError::WAFStaleData(ref cause) => write!(f, "{}", cause),
            DeleteWebACLError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            DeleteWebACLError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteWebACLError {}
/// Errors returned by DeleteXssMatchSet
#[derive(Debug, PartialEq)]
pub enum DeleteXssMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because you tried to delete an object that isn&#39;t empty. For example:</p> <ul> <li> <p>You tried to delete a <code>WebACL</code> that still contains one or more <code>Rule</code> objects.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that still contains one or more <code>ByteMatchSet</code> objects or other predicates.</p> </li> <li> <p>You tried to delete a <code>ByteMatchSet</code> that contains one or more <code>ByteMatchTuple</code> objects.</p> </li> <li> <p>You tried to delete an <code>IPSet</code> that references one or more IP addresses.</p> </li> </ul></p>
    WAFNonEmptyEntity(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl DeleteXssMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteXssMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(DeleteXssMatchSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(DeleteXssMatchSetError::WAFInvalidAccount(err.msg))
                }
                "WAFNonEmptyEntityException" => {
                    return RusotoError::Service(DeleteXssMatchSetError::WAFNonEmptyEntity(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(DeleteXssMatchSetError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(DeleteXssMatchSetError::WAFReferencedItem(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(DeleteXssMatchSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteXssMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteXssMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            DeleteXssMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            DeleteXssMatchSetError::WAFNonEmptyEntity(ref cause) => write!(f, "{}", cause),
            DeleteXssMatchSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            DeleteXssMatchSetError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            DeleteXssMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteXssMatchSetError {}
/// Errors returned by GetByteMatchSet
#[derive(Debug, PartialEq)]
pub enum GetByteMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetByteMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetByteMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetByteMatchSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetByteMatchSetError::WAFInvalidAccount(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetByteMatchSetError::WAFNonexistentItem(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetByteMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetByteMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetByteMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            GetByteMatchSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetByteMatchSetError {}
/// Errors returned by GetChangeToken
#[derive(Debug, PartialEq)]
pub enum GetChangeTokenError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
}

impl GetChangeTokenError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetChangeTokenError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetChangeTokenError::WAFInternalError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetChangeTokenError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetChangeTokenError::WAFInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetChangeTokenError {}
/// Errors returned by GetChangeTokenStatus
#[derive(Debug, PartialEq)]
pub enum GetChangeTokenStatusError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetChangeTokenStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetChangeTokenStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetChangeTokenStatusError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetChangeTokenStatusError::WAFNonexistentItem(
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
impl fmt::Display for GetChangeTokenStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetChangeTokenStatusError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetChangeTokenStatusError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetChangeTokenStatusError {}
/// Errors returned by GetGeoMatchSet
#[derive(Debug, PartialEq)]
pub enum GetGeoMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetGeoMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGeoMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetGeoMatchSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetGeoMatchSetError::WAFInvalidAccount(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetGeoMatchSetError::WAFNonexistentItem(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetGeoMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetGeoMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetGeoMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            GetGeoMatchSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetGeoMatchSetError {}
/// Errors returned by GetIPSet
#[derive(Debug, PartialEq)]
pub enum GetIPSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIPSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetIPSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetIPSetError::WAFInvalidAccount(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetIPSetError::WAFNonexistentItem(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetIPSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetIPSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetIPSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            GetIPSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetIPSetError {}
/// Errors returned by GetLoggingConfiguration
#[derive(Debug, PartialEq)]
pub enum GetLoggingConfigurationError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetLoggingConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLoggingConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetLoggingConfigurationError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetLoggingConfigurationError::WAFNonexistentItem(
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
impl fmt::Display for GetLoggingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLoggingConfigurationError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetLoggingConfigurationError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLoggingConfigurationError {}
/// Errors returned by GetPermissionPolicy
#[derive(Debug, PartialEq)]
pub enum GetPermissionPolicyError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetPermissionPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPermissionPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetPermissionPolicyError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetPermissionPolicyError::WAFNonexistentItem(
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
impl fmt::Display for GetPermissionPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPermissionPolicyError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetPermissionPolicyError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPermissionPolicyError {}
/// Errors returned by GetRateBasedRule
#[derive(Debug, PartialEq)]
pub enum GetRateBasedRuleError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetRateBasedRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRateBasedRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetRateBasedRuleError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetRateBasedRuleError::WAFInvalidAccount(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetRateBasedRuleError::WAFNonexistentItem(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRateBasedRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRateBasedRuleError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetRateBasedRuleError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            GetRateBasedRuleError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRateBasedRuleError {}
/// Errors returned by GetRateBasedRuleManagedKeys
#[derive(Debug, PartialEq)]
pub enum GetRateBasedRuleManagedKeysError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetRateBasedRuleManagedKeysError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRateBasedRuleManagedKeysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(
                        GetRateBasedRuleManagedKeysError::WAFInternalError(err.msg),
                    )
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(
                        GetRateBasedRuleManagedKeysError::WAFInvalidAccount(err.msg),
                    )
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        GetRateBasedRuleManagedKeysError::WAFInvalidParameter(err.msg),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        GetRateBasedRuleManagedKeysError::WAFNonexistentItem(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRateBasedRuleManagedKeysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRateBasedRuleManagedKeysError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetRateBasedRuleManagedKeysError::WAFInvalidAccount(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRateBasedRuleManagedKeysError::WAFInvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetRateBasedRuleManagedKeysError::WAFNonexistentItem(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetRateBasedRuleManagedKeysError {}
/// Errors returned by GetRegexMatchSet
#[derive(Debug, PartialEq)]
pub enum GetRegexMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetRegexMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRegexMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetRegexMatchSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetRegexMatchSetError::WAFInvalidAccount(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetRegexMatchSetError::WAFNonexistentItem(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRegexMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRegexMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetRegexMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            GetRegexMatchSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRegexMatchSetError {}
/// Errors returned by GetRegexPatternSet
#[derive(Debug, PartialEq)]
pub enum GetRegexPatternSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetRegexPatternSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRegexPatternSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetRegexPatternSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetRegexPatternSetError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetRegexPatternSetError::WAFNonexistentItem(
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
impl fmt::Display for GetRegexPatternSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRegexPatternSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetRegexPatternSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            GetRegexPatternSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRegexPatternSetError {}
/// Errors returned by GetRule
#[derive(Debug, PartialEq)]
pub enum GetRuleError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetRuleError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetRuleError::WAFInvalidAccount(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetRuleError::WAFNonexistentItem(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRuleError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetRuleError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            GetRuleError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRuleError {}
/// Errors returned by GetRuleGroup
#[derive(Debug, PartialEq)]
pub enum GetRuleGroupError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetRuleGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRuleGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetRuleGroupError::WAFInternalError(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetRuleGroupError::WAFNonexistentItem(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetRuleGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRuleGroupError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetRuleGroupError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRuleGroupError {}
/// Errors returned by GetSampledRequests
#[derive(Debug, PartialEq)]
pub enum GetSampledRequestsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetSampledRequestsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSampledRequestsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetSampledRequestsError::WAFInternalError(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetSampledRequestsError::WAFNonexistentItem(
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
impl fmt::Display for GetSampledRequestsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSampledRequestsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetSampledRequestsError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSampledRequestsError {}
/// Errors returned by GetSizeConstraintSet
#[derive(Debug, PartialEq)]
pub enum GetSizeConstraintSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetSizeConstraintSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSizeConstraintSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetSizeConstraintSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetSizeConstraintSetError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetSizeConstraintSetError::WAFNonexistentItem(
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
impl fmt::Display for GetSizeConstraintSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSizeConstraintSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetSizeConstraintSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            GetSizeConstraintSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSizeConstraintSetError {}
/// Errors returned by GetSqlInjectionMatchSet
#[derive(Debug, PartialEq)]
pub enum GetSqlInjectionMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetSqlInjectionMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSqlInjectionMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetSqlInjectionMatchSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetSqlInjectionMatchSetError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetSqlInjectionMatchSetError::WAFNonexistentItem(
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
impl fmt::Display for GetSqlInjectionMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSqlInjectionMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetSqlInjectionMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            GetSqlInjectionMatchSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSqlInjectionMatchSetError {}
/// Errors returned by GetWebACL
#[derive(Debug, PartialEq)]
pub enum GetWebACLError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetWebACLError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetWebACLError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetWebACLError::WAFInvalidAccount(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetWebACLError::WAFNonexistentItem(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetWebACLError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetWebACLError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetWebACLError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            GetWebACLError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetWebACLError {}
/// Errors returned by GetXssMatchSet
#[derive(Debug, PartialEq)]
pub enum GetXssMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl GetXssMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetXssMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(GetXssMatchSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(GetXssMatchSetError::WAFInvalidAccount(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(GetXssMatchSetError::WAFNonexistentItem(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetXssMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetXssMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            GetXssMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            GetXssMatchSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetXssMatchSetError {}
/// Errors returned by ListActivatedRulesInRuleGroup
#[derive(Debug, PartialEq)]
pub enum ListActivatedRulesInRuleGroupError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl ListActivatedRulesInRuleGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListActivatedRulesInRuleGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(
                        ListActivatedRulesInRuleGroupError::WAFInternalError(err.msg),
                    )
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        ListActivatedRulesInRuleGroupError::WAFInvalidParameter(err.msg),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        ListActivatedRulesInRuleGroupError::WAFNonexistentItem(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListActivatedRulesInRuleGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListActivatedRulesInRuleGroupError::WAFInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListActivatedRulesInRuleGroupError::WAFInvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            ListActivatedRulesInRuleGroupError::WAFNonexistentItem(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListActivatedRulesInRuleGroupError {}
/// Errors returned by ListByteMatchSets
#[derive(Debug, PartialEq)]
pub enum ListByteMatchSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListByteMatchSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListByteMatchSetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListByteMatchSetsError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListByteMatchSetsError::WAFInvalidAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListByteMatchSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListByteMatchSetsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListByteMatchSetsError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListByteMatchSetsError {}
/// Errors returned by ListGeoMatchSets
#[derive(Debug, PartialEq)]
pub enum ListGeoMatchSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListGeoMatchSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGeoMatchSetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListGeoMatchSetsError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListGeoMatchSetsError::WAFInvalidAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListGeoMatchSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGeoMatchSetsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListGeoMatchSetsError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGeoMatchSetsError {}
/// Errors returned by ListIPSets
#[derive(Debug, PartialEq)]
pub enum ListIPSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListIPSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIPSetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListIPSetsError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListIPSetsError::WAFInvalidAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListIPSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListIPSetsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListIPSetsError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListIPSetsError {}
/// Errors returned by ListLoggingConfigurations
#[derive(Debug, PartialEq)]
pub enum ListLoggingConfigurationsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl ListLoggingConfigurationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLoggingConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListLoggingConfigurationsError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        ListLoggingConfigurationsError::WAFInvalidParameter(err.msg),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        ListLoggingConfigurationsError::WAFNonexistentItem(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLoggingConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLoggingConfigurationsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListLoggingConfigurationsError::WAFInvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            ListLoggingConfigurationsError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLoggingConfigurationsError {}
/// Errors returned by ListRateBasedRules
#[derive(Debug, PartialEq)]
pub enum ListRateBasedRulesError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListRateBasedRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRateBasedRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListRateBasedRulesError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListRateBasedRulesError::WAFInvalidAccount(
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
impl fmt::Display for ListRateBasedRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRateBasedRulesError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListRateBasedRulesError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRateBasedRulesError {}
/// Errors returned by ListRegexMatchSets
#[derive(Debug, PartialEq)]
pub enum ListRegexMatchSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListRegexMatchSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRegexMatchSetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListRegexMatchSetsError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListRegexMatchSetsError::WAFInvalidAccount(
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
impl fmt::Display for ListRegexMatchSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRegexMatchSetsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListRegexMatchSetsError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRegexMatchSetsError {}
/// Errors returned by ListRegexPatternSets
#[derive(Debug, PartialEq)]
pub enum ListRegexPatternSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListRegexPatternSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRegexPatternSetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListRegexPatternSetsError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListRegexPatternSetsError::WAFInvalidAccount(
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
impl fmt::Display for ListRegexPatternSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRegexPatternSetsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListRegexPatternSetsError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRegexPatternSetsError {}
/// Errors returned by ListRuleGroups
#[derive(Debug, PartialEq)]
pub enum ListRuleGroupsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
}

impl ListRuleGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRuleGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListRuleGroupsError::WAFInternalError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRuleGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRuleGroupsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRuleGroupsError {}
/// Errors returned by ListRules
#[derive(Debug, PartialEq)]
pub enum ListRulesError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListRulesError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListRulesError::WAFInvalidAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListRulesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRulesError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListRulesError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRulesError {}
/// Errors returned by ListSizeConstraintSets
#[derive(Debug, PartialEq)]
pub enum ListSizeConstraintSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListSizeConstraintSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSizeConstraintSetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListSizeConstraintSetsError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListSizeConstraintSetsError::WAFInvalidAccount(
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
impl fmt::Display for ListSizeConstraintSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSizeConstraintSetsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListSizeConstraintSetsError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSizeConstraintSetsError {}
/// Errors returned by ListSqlInjectionMatchSets
#[derive(Debug, PartialEq)]
pub enum ListSqlInjectionMatchSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListSqlInjectionMatchSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSqlInjectionMatchSetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListSqlInjectionMatchSetsError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListSqlInjectionMatchSetsError::WAFInvalidAccount(
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
impl fmt::Display for ListSqlInjectionMatchSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSqlInjectionMatchSetsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListSqlInjectionMatchSetsError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSqlInjectionMatchSetsError {}
/// Errors returned by ListSubscribedRuleGroups
#[derive(Debug, PartialEq)]
pub enum ListSubscribedRuleGroupsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
}

impl ListSubscribedRuleGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSubscribedRuleGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListSubscribedRuleGroupsError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(ListSubscribedRuleGroupsError::WAFNonexistentItem(
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
impl fmt::Display for ListSubscribedRuleGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSubscribedRuleGroupsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListSubscribedRuleGroupsError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSubscribedRuleGroupsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p><p/></p>
    WAFBadRequest(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p/></p>
    WAFTagOperation(String),
    /// <p><p/></p>
    WAFTagOperationInternalError(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFBadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::WAFBadRequest(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(ListTagsForResourceError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(ListTagsForResourceError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(
                        ListTagsForResourceError::WAFTagOperationInternalError(err.msg),
                    )
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
            ListTagsForResourceError::WAFBadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::WAFTagOperationInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListWebACLs
#[derive(Debug, PartialEq)]
pub enum ListWebACLsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListWebACLsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListWebACLsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListWebACLsError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListWebACLsError::WAFInvalidAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListWebACLsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListWebACLsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListWebACLsError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListWebACLsError {}
/// Errors returned by ListXssMatchSets
#[derive(Debug, PartialEq)]
pub enum ListXssMatchSetsError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
}

impl ListXssMatchSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListXssMatchSetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(ListXssMatchSetsError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(ListXssMatchSetsError::WAFInvalidAccount(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListXssMatchSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListXssMatchSetsError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            ListXssMatchSetsError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListXssMatchSetsError {}
/// Errors returned by PutLoggingConfiguration
#[derive(Debug, PartialEq)]
pub enum PutLoggingConfigurationError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>AWS WAF is not able to access the service linked role. This can be caused by a previous <code>PutLoggingConfiguration</code> request, which can lock the service linked role for about 20 seconds. Please try your request again. The service linked role can also be locked by a previous <code>DeleteServiceLinkedRole</code> request, which can lock the role for 15 minutes or more. If you recently made a <code>DeleteServiceLinkedRole</code>, wait at least 15 minutes and try the request again. If you receive this same exception again, you will have to wait additional time until the role is unlocked.</p>
    WAFServiceLinkedRoleError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl PutLoggingConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutLoggingConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(PutLoggingConfigurationError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(PutLoggingConfigurationError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFServiceLinkedRoleErrorException" => {
                    return RusotoError::Service(
                        PutLoggingConfigurationError::WAFServiceLinkedRoleError(err.msg),
                    )
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(PutLoggingConfigurationError::WAFStaleData(
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
impl fmt::Display for PutLoggingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutLoggingConfigurationError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            PutLoggingConfigurationError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            PutLoggingConfigurationError::WAFServiceLinkedRoleError(ref cause) => {
                write!(f, "{}", cause)
            }
            PutLoggingConfigurationError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutLoggingConfigurationError {}
/// Errors returned by PutPermissionPolicy
#[derive(Debug, PartialEq)]
pub enum PutPermissionPolicyError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because the specified policy is not in the proper format. </p> <p>The policy is subject to the following restrictions:</p> <ul> <li> <p>You can attach only one policy with each <code>PutPermissionPolicy</code> request.</p> </li> <li> <p>The policy must include an <code>Effect</code>, <code>Action</code> and <code>Principal</code>. </p> </li> <li> <p> <code>Effect</code> must specify <code>Allow</code>.</p> </li> <li> <p>The <code>Action</code> in the policy must be <code>waf:UpdateWebACL</code>, <code>waf-regional:UpdateWebACL</code>, <code>waf:GetRuleGroup</code> and <code>waf-regional:GetRuleGroup</code> . Any extra or wildcard actions in the policy will be rejected.</p> </li> <li> <p>The policy cannot include a <code>Resource</code> parameter.</p> </li> <li> <p>The ARN in the request must be a valid WAF RuleGroup ARN and the RuleGroup must exist in the same region.</p> </li> <li> <p>The user making the request must be the owner of the RuleGroup.</p> </li> <li> <p>Your policy must be composed using IAM Policy version 2012-10-17.</p> </li> </ul></p>
    WAFInvalidPermissionPolicy(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl PutPermissionPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutPermissionPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(PutPermissionPolicyError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidPermissionPolicyException" => {
                    return RusotoError::Service(
                        PutPermissionPolicyError::WAFInvalidPermissionPolicy(err.msg),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(PutPermissionPolicyError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(PutPermissionPolicyError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutPermissionPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutPermissionPolicyError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            PutPermissionPolicyError::WAFInvalidPermissionPolicy(ref cause) => {
                write!(f, "{}", cause)
            }
            PutPermissionPolicyError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            PutPermissionPolicyError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutPermissionPolicyError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p><p/></p>
    WAFBadRequest(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p/></p>
    WAFTagOperation(String),
    /// <p><p/></p>
    WAFTagOperationInternalError(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFBadRequestException" => {
                    return RusotoError::Service(TagResourceError::WAFBadRequest(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(TagResourceError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(TagResourceError::WAFInvalidParameter(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(TagResourceError::WAFLimitsExceeded(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(TagResourceError::WAFNonexistentItem(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(TagResourceError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(TagResourceError::WAFTagOperationInternalError(
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
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::WAFBadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            TagResourceError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            TagResourceError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            TagResourceError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p><p/></p>
    WAFBadRequest(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p/></p>
    WAFTagOperation(String),
    /// <p><p/></p>
    WAFTagOperationInternalError(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFBadRequestException" => {
                    return RusotoError::Service(UntagResourceError::WAFBadRequest(err.msg))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UntagResourceError::WAFInternalError(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UntagResourceError::WAFInvalidParameter(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UntagResourceError::WAFNonexistentItem(err.msg))
                }
                "WAFTagOperationException" => {
                    return RusotoError::Service(UntagResourceError::WAFTagOperation(err.msg))
                }
                "WAFTagOperationInternalErrorException" => {
                    return RusotoError::Service(UntagResourceError::WAFTagOperationInternalError(
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
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::WAFBadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UntagResourceError::WAFTagOperation(ref cause) => write!(f, "{}", cause),
            UntagResourceError::WAFTagOperationInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateByteMatchSet
#[derive(Debug, PartialEq)]
pub enum UpdateByteMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateByteMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateByteMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFInvalidOperation(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFLimitsExceeded(
                        err.msg,
                    ))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFNonexistentContainer(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateByteMatchSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateByteMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateByteMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateByteMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            UpdateByteMatchSetError::WAFInvalidOperation(ref cause) => write!(f, "{}", cause),
            UpdateByteMatchSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateByteMatchSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateByteMatchSetError::WAFNonexistentContainer(ref cause) => write!(f, "{}", cause),
            UpdateByteMatchSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateByteMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateByteMatchSetError {}
/// Errors returned by UpdateGeoMatchSet
#[derive(Debug, PartialEq)]
pub enum UpdateGeoMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateGeoMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGeoMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFInvalidAccount(err.msg))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFInvalidOperation(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFLimitsExceeded(err.msg))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFNonexistentContainer(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFReferencedItem(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateGeoMatchSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateGeoMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGeoMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateGeoMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            UpdateGeoMatchSetError::WAFInvalidOperation(ref cause) => write!(f, "{}", cause),
            UpdateGeoMatchSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateGeoMatchSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateGeoMatchSetError::WAFNonexistentContainer(ref cause) => write!(f, "{}", cause),
            UpdateGeoMatchSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateGeoMatchSetError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            UpdateGeoMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateGeoMatchSetError {}
/// Errors returned by UpdateIPSet
#[derive(Debug, PartialEq)]
pub enum UpdateIPSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateIPSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateIPSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFInvalidAccount(err.msg))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFInvalidOperation(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFInvalidParameter(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFLimitsExceeded(err.msg))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFNonexistentContainer(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFNonexistentItem(err.msg))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFReferencedItem(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateIPSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateIPSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateIPSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateIPSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            UpdateIPSetError::WAFInvalidOperation(ref cause) => write!(f, "{}", cause),
            UpdateIPSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateIPSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateIPSetError::WAFNonexistentContainer(ref cause) => write!(f, "{}", cause),
            UpdateIPSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateIPSetError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            UpdateIPSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateIPSetError {}
/// Errors returned by UpdateRateBasedRule
#[derive(Debug, PartialEq)]
pub enum UpdateRateBasedRuleError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateRateBasedRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRateBasedRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFInvalidOperation(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFLimitsExceeded(
                        err.msg,
                    ))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFNonexistentContainer(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFReferencedItem(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateRateBasedRuleError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRateBasedRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRateBasedRuleError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateRateBasedRuleError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            UpdateRateBasedRuleError::WAFInvalidOperation(ref cause) => write!(f, "{}", cause),
            UpdateRateBasedRuleError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateRateBasedRuleError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateRateBasedRuleError::WAFNonexistentContainer(ref cause) => write!(f, "{}", cause),
            UpdateRateBasedRuleError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateRateBasedRuleError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            UpdateRateBasedRuleError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRateBasedRuleError {}
/// Errors returned by UpdateRegexMatchSet
#[derive(Debug, PartialEq)]
pub enum UpdateRegexMatchSetError {
    /// <p>The name specified is invalid.</p>
    WAFDisallowedName(String),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateRegexMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRegexMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFDisallowedNameException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFDisallowedName(
                        err.msg,
                    ))
                }
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFInvalidOperation(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFLimitsExceeded(
                        err.msg,
                    ))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFNonexistentContainer(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateRegexMatchSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRegexMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRegexMatchSetError::WAFDisallowedName(ref cause) => write!(f, "{}", cause),
            UpdateRegexMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateRegexMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            UpdateRegexMatchSetError::WAFInvalidOperation(ref cause) => write!(f, "{}", cause),
            UpdateRegexMatchSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateRegexMatchSetError::WAFNonexistentContainer(ref cause) => write!(f, "{}", cause),
            UpdateRegexMatchSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateRegexMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRegexMatchSetError {}
/// Errors returned by UpdateRegexPatternSet
#[derive(Debug, PartialEq)]
pub enum UpdateRegexPatternSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p>The regular expression (regex) you specified in <code>RegexPatternString</code> is invalid.</p>
    WAFInvalidRegexPattern(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateRegexPatternSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRegexPatternSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFInvalidOperation(
                        err.msg,
                    ))
                }
                "WAFInvalidRegexPatternException" => {
                    return RusotoError::Service(
                        UpdateRegexPatternSetError::WAFInvalidRegexPattern(err.msg),
                    )
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFLimitsExceeded(
                        err.msg,
                    ))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(
                        UpdateRegexPatternSetError::WAFNonexistentContainer(err.msg),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateRegexPatternSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRegexPatternSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRegexPatternSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateRegexPatternSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            UpdateRegexPatternSetError::WAFInvalidOperation(ref cause) => write!(f, "{}", cause),
            UpdateRegexPatternSetError::WAFInvalidRegexPattern(ref cause) => write!(f, "{}", cause),
            UpdateRegexPatternSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateRegexPatternSetError::WAFNonexistentContainer(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateRegexPatternSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateRegexPatternSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRegexPatternSetError {}
/// Errors returned by UpdateRule
#[derive(Debug, PartialEq)]
pub enum UpdateRuleError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateRuleError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateRuleError::WAFInvalidAccount(err.msg))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateRuleError::WAFInvalidOperation(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateRuleError::WAFInvalidParameter(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateRuleError::WAFLimitsExceeded(err.msg))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateRuleError::WAFNonexistentContainer(err.msg))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateRuleError::WAFNonexistentItem(err.msg))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(UpdateRuleError::WAFReferencedItem(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateRuleError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRuleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRuleError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateRuleError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            UpdateRuleError::WAFInvalidOperation(ref cause) => write!(f, "{}", cause),
            UpdateRuleError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateRuleError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateRuleError::WAFNonexistentContainer(ref cause) => write!(f, "{}", cause),
            UpdateRuleError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateRuleError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            UpdateRuleError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRuleError {}
/// Errors returned by UpdateRuleGroup
#[derive(Debug, PartialEq)]
pub enum UpdateRuleGroupError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateRuleGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRuleGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFInternalError(err.msg))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFInvalidOperation(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFInvalidParameter(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFLimitsExceeded(err.msg))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFNonexistentContainer(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFNonexistentItem(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateRuleGroupError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateRuleGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRuleGroupError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateRuleGroupError::WAFInvalidOperation(ref cause) => write!(f, "{}", cause),
            UpdateRuleGroupError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateRuleGroupError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateRuleGroupError::WAFNonexistentContainer(ref cause) => write!(f, "{}", cause),
            UpdateRuleGroupError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateRuleGroupError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRuleGroupError {}
/// Errors returned by UpdateSizeConstraintSet
#[derive(Debug, PartialEq)]
pub enum UpdateSizeConstraintSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateSizeConstraintSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSizeConstraintSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFInvalidAccount(
                        err.msg,
                    ))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFInvalidOperation(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFLimitsExceeded(
                        err.msg,
                    ))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(
                        UpdateSizeConstraintSetError::WAFNonexistentContainer(err.msg),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFReferencedItem(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateSizeConstraintSetError::WAFStaleData(
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
impl fmt::Display for UpdateSizeConstraintSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSizeConstraintSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateSizeConstraintSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            UpdateSizeConstraintSetError::WAFInvalidOperation(ref cause) => write!(f, "{}", cause),
            UpdateSizeConstraintSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateSizeConstraintSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateSizeConstraintSetError::WAFNonexistentContainer(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateSizeConstraintSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateSizeConstraintSetError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            UpdateSizeConstraintSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSizeConstraintSetError {}
/// Errors returned by UpdateSqlInjectionMatchSet
#[derive(Debug, PartialEq)]
pub enum UpdateSqlInjectionMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateSqlInjectionMatchSetError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateSqlInjectionMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateSqlInjectionMatchSetError::WAFInternalError(
                        err.msg,
                    ))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(
                        UpdateSqlInjectionMatchSetError::WAFInvalidAccount(err.msg),
                    )
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(
                        UpdateSqlInjectionMatchSetError::WAFInvalidOperation(err.msg),
                    )
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(
                        UpdateSqlInjectionMatchSetError::WAFInvalidParameter(err.msg),
                    )
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(
                        UpdateSqlInjectionMatchSetError::WAFLimitsExceeded(err.msg),
                    )
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(
                        UpdateSqlInjectionMatchSetError::WAFNonexistentContainer(err.msg),
                    )
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(
                        UpdateSqlInjectionMatchSetError::WAFNonexistentItem(err.msg),
                    )
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateSqlInjectionMatchSetError::WAFStaleData(
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
impl fmt::Display for UpdateSqlInjectionMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSqlInjectionMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateSqlInjectionMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            UpdateSqlInjectionMatchSetError::WAFInvalidOperation(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateSqlInjectionMatchSetError::WAFInvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateSqlInjectionMatchSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateSqlInjectionMatchSetError::WAFNonexistentContainer(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateSqlInjectionMatchSetError::WAFNonexistentItem(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateSqlInjectionMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSqlInjectionMatchSetError {}
/// Errors returned by UpdateWebACL
#[derive(Debug, PartialEq)]
pub enum UpdateWebACLError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p><p>The operation failed because you tried to delete an object that is still in use. For example:</p> <ul> <li> <p>You tried to delete a <code>ByteMatchSet</code> that is still referenced by a <code>Rule</code>.</p> </li> <li> <p>You tried to delete a <code>Rule</code> that is still referenced by a <code>WebACL</code>.</p> </li> </ul></p>
    WAFReferencedItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
    /// <p>The specified subscription does not exist.</p>
    WAFSubscriptionNotFound(String),
}

impl UpdateWebACLError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateWebACLError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFInvalidAccount(err.msg))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFInvalidOperation(err.msg))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFInvalidParameter(err.msg))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFLimitsExceeded(err.msg))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFNonexistentContainer(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFNonexistentItem(err.msg))
                }
                "WAFReferencedItemException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFReferencedItem(err.msg))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFStaleData(err.msg))
                }
                "WAFSubscriptionNotFoundException" => {
                    return RusotoError::Service(UpdateWebACLError::WAFSubscriptionNotFound(
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
impl fmt::Display for UpdateWebACLError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateWebACLError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFInvalidOperation(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFNonexistentContainer(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFReferencedItem(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFStaleData(ref cause) => write!(f, "{}", cause),
            UpdateWebACLError::WAFSubscriptionNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateWebACLError {}
/// Errors returned by UpdateXssMatchSet
#[derive(Debug, PartialEq)]
pub enum UpdateXssMatchSetError {
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WAFInternalError(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using an invalid account identifier.</p>
    WAFInvalidAccount(String),
    /// <p><p>The operation failed because there was nothing to do. For example:</p> <ul> <li> <p>You tried to remove a <code>Rule</code> from a <code>WebACL</code>, but the <code>Rule</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to remove an IP address from an <code>IPSet</code>, but the IP address isn&#39;t in the specified <code>IPSet</code>.</p> </li> <li> <p>You tried to remove a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> isn&#39;t in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>Rule</code> to a <code>WebACL</code>, but the <code>Rule</code> already exists in the specified <code>WebACL</code>.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to a <code>ByteMatchSet</code>, but the <code>ByteMatchTuple</code> already exists in the specified <code>WebACL</code>.</p> </li> </ul></p>
    WAFInvalidOperation(String),
    /// <p><p>The operation failed because AWS WAF didn&#39;t recognize a parameter in the request. For example:</p> <ul> <li> <p>You specified an invalid parameter name.</p> </li> <li> <p>You specified an invalid value.</p> </li> <li> <p>You tried to update an object (<code>ByteMatchSet</code>, <code>IPSet</code>, <code>Rule</code>, or <code>WebACL</code>) using an action other than <code>INSERT</code> or <code>DELETE</code>.</p> </li> <li> <p>You tried to create a <code>WebACL</code> with a <code>DefaultAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to create a <code>RateBasedRule</code> with a <code>RateKey</code> value other than <code>IP</code>.</p> </li> <li> <p>You tried to update a <code>WebACL</code> with a <code>WafAction</code> <code>Type</code> other than <code>ALLOW</code>, <code>BLOCK</code>, or <code>COUNT</code>.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>FieldToMatch</code> <code>Type</code> other than HEADER, METHOD, QUERY_STRING, URI, or BODY.</p> </li> <li> <p>You tried to update a <code>ByteMatchSet</code> with a <code>Field</code> of <code>HEADER</code> but no value for <code>Data</code>.</p> </li> <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL cannot be associated.</p> </li> </ul></p>
    WAFInvalidParameter(String),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WAFLimitsExceeded(String),
    /// <p><p>The operation failed because you tried to add an object to or delete an object from another object that doesn&#39;t exist. For example:</p> <ul> <li> <p>You tried to add a <code>Rule</code> to or delete a <code>Rule</code> from a <code>WebACL</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchSet</code> to or delete a <code>ByteMatchSet</code> from a <code>Rule</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add an IP address to or delete an IP address from an <code>IPSet</code> that doesn&#39;t exist.</p> </li> <li> <p>You tried to add a <code>ByteMatchTuple</code> to or delete a <code>ByteMatchTuple</code> from a <code>ByteMatchSet</code> that doesn&#39;t exist.</p> </li> </ul></p>
    WAFNonexistentContainer(String),
    /// <p>The operation failed because the referenced object doesn't exist.</p>
    WAFNonexistentItem(String),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WAFStaleData(String),
}

impl UpdateXssMatchSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateXssMatchSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "WAFInternalErrorException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFInternalError(err.msg))
                }
                "WAFInvalidAccountException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFInvalidAccount(err.msg))
                }
                "WAFInvalidOperationException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFInvalidOperation(
                        err.msg,
                    ))
                }
                "WAFInvalidParameterException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFInvalidParameter(
                        err.msg,
                    ))
                }
                "WAFLimitsExceededException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFLimitsExceeded(err.msg))
                }
                "WAFNonexistentContainerException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFNonexistentContainer(
                        err.msg,
                    ))
                }
                "WAFNonexistentItemException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFNonexistentItem(
                        err.msg,
                    ))
                }
                "WAFStaleDataException" => {
                    return RusotoError::Service(UpdateXssMatchSetError::WAFStaleData(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateXssMatchSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateXssMatchSetError::WAFInternalError(ref cause) => write!(f, "{}", cause),
            UpdateXssMatchSetError::WAFInvalidAccount(ref cause) => write!(f, "{}", cause),
            UpdateXssMatchSetError::WAFInvalidOperation(ref cause) => write!(f, "{}", cause),
            UpdateXssMatchSetError::WAFInvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateXssMatchSetError::WAFLimitsExceeded(ref cause) => write!(f, "{}", cause),
            UpdateXssMatchSetError::WAFNonexistentContainer(ref cause) => write!(f, "{}", cause),
            UpdateXssMatchSetError::WAFNonexistentItem(ref cause) => write!(f, "{}", cause),
            UpdateXssMatchSetError::WAFStaleData(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateXssMatchSetError {}
/// Trait representing the capabilities of the WAF API. WAF clients implement this trait.
#[async_trait]
pub trait Waf {
    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <code>ByteMatchSet</code>. You then use <a>UpdateByteMatchSet</a> to identify the part of a web request that you want AWS WAF to inspect, such as the values of the <code>User-Agent</code> header or the query string. For example, you can create a <code>ByteMatchSet</code> that matches any requests with <code>User-Agent</code> headers that contain the string <code>BadBot</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateByteMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateByteMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateByteMatchSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateByteMatchSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_byte_match_set(
        &self,
        input: CreateByteMatchSetRequest,
    ) -> Result<CreateByteMatchSetResponse, RusotoError<CreateByteMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates an <a>GeoMatchSet</a>, which you use to specify which web requests you want to allow or block based on the country that the requests originate from. For example, if you&#39;re receiving a lot of requests from one or more countries and you want to block the requests, you can create an <code>GeoMatchSet</code> that contains those countries and then configure AWS WAF to block the requests. </p> <p>To create and configure a <code>GeoMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateGeoMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateGeoMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateGeoMatchSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateGeoMatchSetSet</code> request to specify the countries that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_geo_match_set(
        &self,
        input: CreateGeoMatchSetRequest,
    ) -> Result<CreateGeoMatchSetResponse, RusotoError<CreateGeoMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates an <a>IPSet</a>, which you use to specify which web requests that you want to allow or block based on the IP addresses that the requests originate from. For example, if you&#39;re receiving a lot of requests from one or more individual IP addresses or one or more ranges of IP addresses and you want to block the requests, you can create an <code>IPSet</code> that contains those IP addresses and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>IPSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateIPSet</code> request.</p> </li> <li> <p>Submit a <code>CreateIPSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateIPSet</code> request to specify the IP addresses that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_ip_set(
        &self,
        input: CreateIPSetRequest,
    ) -> Result<CreateIPSetResponse, RusotoError<CreateIPSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <a>RateBasedRule</a>. The <code>RateBasedRule</code> contains a <code>RateLimit</code>, which specifies the maximum number of requests that AWS WAF allows from a specified IP address in a five-minute period. The <code>RateBasedRule</code> also contains the <code>IPSet</code> objects, <code>ByteMatchSet</code> objects, and other predicates that identify the requests that you want to count or block if these requests exceed the <code>RateLimit</code>.</p> <p>If you add more than one predicate to a <code>RateBasedRule</code>, a request not only must exceed the <code>RateLimit</code>, but it also must match all the conditions to be counted or blocked. For example, suppose you add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 1,000.</p> <p>You then add the <code>RateBasedRule</code> to a <code>WebACL</code> and specify that you want to block requests that meet the conditions in the rule. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>. Further, requests that match these two conditions must be received at a rate of more than 1,000 requests every five minutes. If both conditions are met and the rate is exceeded, AWS WAF blocks the requests. If the rate drops below 1,000 for a five-minute period, AWS WAF no longer blocks the requests.</p> <p>As a second example, suppose you want to limit requests to a particular page on your site. To do this, you could add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>A <code>ByteMatchSet</code> with <code>FieldToMatch</code> of <code>URI</code> </p> </li> <li> <p>A <code>PositionalConstraint</code> of <code>STARTS_WITH</code> </p> </li> <li> <p>A <code>TargetString</code> of <code>login</code> </p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 1,000.</p> <p>By adding this <code>RateBasedRule</code> to a <code>WebACL</code>, you could limit requests to your login page without affecting the rest of your site.</p> <p>To create and configure a <code>RateBasedRule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the rule. For more information, see <a>CreateByteMatchSet</a>, <a>CreateIPSet</a>, and <a>CreateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRule</code> request.</p> </li> <li> <p>Submit a <code>CreateRateBasedRule</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRateBasedRule</code> request to specify the predicates that you want to include in the rule.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>RateBasedRule</code>. For more information, see <a>CreateWebACL</a>.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_rate_based_rule(
        &self,
        input: CreateRateBasedRuleRequest,
    ) -> Result<CreateRateBasedRuleResponse, RusotoError<CreateRateBasedRuleError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <a>RegexMatchSet</a>. You then use <a>UpdateRegexMatchSet</a> to identify the part of a web request that you want AWS WAF to inspect, such as the values of the <code>User-Agent</code> header or the query string. For example, you can create a <code>RegexMatchSet</code> that contains a <code>RegexMatchTuple</code> that looks for any requests with <code>User-Agent</code> headers that match a <code>RegexPatternSet</code> with pattern <code>B[a@]dB[o0]t</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>RegexMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRegexMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateRegexMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexMatchSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateRegexMatchSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value, using a <code>RegexPatternSet</code>, that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_regex_match_set(
        &self,
        input: CreateRegexMatchSetRequest,
    ) -> Result<CreateRegexMatchSetResponse, RusotoError<CreateRegexMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <code>RegexPatternSet</code>. You then use <a>UpdateRegexPatternSet</a> to specify the regular expression (regex) pattern that you want AWS WAF to search for, such as <code>B[a@]dB[o0]t</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>RegexPatternSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRegexPatternSet</code> request.</p> </li> <li> <p>Submit a <code>CreateRegexPatternSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexPatternSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateRegexPatternSet</a> request to specify the string that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_regex_pattern_set(
        &self,
        input: CreateRegexPatternSetRequest,
    ) -> Result<CreateRegexPatternSetResponse, RusotoError<CreateRegexPatternSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <code>Rule</code>, which contains the <code>IPSet</code> objects, <code>ByteMatchSet</code> objects, and other predicates that identify the requests that you want to block. If you add more than one predicate to a <code>Rule</code>, a request must match all of the specifications to be allowed or blocked. For example, suppose that you add the following to a <code>Rule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>You then add the <code>Rule</code> to a <code>WebACL</code> and specify that you want to blocks requests that satisfy the <code>Rule</code>. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>.</p> <p>To create and configure a <code>Rule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the <code>Rule</code>. For more information, see <a>CreateByteMatchSet</a>, <a>CreateIPSet</a>, and <a>CreateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRule</code> request.</p> </li> <li> <p>Submit a <code>CreateRule</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRule</code> request to specify the predicates that you want to include in the <code>Rule</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>Rule</code>. For more information, see <a>CreateWebACL</a>.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_rule(
        &self,
        input: CreateRuleRequest,
    ) -> Result<CreateRuleResponse, RusotoError<CreateRuleError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <code>RuleGroup</code>. A rule group is a collection of predefined rules that you add to a web ACL. You use <a>UpdateRuleGroup</a> to add rules to the rule group.</p> <p>Rule groups are subject to the following limits:</p> <ul> <li> <p>Three rule groups per account. You can request an increase to this limit by contacting customer support.</p> </li> <li> <p>One rule group per web ACL.</p> </li> <li> <p>Ten rules per rule group.</p> </li> </ul> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_rule_group(
        &self,
        input: CreateRuleGroupRequest,
    ) -> Result<CreateRuleGroupResponse, RusotoError<CreateRuleGroupError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <code>SizeConstraintSet</code>. You then use <a>UpdateSizeConstraintSet</a> to identify the part of a web request that you want AWS WAF to check for length, such as the length of the <code>User-Agent</code> header or the length of the query string. For example, you can create a <code>SizeConstraintSet</code> that matches any requests that have a query string that is longer than 100 bytes. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit a <code>CreateSizeConstraintSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateSizeConstraintSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_size_constraint_set(
        &self,
        input: CreateSizeConstraintSetRequest,
    ) -> Result<CreateSizeConstraintSetResponse, RusotoError<CreateSizeConstraintSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <a>SqlInjectionMatchSet</a>, which you use to allow, block, or count requests that contain snippets of SQL code in a specified part of web requests. AWS WAF searches for character sequences that are likely to be malicious strings.</p> <p>To create and configure a <code>SqlInjectionMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateSqlInjectionMatchSet</a> request.</p> </li> <li> <p>Submit an <a>UpdateSqlInjectionMatchSet</a> request to specify the parts of web requests in which you want to allow, block, or count malicious SQL code.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_sql_injection_match_set(
        &self,
        input: CreateSqlInjectionMatchSetRequest,
    ) -> Result<CreateSqlInjectionMatchSetResponse, RusotoError<CreateSqlInjectionMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <code>WebACL</code>, which contains the <code>Rules</code> that identify the CloudFront web requests that you want to allow, block, or count. AWS WAF evaluates <code>Rules</code> in order based on the value of <code>Priority</code> for each <code>Rule</code>.</p> <p>You also specify a default action, either <code>ALLOW</code> or <code>BLOCK</code>. If a web request doesn&#39;t match any of the <code>Rules</code> in a <code>WebACL</code>, AWS WAF responds to the request with the default action. </p> <p>To create and configure a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Create and update the <code>ByteMatchSet</code> objects and other predicates that you want to include in <code>Rules</code>. For more information, see <a>CreateByteMatchSet</a>, <a>UpdateByteMatchSet</a>, <a>CreateIPSet</a>, <a>UpdateIPSet</a>, <a>CreateSqlInjectionMatchSet</a>, and <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>WebACL</code>. For more information, see <a>CreateRule</a> and <a>UpdateRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateWebACL</code> request.</p> </li> <li> <p>Submit a <code>CreateWebACL</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateWebACL</a> request.</p> </li> <li> <p>Submit an <a>UpdateWebACL</a> request to specify the <code>Rules</code> that you want to include in the <code>WebACL</code>, to specify the default action, and to associate the <code>WebACL</code> with a CloudFront distribution.</p> </li> </ol> <p>For more information about how to use the AWS WAF API, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_web_acl(
        &self,
        input: CreateWebACLRequest,
    ) -> Result<CreateWebACLResponse, RusotoError<CreateWebACLError>>;

    /// <p>Creates an AWS CloudFormation WAFV2 template for the specified web ACL in the specified Amazon S3 bucket. Then, in CloudFormation, you create a stack from the template, to create the web ACL and its resources in AWS WAFV2. Use this to migrate your AWS WAF Classic web ACL to the latest version of AWS WAF.</p> <p>This is part of a larger migration procedure for web ACLs from AWS WAF Classic to the latest version of AWS WAF. For the full procedure, including caveats and manual steps to complete the migration and switch over to the new web ACL, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-migrating-from-classic.html">Migrating your AWS WAF Classic resources to AWS WAF</a> in the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p>
    async fn create_web_acl_migration_stack(
        &self,
        input: CreateWebACLMigrationStackRequest,
    ) -> Result<CreateWebACLMigrationStackResponse, RusotoError<CreateWebACLMigrationStackError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates an <a>XssMatchSet</a>, which you use to allow, block, or count requests that contain cross-site scripting attacks in the specified part of web requests. AWS WAF searches for character sequences that are likely to be malicious strings.</p> <p>To create and configure an <code>XssMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateXssMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateXssMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateXssMatchSet</a> request.</p> </li> <li> <p>Submit an <a>UpdateXssMatchSet</a> request to specify the parts of web requests in which you want to allow, block, or count cross-site scripting attacks.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_xss_match_set(
        &self,
        input: CreateXssMatchSetRequest,
    ) -> Result<CreateXssMatchSetResponse, RusotoError<CreateXssMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>ByteMatchSet</a>. You can&#39;t delete a <code>ByteMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any <a>ByteMatchTuple</a> objects (any filters).</p> <p>If you just want to remove a <code>ByteMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>ByteMatchSet</code> to remove filters, if any. For more information, see <a>UpdateByteMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteByteMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteByteMatchSet</code> request.</p> </li> </ol></p>
    async fn delete_byte_match_set(
        &self,
        input: DeleteByteMatchSetRequest,
    ) -> Result<DeleteByteMatchSetResponse, RusotoError<DeleteByteMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>GeoMatchSet</a>. You can&#39;t delete a <code>GeoMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any countries.</p> <p>If you just want to remove a <code>GeoMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>GeoMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>GeoMatchSet</code> to remove any countries. For more information, see <a>UpdateGeoMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteGeoMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteGeoMatchSet</code> request.</p> </li> </ol></p>
    async fn delete_geo_match_set(
        &self,
        input: DeleteGeoMatchSetRequest,
    ) -> Result<DeleteGeoMatchSetResponse, RusotoError<DeleteGeoMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes an <a>IPSet</a>. You can&#39;t delete an <code>IPSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any IP addresses.</p> <p>If you just want to remove an <code>IPSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete an <code>IPSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>IPSet</code> to remove IP address ranges, if any. For more information, see <a>UpdateIPSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteIPSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteIPSet</code> request.</p> </li> </ol></p>
    async fn delete_ip_set(
        &self,
        input: DeleteIPSetRequest,
    ) -> Result<DeleteIPSetResponse, RusotoError<DeleteIPSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes the <a>LoggingConfiguration</a> from the specified web ACL.</p></p>
    async fn delete_logging_configuration(
        &self,
        input: DeleteLoggingConfigurationRequest,
    ) -> Result<DeleteLoggingConfigurationResponse, RusotoError<DeleteLoggingConfigurationError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes an IAM policy from the specified RuleGroup.</p> <p>The user making the request must be the owner of the RuleGroup.</p></p>
    async fn delete_permission_policy(
        &self,
        input: DeletePermissionPolicyRequest,
    ) -> Result<DeletePermissionPolicyResponse, RusotoError<DeletePermissionPolicyError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>RateBasedRule</a>. You can&#39;t delete a rule if it&#39;s still used in any <code>WebACL</code> objects or if it still includes any predicates, such as <code>ByteMatchSet</code> objects.</p> <p>If you just want to remove a rule from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>RateBasedRule</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>RateBasedRule</code> to remove predicates, if any. For more information, see <a>UpdateRateBasedRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRateBasedRule</code> request.</p> </li> <li> <p>Submit a <code>DeleteRateBasedRule</code> request.</p> </li> </ol></p>
    async fn delete_rate_based_rule(
        &self,
        input: DeleteRateBasedRuleRequest,
    ) -> Result<DeleteRateBasedRuleResponse, RusotoError<DeleteRateBasedRuleError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>RegexMatchSet</a>. You can&#39;t delete a <code>RegexMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any <code>RegexMatchTuples</code> objects (any filters).</p> <p>If you just want to remove a <code>RegexMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>RegexMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>RegexMatchSet</code> to remove filters, if any. For more information, see <a>UpdateRegexMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRegexMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteRegexMatchSet</code> request.</p> </li> </ol></p>
    async fn delete_regex_match_set(
        &self,
        input: DeleteRegexMatchSetRequest,
    ) -> Result<DeleteRegexMatchSetResponse, RusotoError<DeleteRegexMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>RegexPatternSet</a>. You can&#39;t delete a <code>RegexPatternSet</code> if it&#39;s still used in any <code>RegexMatchSet</code> or if the <code>RegexPatternSet</code> is not empty. </p></p>
    async fn delete_regex_pattern_set(
        &self,
        input: DeleteRegexPatternSetRequest,
    ) -> Result<DeleteRegexPatternSetResponse, RusotoError<DeleteRegexPatternSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>Rule</a>. You can&#39;t delete a <code>Rule</code> if it&#39;s still used in any <code>WebACL</code> objects or if it still includes any predicates, such as <code>ByteMatchSet</code> objects.</p> <p>If you just want to remove a <code>Rule</code> from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>Rule</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>Rule</code> to remove predicates, if any. For more information, see <a>UpdateRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRule</code> request.</p> </li> <li> <p>Submit a <code>DeleteRule</code> request.</p> </li> </ol></p>
    async fn delete_rule(
        &self,
        input: DeleteRuleRequest,
    ) -> Result<DeleteRuleResponse, RusotoError<DeleteRuleError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>RuleGroup</a>. You can&#39;t delete a <code>RuleGroup</code> if it&#39;s still used in any <code>WebACL</code> objects or if it still includes any rules.</p> <p>If you just want to remove a <code>RuleGroup</code> from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>RuleGroup</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>RuleGroup</code> to remove rules, if any. For more information, see <a>UpdateRuleGroup</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRuleGroup</code> request.</p> </li> <li> <p>Submit a <code>DeleteRuleGroup</code> request.</p> </li> </ol></p>
    async fn delete_rule_group(
        &self,
        input: DeleteRuleGroupRequest,
    ) -> Result<DeleteRuleGroupResponse, RusotoError<DeleteRuleGroupError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>SizeConstraintSet</a>. You can&#39;t delete a <code>SizeConstraintSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any <a>SizeConstraint</a> objects (any filters).</p> <p>If you just want to remove a <code>SizeConstraintSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>SizeConstraintSet</code> to remove filters, if any. For more information, see <a>UpdateSizeConstraintSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteSizeConstraintSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteSizeConstraintSet</code> request.</p> </li> </ol></p>
    async fn delete_size_constraint_set(
        &self,
        input: DeleteSizeConstraintSetRequest,
    ) -> Result<DeleteSizeConstraintSetResponse, RusotoError<DeleteSizeConstraintSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>SqlInjectionMatchSet</a>. You can&#39;t delete a <code>SqlInjectionMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still contains any <a>SqlInjectionMatchTuple</a> objects.</p> <p>If you just want to remove a <code>SqlInjectionMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>SqlInjectionMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>SqlInjectionMatchSet</code> to remove filters, if any. For more information, see <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteSqlInjectionMatchSet</code> request.</p> </li> </ol></p>
    async fn delete_sql_injection_match_set(
        &self,
        input: DeleteSqlInjectionMatchSetRequest,
    ) -> Result<DeleteSqlInjectionMatchSetResponse, RusotoError<DeleteSqlInjectionMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>WebACL</a>. You can&#39;t delete a <code>WebACL</code> if it still contains any <code>Rules</code>.</p> <p>To delete a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>WebACL</code> to remove <code>Rules</code>, if any. For more information, see <a>UpdateWebACL</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteWebACL</code> request.</p> </li> <li> <p>Submit a <code>DeleteWebACL</code> request.</p> </li> </ol></p>
    async fn delete_web_acl(
        &self,
        input: DeleteWebACLRequest,
    ) -> Result<DeleteWebACLResponse, RusotoError<DeleteWebACLError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes an <a>XssMatchSet</a>. You can&#39;t delete an <code>XssMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still contains any <a>XssMatchTuple</a> objects.</p> <p>If you just want to remove an <code>XssMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete an <code>XssMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>XssMatchSet</code> to remove filters, if any. For more information, see <a>UpdateXssMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteXssMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteXssMatchSet</code> request.</p> </li> </ol></p>
    async fn delete_xss_match_set(
        &self,
        input: DeleteXssMatchSetRequest,
    ) -> Result<DeleteXssMatchSetResponse, RusotoError<DeleteXssMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>ByteMatchSet</a> specified by <code>ByteMatchSetId</code>.</p></p>
    async fn get_byte_match_set(
        &self,
        input: GetByteMatchSetRequest,
    ) -> Result<GetByteMatchSetResponse, RusotoError<GetByteMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>When you want to create, update, or delete AWS WAF objects, get a change token and include the change token in the create, update, or delete request. Change tokens ensure that your application doesn&#39;t submit conflicting requests to AWS WAF.</p> <p>Each create, update, or delete request must use a unique change token. If your application submits a <code>GetChangeToken</code> request and then submits a second <code>GetChangeToken</code> request before submitting a create, update, or delete request, the second <code>GetChangeToken</code> request returns the same value as the first <code>GetChangeToken</code> request.</p> <p>When you use a change token in a create, update, or delete request, the status of the change token changes to <code>PENDING</code>, which indicates that AWS WAF is propagating the change to all AWS WAF servers. Use <code>GetChangeTokenStatus</code> to determine the status of your change token.</p></p>
    async fn get_change_token(
        &self,
    ) -> Result<GetChangeTokenResponse, RusotoError<GetChangeTokenError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the status of a <code>ChangeToken</code> that you got by calling <a>GetChangeToken</a>. <code>ChangeTokenStatus</code> is one of the following values:</p> <ul> <li> <p> <code>PROVISIONED</code>: You requested the change token by calling <code>GetChangeToken</code>, but you haven&#39;t used it yet in a call to create, update, or delete an AWS WAF object.</p> </li> <li> <p> <code>PENDING</code>: AWS WAF is propagating the create, update, or delete request to all AWS WAF servers.</p> </li> <li> <p> <code>INSYNC</code>: Propagation is complete.</p> </li> </ul></p>
    async fn get_change_token_status(
        &self,
        input: GetChangeTokenStatusRequest,
    ) -> Result<GetChangeTokenStatusResponse, RusotoError<GetChangeTokenStatusError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>GeoMatchSet</a> that is specified by <code>GeoMatchSetId</code>.</p></p>
    async fn get_geo_match_set(
        &self,
        input: GetGeoMatchSetRequest,
    ) -> Result<GetGeoMatchSetResponse, RusotoError<GetGeoMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>IPSet</a> that is specified by <code>IPSetId</code>.</p></p>
    async fn get_ip_set(
        &self,
        input: GetIPSetRequest,
    ) -> Result<GetIPSetResponse, RusotoError<GetIPSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>LoggingConfiguration</a> for the specified web ACL.</p></p>
    async fn get_logging_configuration(
        &self,
        input: GetLoggingConfigurationRequest,
    ) -> Result<GetLoggingConfigurationResponse, RusotoError<GetLoggingConfigurationError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the IAM policy attached to the RuleGroup.</p></p>
    async fn get_permission_policy(
        &self,
        input: GetPermissionPolicyRequest,
    ) -> Result<GetPermissionPolicyResponse, RusotoError<GetPermissionPolicyError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>RateBasedRule</a> that is specified by the <code>RuleId</code> that you included in the <code>GetRateBasedRule</code> request.</p></p>
    async fn get_rate_based_rule(
        &self,
        input: GetRateBasedRuleRequest,
    ) -> Result<GetRateBasedRuleResponse, RusotoError<GetRateBasedRuleError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of IP addresses currently being blocked by the <a>RateBasedRule</a> that is specified by the <code>RuleId</code>. The maximum number of managed keys that will be blocked is 10,000. If more than 10,000 addresses exceed the rate limit, the 10,000 addresses with the highest rates will be blocked.</p></p>
    async fn get_rate_based_rule_managed_keys(
        &self,
        input: GetRateBasedRuleManagedKeysRequest,
    ) -> Result<GetRateBasedRuleManagedKeysResponse, RusotoError<GetRateBasedRuleManagedKeysError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>RegexMatchSet</a> specified by <code>RegexMatchSetId</code>.</p></p>
    async fn get_regex_match_set(
        &self,
        input: GetRegexMatchSetRequest,
    ) -> Result<GetRegexMatchSetResponse, RusotoError<GetRegexMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>RegexPatternSet</a> specified by <code>RegexPatternSetId</code>.</p></p>
    async fn get_regex_pattern_set(
        &self,
        input: GetRegexPatternSetRequest,
    ) -> Result<GetRegexPatternSetResponse, RusotoError<GetRegexPatternSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>Rule</a> that is specified by the <code>RuleId</code> that you included in the <code>GetRule</code> request.</p></p>
    async fn get_rule(
        &self,
        input: GetRuleRequest,
    ) -> Result<GetRuleResponse, RusotoError<GetRuleError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>RuleGroup</a> that is specified by the <code>RuleGroupId</code> that you included in the <code>GetRuleGroup</code> request.</p> <p>To view the rules in a rule group, use <a>ListActivatedRulesInRuleGroup</a>.</p></p>
    async fn get_rule_group(
        &self,
        input: GetRuleGroupRequest,
    ) -> Result<GetRuleGroupResponse, RusotoError<GetRuleGroupError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Gets detailed information about a specified number of requests--a sample--that AWS WAF randomly selects from among the first 5,000 requests that your AWS resource received during a time range that you choose. You can specify a sample size of up to 500 requests, and you can specify any time range in the previous three hours.</p> <p> <code>GetSampledRequests</code> returns a time range, which is usually the time range that you specified. However, if your resource (such as a CloudFront distribution) received 5,000 requests before the specified time range elapsed, <code>GetSampledRequests</code> returns an updated time range. This new time range indicates the actual period during which AWS WAF selected the requests in the sample.</p></p>
    async fn get_sampled_requests(
        &self,
        input: GetSampledRequestsRequest,
    ) -> Result<GetSampledRequestsResponse, RusotoError<GetSampledRequestsError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>SizeConstraintSet</a> specified by <code>SizeConstraintSetId</code>.</p></p>
    async fn get_size_constraint_set(
        &self,
        input: GetSizeConstraintSetRequest,
    ) -> Result<GetSizeConstraintSetResponse, RusotoError<GetSizeConstraintSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>SqlInjectionMatchSet</a> that is specified by <code>SqlInjectionMatchSetId</code>.</p></p>
    async fn get_sql_injection_match_set(
        &self,
        input: GetSqlInjectionMatchSetRequest,
    ) -> Result<GetSqlInjectionMatchSetResponse, RusotoError<GetSqlInjectionMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>WebACL</a> that is specified by <code>WebACLId</code>.</p></p>
    async fn get_web_acl(
        &self,
        input: GetWebACLRequest,
    ) -> Result<GetWebACLResponse, RusotoError<GetWebACLError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>XssMatchSet</a> that is specified by <code>XssMatchSetId</code>.</p></p>
    async fn get_xss_match_set(
        &self,
        input: GetXssMatchSetRequest,
    ) -> Result<GetXssMatchSetResponse, RusotoError<GetXssMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>ActivatedRule</a> objects.</p></p>
    async fn list_activated_rules_in_rule_group(
        &self,
        input: ListActivatedRulesInRuleGroupRequest,
    ) -> Result<
        ListActivatedRulesInRuleGroupResponse,
        RusotoError<ListActivatedRulesInRuleGroupError>,
    >;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>ByteMatchSetSummary</a> objects.</p></p>
    async fn list_byte_match_sets(
        &self,
        input: ListByteMatchSetsRequest,
    ) -> Result<ListByteMatchSetsResponse, RusotoError<ListByteMatchSetsError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>GeoMatchSetSummary</a> objects in the response.</p></p>
    async fn list_geo_match_sets(
        &self,
        input: ListGeoMatchSetsRequest,
    ) -> Result<ListGeoMatchSetsResponse, RusotoError<ListGeoMatchSetsError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>IPSetSummary</a> objects in the response.</p></p>
    async fn list_ip_sets(
        &self,
        input: ListIPSetsRequest,
    ) -> Result<ListIPSetsResponse, RusotoError<ListIPSetsError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>LoggingConfiguration</a> objects.</p></p>
    async fn list_logging_configurations(
        &self,
        input: ListLoggingConfigurationsRequest,
    ) -> Result<ListLoggingConfigurationsResponse, RusotoError<ListLoggingConfigurationsError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>RuleSummary</a> objects.</p></p>
    async fn list_rate_based_rules(
        &self,
        input: ListRateBasedRulesRequest,
    ) -> Result<ListRateBasedRulesResponse, RusotoError<ListRateBasedRulesError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>RegexMatchSetSummary</a> objects.</p></p>
    async fn list_regex_match_sets(
        &self,
        input: ListRegexMatchSetsRequest,
    ) -> Result<ListRegexMatchSetsResponse, RusotoError<ListRegexMatchSetsError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>RegexPatternSetSummary</a> objects.</p></p>
    async fn list_regex_pattern_sets(
        &self,
        input: ListRegexPatternSetsRequest,
    ) -> Result<ListRegexPatternSetsResponse, RusotoError<ListRegexPatternSetsError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>RuleGroup</a> objects.</p></p>
    async fn list_rule_groups(
        &self,
        input: ListRuleGroupsRequest,
    ) -> Result<ListRuleGroupsResponse, RusotoError<ListRuleGroupsError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>RuleSummary</a> objects.</p></p>
    async fn list_rules(
        &self,
        input: ListRulesRequest,
    ) -> Result<ListRulesResponse, RusotoError<ListRulesError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>SizeConstraintSetSummary</a> objects.</p></p>
    async fn list_size_constraint_sets(
        &self,
        input: ListSizeConstraintSetsRequest,
    ) -> Result<ListSizeConstraintSetsResponse, RusotoError<ListSizeConstraintSetsError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>SqlInjectionMatchSet</a> objects.</p></p>
    async fn list_sql_injection_match_sets(
        &self,
        input: ListSqlInjectionMatchSetsRequest,
    ) -> Result<ListSqlInjectionMatchSetsResponse, RusotoError<ListSqlInjectionMatchSetsError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>RuleGroup</a> objects that you are subscribed to.</p></p>
    async fn list_subscribed_rule_groups(
        &self,
        input: ListSubscribedRuleGroupsRequest,
    ) -> Result<ListSubscribedRuleGroupsResponse, RusotoError<ListSubscribedRuleGroupsError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Retrieves the tags associated with the specified AWS resource. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. For example, you might set the tag key to &quot;customer&quot; and the value to the customer name or ID. You can specify one or more tags to add to each AWS resource, up to 50 tags for a resource.</p> <p>Tagging is only available through the API, SDKs, and CLI. You can&#39;t manage or view tags through the AWS WAF Classic console. You can tag the AWS resources that you manage through AWS WAF Classic: web ACLs, rule groups, and rules. </p></p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>WebACLSummary</a> objects in the response.</p></p>
    async fn list_web_ac_ls(
        &self,
        input: ListWebACLsRequest,
    ) -> Result<ListWebACLsResponse, RusotoError<ListWebACLsError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>XssMatchSet</a> objects.</p></p>
    async fn list_xss_match_sets(
        &self,
        input: ListXssMatchSetsRequest,
    ) -> Result<ListXssMatchSetsResponse, RusotoError<ListXssMatchSetsError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Associates a <a>LoggingConfiguration</a> with a specified web ACL.</p> <p>You can access information about all traffic that AWS WAF inspects using the following steps:</p> <ol> <li> <p>Create an Amazon Kinesis Data Firehose. </p> <p>Create the data firehose with a PUT source and in the region that you are operating. However, if you are capturing logs for Amazon CloudFront, always create the firehose in US East (N. Virginia). </p> <note> <p>Do not create the data firehose using a <code>Kinesis stream</code> as your source.</p> </note> </li> <li> <p>Associate that firehose to your web ACL using a <code>PutLoggingConfiguration</code> request.</p> </li> </ol> <p>When you successfully enable logging using a <code>PutLoggingConfiguration</code> request, AWS WAF will create a service linked role with the necessary permissions to write logs to the Amazon Kinesis Data Firehose. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/logging.html">Logging Web ACL Traffic Information</a> in the <i>AWS WAF Developer Guide</i>.</p></p>
    async fn put_logging_configuration(
        &self,
        input: PutLoggingConfigurationRequest,
    ) -> Result<PutLoggingConfigurationResponse, RusotoError<PutLoggingConfigurationError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Attaches an IAM policy to the specified resource. The only supported use for this action is to share a RuleGroup across accounts.</p> <p>The <code>PutPermissionPolicy</code> is subject to the following restrictions:</p> <ul> <li> <p>You can attach only one policy with each <code>PutPermissionPolicy</code> request.</p> </li> <li> <p>The policy must include an <code>Effect</code>, <code>Action</code> and <code>Principal</code>. </p> </li> <li> <p> <code>Effect</code> must specify <code>Allow</code>.</p> </li> <li> <p>The <code>Action</code> in the policy must be <code>waf:UpdateWebACL</code>, <code>waf-regional:UpdateWebACL</code>, <code>waf:GetRuleGroup</code> and <code>waf-regional:GetRuleGroup</code> . Any extra or wildcard actions in the policy will be rejected.</p> </li> <li> <p>The policy cannot include a <code>Resource</code> parameter.</p> </li> <li> <p>The ARN in the request must be a valid WAF RuleGroup ARN and the RuleGroup must exist in the same region.</p> </li> <li> <p>The user making the request must be the owner of the RuleGroup.</p> </li> <li> <p>Your policy must be composed using IAM Policy version 2012-10-17.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html">IAM Policies</a>. </p> <p>An example of a valid policy parameter is shown in the Examples section below.</p></p>
    async fn put_permission_policy(
        &self,
        input: PutPermissionPolicyRequest,
    ) -> Result<PutPermissionPolicyResponse, RusotoError<PutPermissionPolicyError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Associates tags with the specified AWS resource. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. For example, you might set the tag key to &quot;customer&quot; and the value to the customer name or ID. You can specify one or more tags to add to each AWS resource, up to 50 tags for a resource.</p> <p>Tagging is only available through the API, SDKs, and CLI. You can&#39;t manage or view tags through the AWS WAF Classic console. You can use this action to tag the AWS resources that you manage through AWS WAF Classic: web ACLs, rule groups, and rules. </p></p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p/></p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>ByteMatchTuple</a> objects (filters) in a <a>ByteMatchSet</a>. For each <code>ByteMatchTuple</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>ByteMatchSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to inspect, such as a query string or the value of the <code>User-Agent</code> header. </p> </li> <li> <p>The bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to look for. For more information, including how you specify the values for the AWS WAF API and the AWS CLI or SDKs, see <code>TargetString</code> in the <a>ByteMatchTuple</a> data type. </p> </li> <li> <p>Where to look, such as at the beginning or the end of a query string.</p> </li> <li> <p>Whether to perform any conversions on the request, such as converting it to lowercase, before inspecting it for the specified string.</p> </li> </ul> <p>For example, you can add a <code>ByteMatchSetUpdate</code> object that matches web requests in which <code>User-Agent</code> headers contain the string <code>BadBot</code>. You can then configure AWS WAF to block those requests.</p> <p>To create and configure a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>ByteMatchSet.</code> For more information, see <a>CreateByteMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateByteMatchSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateByteMatchSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_byte_match_set(
        &self,
        input: UpdateByteMatchSetRequest,
    ) -> Result<UpdateByteMatchSetResponse, RusotoError<UpdateByteMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>GeoMatchConstraint</a> objects in an <code>GeoMatchSet</code>. For each <code>GeoMatchConstraint</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change an <code>GeoMatchConstraint</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The <code>Type</code>. The only valid value for <code>Type</code> is <code>Country</code>.</p> </li> <li> <p>The <code>Value</code>, which is a two character code for the country to add to the <code>GeoMatchConstraint</code> object. Valid codes are listed in <a>GeoMatchConstraint$Value</a>.</p> </li> </ul> <p>To create and configure an <code>GeoMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateGeoMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateGeoMatchSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateGeoMatchSet</code> request to specify the country that you want AWS WAF to watch for.</p> </li> </ol> <p>When you update an <code>GeoMatchSet</code>, you specify the country that you want to add and/or the country that you want to delete. If you want to change a country, you delete the existing country and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_geo_match_set(
        &self,
        input: UpdateGeoMatchSetRequest,
    ) -> Result<UpdateGeoMatchSetResponse, RusotoError<UpdateGeoMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>IPSetDescriptor</a> objects in an <code>IPSet</code>. For each <code>IPSetDescriptor</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change an <code>IPSetDescriptor</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The IP address version, <code>IPv4</code> or <code>IPv6</code>. </p> </li> <li> <p>The IP address in CIDR notation, for example, <code>192.0.2.0/24</code> (for the range of IP addresses from <code>192.0.2.0</code> to <code>192.0.2.255</code>) or <code>192.0.2.44/32</code> (for the individual IP address <code>192.0.2.44</code>). </p> </li> </ul> <p>AWS WAF supports IPv4 address ranges: /8 and any range between /16 through /32. AWS WAF supports IPv6 address ranges: /24, /32, /48, /56, /64, and /128. For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p> <p>IPv6 addresses can be represented using any of the following formats:</p> <ul> <li> <p>1111:0000:0000:0000:0000:0000:0000:0111/128</p> </li> <li> <p>1111:0:0:0:0:0:0:0111/128</p> </li> <li> <p>1111::0111/128</p> </li> <li> <p>1111::111/128</p> </li> </ul> <p>You use an <code>IPSet</code> to specify which web requests you want to allow or block based on the IP addresses that the requests originated from. For example, if you&#39;re receiving a lot of requests from one or a small number of IP addresses and you want to block the requests, you can create an <code>IPSet</code> that specifies those IP addresses, and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>IPSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateIPSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateIPSet</code> request to specify the IP addresses that you want AWS WAF to watch for.</p> </li> </ol> <p>When you update an <code>IPSet</code>, you specify the IP addresses that you want to add and/or the IP addresses that you want to delete. If you want to change an IP address, you delete the existing IP address and add the new one.</p> <p>You can insert a maximum of 1000 addresses in a single request.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_ip_set(
        &self,
        input: UpdateIPSetRequest,
    ) -> Result<UpdateIPSetResponse, RusotoError<UpdateIPSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>Predicate</a> objects in a rule and updates the <code>RateLimit</code> in the rule. </p> <p>Each <code>Predicate</code> object identifies a predicate, such as a <a>ByteMatchSet</a> or an <a>IPSet</a>, that specifies the web requests that you want to block or count. The <code>RateLimit</code> specifies the number of requests every five minutes that triggers the rule.</p> <p>If you add more than one predicate to a <code>RateBasedRule</code>, a request must match all the predicates and exceed the <code>RateLimit</code> to be counted or blocked. For example, suppose you add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 1,000.</p> <p>You then add the <code>RateBasedRule</code> to a <code>WebACL</code> and specify that you want to block requests that satisfy the rule. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>. Further, requests that match these two conditions much be received at a rate of more than 1,000 every five minutes. If the rate drops below this limit, AWS WAF no longer blocks the requests.</p> <p>As a second example, suppose you want to limit requests to a particular page on your site. To do this, you could add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>A <code>ByteMatchSet</code> with <code>FieldToMatch</code> of <code>URI</code> </p> </li> <li> <p>A <code>PositionalConstraint</code> of <code>STARTS_WITH</code> </p> </li> <li> <p>A <code>TargetString</code> of <code>login</code> </p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 1,000.</p> <p>By adding this <code>RateBasedRule</code> to a <code>WebACL</code>, you could limit requests to your login page without affecting the rest of your site.</p></p>
    async fn update_rate_based_rule(
        &self,
        input: UpdateRateBasedRuleRequest,
    ) -> Result<UpdateRateBasedRuleResponse, RusotoError<UpdateRateBasedRuleError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>RegexMatchTuple</a> objects (filters) in a <a>RegexMatchSet</a>. For each <code>RegexMatchSetUpdate</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>RegexMatchSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to inspectupdate, such as a query string or the value of the <code>User-Agent</code> header. </p> </li> <li> <p>The identifier of the pattern (a regular expression) that you want AWS WAF to look for. For more information, see <a>RegexPatternSet</a>. </p> </li> <li> <p>Whether to perform any conversions on the request, such as converting it to lowercase, before inspecting it for the specified string.</p> </li> </ul> <p> For example, you can create a <code>RegexPatternSet</code> that matches any requests with <code>User-Agent</code> headers that contain the string <code>B[a@]dB[o0]t</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>RegexMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>RegexMatchSet.</code> For more information, see <a>CreateRegexMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexMatchSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateRegexMatchSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the identifier of the <code>RegexPatternSet</code> that contain the regular expression patters you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_regex_match_set(
        &self,
        input: UpdateRegexMatchSetRequest,
    ) -> Result<UpdateRegexMatchSetResponse, RusotoError<UpdateRegexMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <code>RegexPatternString</code> objects in a <a>RegexPatternSet</a>. For each <code>RegexPatternString</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the <code>RegexPatternString</code>.</p> </li> <li> <p>The regular expression pattern that you want to insert or delete. For more information, see <a>RegexPatternSet</a>. </p> </li> </ul> <p> For example, you can create a <code>RegexPatternString</code> such as <code>B[a@]dB[o0]t</code>. AWS WAF will match this <code>RegexPatternString</code> to:</p> <ul> <li> <p>BadBot</p> </li> <li> <p>BadB0t</p> </li> <li> <p>B@dBot</p> </li> <li> <p>B@dB0t</p> </li> </ul> <p>To create and configure a <code>RegexPatternSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>RegexPatternSet.</code> For more information, see <a>CreateRegexPatternSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexPatternSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateRegexPatternSet</code> request to specify the regular expression pattern that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_regex_pattern_set(
        &self,
        input: UpdateRegexPatternSetRequest,
    ) -> Result<UpdateRegexPatternSetResponse, RusotoError<UpdateRegexPatternSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>Predicate</a> objects in a <code>Rule</code>. Each <code>Predicate</code> object identifies a predicate, such as a <a>ByteMatchSet</a> or an <a>IPSet</a>, that specifies the web requests that you want to allow, block, or count. If you add more than one predicate to a <code>Rule</code>, a request must match all of the specifications to be allowed, blocked, or counted. For example, suppose that you add the following to a <code>Rule</code>: </p> <ul> <li> <p>A <code>ByteMatchSet</code> that matches the value <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44</code> </p> </li> </ul> <p>You then add the <code>Rule</code> to a <code>WebACL</code> and specify that you want to block requests that satisfy the <code>Rule</code>. For a request to be blocked, the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code> <i>and</i> the request must originate from the IP address 192.0.2.44.</p> <p>To create and configure a <code>Rule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the <code>Rule</code>.</p> </li> <li> <p>Create the <code>Rule</code>. See <a>CreateRule</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRule</code> request to add predicates to the <code>Rule</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>Rule</code>. See <a>CreateWebACL</a>.</p> </li> </ol> <p>If you want to replace one <code>ByteMatchSet</code> or <code>IPSet</code> with another, you delete the existing one and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_rule(
        &self,
        input: UpdateRuleRequest,
    ) -> Result<UpdateRuleResponse, RusotoError<UpdateRuleError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>ActivatedRule</a> objects in a <code>RuleGroup</code>.</p> <p>You can only insert <code>REGULAR</code> rules into a rule group.</p> <p>You can have a maximum of ten rules per rule group.</p> <p>To create and configure a <code>RuleGroup</code>, perform the following steps:</p> <ol> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>RuleGroup</code>. See <a>CreateRule</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRuleGroup</a> request.</p> </li> <li> <p>Submit an <code>UpdateRuleGroup</code> request to add <code>Rules</code> to the <code>RuleGroup</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>RuleGroup</code>. See <a>CreateWebACL</a>.</p> </li> </ol> <p>If you want to replace one <code>Rule</code> with another, you delete the existing one and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_rule_group(
        &self,
        input: UpdateRuleGroupRequest,
    ) -> Result<UpdateRuleGroupResponse, RusotoError<UpdateRuleGroupError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>SizeConstraint</a> objects (filters) in a <a>SizeConstraintSet</a>. For each <code>SizeConstraint</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>SizeConstraintSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to evaluate, such as the length of a query string or the length of the <code>User-Agent</code> header.</p> </li> <li> <p>Whether to perform any transformations on the request, such as converting it to lowercase, before checking its length. Note that transformations of the request body are not supported because the AWS resource forwards only the first <code>8192</code> bytes of your request to AWS WAF.</p> <p>You can only specify a single type of TextTransformation.</p> </li> <li> <p>A <code>ComparisonOperator</code> used for evaluating the selected part of the request against the specified <code>Size</code>, such as equals, greater than, less than, and so on.</p> </li> <li> <p>The length, in bytes, that you want AWS WAF to watch for in selected part of the request. The length is computed after applying the transformation.</p> </li> </ul> <p>For example, you can add a <code>SizeConstraintSetUpdate</code> object that matches web requests in which the length of the <code>User-Agent</code> header is greater than 100 bytes. You can then configure AWS WAF to block those requests.</p> <p>To create and configure a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>SizeConstraintSet.</code> For more information, see <a>CreateSizeConstraintSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateSizeConstraintSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_size_constraint_set(
        &self,
        input: UpdateSizeConstraintSetRequest,
    ) -> Result<UpdateSizeConstraintSetResponse, RusotoError<UpdateSizeConstraintSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>SqlInjectionMatchTuple</a> objects (filters) in a <a>SqlInjectionMatchSet</a>. For each <code>SqlInjectionMatchTuple</code> object, you specify the following values:</p> <ul> <li> <p> <code>Action</code>: Whether to insert the object into or delete the object from the array. To change a <code>SqlInjectionMatchTuple</code>, you delete the existing object and add a new one.</p> </li> <li> <p> <code>FieldToMatch</code>: The part of web requests that you want AWS WAF to inspect and, if you want AWS WAF to inspect a header or custom query parameter, the name of the header or parameter.</p> </li> <li> <p> <code>TextTransformation</code>: Which text transformation, if any, to perform on the web request before inspecting the request for snippets of malicious SQL code.</p> <p>You can only specify a single type of TextTransformation.</p> </li> </ul> <p>You use <code>SqlInjectionMatchSet</code> objects to specify which CloudFront requests that you want to allow, block, or count. For example, if you&#39;re receiving requests that contain snippets of SQL code in the query string and you want to block the requests, you can create a <code>SqlInjectionMatchSet</code> with the applicable settings, and then configure AWS WAF to block the requests. </p> <p>To create and configure a <code>SqlInjectionMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateSqlInjectionMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateSqlInjectionMatchSet</code> request to specify the parts of web requests that you want AWS WAF to inspect for snippets of SQL code.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_sql_injection_match_set(
        &self,
        input: UpdateSqlInjectionMatchSetRequest,
    ) -> Result<UpdateSqlInjectionMatchSetResponse, RusotoError<UpdateSqlInjectionMatchSetError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>ActivatedRule</a> objects in a <code>WebACL</code>. Each <code>Rule</code> identifies web requests that you want to allow, block, or count. When you update a <code>WebACL</code>, you specify the following values:</p> <ul> <li> <p>A default action for the <code>WebACL</code>, either <code>ALLOW</code> or <code>BLOCK</code>. AWS WAF performs the default action if a request doesn&#39;t match the criteria in any of the <code>Rules</code> in a <code>WebACL</code>.</p> </li> <li> <p>The <code>Rules</code> that you want to add or delete. If you want to replace one <code>Rule</code> with another, you delete the existing <code>Rule</code> and add the new one.</p> </li> <li> <p>For each <code>Rule</code>, whether you want AWS WAF to allow requests, block requests, or count requests that match the conditions in the <code>Rule</code>.</p> </li> <li> <p>The order in which you want AWS WAF to evaluate the <code>Rules</code> in a <code>WebACL</code>. If you add more than one <code>Rule</code> to a <code>WebACL</code>, AWS WAF evaluates each request against the <code>Rules</code> in order based on the value of <code>Priority</code>. (The <code>Rule</code> that has the lowest value for <code>Priority</code> is evaluated first.) When a web request matches all the predicates (such as <code>ByteMatchSets</code> and <code>IPSets</code>) in a <code>Rule</code>, AWS WAF immediately takes the corresponding action, allow or block, and doesn&#39;t evaluate the request against the remaining <code>Rules</code> in the <code>WebACL</code>, if any. </p> </li> </ul> <p>To create and configure a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in <code>Rules</code>. For more information, see <a>CreateByteMatchSet</a>, <a>UpdateByteMatchSet</a>, <a>CreateIPSet</a>, <a>UpdateIPSet</a>, <a>CreateSqlInjectionMatchSet</a>, and <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>WebACL</code>. For more information, see <a>CreateRule</a> and <a>UpdateRule</a>.</p> </li> <li> <p>Create a <code>WebACL</code>. See <a>CreateWebACL</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateWebACL</a> request.</p> </li> <li> <p>Submit an <code>UpdateWebACL</code> request to specify the <code>Rules</code> that you want to include in the <code>WebACL</code>, to specify the default action, and to associate the <code>WebACL</code> with a CloudFront distribution. </p> <p>The <code>ActivatedRule</code> can be a rule group. If you specify a rule group as your <code>ActivatedRule</code> , you can exclude specific rules from that rule group.</p> <p>If you already have a rule group associated with a web ACL and want to submit an <code>UpdateWebACL</code> request to exclude certain rules from that rule group, you must first remove the rule group from the web ACL, the re-insert it again, specifying the excluded rules. For details, see <a>ActivatedRule$ExcludedRules</a> . </p> </li> </ol> <p>Be aware that if you try to add a RATE_BASED rule to a web ACL without setting the rule type when first creating the rule, the <a>UpdateWebACL</a> request will fail because the request tries to add a REGULAR rule (the default rule type) with the specified ID, which does not exist. </p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_web_acl(
        &self,
        input: UpdateWebACLRequest,
    ) -> Result<UpdateWebACLResponse, RusotoError<UpdateWebACLError>>;

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>XssMatchTuple</a> objects (filters) in an <a>XssMatchSet</a>. For each <code>XssMatchTuple</code> object, you specify the following values:</p> <ul> <li> <p> <code>Action</code>: Whether to insert the object into or delete the object from the array. To change an <code>XssMatchTuple</code>, you delete the existing object and add a new one.</p> </li> <li> <p> <code>FieldToMatch</code>: The part of web requests that you want AWS WAF to inspect and, if you want AWS WAF to inspect a header or custom query parameter, the name of the header or parameter.</p> </li> <li> <p> <code>TextTransformation</code>: Which text transformation, if any, to perform on the web request before inspecting the request for cross-site scripting attacks.</p> <p>You can only specify a single type of TextTransformation.</p> </li> </ul> <p>You use <code>XssMatchSet</code> objects to specify which CloudFront requests that you want to allow, block, or count. For example, if you&#39;re receiving requests that contain cross-site scripting attacks in the request body and you want to block the requests, you can create an <code>XssMatchSet</code> with the applicable settings, and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>XssMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateXssMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateXssMatchSet</code> request to specify the parts of web requests that you want AWS WAF to inspect for cross-site scripting attacks.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_xss_match_set(
        &self,
        input: UpdateXssMatchSetRequest,
    ) -> Result<UpdateXssMatchSetResponse, RusotoError<UpdateXssMatchSetError>>;
}
/// A client for the WAF API.
#[derive(Clone)]
pub struct WafClient {
    client: Client,
    region: region::Region,
}

impl WafClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> WafClient {
        WafClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> WafClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        WafClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> WafClient {
        WafClient { client, region }
    }
}

#[async_trait]
impl Waf for WafClient {
    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <code>ByteMatchSet</code>. You then use <a>UpdateByteMatchSet</a> to identify the part of a web request that you want AWS WAF to inspect, such as the values of the <code>User-Agent</code> header or the query string. For example, you can create a <code>ByteMatchSet</code> that matches any requests with <code>User-Agent</code> headers that contain the string <code>BadBot</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateByteMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateByteMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateByteMatchSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateByteMatchSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_byte_match_set(
        &self,
        input: CreateByteMatchSetRequest,
    ) -> Result<CreateByteMatchSetResponse, RusotoError<CreateByteMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.CreateByteMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateByteMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateByteMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates an <a>GeoMatchSet</a>, which you use to specify which web requests you want to allow or block based on the country that the requests originate from. For example, if you&#39;re receiving a lot of requests from one or more countries and you want to block the requests, you can create an <code>GeoMatchSet</code> that contains those countries and then configure AWS WAF to block the requests. </p> <p>To create and configure a <code>GeoMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateGeoMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateGeoMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateGeoMatchSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateGeoMatchSetSet</code> request to specify the countries that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_geo_match_set(
        &self,
        input: CreateGeoMatchSetRequest,
    ) -> Result<CreateGeoMatchSetResponse, RusotoError<CreateGeoMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.CreateGeoMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateGeoMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateGeoMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates an <a>IPSet</a>, which you use to specify which web requests that you want to allow or block based on the IP addresses that the requests originate from. For example, if you&#39;re receiving a lot of requests from one or more individual IP addresses or one or more ranges of IP addresses and you want to block the requests, you can create an <code>IPSet</code> that contains those IP addresses and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>IPSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateIPSet</code> request.</p> </li> <li> <p>Submit a <code>CreateIPSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateIPSet</code> request to specify the IP addresses that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_ip_set(
        &self,
        input: CreateIPSetRequest,
    ) -> Result<CreateIPSetResponse, RusotoError<CreateIPSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.CreateIPSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateIPSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateIPSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <a>RateBasedRule</a>. The <code>RateBasedRule</code> contains a <code>RateLimit</code>, which specifies the maximum number of requests that AWS WAF allows from a specified IP address in a five-minute period. The <code>RateBasedRule</code> also contains the <code>IPSet</code> objects, <code>ByteMatchSet</code> objects, and other predicates that identify the requests that you want to count or block if these requests exceed the <code>RateLimit</code>.</p> <p>If you add more than one predicate to a <code>RateBasedRule</code>, a request not only must exceed the <code>RateLimit</code>, but it also must match all the conditions to be counted or blocked. For example, suppose you add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 1,000.</p> <p>You then add the <code>RateBasedRule</code> to a <code>WebACL</code> and specify that you want to block requests that meet the conditions in the rule. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>. Further, requests that match these two conditions must be received at a rate of more than 1,000 requests every five minutes. If both conditions are met and the rate is exceeded, AWS WAF blocks the requests. If the rate drops below 1,000 for a five-minute period, AWS WAF no longer blocks the requests.</p> <p>As a second example, suppose you want to limit requests to a particular page on your site. To do this, you could add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>A <code>ByteMatchSet</code> with <code>FieldToMatch</code> of <code>URI</code> </p> </li> <li> <p>A <code>PositionalConstraint</code> of <code>STARTS_WITH</code> </p> </li> <li> <p>A <code>TargetString</code> of <code>login</code> </p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 1,000.</p> <p>By adding this <code>RateBasedRule</code> to a <code>WebACL</code>, you could limit requests to your login page without affecting the rest of your site.</p> <p>To create and configure a <code>RateBasedRule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the rule. For more information, see <a>CreateByteMatchSet</a>, <a>CreateIPSet</a>, and <a>CreateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRule</code> request.</p> </li> <li> <p>Submit a <code>CreateRateBasedRule</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRateBasedRule</code> request to specify the predicates that you want to include in the rule.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>RateBasedRule</code>. For more information, see <a>CreateWebACL</a>.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_rate_based_rule(
        &self,
        input: CreateRateBasedRuleRequest,
    ) -> Result<CreateRateBasedRuleResponse, RusotoError<CreateRateBasedRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.CreateRateBasedRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateRateBasedRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateRateBasedRuleResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <a>RegexMatchSet</a>. You then use <a>UpdateRegexMatchSet</a> to identify the part of a web request that you want AWS WAF to inspect, such as the values of the <code>User-Agent</code> header or the query string. For example, you can create a <code>RegexMatchSet</code> that contains a <code>RegexMatchTuple</code> that looks for any requests with <code>User-Agent</code> headers that match a <code>RegexPatternSet</code> with pattern <code>B[a@]dB[o0]t</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>RegexMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRegexMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateRegexMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexMatchSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateRegexMatchSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value, using a <code>RegexPatternSet</code>, that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_regex_match_set(
        &self,
        input: CreateRegexMatchSetRequest,
    ) -> Result<CreateRegexMatchSetResponse, RusotoError<CreateRegexMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.CreateRegexMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateRegexMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateRegexMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <code>RegexPatternSet</code>. You then use <a>UpdateRegexPatternSet</a> to specify the regular expression (regex) pattern that you want AWS WAF to search for, such as <code>B[a@]dB[o0]t</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>RegexPatternSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRegexPatternSet</code> request.</p> </li> <li> <p>Submit a <code>CreateRegexPatternSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexPatternSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateRegexPatternSet</a> request to specify the string that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_regex_pattern_set(
        &self,
        input: CreateRegexPatternSetRequest,
    ) -> Result<CreateRegexPatternSetResponse, RusotoError<CreateRegexPatternSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.CreateRegexPatternSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateRegexPatternSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateRegexPatternSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <code>Rule</code>, which contains the <code>IPSet</code> objects, <code>ByteMatchSet</code> objects, and other predicates that identify the requests that you want to block. If you add more than one predicate to a <code>Rule</code>, a request must match all of the specifications to be allowed or blocked. For example, suppose that you add the following to a <code>Rule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>You then add the <code>Rule</code> to a <code>WebACL</code> and specify that you want to blocks requests that satisfy the <code>Rule</code>. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>.</p> <p>To create and configure a <code>Rule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the <code>Rule</code>. For more information, see <a>CreateByteMatchSet</a>, <a>CreateIPSet</a>, and <a>CreateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateRule</code> request.</p> </li> <li> <p>Submit a <code>CreateRule</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRule</code> request to specify the predicates that you want to include in the <code>Rule</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>Rule</code>. For more information, see <a>CreateWebACL</a>.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_rule(
        &self,
        input: CreateRuleRequest,
    ) -> Result<CreateRuleResponse, RusotoError<CreateRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.CreateRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateRuleResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <code>RuleGroup</code>. A rule group is a collection of predefined rules that you add to a web ACL. You use <a>UpdateRuleGroup</a> to add rules to the rule group.</p> <p>Rule groups are subject to the following limits:</p> <ul> <li> <p>Three rule groups per account. You can request an increase to this limit by contacting customer support.</p> </li> <li> <p>One rule group per web ACL.</p> </li> <li> <p>Ten rules per rule group.</p> </li> </ul> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_rule_group(
        &self,
        input: CreateRuleGroupRequest,
    ) -> Result<CreateRuleGroupResponse, RusotoError<CreateRuleGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.CreateRuleGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateRuleGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateRuleGroupResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <code>SizeConstraintSet</code>. You then use <a>UpdateSizeConstraintSet</a> to identify the part of a web request that you want AWS WAF to check for length, such as the length of the <code>User-Agent</code> header or the length of the query string. For example, you can create a <code>SizeConstraintSet</code> that matches any requests that have a query string that is longer than 100 bytes. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit a <code>CreateSizeConstraintSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit an <a>UpdateSizeConstraintSet</a> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_size_constraint_set(
        &self,
        input: CreateSizeConstraintSetRequest,
    ) -> Result<CreateSizeConstraintSetResponse, RusotoError<CreateSizeConstraintSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.CreateSizeConstraintSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateSizeConstraintSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateSizeConstraintSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <a>SqlInjectionMatchSet</a>, which you use to allow, block, or count requests that contain snippets of SQL code in a specified part of web requests. AWS WAF searches for character sequences that are likely to be malicious strings.</p> <p>To create and configure a <code>SqlInjectionMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateSqlInjectionMatchSet</a> request.</p> </li> <li> <p>Submit an <a>UpdateSqlInjectionMatchSet</a> request to specify the parts of web requests in which you want to allow, block, or count malicious SQL code.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_sql_injection_match_set(
        &self,
        input: CreateSqlInjectionMatchSetRequest,
    ) -> Result<CreateSqlInjectionMatchSetResponse, RusotoError<CreateSqlInjectionMatchSetError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.CreateSqlInjectionMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateSqlInjectionMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateSqlInjectionMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates a <code>WebACL</code>, which contains the <code>Rules</code> that identify the CloudFront web requests that you want to allow, block, or count. AWS WAF evaluates <code>Rules</code> in order based on the value of <code>Priority</code> for each <code>Rule</code>.</p> <p>You also specify a default action, either <code>ALLOW</code> or <code>BLOCK</code>. If a web request doesn&#39;t match any of the <code>Rules</code> in a <code>WebACL</code>, AWS WAF responds to the request with the default action. </p> <p>To create and configure a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Create and update the <code>ByteMatchSet</code> objects and other predicates that you want to include in <code>Rules</code>. For more information, see <a>CreateByteMatchSet</a>, <a>UpdateByteMatchSet</a>, <a>CreateIPSet</a>, <a>UpdateIPSet</a>, <a>CreateSqlInjectionMatchSet</a>, and <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>WebACL</code>. For more information, see <a>CreateRule</a> and <a>UpdateRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateWebACL</code> request.</p> </li> <li> <p>Submit a <code>CreateWebACL</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateWebACL</a> request.</p> </li> <li> <p>Submit an <a>UpdateWebACL</a> request to specify the <code>Rules</code> that you want to include in the <code>WebACL</code>, to specify the default action, and to associate the <code>WebACL</code> with a CloudFront distribution.</p> </li> </ol> <p>For more information about how to use the AWS WAF API, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_web_acl(
        &self,
        input: CreateWebACLRequest,
    ) -> Result<CreateWebACLResponse, RusotoError<CreateWebACLError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.CreateWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateWebACLError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateWebACLResponse, _>()
    }

    /// <p>Creates an AWS CloudFormation WAFV2 template for the specified web ACL in the specified Amazon S3 bucket. Then, in CloudFormation, you create a stack from the template, to create the web ACL and its resources in AWS WAFV2. Use this to migrate your AWS WAF Classic web ACL to the latest version of AWS WAF.</p> <p>This is part of a larger migration procedure for web ACLs from AWS WAF Classic to the latest version of AWS WAF. For the full procedure, including caveats and manual steps to complete the migration and switch over to the new web ACL, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-migrating-from-classic.html">Migrating your AWS WAF Classic resources to AWS WAF</a> in the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. </p>
    async fn create_web_acl_migration_stack(
        &self,
        input: CreateWebACLMigrationStackRequest,
    ) -> Result<CreateWebACLMigrationStackResponse, RusotoError<CreateWebACLMigrationStackError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.CreateWebACLMigrationStack");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateWebACLMigrationStackError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateWebACLMigrationStackResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Creates an <a>XssMatchSet</a>, which you use to allow, block, or count requests that contain cross-site scripting attacks in the specified part of web requests. AWS WAF searches for character sequences that are likely to be malicious strings.</p> <p>To create and configure an <code>XssMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>CreateXssMatchSet</code> request.</p> </li> <li> <p>Submit a <code>CreateXssMatchSet</code> request.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateXssMatchSet</a> request.</p> </li> <li> <p>Submit an <a>UpdateXssMatchSet</a> request to specify the parts of web requests in which you want to allow, block, or count cross-site scripting attacks.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn create_xss_match_set(
        &self,
        input: CreateXssMatchSetRequest,
    ) -> Result<CreateXssMatchSetResponse, RusotoError<CreateXssMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.CreateXssMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateXssMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateXssMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>ByteMatchSet</a>. You can&#39;t delete a <code>ByteMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any <a>ByteMatchTuple</a> objects (any filters).</p> <p>If you just want to remove a <code>ByteMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>ByteMatchSet</code> to remove filters, if any. For more information, see <a>UpdateByteMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteByteMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteByteMatchSet</code> request.</p> </li> </ol></p>
    async fn delete_byte_match_set(
        &self,
        input: DeleteByteMatchSetRequest,
    ) -> Result<DeleteByteMatchSetResponse, RusotoError<DeleteByteMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.DeleteByteMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteByteMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteByteMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>GeoMatchSet</a>. You can&#39;t delete a <code>GeoMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any countries.</p> <p>If you just want to remove a <code>GeoMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>GeoMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>GeoMatchSet</code> to remove any countries. For more information, see <a>UpdateGeoMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteGeoMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteGeoMatchSet</code> request.</p> </li> </ol></p>
    async fn delete_geo_match_set(
        &self,
        input: DeleteGeoMatchSetRequest,
    ) -> Result<DeleteGeoMatchSetResponse, RusotoError<DeleteGeoMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.DeleteGeoMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteGeoMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteGeoMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes an <a>IPSet</a>. You can&#39;t delete an <code>IPSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any IP addresses.</p> <p>If you just want to remove an <code>IPSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete an <code>IPSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>IPSet</code> to remove IP address ranges, if any. For more information, see <a>UpdateIPSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteIPSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteIPSet</code> request.</p> </li> </ol></p>
    async fn delete_ip_set(
        &self,
        input: DeleteIPSetRequest,
    ) -> Result<DeleteIPSetResponse, RusotoError<DeleteIPSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.DeleteIPSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteIPSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteIPSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes the <a>LoggingConfiguration</a> from the specified web ACL.</p></p>
    async fn delete_logging_configuration(
        &self,
        input: DeleteLoggingConfigurationRequest,
    ) -> Result<DeleteLoggingConfigurationResponse, RusotoError<DeleteLoggingConfigurationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.DeleteLoggingConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteLoggingConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteLoggingConfigurationResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes an IAM policy from the specified RuleGroup.</p> <p>The user making the request must be the owner of the RuleGroup.</p></p>
    async fn delete_permission_policy(
        &self,
        input: DeletePermissionPolicyRequest,
    ) -> Result<DeletePermissionPolicyResponse, RusotoError<DeletePermissionPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.DeletePermissionPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeletePermissionPolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeletePermissionPolicyResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>RateBasedRule</a>. You can&#39;t delete a rule if it&#39;s still used in any <code>WebACL</code> objects or if it still includes any predicates, such as <code>ByteMatchSet</code> objects.</p> <p>If you just want to remove a rule from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>RateBasedRule</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>RateBasedRule</code> to remove predicates, if any. For more information, see <a>UpdateRateBasedRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRateBasedRule</code> request.</p> </li> <li> <p>Submit a <code>DeleteRateBasedRule</code> request.</p> </li> </ol></p>
    async fn delete_rate_based_rule(
        &self,
        input: DeleteRateBasedRuleRequest,
    ) -> Result<DeleteRateBasedRuleResponse, RusotoError<DeleteRateBasedRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.DeleteRateBasedRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteRateBasedRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteRateBasedRuleResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>RegexMatchSet</a>. You can&#39;t delete a <code>RegexMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any <code>RegexMatchTuples</code> objects (any filters).</p> <p>If you just want to remove a <code>RegexMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>RegexMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>RegexMatchSet</code> to remove filters, if any. For more information, see <a>UpdateRegexMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRegexMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteRegexMatchSet</code> request.</p> </li> </ol></p>
    async fn delete_regex_match_set(
        &self,
        input: DeleteRegexMatchSetRequest,
    ) -> Result<DeleteRegexMatchSetResponse, RusotoError<DeleteRegexMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.DeleteRegexMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteRegexMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteRegexMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>RegexPatternSet</a>. You can&#39;t delete a <code>RegexPatternSet</code> if it&#39;s still used in any <code>RegexMatchSet</code> or if the <code>RegexPatternSet</code> is not empty. </p></p>
    async fn delete_regex_pattern_set(
        &self,
        input: DeleteRegexPatternSetRequest,
    ) -> Result<DeleteRegexPatternSetResponse, RusotoError<DeleteRegexPatternSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.DeleteRegexPatternSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteRegexPatternSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteRegexPatternSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>Rule</a>. You can&#39;t delete a <code>Rule</code> if it&#39;s still used in any <code>WebACL</code> objects or if it still includes any predicates, such as <code>ByteMatchSet</code> objects.</p> <p>If you just want to remove a <code>Rule</code> from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>Rule</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>Rule</code> to remove predicates, if any. For more information, see <a>UpdateRule</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRule</code> request.</p> </li> <li> <p>Submit a <code>DeleteRule</code> request.</p> </li> </ol></p>
    async fn delete_rule(
        &self,
        input: DeleteRuleRequest,
    ) -> Result<DeleteRuleResponse, RusotoError<DeleteRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.DeleteRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteRuleResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>RuleGroup</a>. You can&#39;t delete a <code>RuleGroup</code> if it&#39;s still used in any <code>WebACL</code> objects or if it still includes any rules.</p> <p>If you just want to remove a <code>RuleGroup</code> from a <code>WebACL</code>, use <a>UpdateWebACL</a>.</p> <p>To permanently delete a <code>RuleGroup</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>RuleGroup</code> to remove rules, if any. For more information, see <a>UpdateRuleGroup</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteRuleGroup</code> request.</p> </li> <li> <p>Submit a <code>DeleteRuleGroup</code> request.</p> </li> </ol></p>
    async fn delete_rule_group(
        &self,
        input: DeleteRuleGroupRequest,
    ) -> Result<DeleteRuleGroupResponse, RusotoError<DeleteRuleGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.DeleteRuleGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteRuleGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteRuleGroupResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>SizeConstraintSet</a>. You can&#39;t delete a <code>SizeConstraintSet</code> if it&#39;s still used in any <code>Rules</code> or if it still includes any <a>SizeConstraint</a> objects (any filters).</p> <p>If you just want to remove a <code>SizeConstraintSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>SizeConstraintSet</code> to remove filters, if any. For more information, see <a>UpdateSizeConstraintSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteSizeConstraintSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteSizeConstraintSet</code> request.</p> </li> </ol></p>
    async fn delete_size_constraint_set(
        &self,
        input: DeleteSizeConstraintSetRequest,
    ) -> Result<DeleteSizeConstraintSetResponse, RusotoError<DeleteSizeConstraintSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.DeleteSizeConstraintSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteSizeConstraintSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteSizeConstraintSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>SqlInjectionMatchSet</a>. You can&#39;t delete a <code>SqlInjectionMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still contains any <a>SqlInjectionMatchTuple</a> objects.</p> <p>If you just want to remove a <code>SqlInjectionMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete a <code>SqlInjectionMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>SqlInjectionMatchSet</code> to remove filters, if any. For more information, see <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteSqlInjectionMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteSqlInjectionMatchSet</code> request.</p> </li> </ol></p>
    async fn delete_sql_injection_match_set(
        &self,
        input: DeleteSqlInjectionMatchSetRequest,
    ) -> Result<DeleteSqlInjectionMatchSetResponse, RusotoError<DeleteSqlInjectionMatchSetError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.DeleteSqlInjectionMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteSqlInjectionMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteSqlInjectionMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes a <a>WebACL</a>. You can&#39;t delete a <code>WebACL</code> if it still contains any <code>Rules</code>.</p> <p>To delete a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Update the <code>WebACL</code> to remove <code>Rules</code>, if any. For more information, see <a>UpdateWebACL</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteWebACL</code> request.</p> </li> <li> <p>Submit a <code>DeleteWebACL</code> request.</p> </li> </ol></p>
    async fn delete_web_acl(
        &self,
        input: DeleteWebACLRequest,
    ) -> Result<DeleteWebACLResponse, RusotoError<DeleteWebACLError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.DeleteWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteWebACLError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteWebACLResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Permanently deletes an <a>XssMatchSet</a>. You can&#39;t delete an <code>XssMatchSet</code> if it&#39;s still used in any <code>Rules</code> or if it still contains any <a>XssMatchTuple</a> objects.</p> <p>If you just want to remove an <code>XssMatchSet</code> from a <code>Rule</code>, use <a>UpdateRule</a>.</p> <p>To permanently delete an <code>XssMatchSet</code> from AWS WAF, perform the following steps:</p> <ol> <li> <p>Update the <code>XssMatchSet</code> to remove filters, if any. For more information, see <a>UpdateXssMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of a <code>DeleteXssMatchSet</code> request.</p> </li> <li> <p>Submit a <code>DeleteXssMatchSet</code> request.</p> </li> </ol></p>
    async fn delete_xss_match_set(
        &self,
        input: DeleteXssMatchSetRequest,
    ) -> Result<DeleteXssMatchSetResponse, RusotoError<DeleteXssMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.DeleteXssMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteXssMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteXssMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>ByteMatchSet</a> specified by <code>ByteMatchSetId</code>.</p></p>
    async fn get_byte_match_set(
        &self,
        input: GetByteMatchSetRequest,
    ) -> Result<GetByteMatchSetResponse, RusotoError<GetByteMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetByteMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetByteMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetByteMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>When you want to create, update, or delete AWS WAF objects, get a change token and include the change token in the create, update, or delete request. Change tokens ensure that your application doesn&#39;t submit conflicting requests to AWS WAF.</p> <p>Each create, update, or delete request must use a unique change token. If your application submits a <code>GetChangeToken</code> request and then submits a second <code>GetChangeToken</code> request before submitting a create, update, or delete request, the second <code>GetChangeToken</code> request returns the same value as the first <code>GetChangeToken</code> request.</p> <p>When you use a change token in a create, update, or delete request, the status of the change token changes to <code>PENDING</code>, which indicates that AWS WAF is propagating the change to all AWS WAF servers. Use <code>GetChangeTokenStatus</code> to determine the status of your change token.</p></p>
    async fn get_change_token(
        &self,
    ) -> Result<GetChangeTokenResponse, RusotoError<GetChangeTokenError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetChangeToken");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, GetChangeTokenError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetChangeTokenResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the status of a <code>ChangeToken</code> that you got by calling <a>GetChangeToken</a>. <code>ChangeTokenStatus</code> is one of the following values:</p> <ul> <li> <p> <code>PROVISIONED</code>: You requested the change token by calling <code>GetChangeToken</code>, but you haven&#39;t used it yet in a call to create, update, or delete an AWS WAF object.</p> </li> <li> <p> <code>PENDING</code>: AWS WAF is propagating the create, update, or delete request to all AWS WAF servers.</p> </li> <li> <p> <code>INSYNC</code>: Propagation is complete.</p> </li> </ul></p>
    async fn get_change_token_status(
        &self,
        input: GetChangeTokenStatusRequest,
    ) -> Result<GetChangeTokenStatusResponse, RusotoError<GetChangeTokenStatusError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetChangeTokenStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetChangeTokenStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetChangeTokenStatusResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>GeoMatchSet</a> that is specified by <code>GeoMatchSetId</code>.</p></p>
    async fn get_geo_match_set(
        &self,
        input: GetGeoMatchSetRequest,
    ) -> Result<GetGeoMatchSetResponse, RusotoError<GetGeoMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetGeoMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetGeoMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetGeoMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>IPSet</a> that is specified by <code>IPSetId</code>.</p></p>
    async fn get_ip_set(
        &self,
        input: GetIPSetRequest,
    ) -> Result<GetIPSetResponse, RusotoError<GetIPSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetIPSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetIPSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetIPSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>LoggingConfiguration</a> for the specified web ACL.</p></p>
    async fn get_logging_configuration(
        &self,
        input: GetLoggingConfigurationRequest,
    ) -> Result<GetLoggingConfigurationResponse, RusotoError<GetLoggingConfigurationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetLoggingConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetLoggingConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetLoggingConfigurationResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the IAM policy attached to the RuleGroup.</p></p>
    async fn get_permission_policy(
        &self,
        input: GetPermissionPolicyRequest,
    ) -> Result<GetPermissionPolicyResponse, RusotoError<GetPermissionPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetPermissionPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetPermissionPolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetPermissionPolicyResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>RateBasedRule</a> that is specified by the <code>RuleId</code> that you included in the <code>GetRateBasedRule</code> request.</p></p>
    async fn get_rate_based_rule(
        &self,
        input: GetRateBasedRuleRequest,
    ) -> Result<GetRateBasedRuleResponse, RusotoError<GetRateBasedRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetRateBasedRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetRateBasedRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetRateBasedRuleResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of IP addresses currently being blocked by the <a>RateBasedRule</a> that is specified by the <code>RuleId</code>. The maximum number of managed keys that will be blocked is 10,000. If more than 10,000 addresses exceed the rate limit, the 10,000 addresses with the highest rates will be blocked.</p></p>
    async fn get_rate_based_rule_managed_keys(
        &self,
        input: GetRateBasedRuleManagedKeysRequest,
    ) -> Result<GetRateBasedRuleManagedKeysResponse, RusotoError<GetRateBasedRuleManagedKeysError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSWAF_20150824.GetRateBasedRuleManagedKeys",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetRateBasedRuleManagedKeysError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetRateBasedRuleManagedKeysResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>RegexMatchSet</a> specified by <code>RegexMatchSetId</code>.</p></p>
    async fn get_regex_match_set(
        &self,
        input: GetRegexMatchSetRequest,
    ) -> Result<GetRegexMatchSetResponse, RusotoError<GetRegexMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetRegexMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetRegexMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetRegexMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>RegexPatternSet</a> specified by <code>RegexPatternSetId</code>.</p></p>
    async fn get_regex_pattern_set(
        &self,
        input: GetRegexPatternSetRequest,
    ) -> Result<GetRegexPatternSetResponse, RusotoError<GetRegexPatternSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetRegexPatternSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetRegexPatternSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetRegexPatternSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>Rule</a> that is specified by the <code>RuleId</code> that you included in the <code>GetRule</code> request.</p></p>
    async fn get_rule(
        &self,
        input: GetRuleRequest,
    ) -> Result<GetRuleResponse, RusotoError<GetRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetRuleResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>RuleGroup</a> that is specified by the <code>RuleGroupId</code> that you included in the <code>GetRuleGroup</code> request.</p> <p>To view the rules in a rule group, use <a>ListActivatedRulesInRuleGroup</a>.</p></p>
    async fn get_rule_group(
        &self,
        input: GetRuleGroupRequest,
    ) -> Result<GetRuleGroupResponse, RusotoError<GetRuleGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetRuleGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetRuleGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetRuleGroupResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Gets detailed information about a specified number of requests--a sample--that AWS WAF randomly selects from among the first 5,000 requests that your AWS resource received during a time range that you choose. You can specify a sample size of up to 500 requests, and you can specify any time range in the previous three hours.</p> <p> <code>GetSampledRequests</code> returns a time range, which is usually the time range that you specified. However, if your resource (such as a CloudFront distribution) received 5,000 requests before the specified time range elapsed, <code>GetSampledRequests</code> returns an updated time range. This new time range indicates the actual period during which AWS WAF selected the requests in the sample.</p></p>
    async fn get_sampled_requests(
        &self,
        input: GetSampledRequestsRequest,
    ) -> Result<GetSampledRequestsResponse, RusotoError<GetSampledRequestsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetSampledRequests");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetSampledRequestsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetSampledRequestsResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>SizeConstraintSet</a> specified by <code>SizeConstraintSetId</code>.</p></p>
    async fn get_size_constraint_set(
        &self,
        input: GetSizeConstraintSetRequest,
    ) -> Result<GetSizeConstraintSetResponse, RusotoError<GetSizeConstraintSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetSizeConstraintSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetSizeConstraintSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetSizeConstraintSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>SqlInjectionMatchSet</a> that is specified by <code>SqlInjectionMatchSetId</code>.</p></p>
    async fn get_sql_injection_match_set(
        &self,
        input: GetSqlInjectionMatchSetRequest,
    ) -> Result<GetSqlInjectionMatchSetResponse, RusotoError<GetSqlInjectionMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetSqlInjectionMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetSqlInjectionMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<GetSqlInjectionMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>WebACL</a> that is specified by <code>WebACLId</code>.</p></p>
    async fn get_web_acl(
        &self,
        input: GetWebACLRequest,
    ) -> Result<GetWebACLResponse, RusotoError<GetWebACLError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetWebACLError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetWebACLResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns the <a>XssMatchSet</a> that is specified by <code>XssMatchSetId</code>.</p></p>
    async fn get_xss_match_set(
        &self,
        input: GetXssMatchSetRequest,
    ) -> Result<GetXssMatchSetResponse, RusotoError<GetXssMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.GetXssMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, GetXssMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<GetXssMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>ActivatedRule</a> objects.</p></p>
    async fn list_activated_rules_in_rule_group(
        &self,
        input: ListActivatedRulesInRuleGroupRequest,
    ) -> Result<
        ListActivatedRulesInRuleGroupResponse,
        RusotoError<ListActivatedRulesInRuleGroupError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AWSWAF_20150824.ListActivatedRulesInRuleGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListActivatedRulesInRuleGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListActivatedRulesInRuleGroupResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>ByteMatchSetSummary</a> objects.</p></p>
    async fn list_byte_match_sets(
        &self,
        input: ListByteMatchSetsRequest,
    ) -> Result<ListByteMatchSetsResponse, RusotoError<ListByteMatchSetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListByteMatchSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListByteMatchSetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListByteMatchSetsResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>GeoMatchSetSummary</a> objects in the response.</p></p>
    async fn list_geo_match_sets(
        &self,
        input: ListGeoMatchSetsRequest,
    ) -> Result<ListGeoMatchSetsResponse, RusotoError<ListGeoMatchSetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListGeoMatchSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListGeoMatchSetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListGeoMatchSetsResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>IPSetSummary</a> objects in the response.</p></p>
    async fn list_ip_sets(
        &self,
        input: ListIPSetsRequest,
    ) -> Result<ListIPSetsResponse, RusotoError<ListIPSetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListIPSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListIPSetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListIPSetsResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>LoggingConfiguration</a> objects.</p></p>
    async fn list_logging_configurations(
        &self,
        input: ListLoggingConfigurationsRequest,
    ) -> Result<ListLoggingConfigurationsResponse, RusotoError<ListLoggingConfigurationsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListLoggingConfigurations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListLoggingConfigurationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListLoggingConfigurationsResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>RuleSummary</a> objects.</p></p>
    async fn list_rate_based_rules(
        &self,
        input: ListRateBasedRulesRequest,
    ) -> Result<ListRateBasedRulesResponse, RusotoError<ListRateBasedRulesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListRateBasedRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListRateBasedRulesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListRateBasedRulesResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>RegexMatchSetSummary</a> objects.</p></p>
    async fn list_regex_match_sets(
        &self,
        input: ListRegexMatchSetsRequest,
    ) -> Result<ListRegexMatchSetsResponse, RusotoError<ListRegexMatchSetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListRegexMatchSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListRegexMatchSetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListRegexMatchSetsResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>RegexPatternSetSummary</a> objects.</p></p>
    async fn list_regex_pattern_sets(
        &self,
        input: ListRegexPatternSetsRequest,
    ) -> Result<ListRegexPatternSetsResponse, RusotoError<ListRegexPatternSetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListRegexPatternSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListRegexPatternSetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListRegexPatternSetsResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>RuleGroup</a> objects.</p></p>
    async fn list_rule_groups(
        &self,
        input: ListRuleGroupsRequest,
    ) -> Result<ListRuleGroupsResponse, RusotoError<ListRuleGroupsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListRuleGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListRuleGroupsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListRuleGroupsResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>RuleSummary</a> objects.</p></p>
    async fn list_rules(
        &self,
        input: ListRulesRequest,
    ) -> Result<ListRulesResponse, RusotoError<ListRulesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListRulesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListRulesResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>SizeConstraintSetSummary</a> objects.</p></p>
    async fn list_size_constraint_sets(
        &self,
        input: ListSizeConstraintSetsRequest,
    ) -> Result<ListSizeConstraintSetsResponse, RusotoError<ListSizeConstraintSetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListSizeConstraintSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListSizeConstraintSetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListSizeConstraintSetsResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>SqlInjectionMatchSet</a> objects.</p></p>
    async fn list_sql_injection_match_sets(
        &self,
        input: ListSqlInjectionMatchSetsRequest,
    ) -> Result<ListSqlInjectionMatchSetsResponse, RusotoError<ListSqlInjectionMatchSetsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListSqlInjectionMatchSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListSqlInjectionMatchSetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListSqlInjectionMatchSetsResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>RuleGroup</a> objects that you are subscribed to.</p></p>
    async fn list_subscribed_rule_groups(
        &self,
        input: ListSubscribedRuleGroupsRequest,
    ) -> Result<ListSubscribedRuleGroupsResponse, RusotoError<ListSubscribedRuleGroupsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListSubscribedRuleGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListSubscribedRuleGroupsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListSubscribedRuleGroupsResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Retrieves the tags associated with the specified AWS resource. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. For example, you might set the tag key to &quot;customer&quot; and the value to the customer name or ID. You can specify one or more tags to add to each AWS resource, up to 50 tags for a resource.</p> <p>Tagging is only available through the API, SDKs, and CLI. You can&#39;t manage or view tags through the AWS WAF Classic console. You can tag the AWS resources that you manage through AWS WAF Classic: web ACLs, rule groups, and rules. </p></p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>WebACLSummary</a> objects in the response.</p></p>
    async fn list_web_ac_ls(
        &self,
        input: ListWebACLsRequest,
    ) -> Result<ListWebACLsResponse, RusotoError<ListWebACLsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListWebACLs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListWebACLsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListWebACLsResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Returns an array of <a>XssMatchSet</a> objects.</p></p>
    async fn list_xss_match_sets(
        &self,
        input: ListXssMatchSetsRequest,
    ) -> Result<ListXssMatchSetsResponse, RusotoError<ListXssMatchSetsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.ListXssMatchSets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListXssMatchSetsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListXssMatchSetsResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Associates a <a>LoggingConfiguration</a> with a specified web ACL.</p> <p>You can access information about all traffic that AWS WAF inspects using the following steps:</p> <ol> <li> <p>Create an Amazon Kinesis Data Firehose. </p> <p>Create the data firehose with a PUT source and in the region that you are operating. However, if you are capturing logs for Amazon CloudFront, always create the firehose in US East (N. Virginia). </p> <note> <p>Do not create the data firehose using a <code>Kinesis stream</code> as your source.</p> </note> </li> <li> <p>Associate that firehose to your web ACL using a <code>PutLoggingConfiguration</code> request.</p> </li> </ol> <p>When you successfully enable logging using a <code>PutLoggingConfiguration</code> request, AWS WAF will create a service linked role with the necessary permissions to write logs to the Amazon Kinesis Data Firehose. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/logging.html">Logging Web ACL Traffic Information</a> in the <i>AWS WAF Developer Guide</i>.</p></p>
    async fn put_logging_configuration(
        &self,
        input: PutLoggingConfigurationRequest,
    ) -> Result<PutLoggingConfigurationResponse, RusotoError<PutLoggingConfigurationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.PutLoggingConfiguration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutLoggingConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<PutLoggingConfigurationResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Attaches an IAM policy to the specified resource. The only supported use for this action is to share a RuleGroup across accounts.</p> <p>The <code>PutPermissionPolicy</code> is subject to the following restrictions:</p> <ul> <li> <p>You can attach only one policy with each <code>PutPermissionPolicy</code> request.</p> </li> <li> <p>The policy must include an <code>Effect</code>, <code>Action</code> and <code>Principal</code>. </p> </li> <li> <p> <code>Effect</code> must specify <code>Allow</code>.</p> </li> <li> <p>The <code>Action</code> in the policy must be <code>waf:UpdateWebACL</code>, <code>waf-regional:UpdateWebACL</code>, <code>waf:GetRuleGroup</code> and <code>waf-regional:GetRuleGroup</code> . Any extra or wildcard actions in the policy will be rejected.</p> </li> <li> <p>The policy cannot include a <code>Resource</code> parameter.</p> </li> <li> <p>The ARN in the request must be a valid WAF RuleGroup ARN and the RuleGroup must exist in the same region.</p> </li> <li> <p>The user making the request must be the owner of the RuleGroup.</p> </li> <li> <p>Your policy must be composed using IAM Policy version 2012-10-17.</p> </li> </ul> <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html">IAM Policies</a>. </p> <p>An example of a valid policy parameter is shown in the Examples section below.</p></p>
    async fn put_permission_policy(
        &self,
        input: PutPermissionPolicyRequest,
    ) -> Result<PutPermissionPolicyResponse, RusotoError<PutPermissionPolicyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.PutPermissionPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutPermissionPolicyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutPermissionPolicyResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Associates tags with the specified AWS resource. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. For example, you might set the tag key to &quot;customer&quot; and the value to the customer name or ID. You can specify one or more tags to add to each AWS resource, up to 50 tags for a resource.</p> <p>Tagging is only available through the API, SDKs, and CLI. You can&#39;t manage or view tags through the AWS WAF Classic console. You can use this action to tag the AWS resources that you manage through AWS WAF Classic: web ACLs, rule groups, and rules. </p></p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p/></p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>ByteMatchTuple</a> objects (filters) in a <a>ByteMatchSet</a>. For each <code>ByteMatchTuple</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>ByteMatchSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to inspect, such as a query string or the value of the <code>User-Agent</code> header. </p> </li> <li> <p>The bytes (typically a string that corresponds with ASCII characters) that you want AWS WAF to look for. For more information, including how you specify the values for the AWS WAF API and the AWS CLI or SDKs, see <code>TargetString</code> in the <a>ByteMatchTuple</a> data type. </p> </li> <li> <p>Where to look, such as at the beginning or the end of a query string.</p> </li> <li> <p>Whether to perform any conversions on the request, such as converting it to lowercase, before inspecting it for the specified string.</p> </li> </ul> <p>For example, you can add a <code>ByteMatchSetUpdate</code> object that matches web requests in which <code>User-Agent</code> headers contain the string <code>BadBot</code>. You can then configure AWS WAF to block those requests.</p> <p>To create and configure a <code>ByteMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>ByteMatchSet.</code> For more information, see <a>CreateByteMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateByteMatchSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateByteMatchSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_byte_match_set(
        &self,
        input: UpdateByteMatchSetRequest,
    ) -> Result<UpdateByteMatchSetResponse, RusotoError<UpdateByteMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.UpdateByteMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateByteMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateByteMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>GeoMatchConstraint</a> objects in an <code>GeoMatchSet</code>. For each <code>GeoMatchConstraint</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change an <code>GeoMatchConstraint</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The <code>Type</code>. The only valid value for <code>Type</code> is <code>Country</code>.</p> </li> <li> <p>The <code>Value</code>, which is a two character code for the country to add to the <code>GeoMatchConstraint</code> object. Valid codes are listed in <a>GeoMatchConstraint$Value</a>.</p> </li> </ul> <p>To create and configure an <code>GeoMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateGeoMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateGeoMatchSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateGeoMatchSet</code> request to specify the country that you want AWS WAF to watch for.</p> </li> </ol> <p>When you update an <code>GeoMatchSet</code>, you specify the country that you want to add and/or the country that you want to delete. If you want to change a country, you delete the existing country and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_geo_match_set(
        &self,
        input: UpdateGeoMatchSetRequest,
    ) -> Result<UpdateGeoMatchSetResponse, RusotoError<UpdateGeoMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.UpdateGeoMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateGeoMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateGeoMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>IPSetDescriptor</a> objects in an <code>IPSet</code>. For each <code>IPSetDescriptor</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change an <code>IPSetDescriptor</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The IP address version, <code>IPv4</code> or <code>IPv6</code>. </p> </li> <li> <p>The IP address in CIDR notation, for example, <code>192.0.2.0/24</code> (for the range of IP addresses from <code>192.0.2.0</code> to <code>192.0.2.255</code>) or <code>192.0.2.44/32</code> (for the individual IP address <code>192.0.2.44</code>). </p> </li> </ul> <p>AWS WAF supports IPv4 address ranges: /8 and any range between /16 through /32. AWS WAF supports IPv6 address ranges: /24, /32, /48, /56, /64, and /128. For more information about CIDR notation, see the Wikipedia entry <a href="https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing">Classless Inter-Domain Routing</a>.</p> <p>IPv6 addresses can be represented using any of the following formats:</p> <ul> <li> <p>1111:0000:0000:0000:0000:0000:0000:0111/128</p> </li> <li> <p>1111:0:0:0:0:0:0:0111/128</p> </li> <li> <p>1111::0111/128</p> </li> <li> <p>1111::111/128</p> </li> </ul> <p>You use an <code>IPSet</code> to specify which web requests you want to allow or block based on the IP addresses that the requests originated from. For example, if you&#39;re receiving a lot of requests from one or a small number of IP addresses and you want to block the requests, you can create an <code>IPSet</code> that specifies those IP addresses, and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>IPSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateIPSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateIPSet</code> request to specify the IP addresses that you want AWS WAF to watch for.</p> </li> </ol> <p>When you update an <code>IPSet</code>, you specify the IP addresses that you want to add and/or the IP addresses that you want to delete. If you want to change an IP address, you delete the existing IP address and add the new one.</p> <p>You can insert a maximum of 1000 addresses in a single request.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_ip_set(
        &self,
        input: UpdateIPSetRequest,
    ) -> Result<UpdateIPSetResponse, RusotoError<UpdateIPSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.UpdateIPSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateIPSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateIPSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>Predicate</a> objects in a rule and updates the <code>RateLimit</code> in the rule. </p> <p>Each <code>Predicate</code> object identifies a predicate, such as a <a>ByteMatchSet</a> or an <a>IPSet</a>, that specifies the web requests that you want to block or count. The <code>RateLimit</code> specifies the number of requests every five minutes that triggers the rule.</p> <p>If you add more than one predicate to a <code>RateBasedRule</code>, a request must match all the predicates and exceed the <code>RateLimit</code> to be counted or blocked. For example, suppose you add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44/32</code> </p> </li> <li> <p>A <code>ByteMatchSet</code> that matches <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 1,000.</p> <p>You then add the <code>RateBasedRule</code> to a <code>WebACL</code> and specify that you want to block requests that satisfy the rule. For a request to be blocked, it must come from the IP address 192.0.2.44 <i>and</i> the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code>. Further, requests that match these two conditions much be received at a rate of more than 1,000 every five minutes. If the rate drops below this limit, AWS WAF no longer blocks the requests.</p> <p>As a second example, suppose you want to limit requests to a particular page on your site. To do this, you could add the following to a <code>RateBasedRule</code>:</p> <ul> <li> <p>A <code>ByteMatchSet</code> with <code>FieldToMatch</code> of <code>URI</code> </p> </li> <li> <p>A <code>PositionalConstraint</code> of <code>STARTS_WITH</code> </p> </li> <li> <p>A <code>TargetString</code> of <code>login</code> </p> </li> </ul> <p>Further, you specify a <code>RateLimit</code> of 1,000.</p> <p>By adding this <code>RateBasedRule</code> to a <code>WebACL</code>, you could limit requests to your login page without affecting the rest of your site.</p></p>
    async fn update_rate_based_rule(
        &self,
        input: UpdateRateBasedRuleRequest,
    ) -> Result<UpdateRateBasedRuleResponse, RusotoError<UpdateRateBasedRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.UpdateRateBasedRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateRateBasedRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateRateBasedRuleResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>RegexMatchTuple</a> objects (filters) in a <a>RegexMatchSet</a>. For each <code>RegexMatchSetUpdate</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>RegexMatchSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to inspectupdate, such as a query string or the value of the <code>User-Agent</code> header. </p> </li> <li> <p>The identifier of the pattern (a regular expression) that you want AWS WAF to look for. For more information, see <a>RegexPatternSet</a>. </p> </li> <li> <p>Whether to perform any conversions on the request, such as converting it to lowercase, before inspecting it for the specified string.</p> </li> </ul> <p> For example, you can create a <code>RegexPatternSet</code> that matches any requests with <code>User-Agent</code> headers that contain the string <code>B[a@]dB[o0]t</code>. You can then configure AWS WAF to reject those requests.</p> <p>To create and configure a <code>RegexMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>RegexMatchSet.</code> For more information, see <a>CreateRegexMatchSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexMatchSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateRegexMatchSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the identifier of the <code>RegexPatternSet</code> that contain the regular expression patters you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_regex_match_set(
        &self,
        input: UpdateRegexMatchSetRequest,
    ) -> Result<UpdateRegexMatchSetResponse, RusotoError<UpdateRegexMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.UpdateRegexMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateRegexMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateRegexMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <code>RegexPatternString</code> objects in a <a>RegexPatternSet</a>. For each <code>RegexPatternString</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the <code>RegexPatternString</code>.</p> </li> <li> <p>The regular expression pattern that you want to insert or delete. For more information, see <a>RegexPatternSet</a>. </p> </li> </ul> <p> For example, you can create a <code>RegexPatternString</code> such as <code>B[a@]dB[o0]t</code>. AWS WAF will match this <code>RegexPatternString</code> to:</p> <ul> <li> <p>BadBot</p> </li> <li> <p>BadB0t</p> </li> <li> <p>B@dBot</p> </li> <li> <p>B@dB0t</p> </li> </ul> <p>To create and configure a <code>RegexPatternSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>RegexPatternSet.</code> For more information, see <a>CreateRegexPatternSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateRegexPatternSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateRegexPatternSet</code> request to specify the regular expression pattern that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_regex_pattern_set(
        &self,
        input: UpdateRegexPatternSetRequest,
    ) -> Result<UpdateRegexPatternSetResponse, RusotoError<UpdateRegexPatternSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.UpdateRegexPatternSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateRegexPatternSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateRegexPatternSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>Predicate</a> objects in a <code>Rule</code>. Each <code>Predicate</code> object identifies a predicate, such as a <a>ByteMatchSet</a> or an <a>IPSet</a>, that specifies the web requests that you want to allow, block, or count. If you add more than one predicate to a <code>Rule</code>, a request must match all of the specifications to be allowed, blocked, or counted. For example, suppose that you add the following to a <code>Rule</code>: </p> <ul> <li> <p>A <code>ByteMatchSet</code> that matches the value <code>BadBot</code> in the <code>User-Agent</code> header</p> </li> <li> <p>An <code>IPSet</code> that matches the IP address <code>192.0.2.44</code> </p> </li> </ul> <p>You then add the <code>Rule</code> to a <code>WebACL</code> and specify that you want to block requests that satisfy the <code>Rule</code>. For a request to be blocked, the <code>User-Agent</code> header in the request must contain the value <code>BadBot</code> <i>and</i> the request must originate from the IP address 192.0.2.44.</p> <p>To create and configure a <code>Rule</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in the <code>Rule</code>.</p> </li> <li> <p>Create the <code>Rule</code>. See <a>CreateRule</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRule</a> request.</p> </li> <li> <p>Submit an <code>UpdateRule</code> request to add predicates to the <code>Rule</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>Rule</code>. See <a>CreateWebACL</a>.</p> </li> </ol> <p>If you want to replace one <code>ByteMatchSet</code> or <code>IPSet</code> with another, you delete the existing one and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_rule(
        &self,
        input: UpdateRuleRequest,
    ) -> Result<UpdateRuleResponse, RusotoError<UpdateRuleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.UpdateRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateRuleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateRuleResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>ActivatedRule</a> objects in a <code>RuleGroup</code>.</p> <p>You can only insert <code>REGULAR</code> rules into a rule group.</p> <p>You can have a maximum of ten rules per rule group.</p> <p>To create and configure a <code>RuleGroup</code>, perform the following steps:</p> <ol> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>RuleGroup</code>. See <a>CreateRule</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateRuleGroup</a> request.</p> </li> <li> <p>Submit an <code>UpdateRuleGroup</code> request to add <code>Rules</code> to the <code>RuleGroup</code>.</p> </li> <li> <p>Create and update a <code>WebACL</code> that contains the <code>RuleGroup</code>. See <a>CreateWebACL</a>.</p> </li> </ol> <p>If you want to replace one <code>Rule</code> with another, you delete the existing one and add the new one.</p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_rule_group(
        &self,
        input: UpdateRuleGroupRequest,
    ) -> Result<UpdateRuleGroupResponse, RusotoError<UpdateRuleGroupError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.UpdateRuleGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateRuleGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateRuleGroupResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>SizeConstraint</a> objects (filters) in a <a>SizeConstraintSet</a>. For each <code>SizeConstraint</code> object, you specify the following values: </p> <ul> <li> <p>Whether to insert or delete the object from the array. If you want to change a <code>SizeConstraintSetUpdate</code> object, you delete the existing object and add a new one.</p> </li> <li> <p>The part of a web request that you want AWS WAF to evaluate, such as the length of a query string or the length of the <code>User-Agent</code> header.</p> </li> <li> <p>Whether to perform any transformations on the request, such as converting it to lowercase, before checking its length. Note that transformations of the request body are not supported because the AWS resource forwards only the first <code>8192</code> bytes of your request to AWS WAF.</p> <p>You can only specify a single type of TextTransformation.</p> </li> <li> <p>A <code>ComparisonOperator</code> used for evaluating the selected part of the request against the specified <code>Size</code>, such as equals, greater than, less than, and so on.</p> </li> <li> <p>The length, in bytes, that you want AWS WAF to watch for in selected part of the request. The length is computed after applying the transformation.</p> </li> </ul> <p>For example, you can add a <code>SizeConstraintSetUpdate</code> object that matches web requests in which the length of the <code>User-Agent</code> header is greater than 100 bytes. You can then configure AWS WAF to block those requests.</p> <p>To create and configure a <code>SizeConstraintSet</code>, perform the following steps:</p> <ol> <li> <p>Create a <code>SizeConstraintSet.</code> For more information, see <a>CreateSizeConstraintSet</a>.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <code>UpdateSizeConstraintSet</code> request.</p> </li> <li> <p>Submit an <code>UpdateSizeConstraintSet</code> request to specify the part of the request that you want AWS WAF to inspect (for example, the header or the URI) and the value that you want AWS WAF to watch for.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_size_constraint_set(
        &self,
        input: UpdateSizeConstraintSetRequest,
    ) -> Result<UpdateSizeConstraintSetResponse, RusotoError<UpdateSizeConstraintSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.UpdateSizeConstraintSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateSizeConstraintSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateSizeConstraintSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>SqlInjectionMatchTuple</a> objects (filters) in a <a>SqlInjectionMatchSet</a>. For each <code>SqlInjectionMatchTuple</code> object, you specify the following values:</p> <ul> <li> <p> <code>Action</code>: Whether to insert the object into or delete the object from the array. To change a <code>SqlInjectionMatchTuple</code>, you delete the existing object and add a new one.</p> </li> <li> <p> <code>FieldToMatch</code>: The part of web requests that you want AWS WAF to inspect and, if you want AWS WAF to inspect a header or custom query parameter, the name of the header or parameter.</p> </li> <li> <p> <code>TextTransformation</code>: Which text transformation, if any, to perform on the web request before inspecting the request for snippets of malicious SQL code.</p> <p>You can only specify a single type of TextTransformation.</p> </li> </ul> <p>You use <code>SqlInjectionMatchSet</code> objects to specify which CloudFront requests that you want to allow, block, or count. For example, if you&#39;re receiving requests that contain snippets of SQL code in the query string and you want to block the requests, you can create a <code>SqlInjectionMatchSet</code> with the applicable settings, and then configure AWS WAF to block the requests. </p> <p>To create and configure a <code>SqlInjectionMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateSqlInjectionMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateSqlInjectionMatchSet</code> request to specify the parts of web requests that you want AWS WAF to inspect for snippets of SQL code.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_sql_injection_match_set(
        &self,
        input: UpdateSqlInjectionMatchSetRequest,
    ) -> Result<UpdateSqlInjectionMatchSetResponse, RusotoError<UpdateSqlInjectionMatchSetError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.UpdateSqlInjectionMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateSqlInjectionMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateSqlInjectionMatchSetResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>ActivatedRule</a> objects in a <code>WebACL</code>. Each <code>Rule</code> identifies web requests that you want to allow, block, or count. When you update a <code>WebACL</code>, you specify the following values:</p> <ul> <li> <p>A default action for the <code>WebACL</code>, either <code>ALLOW</code> or <code>BLOCK</code>. AWS WAF performs the default action if a request doesn&#39;t match the criteria in any of the <code>Rules</code> in a <code>WebACL</code>.</p> </li> <li> <p>The <code>Rules</code> that you want to add or delete. If you want to replace one <code>Rule</code> with another, you delete the existing <code>Rule</code> and add the new one.</p> </li> <li> <p>For each <code>Rule</code>, whether you want AWS WAF to allow requests, block requests, or count requests that match the conditions in the <code>Rule</code>.</p> </li> <li> <p>The order in which you want AWS WAF to evaluate the <code>Rules</code> in a <code>WebACL</code>. If you add more than one <code>Rule</code> to a <code>WebACL</code>, AWS WAF evaluates each request against the <code>Rules</code> in order based on the value of <code>Priority</code>. (The <code>Rule</code> that has the lowest value for <code>Priority</code> is evaluated first.) When a web request matches all the predicates (such as <code>ByteMatchSets</code> and <code>IPSets</code>) in a <code>Rule</code>, AWS WAF immediately takes the corresponding action, allow or block, and doesn&#39;t evaluate the request against the remaining <code>Rules</code> in the <code>WebACL</code>, if any. </p> </li> </ul> <p>To create and configure a <code>WebACL</code>, perform the following steps:</p> <ol> <li> <p>Create and update the predicates that you want to include in <code>Rules</code>. For more information, see <a>CreateByteMatchSet</a>, <a>UpdateByteMatchSet</a>, <a>CreateIPSet</a>, <a>UpdateIPSet</a>, <a>CreateSqlInjectionMatchSet</a>, and <a>UpdateSqlInjectionMatchSet</a>.</p> </li> <li> <p>Create and update the <code>Rules</code> that you want to include in the <code>WebACL</code>. For more information, see <a>CreateRule</a> and <a>UpdateRule</a>.</p> </li> <li> <p>Create a <code>WebACL</code>. See <a>CreateWebACL</a>.</p> </li> <li> <p>Use <code>GetChangeToken</code> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateWebACL</a> request.</p> </li> <li> <p>Submit an <code>UpdateWebACL</code> request to specify the <code>Rules</code> that you want to include in the <code>WebACL</code>, to specify the default action, and to associate the <code>WebACL</code> with a CloudFront distribution. </p> <p>The <code>ActivatedRule</code> can be a rule group. If you specify a rule group as your <code>ActivatedRule</code> , you can exclude specific rules from that rule group.</p> <p>If you already have a rule group associated with a web ACL and want to submit an <code>UpdateWebACL</code> request to exclude certain rules from that rule group, you must first remove the rule group from the web ACL, the re-insert it again, specifying the excluded rules. For details, see <a>ActivatedRule$ExcludedRules</a> . </p> </li> </ol> <p>Be aware that if you try to add a RATE_BASED rule to a web ACL without setting the rule type when first creating the rule, the <a>UpdateWebACL</a> request will fail because the request tries to add a REGULAR rule (the default rule type) with the specified ID, which does not exist. </p> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_web_acl(
        &self,
        input: UpdateWebACLRequest,
    ) -> Result<UpdateWebACLResponse, RusotoError<UpdateWebACLError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.UpdateWebACL");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateWebACLError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateWebACLResponse, _>()
    }

    /// <p><note> <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p> <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p> </note> <p>Inserts or deletes <a>XssMatchTuple</a> objects (filters) in an <a>XssMatchSet</a>. For each <code>XssMatchTuple</code> object, you specify the following values:</p> <ul> <li> <p> <code>Action</code>: Whether to insert the object into or delete the object from the array. To change an <code>XssMatchTuple</code>, you delete the existing object and add a new one.</p> </li> <li> <p> <code>FieldToMatch</code>: The part of web requests that you want AWS WAF to inspect and, if you want AWS WAF to inspect a header or custom query parameter, the name of the header or parameter.</p> </li> <li> <p> <code>TextTransformation</code>: Which text transformation, if any, to perform on the web request before inspecting the request for cross-site scripting attacks.</p> <p>You can only specify a single type of TextTransformation.</p> </li> </ul> <p>You use <code>XssMatchSet</code> objects to specify which CloudFront requests that you want to allow, block, or count. For example, if you&#39;re receiving requests that contain cross-site scripting attacks in the request body and you want to block the requests, you can create an <code>XssMatchSet</code> with the applicable settings, and then configure AWS WAF to block the requests. </p> <p>To create and configure an <code>XssMatchSet</code>, perform the following steps:</p> <ol> <li> <p>Submit a <a>CreateXssMatchSet</a> request.</p> </li> <li> <p>Use <a>GetChangeToken</a> to get the change token that you provide in the <code>ChangeToken</code> parameter of an <a>UpdateIPSet</a> request.</p> </li> <li> <p>Submit an <code>UpdateXssMatchSet</code> request to specify the parts of web requests that you want AWS WAF to inspect for cross-site scripting attacks.</p> </li> </ol> <p>For more information about how to use the AWS WAF API to allow or block HTTP requests, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/">AWS WAF Developer Guide</a>.</p></p>
    async fn update_xss_match_set(
        &self,
        input: UpdateXssMatchSetRequest,
    ) -> Result<UpdateXssMatchSetResponse, RusotoError<UpdateXssMatchSetError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AWSWAF_20150824.UpdateXssMatchSet");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateXssMatchSetError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateXssMatchSetResponse, _>()
    }
}
