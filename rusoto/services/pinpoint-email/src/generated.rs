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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>An object that contains information about a blacklisting event that impacts one of the dedicated IP addresses that is associated with your account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BlacklistEntry {
    /// <p>Additional information about the blacklisting event, as provided by the blacklist maintainer.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The time when the blacklisting event occurred, shown in Unix time format.</p>
    #[serde(rename = "listingTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listing_time: Option<f64>,
    /// <p>The name of the blacklist that the IP address appears on.</p>
    #[serde(rename = "rblName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rbl_name: Option<String>,
}

/// <p>Represents the body of the email message.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Body {
    /// <p>An object that represents the version of the message that is displayed in email clients that support HTML. HTML messages can include formatted text, hyperlinks, images, and more. </p>
    #[serde(rename = "html")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<Content>,
    /// <p>An object that represents the version of the message that is displayed in email clients that don't support HTML, or clients where the recipient has disabled HTML rendering.</p>
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Content>,
}

/// <p>An object that defines an Amazon CloudWatch destination for email events. You can use Amazon CloudWatch to monitor and gain insights on your email sending metrics.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CloudWatchDestination {
    /// <p>An array of objects that define the dimensions to use when you send email events to Amazon CloudWatch.</p>
    #[serde(rename = "dimensionConfigurations")]
    pub dimension_configurations: Vec<CloudWatchDimensionConfiguration>,
}

/// <p>An object that defines the dimension configuration to use when you send Amazon Pinpoint email events to Amazon CloudWatch.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CloudWatchDimensionConfiguration {
    /// <p><p>The default value of the dimension that is published to Amazon CloudWatch if you don&#39;t provide the value of the dimension when you send an email. This value has to meet the following criteria:</p> <ul> <li> <p>It can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>It can contain no more than 256 characters.</p> </li> </ul></p>
    #[serde(rename = "defaultDimensionValue")]
    pub default_dimension_value: String,
    /// <p><p>The name of an Amazon CloudWatch dimension associated with an email sending metric. The name has to meet the following criteria:</p> <ul> <li> <p>It can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li> <li> <p>It can contain no more than 256 characters.</p> </li> </ul></p>
    #[serde(rename = "dimensionName")]
    pub dimension_name: String,
    /// <p>The location where Amazon Pinpoint finds the value of a dimension to publish to Amazon CloudWatch. If you want Amazon Pinpoint to use the message tags that you specify using an X-SES-MESSAGE-TAGS header or a parameter to the SendEmail/SendRawEmail API, choose <code>messageTag</code>. If you want Amazon Pinpoint to use your own email headers, choose <code>emailHeader</code>. If you want Amazon Pinpoint to use link tags, choose <code>linkTags</code>.</p>
    #[serde(rename = "dimensionValueSource")]
    pub dimension_value_source: String,
}

/// <p>An object that represents the content of the email, and optionally a character set specification.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Content {
    /// <p>The character set for the content. Because of the constraints of the SMTP protocol, Amazon Pinpoint uses 7-bit ASCII by default. If the text includes characters outside of the ASCII range, you have to specify a character set. For example, you could specify <code>UTF-8</code>, <code>ISO-8859-1</code>, or <code>Shift_JIS</code>.</p>
    #[serde(rename = "charset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    /// <p>The content of the message itself.</p>
    #[serde(rename = "data")]
    pub data: String,
}

/// <p>A request to add an event destination to a configuration set.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConfigurationSetEventDestinationRequest {
    /// <p>The name of the configuration set that you want to add an event destination to.</p>
    #[serde(rename = "configurationSetName")]
    pub configuration_set_name: String,
    /// <p>An object that defines the event destination.</p>
    #[serde(rename = "eventDestination")]
    pub event_destination: EventDestinationDefinition,
    /// <p>A name that identifies the event destination within the configuration set.</p>
    #[serde(rename = "eventDestinationName")]
    pub event_destination_name: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConfigurationSetEventDestinationResponse {}

/// <p>A request to create a configuration set.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateConfigurationSetRequest {
    /// <p>The name of the configuration set.</p>
    #[serde(rename = "configurationSetName")]
    pub configuration_set_name: String,
    /// <p>An object that defines the dedicated IP pool that is used to send emails that you send using the configuration set.</p>
    #[serde(rename = "deliveryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_options: Option<DeliveryOptions>,
    /// <p>An object that defines whether or not Amazon Pinpoint collects reputation metrics for the emails that you send that use the configuration set.</p>
    #[serde(rename = "reputationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_options: Option<ReputationOptions>,
    /// <p>An object that defines whether or not Amazon Pinpoint can send email that you send using the configuration set.</p>
    #[serde(rename = "sendingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_options: Option<SendingOptions>,
    /// <p>An array of objects that define the tags (keys and values) that you want to associate with the configuration set.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>An object that defines the open and click tracking options for emails that you send using the configuration set.</p>
    #[serde(rename = "trackingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_options: Option<TrackingOptions>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateConfigurationSetResponse {}

/// <p>A request to create a new dedicated IP pool.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDedicatedIpPoolRequest {
    /// <p>The name of the dedicated IP pool.</p>
    #[serde(rename = "poolName")]
    pub pool_name: String,
    /// <p>An object that defines the tags (keys and values) that you want to associate with the pool.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDedicatedIpPoolResponse {}

/// <p>A request to perform a predictive inbox placement test. Predictive inbox placement tests can help you predict how your messages will be handled by various email providers around the world. When you perform a predictive inbox placement test, you provide a sample message that contains the content that you plan to send to your customers. Amazon Pinpoint then sends that message to special email addresses spread across several major email providers. After about 24 hours, the test is complete, and you can use the <code>GetDeliverabilityTestReport</code> operation to view the results of the test.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDeliverabilityTestReportRequest {
    /// <p>The HTML body of the message that you sent when you performed the predictive inbox placement test.</p>
    #[serde(rename = "content")]
    pub content: EmailContent,
    /// <p>The email address that the predictive inbox placement test email was sent from.</p>
    #[serde(rename = "fromEmailAddress")]
    pub from_email_address: String,
    /// <p>A unique name that helps you to identify the predictive inbox placement test when you retrieve the results.</p>
    #[serde(rename = "reportName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_name: Option<String>,
    /// <p>An array of objects that define the tags (keys and values) that you want to associate with the predictive inbox placement test.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Information about the predictive inbox placement test that you created.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDeliverabilityTestReportResponse {
    /// <p>The status of the predictive inbox placement test. If the status is <code>IN_PROGRESS</code>, then the predictive inbox placement test is currently running. Predictive inbox placement tests are usually complete within 24 hours of creating the test. If the status is <code>COMPLETE</code>, then the test is finished, and you can use the <code>GetDeliverabilityTestReport</code> to view the results of the test.</p>
    #[serde(rename = "deliverabilityTestStatus")]
    pub deliverability_test_status: String,
    /// <p>A unique string that identifies the predictive inbox placement test.</p>
    #[serde(rename = "reportId")]
    pub report_id: String,
}

/// <p>A request to begin the verification process for an email identity (an email address or domain).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEmailIdentityRequest {
    /// <p>The email address or domain that you want to verify.</p>
    #[serde(rename = "emailIdentity")]
    pub email_identity: String,
    /// <p>An array of objects that define the tags (keys and values) that you want to associate with the email identity.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>If the email identity is a domain, this object contains tokens that you can use to create a set of CNAME records. To sucessfully verify your domain, you have to add these records to the DNS configuration for your domain.</p> <p>If the email identity is an email address, this object is empty. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEmailIdentityResponse {
    /// <p>An object that contains information about the DKIM attributes for the identity. This object includes the tokens that you use to create the CNAME records that are required to complete the DKIM verification process.</p>
    #[serde(rename = "dkimAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_attributes: Option<DkimAttributes>,
    /// <p>The email identity type.</p>
    #[serde(rename = "identityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    /// <p>Specifies whether or not the identity is verified. In Amazon Pinpoint, you can only send email from verified email addresses or domains. For more information about verifying identities, see the <a href="https://docs.aws.amazon.com/pinpoint/latest/userguide/channels-email-manage-verify.html">Amazon Pinpoint User Guide</a>.</p>
    #[serde(rename = "verifiedForSendingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_for_sending_status: Option<bool>,
}

/// <p>An object that contains information about the volume of email sent on each day of the analysis period.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DailyVolume {
    /// <p>An object that contains inbox placement metrics for a specified day in the analysis period, broken out by the recipient's email provider.</p>
    #[serde(rename = "domainIspPlacements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_isp_placements: Option<Vec<DomainIspPlacement>>,
    /// <p>The date that the DailyVolume metrics apply to, in Unix time.</p>
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    /// <p>An object that contains inbox placement metrics for a specific day in the analysis period.</p>
    #[serde(rename = "volumeStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_statistics: Option<VolumeStatistics>,
}

/// <p><p>Contains information about a dedicated IP address that is associated with your Amazon Pinpoint account.</p> <p/></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DedicatedIp {
    /// <p>An IP address that is reserved for use by your Amazon Pinpoint account.</p>
    #[serde(rename = "ip")]
    pub ip: String,
    /// <p>The name of the dedicated IP pool that the IP address is associated with.</p>
    #[serde(rename = "poolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
    /// <p>Indicates how complete the dedicated IP warm-up process is. When this value equals 1, the address has completed the warm-up process and is ready for use.</p>
    #[serde(rename = "warmupPercentage")]
    pub warmup_percentage: i64,
    /// <p><p>The warm-up status of a dedicated IP address. The status can have one of the following values:</p> <ul> <li> <p> <code>IN_PROGRESS</code> – The IP address isn&#39;t ready to use because the dedicated IP warm-up process is ongoing.</p> </li> <li> <p> <code>DONE</code> – The dedicated IP warm-up process is complete, and the IP address is ready to use.</p> </li> </ul></p>
    #[serde(rename = "warmupStatus")]
    pub warmup_status: String,
}

/// <p>A request to delete an event destination from a configuration set.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConfigurationSetEventDestinationRequest {
    /// <p>The name of the configuration set that contains the event destination that you want to delete.</p>
    #[serde(rename = "configurationSetName")]
    pub configuration_set_name: String,
    /// <p>The name of the event destination that you want to delete.</p>
    #[serde(rename = "eventDestinationName")]
    pub event_destination_name: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteConfigurationSetEventDestinationResponse {}

/// <p>A request to delete a configuration set.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConfigurationSetRequest {
    /// <p>The name of the configuration set that you want to delete.</p>
    #[serde(rename = "configurationSetName")]
    pub configuration_set_name: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteConfigurationSetResponse {}

/// <p>A request to delete a dedicated IP pool.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDedicatedIpPoolRequest {
    /// <p>The name of the dedicated IP pool that you want to delete.</p>
    #[serde(rename = "poolName")]
    pub pool_name: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDedicatedIpPoolResponse {}

/// <p>A request to delete an existing email identity. When you delete an identity, you lose the ability to use Amazon Pinpoint to send email from that identity. You can restore your ability to send email by completing the verification process for the identity again.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEmailIdentityRequest {
    /// <p>The identity (that is, the email address or domain) that you want to delete from your Amazon Pinpoint account.</p>
    #[serde(rename = "emailIdentity")]
    pub email_identity: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEmailIdentityResponse {}

/// <p>An object that contains metadata related to a predictive inbox placement test.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeliverabilityTestReport {
    /// <p>The date and time when the predictive inbox placement test was created, in Unix time format.</p>
    #[serde(rename = "createDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    /// <p>The status of the predictive inbox placement test. If the status is <code>IN_PROGRESS</code>, then the predictive inbox placement test is currently running. Predictive inbox placement tests are usually complete within 24 hours of creating the test. If the status is <code>COMPLETE</code>, then the test is finished, and you can use the <code>GetDeliverabilityTestReport</code> to view the results of the test.</p>
    #[serde(rename = "deliverabilityTestStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverability_test_status: Option<String>,
    /// <p>The sender address that you specified for the predictive inbox placement test.</p>
    #[serde(rename = "fromEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    /// <p>A unique string that identifies the predictive inbox placement test.</p>
    #[serde(rename = "reportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    /// <p>A name that helps you identify a predictive inbox placement test report.</p>
    #[serde(rename = "reportName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_name: Option<String>,
    /// <p>The subject line for an email that you submitted in a predictive inbox placement test.</p>
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

/// <p>Used to associate a configuration set with a dedicated IP pool.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DeliveryOptions {
    /// <p>The name of the dedicated IP pool that you want to associate with the configuration set.</p>
    #[serde(rename = "sendingPoolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_pool_name: Option<String>,
    /// <p>Specifies whether messages that use the configuration set are required to use Transport Layer Security (TLS). If the value is <code>Require</code>, messages are only delivered if a TLS connection can be established. If the value is <code>Optional</code>, messages can be delivered in plain text if a TLS connection can't be established.</p>
    #[serde(rename = "tlsPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_policy: Option<String>,
}

/// <p>An object that describes the recipients for an email.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Destination {
    /// <p>An array that contains the email addresses of the "BCC" (blind carbon copy) recipients for the email.</p>
    #[serde(rename = "bccAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc_addresses: Option<Vec<String>>,
    /// <p>An array that contains the email addresses of the "CC" (carbon copy) recipients for the email.</p>
    #[serde(rename = "ccAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_addresses: Option<Vec<String>>,
    /// <p>An array that contains the email addresses of the "To" recipients for the email.</p>
    #[serde(rename = "toAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_addresses: Option<Vec<String>>,
}

/// <p>An object that contains information about the DKIM configuration for an email identity.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DkimAttributes {
    /// <p>If the value is <code>true</code>, then the messages that Amazon Pinpoint sends from the identity are DKIM-signed. If the value is <code>false</code>, then the messages that Amazon Pinpoint sends from the identity aren't DKIM-signed.</p>
    #[serde(rename = "signingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_enabled: Option<bool>,
    /// <p><p>Describes whether or not Amazon Pinpoint has successfully located the DKIM records in the DNS records for the domain. The status can be one of the following:</p> <ul> <li> <p> <code>PENDING</code> – Amazon Pinpoint hasn&#39;t yet located the DKIM records in the DNS configuration for the domain, but will continue to attempt to locate them.</p> </li> <li> <p> <code>SUCCESS</code> – Amazon Pinpoint located the DKIM records in the DNS configuration for the domain and determined that they&#39;re correct. Amazon Pinpoint can now send DKIM-signed email from the identity.</p> </li> <li> <p> <code>FAILED</code> – Amazon Pinpoint was unable to locate the DKIM records in the DNS settings for the domain, and won&#39;t continue to search for them.</p> </li> <li> <p> <code>TEMPORARY<em>FAILURE</code> – A temporary issue occurred, which prevented Amazon Pinpoint from determining the DKIM status for the domain.</p> </li> <li> <p> <code>NOT</em>STARTED</code> – Amazon Pinpoint hasn&#39;t yet started searching for the DKIM records in the DKIM records for the domain.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A set of unique strings that you use to create a set of CNAME records that you add to the DNS configuration for your domain. When Amazon Pinpoint detects these records in the DNS configuration for your domain, the DKIM authentication process is complete. Amazon Pinpoint usually detects these records within about 72 hours of adding them to the DNS configuration for your domain.</p>
    #[serde(rename = "tokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<String>>,
}

/// <p>An object that contains the deliverability data for a specific campaign. This data is available for a campaign only if the campaign sent email by using a domain that the Deliverability dashboard is enabled for (<code>PutDeliverabilityDashboardOption</code> operation).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainDeliverabilityCampaign {
    /// <p>The unique identifier for the campaign. Amazon Pinpoint automatically generates and assigns this identifier to a campaign. This value is not the same as the campaign identifier that Amazon Pinpoint assigns to campaigns that you create and manage by using the Amazon Pinpoint API or the Amazon Pinpoint console.</p>
    #[serde(rename = "campaignId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// <p>The percentage of email messages that were deleted by recipients, without being opened first. Due to technical limitations, this value only includes recipients who opened the message by using an email client that supports images.</p>
    #[serde(rename = "deleteRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_rate: Option<f64>,
    /// <p>The major email providers who handled the email message.</p>
    #[serde(rename = "esps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esps: Option<Vec<String>>,
    /// <p>The first time, in Unix time format, when the email message was delivered to any recipient's inbox. This value can help you determine how long it took for a campaign to deliver an email message.</p>
    #[serde(rename = "firstSeenDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_date_time: Option<f64>,
    /// <p>The verified email address that the email message was sent from.</p>
    #[serde(rename = "fromAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// <p>The URL of an image that contains a snapshot of the email message that was sent.</p>
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// <p>The number of email messages that were delivered to recipients’ inboxes.</p>
    #[serde(rename = "inboxCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_count: Option<i64>,
    /// <p>The last time, in Unix time format, when the email message was delivered to any recipient's inbox. This value can help you determine how long it took for a campaign to deliver an email message.</p>
    #[serde(rename = "lastSeenDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_date_time: Option<f64>,
    /// <p>The projected number of recipients that the email message was sent to.</p>
    #[serde(rename = "projectedVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_volume: Option<i64>,
    /// <p>The percentage of email messages that were opened and then deleted by recipients. Due to technical limitations, this value only includes recipients who opened the message by using an email client that supports images.</p>
    #[serde(rename = "readDeleteRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_delete_rate: Option<f64>,
    /// <p>The percentage of email messages that were opened by recipients. Due to technical limitations, this value only includes recipients who opened the message by using an email client that supports images.</p>
    #[serde(rename = "readRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_rate: Option<f64>,
    /// <p>The IP addresses that were used to send the email message.</p>
    #[serde(rename = "sendingIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_ips: Option<Vec<String>>,
    /// <p>The number of email messages that were delivered to recipients' spam or junk mail folders.</p>
    #[serde(rename = "spamCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_count: Option<i64>,
    /// <p>The subject line, or title, of the email message.</p>
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

/// <p>An object that contains information about the Deliverability dashboard subscription for a verified domain that you use to send email and currently has an active Deliverability dashboard subscription. If a Deliverability dashboard subscription is active for a domain, you gain access to reputation, inbox placement, and other metrics for the domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DomainDeliverabilityTrackingOption {
    /// <p>A verified domain that’s associated with your AWS account and currently has an active Deliverability dashboard subscription.</p>
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// <p>An object that contains information about the inbox placement data settings for the domain.</p>
    #[serde(rename = "inboxPlacementTrackingOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_placement_tracking_option: Option<InboxPlacementTrackingOption>,
    /// <p>The date, in Unix time format, when you enabled the Deliverability dashboard for the domain.</p>
    #[serde(rename = "subscriptionStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_start_date: Option<f64>,
}

/// <p>An object that contains inbox placement data for email sent from one of your email domains to a specific email provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DomainIspPlacement {
    /// <p>The percentage of messages that were sent from the selected domain to the specified email provider that arrived in recipients' inboxes.</p>
    #[serde(rename = "inboxPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_percentage: Option<f64>,
    /// <p>The total number of messages that were sent from the selected domain to the specified email provider that arrived in recipients' inboxes.</p>
    #[serde(rename = "inboxRawCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_raw_count: Option<i64>,
    /// <p>The name of the email provider that the inbox placement data applies to.</p>
    #[serde(rename = "ispName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name: Option<String>,
    /// <p>The percentage of messages that were sent from the selected domain to the specified email provider that arrived in recipients' spam or junk mail folders.</p>
    #[serde(rename = "spamPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_percentage: Option<f64>,
    /// <p>The total number of messages that were sent from the selected domain to the specified email provider that arrived in recipients' spam or junk mail folders.</p>
    #[serde(rename = "spamRawCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_raw_count: Option<i64>,
}

/// <p>An object that defines the entire content of the email, including the message headers and the body content. You can create a simple email message, in which you specify the subject and the text and HTML versions of the message body. You can also create raw messages, in which you specify a complete MIME-formatted message. Raw messages can include attachments and custom headers.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EmailContent {
    /// <p><p>The raw email message. The message has to meet the following criteria:</p> <ul> <li> <p>The message has to contain a header and a body, separated by one blank line.</p> </li> <li> <p>All of the required header fields must be present in the message.</p> </li> <li> <p>Each part of a multipart MIME message must be formatted properly.</p> </li> <li> <p>If you include attachments, they must be in a file format that Amazon Pinpoint supports. </p> </li> <li> <p>The entire message must be Base64 encoded.</p> </li> <li> <p>If any of the MIME parts in your message contain content that is outside of the 7-bit ASCII character range, you should encode that content to ensure that recipients&#39; email clients render the message properly.</p> </li> <li> <p>The length of any single line of text in the message can&#39;t exceed 1,000 characters. This restriction is defined in <a href="https://tools.ietf.org/html/rfc5321">RFC 5321</a>.</p> </li> </ul></p>
    #[serde(rename = "raw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<RawMessage>,
    /// <p>The simple email message. The message consists of a subject and a message body.</p>
    #[serde(rename = "simple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple: Option<Message>,
    /// <p>The template to use for the email message.</p>
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
}

/// <p>In Amazon Pinpoint, <i>events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventDestination {
    /// <p>An object that defines an Amazon CloudWatch destination for email events. You can use Amazon CloudWatch to monitor and gain insights on your email sending metrics.</p>
    #[serde(rename = "cloudWatchDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_destination: Option<CloudWatchDestination>,
    /// <p>If <code>true</code>, the event destination is enabled. When the event destination is enabled, the specified event types are sent to the destinations in this <code>EventDestinationDefinition</code>.</p> <p>If <code>false</code>, the event destination is disabled. When the event destination is disabled, events aren't sent to the specified destinations.</p>
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>An object that defines an Amazon Kinesis Data Firehose destination for email events. You can use Amazon Kinesis Data Firehose to stream data to other services, such as Amazon S3 and Amazon Redshift.</p>
    #[serde(rename = "kinesisFirehoseDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_destination: Option<KinesisFirehoseDestination>,
    /// <p>The types of events that Amazon Pinpoint sends to the specified event destinations.</p>
    #[serde(rename = "matchingEventTypes")]
    pub matching_event_types: Vec<String>,
    /// <p>A name that identifies the event destination.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>An object that defines a Amazon Pinpoint destination for email events. You can use Amazon Pinpoint events to create attributes in Amazon Pinpoint projects. You can use these attributes to create segments for your campaigns.</p>
    #[serde(rename = "pinpointDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinpoint_destination: Option<PinpointDestination>,
    /// <p>An object that defines an Amazon SNS destination for email events. You can use Amazon SNS to send notification when certain email events occur.</p>
    #[serde(rename = "snsDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_destination: Option<SnsDestination>,
}

/// <p>An object that defines the event destination. Specifically, it defines which services receive events from emails sent using the configuration set that the event destination is associated with. Also defines the types of events that are sent to the event destination.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EventDestinationDefinition {
    /// <p>An object that defines an Amazon CloudWatch destination for email events. You can use Amazon CloudWatch to monitor and gain insights on your email sending metrics.</p>
    #[serde(rename = "cloudWatchDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_destination: Option<CloudWatchDestination>,
    /// <p>If <code>true</code>, the event destination is enabled. When the event destination is enabled, the specified event types are sent to the destinations in this <code>EventDestinationDefinition</code>.</p> <p>If <code>false</code>, the event destination is disabled. When the event destination is disabled, events aren't sent to the specified destinations.</p>
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>An object that defines an Amazon Kinesis Data Firehose destination for email events. You can use Amazon Kinesis Data Firehose to stream data to other services, such as Amazon S3 and Amazon Redshift.</p>
    #[serde(rename = "kinesisFirehoseDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_destination: Option<KinesisFirehoseDestination>,
    /// <p>An array that specifies which events Amazon Pinpoint should send to the destinations in this <code>EventDestinationDefinition</code>.</p>
    #[serde(rename = "matchingEventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_event_types: Option<Vec<String>>,
    /// <p>An object that defines a Amazon Pinpoint destination for email events. You can use Amazon Pinpoint events to create attributes in Amazon Pinpoint projects. You can use these attributes to create segments for your campaigns.</p>
    #[serde(rename = "pinpointDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinpoint_destination: Option<PinpointDestination>,
    /// <p>An object that defines an Amazon SNS destination for email events. You can use Amazon SNS to send notification when certain email events occur.</p>
    #[serde(rename = "snsDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_destination: Option<SnsDestination>,
}

/// <p>A request to obtain information about the email-sending capabilities of your Amazon Pinpoint account.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAccountRequest {}

/// <p>A list of details about the email-sending capabilities of your Amazon Pinpoint account in the current AWS Region.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetAccountResponse {
    /// <p>Indicates whether or not the automatic warm-up feature is enabled for dedicated IP addresses that are associated with your account.</p>
    #[serde(rename = "dedicatedIpAutoWarmupEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_ip_auto_warmup_enabled: Option<bool>,
    /// <p><p>The reputation status of your Amazon Pinpoint account. The status can be one of the following:</p> <ul> <li> <p> <code>HEALTHY</code> – There are no reputation-related issues that currently impact your account.</p> </li> <li> <p> <code>PROBATION</code> – We&#39;ve identified some issues with your Amazon Pinpoint account. We&#39;re placing your account under review while you work on correcting these issues.</p> </li> <li> <p> <code>SHUTDOWN</code> – Your account&#39;s ability to send email is currently paused because of an issue with the email sent from your account. When you correct the issue, you can contact us and request that your account&#39;s ability to send email is resumed.</p> </li> </ul></p>
    #[serde(rename = "enforcementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement_status: Option<String>,
    /// <p>Indicates whether or not your account has production access in the current AWS Region.</p> <p>If the value is <code>false</code>, then your account is in the <i>sandbox</i>. When your account is in the sandbox, you can only send email to verified identities. Additionally, the maximum number of emails you can send in a 24-hour period (your sending quota) is 200, and the maximum number of emails you can send per second (your maximum sending rate) is 1.</p> <p>If the value is <code>true</code>, then your account has production access. When your account has production access, you can send email to any address. The sending quota and maximum sending rate for your account vary based on your specific use case.</p>
    #[serde(rename = "productionAccessEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_access_enabled: Option<bool>,
    /// <p>An object that contains information about the per-day and per-second sending limits for your Amazon Pinpoint account in the current AWS Region.</p>
    #[serde(rename = "sendQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_quota: Option<SendQuota>,
    /// <p>Indicates whether or not email sending is enabled for your Amazon Pinpoint account in the current AWS Region.</p>
    #[serde(rename = "sendingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
}

/// <p>A request to retrieve a list of the blacklists that your dedicated IP addresses appear on.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBlacklistReportsRequest {
    /// <p>A list of IP addresses that you want to retrieve blacklist information about. You can only specify the dedicated IP addresses that you use to send email using Amazon Pinpoint or Amazon SES.</p>
    #[serde(rename = "blacklistItemNames")]
    pub blacklist_item_names: Vec<String>,
}

/// <p>An object that contains information about blacklist events.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBlacklistReportsResponse {
    /// <p>An object that contains information about a blacklist that one of your dedicated IP addresses appears on.</p>
    #[serde(rename = "blacklistReport")]
    pub blacklist_report: ::std::collections::HashMap<String, Vec<BlacklistEntry>>,
}

/// <p>A request to obtain information about the event destinations for a configuration set.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConfigurationSetEventDestinationsRequest {
    /// <p>The name of the configuration set that contains the event destination.</p>
    #[serde(rename = "configurationSetName")]
    pub configuration_set_name: String,
}

/// <p>Information about an event destination for a configuration set.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConfigurationSetEventDestinationsResponse {
    /// <p>An array that includes all of the events destinations that have been configured for the configuration set.</p>
    #[serde(rename = "eventDestinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_destinations: Option<Vec<EventDestination>>,
}

/// <p>A request to obtain information about a configuration set.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetConfigurationSetRequest {
    /// <p>The name of the configuration set that you want to obtain more information about.</p>
    #[serde(rename = "configurationSetName")]
    pub configuration_set_name: String,
}

/// <p>Information about a configuration set.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetConfigurationSetResponse {
    /// <p>The name of the configuration set.</p>
    #[serde(rename = "configurationSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    /// <p>An object that defines the dedicated IP pool that is used to send emails that you send using the configuration set.</p>
    #[serde(rename = "deliveryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_options: Option<DeliveryOptions>,
    /// <p>An object that defines whether or not Amazon Pinpoint collects reputation metrics for the emails that you send that use the configuration set.</p>
    #[serde(rename = "reputationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_options: Option<ReputationOptions>,
    /// <p>An object that defines whether or not Amazon Pinpoint can send email that you send using the configuration set.</p>
    #[serde(rename = "sendingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_options: Option<SendingOptions>,
    /// <p>An array of objects that define the tags (keys and values) that are associated with the configuration set.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>An object that defines the open and click tracking options for emails that you send using the configuration set.</p>
    #[serde(rename = "trackingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_options: Option<TrackingOptions>,
}

/// <p>A request to obtain more information about a dedicated IP address.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDedicatedIpRequest {
    /// <p>The IP address that you want to obtain more information about. The value you specify has to be a dedicated IP address that's assocaited with your Amazon Pinpoint account.</p>
    #[serde(rename = "ip")]
    pub ip: String,
}

/// <p>Information about a dedicated IP address.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDedicatedIpResponse {
    /// <p>An object that contains information about a dedicated IP address.</p>
    #[serde(rename = "dedicatedIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_ip: Option<DedicatedIp>,
}

/// <p>A request to obtain more information about dedicated IP pools.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDedicatedIpsRequest {
    /// <p>A token returned from a previous call to <code>GetDedicatedIps</code> to indicate the position of the dedicated IP pool in the list of IP pools.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of results to show in a single call to <code>GetDedicatedIpsRequest</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The name of the IP pool that the dedicated IP address is associated with.</p>
    #[serde(rename = "poolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
}

/// <p>Information about the dedicated IP addresses that are associated with your Amazon Pinpoint account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDedicatedIpsResponse {
    /// <p>A list of dedicated IP addresses that are reserved for use by your Amazon Pinpoint account.</p>
    #[serde(rename = "dedicatedIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_ips: Option<Vec<DedicatedIp>>,
    /// <p>A token that indicates that there are additional dedicated IP addresses to list. To view additional addresses, issue another request to <code>GetDedicatedIps</code>, passing this token in the <code>NextToken</code> parameter.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Retrieve information about the status of the Deliverability dashboard for your Amazon Pinpoint account. When the Deliverability dashboard is enabled, you gain access to reputation, deliverability, and other metrics for the domains that you use to send email using Amazon Pinpoint. You also gain the ability to perform predictive inbox placement tests.</p> <p>When you use the Deliverability dashboard, you pay a monthly subscription charge, in addition to any other fees that you accrue by using Amazon Pinpoint. For more information about the features and cost of a Deliverability dashboard subscription, see <a href="http://aws.amazon.com/pinpoint/pricing/">Amazon Pinpoint Pricing</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeliverabilityDashboardOptionsRequest {}

/// <p>An object that shows the status of the Deliverability dashboard for your Amazon Pinpoint account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeliverabilityDashboardOptionsResponse {
    /// <p>The current status of your Deliverability dashboard subscription. If this value is <code>PENDING_EXPIRATION</code>, your subscription is scheduled to expire at the end of the current calendar month.</p>
    #[serde(rename = "accountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_status: Option<String>,
    /// <p>An array of objects, one for each verified domain that you use to send email and currently has an active Deliverability dashboard subscription that isn’t scheduled to expire at the end of the current calendar month.</p>
    #[serde(rename = "activeSubscribedDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_subscribed_domains: Option<Vec<DomainDeliverabilityTrackingOption>>,
    /// <p>Specifies whether the Deliverability dashboard is enabled for your Amazon Pinpoint account. If this value is <code>true</code>, the dashboard is enabled.</p>
    #[serde(rename = "dashboardEnabled")]
    pub dashboard_enabled: bool,
    /// <p>An array of objects, one for each verified domain that you use to send email and currently has an active Deliverability dashboard subscription that's scheduled to expire at the end of the current calendar month.</p>
    #[serde(rename = "pendingExpirationSubscribedDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_expiration_subscribed_domains: Option<Vec<DomainDeliverabilityTrackingOption>>,
    /// <p>The date, in Unix time format, when your current subscription to the Deliverability dashboard is scheduled to expire, if your subscription is scheduled to expire at the end of the current calendar month. This value is null if you have an active subscription that isn’t due to expire at the end of the month.</p>
    #[serde(rename = "subscriptionExpiryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_expiry_date: Option<f64>,
}

/// <p>A request to retrieve the results of a predictive inbox placement test.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDeliverabilityTestReportRequest {
    /// <p>A unique string that identifies the predictive inbox placement test.</p>
    #[serde(rename = "reportId")]
    pub report_id: String,
}

/// <p>The results of the predictive inbox placement test.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDeliverabilityTestReportResponse {
    /// <p>An object that contains the results of the predictive inbox placement test.</p>
    #[serde(rename = "deliverabilityTestReport")]
    pub deliverability_test_report: DeliverabilityTestReport,
    /// <p>An object that describes how the test email was handled by several email providers, including Gmail, Hotmail, Yahoo, AOL, and others.</p>
    #[serde(rename = "ispPlacements")]
    pub isp_placements: Vec<IspPlacement>,
    /// <p>An object that contains the message that you sent when you performed this predictive inbox placement test.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>An object that specifies how many test messages that were sent during the predictive inbox placement test were delivered to recipients' inboxes, how many were sent to recipients' spam folders, and how many weren't delivered.</p>
    #[serde(rename = "overallPlacement")]
    pub overall_placement: PlacementStatistics,
    /// <p>An array of objects that define the tags (keys and values) that are associated with the predictive inbox placement test.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Retrieve all the deliverability data for a specific campaign. This data is available for a campaign only if the campaign sent email by using a domain that the Deliverability dashboard is enabled for (<code>PutDeliverabilityDashboardOption</code> operation).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDomainDeliverabilityCampaignRequest {
    /// <p>The unique identifier for the campaign. Amazon Pinpoint automatically generates and assigns this identifier to a campaign. This value is not the same as the campaign identifier that Amazon Pinpoint assigns to campaigns that you create and manage by using the Amazon Pinpoint API or the Amazon Pinpoint console.</p>
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
}

/// <p>An object that contains all the deliverability data for a specific campaign. This data is available for a campaign only if the campaign sent email by using a domain that the Deliverability dashboard is enabled for (<code>PutDeliverabilityDashboardOption</code> operation).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDomainDeliverabilityCampaignResponse {
    /// <p>An object that contains the deliverability data for the campaign.</p>
    #[serde(rename = "domainDeliverabilityCampaign")]
    pub domain_deliverability_campaign: DomainDeliverabilityCampaign,
}

/// <p>A request to obtain deliverability metrics for a domain.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDomainStatisticsReportRequest {
    /// <p>The domain that you want to obtain deliverability metrics for.</p>
    #[serde(rename = "domain")]
    pub domain: String,
    /// <p>The last day (in Unix time) that you want to obtain domain deliverability metrics for. The <code>EndDate</code> that you specify has to be less than or equal to 30 days after the <code>StartDate</code>.</p>
    #[serde(rename = "endDate")]
    pub end_date: f64,
    /// <p>The first day (in Unix time) that you want to obtain domain deliverability metrics for.</p>
    #[serde(rename = "startDate")]
    pub start_date: f64,
}

/// <p>An object that includes statistics that are related to the domain that you specified.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDomainStatisticsReportResponse {
    /// <p>An object that contains deliverability metrics for the domain that you specified. This object contains data for each day, starting on the <code>StartDate</code> and ending on the <code>EndDate</code>.</p>
    #[serde(rename = "dailyVolumes")]
    pub daily_volumes: Vec<DailyVolume>,
    /// <p>An object that contains deliverability metrics for the domain that you specified. The data in this object is a summary of all of the data that was collected from the <code>StartDate</code> to the <code>EndDate</code>.</p>
    #[serde(rename = "overallVolume")]
    pub overall_volume: OverallVolume,
}

/// <p>A request to return details about an email identity.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEmailIdentityRequest {
    /// <p>The email identity that you want to retrieve details for.</p>
    #[serde(rename = "emailIdentity")]
    pub email_identity: String,
}

/// <p>Details about an email identity.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetEmailIdentityResponse {
    /// <p>An object that contains information about the DKIM attributes for the identity. This object includes the tokens that you use to create the CNAME records that are required to complete the DKIM verification process.</p>
    #[serde(rename = "dkimAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_attributes: Option<DkimAttributes>,
    /// <p>The feedback forwarding configuration for the identity.</p> <p>If the value is <code>true</code>, Amazon Pinpoint sends you email notifications when bounce or complaint events occur. Amazon Pinpoint sends this notification to the address that you specified in the Return-Path header of the original email.</p> <p>When you set this value to <code>false</code>, Amazon Pinpoint sends notifications through other mechanisms, such as by notifying an Amazon SNS topic or another event destination. You're required to have a method of tracking bounces and complaints. If you haven't set up another mechanism for receiving bounce or complaint notifications, Amazon Pinpoint sends an email notification when these events occur (even if this setting is disabled).</p>
    #[serde(rename = "feedbackForwardingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_status: Option<bool>,
    /// <p>The email identity type.</p>
    #[serde(rename = "identityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    /// <p>An object that contains information about the Mail-From attributes for the email identity.</p>
    #[serde(rename = "mailFromAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_from_attributes: Option<MailFromAttributes>,
    /// <p>An array of objects that define the tags (keys and values) that are associated with the email identity.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Specifies whether or not the identity is verified. In Amazon Pinpoint, you can only send email from verified email addresses or domains. For more information about verifying identities, see the <a href="https://docs.aws.amazon.com/pinpoint/latest/userguide/channels-email-manage-verify.html">Amazon Pinpoint User Guide</a>.</p>
    #[serde(rename = "verifiedForSendingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_for_sending_status: Option<bool>,
}

/// <p>Information about an email identity.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IdentityInfo {
    /// <p>The address or domain of the identity.</p>
    #[serde(rename = "identityName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_name: Option<String>,
    /// <p><p>The email identity type. The identity type can be one of the following:</p> <ul> <li> <p> <code>EMAIL<em>ADDRESS</code> – The identity is an email address.</p> </li> <li> <p> <code>DOMAIN</code> – The identity is a domain.</p> </li> <li> <p> <code>MANAGED</em>DOMAIN</code> – The identity is a domain that is managed by AWS.</p> </li> </ul></p>
    #[serde(rename = "identityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    /// <p>Indicates whether or not you can send email from the identity.</p> <p>In Amazon Pinpoint, an identity is an email address or domain that you send email from. Before you can send email from an identity, you have to demostrate that you own the identity, and that you authorize Amazon Pinpoint to send email from that identity.</p>
    #[serde(rename = "sendingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
}

/// <p>An object that contains information about the inbox placement data settings for a verified domain that’s associated with your AWS account. This data is available only if you enabled the Deliverability dashboard for the domain (<code>PutDeliverabilityDashboardOption</code> operation).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InboxPlacementTrackingOption {
    /// <p>Specifies whether inbox placement data is being tracked for the domain.</p>
    #[serde(rename = "global")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<bool>,
    /// <p>An array of strings, one for each major email provider that the inbox placement data applies to.</p>
    #[serde(rename = "trackedIsps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracked_isps: Option<Vec<String>>,
}

/// <p>An object that describes how email sent during the predictive inbox placement test was handled by a certain email provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct IspPlacement {
    /// <p>The name of the email provider that the inbox placement data applies to.</p>
    #[serde(rename = "ispName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name: Option<String>,
    /// <p>An object that contains inbox placement metrics for a specific email provider.</p>
    #[serde(rename = "placementStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_statistics: Option<PlacementStatistics>,
}

/// <p>An object that defines an Amazon Kinesis Data Firehose destination for email events. You can use Amazon Kinesis Data Firehose to stream data to other services, such as Amazon S3 and Amazon Redshift.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KinesisFirehoseDestination {
    /// <p>The Amazon Resource Name (ARN) of the Amazon Kinesis Data Firehose stream that Amazon Pinpoint sends email events to.</p>
    #[serde(rename = "deliveryStreamArn")]
    pub delivery_stream_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that Amazon Pinpoint uses when sending email events to the Amazon Kinesis Data Firehose stream.</p>
    #[serde(rename = "iamRoleArn")]
    pub iam_role_arn: String,
}

/// <p>A request to obtain a list of configuration sets for your Amazon Pinpoint account in the current AWS Region.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListConfigurationSetsRequest {
    /// <p>A token returned from a previous call to <code>ListConfigurationSets</code> to indicate the position in the list of configuration sets.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of results to show in a single call to <code>ListConfigurationSets</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>A list of configuration sets in your Amazon Pinpoint account in the current AWS Region.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConfigurationSetsResponse {
    /// <p>An array that contains all of the configuration sets in your Amazon Pinpoint account in the current AWS Region.</p>
    #[serde(rename = "configurationSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_sets: Option<Vec<String>>,
    /// <p>A token that indicates that there are additional configuration sets to list. To view additional configuration sets, issue another request to <code>ListConfigurationSets</code>, and pass this token in the <code>NextToken</code> parameter.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A request to obtain a list of dedicated IP pools.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDedicatedIpPoolsRequest {
    /// <p>A token returned from a previous call to <code>ListDedicatedIpPools</code> to indicate the position in the list of dedicated IP pools.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of results to show in a single call to <code>ListDedicatedIpPools</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>A list of dedicated IP pools.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDedicatedIpPoolsResponse {
    /// <p>A list of all of the dedicated IP pools that are associated with your Amazon Pinpoint account.</p>
    #[serde(rename = "dedicatedIpPools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_ip_pools: Option<Vec<String>>,
    /// <p>A token that indicates that there are additional IP pools to list. To view additional IP pools, issue another request to <code>ListDedicatedIpPools</code>, passing this token in the <code>NextToken</code> parameter.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A request to list all of the predictive inbox placement tests that you've performed.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDeliverabilityTestReportsRequest {
    /// <p>A token returned from a previous call to <code>ListDeliverabilityTestReports</code> to indicate the position in the list of predictive inbox placement tests.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of results to show in a single call to <code>ListDeliverabilityTestReports</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p> <p>The value you specify has to be at least 0, and can be no more than 1000.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>A list of the predictive inbox placement test reports that are available for your account, regardless of whether or not those tests are complete.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDeliverabilityTestReportsResponse {
    /// <p>An object that contains a lists of predictive inbox placement tests that you've performed.</p>
    #[serde(rename = "deliverabilityTestReports")]
    pub deliverability_test_reports: Vec<DeliverabilityTestReport>,
    /// <p>A token that indicates that there are additional predictive inbox placement tests to list. To view additional predictive inbox placement tests, issue another request to <code>ListDeliverabilityTestReports</code>, and pass this token in the <code>NextToken</code> parameter.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Retrieve deliverability data for all the campaigns that used a specific domain to send email during a specified time range. This data is available for a domain only if you enabled the Deliverability dashboard (<code>PutDeliverabilityDashboardOption</code> operation) for the domain.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDomainDeliverabilityCampaignsRequest {
    /// <p>The last day, in Unix time format, that you want to obtain deliverability data for. This value has to be less than or equal to 30 days after the value of the <code>StartDate</code> parameter.</p>
    #[serde(rename = "endDate")]
    pub end_date: f64,
    /// <p>A token that’s returned from a previous call to the <code>ListDomainDeliverabilityCampaigns</code> operation. This token indicates the position of a campaign in the list of campaigns.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The maximum number of results to include in response to a single call to the <code>ListDomainDeliverabilityCampaigns</code> operation. If the number of results is larger than the number that you specify in this parameter, the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The first day, in Unix time format, that you want to obtain deliverability data for.</p>
    #[serde(rename = "startDate")]
    pub start_date: f64,
    /// <p>The domain to obtain deliverability data for.</p>
    #[serde(rename = "subscribedDomain")]
    pub subscribed_domain: String,
}

/// <p>An array of objects that provide deliverability data for all the campaigns that used a specific domain to send email during a specified time range. This data is available for a domain only if you enabled the Deliverability dashboard (<code>PutDeliverabilityDashboardOption</code> operation) for the domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDomainDeliverabilityCampaignsResponse {
    /// <p>An array of responses, one for each campaign that used the domain to send email during the specified time range.</p>
    #[serde(rename = "domainDeliverabilityCampaigns")]
    pub domain_deliverability_campaigns: Vec<DomainDeliverabilityCampaign>,
    /// <p>A token that’s returned from a previous call to the <code>ListDomainDeliverabilityCampaigns</code> operation. This token indicates the position of the campaign in the list of campaigns.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A request to list all of the email identities associated with your Amazon Pinpoint account. This list includes identities that you've already verified, identities that are unverified, and identities that were verified in the past, but are no longer verified.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEmailIdentitiesRequest {
    /// <p>A token returned from a previous call to <code>ListEmailIdentities</code> to indicate the position in the list of identities.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The number of results to show in a single call to <code>ListEmailIdentities</code>. If the number of results is larger than the number you specified in this parameter, then the response includes a <code>NextToken</code> element, which you can use to obtain additional results.</p> <p>The value you specify has to be at least 0, and can be no more than 1000.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>A list of all of the identities that you've attempted to verify for use with Amazon Pinpoint, regardless of whether or not those identities were successfully verified.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListEmailIdentitiesResponse {
    /// <p>An array that includes all of the identities associated with your Amazon Pinpoint account.</p>
    #[serde(rename = "emailIdentities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_identities: Option<Vec<IdentityInfo>>,
    /// <p>A token that indicates that there are additional configuration sets to list. To view additional configuration sets, issue another request to <code>ListEmailIdentities</code>, and pass this token in the <code>NextToken</code> parameter.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to retrieve tag information for.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>An array that lists all the tags that are associated with the resource. Each tag consists of a required tag key (<code>Key</code>) and an associated tag value (<code>Value</code>)</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

/// <p>A list of attributes that are associated with a MAIL FROM domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MailFromAttributes {
    /// <p>The action that Amazon Pinpoint to takes if it can't read the required MX record for a custom MAIL FROM domain. When you set this value to <code>UseDefaultValue</code>, Amazon Pinpoint uses <i>amazonses.com</i> as the MAIL FROM domain. When you set this value to <code>RejectMessage</code>, Amazon Pinpoint returns a <code>MailFromDomainNotVerified</code> error, and doesn't attempt to deliver the email.</p> <p>These behaviors are taken when the custom MAIL FROM domain configuration is in the <code>Pending</code>, <code>Failed</code>, and <code>TemporaryFailure</code> states.</p>
    #[serde(rename = "behaviorOnMxFailure")]
    pub behavior_on_mx_failure: String,
    /// <p>The name of a domain that an email identity uses as a custom MAIL FROM domain.</p>
    #[serde(rename = "mailFromDomain")]
    pub mail_from_domain: String,
    /// <p><p>The status of the MAIL FROM domain. This status can have the following values:</p> <ul> <li> <p> <code>PENDING</code> – Amazon Pinpoint hasn&#39;t started searching for the MX record yet.</p> </li> <li> <p> <code>SUCCESS</code> – Amazon Pinpoint detected the required MX record for the MAIL FROM domain.</p> </li> <li> <p> <code>FAILED</code> – Amazon Pinpoint can&#39;t find the required MX record, or the record no longer exists.</p> </li> <li> <p> <code>TEMPORARY_FAILURE</code> – A temporary issue occurred, which prevented Amazon Pinpoint from determining the status of the MAIL FROM domain.</p> </li> </ul></p>
    #[serde(rename = "mailFromDomainStatus")]
    pub mail_from_domain_status: String,
}

/// <p>Represents the email message that you're sending. The <code>Message</code> object consists of a subject line and a message body.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Message {
    /// <p>The body of the message. You can specify an HTML version of the message, a text-only version of the message, or both.</p>
    #[serde(rename = "body")]
    pub body: Body,
    /// <p>The subject line of the email. The subject line can only contain 7-bit ASCII characters. However, you can specify non-ASCII characters in the subject line by using encoded-word syntax, as described in <a href="https://tools.ietf.org/html/rfc2047">RFC 2047</a>.</p>
    #[serde(rename = "subject")]
    pub subject: Content,
}

/// <p>Contains the name and value of a tag that you apply to an email. You can use message tags when you publish email sending events. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MessageTag {
    /// <p><p>The name of the message tag. The message tag name has to meet the following criteria:</p> <ul> <li> <p>It can only contain ASCII letters (a–z, A–Z), numbers (0–9), underscores (_), or dashes (-).</p> </li> <li> <p>It can contain no more than 256 characters.</p> </li> </ul></p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p>The value of the message tag. The message tag value has to meet the following criteria:</p> <ul> <li> <p>It can only contain ASCII letters (a–z, A–Z), numbers (0–9), underscores (_), or dashes (-).</p> </li> <li> <p>It can contain no more than 256 characters.</p> </li> </ul></p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>An object that contains information about email that was sent from the selected domain.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OverallVolume {
    /// <p>An object that contains inbox and junk mail placement metrics for individual email providers.</p>
    #[serde(rename = "domainIspPlacements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_isp_placements: Option<Vec<DomainIspPlacement>>,
    /// <p>The percentage of emails that were sent from the domain that were read by their recipients.</p>
    #[serde(rename = "readRatePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_rate_percent: Option<f64>,
    /// <p>An object that contains information about the numbers of messages that arrived in recipients' inboxes and junk mail folders.</p>
    #[serde(rename = "volumeStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_statistics: Option<VolumeStatistics>,
}

/// <p>An object that defines a Amazon Pinpoint destination for email events. You can use Amazon Pinpoint events to create attributes in Amazon Pinpoint projects. You can use these attributes to create segments for your campaigns.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PinpointDestination {
    /// <p>The Amazon Resource Name (ARN) of the Amazon Pinpoint project that you want to send email events to.</p>
    #[serde(rename = "applicationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
}

/// <p>An object that contains inbox placement data for an email provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PlacementStatistics {
    /// <p>The percentage of emails that were authenticated by using DomainKeys Identified Mail (DKIM) during the predictive inbox placement test.</p>
    #[serde(rename = "dkimPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_percentage: Option<f64>,
    /// <p>The percentage of emails that arrived in recipients' inboxes during the predictive inbox placement test.</p>
    #[serde(rename = "inboxPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_percentage: Option<f64>,
    /// <p>The percentage of emails that didn't arrive in recipients' inboxes at all during the predictive inbox placement test.</p>
    #[serde(rename = "missingPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_percentage: Option<f64>,
    /// <p>The percentage of emails that arrived in recipients' spam or junk mail folders during the predictive inbox placement test.</p>
    #[serde(rename = "spamPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_percentage: Option<f64>,
    /// <p>The percentage of emails that were authenticated by using Sender Policy Framework (SPF) during the predictive inbox placement test.</p>
    #[serde(rename = "spfPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spf_percentage: Option<f64>,
}

/// <p>A request to enable or disable the automatic IP address warm-up feature.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAccountDedicatedIpWarmupAttributesRequest {
    /// <p>Enables or disables the automatic warm-up feature for dedicated IP addresses that are associated with your Amazon Pinpoint account in the current AWS Region. Set to <code>true</code> to enable the automatic warm-up feature, or set to <code>false</code> to disable it.</p>
    #[serde(rename = "autoWarmupEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_warmup_enabled: Option<bool>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAccountDedicatedIpWarmupAttributesResponse {}

/// <p>A request to change the ability of your account to send email.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutAccountSendingAttributesRequest {
    /// <p><p>Enables or disables your account&#39;s ability to send email. Set to <code>true</code> to enable email sending, or set to <code>false</code> to disable email sending.</p> <note> <p>If AWS paused your account&#39;s ability to send email, you can&#39;t use this operation to resume your account&#39;s ability to send email.</p> </note></p>
    #[serde(rename = "sendingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutAccountSendingAttributesResponse {}

/// <p>A request to associate a configuration set with a dedicated IP pool.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutConfigurationSetDeliveryOptionsRequest {
    /// <p>The name of the configuration set that you want to associate with a dedicated IP pool.</p>
    #[serde(rename = "configurationSetName")]
    pub configuration_set_name: String,
    /// <p>The name of the dedicated IP pool that you want to associate with the configuration set.</p>
    #[serde(rename = "sendingPoolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_pool_name: Option<String>,
    /// <p>Specifies whether messages that use the configuration set are required to use Transport Layer Security (TLS). If the value is <code>Require</code>, messages are only delivered if a TLS connection can be established. If the value is <code>Optional</code>, messages can be delivered in plain text if a TLS connection can't be established.</p>
    #[serde(rename = "tlsPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_policy: Option<String>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutConfigurationSetDeliveryOptionsResponse {}

/// <p>A request to enable or disable tracking of reputation metrics for a configuration set.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutConfigurationSetReputationOptionsRequest {
    /// <p>The name of the configuration set that you want to enable or disable reputation metric tracking for.</p>
    #[serde(rename = "configurationSetName")]
    pub configuration_set_name: String,
    /// <p>If <code>true</code>, tracking of reputation metrics is enabled for the configuration set. If <code>false</code>, tracking of reputation metrics is disabled for the configuration set.</p>
    #[serde(rename = "reputationMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_metrics_enabled: Option<bool>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutConfigurationSetReputationOptionsResponse {}

/// <p>A request to enable or disable the ability of Amazon Pinpoint to send emails that use a specific configuration set.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutConfigurationSetSendingOptionsRequest {
    /// <p>The name of the configuration set that you want to enable or disable email sending for.</p>
    #[serde(rename = "configurationSetName")]
    pub configuration_set_name: String,
    /// <p>If <code>true</code>, email sending is enabled for the configuration set. If <code>false</code>, email sending is disabled for the configuration set.</p>
    #[serde(rename = "sendingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutConfigurationSetSendingOptionsResponse {}

/// <p>A request to add a custom domain for tracking open and click events to a configuration set.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutConfigurationSetTrackingOptionsRequest {
    /// <p>The name of the configuration set that you want to add a custom tracking domain to.</p>
    #[serde(rename = "configurationSetName")]
    pub configuration_set_name: String,
    /// <p>The domain that you want to use to track open and click events.</p>
    #[serde(rename = "customRedirectDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_redirect_domain: Option<String>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutConfigurationSetTrackingOptionsResponse {}

/// <p>A request to move a dedicated IP address to a dedicated IP pool.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutDedicatedIpInPoolRequest {
    /// <p>The name of the IP pool that you want to add the dedicated IP address to. You have to specify an IP pool that already exists.</p>
    #[serde(rename = "destinationPoolName")]
    pub destination_pool_name: String,
    /// <p>The IP address that you want to move to the dedicated IP pool. The value you specify has to be a dedicated IP address that's associated with your Amazon Pinpoint account.</p>
    #[serde(rename = "ip")]
    pub ip: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutDedicatedIpInPoolResponse {}

/// <p>A request to change the warm-up attributes for a dedicated IP address. This operation is useful when you want to resume the warm-up process for an existing IP address.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutDedicatedIpWarmupAttributesRequest {
    /// <p>The dedicated IP address that you want to update the warm-up attributes for.</p>
    #[serde(rename = "ip")]
    pub ip: String,
    /// <p>The warm-up percentage that you want to associate with the dedicated IP address.</p>
    #[serde(rename = "warmupPercentage")]
    pub warmup_percentage: i64,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutDedicatedIpWarmupAttributesResponse {}

/// <p>Enable or disable the Deliverability dashboard for your Amazon Pinpoint account. When you enable the Deliverability dashboard, you gain access to reputation, deliverability, and other metrics for the domains that you use to send email using Amazon Pinpoint. You also gain the ability to perform predictive inbox placement tests.</p> <p>When you use the Deliverability dashboard, you pay a monthly subscription charge, in addition to any other fees that you accrue by using Amazon Pinpoint. For more information about the features and cost of a Deliverability dashboard subscription, see <a href="http://aws.amazon.com/pinpoint/pricing/">Amazon Pinpoint Pricing</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutDeliverabilityDashboardOptionRequest {
    /// <p>Specifies whether to enable the Deliverability dashboard for your Amazon Pinpoint account. To enable the dashboard, set this value to <code>true</code>.</p>
    #[serde(rename = "dashboardEnabled")]
    pub dashboard_enabled: bool,
    /// <p>An array of objects, one for each verified domain that you use to send email and enabled the Deliverability dashboard for.</p>
    #[serde(rename = "subscribedDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_domains: Option<Vec<DomainDeliverabilityTrackingOption>>,
}

/// <p>A response that indicates whether the Deliverability dashboard is enabled for your Amazon Pinpoint account.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutDeliverabilityDashboardOptionResponse {}

/// <p>A request to enable or disable DKIM signing of email that you send from an email identity.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEmailIdentityDkimAttributesRequest {
    /// <p>The email identity that you want to change the DKIM settings for.</p>
    #[serde(rename = "emailIdentity")]
    pub email_identity: String,
    /// <p>Sets the DKIM signing configuration for the identity.</p> <p>When you set this value <code>true</code>, then the messages that Amazon Pinpoint sends from the identity are DKIM-signed. When you set this value to <code>false</code>, then the messages that Amazon Pinpoint sends from the identity aren't DKIM-signed.</p>
    #[serde(rename = "signingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_enabled: Option<bool>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEmailIdentityDkimAttributesResponse {}

/// <p>A request to set the attributes that control how bounce and complaint events are processed.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEmailIdentityFeedbackAttributesRequest {
    /// <p>Sets the feedback forwarding configuration for the identity.</p> <p>If the value is <code>true</code>, Amazon Pinpoint sends you email notifications when bounce or complaint events occur. Amazon Pinpoint sends this notification to the address that you specified in the Return-Path header of the original email.</p> <p>When you set this value to <code>false</code>, Amazon Pinpoint sends notifications through other mechanisms, such as by notifying an Amazon SNS topic or another event destination. You're required to have a method of tracking bounces and complaints. If you haven't set up another mechanism for receiving bounce or complaint notifications, Amazon Pinpoint sends an email notification when these events occur (even if this setting is disabled).</p>
    #[serde(rename = "emailForwardingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_forwarding_enabled: Option<bool>,
    /// <p>The email identity that you want to configure bounce and complaint feedback forwarding for.</p>
    #[serde(rename = "emailIdentity")]
    pub email_identity: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEmailIdentityFeedbackAttributesResponse {}

/// <p>A request to configure the custom MAIL FROM domain for a verified identity.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutEmailIdentityMailFromAttributesRequest {
    /// <p>The action that you want Amazon Pinpoint to take if it can't read the required MX record when you send an email. When you set this value to <code>UseDefaultValue</code>, Amazon Pinpoint uses <i>amazonses.com</i> as the MAIL FROM domain. When you set this value to <code>RejectMessage</code>, Amazon Pinpoint returns a <code>MailFromDomainNotVerified</code> error, and doesn't attempt to deliver the email.</p> <p>These behaviors are taken when the custom MAIL FROM domain configuration is in the <code>Pending</code>, <code>Failed</code>, and <code>TemporaryFailure</code> states.</p>
    #[serde(rename = "behaviorOnMxFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_on_mx_failure: Option<String>,
    /// <p>The verified email identity that you want to set up the custom MAIL FROM domain for.</p>
    #[serde(rename = "emailIdentity")]
    pub email_identity: String,
    /// <p><p> The custom MAIL FROM domain that you want the verified identity to use. The MAIL FROM domain must meet the following criteria:</p> <ul> <li> <p>It has to be a subdomain of the verified identity.</p> </li> <li> <p>It can&#39;t be used to receive email.</p> </li> <li> <p>It can&#39;t be used in a &quot;From&quot; address if the MAIL FROM domain is a destination for feedback forwarding emails.</p> </li> </ul></p>
    #[serde(rename = "mailFromDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_from_domain: Option<String>,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutEmailIdentityMailFromAttributesResponse {}

/// <p>The raw email message.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RawMessage {
    /// <p><p>The raw email message. The message has to meet the following criteria:</p> <ul> <li> <p>The message has to contain a header and a body, separated by one blank line.</p> </li> <li> <p>All of the required header fields must be present in the message.</p> </li> <li> <p>Each part of a multipart MIME message must be formatted properly.</p> </li> <li> <p>Attachments must be in a file format that Amazon Pinpoint supports. </p> </li> <li> <p>The entire message must be Base64 encoded.</p> </li> <li> <p>If any of the MIME parts in your message contain content that is outside of the 7-bit ASCII character range, you should encode that content to ensure that recipients&#39; email clients render the message properly.</p> </li> <li> <p>The length of any single line of text in the message can&#39;t exceed 1,000 characters. This restriction is defined in <a href="https://tools.ietf.org/html/rfc5321">RFC 5321</a>.</p> </li> </ul></p>
    #[serde(rename = "data")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub data: bytes::Bytes,
}

/// <p>Enable or disable collection of reputation metrics for emails that you send using this configuration set in the current AWS Region. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ReputationOptions {
    /// <p>The date and time (in Unix time) when the reputation metrics were last given a fresh start. When your account is given a fresh start, your reputation metrics are calculated starting from the date of the fresh start.</p>
    #[serde(rename = "lastFreshStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fresh_start: Option<f64>,
    /// <p>If <code>true</code>, tracking of reputation metrics is enabled for the configuration set. If <code>false</code>, tracking of reputation metrics is disabled for the configuration set.</p>
    #[serde(rename = "reputationMetricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_metrics_enabled: Option<bool>,
}

/// <p>A request to send an email message.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SendEmailRequest {
    /// <p>The name of the configuration set that you want to use when sending the email.</p>
    #[serde(rename = "configurationSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    /// <p>An object that contains the body of the message. You can send either a Simple message or a Raw message.</p>
    #[serde(rename = "content")]
    pub content: EmailContent,
    /// <p>An object that contains the recipients of the email message.</p>
    #[serde(rename = "destination")]
    pub destination: Destination,
    /// <p>A list of tags, in the form of name/value pairs, to apply to an email that you send using the <code>SendEmail</code> operation. Tags correspond to characteristics of the email that you define, so that you can publish email sending events. </p>
    #[serde(rename = "emailTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_tags: Option<Vec<MessageTag>>,
    /// <p>The address that Amazon Pinpoint should send bounce and complaint notifications to.</p>
    #[serde(rename = "feedbackForwardingEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_email_address: Option<String>,
    /// <p>The email address that you want to use as the "From" address for the email. The address that you specify has to be verified. </p>
    #[serde(rename = "fromEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    /// <p>The "Reply-to" email addresses for the message. When the recipient replies to the message, each Reply-to address receives the reply.</p>
    #[serde(rename = "replyToAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_addresses: Option<Vec<String>>,
}

/// <p>A unique message ID that you receive when Amazon Pinpoint accepts an email for sending.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendEmailResponse {
    /// <p><p>A unique identifier for the message that is generated when Amazon Pinpoint accepts the message.</p> <note> <p>It is possible for Amazon Pinpoint to accept a message without sending it. This can happen when the message you&#39;re trying to send has an attachment doesn&#39;t pass a virus check, or when you send a templated email that contains invalid personalization content, for example.</p> </note></p>
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

/// <p>An object that contains information about the per-day and per-second sending limits for your Amazon Pinpoint account in the current AWS Region.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SendQuota {
    /// <p>The maximum number of emails that you can send in the current AWS Region over a 24-hour period. This value is also called your <i>sending quota</i>.</p>
    #[serde(rename = "max24HourSend")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_24_hour_send: Option<f64>,
    /// <p>The maximum number of emails that you can send per second in the current AWS Region. This value is also called your <i>maximum sending rate</i> or your <i>maximum TPS (transactions per second) rate</i>.</p>
    #[serde(rename = "maxSendRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_send_rate: Option<f64>,
    /// <p>The number of emails sent from your Amazon Pinpoint account in the current AWS Region over the past 24 hours.</p>
    #[serde(rename = "sentLast24Hours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_last_24_hours: Option<f64>,
}

/// <p>Used to enable or disable email sending for messages that use this configuration set in the current AWS Region.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SendingOptions {
    /// <p>If <code>true</code>, email sending is enabled for the configuration set. If <code>false</code>, email sending is disabled for the configuration set.</p>
    #[serde(rename = "sendingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
}

/// <p>An object that defines an Amazon SNS destination for email events. You can use Amazon SNS to send notification when certain email events occur.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SnsDestination {
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic that you want to publish email events to. For more information about Amazon SNS topics, see the <a href="https://docs.aws.amazon.com/sns/latest/dg/CreateTopic.html">Amazon SNS Developer Guide</a>.</p>
    #[serde(rename = "topicArn")]
    pub topic_arn: String,
}

/// <p><p>An object that defines the tags that are associated with a resource. A <i>tag</i> is a label that you optionally define and associate with a resource in Amazon Pinpoint. Tags can help you categorize and manage resources in different ways, such as by purpose, owner, environment, or other criteria. A resource can have as many as 50 tags.</p> <p>Each tag consists of a required <i>tag key</i> and an associated <i>tag value</i>, both of which you define. A tag key is a general label that acts as a category for a more specific tag value. A tag value acts as a descriptor within a tag key. A tag key can contain as many as 128 characters. A tag value can contain as many as 256 characters. The characters can be Unicode letters, digits, white space, or one of the following symbols: _ . : / = + -. The following additional restrictions apply to tags:</p> <ul> <li> <p>Tag keys and values are case sensitive.</p> </li> <li> <p>For each associated resource, each tag key must be unique and it can have only one value.</p> </li> <li> <p>The <code>aws:</code> prefix is reserved for use by AWS; you can’t use it in any tag keys or values that you define. In addition, you can&#39;t edit or remove tag keys or values that use this prefix. Tags that use this prefix don’t count against the limit of 50 tags per resource.</p> </li> <li> <p>You can associate tags with public or shared resources, but the tags are available only for your AWS account, not any other accounts that share the resource. In addition, the tags are available only for resources that are located in the specified AWS Region for your AWS account.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>One part of a key-value pair that defines a tag. The maximum length of a tag key is 128 characters. The minimum length is 1 character.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The optional part of a key-value pair that defines a tag. The maximum length of a tag value is 256 characters. The minimum length is 0 characters. If you don’t want a resource to have a specific tag value, don’t specify a value for this parameter. Amazon Pinpoint will set the value to an empty string.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to add one or more tags to.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>A list of the tags that you want to add to the resource. A tag consists of a required tag key (<code>Key</code>) and an associated tag value (<code>Value</code>). The maximum length of a tag key is 128 characters. The maximum length of a tag value is 256 characters.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Template {
    /// <p>The Amazon Resource Name (ARN) of the template.</p>
    #[serde(rename = "templateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    /// <p>An object that defines the values to use for message variables in the template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the value to use for that variable.</p>
    #[serde(rename = "templateData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_data: Option<String>,
}

/// <p>An object that defines the tracking options for a configuration set. When you use Amazon Pinpoint to send an email, it contains an invisible image that's used to track when recipients open your email. If your email contains links, those links are changed slightly in order to track when recipients click them.</p> <p>These images and links include references to a domain operated by AWS. You can optionally configure Amazon Pinpoint to use a domain that you operate for these images and links.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TrackingOptions {
    /// <p>The domain that you want to use for tracking open and click events.</p>
    #[serde(rename = "customRedirectDomain")]
    pub custom_redirect_domain: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to remove one or more tags from.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags (tag keys) that you want to remove from the resource. When you specify a tag key, the action removes both that key and its associated tag value.</p> <p>To remove more than one tag from the resource, append the <code>TagKeys</code> parameter and argument for each additional tag to remove, separated by an ampersand. For example: <code>/v1/email/tags?ResourceArn=ResourceArn&amp;TagKeys=Key1&amp;TagKeys=Key2</code> </p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>A request to change the settings for an event destination for a configuration set.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateConfigurationSetEventDestinationRequest {
    /// <p>The name of the configuration set that contains the event destination that you want to modify.</p>
    #[serde(rename = "configurationSetName")]
    pub configuration_set_name: String,
    /// <p>An object that defines the event destination.</p>
    #[serde(rename = "eventDestination")]
    pub event_destination: EventDestinationDefinition,
    /// <p>The name of the event destination that you want to modify.</p>
    #[serde(rename = "eventDestinationName")]
    pub event_destination_name: String,
}

/// <p>An HTTP 200 response if the request succeeds, or an error message if the request fails.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateConfigurationSetEventDestinationResponse {}

/// <p>An object that contains information about the amount of email that was delivered to recipients.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VolumeStatistics {
    /// <p>The total number of emails that arrived in recipients' inboxes.</p>
    #[serde(rename = "inboxRawCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_raw_count: Option<i64>,
    /// <p>An estimate of the percentage of emails sent from the current domain that will arrive in recipients' inboxes.</p>
    #[serde(rename = "projectedInbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_inbox: Option<i64>,
    /// <p>An estimate of the percentage of emails sent from the current domain that will arrive in recipients' spam or junk mail folders.</p>
    #[serde(rename = "projectedSpam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_spam: Option<i64>,
    /// <p>The total number of emails that arrived in recipients' spam or junk mail folders.</p>
    #[serde(rename = "spamRawCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_raw_count: Option<i64>,
}

/// Errors returned by CreateConfigurationSet
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationSetError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateConfigurationSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateConfigurationSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateConfigurationSetError::AlreadyExists(
                        err.msg,
                    ))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateConfigurationSetError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetError::ConcurrentModification(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateConfigurationSetError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateConfigurationSetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateConfigurationSetError::TooManyRequests(
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
impl fmt::Display for CreateConfigurationSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConfigurationSetError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateConfigurationSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateConfigurationSetError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateConfigurationSetError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateConfigurationSetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateConfigurationSetError {}
/// Errors returned by CreateConfigurationSetEventDestination
#[derive(Debug, PartialEq)]
pub enum CreateConfigurationSetEventDestinationError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateConfigurationSetEventDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateConfigurationSetEventDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::AlreadyExists(err.msg),
                    )
                }
                "BadRequestException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::BadRequest(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::LimitExceeded(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        CreateConfigurationSetEventDestinationError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateConfigurationSetEventDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateConfigurationSetEventDestinationError::AlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetEventDestinationError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetEventDestinationError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetEventDestinationError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateConfigurationSetEventDestinationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateConfigurationSetEventDestinationError {}
/// Errors returned by CreateDedicatedIpPool
#[derive(Debug, PartialEq)]
pub enum CreateDedicatedIpPoolError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateDedicatedIpPoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDedicatedIpPoolError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateDedicatedIpPoolError::AlreadyExists(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateDedicatedIpPoolError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        CreateDedicatedIpPoolError::ConcurrentModification(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDedicatedIpPoolError::LimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateDedicatedIpPoolError::TooManyRequests(
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
impl fmt::Display for CreateDedicatedIpPoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDedicatedIpPoolError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateDedicatedIpPoolError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateDedicatedIpPoolError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateDedicatedIpPoolError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDedicatedIpPoolError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDedicatedIpPoolError {}
/// Errors returned by CreateDeliverabilityTestReport
#[derive(Debug, PartialEq)]
pub enum CreateDeliverabilityTestReportError {
    /// <p>The message can't be sent because the account's ability to send email has been permanently restricted.</p>
    AccountSuspended(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The message can't be sent because the sending domain isn't verified.</p>
    MailFromDomainNotVerified(String),
    /// <p>The message can't be sent because it contains invalid content.</p>
    MessageRejected(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>The message can't be sent because the account's ability to send email is currently paused.</p>
    SendingPaused(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateDeliverabilityTestReportError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateDeliverabilityTestReportError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccountSuspendedException" => {
                    return RusotoError::Service(
                        CreateDeliverabilityTestReportError::AccountSuspended(err.msg),
                    )
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateDeliverabilityTestReportError::BadRequest(
                        err.msg,
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        CreateDeliverabilityTestReportError::ConcurrentModification(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        CreateDeliverabilityTestReportError::LimitExceeded(err.msg),
                    )
                }
                "MailFromDomainNotVerifiedException" => {
                    return RusotoError::Service(
                        CreateDeliverabilityTestReportError::MailFromDomainNotVerified(err.msg),
                    )
                }
                "MessageRejected" => {
                    return RusotoError::Service(
                        CreateDeliverabilityTestReportError::MessageRejected(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDeliverabilityTestReportError::NotFound(
                        err.msg,
                    ))
                }
                "SendingPausedException" => {
                    return RusotoError::Service(
                        CreateDeliverabilityTestReportError::SendingPaused(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        CreateDeliverabilityTestReportError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDeliverabilityTestReportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDeliverabilityTestReportError::AccountSuspended(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeliverabilityTestReportError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateDeliverabilityTestReportError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeliverabilityTestReportError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDeliverabilityTestReportError::MailFromDomainNotVerified(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeliverabilityTestReportError::MessageRejected(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateDeliverabilityTestReportError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateDeliverabilityTestReportError::SendingPaused(ref cause) => write!(f, "{}", cause),
            CreateDeliverabilityTestReportError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateDeliverabilityTestReportError {}
/// Errors returned by CreateEmailIdentity
#[derive(Debug, PartialEq)]
pub enum CreateEmailIdentityError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl CreateEmailIdentityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEmailIdentityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateEmailIdentityError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateEmailIdentityError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateEmailIdentityError::LimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateEmailIdentityError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateEmailIdentityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEmailIdentityError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateEmailIdentityError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateEmailIdentityError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateEmailIdentityError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEmailIdentityError {}
/// Errors returned by DeleteConfigurationSet
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationSetError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl DeleteConfigurationSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConfigurationSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteConfigurationSetError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteConfigurationSetError::ConcurrentModification(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteConfigurationSetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteConfigurationSetError::TooManyRequests(
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
impl fmt::Display for DeleteConfigurationSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConfigurationSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteConfigurationSetError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteConfigurationSetError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteConfigurationSetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConfigurationSetError {}
/// Errors returned by DeleteConfigurationSetEventDestination
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationSetEventDestinationError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl DeleteConfigurationSetEventDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteConfigurationSetEventDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        DeleteConfigurationSetEventDestinationError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        DeleteConfigurationSetEventDestinationError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DeleteConfigurationSetEventDestinationError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteConfigurationSetEventDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConfigurationSetEventDestinationError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteConfigurationSetEventDestinationError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteConfigurationSetEventDestinationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteConfigurationSetEventDestinationError {}
/// Errors returned by DeleteDedicatedIpPool
#[derive(Debug, PartialEq)]
pub enum DeleteDedicatedIpPoolError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl DeleteDedicatedIpPoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDedicatedIpPoolError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteDedicatedIpPoolError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteDedicatedIpPoolError::ConcurrentModification(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDedicatedIpPoolError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteDedicatedIpPoolError::TooManyRequests(
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
impl fmt::Display for DeleteDedicatedIpPoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDedicatedIpPoolError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteDedicatedIpPoolError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteDedicatedIpPoolError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteDedicatedIpPoolError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDedicatedIpPoolError {}
/// Errors returned by DeleteEmailIdentity
#[derive(Debug, PartialEq)]
pub enum DeleteEmailIdentityError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl DeleteEmailIdentityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEmailIdentityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteEmailIdentityError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteEmailIdentityError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteEmailIdentityError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteEmailIdentityError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEmailIdentityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEmailIdentityError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteEmailIdentityError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteEmailIdentityError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteEmailIdentityError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEmailIdentityError {}
/// Errors returned by GetAccount
#[derive(Debug, PartialEq)]
pub enum GetAccountError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetAccountError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetAccountError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAccountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAccountError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetAccountError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAccountError {}
/// Errors returned by GetBlacklistReports
#[derive(Debug, PartialEq)]
pub enum GetBlacklistReportsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetBlacklistReportsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBlacklistReportsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetBlacklistReportsError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetBlacklistReportsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetBlacklistReportsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetBlacklistReportsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBlacklistReportsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetBlacklistReportsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetBlacklistReportsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBlacklistReportsError {}
/// Errors returned by GetConfigurationSet
#[derive(Debug, PartialEq)]
pub enum GetConfigurationSetError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetConfigurationSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConfigurationSetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetConfigurationSetError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetConfigurationSetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetConfigurationSetError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetConfigurationSetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetConfigurationSetError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetConfigurationSetError::NotFound(ref cause) => write!(f, "{}", cause),
            GetConfigurationSetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetConfigurationSetError {}
/// Errors returned by GetConfigurationSetEventDestinations
#[derive(Debug, PartialEq)]
pub enum GetConfigurationSetEventDestinationsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetConfigurationSetEventDestinationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetConfigurationSetEventDestinationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        GetConfigurationSetEventDestinationsError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        GetConfigurationSetEventDestinationsError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetConfigurationSetEventDestinationsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetConfigurationSetEventDestinationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetConfigurationSetEventDestinationsError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            GetConfigurationSetEventDestinationsError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetConfigurationSetEventDestinationsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetConfigurationSetEventDestinationsError {}
/// Errors returned by GetDedicatedIp
#[derive(Debug, PartialEq)]
pub enum GetDedicatedIpError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetDedicatedIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDedicatedIpError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDedicatedIpError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDedicatedIpError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDedicatedIpError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDedicatedIpError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDedicatedIpError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDedicatedIpError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDedicatedIpError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDedicatedIpError {}
/// Errors returned by GetDedicatedIps
#[derive(Debug, PartialEq)]
pub enum GetDedicatedIpsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetDedicatedIpsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDedicatedIpsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDedicatedIpsError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDedicatedIpsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDedicatedIpsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDedicatedIpsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDedicatedIpsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDedicatedIpsError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDedicatedIpsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDedicatedIpsError {}
/// Errors returned by GetDeliverabilityDashboardOptions
#[derive(Debug, PartialEq)]
pub enum GetDeliverabilityDashboardOptionsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetDeliverabilityDashboardOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDeliverabilityDashboardOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        GetDeliverabilityDashboardOptionsError::BadRequest(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        GetDeliverabilityDashboardOptionsError::LimitExceeded(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetDeliverabilityDashboardOptionsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDeliverabilityDashboardOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeliverabilityDashboardOptionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDeliverabilityDashboardOptionsError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            GetDeliverabilityDashboardOptionsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetDeliverabilityDashboardOptionsError {}
/// Errors returned by GetDeliverabilityTestReport
#[derive(Debug, PartialEq)]
pub enum GetDeliverabilityTestReportError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetDeliverabilityTestReportError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDeliverabilityTestReportError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDeliverabilityTestReportError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDeliverabilityTestReportError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDeliverabilityTestReportError::TooManyRequests(
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
impl fmt::Display for GetDeliverabilityTestReportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDeliverabilityTestReportError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDeliverabilityTestReportError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDeliverabilityTestReportError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDeliverabilityTestReportError {}
/// Errors returned by GetDomainDeliverabilityCampaign
#[derive(Debug, PartialEq)]
pub enum GetDomainDeliverabilityCampaignError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetDomainDeliverabilityCampaignError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDomainDeliverabilityCampaignError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDomainDeliverabilityCampaignError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDomainDeliverabilityCampaignError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetDomainDeliverabilityCampaignError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetDomainDeliverabilityCampaignError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDomainDeliverabilityCampaignError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDomainDeliverabilityCampaignError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDomainDeliverabilityCampaignError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetDomainDeliverabilityCampaignError {}
/// Errors returned by GetDomainStatisticsReport
#[derive(Debug, PartialEq)]
pub enum GetDomainStatisticsReportError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetDomainStatisticsReportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDomainStatisticsReportError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDomainStatisticsReportError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDomainStatisticsReportError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDomainStatisticsReportError::TooManyRequests(
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
impl fmt::Display for GetDomainStatisticsReportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDomainStatisticsReportError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetDomainStatisticsReportError::NotFound(ref cause) => write!(f, "{}", cause),
            GetDomainStatisticsReportError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDomainStatisticsReportError {}
/// Errors returned by GetEmailIdentity
#[derive(Debug, PartialEq)]
pub enum GetEmailIdentityError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl GetEmailIdentityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEmailIdentityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetEmailIdentityError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetEmailIdentityError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetEmailIdentityError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetEmailIdentityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEmailIdentityError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetEmailIdentityError::NotFound(ref cause) => write!(f, "{}", cause),
            GetEmailIdentityError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEmailIdentityError {}
/// Errors returned by ListConfigurationSets
#[derive(Debug, PartialEq)]
pub enum ListConfigurationSetsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListConfigurationSetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConfigurationSetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListConfigurationSetsError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListConfigurationSetsError::TooManyRequests(
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
impl fmt::Display for ListConfigurationSetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListConfigurationSetsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListConfigurationSetsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListConfigurationSetsError {}
/// Errors returned by ListDedicatedIpPools
#[derive(Debug, PartialEq)]
pub enum ListDedicatedIpPoolsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListDedicatedIpPoolsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDedicatedIpPoolsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListDedicatedIpPoolsError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListDedicatedIpPoolsError::TooManyRequests(
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
impl fmt::Display for ListDedicatedIpPoolsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDedicatedIpPoolsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListDedicatedIpPoolsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDedicatedIpPoolsError {}
/// Errors returned by ListDeliverabilityTestReports
#[derive(Debug, PartialEq)]
pub enum ListDeliverabilityTestReportsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListDeliverabilityTestReportsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDeliverabilityTestReportsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListDeliverabilityTestReportsError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListDeliverabilityTestReportsError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListDeliverabilityTestReportsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDeliverabilityTestReportsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDeliverabilityTestReportsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListDeliverabilityTestReportsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListDeliverabilityTestReportsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListDeliverabilityTestReportsError {}
/// Errors returned by ListDomainDeliverabilityCampaigns
#[derive(Debug, PartialEq)]
pub enum ListDomainDeliverabilityCampaignsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListDomainDeliverabilityCampaignsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListDomainDeliverabilityCampaignsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        ListDomainDeliverabilityCampaignsError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListDomainDeliverabilityCampaignsError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListDomainDeliverabilityCampaignsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDomainDeliverabilityCampaignsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDomainDeliverabilityCampaignsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListDomainDeliverabilityCampaignsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListDomainDeliverabilityCampaignsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListDomainDeliverabilityCampaignsError {}
/// Errors returned by ListEmailIdentities
#[derive(Debug, PartialEq)]
pub enum ListEmailIdentitiesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListEmailIdentitiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEmailIdentitiesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListEmailIdentitiesError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListEmailIdentitiesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListEmailIdentitiesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEmailIdentitiesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListEmailIdentitiesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEmailIdentitiesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTagsForResourceError::TooManyRequests(err.msg))
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
            ListTagsForResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PutAccountDedicatedIpWarmupAttributes
#[derive(Debug, PartialEq)]
pub enum PutAccountDedicatedIpWarmupAttributesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutAccountDedicatedIpWarmupAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutAccountDedicatedIpWarmupAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutAccountDedicatedIpWarmupAttributesError::BadRequest(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutAccountDedicatedIpWarmupAttributesError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutAccountDedicatedIpWarmupAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAccountDedicatedIpWarmupAttributesError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutAccountDedicatedIpWarmupAttributesError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutAccountDedicatedIpWarmupAttributesError {}
/// Errors returned by PutAccountSendingAttributes
#[derive(Debug, PartialEq)]
pub enum PutAccountSendingAttributesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutAccountSendingAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutAccountSendingAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutAccountSendingAttributesError::BadRequest(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutAccountSendingAttributesError::TooManyRequests(
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
impl fmt::Display for PutAccountSendingAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutAccountSendingAttributesError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutAccountSendingAttributesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutAccountSendingAttributesError {}
/// Errors returned by PutConfigurationSetDeliveryOptions
#[derive(Debug, PartialEq)]
pub enum PutConfigurationSetDeliveryOptionsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutConfigurationSetDeliveryOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutConfigurationSetDeliveryOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutConfigurationSetDeliveryOptionsError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutConfigurationSetDeliveryOptionsError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutConfigurationSetDeliveryOptionsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutConfigurationSetDeliveryOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConfigurationSetDeliveryOptionsError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationSetDeliveryOptionsError::NotFound(ref cause) => write!(f, "{}", cause),
            PutConfigurationSetDeliveryOptionsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutConfigurationSetDeliveryOptionsError {}
/// Errors returned by PutConfigurationSetReputationOptions
#[derive(Debug, PartialEq)]
pub enum PutConfigurationSetReputationOptionsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutConfigurationSetReputationOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutConfigurationSetReputationOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutConfigurationSetReputationOptionsError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        PutConfigurationSetReputationOptionsError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutConfigurationSetReputationOptionsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutConfigurationSetReputationOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConfigurationSetReputationOptionsError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationSetReputationOptionsError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationSetReputationOptionsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutConfigurationSetReputationOptionsError {}
/// Errors returned by PutConfigurationSetSendingOptions
#[derive(Debug, PartialEq)]
pub enum PutConfigurationSetSendingOptionsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutConfigurationSetSendingOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutConfigurationSetSendingOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutConfigurationSetSendingOptionsError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutConfigurationSetSendingOptionsError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutConfigurationSetSendingOptionsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutConfigurationSetSendingOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConfigurationSetSendingOptionsError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutConfigurationSetSendingOptionsError::NotFound(ref cause) => write!(f, "{}", cause),
            PutConfigurationSetSendingOptionsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutConfigurationSetSendingOptionsError {}
/// Errors returned by PutConfigurationSetTrackingOptions
#[derive(Debug, PartialEq)]
pub enum PutConfigurationSetTrackingOptionsError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutConfigurationSetTrackingOptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutConfigurationSetTrackingOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutConfigurationSetTrackingOptionsError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutConfigurationSetTrackingOptionsError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutConfigurationSetTrackingOptionsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutConfigurationSetTrackingOptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutConfigurationSetTrackingOptionsError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutConfigurationSetTrackingOptionsError::NotFound(ref cause) => write!(f, "{}", cause),
            PutConfigurationSetTrackingOptionsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutConfigurationSetTrackingOptionsError {}
/// Errors returned by PutDedicatedIpInPool
#[derive(Debug, PartialEq)]
pub enum PutDedicatedIpInPoolError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutDedicatedIpInPoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutDedicatedIpInPoolError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutDedicatedIpInPoolError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutDedicatedIpInPoolError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutDedicatedIpInPoolError::TooManyRequests(
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
impl fmt::Display for PutDedicatedIpInPoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutDedicatedIpInPoolError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutDedicatedIpInPoolError::NotFound(ref cause) => write!(f, "{}", cause),
            PutDedicatedIpInPoolError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutDedicatedIpInPoolError {}
/// Errors returned by PutDedicatedIpWarmupAttributes
#[derive(Debug, PartialEq)]
pub enum PutDedicatedIpWarmupAttributesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutDedicatedIpWarmupAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutDedicatedIpWarmupAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutDedicatedIpWarmupAttributesError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutDedicatedIpWarmupAttributesError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutDedicatedIpWarmupAttributesError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutDedicatedIpWarmupAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutDedicatedIpWarmupAttributesError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutDedicatedIpWarmupAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
            PutDedicatedIpWarmupAttributesError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutDedicatedIpWarmupAttributesError {}
/// Errors returned by PutDeliverabilityDashboardOption
#[derive(Debug, PartialEq)]
pub enum PutDeliverabilityDashboardOptionError {
    /// <p>The resource specified in your request already exists.</p>
    AlreadyExists(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutDeliverabilityDashboardOptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutDeliverabilityDashboardOptionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(
                        PutDeliverabilityDashboardOptionError::AlreadyExists(err.msg),
                    )
                }
                "BadRequestException" => {
                    return RusotoError::Service(PutDeliverabilityDashboardOptionError::BadRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(
                        PutDeliverabilityDashboardOptionError::LimitExceeded(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutDeliverabilityDashboardOptionError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutDeliverabilityDashboardOptionError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutDeliverabilityDashboardOptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutDeliverabilityDashboardOptionError::AlreadyExists(ref cause) => {
                write!(f, "{}", cause)
            }
            PutDeliverabilityDashboardOptionError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutDeliverabilityDashboardOptionError::LimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            PutDeliverabilityDashboardOptionError::NotFound(ref cause) => write!(f, "{}", cause),
            PutDeliverabilityDashboardOptionError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutDeliverabilityDashboardOptionError {}
/// Errors returned by PutEmailIdentityDkimAttributes
#[derive(Debug, PartialEq)]
pub enum PutEmailIdentityDkimAttributesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutEmailIdentityDkimAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutEmailIdentityDkimAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutEmailIdentityDkimAttributesError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutEmailIdentityDkimAttributesError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutEmailIdentityDkimAttributesError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEmailIdentityDkimAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEmailIdentityDkimAttributesError::BadRequest(ref cause) => write!(f, "{}", cause),
            PutEmailIdentityDkimAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
            PutEmailIdentityDkimAttributesError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutEmailIdentityDkimAttributesError {}
/// Errors returned by PutEmailIdentityFeedbackAttributes
#[derive(Debug, PartialEq)]
pub enum PutEmailIdentityFeedbackAttributesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutEmailIdentityFeedbackAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutEmailIdentityFeedbackAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutEmailIdentityFeedbackAttributesError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutEmailIdentityFeedbackAttributesError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutEmailIdentityFeedbackAttributesError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEmailIdentityFeedbackAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEmailIdentityFeedbackAttributesError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutEmailIdentityFeedbackAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
            PutEmailIdentityFeedbackAttributesError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutEmailIdentityFeedbackAttributesError {}
/// Errors returned by PutEmailIdentityMailFromAttributes
#[derive(Debug, PartialEq)]
pub enum PutEmailIdentityMailFromAttributesError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl PutEmailIdentityMailFromAttributesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutEmailIdentityMailFromAttributesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        PutEmailIdentityMailFromAttributesError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutEmailIdentityMailFromAttributesError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutEmailIdentityMailFromAttributesError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutEmailIdentityMailFromAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutEmailIdentityMailFromAttributesError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            PutEmailIdentityMailFromAttributesError::NotFound(ref cause) => write!(f, "{}", cause),
            PutEmailIdentityMailFromAttributesError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutEmailIdentityMailFromAttributesError {}
/// Errors returned by SendEmail
#[derive(Debug, PartialEq)]
pub enum SendEmailError {
    /// <p>The message can't be sent because the account's ability to send email has been permanently restricted.</p>
    AccountSuspended(String),
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>There are too many instances of the specified resource type.</p>
    LimitExceeded(String),
    /// <p>The message can't be sent because the sending domain isn't verified.</p>
    MailFromDomainNotVerified(String),
    /// <p>The message can't be sent because it contains invalid content.</p>
    MessageRejected(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>The message can't be sent because the account's ability to send email is currently paused.</p>
    SendingPaused(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl SendEmailError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SendEmailError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccountSuspendedException" => {
                    return RusotoError::Service(SendEmailError::AccountSuspended(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(SendEmailError::BadRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(SendEmailError::LimitExceeded(err.msg))
                }
                "MailFromDomainNotVerifiedException" => {
                    return RusotoError::Service(SendEmailError::MailFromDomainNotVerified(err.msg))
                }
                "MessageRejected" => {
                    return RusotoError::Service(SendEmailError::MessageRejected(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(SendEmailError::NotFound(err.msg))
                }
                "SendingPausedException" => {
                    return RusotoError::Service(SendEmailError::SendingPaused(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SendEmailError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SendEmailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SendEmailError::AccountSuspended(ref cause) => write!(f, "{}", cause),
            SendEmailError::BadRequest(ref cause) => write!(f, "{}", cause),
            SendEmailError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            SendEmailError::MailFromDomainNotVerified(ref cause) => write!(f, "{}", cause),
            SendEmailError::MessageRejected(ref cause) => write!(f, "{}", cause),
            SendEmailError::NotFound(ref cause) => write!(f, "{}", cause),
            SendEmailError::SendingPaused(ref cause) => write!(f, "{}", cause),
            SendEmailError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SendEmailError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(TagResourceError::ConcurrentModification(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TagResourceError::TooManyRequests(err.msg))
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
            TagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource is being modified by another operation or thread.</p>
    ConcurrentModification(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UntagResourceError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UntagResourceError::TooManyRequests(err.msg))
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
            UntagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateConfigurationSetEventDestination
#[derive(Debug, PartialEq)]
pub enum UpdateConfigurationSetEventDestinationError {
    /// <p>The input you provided is invalid.</p>
    BadRequest(String),
    /// <p>The resource you attempted to access doesn't exist.</p>
    NotFound(String),
    /// <p>Too many requests have been made to the operation.</p>
    TooManyRequests(String),
}

impl UpdateConfigurationSetEventDestinationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateConfigurationSetEventDestinationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(
                        UpdateConfigurationSetEventDestinationError::BadRequest(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        UpdateConfigurationSetEventDestinationError::NotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        UpdateConfigurationSetEventDestinationError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateConfigurationSetEventDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateConfigurationSetEventDestinationError::BadRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateConfigurationSetEventDestinationError::NotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateConfigurationSetEventDestinationError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateConfigurationSetEventDestinationError {}
/// Trait representing the capabilities of the Pinpoint Email API. Pinpoint Email clients implement this trait.
#[async_trait]
pub trait PinpointEmail {
    /// <p>Create a configuration set. <i>Configuration sets</i> are groups of rules that you can apply to the emails you send using Amazon Pinpoint. You apply a configuration set to an email by including a reference to the configuration set in the headers of the email. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email. </p>
    async fn create_configuration_set(
        &self,
        input: CreateConfigurationSetRequest,
    ) -> Result<CreateConfigurationSetResponse, RusotoError<CreateConfigurationSetError>>;

    /// <p>Create an event destination. In Amazon Pinpoint, <i>events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p> <p>A single configuration set can include more than one event destination.</p>
    async fn create_configuration_set_event_destination(
        &self,
        input: CreateConfigurationSetEventDestinationRequest,
    ) -> Result<
        CreateConfigurationSetEventDestinationResponse,
        RusotoError<CreateConfigurationSetEventDestinationError>,
    >;

    /// <p>Create a new pool of dedicated IP addresses. A pool can include one or more dedicated IP addresses that are associated with your Amazon Pinpoint account. You can associate a pool with a configuration set. When you send an email that uses that configuration set, Amazon Pinpoint sends it using only the IP addresses in the associated pool.</p>
    async fn create_dedicated_ip_pool(
        &self,
        input: CreateDedicatedIpPoolRequest,
    ) -> Result<CreateDedicatedIpPoolResponse, RusotoError<CreateDedicatedIpPoolError>>;

    /// <p>Create a new predictive inbox placement test. Predictive inbox placement tests can help you predict how your messages will be handled by various email providers around the world. When you perform a predictive inbox placement test, you provide a sample message that contains the content that you plan to send to your customers. Amazon Pinpoint then sends that message to special email addresses spread across several major email providers. After about 24 hours, the test is complete, and you can use the <code>GetDeliverabilityTestReport</code> operation to view the results of the test.</p>
    async fn create_deliverability_test_report(
        &self,
        input: CreateDeliverabilityTestReportRequest,
    ) -> Result<
        CreateDeliverabilityTestReportResponse,
        RusotoError<CreateDeliverabilityTestReportError>,
    >;

    /// <p>Verifies an email identity for use with Amazon Pinpoint. In Amazon Pinpoint, an identity is an email address or domain that you use when you send email. Before you can use an identity to send email with Amazon Pinpoint, you first have to verify it. By verifying an address, you demonstrate that you're the owner of the address, and that you've given Amazon Pinpoint permission to send email from the address.</p> <p>When you verify an email address, Amazon Pinpoint sends an email to the address. Your email address is verified as soon as you follow the link in the verification email. </p> <p>When you verify a domain, this operation provides a set of DKIM tokens, which you can convert into CNAME tokens. You add these CNAME tokens to the DNS configuration for your domain. Your domain is verified when Amazon Pinpoint detects these records in the DNS configuration for your domain. It usually takes around 72 hours to complete the domain verification process.</p>
    async fn create_email_identity(
        &self,
        input: CreateEmailIdentityRequest,
    ) -> Result<CreateEmailIdentityResponse, RusotoError<CreateEmailIdentityError>>;

    /// <p>Delete an existing configuration set.</p> <p>In Amazon Pinpoint, <i>configuration sets</i> are groups of rules that you can apply to the emails you send. You apply a configuration set to an email by including a reference to the configuration set in the headers of the email. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email.</p>
    async fn delete_configuration_set(
        &self,
        input: DeleteConfigurationSetRequest,
    ) -> Result<DeleteConfigurationSetResponse, RusotoError<DeleteConfigurationSetError>>;

    /// <p>Delete an event destination.</p> <p>In Amazon Pinpoint, <i>events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p>
    async fn delete_configuration_set_event_destination(
        &self,
        input: DeleteConfigurationSetEventDestinationRequest,
    ) -> Result<
        DeleteConfigurationSetEventDestinationResponse,
        RusotoError<DeleteConfigurationSetEventDestinationError>,
    >;

    /// <p>Delete a dedicated IP pool.</p>
    async fn delete_dedicated_ip_pool(
        &self,
        input: DeleteDedicatedIpPoolRequest,
    ) -> Result<DeleteDedicatedIpPoolResponse, RusotoError<DeleteDedicatedIpPoolError>>;

    /// <p>Deletes an email identity that you previously verified for use with Amazon Pinpoint. An identity can be either an email address or a domain name.</p>
    async fn delete_email_identity(
        &self,
        input: DeleteEmailIdentityRequest,
    ) -> Result<DeleteEmailIdentityResponse, RusotoError<DeleteEmailIdentityError>>;

    /// <p>Obtain information about the email-sending status and capabilities of your Amazon Pinpoint account in the current AWS Region.</p>
    async fn get_account(&self) -> Result<GetAccountResponse, RusotoError<GetAccountError>>;

    /// <p>Retrieve a list of the blacklists that your dedicated IP addresses appear on.</p>
    async fn get_blacklist_reports(
        &self,
        input: GetBlacklistReportsRequest,
    ) -> Result<GetBlacklistReportsResponse, RusotoError<GetBlacklistReportsError>>;

    /// <p>Get information about an existing configuration set, including the dedicated IP pool that it's associated with, whether or not it's enabled for sending email, and more.</p> <p>In Amazon Pinpoint, <i>configuration sets</i> are groups of rules that you can apply to the emails you send. You apply a configuration set to an email by including a reference to the configuration set in the headers of the email. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email.</p>
    async fn get_configuration_set(
        &self,
        input: GetConfigurationSetRequest,
    ) -> Result<GetConfigurationSetResponse, RusotoError<GetConfigurationSetError>>;

    /// <p>Retrieve a list of event destinations that are associated with a configuration set.</p> <p>In Amazon Pinpoint, <i>events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p>
    async fn get_configuration_set_event_destinations(
        &self,
        input: GetConfigurationSetEventDestinationsRequest,
    ) -> Result<
        GetConfigurationSetEventDestinationsResponse,
        RusotoError<GetConfigurationSetEventDestinationsError>,
    >;

    /// <p>Get information about a dedicated IP address, including the name of the dedicated IP pool that it's associated with, as well information about the automatic warm-up process for the address.</p>
    async fn get_dedicated_ip(
        &self,
        input: GetDedicatedIpRequest,
    ) -> Result<GetDedicatedIpResponse, RusotoError<GetDedicatedIpError>>;

    /// <p>List the dedicated IP addresses that are associated with your Amazon Pinpoint account.</p>
    async fn get_dedicated_ips(
        &self,
        input: GetDedicatedIpsRequest,
    ) -> Result<GetDedicatedIpsResponse, RusotoError<GetDedicatedIpsError>>;

    /// <p>Retrieve information about the status of the Deliverability dashboard for your Amazon Pinpoint account. When the Deliverability dashboard is enabled, you gain access to reputation, deliverability, and other metrics for the domains that you use to send email using Amazon Pinpoint. You also gain the ability to perform predictive inbox placement tests.</p> <p>When you use the Deliverability dashboard, you pay a monthly subscription charge, in addition to any other fees that you accrue by using Amazon Pinpoint. For more information about the features and cost of a Deliverability dashboard subscription, see <a href="http://aws.amazon.com/pinpoint/pricing/">Amazon Pinpoint Pricing</a>.</p>
    async fn get_deliverability_dashboard_options(
        &self,
    ) -> Result<
        GetDeliverabilityDashboardOptionsResponse,
        RusotoError<GetDeliverabilityDashboardOptionsError>,
    >;

    /// <p>Retrieve the results of a predictive inbox placement test.</p>
    async fn get_deliverability_test_report(
        &self,
        input: GetDeliverabilityTestReportRequest,
    ) -> Result<GetDeliverabilityTestReportResponse, RusotoError<GetDeliverabilityTestReportError>>;

    /// <p>Retrieve all the deliverability data for a specific campaign. This data is available for a campaign only if the campaign sent email by using a domain that the Deliverability dashboard is enabled for (<code>PutDeliverabilityDashboardOption</code> operation).</p>
    async fn get_domain_deliverability_campaign(
        &self,
        input: GetDomainDeliverabilityCampaignRequest,
    ) -> Result<
        GetDomainDeliverabilityCampaignResponse,
        RusotoError<GetDomainDeliverabilityCampaignError>,
    >;

    /// <p>Retrieve inbox placement and engagement rates for the domains that you use to send email.</p>
    async fn get_domain_statistics_report(
        &self,
        input: GetDomainStatisticsReportRequest,
    ) -> Result<GetDomainStatisticsReportResponse, RusotoError<GetDomainStatisticsReportError>>;

    /// <p>Provides information about a specific identity associated with your Amazon Pinpoint account, including the identity's verification status, its DKIM authentication status, and its custom Mail-From settings.</p>
    async fn get_email_identity(
        &self,
        input: GetEmailIdentityRequest,
    ) -> Result<GetEmailIdentityResponse, RusotoError<GetEmailIdentityError>>;

    /// <p>List all of the configuration sets associated with your Amazon Pinpoint account in the current region.</p> <p>In Amazon Pinpoint, <i>configuration sets</i> are groups of rules that you can apply to the emails you send. You apply a configuration set to an email by including a reference to the configuration set in the headers of the email. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email.</p>
    async fn list_configuration_sets(
        &self,
        input: ListConfigurationSetsRequest,
    ) -> Result<ListConfigurationSetsResponse, RusotoError<ListConfigurationSetsError>>;

    /// <p>List all of the dedicated IP pools that exist in your Amazon Pinpoint account in the current AWS Region.</p>
    async fn list_dedicated_ip_pools(
        &self,
        input: ListDedicatedIpPoolsRequest,
    ) -> Result<ListDedicatedIpPoolsResponse, RusotoError<ListDedicatedIpPoolsError>>;

    /// <p>Show a list of the predictive inbox placement tests that you've performed, regardless of their statuses. For predictive inbox placement tests that are complete, you can use the <code>GetDeliverabilityTestReport</code> operation to view the results.</p>
    async fn list_deliverability_test_reports(
        &self,
        input: ListDeliverabilityTestReportsRequest,
    ) -> Result<
        ListDeliverabilityTestReportsResponse,
        RusotoError<ListDeliverabilityTestReportsError>,
    >;

    /// <p>Retrieve deliverability data for all the campaigns that used a specific domain to send email during a specified time range. This data is available for a domain only if you enabled the Deliverability dashboard (<code>PutDeliverabilityDashboardOption</code> operation) for the domain.</p>
    async fn list_domain_deliverability_campaigns(
        &self,
        input: ListDomainDeliverabilityCampaignsRequest,
    ) -> Result<
        ListDomainDeliverabilityCampaignsResponse,
        RusotoError<ListDomainDeliverabilityCampaignsError>,
    >;

    /// <p>Returns a list of all of the email identities that are associated with your Amazon Pinpoint account. An identity can be either an email address or a domain. This operation returns identities that are verified as well as those that aren't.</p>
    async fn list_email_identities(
        &self,
        input: ListEmailIdentitiesRequest,
    ) -> Result<ListEmailIdentitiesResponse, RusotoError<ListEmailIdentitiesError>>;

    /// <p>Retrieve a list of the tags (keys and values) that are associated with a specified resource. A <i>tag</i> is a label that you optionally define and associate with a resource in Amazon Pinpoint. Each tag consists of a required <i>tag key</i> and an optional associated <i>tag value</i>. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Enable or disable the automatic warm-up feature for dedicated IP addresses.</p>
    async fn put_account_dedicated_ip_warmup_attributes(
        &self,
        input: PutAccountDedicatedIpWarmupAttributesRequest,
    ) -> Result<
        PutAccountDedicatedIpWarmupAttributesResponse,
        RusotoError<PutAccountDedicatedIpWarmupAttributesError>,
    >;

    /// <p>Enable or disable the ability of your account to send email.</p>
    async fn put_account_sending_attributes(
        &self,
        input: PutAccountSendingAttributesRequest,
    ) -> Result<PutAccountSendingAttributesResponse, RusotoError<PutAccountSendingAttributesError>>;

    /// <p>Associate a configuration set with a dedicated IP pool. You can use dedicated IP pools to create groups of dedicated IP addresses for sending specific types of email.</p>
    async fn put_configuration_set_delivery_options(
        &self,
        input: PutConfigurationSetDeliveryOptionsRequest,
    ) -> Result<
        PutConfigurationSetDeliveryOptionsResponse,
        RusotoError<PutConfigurationSetDeliveryOptionsError>,
    >;

    /// <p>Enable or disable collection of reputation metrics for emails that you send using a particular configuration set in a specific AWS Region.</p>
    async fn put_configuration_set_reputation_options(
        &self,
        input: PutConfigurationSetReputationOptionsRequest,
    ) -> Result<
        PutConfigurationSetReputationOptionsResponse,
        RusotoError<PutConfigurationSetReputationOptionsError>,
    >;

    /// <p>Enable or disable email sending for messages that use a particular configuration set in a specific AWS Region.</p>
    async fn put_configuration_set_sending_options(
        &self,
        input: PutConfigurationSetSendingOptionsRequest,
    ) -> Result<
        PutConfigurationSetSendingOptionsResponse,
        RusotoError<PutConfigurationSetSendingOptionsError>,
    >;

    /// <p>Specify a custom domain to use for open and click tracking elements in email that you send using Amazon Pinpoint.</p>
    async fn put_configuration_set_tracking_options(
        &self,
        input: PutConfigurationSetTrackingOptionsRequest,
    ) -> Result<
        PutConfigurationSetTrackingOptionsResponse,
        RusotoError<PutConfigurationSetTrackingOptionsError>,
    >;

    /// <p><p>Move a dedicated IP address to an existing dedicated IP pool.</p> <note> <p>The dedicated IP address that you specify must already exist, and must be associated with your Amazon Pinpoint account. </p> <p>The dedicated IP pool you specify must already exist. You can create a new pool by using the <code>CreateDedicatedIpPool</code> operation.</p> </note></p>
    async fn put_dedicated_ip_in_pool(
        &self,
        input: PutDedicatedIpInPoolRequest,
    ) -> Result<PutDedicatedIpInPoolResponse, RusotoError<PutDedicatedIpInPoolError>>;

    /// <p><p/></p>
    async fn put_dedicated_ip_warmup_attributes(
        &self,
        input: PutDedicatedIpWarmupAttributesRequest,
    ) -> Result<
        PutDedicatedIpWarmupAttributesResponse,
        RusotoError<PutDedicatedIpWarmupAttributesError>,
    >;

    /// <p>Enable or disable the Deliverability dashboard for your Amazon Pinpoint account. When you enable the Deliverability dashboard, you gain access to reputation, deliverability, and other metrics for the domains that you use to send email using Amazon Pinpoint. You also gain the ability to perform predictive inbox placement tests.</p> <p>When you use the Deliverability dashboard, you pay a monthly subscription charge, in addition to any other fees that you accrue by using Amazon Pinpoint. For more information about the features and cost of a Deliverability dashboard subscription, see <a href="http://aws.amazon.com/pinpoint/pricing/">Amazon Pinpoint Pricing</a>.</p>
    async fn put_deliverability_dashboard_option(
        &self,
        input: PutDeliverabilityDashboardOptionRequest,
    ) -> Result<
        PutDeliverabilityDashboardOptionResponse,
        RusotoError<PutDeliverabilityDashboardOptionError>,
    >;

    /// <p>Used to enable or disable DKIM authentication for an email identity.</p>
    async fn put_email_identity_dkim_attributes(
        &self,
        input: PutEmailIdentityDkimAttributesRequest,
    ) -> Result<
        PutEmailIdentityDkimAttributesResponse,
        RusotoError<PutEmailIdentityDkimAttributesError>,
    >;

    /// <p>Used to enable or disable feedback forwarding for an identity. This setting determines what happens when an identity is used to send an email that results in a bounce or complaint event.</p> <p>When you enable feedback forwarding, Amazon Pinpoint sends you email notifications when bounce or complaint events occur. Amazon Pinpoint sends this notification to the address that you specified in the Return-Path header of the original email.</p> <p>When you disable feedback forwarding, Amazon Pinpoint sends notifications through other mechanisms, such as by notifying an Amazon SNS topic. You're required to have a method of tracking bounces and complaints. If you haven't set up another mechanism for receiving bounce or complaint notifications, Amazon Pinpoint sends an email notification when these events occur (even if this setting is disabled).</p>
    async fn put_email_identity_feedback_attributes(
        &self,
        input: PutEmailIdentityFeedbackAttributesRequest,
    ) -> Result<
        PutEmailIdentityFeedbackAttributesResponse,
        RusotoError<PutEmailIdentityFeedbackAttributesError>,
    >;

    /// <p>Used to enable or disable the custom Mail-From domain configuration for an email identity.</p>
    async fn put_email_identity_mail_from_attributes(
        &self,
        input: PutEmailIdentityMailFromAttributesRequest,
    ) -> Result<
        PutEmailIdentityMailFromAttributesResponse,
        RusotoError<PutEmailIdentityMailFromAttributesError>,
    >;

    /// <p><p>Sends an email message. You can use the Amazon Pinpoint Email API to send two types of messages:</p> <ul> <li> <p> <b>Simple</b> – A standard email message. When you create this type of message, you specify the sender, the recipient, and the message body, and Amazon Pinpoint assembles the message for you.</p> </li> <li> <p> <b>Raw</b> – A raw, MIME-formatted email message. When you send this type of email, you have to specify all of the message headers, as well as the message body. You can use this message type to send messages that contain attachments. The message that you specify has to be a valid MIME message.</p> </li> </ul></p>
    async fn send_email(
        &self,
        input: SendEmailRequest,
    ) -> Result<SendEmailResponse, RusotoError<SendEmailError>>;

    /// <p>Add one or more tags (keys and values) to a specified resource. A <i>tag</i> is a label that you optionally define and associate with a resource in Amazon Pinpoint. Tags can help you categorize and manage resources in different ways, such as by purpose, owner, environment, or other criteria. A resource can have as many as 50 tags.</p> <p>Each tag consists of a required <i>tag key</i> and an associated <i>tag value</i>, both of which you define. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Remove one or more tags (keys and values) from a specified resource.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Update the configuration of an event destination for a configuration set.</p> <p>In Amazon Pinpoint, <i>events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p>
    async fn update_configuration_set_event_destination(
        &self,
        input: UpdateConfigurationSetEventDestinationRequest,
    ) -> Result<
        UpdateConfigurationSetEventDestinationResponse,
        RusotoError<UpdateConfigurationSetEventDestinationError>,
    >;
}
/// A client for the Pinpoint Email API.
#[derive(Clone)]
pub struct PinpointEmailClient {
    client: Client,
    region: region::Region,
}

impl PinpointEmailClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> PinpointEmailClient {
        PinpointEmailClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PinpointEmailClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        PinpointEmailClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> PinpointEmailClient {
        PinpointEmailClient { client, region }
    }
}

#[async_trait]
impl PinpointEmail for PinpointEmailClient {
    /// <p>Create a configuration set. <i>Configuration sets</i> are groups of rules that you can apply to the emails you send using Amazon Pinpoint. You apply a configuration set to an email by including a reference to the configuration set in the headers of the email. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email. </p>
    #[allow(unused_mut)]
    async fn create_configuration_set(
        &self,
        input: CreateConfigurationSetRequest,
    ) -> Result<CreateConfigurationSetResponse, RusotoError<CreateConfigurationSetError>> {
        let request_uri = "/v1/email/configuration-sets";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateConfigurationSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateConfigurationSetError::from_response(response))
        }
    }

    /// <p>Create an event destination. In Amazon Pinpoint, <i>events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p> <p>A single configuration set can include more than one event destination.</p>
    #[allow(unused_mut)]
    async fn create_configuration_set_event_destination(
        &self,
        input: CreateConfigurationSetEventDestinationRequest,
    ) -> Result<
        CreateConfigurationSetEventDestinationResponse,
        RusotoError<CreateConfigurationSetEventDestinationError>,
    > {
        let request_uri = format!(
            "/v1/email/configuration-sets/{configuration_set_name}/event-destinations",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateConfigurationSetEventDestinationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateConfigurationSetEventDestinationError::from_response(
                response,
            ))
        }
    }

    /// <p>Create a new pool of dedicated IP addresses. A pool can include one or more dedicated IP addresses that are associated with your Amazon Pinpoint account. You can associate a pool with a configuration set. When you send an email that uses that configuration set, Amazon Pinpoint sends it using only the IP addresses in the associated pool.</p>
    #[allow(unused_mut)]
    async fn create_dedicated_ip_pool(
        &self,
        input: CreateDedicatedIpPoolRequest,
    ) -> Result<CreateDedicatedIpPoolResponse, RusotoError<CreateDedicatedIpPoolError>> {
        let request_uri = "/v1/email/dedicated-ip-pools";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDedicatedIpPoolResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDedicatedIpPoolError::from_response(response))
        }
    }

    /// <p>Create a new predictive inbox placement test. Predictive inbox placement tests can help you predict how your messages will be handled by various email providers around the world. When you perform a predictive inbox placement test, you provide a sample message that contains the content that you plan to send to your customers. Amazon Pinpoint then sends that message to special email addresses spread across several major email providers. After about 24 hours, the test is complete, and you can use the <code>GetDeliverabilityTestReport</code> operation to view the results of the test.</p>
    #[allow(unused_mut)]
    async fn create_deliverability_test_report(
        &self,
        input: CreateDeliverabilityTestReportRequest,
    ) -> Result<
        CreateDeliverabilityTestReportResponse,
        RusotoError<CreateDeliverabilityTestReportError>,
    > {
        let request_uri = "/v1/email/deliverability-dashboard/test";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateDeliverabilityTestReportResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateDeliverabilityTestReportError::from_response(response))
        }
    }

    /// <p>Verifies an email identity for use with Amazon Pinpoint. In Amazon Pinpoint, an identity is an email address or domain that you use when you send email. Before you can use an identity to send email with Amazon Pinpoint, you first have to verify it. By verifying an address, you demonstrate that you're the owner of the address, and that you've given Amazon Pinpoint permission to send email from the address.</p> <p>When you verify an email address, Amazon Pinpoint sends an email to the address. Your email address is verified as soon as you follow the link in the verification email. </p> <p>When you verify a domain, this operation provides a set of DKIM tokens, which you can convert into CNAME tokens. You add these CNAME tokens to the DNS configuration for your domain. Your domain is verified when Amazon Pinpoint detects these records in the DNS configuration for your domain. It usually takes around 72 hours to complete the domain verification process.</p>
    #[allow(unused_mut)]
    async fn create_email_identity(
        &self,
        input: CreateEmailIdentityRequest,
    ) -> Result<CreateEmailIdentityResponse, RusotoError<CreateEmailIdentityError>> {
        let request_uri = "/v1/email/identities";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateEmailIdentityResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateEmailIdentityError::from_response(response))
        }
    }

    /// <p>Delete an existing configuration set.</p> <p>In Amazon Pinpoint, <i>configuration sets</i> are groups of rules that you can apply to the emails you send. You apply a configuration set to an email by including a reference to the configuration set in the headers of the email. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email.</p>
    #[allow(unused_mut)]
    async fn delete_configuration_set(
        &self,
        input: DeleteConfigurationSetRequest,
    ) -> Result<DeleteConfigurationSetResponse, RusotoError<DeleteConfigurationSetError>> {
        let request_uri = format!(
            "/v1/email/configuration-sets/{configuration_set_name}",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteConfigurationSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteConfigurationSetError::from_response(response))
        }
    }

    /// <p>Delete an event destination.</p> <p>In Amazon Pinpoint, <i>events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p>
    #[allow(unused_mut)]
    async fn delete_configuration_set_event_destination(
        &self,
        input: DeleteConfigurationSetEventDestinationRequest,
    ) -> Result<
        DeleteConfigurationSetEventDestinationResponse,
        RusotoError<DeleteConfigurationSetEventDestinationError>,
    > {
        let request_uri = format!("/v1/email/configuration-sets/{configuration_set_name}/event-destinations/{event_destination_name}", configuration_set_name = input.configuration_set_name, event_destination_name = input.event_destination_name);

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteConfigurationSetEventDestinationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteConfigurationSetEventDestinationError::from_response(
                response,
            ))
        }
    }

    /// <p>Delete a dedicated IP pool.</p>
    #[allow(unused_mut)]
    async fn delete_dedicated_ip_pool(
        &self,
        input: DeleteDedicatedIpPoolRequest,
    ) -> Result<DeleteDedicatedIpPoolResponse, RusotoError<DeleteDedicatedIpPoolError>> {
        let request_uri = format!(
            "/v1/email/dedicated-ip-pools/{pool_name}",
            pool_name = input.pool_name
        );

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDedicatedIpPoolResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDedicatedIpPoolError::from_response(response))
        }
    }

    /// <p>Deletes an email identity that you previously verified for use with Amazon Pinpoint. An identity can be either an email address or a domain name.</p>
    #[allow(unused_mut)]
    async fn delete_email_identity(
        &self,
        input: DeleteEmailIdentityRequest,
    ) -> Result<DeleteEmailIdentityResponse, RusotoError<DeleteEmailIdentityError>> {
        let request_uri = format!(
            "/v1/email/identities/{email_identity}",
            email_identity = input.email_identity
        );

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteEmailIdentityResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEmailIdentityError::from_response(response))
        }
    }

    /// <p>Obtain information about the email-sending status and capabilities of your Amazon Pinpoint account in the current AWS Region.</p>
    #[allow(unused_mut)]
    async fn get_account(&self) -> Result<GetAccountResponse, RusotoError<GetAccountError>> {
        let request_uri = "/v1/email/account";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAccountResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAccountError::from_response(response))
        }
    }

    /// <p>Retrieve a list of the blacklists that your dedicated IP addresses appear on.</p>
    #[allow(unused_mut)]
    async fn get_blacklist_reports(
        &self,
        input: GetBlacklistReportsRequest,
    ) -> Result<GetBlacklistReportsResponse, RusotoError<GetBlacklistReportsError>> {
        let request_uri = "/v1/email/deliverability-dashboard/blacklist-report";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        for item in input.blacklist_item_names.iter() {
            params.put("BlacklistItemNames", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBlacklistReportsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBlacklistReportsError::from_response(response))
        }
    }

    /// <p>Get information about an existing configuration set, including the dedicated IP pool that it's associated with, whether or not it's enabled for sending email, and more.</p> <p>In Amazon Pinpoint, <i>configuration sets</i> are groups of rules that you can apply to the emails you send. You apply a configuration set to an email by including a reference to the configuration set in the headers of the email. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email.</p>
    #[allow(unused_mut)]
    async fn get_configuration_set(
        &self,
        input: GetConfigurationSetRequest,
    ) -> Result<GetConfigurationSetResponse, RusotoError<GetConfigurationSetError>> {
        let request_uri = format!(
            "/v1/email/configuration-sets/{configuration_set_name}",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetConfigurationSetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetConfigurationSetError::from_response(response))
        }
    }

    /// <p>Retrieve a list of event destinations that are associated with a configuration set.</p> <p>In Amazon Pinpoint, <i>events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p>
    #[allow(unused_mut)]
    async fn get_configuration_set_event_destinations(
        &self,
        input: GetConfigurationSetEventDestinationsRequest,
    ) -> Result<
        GetConfigurationSetEventDestinationsResponse,
        RusotoError<GetConfigurationSetEventDestinationsError>,
    > {
        let request_uri = format!(
            "/v1/email/configuration-sets/{configuration_set_name}/event-destinations",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetConfigurationSetEventDestinationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetConfigurationSetEventDestinationsError::from_response(
                response,
            ))
        }
    }

    /// <p>Get information about a dedicated IP address, including the name of the dedicated IP pool that it's associated with, as well information about the automatic warm-up process for the address.</p>
    #[allow(unused_mut)]
    async fn get_dedicated_ip(
        &self,
        input: GetDedicatedIpRequest,
    ) -> Result<GetDedicatedIpResponse, RusotoError<GetDedicatedIpError>> {
        let request_uri = format!("/v1/email/dedicated-ips/{ip}", ip = input.ip);

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDedicatedIpResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDedicatedIpError::from_response(response))
        }
    }

    /// <p>List the dedicated IP addresses that are associated with your Amazon Pinpoint account.</p>
    #[allow(unused_mut)]
    async fn get_dedicated_ips(
        &self,
        input: GetDedicatedIpsRequest,
    ) -> Result<GetDedicatedIpsResponse, RusotoError<GetDedicatedIpsError>> {
        let request_uri = "/v1/email/dedicated-ips";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        if let Some(ref x) = input.pool_name {
            params.put("PoolName", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDedicatedIpsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDedicatedIpsError::from_response(response))
        }
    }

    /// <p>Retrieve information about the status of the Deliverability dashboard for your Amazon Pinpoint account. When the Deliverability dashboard is enabled, you gain access to reputation, deliverability, and other metrics for the domains that you use to send email using Amazon Pinpoint. You also gain the ability to perform predictive inbox placement tests.</p> <p>When you use the Deliverability dashboard, you pay a monthly subscription charge, in addition to any other fees that you accrue by using Amazon Pinpoint. For more information about the features and cost of a Deliverability dashboard subscription, see <a href="http://aws.amazon.com/pinpoint/pricing/">Amazon Pinpoint Pricing</a>.</p>
    #[allow(unused_mut)]
    async fn get_deliverability_dashboard_options(
        &self,
    ) -> Result<
        GetDeliverabilityDashboardOptionsResponse,
        RusotoError<GetDeliverabilityDashboardOptionsError>,
    > {
        let request_uri = "/v1/email/deliverability-dashboard";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDeliverabilityDashboardOptionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDeliverabilityDashboardOptionsError::from_response(
                response,
            ))
        }
    }

    /// <p>Retrieve the results of a predictive inbox placement test.</p>
    #[allow(unused_mut)]
    async fn get_deliverability_test_report(
        &self,
        input: GetDeliverabilityTestReportRequest,
    ) -> Result<GetDeliverabilityTestReportResponse, RusotoError<GetDeliverabilityTestReportError>>
    {
        let request_uri = format!(
            "/v1/email/deliverability-dashboard/test-reports/{report_id}",
            report_id = input.report_id
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDeliverabilityTestReportResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDeliverabilityTestReportError::from_response(response))
        }
    }

    /// <p>Retrieve all the deliverability data for a specific campaign. This data is available for a campaign only if the campaign sent email by using a domain that the Deliverability dashboard is enabled for (<code>PutDeliverabilityDashboardOption</code> operation).</p>
    #[allow(unused_mut)]
    async fn get_domain_deliverability_campaign(
        &self,
        input: GetDomainDeliverabilityCampaignRequest,
    ) -> Result<
        GetDomainDeliverabilityCampaignResponse,
        RusotoError<GetDomainDeliverabilityCampaignError>,
    > {
        let request_uri = format!(
            "/v1/email/deliverability-dashboard/campaigns/{campaign_id}",
            campaign_id = input.campaign_id
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDomainDeliverabilityCampaignResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDomainDeliverabilityCampaignError::from_response(
                response,
            ))
        }
    }

    /// <p>Retrieve inbox placement and engagement rates for the domains that you use to send email.</p>
    #[allow(unused_mut)]
    async fn get_domain_statistics_report(
        &self,
        input: GetDomainStatisticsReportRequest,
    ) -> Result<GetDomainStatisticsReportResponse, RusotoError<GetDomainStatisticsReportError>>
    {
        let request_uri = format!(
            "/v1/email/deliverability-dashboard/statistics-report/{domain}",
            domain = input.domain
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        params.put("EndDate", &input.end_date);
        params.put("StartDate", &input.start_date);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDomainStatisticsReportResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDomainStatisticsReportError::from_response(response))
        }
    }

    /// <p>Provides information about a specific identity associated with your Amazon Pinpoint account, including the identity's verification status, its DKIM authentication status, and its custom Mail-From settings.</p>
    #[allow(unused_mut)]
    async fn get_email_identity(
        &self,
        input: GetEmailIdentityRequest,
    ) -> Result<GetEmailIdentityResponse, RusotoError<GetEmailIdentityError>> {
        let request_uri = format!(
            "/v1/email/identities/{email_identity}",
            email_identity = input.email_identity
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetEmailIdentityResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetEmailIdentityError::from_response(response))
        }
    }

    /// <p>List all of the configuration sets associated with your Amazon Pinpoint account in the current region.</p> <p>In Amazon Pinpoint, <i>configuration sets</i> are groups of rules that you can apply to the emails you send. You apply a configuration set to an email by including a reference to the configuration set in the headers of the email. When you apply a configuration set to an email, all of the rules in that configuration set are applied to the email.</p>
    #[allow(unused_mut)]
    async fn list_configuration_sets(
        &self,
        input: ListConfigurationSetsRequest,
    ) -> Result<ListConfigurationSetsResponse, RusotoError<ListConfigurationSetsError>> {
        let request_uri = "/v1/email/configuration-sets";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListConfigurationSetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListConfigurationSetsError::from_response(response))
        }
    }

    /// <p>List all of the dedicated IP pools that exist in your Amazon Pinpoint account in the current AWS Region.</p>
    #[allow(unused_mut)]
    async fn list_dedicated_ip_pools(
        &self,
        input: ListDedicatedIpPoolsRequest,
    ) -> Result<ListDedicatedIpPoolsResponse, RusotoError<ListDedicatedIpPoolsError>> {
        let request_uri = "/v1/email/dedicated-ip-pools";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDedicatedIpPoolsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDedicatedIpPoolsError::from_response(response))
        }
    }

    /// <p>Show a list of the predictive inbox placement tests that you've performed, regardless of their statuses. For predictive inbox placement tests that are complete, you can use the <code>GetDeliverabilityTestReport</code> operation to view the results.</p>
    #[allow(unused_mut)]
    async fn list_deliverability_test_reports(
        &self,
        input: ListDeliverabilityTestReportsRequest,
    ) -> Result<
        ListDeliverabilityTestReportsResponse,
        RusotoError<ListDeliverabilityTestReportsError>,
    > {
        let request_uri = "/v1/email/deliverability-dashboard/test-reports";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDeliverabilityTestReportsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDeliverabilityTestReportsError::from_response(response))
        }
    }

    /// <p>Retrieve deliverability data for all the campaigns that used a specific domain to send email during a specified time range. This data is available for a domain only if you enabled the Deliverability dashboard (<code>PutDeliverabilityDashboardOption</code> operation) for the domain.</p>
    #[allow(unused_mut)]
    async fn list_domain_deliverability_campaigns(
        &self,
        input: ListDomainDeliverabilityCampaignsRequest,
    ) -> Result<
        ListDomainDeliverabilityCampaignsResponse,
        RusotoError<ListDomainDeliverabilityCampaignsError>,
    > {
        let request_uri = format!(
            "/v1/email/deliverability-dashboard/domains/{subscribed_domain}/campaigns",
            subscribed_domain = input.subscribed_domain
        );

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        params.put("EndDate", &input.end_date);
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        params.put("StartDate", &input.start_date);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDomainDeliverabilityCampaignsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDomainDeliverabilityCampaignsError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns a list of all of the email identities that are associated with your Amazon Pinpoint account. An identity can be either an email address or a domain. This operation returns identities that are verified as well as those that aren't.</p>
    #[allow(unused_mut)]
    async fn list_email_identities(
        &self,
        input: ListEmailIdentitiesRequest,
    ) -> Result<ListEmailIdentitiesResponse, RusotoError<ListEmailIdentitiesError>> {
        let request_uri = "/v1/email/identities";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("PageSize", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListEmailIdentitiesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListEmailIdentitiesError::from_response(response))
        }
    }

    /// <p>Retrieve a list of the tags (keys and values) that are associated with a specified resource. A <i>tag</i> is a label that you optionally define and associate with a resource in Amazon Pinpoint. Each tag consists of a required <i>tag key</i> and an optional associated <i>tag value</i>. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = "/v1/email/tags";

        let mut request = SignedRequest::new("GET", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        params.put("ResourceArn", &input.resource_arn);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Enable or disable the automatic warm-up feature for dedicated IP addresses.</p>
    #[allow(unused_mut)]
    async fn put_account_dedicated_ip_warmup_attributes(
        &self,
        input: PutAccountDedicatedIpWarmupAttributesRequest,
    ) -> Result<
        PutAccountDedicatedIpWarmupAttributesResponse,
        RusotoError<PutAccountDedicatedIpWarmupAttributesError>,
    > {
        let request_uri = "/v1/email/account/dedicated-ips/warmup";

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutAccountDedicatedIpWarmupAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutAccountDedicatedIpWarmupAttributesError::from_response(
                response,
            ))
        }
    }

    /// <p>Enable or disable the ability of your account to send email.</p>
    #[allow(unused_mut)]
    async fn put_account_sending_attributes(
        &self,
        input: PutAccountSendingAttributesRequest,
    ) -> Result<PutAccountSendingAttributesResponse, RusotoError<PutAccountSendingAttributesError>>
    {
        let request_uri = "/v1/email/account/sending";

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutAccountSendingAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutAccountSendingAttributesError::from_response(response))
        }
    }

    /// <p>Associate a configuration set with a dedicated IP pool. You can use dedicated IP pools to create groups of dedicated IP addresses for sending specific types of email.</p>
    #[allow(unused_mut)]
    async fn put_configuration_set_delivery_options(
        &self,
        input: PutConfigurationSetDeliveryOptionsRequest,
    ) -> Result<
        PutConfigurationSetDeliveryOptionsResponse,
        RusotoError<PutConfigurationSetDeliveryOptionsError>,
    > {
        let request_uri = format!(
            "/v1/email/configuration-sets/{configuration_set_name}/delivery-options",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutConfigurationSetDeliveryOptionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutConfigurationSetDeliveryOptionsError::from_response(
                response,
            ))
        }
    }

    /// <p>Enable or disable collection of reputation metrics for emails that you send using a particular configuration set in a specific AWS Region.</p>
    #[allow(unused_mut)]
    async fn put_configuration_set_reputation_options(
        &self,
        input: PutConfigurationSetReputationOptionsRequest,
    ) -> Result<
        PutConfigurationSetReputationOptionsResponse,
        RusotoError<PutConfigurationSetReputationOptionsError>,
    > {
        let request_uri = format!(
            "/v1/email/configuration-sets/{configuration_set_name}/reputation-options",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutConfigurationSetReputationOptionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutConfigurationSetReputationOptionsError::from_response(
                response,
            ))
        }
    }

    /// <p>Enable or disable email sending for messages that use a particular configuration set in a specific AWS Region.</p>
    #[allow(unused_mut)]
    async fn put_configuration_set_sending_options(
        &self,
        input: PutConfigurationSetSendingOptionsRequest,
    ) -> Result<
        PutConfigurationSetSendingOptionsResponse,
        RusotoError<PutConfigurationSetSendingOptionsError>,
    > {
        let request_uri = format!(
            "/v1/email/configuration-sets/{configuration_set_name}/sending",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutConfigurationSetSendingOptionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutConfigurationSetSendingOptionsError::from_response(
                response,
            ))
        }
    }

    /// <p>Specify a custom domain to use for open and click tracking elements in email that you send using Amazon Pinpoint.</p>
    #[allow(unused_mut)]
    async fn put_configuration_set_tracking_options(
        &self,
        input: PutConfigurationSetTrackingOptionsRequest,
    ) -> Result<
        PutConfigurationSetTrackingOptionsResponse,
        RusotoError<PutConfigurationSetTrackingOptionsError>,
    > {
        let request_uri = format!(
            "/v1/email/configuration-sets/{configuration_set_name}/tracking-options",
            configuration_set_name = input.configuration_set_name
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutConfigurationSetTrackingOptionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutConfigurationSetTrackingOptionsError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Move a dedicated IP address to an existing dedicated IP pool.</p> <note> <p>The dedicated IP address that you specify must already exist, and must be associated with your Amazon Pinpoint account. </p> <p>The dedicated IP pool you specify must already exist. You can create a new pool by using the <code>CreateDedicatedIpPool</code> operation.</p> </note></p>
    #[allow(unused_mut)]
    async fn put_dedicated_ip_in_pool(
        &self,
        input: PutDedicatedIpInPoolRequest,
    ) -> Result<PutDedicatedIpInPoolResponse, RusotoError<PutDedicatedIpInPoolError>> {
        let request_uri = format!("/v1/email/dedicated-ips/{ip}/pool", ip = input.ip);

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutDedicatedIpInPoolResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutDedicatedIpInPoolError::from_response(response))
        }
    }

    /// <p><p/></p>
    #[allow(unused_mut)]
    async fn put_dedicated_ip_warmup_attributes(
        &self,
        input: PutDedicatedIpWarmupAttributesRequest,
    ) -> Result<
        PutDedicatedIpWarmupAttributesResponse,
        RusotoError<PutDedicatedIpWarmupAttributesError>,
    > {
        let request_uri = format!("/v1/email/dedicated-ips/{ip}/warmup", ip = input.ip);

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutDedicatedIpWarmupAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutDedicatedIpWarmupAttributesError::from_response(response))
        }
    }

    /// <p>Enable or disable the Deliverability dashboard for your Amazon Pinpoint account. When you enable the Deliverability dashboard, you gain access to reputation, deliverability, and other metrics for the domains that you use to send email using Amazon Pinpoint. You also gain the ability to perform predictive inbox placement tests.</p> <p>When you use the Deliverability dashboard, you pay a monthly subscription charge, in addition to any other fees that you accrue by using Amazon Pinpoint. For more information about the features and cost of a Deliverability dashboard subscription, see <a href="http://aws.amazon.com/pinpoint/pricing/">Amazon Pinpoint Pricing</a>.</p>
    #[allow(unused_mut)]
    async fn put_deliverability_dashboard_option(
        &self,
        input: PutDeliverabilityDashboardOptionRequest,
    ) -> Result<
        PutDeliverabilityDashboardOptionResponse,
        RusotoError<PutDeliverabilityDashboardOptionError>,
    > {
        let request_uri = "/v1/email/deliverability-dashboard";

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutDeliverabilityDashboardOptionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutDeliverabilityDashboardOptionError::from_response(
                response,
            ))
        }
    }

    /// <p>Used to enable or disable DKIM authentication for an email identity.</p>
    #[allow(unused_mut)]
    async fn put_email_identity_dkim_attributes(
        &self,
        input: PutEmailIdentityDkimAttributesRequest,
    ) -> Result<
        PutEmailIdentityDkimAttributesResponse,
        RusotoError<PutEmailIdentityDkimAttributesError>,
    > {
        let request_uri = format!(
            "/v1/email/identities/{email_identity}/dkim",
            email_identity = input.email_identity
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutEmailIdentityDkimAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutEmailIdentityDkimAttributesError::from_response(response))
        }
    }

    /// <p>Used to enable or disable feedback forwarding for an identity. This setting determines what happens when an identity is used to send an email that results in a bounce or complaint event.</p> <p>When you enable feedback forwarding, Amazon Pinpoint sends you email notifications when bounce or complaint events occur. Amazon Pinpoint sends this notification to the address that you specified in the Return-Path header of the original email.</p> <p>When you disable feedback forwarding, Amazon Pinpoint sends notifications through other mechanisms, such as by notifying an Amazon SNS topic. You're required to have a method of tracking bounces and complaints. If you haven't set up another mechanism for receiving bounce or complaint notifications, Amazon Pinpoint sends an email notification when these events occur (even if this setting is disabled).</p>
    #[allow(unused_mut)]
    async fn put_email_identity_feedback_attributes(
        &self,
        input: PutEmailIdentityFeedbackAttributesRequest,
    ) -> Result<
        PutEmailIdentityFeedbackAttributesResponse,
        RusotoError<PutEmailIdentityFeedbackAttributesError>,
    > {
        let request_uri = format!(
            "/v1/email/identities/{email_identity}/feedback",
            email_identity = input.email_identity
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutEmailIdentityFeedbackAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutEmailIdentityFeedbackAttributesError::from_response(
                response,
            ))
        }
    }

    /// <p>Used to enable or disable the custom Mail-From domain configuration for an email identity.</p>
    #[allow(unused_mut)]
    async fn put_email_identity_mail_from_attributes(
        &self,
        input: PutEmailIdentityMailFromAttributesRequest,
    ) -> Result<
        PutEmailIdentityMailFromAttributesResponse,
        RusotoError<PutEmailIdentityMailFromAttributesError>,
    > {
        let request_uri = format!(
            "/v1/email/identities/{email_identity}/mail-from",
            email_identity = input.email_identity
        );

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutEmailIdentityMailFromAttributesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutEmailIdentityMailFromAttributesError::from_response(
                response,
            ))
        }
    }

    /// <p><p>Sends an email message. You can use the Amazon Pinpoint Email API to send two types of messages:</p> <ul> <li> <p> <b>Simple</b> – A standard email message. When you create this type of message, you specify the sender, the recipient, and the message body, and Amazon Pinpoint assembles the message for you.</p> </li> <li> <p> <b>Raw</b> – A raw, MIME-formatted email message. When you send this type of email, you have to specify all of the message headers, as well as the message body. You can use this message type to send messages that contain attachments. The message that you specify has to be a valid MIME message.</p> </li> </ul></p>
    #[allow(unused_mut)]
    async fn send_email(
        &self,
        input: SendEmailRequest,
    ) -> Result<SendEmailResponse, RusotoError<SendEmailError>> {
        let request_uri = "/v1/email/outbound-emails";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SendEmailResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SendEmailError::from_response(response))
        }
    }

    /// <p>Add one or more tags (keys and values) to a specified resource. A <i>tag</i> is a label that you optionally define and associate with a resource in Amazon Pinpoint. Tags can help you categorize and manage resources in different ways, such as by purpose, owner, environment, or other criteria. A resource can have as many as 50 tags.</p> <p>Each tag consists of a required <i>tag key</i> and an associated <i>tag value</i>, both of which you define. A tag key is a general label that acts as a category for more specific tag values. A tag value acts as a descriptor within a tag key.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = "/v1/email/tags";

        let mut request = SignedRequest::new("POST", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Remove one or more tags (keys and values) from a specified resource.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = "/v1/email/tags";

        let mut request = SignedRequest::new("DELETE", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());

        let mut params = Params::new();
        params.put("ResourceArn", &input.resource_arn);
        for item in input.tag_keys.iter() {
            params.put("TagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Update the configuration of an event destination for a configuration set.</p> <p>In Amazon Pinpoint, <i>events</i> include message sends, deliveries, opens, clicks, bounces, and complaints. <i>Event destinations</i> are places that you can send information about these events to. For example, you can send event data to Amazon SNS to receive notifications when you receive bounces or complaints, or you can use Amazon Kinesis Data Firehose to stream data to Amazon S3 for long-term storage.</p>
    #[allow(unused_mut)]
    async fn update_configuration_set_event_destination(
        &self,
        input: UpdateConfigurationSetEventDestinationRequest,
    ) -> Result<
        UpdateConfigurationSetEventDestinationResponse,
        RusotoError<UpdateConfigurationSetEventDestinationError>,
    > {
        let request_uri = format!("/v1/email/configuration-sets/{configuration_set_name}/event-destinations/{event_destination_name}", configuration_set_name = input.configuration_set_name, event_destination_name = input.event_destination_name);

        let mut request = SignedRequest::new("PUT", "ses", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("email".to_string());
        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateConfigurationSetEventDestinationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateConfigurationSetEventDestinationError::from_response(
                response,
            ))
        }
    }
}
