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

impl StorageGatewayClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "storagegateway", &self.region, request_uri);

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
/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>ActivateGatewayInput$ActivationKey</a> </p> </li> <li> <p> <a>ActivateGatewayInput$GatewayName</a> </p> </li> <li> <p> <a>ActivateGatewayInput$GatewayRegion</a> </p> </li> <li> <p> <a>ActivateGatewayInput$GatewayTimezone</a> </p> </li> <li> <p> <a>ActivateGatewayInput$GatewayType</a> </p> </li> <li> <p> <a>ActivateGatewayInput$MediumChangerType</a> </p> </li> <li> <p> <a>ActivateGatewayInput$TapeDriveType</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ActivateGatewayInput {
    /// <p>Your gateway activation key. You can obtain the activation key by sending an HTTP GET request with redirects enabled to the gateway IP address (port 80). The redirect URL returned in the response provides you the activation key for your gateway in the query string parameter <code>activationKey</code>. It may also include other activation-related parameters, however, these are merely defaults -- the arguments you pass to the <code>ActivateGateway</code> API call determine the actual configuration of your gateway.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/get-activation-key.html">Getting activation key</a> in the <i>AWS Storage Gateway User Guide</i>.</p>
    #[serde(rename = "activationKey")]
    pub activation_key: String,
    /// <p>The name you configured for your gateway.</p>
    #[serde(rename = "gatewayName")]
    pub gateway_name: String,
    /// <p>A value that indicates the AWS Region where you want to store your data. The gateway AWS Region specified must be the same AWS Region as the AWS Region in your <code>Host</code> header in the request. For more information about available AWS Regions and endpoints for AWS Storage Gateway, see <a href="https://docs.aws.amazon.com/general/latest/gr/sg.html">AWS Storage Gateway endpoints and quotas</a> in the <i>AWS General Reference</i>.</p> <p>Valid Values: See <a href="https://docs.aws.amazon.com/general/latest/gr/sg.html">AWS Storage Gateway endpoints and quotas</a> in the <i>AWS General Reference</i>. </p>
    #[serde(rename = "gatewayRegion")]
    pub gateway_region: String,
    /// <p>A value that indicates the time zone you want to set for the gateway. The time zone is of the format "GMT-hr:mm" or "GMT+hr:mm". For example, GMT-4:00 indicates the time is 4 hours behind GMT. GMT+2:00 indicates the time is 2 hours ahead of GMT. The time zone is used, for example, for scheduling snapshots and your gateway's maintenance schedule.</p>
    #[serde(rename = "gatewayTimezone")]
    pub gateway_timezone: String,
    /// <p>A value that defines the type of gateway to activate. The type specified is critical to all later functions of the gateway and cannot be changed after activation. The default value is <code>CACHED</code>.</p> <p>Valid Values: <code>STORED</code> | <code>CACHED</code> | <code>VTL</code> | <code>FILE_S3</code> </p>
    #[serde(rename = "gatewayType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_type: Option<String>,
    /// <p>The value that indicates the type of medium changer to use for tape gateway. This field is optional.</p> <p>Valid Values: <code>STK-L700</code> | <code>AWS-Gateway-VTL</code> | <code>IBM-03584L32-0402</code> </p>
    #[serde(rename = "mediumChangerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_changer_type: Option<String>,
    /// <p><p>A list of up to 50 tags that you can assign to the gateway. Each tag is a key-value pair.</p> <note> <p>Valid characters for key and value are letters, spaces, and numbers that can be represented in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag&#39;s key is 128 characters, and the maximum length for a tag&#39;s value is 256 characters.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The value that indicates the type of tape drive to use for tape gateway. This field is optional.</p> <p>Valid Values: <code>IBM-ULT3580-TD5</code> </p>
    #[serde(rename = "tapeDriveType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_drive_type: Option<String>,
}

/// <p><p>AWS Storage Gateway returns the Amazon Resource Name (ARN) of the activated gateway. It is a string made of information such as your account, gateway name, and AWS Region. This ARN is used to reference the gateway in other API operations as well as resource-based authorization.</p> <note> <p>For gateways activated prior to September 02, 2015, the gateway ARN contains the gateway name rather than the gateway ID. Changing the name of the gateway has no effect on the gateway ARN.</p> </note></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ActivateGatewayOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddCacheInput {
    /// <p>An array of strings that identify disks that are to be configured as working storage. Each string has a minimum length of 1 and maximum length of 300. You can get the disk IDs from the <a>ListLocalDisks</a> API.</p>
    #[serde(rename = "diskIds")]
    pub disk_ids: Vec<String>,
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddCacheOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>AddTagsToResourceInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsToResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource you want to add tags to.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p><p>The key-value pair that represents the tag you want to add to the resource. The value can be an empty string.</p> <note> <p>Valid characters for key and value are letters, spaces, and numbers representable in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag&#39;s key is 128 characters, and the maximum length for a tag&#39;s value is 256.</p> </note></p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

/// <p>AddTagsToResourceOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddTagsToResourceOutput {
    /// <p>The Amazon Resource Name (ARN) of the resource you want to add tags to.</p>
    #[serde(rename = "resourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddUploadBufferInput {
    /// <p>An array of strings that identify disks that are to be configured as working storage. Each string has a minimum length of 1 and maximum length of 300. You can get the disk IDs from the <a>ListLocalDisks</a> API.</p>
    #[serde(rename = "diskIds")]
    pub disk_ids: Vec<String>,
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddUploadBufferOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>AddWorkingStorageInput$DiskIds</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddWorkingStorageInput {
    /// <p>An array of strings that identify disks that are to be configured as working storage. Each string has a minimum length of 1 and maximum length of 300. You can get the disk IDs from the <a>ListLocalDisks</a> API.</p>
    #[serde(rename = "diskIds")]
    pub disk_ids: Vec<String>,
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway for which working storage was configured.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddWorkingStorageOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssignTapePoolInput {
    /// <p>Set permissions to bypass governance retention. If the lock type of the archived tape is <code>Governance</code>, the tape's archived age is not older than <code>RetentionLockInDays</code>, and the user does not already have <code>BypassGovernanceRetention</code>, setting this to TRUE enables the user to bypass the retention lock. This parameter is set to true by default for calls from the console.</p> <p>Valid values: <code>TRUE</code> | <code>FALSE</code> </p>
    #[serde(rename = "bypassGovernanceRetention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_governance_retention: Option<bool>,
    /// <p>The ID of the pool that you want to add your tape to for archiving. The tape in this pool is archived in the S3 storage class that is associated with the pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class (S3 Glacier or S3 Glacier Deep Archive) that corresponds to the pool.</p> <p>Valid Values: <code>GLACIER</code> | <code>DEEP_ARCHIVE</code> </p>
    #[serde(rename = "poolId")]
    pub pool_id: String,
    /// <p>The unique Amazon Resource Name (ARN) of the virtual tape that you want to add to the tape pool.</p>
    #[serde(rename = "tapeARN")]
    pub tape_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssignTapePoolOutput {
    /// <p>The unique Amazon Resource Names (ARN) of the virtual tape that was added to the tape pool.</p>
    #[serde(rename = "tapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateFileSystemInput {
    /// <p>The Amazon Resource Name (ARN) of the storage used for the audit logs.</p>
    #[serde(rename = "auditDestinationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_destination_arn: Option<String>,
    #[serde(rename = "cacheAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_attributes: Option<CacheAttributes>,
    /// <p>A unique string value that you supply that is used by the file gateway to ensure idempotent file system association creation.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the Amazon FSx file system to associate with the Amazon FSx file gateway.</p>
    #[serde(rename = "locationARN")]
    pub location_arn: String,
    /// <p>The password of the user credential.</p>
    #[serde(rename = "password")]
    pub password: String,
    /// <p>A list of up to 50 tags that can be assigned to the file system association. Each tag is a key-value pair.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The user name of the user credential that has permission to access the root share D$ of the Amazon FSx file system. The user account must belong to the Amazon FSx delegated admin user group.</p>
    #[serde(rename = "userName")]
    pub user_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateFileSystemOutput {
    /// <p>The ARN of the newly created file system association.</p>
    #[serde(rename = "fileSystemAssociationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_association_arn: Option<String>,
}

/// <p>AttachVolumeInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AttachVolumeInput {
    /// <p>The unique device ID or other distinguishing data that identifies the local disk used to create the volume. This value is only required when you are attaching a stored volume.</p>
    #[serde(rename = "diskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the gateway that you want to attach the volume to.</p>
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>The network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted. Use <a>DescribeGatewayInformation</a> to get a list of the network interfaces available on a gateway.</p> <p>Valid Values: A valid IP address.</p>
    #[serde(rename = "networkInterfaceId")]
    pub network_interface_id: String,
    /// <p>The name of the iSCSI target used by an initiator to connect to a volume and used as a suffix for the target ARN. For example, specifying <code>TargetName</code> as <i>myvolume</i> results in the target ARN of <code>arn:aws:storagegateway:us-east-2:111122223333:gateway/sgw-12A3456B/target/iqn.1997-05.com.amazon:myvolume</code>. The target name must be unique across all volumes on a gateway.</p> <p>If you don't specify a value, Storage Gateway uses the value that was previously used for this volume as the new target name.</p>
    #[serde(rename = "targetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the volume to attach to the specified gateway.</p>
    #[serde(rename = "volumeARN")]
    pub volume_arn: String,
}

/// <p>AttachVolumeOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AttachVolumeOutput {
    /// <p>The Amazon Resource Name (ARN) of the volume target, which includes the iSCSI name for the initiator that was used to connect to the target.</p>
    #[serde(rename = "targetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the volume that was attached to the gateway.</p>
    #[serde(rename = "volumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

/// <p>Information about the gateway's automatic tape creation policies, including the automatic tape creation rules and the gateway that is using the policies.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AutomaticTapeCreationPolicyInfo {
    /// <p>An automatic tape creation policy consists of a list of automatic tape creation rules. This returns the rules that determine when and how to automatically create new tapes.</p>
    #[serde(rename = "automaticTapeCreationRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tape_creation_rules: Option<Vec<AutomaticTapeCreationRule>>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>An automatic tape creation policy consists of automatic tape creation rules where each rule defines when and how to create new tapes. For more information about automatic tape creation, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/GettingStartedCreateTapes.html#CreateTapesAutomatically">Creating Tapes Automatically</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AutomaticTapeCreationRule {
    /// <p>The minimum number of available virtual tapes that the gateway maintains at all times. If the number of tapes on the gateway goes below this value, the gateway creates as many new tapes as are needed to have <code>MinimumNumTapes</code> on the gateway. For more information about automatic tape creation, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/GettingStartedCreateTapes.html#CreateTapesAutomatically">Creating Tapes Automatically</a>.</p>
    #[serde(rename = "minimumNumTapes")]
    pub minimum_num_tapes: i64,
    /// <p>The ID of the pool that you want to add your tape to for archiving. The tape in this pool is archived in the Amazon S3 storage class that is associated with the pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class (S3 Glacier or S3 Glacier Deep Archive) that corresponds to the pool.</p> <p>Valid Values: <code>GLACIER</code> | <code>DEEP_ARCHIVE</code> </p>
    #[serde(rename = "poolId")]
    pub pool_id: String,
    /// <p><p>A prefix that you append to the barcode of the virtual tape that you are creating. This prefix makes the barcode unique.</p> <note> <p>The prefix must be 1-4 characters in length and must be one of the uppercase letters from A to Z.</p> </note></p>
    #[serde(rename = "tapeBarcodePrefix")]
    pub tape_barcode_prefix: String,
    /// <p>The size, in bytes, of the virtual tape capacity.</p>
    #[serde(rename = "tapeSizeInBytes")]
    pub tape_size_in_bytes: i64,
    /// <p>Set to <code>true</code> to indicate that tapes are to be archived as write-once-read-many (WORM). Set to <code>false</code> when WORM is not enabled for tapes.</p>
    #[serde(rename = "worm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worm: Option<bool>,
}

/// <p> Describes a bandwidth rate limit interval for a gateway. A bandwidth rate limit schedule consists of one or more bandwidth rate limit intervals. A bandwidth rate limit interval defines a period of time on one or more days of the week, during which bandwidth rate limits are specified for uploading, downloading, or both. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BandwidthRateLimitInterval {
    /// <p> The average download rate limit component of the bandwidth rate limit interval, in bits per second. This field does not appear in the response if the download rate limit is not set. </p>
    #[serde(rename = "averageDownloadRateLimitInBitsPerSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_download_rate_limit_in_bits_per_sec: Option<i64>,
    /// <p> The average upload rate limit component of the bandwidth rate limit interval, in bits per second. This field does not appear in the response if the upload rate limit is not set. </p>
    #[serde(rename = "averageUploadRateLimitInBitsPerSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_upload_rate_limit_in_bits_per_sec: Option<i64>,
    /// <p> The days of the week component of the bandwidth rate limit interval, represented as ordinal numbers from 0 to 6, where 0 represents Sunday and 6 represents Saturday. </p>
    #[serde(rename = "daysOfWeek")]
    pub days_of_week: Vec<i64>,
    /// <p> The hour of the day to end the bandwidth rate limit interval. </p>
    #[serde(rename = "endHourOfDay")]
    pub end_hour_of_day: i64,
    /// <p><p> The minute of the hour to end the bandwidth rate limit interval. </p> <important> <p> The bandwidth rate limit interval ends at the end of the minute. To end an interval at the end of an hour, use the value <code>59</code>. </p> </important></p>
    #[serde(rename = "endMinuteOfHour")]
    pub end_minute_of_hour: i64,
    /// <p> The hour of the day to start the bandwidth rate limit interval. </p>
    #[serde(rename = "startHourOfDay")]
    pub start_hour_of_day: i64,
    /// <p> The minute of the hour to start the bandwidth rate limit interval. The interval begins at the start of that minute. To begin an interval exactly at the start of the hour, use the value <code>0</code>. </p>
    #[serde(rename = "startMinuteOfHour")]
    pub start_minute_of_hour: i64,
}

/// <p>The refresh cache information for the file share.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CacheAttributes {
    /// <p>Refreshes a file share's cache by using Time To Live (TTL). TTL is the length of time since the last refresh after which access to the directory would cause the file gateway to first refresh that directory's contents from the Amazon S3 bucket or Amazon FSx file system. The TTL duration is in seconds.</p> <p>Valid Values: 300 to 2,592,000 seconds (5 minutes to 30 days)</p>
    #[serde(rename = "cacheStaleTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_stale_timeout_in_seconds: Option<i64>,
}

/// <p>Describes an iSCSI cached volume.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CachediSCSIVolume {
    /// <p>The date the volume was created. Volumes created prior to March 28, 2017 don’t have this timestamp.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>If the cached volume was created from a snapshot, this field contains the snapshot ID used, e.g., snap-78e22663. Otherwise, this field is not included.</p>
    #[serde(rename = "sourceSnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_snapshot_id: Option<String>,
    /// <p>The name of the iSCSI target used by an initiator to connect to a volume and used as a suffix for the target ARN. For example, specifying <code>TargetName</code> as <i>myvolume</i> results in the target ARN of <code>arn:aws:storagegateway:us-east-2:111122223333:gateway/sgw-12A3456B/target/iqn.1997-05.com.amazon:myvolume</code>. The target name must be unique across all volumes on a gateway.</p> <p>If you don't specify a value, Storage Gateway uses the value that was previously used for this volume as the new target name.</p>
    #[serde(rename = "targetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the storage volume.</p>
    #[serde(rename = "volumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
    /// <p>A value that indicates whether a storage volume is attached to or detached from a gateway. For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/managing-volumes.html#attach-detach-volume">Moving your volumes to a different gateway</a>.</p>
    #[serde(rename = "volumeAttachmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_attachment_status: Option<String>,
    /// <p>The unique identifier of the volume, e.g., vol-AE4B946D.</p>
    #[serde(rename = "volumeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    /// <p>Represents the percentage complete if the volume is restoring or bootstrapping that represents the percent of data transferred. This field does not appear in the response if the cached volume is not restoring or bootstrapping.</p>
    #[serde(rename = "volumeProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_progress: Option<f64>,
    /// <p>The size, in bytes, of the volume capacity.</p>
    #[serde(rename = "volumeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_bytes: Option<i64>,
    /// <p>One of the VolumeStatus values that indicates the state of the storage volume.</p>
    #[serde(rename = "volumeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_status: Option<String>,
    /// <p>One of the VolumeType enumeration values that describes the type of the volume.</p>
    #[serde(rename = "volumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
    /// <p><p>The size of the data stored on the volume in bytes. This value is calculated based on the number of blocks that are touched, instead of the actual amount of data written. This value can be useful for sequential write patterns but less accurate for random write patterns. <code>VolumeUsedInBytes</code> is different from the compressed size of the volume, which is the value that is used to calculate your bill.</p> <note> <p>This value is not available for volumes created prior to May 13, 2015, until you store data on the volume.</p> </note></p>
    #[serde(rename = "volumeUsedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_used_in_bytes: Option<i64>,
    /// <p>An <a>VolumeiSCSIAttributes</a> object that represents a collection of iSCSI attributes for one stored volume.</p>
    #[serde(rename = "volumeiSCSIAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumei_scsi_attributes: Option<VolumeiSCSIAttributes>,
}

/// <p>CancelArchivalInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelArchivalInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the virtual tape you want to cancel archiving for.</p>
    #[serde(rename = "tapeARN")]
    pub tape_arn: String,
}

/// <p>CancelArchivalOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelArchivalOutput {
    /// <p>The Amazon Resource Name (ARN) of the virtual tape for which archiving was canceled.</p>
    #[serde(rename = "tapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

/// <p>CancelRetrievalInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelRetrievalInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the virtual tape you want to cancel retrieval for.</p>
    #[serde(rename = "tapeARN")]
    pub tape_arn: String,
}

/// <p>CancelRetrievalOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelRetrievalOutput {
    /// <p>The Amazon Resource Name (ARN) of the virtual tape for which retrieval was canceled.</p>
    #[serde(rename = "tapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

/// <p>Describes Challenge-Handshake Authentication Protocol (CHAP) information that supports authentication between your gateway and iSCSI initiators.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ChapInfo {
    /// <p>The iSCSI initiator that connects to the target.</p>
    #[serde(rename = "initiatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<String>,
    /// <p>The secret key that the initiator (for example, the Windows client) must provide to participate in mutual CHAP with the target.</p>
    #[serde(rename = "secretToAuthenticateInitiator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_to_authenticate_initiator: Option<String>,
    /// <p>The secret key that the target must provide to participate in mutual CHAP with the initiator (e.g., Windows client).</p>
    #[serde(rename = "secretToAuthenticateTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_to_authenticate_target: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the volume.</p> <p>Valid Values: 50 to 500 lowercase letters, numbers, periods (.), and hyphens (-).</p>
    #[serde(rename = "targetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCachediSCSIVolumeInput {
    /// <p>A unique identifier that you use to retry a request. If you retry a request, use the same <code>ClientToken</code> you specified in the initial request.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own AWS KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "kMSEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encrypted: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted. Use <a>DescribeGatewayInformation</a> to get a list of the network interfaces available on a gateway.</p> <p>Valid Values: A valid IP address.</p>
    #[serde(rename = "networkInterfaceId")]
    pub network_interface_id: String,
    /// <p>The snapshot ID (e.g. "snap-1122aabb") of the snapshot to restore as the new cached volume. Specify this field if you want to create the iSCSI storage volume from a snapshot; otherwise, do not include this field. To list snapshots for your account use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-DescribeSnapshots.html">DescribeSnapshots</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p>
    #[serde(rename = "snapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// <p>The ARN for an existing volume. Specifying this ARN makes the new volume into an exact copy of the specified existing volume's latest recovery point. The <code>VolumeSizeInBytes</code> value for this new volume must be equal to or larger than the size of the existing volume, in bytes.</p>
    #[serde(rename = "sourceVolumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_volume_arn: Option<String>,
    /// <p><p>A list of up to 50 tags that you can assign to a cached volume. Each tag is a key-value pair.</p> <note> <p>Valid characters for key and value are letters, spaces, and numbers that you can represent in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag&#39;s key is 128 characters, and the maximum length for a tag&#39;s value is 256 characters.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The name of the iSCSI target used by an initiator to connect to a volume and used as a suffix for the target ARN. For example, specifying <code>TargetName</code> as <i>myvolume</i> results in the target ARN of <code>arn:aws:storagegateway:us-east-2:111122223333:gateway/sgw-12A3456B/target/iqn.1997-05.com.amazon:myvolume</code>. The target name must be unique across all volumes on a gateway.</p> <p>If you don't specify a value, Storage Gateway uses the value that was previously used for this volume as the new target name.</p>
    #[serde(rename = "targetName")]
    pub target_name: String,
    /// <p>The size of the volume in bytes.</p>
    #[serde(rename = "volumeSizeInBytes")]
    pub volume_size_in_bytes: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCachediSCSIVolumeOutput {
    /// <p>The Amazon Resource Name (ARN) of the volume target, which includes the iSCSI name that initiators can use to connect to the target.</p>
    #[serde(rename = "targetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the configured volume.</p>
    #[serde(rename = "volumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

/// <p>CreateNFSFileShareInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateNFSFileShareInput {
    /// <p>Specifies refresh cache information for the file share.</p>
    #[serde(rename = "cacheAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_attributes: Option<CacheAttributes>,
    /// <p>The list of clients that are allowed to access the file gateway. The list must contain either valid IP addresses or valid CIDR blocks.</p>
    #[serde(rename = "clientList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_list: Option<Vec<String>>,
    /// <p>A unique string value that you supply that is used by file gateway to ensure idempotent file share creation.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The default storage class for objects put into an Amazon S3 bucket by the file gateway. The default value is <code>S3_INTELLIGENT_TIERING</code>. Optional.</p> <p>Valid Values: <code>S3_STANDARD</code> | <code>S3_INTELLIGENT_TIERING</code> | <code>S3_STANDARD_IA</code> | <code>S3_ONEZONE_IA</code> </p>
    #[serde(rename = "defaultStorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_storage_class: Option<String>,
    /// <p><p>The name of the file share. Optional.</p> <note> <p> <code>FileShareName</code> must be set if an S3 prefix name is set in <code>LocationARN</code>.</p> </note></p>
    #[serde(rename = "fileShareName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the file gateway on which you want to create a file share.</p>
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>A value that enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to <code>true</code> to enable MIME type guessing, otherwise set to <code>false</code>. The default value is <code>true</code>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "guessMIMETypeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guess_mime_type_enabled: Option<bool>,
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own AWS KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "kMSEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encrypted: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The ARN of the backend storage used for storing file data. A prefix name can be added to the S3 bucket name. It must end with a "/".</p>
    #[serde(rename = "locationARN")]
    pub location_arn: String,
    /// <p>File share default values. Optional.</p>
    #[serde(rename = "nFSFileShareDefaults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_file_share_defaults: Option<NFSFileShareDefaults>,
    /// <p>The notification policy of the file share. <code>SettlingTimeInSeconds</code> controls the number of seconds to wait after the last point in time a client wrote to a file before generating an <code>ObjectUploaded</code> notification. Because clients can make many small writes to files, it's best to set this parameter for as long as possible to avoid generating multiple notifications for the same file in a small time period.</p> <note> <p> <code>SettlingTimeInSeconds</code> has no effect on the timing of the object uploading to Amazon S3, only the timing of the notification.</p> </note> <p>The following example sets <code>NotificationPolicy</code> on with <code>SettlingTimeInSeconds</code> set to 60.</p> <p> <code>{\"Upload\": {\"SettlingTimeInSeconds\": 60}}</code> </p> <p>The following example sets <code>NotificationPolicy</code> off.</p> <p> <code>{}</code> </p>
    #[serde(rename = "notificationPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_policy: Option<String>,
    /// <p>A value that sets the access control list (ACL) permission for objects in the S3 bucket that a file gateway puts objects into. The default value is <code>private</code>.</p>
    #[serde(rename = "objectACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_acl: Option<String>,
    /// <p>A value that sets the write status of a file share. Set this value to <code>true</code> to set the write status to read-only, otherwise set to <code>false</code>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>A value that sets who pays the cost of the request and the cost associated with data download from the S3 bucket. If this value is set to <code>true</code>, the requester pays the costs; otherwise, the S3 bucket owner pays. However, the S3 bucket owner always pays the cost of storing data.</p> <note> <p> <code>RequesterPays</code> is a configuration for the S3 bucket that backs the file share, so make sure that the configuration on the file share is the same as the S3 bucket configuration.</p> </note> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "requesterPays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_pays: Option<bool>,
    /// <p>The ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage.</p>
    #[serde(rename = "role")]
    pub role: String,
    /// <p><p>A value that maps a user to anonymous user.</p> <p>Valid values are the following:</p> <ul> <li> <p> <code>RootSquash</code>: Only root is mapped to anonymous user.</p> </li> <li> <p> <code>NoSquash</code>: No one is mapped to anonymous user.</p> </li> <li> <p> <code>AllSquash</code>: Everyone is mapped to anonymous user.</p> </li> </ul></p>
    #[serde(rename = "squash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash: Option<String>,
    /// <p><p>A list of up to 50 tags that can be assigned to the NFS file share. Each tag is a key-value pair.</p> <note> <p>Valid characters for key and value are letters, spaces, and numbers representable in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag&#39;s key is 128 characters, and the maximum length for a tag&#39;s value is 256.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>CreateNFSFileShareOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateNFSFileShareOutput {
    /// <p>The Amazon Resource Name (ARN) of the newly created file share.</p>
    #[serde(rename = "fileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
}

/// <p>CreateSMBFileShareInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSMBFileShareInput {
    /// <p>The files and folders on this share will only be visible to users with read access.</p>
    #[serde(rename = "accessBasedEnumeration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_based_enumeration: Option<bool>,
    /// <p><p>A list of users or groups in the Active Directory that will be granted administrator privileges on the file share. These users can do all file operations as the super-user. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>.</p> <important> <p>Use this option very carefully, because any user in this list can do anything they like on the file share, regardless of file permissions.</p> </important></p>
    #[serde(rename = "adminUserList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_user_list: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the storage used for audit logs.</p>
    #[serde(rename = "auditDestinationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_destination_arn: Option<String>,
    /// <p>The authentication method that users use to access the file share. The default is <code>ActiveDirectory</code>.</p> <p>Valid Values: <code>ActiveDirectory</code> | <code>GuestAccess</code> </p>
    #[serde(rename = "authentication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
    /// <p>Specifies refresh cache information for the file share.</p>
    #[serde(rename = "cacheAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_attributes: Option<CacheAttributes>,
    /// <p>The case of an object name in an Amazon S3 bucket. For <code>ClientSpecified</code>, the client determines the case sensitivity. For <code>CaseSensitive</code>, the gateway determines the case sensitivity. The default value is <code>ClientSpecified</code>.</p>
    #[serde(rename = "caseSensitivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitivity: Option<String>,
    /// <p>A unique string value that you supply that is used by file gateway to ensure idempotent file share creation.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The default storage class for objects put into an Amazon S3 bucket by the file gateway. The default value is <code>S3_INTELLIGENT_TIERING</code>. Optional.</p> <p>Valid Values: <code>S3_STANDARD</code> | <code>S3_INTELLIGENT_TIERING</code> | <code>S3_STANDARD_IA</code> | <code>S3_ONEZONE_IA</code> </p>
    #[serde(rename = "defaultStorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_storage_class: Option<String>,
    /// <p><p>The name of the file share. Optional.</p> <note> <p> <code>FileShareName</code> must be set if an S3 prefix name is set in <code>LocationARN</code>.</p> </note></p>
    #[serde(rename = "fileShareName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_name: Option<String>,
    /// <p>The ARN of the file gateway on which you want to create a file share.</p>
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>A value that enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to <code>true</code> to enable MIME type guessing, otherwise set to <code>false</code>. The default value is <code>true</code>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "guessMIMETypeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guess_mime_type_enabled: Option<bool>,
    /// <p>A list of users or groups in the Active Directory that are not allowed to access the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    #[serde(rename = "invalidUserList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_user_list: Option<Vec<String>>,
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own AWS KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "kMSEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encrypted: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The ARN of the backend storage used for storing file data. A prefix name can be added to the S3 bucket name. It must end with a "/".</p>
    #[serde(rename = "locationARN")]
    pub location_arn: String,
    /// <p>The notification policy of the file share. <code>SettlingTimeInSeconds</code> controls the number of seconds to wait after the last point in time a client wrote to a file before generating an <code>ObjectUploaded</code> notification. Because clients can make many small writes to files, it's best to set this parameter for as long as possible to avoid generating multiple notifications for the same file in a small time period.</p> <note> <p> <code>SettlingTimeInSeconds</code> has no effect on the timing of the object uploading to Amazon S3, only the timing of the notification.</p> </note> <p>The following example sets <code>NotificationPolicy</code> on with <code>SettlingTimeInSeconds</code> set to 60.</p> <p> <code>{\"Upload\": {\"SettlingTimeInSeconds\": 60}}</code> </p> <p>The following example sets <code>NotificationPolicy</code> off.</p> <p> <code>{}</code> </p>
    #[serde(rename = "notificationPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_policy: Option<String>,
    /// <p>A value that sets the access control list (ACL) permission for objects in the S3 bucket that a file gateway puts objects into. The default value is <code>private</code>.</p>
    #[serde(rename = "objectACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_acl: Option<String>,
    /// <p>A value that sets the write status of a file share. Set this value to <code>true</code> to set the write status to read-only, otherwise set to <code>false</code>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>A value that sets who pays the cost of the request and the cost associated with data download from the S3 bucket. If this value is set to <code>true</code>, the requester pays the costs; otherwise, the S3 bucket owner pays. However, the S3 bucket owner always pays the cost of storing data.</p> <note> <p> <code>RequesterPays</code> is a configuration for the S3 bucket that backs the file share, so make sure that the configuration on the file share is the same as the S3 bucket configuration.</p> </note> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "requesterPays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_pays: Option<bool>,
    /// <p>The ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage.</p>
    #[serde(rename = "role")]
    pub role: String,
    /// <p>Set this value to <code>true</code> to enable access control list (ACL) on the SMB file share. Set it to <code>false</code> to map file and directory permissions to the POSIX permissions.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/smb-acl.html">Using Microsoft Windows ACLs to control access to an SMB file share</a> in the <i>AWS Storage Gateway User Guide</i>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "sMBACLEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smbacl_enabled: Option<bool>,
    /// <p><p>A list of up to 50 tags that can be assigned to the NFS file share. Each tag is a key-value pair.</p> <note> <p>Valid characters for key and value are letters, spaces, and numbers representable in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag&#39;s key is 128 characters, and the maximum length for a tag&#39;s value is 256.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A list of users or groups in the Active Directory that are allowed to access the file <a href=""/> share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    #[serde(rename = "validUserList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_user_list: Option<Vec<String>>,
}

/// <p>CreateSMBFileShareOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSMBFileShareOutput {
    /// <p>The Amazon Resource Name (ARN) of the newly created file share.</p>
    #[serde(rename = "fileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSnapshotFromVolumeRecoveryPointInput {
    /// <p>Textual description of the snapshot that appears in the Amazon EC2 console, Elastic Block Store snapshots panel in the <b>Description</b> field, and in the AWS Storage Gateway snapshot <b>Details</b> pane, <b>Description</b> field.</p>
    #[serde(rename = "snapshotDescription")]
    pub snapshot_description: String,
    /// <p><p>A list of up to 50 tags that can be assigned to a snapshot. Each tag is a key-value pair.</p> <note> <p>Valid characters for key and value are letters, spaces, and numbers representable in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag&#39;s key is 128 characters, and the maximum length for a tag&#39;s value is 256.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <a>DescribeStorediSCSIVolumes</a> operation to return to retrieve the TargetARN for specified VolumeARN.</p>
    #[serde(rename = "volumeARN")]
    pub volume_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSnapshotFromVolumeRecoveryPointOutput {
    /// <p>The ID of the snapshot.</p>
    #[serde(rename = "snapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <a>DescribeStorediSCSIVolumes</a> operation to return to retrieve the TargetARN for specified VolumeARN.</p>
    #[serde(rename = "volumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
    /// <p>The time the volume was created from the recovery point.</p>
    #[serde(rename = "volumeRecoveryPointTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_recovery_point_time: Option<String>,
}

/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>CreateSnapshotInput$SnapshotDescription</a> </p> </li> <li> <p> <a>CreateSnapshotInput$VolumeARN</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSnapshotInput {
    /// <p>Textual description of the snapshot that appears in the Amazon EC2 console, Elastic Block Store snapshots panel in the <b>Description</b> field, and in the AWS Storage Gateway snapshot <b>Details</b> pane, <b>Description</b> field.</p>
    #[serde(rename = "snapshotDescription")]
    pub snapshot_description: String,
    /// <p><p>A list of up to 50 tags that can be assigned to a snapshot. Each tag is a key-value pair.</p> <note> <p>Valid characters for key and value are letters, spaces, and numbers representable in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag&#39;s key is 128 characters, and the maximum length for a tag&#39;s value is 256.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The Amazon Resource Name (ARN) of the volume. Use the <a>ListVolumes</a> operation to return a list of gateway volumes.</p>
    #[serde(rename = "volumeARN")]
    pub volume_arn: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateSnapshotOutput {
    /// <p>The snapshot ID that is used to refer to the snapshot in future operations such as describing snapshots (Amazon Elastic Compute Cloud API <code>DescribeSnapshots</code>) or creating a volume from a snapshot (<a>CreateStorediSCSIVolume</a>).</p>
    #[serde(rename = "snapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the volume of which the snapshot was taken.</p>
    #[serde(rename = "volumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>CreateStorediSCSIVolumeInput$DiskId</a> </p> </li> <li> <p> <a>CreateStorediSCSIVolumeInput$NetworkInterfaceId</a> </p> </li> <li> <p> <a>CreateStorediSCSIVolumeInput$PreserveExistingData</a> </p> </li> <li> <p> <a>CreateStorediSCSIVolumeInput$SnapshotId</a> </p> </li> <li> <p> <a>CreateStorediSCSIVolumeInput$TargetName</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateStorediSCSIVolumeInput {
    /// <p>The unique identifier for the gateway local disk that is configured as a stored volume. Use <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/API_ListLocalDisks.html">ListLocalDisks</a> to list disk IDs for a gateway.</p>
    #[serde(rename = "diskId")]
    pub disk_id: String,
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own AWS KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "kMSEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encrypted: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted. Use <a>DescribeGatewayInformation</a> to get a list of the network interfaces available on a gateway.</p> <p>Valid Values: A valid IP address.</p>
    #[serde(rename = "networkInterfaceId")]
    pub network_interface_id: String,
    /// <p>Set to <code>true</code> if you want to preserve the data on the local disk. Otherwise, set to <code>false</code> to create an empty volume.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "preserveExistingData")]
    pub preserve_existing_data: bool,
    /// <p>The snapshot ID (e.g., "snap-1122aabb") of the snapshot to restore as the new stored volume. Specify this field if you want to create the iSCSI storage volume from a snapshot; otherwise, do not include this field. To list snapshots for your account use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-DescribeSnapshots.html">DescribeSnapshots</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p>
    #[serde(rename = "snapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// <p><p>A list of up to 50 tags that can be assigned to a stored volume. Each tag is a key-value pair.</p> <note> <p>Valid characters for key and value are letters, spaces, and numbers representable in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag&#39;s key is 128 characters, and the maximum length for a tag&#39;s value is 256.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The name of the iSCSI target used by an initiator to connect to a volume and used as a suffix for the target ARN. For example, specifying <code>TargetName</code> as <i>myvolume</i> results in the target ARN of <code>arn:aws:storagegateway:us-east-2:111122223333:gateway/sgw-12A3456B/target/iqn.1997-05.com.amazon:myvolume</code>. The target name must be unique across all volumes on a gateway.</p> <p>If you don't specify a value, Storage Gateway uses the value that was previously used for this volume as the new target name.</p>
    #[serde(rename = "targetName")]
    pub target_name: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateStorediSCSIVolumeOutput {
    /// <p>The Amazon Resource Name (ARN) of the volume target, which includes the iSCSI name that initiators can use to connect to the target.</p>
    #[serde(rename = "targetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the configured volume.</p>
    #[serde(rename = "volumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
    /// <p>The size of the volume in bytes.</p>
    #[serde(rename = "volumeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_bytes: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTapePoolInput {
    /// <p>The name of the new custom tape pool.</p>
    #[serde(rename = "poolName")]
    pub pool_name: String,
    /// <p>Tape retention lock time is set in days. Tape retention lock can be enabled for up to 100 years (36,500 days).</p>
    #[serde(rename = "retentionLockTimeInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_lock_time_in_days: Option<i64>,
    /// <p>Tape retention lock can be configured in two modes. When configured in governance mode, AWS accounts with specific IAM permissions are authorized to remove the tape retention lock from archived virtual tapes. When configured in compliance mode, the tape retention lock cannot be removed by any user, including the root AWS account.</p>
    #[serde(rename = "retentionLockType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_lock_type: Option<String>,
    /// <p>The storage class that is associated with the new custom pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class (S3 Glacier or S3 Glacier Deep Archive) that corresponds to the pool.</p>
    #[serde(rename = "storageClass")]
    pub storage_class: String,
    /// <p><p>A list of up to 50 tags that can be assigned to tape pool. Each tag is a key-value pair.</p> <note> <p>Valid characters for key and value are letters, spaces, and numbers representable in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag&#39;s key is 128 characters, and the maximum length for a tag&#39;s value is 256.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTapePoolOutput {
    /// <p>The unique Amazon Resource Name (ARN) that represents the custom tape pool. Use the <a>ListTapePools</a> operation to return a list of tape pools for your account and AWS Region.</p>
    #[serde(rename = "poolARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_arn: Option<String>,
}

/// <p>CreateTapeWithBarcodeInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTapeWithBarcodeInput {
    /// <p>The unique Amazon Resource Name (ARN) that represents the gateway to associate the virtual tape with. Use the <a>ListGateways</a> operation to return a list of gateways for your account and AWS Region.</p>
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own AWS KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "kMSEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encrypted: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The ID of the pool that you want to add your tape to for archiving. The tape in this pool is archived in the S3 storage class that is associated with the pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class (S3 Glacier or S3 Deep Archive) that corresponds to the pool.</p> <p>Valid Values: <code>GLACIER</code> | <code>DEEP_ARCHIVE</code> </p>
    #[serde(rename = "poolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    /// <p><p>A list of up to 50 tags that can be assigned to a virtual tape that has a barcode. Each tag is a key-value pair.</p> <note> <p>Valid characters for key and value are letters, spaces, and numbers representable in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag&#39;s key is 128 characters, and the maximum length for a tag&#39;s value is 256.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p><p>The barcode that you want to assign to the tape.</p> <note> <p>Barcodes cannot be reused. This includes barcodes used for tapes that have been deleted.</p> </note></p>
    #[serde(rename = "tapeBarcode")]
    pub tape_barcode: String,
    /// <p><p>The size, in bytes, of the virtual tape that you want to create.</p> <note> <p>The size must be aligned by gigabyte (1024<em>1024</em>1024 bytes).</p> </note></p>
    #[serde(rename = "tapeSizeInBytes")]
    pub tape_size_in_bytes: i64,
    /// <p>Set to <code>TRUE</code> if the tape you are creating is to be configured as a write-once-read-many (WORM) tape.</p>
    #[serde(rename = "worm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worm: Option<bool>,
}

/// <p>CreateTapeOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTapeWithBarcodeOutput {
    /// <p>A unique Amazon Resource Name (ARN) that represents the virtual tape that was created.</p>
    #[serde(rename = "tapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

/// <p>CreateTapesInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTapesInput {
    /// <p><p>A unique identifier that you use to retry a request. If you retry a request, use the same <code>ClientToken</code> you specified in the initial request.</p> <note> <p>Using the same <code>ClientToken</code> prevents creating the tape multiple times.</p> </note></p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The unique Amazon Resource Name (ARN) that represents the gateway to associate the virtual tapes with. Use the <a>ListGateways</a> operation to return a list of gateways for your account and AWS Region.</p>
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own AWS KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "kMSEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encrypted: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The number of virtual tapes that you want to create.</p>
    #[serde(rename = "numTapesToCreate")]
    pub num_tapes_to_create: i64,
    /// <p>The ID of the pool that you want to add your tape to for archiving. The tape in this pool is archived in the S3 storage class that is associated with the pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class (S3 Glacier or S3 Glacier Deep Archive) that corresponds to the pool.</p> <p>Valid Values: <code>GLACIER</code> | <code>DEEP_ARCHIVE</code> </p>
    #[serde(rename = "poolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    /// <p><p>A list of up to 50 tags that can be assigned to a virtual tape. Each tag is a key-value pair.</p> <note> <p>Valid characters for key and value are letters, spaces, and numbers representable in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag&#39;s key is 128 characters, and the maximum length for a tag&#39;s value is 256.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p><p>A prefix that you append to the barcode of the virtual tape you are creating. This prefix makes the barcode unique.</p> <note> <p>The prefix must be 1-4 characters in length and must be one of the uppercase letters from A to Z.</p> </note></p>
    #[serde(rename = "tapeBarcodePrefix")]
    pub tape_barcode_prefix: String,
    /// <p><p>The size, in bytes, of the virtual tapes that you want to create.</p> <note> <p>The size must be aligned by gigabyte (1024<em>1024</em>1024 bytes).</p> </note></p>
    #[serde(rename = "tapeSizeInBytes")]
    pub tape_size_in_bytes: i64,
    /// <p>Set to <code>TRUE</code> if the tape you are creating is to be configured as a write-once-read-many (WORM) tape.</p>
    #[serde(rename = "worm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worm: Option<bool>,
}

/// <p>CreateTapeOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTapesOutput {
    /// <p>A list of unique Amazon Resource Names (ARNs) that represents the virtual tapes that were created.</p>
    #[serde(rename = "tapeARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_ar_ns: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAutomaticTapeCreationPolicyInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteAutomaticTapeCreationPolicyOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p><p>A JSON object containing the following fields:</p> <ul> <li> <p> <a>DeleteBandwidthRateLimitInput$BandwidthType</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteBandwidthRateLimitInput {
    /// <p>One of the BandwidthType values that indicates the gateway bandwidth rate limit to delete.</p> <p>Valid Values: <code>UPLOAD</code> | <code>DOWNLOAD</code> | <code>ALL</code> </p>
    #[serde(rename = "bandwidthType")]
    pub bandwidth_type: String,
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway whose bandwidth rate information was deleted.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteBandwidthRateLimitOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>DeleteChapCredentialsInput$InitiatorName</a> </p> </li> <li> <p> <a>DeleteChapCredentialsInput$TargetARN</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteChapCredentialsInput {
    /// <p>The iSCSI initiator that connects to the target.</p>
    #[serde(rename = "initiatorName")]
    pub initiator_name: String,
    /// <p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <a>DescribeStorediSCSIVolumes</a> operation to return to retrieve the TargetARN for specified VolumeARN.</p>
    #[serde(rename = "targetARN")]
    pub target_arn: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteChapCredentialsOutput {
    /// <p>The iSCSI initiator that connects to the target.</p>
    #[serde(rename = "initiatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the target.</p>
    #[serde(rename = "targetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

/// <p>DeleteFileShareInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFileShareInput {
    /// <p>The Amazon Resource Name (ARN) of the file share to be deleted.</p>
    #[serde(rename = "fileShareARN")]
    pub file_share_arn: String,
    /// <p>If this value is set to <code>true</code>, the operation deletes a file share immediately and aborts all data uploads to AWS. Otherwise, the file share is not deleted until all data is uploaded to AWS. This process aborts the data upload process, and the file share enters the <code>FORCE_DELETING</code> status.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "forceDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete: Option<bool>,
}

/// <p>DeleteFileShareOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteFileShareOutput {
    /// <p>The Amazon Resource Name (ARN) of the deleted file share.</p>
    #[serde(rename = "fileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
}

/// <p>A JSON object containing the ID of the gateway to delete.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGatewayInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the ID of the deleted gateway.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteGatewayOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSnapshotScheduleInput {
    /// <p>The volume which snapshot schedule to delete.</p>
    #[serde(rename = "volumeARN")]
    pub volume_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteSnapshotScheduleOutput {
    /// <p>The volume which snapshot schedule was deleted.</p>
    #[serde(rename = "volumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

/// <p>DeleteTapeArchiveInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTapeArchiveInput {
    /// <p>Set to <code>TRUE</code> to delete an archived tape that belongs to a custom pool with tape retention lock. Only archived tapes with tape retention lock set to <code>governance</code> can be deleted. Archived tapes with tape retention lock set to <code>compliance</code> can't be deleted.</p>
    #[serde(rename = "bypassGovernanceRetention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_governance_retention: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the virtual tape to delete from the virtual tape shelf (VTS).</p>
    #[serde(rename = "tapeARN")]
    pub tape_arn: String,
}

/// <p>DeleteTapeArchiveOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTapeArchiveOutput {
    /// <p>The Amazon Resource Name (ARN) of the virtual tape that was deleted from the virtual tape shelf (VTS).</p>
    #[serde(rename = "tapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

/// <p>DeleteTapeInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTapeInput {
    /// <p>Set to <code>TRUE</code> to delete an archived tape that belongs to a custom pool with tape retention lock. Only archived tapes with tape retention lock set to <code>governance</code> can be deleted. Archived tapes with tape retention lock set to <code>compliance</code> can't be deleted.</p>
    #[serde(rename = "bypassGovernanceRetention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_governance_retention: Option<bool>,
    /// <p>The unique Amazon Resource Name (ARN) of the gateway that the virtual tape to delete is associated with. Use the <a>ListGateways</a> operation to return a list of gateways for your account and AWS Region.</p>
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the virtual tape to delete.</p>
    #[serde(rename = "tapeARN")]
    pub tape_arn: String,
}

/// <p>DeleteTapeOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTapeOutput {
    /// <p>The Amazon Resource Name (ARN) of the deleted virtual tape.</p>
    #[serde(rename = "tapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTapePoolInput {
    /// <p>The Amazon Resource Name (ARN) of the custom tape pool to delete.</p>
    #[serde(rename = "poolARN")]
    pub pool_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTapePoolOutput {
    /// <p>The Amazon Resource Name (ARN) of the custom tape pool being deleted.</p>
    #[serde(rename = "poolARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_arn: Option<String>,
}

/// <p>A JSON object containing the <a>DeleteVolumeInput$VolumeARN</a> to delete.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVolumeInput {
    /// <p>The Amazon Resource Name (ARN) of the volume. Use the <a>ListVolumes</a> operation to return a list of gateway volumes.</p>
    #[serde(rename = "volumeARN")]
    pub volume_arn: String,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the storage volume that was deleted.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVolumeOutput {
    /// <p>The Amazon Resource Name (ARN) of the storage volume that was deleted. It is the same ARN you provided in the request.</p>
    #[serde(rename = "volumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAvailabilityMonitorTestInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAvailabilityMonitorTestOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The time the high availability monitoring test was started. If a test hasn't been performed, the value of this field is null.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the high availability monitoring test. If a test hasn't been performed, the value of this field is null.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeBandwidthRateLimitInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBandwidthRateLimitOutput {
    /// <p>The average download bandwidth rate limit in bits per second. This field does not appear in the response if the download rate limit is not set.</p>
    #[serde(rename = "averageDownloadRateLimitInBitsPerSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_download_rate_limit_in_bits_per_sec: Option<i64>,
    /// <p>The average upload bandwidth rate limit in bits per second. This field does not appear in the response if the upload rate limit is not set.</p>
    #[serde(rename = "averageUploadRateLimitInBitsPerSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_upload_rate_limit_in_bits_per_sec: Option<i64>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeBandwidthRateLimitScheduleInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeBandwidthRateLimitScheduleOutput {
    /// <p> An array that contains the bandwidth rate limit intervals for a tape or volume gateway. </p>
    #[serde(rename = "bandwidthRateLimitIntervals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_rate_limit_intervals: Option<Vec<BandwidthRateLimitInterval>>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCacheInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCacheOutput {
    /// <p>The amount of cache in bytes allocated to a gateway.</p>
    #[serde(rename = "cacheAllocatedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_allocated_in_bytes: Option<i64>,
    /// <p>The file share's contribution to the overall percentage of the gateway's cache that has not been persisted to AWS. The sample is taken at the end of the reporting period.</p>
    #[serde(rename = "cacheDirtyPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_dirty_percentage: Option<f64>,
    /// <p>Percent of application read operations from the file shares that are served from cache. The sample is taken at the end of the reporting period.</p>
    #[serde(rename = "cacheHitPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_hit_percentage: Option<f64>,
    /// <p>Percent of application read operations from the file shares that are not served from cache. The sample is taken at the end of the reporting period.</p>
    #[serde(rename = "cacheMissPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_miss_percentage: Option<f64>,
    /// <p>Percent use of the gateway's cache storage. This metric applies only to the gateway-cached volume setup. The sample is taken at the end of the reporting period.</p>
    #[serde(rename = "cacheUsedPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_used_percentage: Option<f64>,
    /// <p>An array of strings that identify disks that are to be configured as working storage. Each string has a minimum length of 1 and maximum length of 300. You can get the disk IDs from the <a>ListLocalDisks</a> API.</p>
    #[serde(rename = "diskIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_ids: Option<Vec<String>>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCachediSCSIVolumesInput {
    /// <p>An array of strings where each string represents the Amazon Resource Name (ARN) of a cached volume. All of the specified cached volumes must be from the same gateway. Use <a>ListVolumes</a> to get volume ARNs for a gateway.</p>
    #[serde(rename = "volumeARNs")]
    pub volume_ar_ns: Vec<String>,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCachediSCSIVolumesOutput {
    /// <p>An array of objects where each object contains metadata about one cached volume.</p>
    #[serde(rename = "cachediSCSIVolumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cachedi_scsi_volumes: Option<Vec<CachediSCSIVolume>>,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the iSCSI volume target.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeChapCredentialsInput {
    /// <p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <a>DescribeStorediSCSIVolumes</a> operation to return to retrieve the TargetARN for specified VolumeARN.</p>
    #[serde(rename = "targetARN")]
    pub target_arn: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeChapCredentialsOutput {
    /// <p><p>An array of <a>ChapInfo</a> objects that represent CHAP credentials. Each object in the array contains CHAP credential information for one target-initiator pair. If no CHAP credentials are set, an empty array is returned. CHAP credential information is provided in a JSON object with the following fields:</p> <ul> <li> <p> <b>InitiatorName</b>: The iSCSI initiator that connects to the target.</p> </li> <li> <p> <b>SecretToAuthenticateInitiator</b>: The secret key that the initiator (for example, the Windows client) must provide to participate in mutual CHAP with the target.</p> </li> <li> <p> <b>SecretToAuthenticateTarget</b>: The secret key that the target must provide to participate in mutual CHAP with the initiator (e.g. Windows client).</p> </li> <li> <p> <b>TargetARN</b>: The Amazon Resource Name (ARN) of the storage volume.</p> </li> </ul></p>
    #[serde(rename = "chapCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chap_credentials: Option<Vec<ChapInfo>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeFileSystemAssociationsInput {
    /// <p>An array containing the Amazon Resource Name (ARN) of each file system association to be described.</p>
    #[serde(rename = "fileSystemAssociationARNList")]
    pub file_system_association_arn_list: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeFileSystemAssociationsOutput {
    /// <p>An array containing the <code>FileSystemAssociationInfo</code> data type of each file system association to be described. </p>
    #[serde(rename = "fileSystemAssociationInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_association_info_list: Option<Vec<FileSystemAssociationInfo>>,
}

/// <p>A JSON object containing the ID of the gateway.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGatewayInformationInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeGatewayInformationOutput {
    /// <p>The Amazon Resource Name (ARN) of the Amazon CloudWatch log group that is used to monitor events in the gateway.</p>
    #[serde(rename = "cloudWatchLogGroupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_arn: Option<String>,
    /// <p>Date after which this gateway will not receive software updates for new features and bug fixes.</p>
    #[serde(rename = "deprecationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<String>,
    /// <p>The ID of the Amazon EC2 instance that was used to launch the gateway.</p>
    #[serde(rename = "ec2InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_instance_id: Option<String>,
    /// <p>The AWS Region where the Amazon EC2 instance is located.</p>
    #[serde(rename = "ec2InstanceRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_instance_region: Option<String>,
    /// <p>The type of endpoint for your gateway.</p> <p>Valid Values: <code>STANDARD</code> | <code>FIPS</code> </p>
    #[serde(rename = "endpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The unique identifier assigned to your gateway during activation. This ID becomes part of the gateway Amazon Resource Name (ARN), which you use as input for other operations.</p>
    #[serde(rename = "gatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    /// <p>The name you configured for your gateway.</p>
    #[serde(rename = "gatewayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_name: Option<String>,
    /// <p>A <a>NetworkInterface</a> array that contains descriptions of the gateway network interfaces.</p>
    #[serde(rename = "gatewayNetworkInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_network_interfaces: Option<Vec<NetworkInterface>>,
    /// <p>A value that indicates the operating state of the gateway.</p>
    #[serde(rename = "gatewayState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_state: Option<String>,
    /// <p>A value that indicates the time zone configured for the gateway.</p>
    #[serde(rename = "gatewayTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_timezone: Option<String>,
    /// <p>The type of the gateway.</p>
    #[serde(rename = "gatewayType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_type: Option<String>,
    /// <p>The type of hypervisor environment used by the host.</p>
    #[serde(rename = "hostEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_environment: Option<String>,
    /// <p>The date on which the last software update was applied to the gateway. If the gateway has never been updated, this field does not return a value in the response.</p>
    #[serde(rename = "lastSoftwareUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_software_update: Option<String>,
    /// <p>The date on which an update to the gateway is available. This date is in the time zone of the gateway. If the gateway is not available for an update this field is not returned in the response.</p>
    #[serde(rename = "nextUpdateAvailabilityDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_update_availability_date: Option<String>,
    /// <p>Date after which this gateway will not receive software updates for new features.</p>
    #[serde(rename = "softwareUpdatesEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_updates_end_date: Option<String>,
    /// <p>A list of up to 50 tags assigned to the gateway, sorted alphabetically by key name. Each tag is a key-value pair. For a gateway with more than 10 tags assigned, you can view all tags using the <code>ListTagsForResource</code> API operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The configuration settings for the virtual private cloud (VPC) endpoint for your gateway.</p>
    #[serde(rename = "vPCEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint: Option<String>,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeMaintenanceStartTimeInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

/// <p><p>A JSON object containing the following fields:</p> <ul> <li> <p> <a>DescribeMaintenanceStartTimeOutput$DayOfMonth</a> </p> </li> <li> <p> <a>DescribeMaintenanceStartTimeOutput$DayOfWeek</a> </p> </li> <li> <p> <a>DescribeMaintenanceStartTimeOutput$HourOfDay</a> </p> </li> <li> <p> <a>DescribeMaintenanceStartTimeOutput$MinuteOfHour</a> </p> </li> <li> <p> <a>DescribeMaintenanceStartTimeOutput$Timezone</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMaintenanceStartTimeOutput {
    /// <p>The day of the month component of the maintenance start time represented as an ordinal number from 1 to 28, where 1 represents the first day of the month and 28 represents the last day of the month.</p>
    #[serde(rename = "dayOfMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<i64>,
    /// <p>An ordinal number between 0 and 6 that represents the day of the week, where 0 represents Sunday and 6 represents Saturday. The day of week is in the time zone of the gateway.</p>
    #[serde(rename = "dayOfWeek")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i64>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The hour component of the maintenance start time represented as <i>hh</i>, where <i>hh</i> is the hour (0 to 23). The hour of the day is in the time zone of the gateway.</p>
    #[serde(rename = "hourOfDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour_of_day: Option<i64>,
    /// <p>The minute component of the maintenance start time represented as <i>mm</i>, where <i>mm</i> is the minute (0 to 59). The minute of the hour is in the time zone of the gateway.</p>
    #[serde(rename = "minuteOfHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute_of_hour: Option<i64>,
    /// <p>A value that indicates the time zone that is set for the gateway. The start time and day of week specified should be in the time zone of the gateway.</p>
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// <p>DescribeNFSFileSharesInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeNFSFileSharesInput {
    /// <p>An array containing the Amazon Resource Name (ARN) of each file share to be described.</p>
    #[serde(rename = "fileShareARNList")]
    pub file_share_arn_list: Vec<String>,
}

/// <p>DescribeNFSFileSharesOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeNFSFileSharesOutput {
    /// <p>An array containing a description for each requested file share.</p>
    #[serde(rename = "nFSFileShareInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_file_share_info_list: Option<Vec<NFSFileShareInfo>>,
}

/// <p>DescribeSMBFileSharesInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSMBFileSharesInput {
    /// <p>An array containing the Amazon Resource Name (ARN) of each file share to be described.</p>
    #[serde(rename = "fileShareARNList")]
    pub file_share_arn_list: Vec<String>,
}

/// <p>DescribeSMBFileSharesOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSMBFileSharesOutput {
    /// <p>An array containing a description for each requested file share.</p>
    #[serde(rename = "sMBFileShareInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_file_share_info_list: Option<Vec<SMBFileShareInfo>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSMBSettingsInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSMBSettingsOutput {
    /// <p><p>Indicates the status of a gateway that is a member of the Active Directory domain.</p> <ul> <li> <p> <code>ACCESS<em>DENIED</code>: Indicates that the <code>JoinDomain</code> operation failed due to an authentication error.</p> </li> <li> <p> <code>DETACHED</code>: Indicates that gateway is not joined to a domain.</p> </li> <li> <p> <code>JOINED</code>: Indicates that the gateway has successfully joined a domain.</p> </li> <li> <p> <code>JOINING</code>: Indicates that a <code>JoinDomain</code> operation is in progress.</p> </li> <li> <p> <code>NETWORK</em>ERROR</code>: Indicates that <code>JoinDomain</code> operation failed due to a network or connectivity error.</p> </li> <li> <p> <code>TIMEOUT</code>: Indicates that the <code>JoinDomain</code> operation failed because the operation didn&#39;t complete within the allotted time.</p> </li> <li> <p> <code>UNKNOWN_ERROR</code>: Indicates that the <code>JoinDomain</code> operation failed due to another type of error.</p> </li> </ul></p>
    #[serde(rename = "activeDirectoryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_status: Option<String>,
    /// <p>The name of the domain that the gateway is joined to.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The shares on this gateway appear when listing shares.</p>
    #[serde(rename = "fileSharesVisible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_shares_visible: Option<bool>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>This value is <code>true</code> if a password for the guest user <code>smbguest</code> is set, otherwise <code>false</code>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "sMBGuestPasswordSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_guest_password_set: Option<bool>,
    /// <p><p>The type of security strategy that was specified for file gateway.</p> <ul> <li> <p> <code>ClientSpecified</code>: If you use this option, requests are established based on what is negotiated by the client. This option is recommended when you want to maximize compatibility across different clients in your environment.</p> </li> <li> <p> <code>MandatorySigning</code>: If you use this option, file gateway only allows connections from SMBv2 or SMBv3 clients that have signing enabled. This option works with SMB clients on Microsoft Windows Vista, Windows Server 2008 or newer.</p> </li> <li> <p> <code>MandatoryEncryption</code>: If you use this option, file gateway only allows connections from SMBv3 clients that have encryption enabled. This option is highly recommended for environments that handle sensitive data. This option works with SMB clients on Microsoft Windows 8, Windows Server 2012 or newer.</p> </li> </ul></p>
    #[serde(rename = "sMBSecurityStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_security_strategy: Option<String>,
}

/// <p>A JSON object containing the <a>DescribeSnapshotScheduleInput$VolumeARN</a> of the volume.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSnapshotScheduleInput {
    /// <p>The Amazon Resource Name (ARN) of the volume. Use the <a>ListVolumes</a> operation to return a list of gateway volumes.</p>
    #[serde(rename = "volumeARN")]
    pub volume_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSnapshotScheduleOutput {
    /// <p>The snapshot description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The number of hours between snapshots.</p>
    #[serde(rename = "recurrenceInHours")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_in_hours: Option<i64>,
    /// <p>The hour of the day at which the snapshot schedule begins represented as <i>hh</i>, where <i>hh</i> is the hour (0 to 23). The hour of the day is in the time zone of the gateway.</p>
    #[serde(rename = "startAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// <p>A list of up to 50 tags assigned to the snapshot schedule, sorted alphabetically by key name. Each tag is a key-value pair. For a gateway with more than 10 tags assigned, you can view all tags using the <code>ListTagsForResource</code> API operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A value that indicates the time zone of the gateway.</p>
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the volume that was specified in the request.</p>
    #[serde(rename = "volumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

/// <p>A JSON object containing a list of <a>DescribeStorediSCSIVolumesInput$VolumeARNs</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeStorediSCSIVolumesInput {
    /// <p>An array of strings where each string represents the Amazon Resource Name (ARN) of a stored volume. All of the specified stored volumes must be from the same gateway. Use <a>ListVolumes</a> to get volume ARNs for a gateway.</p>
    #[serde(rename = "volumeARNs")]
    pub volume_ar_ns: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeStorediSCSIVolumesOutput {
    /// <p><p>Describes a single unit of output from <a>DescribeStorediSCSIVolumes</a>. The following fields are returned:</p> <ul> <li> <p> <code>ChapEnabled</code>: Indicates whether mutual CHAP is enabled for the iSCSI target.</p> </li> <li> <p> <code>LunNumber</code>: The logical disk number.</p> </li> <li> <p> <code>NetworkInterfaceId</code>: The network interface ID of the stored volume that initiator use to map the stored volume as an iSCSI target.</p> </li> <li> <p> <code>NetworkInterfacePort</code>: The port used to communicate with iSCSI targets.</p> </li> <li> <p> <code>PreservedExistingData</code>: Indicates when the stored volume was created, existing data on the underlying local disk was preserved.</p> </li> <li> <p> <code>SourceSnapshotId</code>: If the stored volume was created from a snapshot, this field contains the snapshot ID used, e.g. <code>snap-1122aabb</code>. Otherwise, this field is not included.</p> </li> <li> <p> <code>StorediSCSIVolumes</code>: An array of StorediSCSIVolume objects where each object contains metadata about one stored volume.</p> </li> <li> <p> <code>TargetARN</code>: The Amazon Resource Name (ARN) of the volume target.</p> </li> <li> <p> <code>VolumeARN</code>: The Amazon Resource Name (ARN) of the stored volume.</p> </li> <li> <p> <code>VolumeDiskId</code>: The disk ID of the local disk that was specified in the <a>CreateStorediSCSIVolume</a> operation.</p> </li> <li> <p> <code>VolumeId</code>: The unique identifier of the storage volume, e.g. <code>vol-1122AABB</code>.</p> </li> <li> <p> <code>VolumeiSCSIAttributes</code>: An <a>VolumeiSCSIAttributes</a> object that represents a collection of iSCSI attributes for one stored volume.</p> </li> <li> <p> <code>VolumeProgress</code>: Represents the percentage complete if the volume is restoring or bootstrapping that represents the percent of data transferred. This field does not appear in the response if the stored volume is not restoring or bootstrapping.</p> </li> <li> <p> <code>VolumeSizeInBytes</code>: The size of the volume in bytes.</p> </li> <li> <p> <code>VolumeStatus</code>: One of the <code>VolumeStatus</code> values that indicates the state of the volume.</p> </li> <li> <p> <code>VolumeType</code>: One of the enumeration values describing the type of the volume. Currently, only <code>STORED</code> volumes are supported.</p> </li> </ul></p>
    #[serde(rename = "storediSCSIVolumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storedi_scsi_volumes: Option<Vec<StorediSCSIVolume>>,
}

/// <p>DescribeTapeArchivesInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTapeArchivesInput {
    /// <p>Specifies that the number of virtual tapes described be limited to the specified number.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin describing virtual tapes.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Specifies one or more unique Amazon Resource Names (ARNs) that represent the virtual tapes you want to describe.</p>
    #[serde(rename = "tapeARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_ar_ns: Option<Vec<String>>,
}

/// <p>DescribeTapeArchivesOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTapeArchivesOutput {
    /// <p>An opaque string that indicates the position at which the virtual tapes that were fetched for description ended. Use this marker in your next request to fetch the next set of virtual tapes in the virtual tape shelf (VTS). If there are no more virtual tapes to describe, this field does not appear in the response.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>An array of virtual tape objects in the virtual tape shelf (VTS). The description includes of the Amazon Resource Name (ARN) of the virtual tapes. The information returned includes the Amazon Resource Names (ARNs) of the tapes, size of the tapes, status of the tapes, progress of the description, and tape barcode.</p>
    #[serde(rename = "tapeArchives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_archives: Option<Vec<TapeArchive>>,
}

/// <p>DescribeTapeRecoveryPointsInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTapeRecoveryPointsInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>Specifies that the number of virtual tape recovery points that are described be limited to the specified number.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin describing the virtual tape recovery points.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>DescribeTapeRecoveryPointsOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTapeRecoveryPointsOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>An opaque string that indicates the position at which the virtual tape recovery points that were listed for description ended.</p> <p>Use this marker in your next request to list the next set of virtual tape recovery points in the list. If there are no more recovery points to describe, this field does not appear in the response.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>An array of TapeRecoveryPointInfos that are available for the specified gateway.</p>
    #[serde(rename = "tapeRecoveryPointInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_recovery_point_infos: Option<Vec<TapeRecoveryPointInfo>>,
}

/// <p>DescribeTapesInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTapesInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p><p>Specifies that the number of virtual tapes described be limited to the specified number.</p> <note> <p>Amazon Web Services may impose its own limit, if this field is not set.</p> </note></p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A marker value, obtained in a previous call to <code>DescribeTapes</code>. This marker indicates which page of results to retrieve.</p> <p>If not specified, the first page of results is retrieved.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Specifies one or more unique Amazon Resource Names (ARNs) that represent the virtual tapes you want to describe. If this parameter is not specified, Tape gateway returns a description of all virtual tapes associated with the specified gateway.</p>
    #[serde(rename = "tapeARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_ar_ns: Option<Vec<String>>,
}

/// <p>DescribeTapesOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTapesOutput {
    /// <p>An opaque string that can be used as part of a subsequent <code>DescribeTapes</code> call to retrieve the next page of results.</p> <p>If a response does not contain a marker, then there are no more results to be retrieved.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>An array of virtual tape descriptions.</p>
    #[serde(rename = "tapes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tapes: Option<Vec<Tape>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUploadBufferInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeUploadBufferOutput {
    /// <p>An array of the gateway's local disk IDs that are configured as working storage. Each local disk ID is specified as a string (minimum length of 1 and maximum length of 300). If no local disks are configured as working storage, then the DiskIds array is empty.</p>
    #[serde(rename = "diskIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_ids: Option<Vec<String>>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The total number of bytes allocated in the gateway's as upload buffer.</p>
    #[serde(rename = "uploadBufferAllocatedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_buffer_allocated_in_bytes: Option<i64>,
    /// <p>The total number of bytes being used in the gateway's upload buffer.</p>
    #[serde(rename = "uploadBufferUsedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_buffer_used_in_bytes: Option<i64>,
}

/// <p>DescribeVTLDevicesInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeVTLDevicesInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>Specifies that the number of VTL devices described be limited to the specified number.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin describing the VTL devices.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p><p>An array of strings, where each string represents the Amazon Resource Name (ARN) of a VTL device.</p> <note> <p>All of the specified VTL devices must be from the same gateway. If no VTL devices are specified, the result will contain all devices on the specified gateway.</p> </note></p>
    #[serde(rename = "vTLDeviceARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_device_ar_ns: Option<Vec<String>>,
}

/// <p>DescribeVTLDevicesOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeVTLDevicesOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>An opaque string that indicates the position at which the VTL devices that were fetched for description ended. Use the marker in your next request to fetch the next set of VTL devices in the list. If there are no more VTL devices to describe, this field does not appear in the response.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>An array of VTL device objects composed of the Amazon Resource Name (ARN) of the VTL devices.</p>
    #[serde(rename = "vTLDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_devices: Option<Vec<VTLDevice>>,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeWorkingStorageInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeWorkingStorageOutput {
    /// <p>An array of the gateway's local disk IDs that are configured as working storage. Each local disk ID is specified as a string (minimum length of 1 and maximum length of 300). If no local disks are configured as working storage, then the DiskIds array is empty.</p>
    #[serde(rename = "diskIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_ids: Option<Vec<String>>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The total working storage in bytes allocated for the gateway. If no working storage is configured for the gateway, this field returns 0.</p>
    #[serde(rename = "workingStorageAllocatedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_storage_allocated_in_bytes: Option<i64>,
    /// <p>The total working storage in bytes in use by the gateway. If no working storage is configured for the gateway, this field returns 0.</p>
    #[serde(rename = "workingStorageUsedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_storage_used_in_bytes: Option<i64>,
}

/// <p>AttachVolumeInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DetachVolumeInput {
    /// <p>Set to <code>true</code> to forcibly remove the iSCSI connection of the target volume and detach the volume. The default is <code>false</code>. If this value is set to <code>false</code>, you must manually disconnect the iSCSI connection from the target volume.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "forceDetach")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_detach: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the volume to detach from the gateway.</p>
    #[serde(rename = "volumeARN")]
    pub volume_arn: String,
}

/// <p>AttachVolumeOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DetachVolumeOutput {
    /// <p>The Amazon Resource Name (ARN) of the volume that was detached.</p>
    #[serde(rename = "volumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

/// <p>Lists iSCSI information about a VTL device.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeviceiSCSIAttributes {
    /// <p>Indicates whether mutual CHAP is enabled for the iSCSI target.</p>
    #[serde(rename = "chapEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chap_enabled: Option<bool>,
    /// <p>The network interface identifier of the VTL device.</p>
    #[serde(rename = "networkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>The port used to communicate with iSCSI VTL device targets.</p>
    #[serde(rename = "networkInterfacePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_port: Option<i64>,
    /// <p>Specifies the unique Amazon Resource Name (ARN) that encodes the iSCSI qualified name(iqn) of a tape drive or media changer target.</p>
    #[serde(rename = "targetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

/// <p>DisableGatewayInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisableGatewayInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

/// <p>DisableGatewayOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisableGatewayOutput {
    /// <p>The unique Amazon Resource Name (ARN) of the disabled gateway.</p>
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateFileSystemInput {
    /// <p>The Amazon Resource Name (ARN) of the file system association to be deleted.</p>
    #[serde(rename = "fileSystemAssociationARN")]
    pub file_system_association_arn: String,
    /// <p>If this value is set to true, the operation disassociates an Amazon FSx file system immediately. It ends all data uploads to the file system, and the file system association enters the <code>FORCE_DELETING</code> status. If this value is set to false, the Amazon FSx file system does not disassociate until all data is uploaded.</p>
    #[serde(rename = "forceDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateFileSystemOutput {
    /// <p>The Amazon Resource Name (ARN) of the deleted file system association.</p>
    #[serde(rename = "fileSystemAssociationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_association_arn: Option<String>,
}

/// <p>Represents a gateway's local disk.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Disk {
    /// <p>The iSCSI qualified name (IQN) that is defined for a disk. This field is not included in the response if the local disk is not defined as an iSCSI target. The format of this field is <i>targetIqn::LUNNumber::region-volumeId</i>.</p>
    #[serde(rename = "diskAllocationResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_allocation_resource: Option<String>,
    #[serde(rename = "diskAllocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_allocation_type: Option<String>,
    #[serde(rename = "diskAttributeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_attribute_list: Option<Vec<String>>,
    /// <p>The unique device ID or other distinguishing data that identifies a local disk.</p>
    #[serde(rename = "diskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    /// <p>The device node of a local disk as assigned by the virtualization environment.</p>
    #[serde(rename = "diskNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_node: Option<String>,
    /// <p>The path of a local disk in the gateway virtual machine (VM).</p>
    #[serde(rename = "diskPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_path: Option<String>,
    /// <p>The local disk size in bytes.</p>
    #[serde(rename = "diskSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_in_bytes: Option<i64>,
    /// <p>A value that represents the status of a local disk.</p>
    #[serde(rename = "diskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_status: Option<String>,
}

/// <p>Describes a file share.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FileShareInfo {
    #[serde(rename = "fileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
    #[serde(rename = "fileShareId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_id: Option<String>,
    #[serde(rename = "fileShareStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_status: Option<String>,
    #[serde(rename = "fileShareType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_type: Option<String>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>Describes the object returned by <code>DescribeFileSystemAssociations</code> that describes a created file system association.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FileSystemAssociationInfo {
    /// <p>The Amazon Resource Name (ARN) of the storage used for the audit logs.</p>
    #[serde(rename = "auditDestinationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_destination_arn: Option<String>,
    #[serde(rename = "cacheAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_attributes: Option<CacheAttributes>,
    /// <p>The Amazon Resource Name (ARN) of the file system association.</p>
    #[serde(rename = "fileSystemAssociationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_association_arn: Option<String>,
    /// <p>The status of the file system association. Valid Values: <code>AVAILABLE</code> | <code>CREATING</code> | <code>DELETING</code> | <code>FORCE_DELETING</code> | <code>MISCONFIGURED</code> | <code>UPDATING</code> | <code>UNAVAILABLE</code> </p>
    #[serde(rename = "fileSystemAssociationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_association_status: Option<String>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The ARN of the backend Amazon FSx file system used for storing file data. For information, see <a href="https://docs.aws.amazon.com/fsx/latest/APIReference/API_FileSystem.html">FileSystem</a> in the <i>Amazon FSx API Reference</i>.</p>
    #[serde(rename = "locationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    /// <p>A list of up to 50 tags assigned to the SMB file share, sorted alphabetically by key name. Each tag is a key-value pair.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Gets the summary returned by <code>ListFileSystemAssociation</code>, which is a summary of a created file system association.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FileSystemAssociationSummary {
    /// <p>The Amazon Resource Name (ARN) of the file system association.</p>
    #[serde(rename = "fileSystemAssociationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_association_arn: Option<String>,
    /// <p>The ID of the file system association.</p>
    #[serde(rename = "fileSystemAssociationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_association_id: Option<String>,
    /// <p>The status of the file share. Valid Values: <code>AVAILABLE</code> | <code>CREATING</code> | <code>DELETING</code> | <code>FORCE_DELETING</code> | <code>MISCONFIGURED</code> | <code>UPDATING</code> | <code>UNAVAILABLE</code> </p>
    #[serde(rename = "fileSystemAssociationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_association_status: Option<String>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>Describes a gateway object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GatewayInfo {
    /// <p>The ID of the Amazon EC2 instance that was used to launch the gateway.</p>
    #[serde(rename = "ec2InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_instance_id: Option<String>,
    /// <p>The AWS Region where the Amazon EC2 instance is located.</p>
    #[serde(rename = "ec2InstanceRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_instance_region: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <a>ListGateways</a> operation to return a list of gateways for your account and AWS Region.</p>
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The unique identifier assigned to your gateway during activation. This ID becomes part of the gateway Amazon Resource Name (ARN), which you use as input for other operations.</p>
    #[serde(rename = "gatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    /// <p>The name of the gateway.</p>
    #[serde(rename = "gatewayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_name: Option<String>,
    /// <p>The state of the gateway.</p> <p>Valid Values: <code>DISABLED</code> | <code>ACTIVE</code> </p>
    #[serde(rename = "gatewayOperationalState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_operational_state: Option<String>,
    /// <p>The type of the gateway.</p>
    #[serde(rename = "gatewayType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_type: Option<String>,
}

/// <p>JoinDomainInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct JoinDomainInput {
    /// <p>List of IPv4 addresses, NetBIOS names, or host names of your domain server. If you need to specify the port number include it after the colon (“:”). For example, <code>mydc.mydomain.com:389</code>.</p>
    #[serde(rename = "domainControllers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_controllers: Option<Vec<String>>,
    /// <p>The name of the domain that you want the gateway to join.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and AWS Region.</p>
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>The organizational unit (OU) is a container in an Active Directory that can hold users, groups, computers, and other OUs and this parameter specifies the OU that the gateway will join within the AD domain.</p>
    #[serde(rename = "organizationalUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<String>,
    /// <p>Sets the password of the user who has permission to add the gateway to the Active Directory domain.</p>
    #[serde(rename = "password")]
    pub password: String,
    /// <p>Specifies the time in seconds, in which the <code>JoinDomain</code> operation must complete. The default is 20 seconds.</p>
    #[serde(rename = "timeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i64>,
    /// <p>Sets the user name of user who has permission to add the gateway to the Active Directory domain. The domain user account should be enabled to join computers to the domain. For example, you can use the domain administrator account or an account with delegated permissions to join computers to the domain.</p>
    #[serde(rename = "userName")]
    pub user_name: String,
}

/// <p>JoinDomainOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JoinDomainOutput {
    /// <p><p>Indicates the status of the gateway as a member of the Active Directory domain.</p> <ul> <li> <p> <code>ACCESS<em>DENIED</code>: Indicates that the <code>JoinDomain</code> operation failed due to an authentication error.</p> </li> <li> <p> <code>DETACHED</code>: Indicates that gateway is not joined to a domain.</p> </li> <li> <p> <code>JOINED</code>: Indicates that the gateway has successfully joined a domain.</p> </li> <li> <p> <code>JOINING</code>: Indicates that a <code>JoinDomain</code> operation is in progress.</p> </li> <li> <p> <code>NETWORK</em>ERROR</code>: Indicates that <code>JoinDomain</code> operation failed due to a network or connectivity error.</p> </li> <li> <p> <code>TIMEOUT</code>: Indicates that the <code>JoinDomain</code> operation failed because the operation didn&#39;t complete within the allotted time.</p> </li> <li> <p> <code>UNKNOWN_ERROR</code>: Indicates that the <code>JoinDomain</code> operation failed due to another type of error.</p> </li> </ul></p>
    #[serde(rename = "activeDirectoryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_status: Option<String>,
    /// <p>The unique Amazon Resource Name (ARN) of the gateway that joined the domain.</p>
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAutomaticTapeCreationPoliciesInput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAutomaticTapeCreationPoliciesOutput {
    /// <p>Gets a listing of information about the gateway's automatic tape creation policies, including the automatic tape creation rules and the gateway that is using the policies.</p>
    #[serde(rename = "automaticTapeCreationPolicyInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tape_creation_policy_infos: Option<Vec<AutomaticTapeCreationPolicyInfo>>,
}

/// <p>ListFileShareInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFileSharesInput {
    /// <p>The Amazon Resource Name (ARN) of the gateway whose file shares you want to list. If this field is not present, all file shares under your account are listed.</p>
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The maximum number of file shares to return in the response. The value must be an integer with a value greater than zero. Optional.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Opaque pagination token returned from a previous ListFileShares operation. If present, <code>Marker</code> specifies where to continue the list from after a previous call to ListFileShares. Optional.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>ListFileShareOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFileSharesOutput {
    /// <p>An array of information about the file gateway's file shares.</p>
    #[serde(rename = "fileShareInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_info_list: Option<Vec<FileShareInfo>>,
    /// <p>If the request includes <code>Marker</code>, the response returns that value in this field.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>If a value is present, there are more file shares to return. In a subsequent request, use <code>NextMarker</code> as the value for <code>Marker</code> to retrieve the next set of file shares.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFileSystemAssociationsInput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The maximum number of file system associations to return in the response. If present, <code>Limit</code> must be an integer with a value greater than zero. Optional.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Opaque pagination token returned from a previous <code>ListFileSystemAssociations</code> operation. If present, <code>Marker</code> specifies where to continue the list from after a previous call to <code>ListFileSystemAssociations</code>. Optional.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFileSystemAssociationsOutput {
    /// <p>An array of information about the Amazon FSx gateway's file system associations.</p>
    #[serde(rename = "fileSystemAssociationSummaryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_association_summary_list: Option<Vec<FileSystemAssociationSummary>>,
    /// <p>If the request includes <code>Marker</code>, the response returns that value in this field.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>If a value is present, there are more file system associations to return. In a subsequent request, use <code>NextMarker</code> as the value for <code>Marker</code> to retrieve the next set of file system associations.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

/// <p><p>A JSON object containing zero or more of the following fields:</p> <ul> <li> <p> <a>ListGatewaysInput$Limit</a> </p> </li> <li> <p> <a>ListGatewaysInput$Marker</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListGatewaysInput {
    /// <p>Specifies that the list of gateways returned be limited to the specified number of items.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin the returned list of gateways.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGatewaysOutput {
    /// <p>An array of <a>GatewayInfo</a> objects.</p>
    #[serde(rename = "gateways")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<GatewayInfo>>,
    /// <p>Use the marker in your next request to fetch the next set of gateways in the list. If there are no more gateways to list, this field does not appear in the response.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLocalDisksInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLocalDisksOutput {
    /// <p><p>A JSON object containing the following fields:</p> <ul> <li> <p> <a>ListLocalDisksOutput$Disks</a> </p> </li> </ul></p>
    #[serde(rename = "disks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<Disk>>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>ListTagsForResourceInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceInput {
    /// <p>Specifies that the list of tags returned be limited to the specified number of items.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>An opaque string that indicates the position at which to begin returning the list of tags.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource for which you want to list tags.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

/// <p>ListTagsForResourceOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceOutput {
    /// <p>An opaque string that indicates the position at which to stop returning the list of tags.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource for which you want to list tags.</p>
    #[serde(rename = "resourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>An array that contains the tags for the specified resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTapePoolsInput {
    /// <p>An optional number limit for the tape pools in the list returned by this call.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A string that indicates the position at which to begin the returned list of tape pools.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of each of the custom tape pools you want to list. If you don't specify a custom tape pool ARN, the response lists all custom tape pools. </p>
    #[serde(rename = "poolARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_ar_ns: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTapePoolsOutput {
    /// <p>A string that indicates the position at which to begin the returned list of tape pools. Use the marker in your next request to continue pagination of tape pools. If there are no more tape pools to list, this element does not appear in the response body. </p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>An array of <code>PoolInfo</code> objects, where each object describes a single custom tape pool. If there are no custom tape pools, the <code>PoolInfos</code> is an empty array. </p>
    #[serde(rename = "poolInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_infos: Option<Vec<PoolInfo>>,
}

/// <p><p>A JSON object that contains one or more of the following fields:</p> <ul> <li> <p> <a>ListTapesInput$Limit</a> </p> </li> <li> <p> <a>ListTapesInput$Marker</a> </p> </li> <li> <p> <a>ListTapesInput$TapeARNs</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTapesInput {
    /// <p>An optional number limit for the tapes in the list returned by this call.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A string that indicates the position at which to begin the returned list of tapes.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "tapeARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_ar_ns: Option<Vec<String>>,
}

/// <p><p>A JSON object containing the following fields:</p> <ul> <li> <p> <a>ListTapesOutput$Marker</a> </p> </li> <li> <p> <a>ListTapesOutput$VolumeInfos</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTapesOutput {
    /// <p>A string that indicates the position at which to begin returning the next list of tapes. Use the marker in your next request to continue pagination of tapes. If there are no more tapes to list, this element does not appear in the response body.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "tapeInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_infos: Option<Vec<TapeInfo>>,
}

/// <p>ListVolumeInitiatorsInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVolumeInitiatorsInput {
    /// <p>The Amazon Resource Name (ARN) of the volume. Use the <a>ListVolumes</a> operation to return a list of gateway volumes for the gateway.</p>
    #[serde(rename = "volumeARN")]
    pub volume_arn: String,
}

/// <p>ListVolumeInitiatorsOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVolumeInitiatorsOutput {
    /// <p>The host names and port numbers of all iSCSI initiators that are connected to the gateway.</p>
    #[serde(rename = "initiators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiators: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVolumeRecoveryPointsInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVolumeRecoveryPointsOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>An array of <a>VolumeRecoveryPointInfo</a> objects.</p>
    #[serde(rename = "volumeRecoveryPointInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_recovery_point_infos: Option<Vec<VolumeRecoveryPointInfo>>,
}

/// <p><p>A JSON object that contains one or more of the following fields:</p> <ul> <li> <p> <a>ListVolumesInput$Limit</a> </p> </li> <li> <p> <a>ListVolumesInput$Marker</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVolumesInput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>Specifies that the list of volumes returned be limited to the specified number of items.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A string that indicates the position at which to begin the returned list of volumes. Obtain the marker from the response of a previous List iSCSI Volumes request.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p><p>A JSON object containing the following fields:</p> <ul> <li> <p> <a>ListVolumesOutput$Marker</a> </p> </li> <li> <p> <a>ListVolumesOutput$VolumeInfos</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVolumesOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>Use the marker in your next request to continue pagination of iSCSI volumes. If there are no more volumes to list, this field does not appear in the response body.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>An array of <a>VolumeInfo</a> objects, where each object describes an iSCSI volume. If no volumes are defined for the gateway, then <code>VolumeInfos</code> is an empty array "[]".</p>
    #[serde(rename = "volumeInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_infos: Option<Vec<VolumeInfo>>,
}

/// <p>Describes Network File System (NFS) file share default values. Files and folders stored as Amazon S3 objects in S3 buckets don't, by default, have Unix file permissions assigned to them. Upon discovery in an S3 bucket by Storage Gateway, the S3 objects that represent files and folders are assigned these default Unix permissions. This operation is only supported for file gateways.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NFSFileShareDefaults {
    /// <p>The Unix directory mode in the form "nnnn". For example, <code>0666</code> represents the default access mode for all directories inside the file share. The default value is <code>0777</code>.</p>
    #[serde(rename = "directoryMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_mode: Option<String>,
    /// <p>The Unix file mode in the form "nnnn". For example, <code>0666</code> represents the default file mode inside the file share. The default value is <code>0666</code>.</p>
    #[serde(rename = "fileMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    /// <p>The default group ID for the file share (unless the files have another group ID specified). The default value is <code>nfsnobody</code>.</p>
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    /// <p>The default owner ID for files in the file share (unless the files have another owner ID specified). The default value is <code>nfsnobody</code>.</p>
    #[serde(rename = "ownerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i64>,
}

/// <p>The Unix file permissions and ownership information assigned, by default, to native S3 objects when file gateway discovers them in S3 buckets. This operation is only supported in file gateways.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NFSFileShareInfo {
    /// <p>Refresh cache information for the file share.</p>
    #[serde(rename = "cacheAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_attributes: Option<CacheAttributes>,
    #[serde(rename = "clientList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_list: Option<Vec<String>>,
    /// <p>The default storage class for objects put into an Amazon S3 bucket by the file gateway. The default value is <code>S3_INTELLIGENT_TIERING</code>. Optional.</p> <p>Valid Values: <code>S3_STANDARD</code> | <code>S3_INTELLIGENT_TIERING</code> | <code>S3_STANDARD_IA</code> | <code>S3_ONEZONE_IA</code> </p>
    #[serde(rename = "defaultStorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_storage_class: Option<String>,
    #[serde(rename = "fileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
    #[serde(rename = "fileShareId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_id: Option<String>,
    /// <p><p>The name of the file share. Optional.</p> <note> <p> <code>FileShareName</code> must be set if an S3 prefix name is set in <code>LocationARN</code>.</p> </note></p>
    #[serde(rename = "fileShareName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_name: Option<String>,
    #[serde(rename = "fileShareStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_status: Option<String>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>A value that enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to <code>true</code> to enable MIME type guessing, otherwise set to <code>false</code>. The default value is <code>true</code>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "guessMIMETypeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guess_mime_type_enabled: Option<bool>,
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own AWS KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "kMSEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encrypted: Option<bool>,
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    #[serde(rename = "locationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    #[serde(rename = "nFSFileShareDefaults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_file_share_defaults: Option<NFSFileShareDefaults>,
    /// <p>The notification policy of the file share. <code>SettlingTimeInSeconds</code> controls the number of seconds to wait after the last point in time a client wrote to a file before generating an <code>ObjectUploaded</code> notification. Because clients can make many small writes to files, it's best to set this parameter for as long as possible to avoid generating multiple notifications for the same file in a small time period.</p> <note> <p> <code>SettlingTimeInSeconds</code> has no effect on the timing of the object uploading to Amazon S3, only the timing of the notification.</p> </note> <p>The following example sets <code>NotificationPolicy</code> on with <code>SettlingTimeInSeconds</code> set to 60.</p> <p> <code>{\"Upload\": {\"SettlingTimeInSeconds\": 60}}</code> </p> <p>The following example sets <code>NotificationPolicy</code> off.</p> <p> <code>{}</code> </p>
    #[serde(rename = "notificationPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_policy: Option<String>,
    #[serde(rename = "objectACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_acl: Option<String>,
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>A value that sets the write status of a file share. Set this value to <code>true</code> to set the write status to read-only, otherwise set to <code>false</code>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>A value that sets who pays the cost of the request and the cost associated with data download from the S3 bucket. If this value is set to <code>true</code>, the requester pays the costs; otherwise, the S3 bucket owner pays. However, the S3 bucket owner always pays the cost of storing data.</p> <note> <p> <code>RequesterPays</code> is a configuration for the S3 bucket that backs the file share, so make sure that the configuration on the file share is the same as the S3 bucket configuration.</p> </note> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "requesterPays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_pays: Option<bool>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "squash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash: Option<String>,
    /// <p>A list of up to 50 tags assigned to the NFS file share, sorted alphabetically by key name. Each tag is a key-value pair. For a gateway with more than 10 tags assigned, you can view all tags using the <code>ListTagsForResource</code> API operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes a gateway's network interface.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NetworkInterface {
    /// <p>The Internet Protocol version 4 (IPv4) address of the interface.</p>
    #[serde(rename = "ipv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_4_address: Option<String>,
    /// <p>The Internet Protocol version 6 (IPv6) address of the interface. <i>Currently not supported</i>.</p>
    #[serde(rename = "ipv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_address: Option<String>,
    /// <p><p>The Media Access Control (MAC) address of the interface.</p> <note> <p>This is currently unsupported and will not be returned in output.</p> </note></p>
    #[serde(rename = "macAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NotifyWhenUploadedInput {
    #[serde(rename = "fileShareARN")]
    pub file_share_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NotifyWhenUploadedOutput {
    #[serde(rename = "fileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
    #[serde(rename = "notificationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<String>,
}

/// <p>Describes a custom tape pool.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PoolInfo {
    /// <p>The Amazon Resource Name (ARN) of the custom tape pool. Use the <a>ListTapePools</a> operation to return a list of custom tape pools for your account and AWS Region.</p>
    #[serde(rename = "poolARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_arn: Option<String>,
    /// <p>The name of the custom tape pool. <code>PoolName</code> can use all ASCII characters, except '/' and '\'.</p>
    #[serde(rename = "poolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
    /// <p>Status of the custom tape pool. Pool can be <code>ACTIVE</code> or <code>DELETED</code>.</p>
    #[serde(rename = "poolStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_status: Option<String>,
    /// <p>Tape retention lock time is set in days. Tape retention lock can be enabled for up to 100 years (36,500 days).</p>
    #[serde(rename = "retentionLockTimeInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_lock_time_in_days: Option<i64>,
    /// <p>Tape retention lock type, which can be configured in two modes. When configured in governance mode, AWS accounts with specific IAM permissions are authorized to remove the tape retention lock from archived virtual tapes. When configured in compliance mode, the tape retention lock cannot be removed by any user, including the root AWS account.</p>
    #[serde(rename = "retentionLockType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_lock_type: Option<String>,
    /// <p>The storage class that is associated with the custom pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class (S3 Glacier or S3 Glacier Deep Archive) that corresponds to the pool.</p>
    #[serde(rename = "storageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

/// <p>RefreshCacheInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RefreshCacheInput {
    /// <p>The Amazon Resource Name (ARN) of the file share you want to refresh.</p>
    #[serde(rename = "fileShareARN")]
    pub file_share_arn: String,
    /// <p>A comma-separated list of the paths of folders to refresh in the cache. The default is [<code>"/"</code>]. The default refreshes objects and folders at the root of the Amazon S3 bucket. If <code>Recursive</code> is set to <code>true</code>, the entire S3 bucket that the file share has access to is refreshed.</p>
    #[serde(rename = "folderList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_list: Option<Vec<String>>,
    /// <p>A value that specifies whether to recursively refresh folders in the cache. The refresh includes folders that were in the cache the last time the gateway listed the folder's contents. If this value set to <code>true</code>, each folder that is listed in <code>FolderList</code> is recursively updated. Otherwise, subfolders listed in <code>FolderList</code> are not refreshed. Only objects that are in folders listed directly under <code>FolderList</code> are found and used for the update. The default is <code>true</code>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "recursive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
}

/// <p>RefreshCacheOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RefreshCacheOutput {
    #[serde(rename = "fileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
    #[serde(rename = "notificationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<String>,
}

/// <p>RemoveTagsFromResourceInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTagsFromResourceInput {
    /// <p>The Amazon Resource Name (ARN) of the resource you want to remove the tags from.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>The keys of the tags you want to remove from the specified resource. A tag is composed of a key-value pair.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>RemoveTagsFromResourceOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveTagsFromResourceOutput {
    /// <p>The Amazon Resource Name (ARN) of the resource that the tags were removed from.</p>
    #[serde(rename = "resourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResetCacheInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResetCacheOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>RetrieveTapeArchiveInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RetrieveTapeArchiveInput {
    /// <p>The Amazon Resource Name (ARN) of the gateway you want to retrieve the virtual tape to. Use the <a>ListGateways</a> operation to return a list of gateways for your account and AWS Region.</p> <p>You retrieve archived virtual tapes to only one gateway and the gateway must be a tape gateway.</p>
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the virtual tape you want to retrieve from the virtual tape shelf (VTS).</p>
    #[serde(rename = "tapeARN")]
    pub tape_arn: String,
}

/// <p>RetrieveTapeArchiveOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RetrieveTapeArchiveOutput {
    /// <p>The Amazon Resource Name (ARN) of the retrieved virtual tape.</p>
    #[serde(rename = "tapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

/// <p>RetrieveTapeRecoveryPointInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RetrieveTapeRecoveryPointInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the virtual tape for which you want to retrieve the recovery point.</p>
    #[serde(rename = "tapeARN")]
    pub tape_arn: String,
}

/// <p>RetrieveTapeRecoveryPointOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RetrieveTapeRecoveryPointOutput {
    /// <p>The Amazon Resource Name (ARN) of the virtual tape for which the recovery point was retrieved.</p>
    #[serde(rename = "tapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
}

/// <p>The Windows file permissions and ownership information assigned, by default, to native S3 objects when file gateway discovers them in S3 buckets. This operation is only supported for file gateways.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SMBFileShareInfo {
    /// <p>Indicates whether <code>AccessBasedEnumeration</code> is enabled.</p>
    #[serde(rename = "accessBasedEnumeration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_based_enumeration: Option<bool>,
    /// <p>A list of users or groups in the Active Directory that have administrator rights to the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    #[serde(rename = "adminUserList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_user_list: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the storage used for audit logs.</p>
    #[serde(rename = "auditDestinationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_destination_arn: Option<String>,
    #[serde(rename = "authentication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
    /// <p>Refresh cache information for the file share.</p>
    #[serde(rename = "cacheAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_attributes: Option<CacheAttributes>,
    /// <p>The case of an object name in an Amazon S3 bucket. For <code>ClientSpecified</code>, the client determines the case sensitivity. For <code>CaseSensitive</code>, the gateway determines the case sensitivity. The default value is <code>ClientSpecified</code>.</p>
    #[serde(rename = "caseSensitivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitivity: Option<String>,
    /// <p>The default storage class for objects put into an Amazon S3 bucket by the file gateway. The default value is <code>S3_INTELLIGENT_TIERING</code>. Optional.</p> <p>Valid Values: <code>S3_STANDARD</code> | <code>S3_INTELLIGENT_TIERING</code> | <code>S3_STANDARD_IA</code> | <code>S3_ONEZONE_IA</code> </p>
    #[serde(rename = "defaultStorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_storage_class: Option<String>,
    #[serde(rename = "fileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
    #[serde(rename = "fileShareId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_id: Option<String>,
    /// <p><p>The name of the file share. Optional.</p> <note> <p> <code>FileShareName</code> must be set if an S3 prefix name is set in <code>LocationARN</code>.</p> </note></p>
    #[serde(rename = "fileShareName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_name: Option<String>,
    #[serde(rename = "fileShareStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_status: Option<String>,
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>A value that enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to <code>true</code> to enable MIME type guessing, otherwise set to <code>false</code>. The default value is <code>true</code>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "guessMIMETypeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guess_mime_type_enabled: Option<bool>,
    /// <p>A list of users or groups in the Active Directory that are not allowed to access the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    #[serde(rename = "invalidUserList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_user_list: Option<Vec<String>>,
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own AWS KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "kMSEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encrypted: Option<bool>,
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    #[serde(rename = "locationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_arn: Option<String>,
    /// <p>The notification policy of the file share. <code>SettlingTimeInSeconds</code> controls the number of seconds to wait after the last point in time a client wrote to a file before generating an <code>ObjectUploaded</code> notification. Because clients can make many small writes to files, it's best to set this parameter for as long as possible to avoid generating multiple notifications for the same file in a small time period.</p> <note> <p> <code>SettlingTimeInSeconds</code> has no effect on the timing of the object uploading to Amazon S3, only the timing of the notification.</p> </note> <p>The following example sets <code>NotificationPolicy</code> on with <code>SettlingTimeInSeconds</code> set to 60.</p> <p> <code>{\"Upload\": {\"SettlingTimeInSeconds\": 60}}</code> </p> <p>The following example sets <code>NotificationPolicy</code> off.</p> <p> <code>{}</code> </p>
    #[serde(rename = "notificationPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_policy: Option<String>,
    #[serde(rename = "objectACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_acl: Option<String>,
    /// <p>The file share path used by the SMB client to identify the mount point.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>A value that sets the write status of a file share. Set this value to <code>true</code> to set the write status to read-only, otherwise set to <code>false</code>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>A value that sets who pays the cost of the request and the cost associated with data download from the S3 bucket. If this value is set to <code>true</code>, the requester pays the costs; otherwise, the S3 bucket owner pays. However, the S3 bucket owner always pays the cost of storing data.</p> <note> <p> <code>RequesterPays</code> is a configuration for the S3 bucket that backs the file share, so make sure that the configuration on the file share is the same as the S3 bucket configuration.</p> </note> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "requesterPays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_pays: Option<bool>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>If this value is set to <code>true</code>, it indicates that access control list (ACL) is enabled on the SMB file share. If it is set to <code>false</code>, it indicates that file and directory permissions are mapped to the POSIX permission.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/smb-acl.html">Using Microsoft Windows ACLs to control access to an SMB file share</a> in the <i>AWS Storage Gateway User Guide</i>.</p>
    #[serde(rename = "sMBACLEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smbacl_enabled: Option<bool>,
    /// <p>A list of up to 50 tags assigned to the SMB file share, sorted alphabetically by key name. Each tag is a key-value pair. For a gateway with more than 10 tags assigned, you can view all tags using the <code>ListTagsForResource</code> API operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A list of users or groups in the Active Directory that are allowed to access the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    #[serde(rename = "validUserList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_user_list: Option<Vec<String>>,
}

/// <p>SetLocalConsolePasswordInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetLocalConsolePasswordInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>The password you want to set for your VM local console.</p>
    #[serde(rename = "localConsolePassword")]
    pub local_console_password: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SetLocalConsolePasswordOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>SetSMBGuestPasswordInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SetSMBGuestPasswordInput {
    /// <p>The Amazon Resource Name (ARN) of the file gateway the SMB file share is associated with.</p>
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>The password that you want to set for your SMB server.</p>
    #[serde(rename = "password")]
    pub password: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SetSMBGuestPasswordOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway to shut down.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ShutdownGatewayInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway that was shut down.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ShutdownGatewayOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartAvailabilityMonitorTestInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartAvailabilityMonitorTestOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway to start.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartGatewayInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway that was restarted.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartGatewayOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>Provides additional information about an error that was returned by the service. See the <code>errorCode</code> and <code>errorDetails</code> members for more information about the error.</p>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageGatewayError {
    /// <p>Additional information about the error.</p>
    pub error_code: Option<String>,
    /// <p>Human-readable text that provides detail about the error that occurred.</p>
    pub error_details: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Describes an iSCSI stored volume.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StorediSCSIVolume {
    /// <p>The date the volume was created. Volumes created prior to March 28, 2017 don’t have this timestamp.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>Indicates if when the stored volume was created, existing data on the underlying local disk was preserved.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "preservedExistingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserved_existing_data: Option<bool>,
    /// <p>If the stored volume was created from a snapshot, this field contains the snapshot ID used, e.g. snap-78e22663. Otherwise, this field is not included.</p>
    #[serde(rename = "sourceSnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_snapshot_id: Option<String>,
    /// <p>The name of the iSCSI target used by an initiator to connect to a volume and used as a suffix for the target ARN. For example, specifying <code>TargetName</code> as <i>myvolume</i> results in the target ARN of <code>arn:aws:storagegateway:us-east-2:111122223333:gateway/sgw-12A3456B/target/iqn.1997-05.com.amazon:myvolume</code>. The target name must be unique across all volumes on a gateway.</p> <p>If you don't specify a value, Storage Gateway uses the value that was previously used for this volume as the new target name.</p>
    #[serde(rename = "targetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the storage volume.</p>
    #[serde(rename = "volumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
    /// <p>A value that indicates whether a storage volume is attached to, detached from, or is in the process of detaching from a gateway. For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/managing-volumes.html#attach-detach-volume">Moving your volumes to a different gateway</a>.</p>
    #[serde(rename = "volumeAttachmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_attachment_status: Option<String>,
    /// <p>The ID of the local disk that was specified in the <a>CreateStorediSCSIVolume</a> operation.</p>
    #[serde(rename = "volumeDiskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_disk_id: Option<String>,
    /// <p>The unique identifier of the volume, e.g., vol-AE4B946D.</p>
    #[serde(rename = "volumeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    /// <p>Represents the percentage complete if the volume is restoring or bootstrapping that represents the percent of data transferred. This field does not appear in the response if the stored volume is not restoring or bootstrapping.</p>
    #[serde(rename = "volumeProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_progress: Option<f64>,
    /// <p>The size of the volume in bytes.</p>
    #[serde(rename = "volumeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_bytes: Option<i64>,
    /// <p>One of the VolumeStatus values that indicates the state of the storage volume.</p>
    #[serde(rename = "volumeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_status: Option<String>,
    /// <p>One of the VolumeType enumeration values describing the type of the volume.</p>
    #[serde(rename = "volumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
    /// <p><p>The size of the data stored on the volume in bytes. This value is calculated based on the number of blocks that are touched, instead of the actual amount of data written. This value can be useful for sequential write patterns but less accurate for random write patterns. <code>VolumeUsedInBytes</code> is different from the compressed size of the volume, which is the value that is used to calculate your bill.</p> <note> <p>This value is not available for volumes created prior to May 13, 2015, until you store data on the volume.</p> </note></p>
    #[serde(rename = "volumeUsedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_used_in_bytes: Option<i64>,
    /// <p>An <a>VolumeiSCSIAttributes</a> object that represents a collection of iSCSI attributes for one stored volume.</p>
    #[serde(rename = "volumeiSCSIAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumei_scsi_attributes: Option<VolumeiSCSIAttributes>,
}

/// <p>A key-value pair that helps you manage, filter, and search for your resource. Allowed characters: letters, white space, and numbers, representable in UTF-8, and the following characters: + - = . _ : /.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>Tag key. The key can't start with aws:.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>Value of the tag key.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>Describes a virtual tape object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Tape {
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The date that the tape enters a custom tape pool.</p>
    #[serde(rename = "poolEntryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_entry_date: Option<f64>,
    /// <p>The ID of the pool that contains tapes that will be archived. The tapes in this pool are archived in the S3 storage class that is associated with the pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class (S3 Glacier or S3 Glacier Deep Archive) that corresponds to the pool.</p> <p>Valid Values: <code>GLACIER</code> | <code>DEEP_ARCHIVE</code> </p>
    #[serde(rename = "poolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    /// <p>For archiving virtual tapes, indicates how much data remains to be uploaded before archiving is complete.</p> <p>Range: 0 (not started) to 100 (complete).</p>
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    /// <p>The date that the tape is first archived with tape retention lock enabled.</p>
    #[serde(rename = "retentionStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_start_date: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the virtual tape.</p>
    #[serde(rename = "tapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
    /// <p>The barcode that identifies a specific virtual tape.</p>
    #[serde(rename = "tapeBarcode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_barcode: Option<String>,
    /// <p>The date the virtual tape was created.</p>
    #[serde(rename = "tapeCreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_created_date: Option<f64>,
    /// <p>The size, in bytes, of the virtual tape capacity.</p>
    #[serde(rename = "tapeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_size_in_bytes: Option<i64>,
    /// <p>The current state of the virtual tape.</p>
    #[serde(rename = "tapeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_status: Option<String>,
    /// <p><p>The size, in bytes, of data stored on the virtual tape.</p> <note> <p>This value is not available for tapes created prior to May 13, 2015.</p> </note></p>
    #[serde(rename = "tapeUsedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_used_in_bytes: Option<i64>,
    /// <p>The virtual tape library (VTL) device that the virtual tape is associated with.</p>
    #[serde(rename = "vTLDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_device: Option<String>,
    /// <p>If the tape is archived as write-once-read-many (WORM), this value is <code>true</code>.</p>
    #[serde(rename = "worm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worm: Option<bool>,
}

/// <p>Represents a virtual tape that is archived in the virtual tape shelf (VTS).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TapeArchive {
    /// <p>The time that the archiving of the virtual tape was completed.</p> <p>The default timestamp format is in the ISO8601 extended YYYY-MM-DD'T'HH:MM:SS'Z' format.</p>
    #[serde(rename = "completionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The time that the tape entered the custom tape pool.</p> <p>The default timestamp format is in the ISO8601 extended YYYY-MM-DD'T'HH:MM:SS'Z' format.</p>
    #[serde(rename = "poolEntryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_entry_date: Option<f64>,
    /// <p>The ID of the pool that was used to archive the tape. The tapes in this pool are archived in the S3 storage class that is associated with the pool.</p> <p>Valid Values: <code>GLACIER</code> | <code>DEEP_ARCHIVE</code> </p>
    #[serde(rename = "poolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    /// <p>If the archived tape is subject to tape retention lock, the date that the archived tape started being retained.</p>
    #[serde(rename = "retentionStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_start_date: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the tape gateway that the virtual tape is being retrieved to.</p> <p>The virtual tape is retrieved from the virtual tape shelf (VTS).</p>
    #[serde(rename = "retrievedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_to: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of an archived virtual tape.</p>
    #[serde(rename = "tapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
    /// <p>The barcode that identifies the archived virtual tape.</p>
    #[serde(rename = "tapeBarcode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_barcode: Option<String>,
    /// <p>The date the virtual tape was created.</p>
    #[serde(rename = "tapeCreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_created_date: Option<f64>,
    /// <p>The size, in bytes, of the archived virtual tape.</p>
    #[serde(rename = "tapeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_size_in_bytes: Option<i64>,
    /// <p>The current state of the archived virtual tape.</p>
    #[serde(rename = "tapeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_status: Option<String>,
    /// <p><p>The size, in bytes, of data stored on the virtual tape.</p> <note> <p>This value is not available for tapes created prior to May 13, 2015.</p> </note></p>
    #[serde(rename = "tapeUsedInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_used_in_bytes: Option<i64>,
    /// <p>Set to <code>true</code> if the archived tape is stored as write-once-read-many (WORM).</p>
    #[serde(rename = "worm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worm: Option<bool>,
}

/// <p>Describes a virtual tape.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TapeInfo {
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <a>ListGateways</a> operation to return a list of gateways for your account and AWS Region.</p>
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The date that the tape entered the custom tape pool with tape retention lock enabled.</p>
    #[serde(rename = "poolEntryDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_entry_date: Option<f64>,
    /// <p>The ID of the pool that you want to add your tape to for archiving. The tape in this pool is archived in the S3 storage class that is associated with the pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class (S3 Glacier or S3 Glacier Deep Archive) that corresponds to the pool.</p> <p>Valid Values: <code>GLACIER</code> | <code>DEEP_ARCHIVE</code> </p>
    #[serde(rename = "poolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    /// <p>The date that the tape became subject to tape retention lock.</p>
    #[serde(rename = "retentionStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_start_date: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of a virtual tape.</p>
    #[serde(rename = "tapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
    /// <p>The barcode that identifies a specific virtual tape.</p>
    #[serde(rename = "tapeBarcode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_barcode: Option<String>,
    /// <p>The size, in bytes, of a virtual tape.</p>
    #[serde(rename = "tapeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_size_in_bytes: Option<i64>,
    /// <p>The status of the tape.</p>
    #[serde(rename = "tapeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_status: Option<String>,
}

/// <p>Describes a recovery point.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TapeRecoveryPointInfo {
    /// <p>The Amazon Resource Name (ARN) of the virtual tape.</p>
    #[serde(rename = "tapeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_arn: Option<String>,
    /// <p>The time when the point-in-time view of the virtual tape was replicated for later recovery.</p> <p>The default timestamp format of the tape recovery point time is in the ISO8601 extended YYYY-MM-DD'T'HH:MM:SS'Z' format.</p>
    #[serde(rename = "tapeRecoveryPointTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_recovery_point_time: Option<f64>,
    /// <p>The size, in bytes, of the virtual tapes to recover.</p>
    #[serde(rename = "tapeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_size_in_bytes: Option<i64>,
    /// <p>The status of the virtual tapes.</p>
    #[serde(rename = "tapeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tape_status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateAutomaticTapeCreationPolicyInput {
    /// <p>An automatic tape creation policy consists of a list of automatic tape creation rules. The rules determine when and how to automatically create new tapes.</p>
    #[serde(rename = "automaticTapeCreationRules")]
    pub automatic_tape_creation_rules: Vec<AutomaticTapeCreationRule>,
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateAutomaticTapeCreationPolicyOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>UpdateBandwidthRateLimitInput$AverageDownloadRateLimitInBitsPerSec</a> </p> </li> <li> <p> <a>UpdateBandwidthRateLimitInput$AverageUploadRateLimitInBitsPerSec</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateBandwidthRateLimitInput {
    /// <p>The average download bandwidth rate limit in bits per second.</p>
    #[serde(rename = "averageDownloadRateLimitInBitsPerSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_download_rate_limit_in_bits_per_sec: Option<i64>,
    /// <p>The average upload bandwidth rate limit in bits per second.</p>
    #[serde(rename = "averageUploadRateLimitInBitsPerSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_upload_rate_limit_in_bits_per_sec: Option<i64>,
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway whose throttle information was updated.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateBandwidthRateLimitOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateBandwidthRateLimitScheduleInput {
    /// <p> An array containing bandwidth rate limit schedule intervals for a gateway. When no bandwidth rate limit intervals have been scheduled, the array is empty. </p>
    #[serde(rename = "bandwidthRateLimitIntervals")]
    pub bandwidth_rate_limit_intervals: Vec<BandwidthRateLimitInterval>,
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateBandwidthRateLimitScheduleOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>UpdateChapCredentialsInput$InitiatorName</a> </p> </li> <li> <p> <a>UpdateChapCredentialsInput$SecretToAuthenticateInitiator</a> </p> </li> <li> <p> <a>UpdateChapCredentialsInput$SecretToAuthenticateTarget</a> </p> </li> <li> <p> <a>UpdateChapCredentialsInput$TargetARN</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateChapCredentialsInput {
    /// <p>The iSCSI initiator that connects to the target.</p>
    #[serde(rename = "initiatorName")]
    pub initiator_name: String,
    /// <p><p>The secret key that the initiator (for example, the Windows client) must provide to participate in mutual CHAP with the target.</p> <note> <p>The secret key must be between 12 and 16 bytes when encoded in UTF-8.</p> </note></p>
    #[serde(rename = "secretToAuthenticateInitiator")]
    pub secret_to_authenticate_initiator: String,
    /// <p><p>The secret key that the target must provide to participate in mutual CHAP with the initiator (e.g. Windows client).</p> <p>Byte constraints: Minimum bytes of 12. Maximum bytes of 16.</p> <note> <p>The secret key must be between 12 and 16 bytes when encoded in UTF-8.</p> </note></p>
    #[serde(rename = "secretToAuthenticateTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_to_authenticate_target: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <a>DescribeStorediSCSIVolumes</a> operation to return the TargetARN for specified VolumeARN.</p>
    #[serde(rename = "targetARN")]
    pub target_arn: String,
}

/// <p>A JSON object containing the following fields:</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateChapCredentialsOutput {
    /// <p>The iSCSI initiator that connects to the target. This is the same initiator name specified in the request.</p>
    #[serde(rename = "initiatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the target. This is the same target specified in the request.</p>
    #[serde(rename = "targetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFileSystemAssociationInput {
    /// <p>The Amazon Resource Name (ARN) of the storage used for the audit logs.</p>
    #[serde(rename = "auditDestinationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_destination_arn: Option<String>,
    #[serde(rename = "cacheAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_attributes: Option<CacheAttributes>,
    /// <p>The Amazon Resource Name (ARN) of the file system association that you want to update.</p>
    #[serde(rename = "fileSystemAssociationARN")]
    pub file_system_association_arn: String,
    /// <p>The password of the user credential.</p>
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The user name of the user credential that has permission to access the root share D$ of the Amazon FSx file system. The user account must belong to the Amazon FSx delegated admin user group.</p>
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateFileSystemAssociationOutput {
    /// <p>The ARN of the updated file system association.</p>
    #[serde(rename = "fileSystemAssociationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_association_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGatewayInformationInput {
    /// <p>The Amazon Resource Name (ARN) of the Amazon CloudWatch log group that you want to use to monitor and log events in the gateway.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/WhatIsCloudWatchLogs.html">What is Amazon CloudWatch Logs?</a> </p>
    #[serde(rename = "cloudWatchLogGroupARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_arn: Option<String>,
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    #[serde(rename = "gatewayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_name: Option<String>,
    /// <p>A value that indicates the time zone of the gateway.</p>
    #[serde(rename = "gatewayTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_timezone: Option<String>,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway that was updated.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGatewayInformationOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The name you configured for your gateway.</p>
    #[serde(rename = "gatewayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_name: Option<String>,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway to update.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateGatewaySoftwareNowInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway that was updated.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateGatewaySoftwareNowOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p><p>A JSON object containing the following fields:</p> <ul> <li> <p> <a>UpdateMaintenanceStartTimeInput$DayOfMonth</a> </p> </li> <li> <p> <a>UpdateMaintenanceStartTimeInput$DayOfWeek</a> </p> </li> <li> <p> <a>UpdateMaintenanceStartTimeInput$HourOfDay</a> </p> </li> <li> <p> <a>UpdateMaintenanceStartTimeInput$MinuteOfHour</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMaintenanceStartTimeInput {
    /// <p>The day of the month component of the maintenance start time represented as an ordinal number from 1 to 28, where 1 represents the first day of the month and 28 represents the last day of the month.</p>
    #[serde(rename = "dayOfMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<i64>,
    /// <p>The day of the week component of the maintenance start time week represented as an ordinal number from 0 to 6, where 0 represents Sunday and 6 Saturday.</p>
    #[serde(rename = "dayOfWeek")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i64>,
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>The hour component of the maintenance start time represented as <i>hh</i>, where <i>hh</i> is the hour (00 to 23). The hour of the day is in the time zone of the gateway.</p>
    #[serde(rename = "hourOfDay")]
    pub hour_of_day: i64,
    /// <p>The minute component of the maintenance start time represented as <i>mm</i>, where <i>mm</i> is the minute (00 to 59). The minute of the hour is in the time zone of the gateway.</p>
    #[serde(rename = "minuteOfHour")]
    pub minute_of_hour: i64,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the gateway whose maintenance start time is updated.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMaintenanceStartTimeOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p>UpdateNFSFileShareInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateNFSFileShareInput {
    /// <p>specifies refresh cache information for the file share.</p>
    #[serde(rename = "cacheAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_attributes: Option<CacheAttributes>,
    /// <p>The list of clients that are allowed to access the file gateway. The list must contain either valid IP addresses or valid CIDR blocks.</p>
    #[serde(rename = "clientList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_list: Option<Vec<String>>,
    /// <p>The default storage class for objects put into an Amazon S3 bucket by the file gateway. The default value is <code>S3_INTELLIGENT_TIERING</code>. Optional.</p> <p>Valid Values: <code>S3_STANDARD</code> | <code>S3_INTELLIGENT_TIERING</code> | <code>S3_STANDARD_IA</code> | <code>S3_ONEZONE_IA</code> </p>
    #[serde(rename = "defaultStorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_storage_class: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the file share to be updated.</p>
    #[serde(rename = "fileShareARN")]
    pub file_share_arn: String,
    /// <p><p>The name of the file share. Optional.</p> <note> <p> <code>FileShareName</code> must be set if an S3 prefix name is set in <code>LocationARN</code>.</p> </note></p>
    #[serde(rename = "fileShareName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_name: Option<String>,
    /// <p>A value that enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to <code>true</code> to enable MIME type guessing, otherwise set to <code>false</code>. The default value is <code>true</code>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "guessMIMETypeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guess_mime_type_enabled: Option<bool>,
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own AWS KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "kMSEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encrypted: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The default values for the file share. Optional.</p>
    #[serde(rename = "nFSFileShareDefaults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_file_share_defaults: Option<NFSFileShareDefaults>,
    /// <p>The notification policy of the file share. <code>SettlingTimeInSeconds</code> controls the number of seconds to wait after the last point in time a client wrote to a file before generating an <code>ObjectUploaded</code> notification. Because clients can make many small writes to files, it's best to set this parameter for as long as possible to avoid generating multiple notifications for the same file in a small time period.</p> <note> <p> <code>SettlingTimeInSeconds</code> has no effect on the timing of the object uploading to Amazon S3, only the timing of the notification.</p> </note> <p>The following example sets <code>NotificationPolicy</code> on with <code>SettlingTimeInSeconds</code> set to 60.</p> <p> <code>{\"Upload\": {\"SettlingTimeInSeconds\": 60}}</code> </p> <p>The following example sets <code>NotificationPolicy</code> off.</p> <p> <code>{}</code> </p>
    #[serde(rename = "notificationPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_policy: Option<String>,
    /// <p>A value that sets the access control list (ACL) permission for objects in the S3 bucket that a file gateway puts objects into. The default value is <code>private</code>.</p>
    #[serde(rename = "objectACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_acl: Option<String>,
    /// <p>A value that sets the write status of a file share. Set this value to <code>true</code> to set the write status to read-only, otherwise set to <code>false</code>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>A value that sets who pays the cost of the request and the cost associated with data download from the S3 bucket. If this value is set to <code>true</code>, the requester pays the costs; otherwise, the S3 bucket owner pays. However, the S3 bucket owner always pays the cost of storing data.</p> <note> <p> <code>RequesterPays</code> is a configuration for the S3 bucket that backs the file share, so make sure that the configuration on the file share is the same as the S3 bucket configuration.</p> </note> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "requesterPays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_pays: Option<bool>,
    /// <p><p>The user mapped to anonymous user.</p> <p>Valid values are the following:</p> <ul> <li> <p> <code>RootSquash</code>: Only root is mapped to anonymous user.</p> </li> <li> <p> <code>NoSquash</code>: No one is mapped to anonymous user.</p> </li> <li> <p> <code>AllSquash</code>: Everyone is mapped to anonymous user.</p> </li> </ul></p>
    #[serde(rename = "squash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash: Option<String>,
}

/// <p>UpdateNFSFileShareOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateNFSFileShareOutput {
    /// <p>The Amazon Resource Name (ARN) of the updated file share.</p>
    #[serde(rename = "fileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
}

/// <p>UpdateSMBFileShareInput</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSMBFileShareInput {
    /// <p>The files and folders on this share will only be visible to users with read access.</p>
    #[serde(rename = "accessBasedEnumeration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_based_enumeration: Option<bool>,
    /// <p>A list of users or groups in the Active Directory that have administrator rights to the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    #[serde(rename = "adminUserList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_user_list: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the storage used for audit logs.</p>
    #[serde(rename = "auditDestinationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_destination_arn: Option<String>,
    /// <p>Specifies refresh cache information for the file share.</p>
    #[serde(rename = "cacheAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_attributes: Option<CacheAttributes>,
    /// <p>The case of an object name in an Amazon S3 bucket. For <code>ClientSpecified</code>, the client determines the case sensitivity. For <code>CaseSensitive</code>, the gateway determines the case sensitivity. The default value is <code>ClientSpecified</code>.</p>
    #[serde(rename = "caseSensitivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitivity: Option<String>,
    /// <p>The default storage class for objects put into an Amazon S3 bucket by the file gateway. The default value is <code>S3_INTELLIGENT_TIERING</code>. Optional.</p> <p>Valid Values: <code>S3_STANDARD</code> | <code>S3_INTELLIGENT_TIERING</code> | <code>S3_STANDARD_IA</code> | <code>S3_ONEZONE_IA</code> </p>
    #[serde(rename = "defaultStorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_storage_class: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the SMB file share that you want to update.</p>
    #[serde(rename = "fileShareARN")]
    pub file_share_arn: String,
    /// <p><p>The name of the file share. Optional.</p> <note> <p> <code>FileShareName</code> must be set if an S3 prefix name is set in <code>LocationARN</code>.</p> </note></p>
    #[serde(rename = "fileShareName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_name: Option<String>,
    /// <p>A value that enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to <code>true</code> to enable MIME type guessing, otherwise set to <code>false</code>. The default value is <code>true</code>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "guessMIMETypeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guess_mime_type_enabled: Option<bool>,
    /// <p>A list of users or groups in the Active Directory that are not allowed to access the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    #[serde(rename = "invalidUserList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_user_list: Option<Vec<String>>,
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own AWS KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "kMSEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encrypted: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    #[serde(rename = "kMSKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    /// <p>The notification policy of the file share. <code>SettlingTimeInSeconds</code> controls the number of seconds to wait after the last point in time a client wrote to a file before generating an <code>ObjectUploaded</code> notification. Because clients can make many small writes to files, it's best to set this parameter for as long as possible to avoid generating multiple notifications for the same file in a small time period.</p> <note> <p> <code>SettlingTimeInSeconds</code> has no effect on the timing of the object uploading to Amazon S3, only the timing of the notification.</p> </note> <p>The following example sets <code>NotificationPolicy</code> on with <code>SettlingTimeInSeconds</code> set to 60.</p> <p> <code>{\"Upload\": {\"SettlingTimeInSeconds\": 60}}</code> </p> <p>The following example sets <code>NotificationPolicy</code> off.</p> <p> <code>{}</code> </p>
    #[serde(rename = "notificationPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_policy: Option<String>,
    /// <p>A value that sets the access control list (ACL) permission for objects in the S3 bucket that a file gateway puts objects into. The default value is <code>private</code>.</p>
    #[serde(rename = "objectACL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_acl: Option<String>,
    /// <p>A value that sets the write status of a file share. Set this value to <code>true</code> to set write status to read-only, otherwise set to <code>false</code>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>A value that sets who pays the cost of the request and the cost associated with data download from the S3 bucket. If this value is set to <code>true</code>, the requester pays the costs; otherwise, the S3 bucket owner pays. However, the S3 bucket owner always pays the cost of storing data.</p> <note> <p> <code>RequesterPays</code> is a configuration for the S3 bucket that backs the file share, so make sure that the configuration on the file share is the same as the S3 bucket configuration.</p> </note> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "requesterPays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_pays: Option<bool>,
    /// <p>Set this value to <code>true</code> to enable access control list (ACL) on the SMB file share. Set it to <code>false</code> to map file and directory permissions to the POSIX permissions.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/smb-acl.html">Using Microsoft Windows ACLs to control access to an SMB file share</a> in the <i>AWS Storage Gateway User Guide</i>.</p> <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[serde(rename = "sMBACLEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smbacl_enabled: Option<bool>,
    /// <p>A list of users or groups in the Active Directory that are allowed to access the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    #[serde(rename = "validUserList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_user_list: Option<Vec<String>>,
}

/// <p>UpdateSMBFileShareOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSMBFileShareOutput {
    /// <p>The Amazon Resource Name (ARN) of the updated SMB file share.</p>
    #[serde(rename = "fileShareARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSMBFileShareVisibilityInput {
    /// <p>The shares on this gateway appear when listing shares.</p>
    #[serde(rename = "fileSharesVisible")]
    pub file_shares_visible: bool,
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSMBFileShareVisibilityOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSMBSecurityStrategyInput {
    #[serde(rename = "gatewayARN")]
    pub gateway_arn: String,
    /// <p>Specifies the type of security strategy.</p> <p>ClientSpecified: if you use this option, requests are established based on what is negotiated by the client. This option is recommended when you want to maximize compatibility across different clients in your environment.</p> <p>MandatorySigning: if you use this option, file gateway only allows connections from SMBv2 or SMBv3 clients that have signing enabled. This option works with SMB clients on Microsoft Windows Vista, Windows Server 2008 or newer.</p> <p>MandatoryEncryption: if you use this option, file gateway only allows connections from SMBv3 clients that have encryption enabled. This option is highly recommended for environments that handle sensitive data. This option works with SMB clients on Microsoft Windows 8, Windows Server 2012 or newer.</p>
    #[serde(rename = "sMBSecurityStrategy")]
    pub smb_security_strategy: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSMBSecurityStrategyOutput {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
}

/// <p><p>A JSON object containing one or more of the following fields:</p> <ul> <li> <p> <a>UpdateSnapshotScheduleInput$Description</a> </p> </li> <li> <p> <a>UpdateSnapshotScheduleInput$RecurrenceInHours</a> </p> </li> <li> <p> <a>UpdateSnapshotScheduleInput$StartAt</a> </p> </li> <li> <p> <a>UpdateSnapshotScheduleInput$VolumeARN</a> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateSnapshotScheduleInput {
    /// <p>Optional description of the snapshot that overwrites the existing description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Frequency of snapshots. Specify the number of hours between snapshots.</p>
    #[serde(rename = "recurrenceInHours")]
    pub recurrence_in_hours: i64,
    /// <p>The hour of the day at which the snapshot schedule begins represented as <i>hh</i>, where <i>hh</i> is the hour (0 to 23). The hour of the day is in the time zone of the gateway.</p>
    #[serde(rename = "startAt")]
    pub start_at: i64,
    /// <p><p>A list of up to 50 tags that can be assigned to a snapshot. Each tag is a key-value pair.</p> <note> <p>Valid characters for key and value are letters, spaces, and numbers representable in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag&#39;s key is 128 characters, and the maximum length for a tag&#39;s value is 256.</p> </note></p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The Amazon Resource Name (ARN) of the volume. Use the <a>ListVolumes</a> operation to return a list of gateway volumes.</p>
    #[serde(rename = "volumeARN")]
    pub volume_arn: String,
}

/// <p>A JSON object containing the Amazon Resource Name (ARN) of the updated storage volume.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateSnapshotScheduleOutput {
    /// <p>The Amazon Resource Name (ARN) of the volume. Use the <a>ListVolumes</a> operation to return a list of gateway volumes.</p>
    #[serde(rename = "volumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateVTLDeviceTypeInput {
    /// <p>The type of medium changer you want to select.</p> <p>Valid Values: <code>STK-L700</code> | <code>AWS-Gateway-VTL</code> | <code>IBM-03584L32-0402</code> </p>
    #[serde(rename = "deviceType")]
    pub device_type: String,
    /// <p>The Amazon Resource Name (ARN) of the medium changer you want to select.</p>
    #[serde(rename = "vTLDeviceARN")]
    pub vtl_device_arn: String,
}

/// <p>UpdateVTLDeviceTypeOutput</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateVTLDeviceTypeOutput {
    /// <p>The Amazon Resource Name (ARN) of the medium changer you have selected.</p>
    #[serde(rename = "vTLDeviceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_device_arn: Option<String>,
}

/// <p>Represents a device object associated with a tape gateway.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VTLDevice {
    /// <p>A list of iSCSI information about a VTL device.</p>
    #[serde(rename = "deviceiSCSIAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devicei_scsi_attributes: Option<DeviceiSCSIAttributes>,
    /// <p>Specifies the unique Amazon Resource Name (ARN) of the device (tape drive or media changer).</p>
    #[serde(rename = "vTLDeviceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_device_arn: Option<String>,
    /// <p>Specifies the model number of device that the VTL device emulates.</p>
    #[serde(rename = "vTLDeviceProductIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_device_product_identifier: Option<String>,
    /// <p>Specifies the type of device that the VTL device emulates.</p>
    #[serde(rename = "vTLDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_device_type: Option<String>,
    /// <p>Specifies the vendor of the device that the VTL device object emulates.</p>
    #[serde(rename = "vTLDeviceVendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtl_device_vendor: Option<String>,
}

/// <p>Describes a storage volume object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VolumeInfo {
    #[serde(rename = "gatewayARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_arn: Option<String>,
    /// <p>The unique identifier assigned to your gateway during activation. This ID becomes part of the gateway Amazon Resource Name (ARN), which you use as input for other operations.</p> <p>Valid Values: 50 to 500 lowercase letters, numbers, periods (.), and hyphens (-).</p>
    #[serde(rename = "gatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the storage volume. For example, the following is a valid ARN:</p> <p> <code>arn:aws:storagegateway:us-east-2:111122223333:gateway/sgw-12A3456B/volume/vol-1122AABB</code> </p> <p>Valid Values: 50 to 500 lowercase letters, numbers, periods (.), and hyphens (-).</p>
    #[serde(rename = "volumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
    /// <p>One of the VolumeStatus values that indicates the state of the storage volume.</p>
    #[serde(rename = "volumeAttachmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_attachment_status: Option<String>,
    /// <p>The unique identifier assigned to the volume. This ID becomes part of the volume Amazon Resource Name (ARN), which you use as input for other operations.</p> <p>Valid Values: 50 to 500 lowercase letters, numbers, periods (.), and hyphens (-).</p>
    #[serde(rename = "volumeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    /// <p>The size of the volume in bytes.</p> <p>Valid Values: 50 to 500 lowercase letters, numbers, periods (.), and hyphens (-).</p>
    #[serde(rename = "volumeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_bytes: Option<i64>,
    /// <p>One of the VolumeType enumeration values describing the type of the volume.</p>
    #[serde(rename = "volumeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

/// <p>Describes a storage volume recovery point object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VolumeRecoveryPointInfo {
    /// <p>The Amazon Resource Name (ARN) of the volume target.</p>
    #[serde(rename = "volumeARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_arn: Option<String>,
    /// <p>The time the recovery point was taken.</p>
    #[serde(rename = "volumeRecoveryPointTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_recovery_point_time: Option<String>,
    /// <p>The size of the volume in bytes.</p>
    #[serde(rename = "volumeSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_bytes: Option<i64>,
    /// <p><p>The size of the data stored on the volume in bytes.</p> <note> <p>This value is not available for volumes created prior to May 13, 2015, until you store data on the volume.</p> </note></p>
    #[serde(rename = "volumeUsageInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_usage_in_bytes: Option<i64>,
}

/// <p>Lists iSCSI information about a volume.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VolumeiSCSIAttributes {
    /// <p>Indicates whether mutual CHAP is enabled for the iSCSI target.</p>
    #[serde(rename = "chapEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chap_enabled: Option<bool>,
    /// <p>The logical disk number.</p>
    #[serde(rename = "lunNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lun_number: Option<i64>,
    /// <p>The network interface identifier.</p>
    #[serde(rename = "networkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>The port used to communicate with iSCSI targets.</p>
    #[serde(rename = "networkInterfacePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_port: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the volume target.</p>
    #[serde(rename = "targetARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

/// Errors returned by ActivateGateway
#[derive(Debug, PartialEq)]
pub enum ActivateGatewayError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl ActivateGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ActivateGatewayError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ActivateGatewayError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(ActivateGatewayError::InvalidGatewayRequest(
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
impl fmt::Display for ActivateGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ActivateGatewayError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ActivateGatewayError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ActivateGatewayError {}
/// Errors returned by AddCache
#[derive(Debug, PartialEq)]
pub enum AddCacheError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl AddCacheError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddCacheError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(AddCacheError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(AddCacheError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddCacheError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddCacheError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AddCacheError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddCacheError {}
/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl AddTagsToResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsToResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(AddTagsToResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(AddTagsToResourceError::InvalidGatewayRequest(
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
impl fmt::Display for AddTagsToResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsToResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AddTagsToResourceError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsToResourceError {}
/// Errors returned by AddUploadBuffer
#[derive(Debug, PartialEq)]
pub enum AddUploadBufferError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl AddUploadBufferError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddUploadBufferError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(AddUploadBufferError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(AddUploadBufferError::InvalidGatewayRequest(
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
impl fmt::Display for AddUploadBufferError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddUploadBufferError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AddUploadBufferError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddUploadBufferError {}
/// Errors returned by AddWorkingStorage
#[derive(Debug, PartialEq)]
pub enum AddWorkingStorageError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl AddWorkingStorageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddWorkingStorageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(AddWorkingStorageError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(AddWorkingStorageError::InvalidGatewayRequest(
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
impl fmt::Display for AddWorkingStorageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddWorkingStorageError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AddWorkingStorageError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddWorkingStorageError {}
/// Errors returned by AssignTapePool
#[derive(Debug, PartialEq)]
pub enum AssignTapePoolError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl AssignTapePoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssignTapePoolError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(AssignTapePoolError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(AssignTapePoolError::InvalidGatewayRequest(
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
impl fmt::Display for AssignTapePoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssignTapePoolError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AssignTapePoolError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssignTapePoolError {}
/// Errors returned by AssociateFileSystem
#[derive(Debug, PartialEq)]
pub enum AssociateFileSystemError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl AssociateFileSystemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateFileSystemError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(AssociateFileSystemError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(AssociateFileSystemError::InvalidGatewayRequest(
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
impl fmt::Display for AssociateFileSystemError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateFileSystemError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AssociateFileSystemError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateFileSystemError {}
/// Errors returned by AttachVolume
#[derive(Debug, PartialEq)]
pub enum AttachVolumeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl AttachVolumeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AttachVolumeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(AttachVolumeError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(AttachVolumeError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AttachVolumeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttachVolumeError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AttachVolumeError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AttachVolumeError {}
/// Errors returned by CancelArchival
#[derive(Debug, PartialEq)]
pub enum CancelArchivalError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl CancelArchivalError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelArchivalError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(CancelArchivalError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(CancelArchivalError::InvalidGatewayRequest(
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
impl fmt::Display for CancelArchivalError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelArchivalError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CancelArchivalError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelArchivalError {}
/// Errors returned by CancelRetrieval
#[derive(Debug, PartialEq)]
pub enum CancelRetrievalError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl CancelRetrievalError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelRetrievalError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(CancelRetrievalError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(CancelRetrievalError::InvalidGatewayRequest(
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
impl fmt::Display for CancelRetrievalError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelRetrievalError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CancelRetrievalError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelRetrievalError {}
/// Errors returned by CreateCachediSCSIVolume
#[derive(Debug, PartialEq)]
pub enum CreateCachediSCSIVolumeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl CreateCachediSCSIVolumeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCachediSCSIVolumeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(CreateCachediSCSIVolumeError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        CreateCachediSCSIVolumeError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateCachediSCSIVolumeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCachediSCSIVolumeError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateCachediSCSIVolumeError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateCachediSCSIVolumeError {}
/// Errors returned by CreateNFSFileShare
#[derive(Debug, PartialEq)]
pub enum CreateNFSFileShareError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl CreateNFSFileShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateNFSFileShareError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(CreateNFSFileShareError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(CreateNFSFileShareError::InvalidGatewayRequest(
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
impl fmt::Display for CreateNFSFileShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateNFSFileShareError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateNFSFileShareError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateNFSFileShareError {}
/// Errors returned by CreateSMBFileShare
#[derive(Debug, PartialEq)]
pub enum CreateSMBFileShareError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl CreateSMBFileShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSMBFileShareError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(CreateSMBFileShareError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(CreateSMBFileShareError::InvalidGatewayRequest(
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
impl fmt::Display for CreateSMBFileShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSMBFileShareError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateSMBFileShareError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSMBFileShareError {}
/// Errors returned by CreateSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateSnapshotError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// <p>An internal server error has occurred because the service is unavailable. For more information, see the error and message fields.</p>
    ServiceUnavailableError(String),
}

impl CreateSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(CreateSnapshotError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(CreateSnapshotError::InvalidGatewayRequest(
                        err.msg,
                    ))
                }
                "ServiceUnavailableError" => {
                    return RusotoError::Service(CreateSnapshotError::ServiceUnavailableError(
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
impl fmt::Display for CreateSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSnapshotError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateSnapshotError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
            CreateSnapshotError::ServiceUnavailableError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSnapshotError {}
/// Errors returned by CreateSnapshotFromVolumeRecoveryPoint
#[derive(Debug, PartialEq)]
pub enum CreateSnapshotFromVolumeRecoveryPointError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
    /// <p>An internal server error has occurred because the service is unavailable. For more information, see the error and message fields.</p>
    ServiceUnavailableError(String),
}

impl CreateSnapshotFromVolumeRecoveryPointError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateSnapshotFromVolumeRecoveryPointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        CreateSnapshotFromVolumeRecoveryPointError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        CreateSnapshotFromVolumeRecoveryPointError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ServiceUnavailableError" => {
                    return RusotoError::Service(
                        CreateSnapshotFromVolumeRecoveryPointError::ServiceUnavailableError(
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
impl fmt::Display for CreateSnapshotFromVolumeRecoveryPointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSnapshotFromVolumeRecoveryPointError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateSnapshotFromVolumeRecoveryPointError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateSnapshotFromVolumeRecoveryPointError::ServiceUnavailableError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateSnapshotFromVolumeRecoveryPointError {}
/// Errors returned by CreateStorediSCSIVolume
#[derive(Debug, PartialEq)]
pub enum CreateStorediSCSIVolumeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl CreateStorediSCSIVolumeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateStorediSCSIVolumeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(CreateStorediSCSIVolumeError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        CreateStorediSCSIVolumeError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateStorediSCSIVolumeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateStorediSCSIVolumeError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateStorediSCSIVolumeError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateStorediSCSIVolumeError {}
/// Errors returned by CreateTapePool
#[derive(Debug, PartialEq)]
pub enum CreateTapePoolError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl CreateTapePoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTapePoolError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(CreateTapePoolError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(CreateTapePoolError::InvalidGatewayRequest(
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
impl fmt::Display for CreateTapePoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTapePoolError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateTapePoolError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTapePoolError {}
/// Errors returned by CreateTapeWithBarcode
#[derive(Debug, PartialEq)]
pub enum CreateTapeWithBarcodeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl CreateTapeWithBarcodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTapeWithBarcodeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(CreateTapeWithBarcodeError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(CreateTapeWithBarcodeError::InvalidGatewayRequest(
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
impl fmt::Display for CreateTapeWithBarcodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTapeWithBarcodeError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateTapeWithBarcodeError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTapeWithBarcodeError {}
/// Errors returned by CreateTapes
#[derive(Debug, PartialEq)]
pub enum CreateTapesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl CreateTapesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTapesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(CreateTapesError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(CreateTapesError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTapesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTapesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateTapesError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTapesError {}
/// Errors returned by DeleteAutomaticTapeCreationPolicy
#[derive(Debug, PartialEq)]
pub enum DeleteAutomaticTapeCreationPolicyError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DeleteAutomaticTapeCreationPolicyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteAutomaticTapeCreationPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DeleteAutomaticTapeCreationPolicyError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DeleteAutomaticTapeCreationPolicyError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAutomaticTapeCreationPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAutomaticTapeCreationPolicyError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteAutomaticTapeCreationPolicyError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteAutomaticTapeCreationPolicyError {}
/// Errors returned by DeleteBandwidthRateLimit
#[derive(Debug, PartialEq)]
pub enum DeleteBandwidthRateLimitError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DeleteBandwidthRateLimitError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBandwidthRateLimitError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DeleteBandwidthRateLimitError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DeleteBandwidthRateLimitError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteBandwidthRateLimitError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteBandwidthRateLimitError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteBandwidthRateLimitError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteBandwidthRateLimitError {}
/// Errors returned by DeleteChapCredentials
#[derive(Debug, PartialEq)]
pub enum DeleteChapCredentialsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DeleteChapCredentialsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteChapCredentialsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeleteChapCredentialsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DeleteChapCredentialsError::InvalidGatewayRequest(
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
impl fmt::Display for DeleteChapCredentialsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteChapCredentialsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteChapCredentialsError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteChapCredentialsError {}
/// Errors returned by DeleteFileShare
#[derive(Debug, PartialEq)]
pub enum DeleteFileShareError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DeleteFileShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFileShareError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeleteFileShareError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DeleteFileShareError::InvalidGatewayRequest(
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
impl fmt::Display for DeleteFileShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFileShareError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteFileShareError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFileShareError {}
/// Errors returned by DeleteGateway
#[derive(Debug, PartialEq)]
pub enum DeleteGatewayError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DeleteGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGatewayError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeleteGatewayError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DeleteGatewayError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGatewayError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteGatewayError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteGatewayError {}
/// Errors returned by DeleteSnapshotSchedule
#[derive(Debug, PartialEq)]
pub enum DeleteSnapshotScheduleError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DeleteSnapshotScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSnapshotScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeleteSnapshotScheduleError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DeleteSnapshotScheduleError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteSnapshotScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSnapshotScheduleError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteSnapshotScheduleError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSnapshotScheduleError {}
/// Errors returned by DeleteTape
#[derive(Debug, PartialEq)]
pub enum DeleteTapeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DeleteTapeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTapeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeleteTapeError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DeleteTapeError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTapeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTapeError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteTapeError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTapeError {}
/// Errors returned by DeleteTapeArchive
#[derive(Debug, PartialEq)]
pub enum DeleteTapeArchiveError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DeleteTapeArchiveError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTapeArchiveError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeleteTapeArchiveError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DeleteTapeArchiveError::InvalidGatewayRequest(
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
impl fmt::Display for DeleteTapeArchiveError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTapeArchiveError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteTapeArchiveError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTapeArchiveError {}
/// Errors returned by DeleteTapePool
#[derive(Debug, PartialEq)]
pub enum DeleteTapePoolError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DeleteTapePoolError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTapePoolError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeleteTapePoolError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DeleteTapePoolError::InvalidGatewayRequest(
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
impl fmt::Display for DeleteTapePoolError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTapePoolError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteTapePoolError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTapePoolError {}
/// Errors returned by DeleteVolume
#[derive(Debug, PartialEq)]
pub enum DeleteVolumeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DeleteVolumeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVolumeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DeleteVolumeError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DeleteVolumeError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVolumeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVolumeError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteVolumeError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVolumeError {}
/// Errors returned by DescribeAvailabilityMonitorTest
#[derive(Debug, PartialEq)]
pub enum DescribeAvailabilityMonitorTestError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeAvailabilityMonitorTestError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAvailabilityMonitorTestError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeAvailabilityMonitorTestError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DescribeAvailabilityMonitorTestError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAvailabilityMonitorTestError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAvailabilityMonitorTestError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeAvailabilityMonitorTestError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeAvailabilityMonitorTestError {}
/// Errors returned by DescribeBandwidthRateLimit
#[derive(Debug, PartialEq)]
pub enum DescribeBandwidthRateLimitError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeBandwidthRateLimitError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeBandwidthRateLimitError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeBandwidthRateLimitError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DescribeBandwidthRateLimitError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeBandwidthRateLimitError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeBandwidthRateLimitError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeBandwidthRateLimitError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeBandwidthRateLimitError {}
/// Errors returned by DescribeBandwidthRateLimitSchedule
#[derive(Debug, PartialEq)]
pub enum DescribeBandwidthRateLimitScheduleError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeBandwidthRateLimitScheduleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeBandwidthRateLimitScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeBandwidthRateLimitScheduleError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DescribeBandwidthRateLimitScheduleError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeBandwidthRateLimitScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeBandwidthRateLimitScheduleError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeBandwidthRateLimitScheduleError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeBandwidthRateLimitScheduleError {}
/// Errors returned by DescribeCache
#[derive(Debug, PartialEq)]
pub enum DescribeCacheError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeCacheError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCacheError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeCacheError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DescribeCacheError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCacheError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCacheError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeCacheError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCacheError {}
/// Errors returned by DescribeCachediSCSIVolumes
#[derive(Debug, PartialEq)]
pub enum DescribeCachediSCSIVolumesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeCachediSCSIVolumesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeCachediSCSIVolumesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeCachediSCSIVolumesError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DescribeCachediSCSIVolumesError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeCachediSCSIVolumesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCachediSCSIVolumesError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCachediSCSIVolumesError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeCachediSCSIVolumesError {}
/// Errors returned by DescribeChapCredentials
#[derive(Debug, PartialEq)]
pub enum DescribeChapCredentialsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeChapCredentialsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeChapCredentialsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeChapCredentialsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DescribeChapCredentialsError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeChapCredentialsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeChapCredentialsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeChapCredentialsError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeChapCredentialsError {}
/// Errors returned by DescribeFileSystemAssociations
#[derive(Debug, PartialEq)]
pub enum DescribeFileSystemAssociationsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeFileSystemAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeFileSystemAssociationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeFileSystemAssociationsError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DescribeFileSystemAssociationsError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeFileSystemAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeFileSystemAssociationsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeFileSystemAssociationsError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeFileSystemAssociationsError {}
/// Errors returned by DescribeGatewayInformation
#[derive(Debug, PartialEq)]
pub enum DescribeGatewayInformationError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeGatewayInformationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeGatewayInformationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeGatewayInformationError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DescribeGatewayInformationError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeGatewayInformationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGatewayInformationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeGatewayInformationError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeGatewayInformationError {}
/// Errors returned by DescribeMaintenanceStartTime
#[derive(Debug, PartialEq)]
pub enum DescribeMaintenanceStartTimeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeMaintenanceStartTimeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeMaintenanceStartTimeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeMaintenanceStartTimeError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DescribeMaintenanceStartTimeError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeMaintenanceStartTimeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMaintenanceStartTimeError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeMaintenanceStartTimeError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeMaintenanceStartTimeError {}
/// Errors returned by DescribeNFSFileShares
#[derive(Debug, PartialEq)]
pub enum DescribeNFSFileSharesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeNFSFileSharesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeNFSFileSharesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeNFSFileSharesError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DescribeNFSFileSharesError::InvalidGatewayRequest(
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
impl fmt::Display for DescribeNFSFileSharesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeNFSFileSharesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeNFSFileSharesError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeNFSFileSharesError {}
/// Errors returned by DescribeSMBFileShares
#[derive(Debug, PartialEq)]
pub enum DescribeSMBFileSharesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeSMBFileSharesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSMBFileSharesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeSMBFileSharesError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DescribeSMBFileSharesError::InvalidGatewayRequest(
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
impl fmt::Display for DescribeSMBFileSharesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSMBFileSharesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeSMBFileSharesError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSMBFileSharesError {}
/// Errors returned by DescribeSMBSettings
#[derive(Debug, PartialEq)]
pub enum DescribeSMBSettingsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeSMBSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSMBSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeSMBSettingsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DescribeSMBSettingsError::InvalidGatewayRequest(
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
impl fmt::Display for DescribeSMBSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSMBSettingsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeSMBSettingsError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSMBSettingsError {}
/// Errors returned by DescribeSnapshotSchedule
#[derive(Debug, PartialEq)]
pub enum DescribeSnapshotScheduleError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeSnapshotScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSnapshotScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeSnapshotScheduleError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DescribeSnapshotScheduleError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeSnapshotScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSnapshotScheduleError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeSnapshotScheduleError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeSnapshotScheduleError {}
/// Errors returned by DescribeStorediSCSIVolumes
#[derive(Debug, PartialEq)]
pub enum DescribeStorediSCSIVolumesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeStorediSCSIVolumesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeStorediSCSIVolumesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeStorediSCSIVolumesError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DescribeStorediSCSIVolumesError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeStorediSCSIVolumesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeStorediSCSIVolumesError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeStorediSCSIVolumesError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeStorediSCSIVolumesError {}
/// Errors returned by DescribeTapeArchives
#[derive(Debug, PartialEq)]
pub enum DescribeTapeArchivesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeTapeArchivesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTapeArchivesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeTapeArchivesError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DescribeTapeArchivesError::InvalidGatewayRequest(
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
impl fmt::Display for DescribeTapeArchivesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTapeArchivesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeTapeArchivesError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTapeArchivesError {}
/// Errors returned by DescribeTapeRecoveryPoints
#[derive(Debug, PartialEq)]
pub enum DescribeTapeRecoveryPointsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeTapeRecoveryPointsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeTapeRecoveryPointsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        DescribeTapeRecoveryPointsError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DescribeTapeRecoveryPointsError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTapeRecoveryPointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTapeRecoveryPointsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeTapeRecoveryPointsError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeTapeRecoveryPointsError {}
/// Errors returned by DescribeTapes
#[derive(Debug, PartialEq)]
pub enum DescribeTapesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeTapesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTapesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeTapesError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DescribeTapesError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTapesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTapesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeTapesError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTapesError {}
/// Errors returned by DescribeUploadBuffer
#[derive(Debug, PartialEq)]
pub enum DescribeUploadBufferError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeUploadBufferError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUploadBufferError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeUploadBufferError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DescribeUploadBufferError::InvalidGatewayRequest(
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
impl fmt::Display for DescribeUploadBufferError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUploadBufferError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeUploadBufferError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUploadBufferError {}
/// Errors returned by DescribeVTLDevices
#[derive(Debug, PartialEq)]
pub enum DescribeVTLDevicesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeVTLDevicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVTLDevicesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeVTLDevicesError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DescribeVTLDevicesError::InvalidGatewayRequest(
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
impl fmt::Display for DescribeVTLDevicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeVTLDevicesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeVTLDevicesError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeVTLDevicesError {}
/// Errors returned by DescribeWorkingStorage
#[derive(Debug, PartialEq)]
pub enum DescribeWorkingStorageError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DescribeWorkingStorageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeWorkingStorageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DescribeWorkingStorageError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DescribeWorkingStorageError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeWorkingStorageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeWorkingStorageError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeWorkingStorageError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeWorkingStorageError {}
/// Errors returned by DetachVolume
#[derive(Debug, PartialEq)]
pub enum DetachVolumeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DetachVolumeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetachVolumeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DetachVolumeError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DetachVolumeError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DetachVolumeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DetachVolumeError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DetachVolumeError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DetachVolumeError {}
/// Errors returned by DisableGateway
#[derive(Debug, PartialEq)]
pub enum DisableGatewayError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DisableGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableGatewayError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DisableGatewayError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(DisableGatewayError::InvalidGatewayRequest(
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
impl fmt::Display for DisableGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableGatewayError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DisableGatewayError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableGatewayError {}
/// Errors returned by DisassociateFileSystem
#[derive(Debug, PartialEq)]
pub enum DisassociateFileSystemError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl DisassociateFileSystemError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateFileSystemError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(DisassociateFileSystemError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        DisassociateFileSystemError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateFileSystemError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateFileSystemError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DisassociateFileSystemError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateFileSystemError {}
/// Errors returned by JoinDomain
#[derive(Debug, PartialEq)]
pub enum JoinDomainError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl JoinDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<JoinDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(JoinDomainError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(JoinDomainError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for JoinDomainError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JoinDomainError::InternalServerError(ref cause) => write!(f, "{}", cause),
            JoinDomainError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for JoinDomainError {}
/// Errors returned by ListAutomaticTapeCreationPolicies
#[derive(Debug, PartialEq)]
pub enum ListAutomaticTapeCreationPoliciesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl ListAutomaticTapeCreationPoliciesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAutomaticTapeCreationPoliciesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        ListAutomaticTapeCreationPoliciesError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        ListAutomaticTapeCreationPoliciesError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAutomaticTapeCreationPoliciesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAutomaticTapeCreationPoliciesError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAutomaticTapeCreationPoliciesError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListAutomaticTapeCreationPoliciesError {}
/// Errors returned by ListFileShares
#[derive(Debug, PartialEq)]
pub enum ListFileSharesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl ListFileSharesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFileSharesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListFileSharesError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(ListFileSharesError::InvalidGatewayRequest(
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
impl fmt::Display for ListFileSharesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFileSharesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListFileSharesError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFileSharesError {}
/// Errors returned by ListFileSystemAssociations
#[derive(Debug, PartialEq)]
pub enum ListFileSystemAssociationsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl ListFileSystemAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListFileSystemAssociationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        ListFileSystemAssociationsError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        ListFileSystemAssociationsError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFileSystemAssociationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFileSystemAssociationsError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            ListFileSystemAssociationsError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListFileSystemAssociationsError {}
/// Errors returned by ListGateways
#[derive(Debug, PartialEq)]
pub enum ListGatewaysError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl ListGatewaysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGatewaysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListGatewaysError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(ListGatewaysError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListGatewaysError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGatewaysError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListGatewaysError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGatewaysError {}
/// Errors returned by ListLocalDisks
#[derive(Debug, PartialEq)]
pub enum ListLocalDisksError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl ListLocalDisksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLocalDisksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListLocalDisksError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(ListLocalDisksError::InvalidGatewayRequest(
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
impl fmt::Display for ListLocalDisksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLocalDisksError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListLocalDisksError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLocalDisksError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidGatewayRequest(
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
            ListTagsForResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListTapePools
#[derive(Debug, PartialEq)]
pub enum ListTapePoolsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl ListTapePoolsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTapePoolsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListTapePoolsError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(ListTapePoolsError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTapePoolsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTapePoolsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListTapePoolsError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTapePoolsError {}
/// Errors returned by ListTapes
#[derive(Debug, PartialEq)]
pub enum ListTapesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl ListTapesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTapesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListTapesError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(ListTapesError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTapesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTapesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListTapesError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTapesError {}
/// Errors returned by ListVolumeInitiators
#[derive(Debug, PartialEq)]
pub enum ListVolumeInitiatorsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl ListVolumeInitiatorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVolumeInitiatorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListVolumeInitiatorsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(ListVolumeInitiatorsError::InvalidGatewayRequest(
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
impl fmt::Display for ListVolumeInitiatorsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVolumeInitiatorsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListVolumeInitiatorsError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVolumeInitiatorsError {}
/// Errors returned by ListVolumeRecoveryPoints
#[derive(Debug, PartialEq)]
pub enum ListVolumeRecoveryPointsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl ListVolumeRecoveryPointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVolumeRecoveryPointsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        ListVolumeRecoveryPointsError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        ListVolumeRecoveryPointsError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListVolumeRecoveryPointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVolumeRecoveryPointsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListVolumeRecoveryPointsError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListVolumeRecoveryPointsError {}
/// Errors returned by ListVolumes
#[derive(Debug, PartialEq)]
pub enum ListVolumesError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl ListVolumesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVolumesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ListVolumesError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(ListVolumesError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListVolumesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVolumesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListVolumesError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVolumesError {}
/// Errors returned by NotifyWhenUploaded
#[derive(Debug, PartialEq)]
pub enum NotifyWhenUploadedError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl NotifyWhenUploadedError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<NotifyWhenUploadedError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(NotifyWhenUploadedError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(NotifyWhenUploadedError::InvalidGatewayRequest(
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
impl fmt::Display for NotifyWhenUploadedError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NotifyWhenUploadedError::InternalServerError(ref cause) => write!(f, "{}", cause),
            NotifyWhenUploadedError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for NotifyWhenUploadedError {}
/// Errors returned by RefreshCache
#[derive(Debug, PartialEq)]
pub enum RefreshCacheError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl RefreshCacheError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RefreshCacheError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(RefreshCacheError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(RefreshCacheError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RefreshCacheError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RefreshCacheError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RefreshCacheError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RefreshCacheError {}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl RemoveTagsFromResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsFromResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(RemoveTagsFromResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        RemoveTagsFromResourceError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveTagsFromResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsFromResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromResourceError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTagsFromResourceError {}
/// Errors returned by ResetCache
#[derive(Debug, PartialEq)]
pub enum ResetCacheError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl ResetCacheError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResetCacheError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ResetCacheError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(ResetCacheError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ResetCacheError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResetCacheError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ResetCacheError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ResetCacheError {}
/// Errors returned by RetrieveTapeArchive
#[derive(Debug, PartialEq)]
pub enum RetrieveTapeArchiveError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl RetrieveTapeArchiveError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RetrieveTapeArchiveError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(RetrieveTapeArchiveError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(RetrieveTapeArchiveError::InvalidGatewayRequest(
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
impl fmt::Display for RetrieveTapeArchiveError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RetrieveTapeArchiveError::InternalServerError(ref cause) => write!(f, "{}", cause),
            RetrieveTapeArchiveError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RetrieveTapeArchiveError {}
/// Errors returned by RetrieveTapeRecoveryPoint
#[derive(Debug, PartialEq)]
pub enum RetrieveTapeRecoveryPointError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl RetrieveTapeRecoveryPointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RetrieveTapeRecoveryPointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        RetrieveTapeRecoveryPointError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        RetrieveTapeRecoveryPointError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RetrieveTapeRecoveryPointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RetrieveTapeRecoveryPointError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            RetrieveTapeRecoveryPointError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RetrieveTapeRecoveryPointError {}
/// Errors returned by SetLocalConsolePassword
#[derive(Debug, PartialEq)]
pub enum SetLocalConsolePasswordError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl SetLocalConsolePasswordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetLocalConsolePasswordError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(SetLocalConsolePasswordError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        SetLocalConsolePasswordError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for SetLocalConsolePasswordError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetLocalConsolePasswordError::InternalServerError(ref cause) => write!(f, "{}", cause),
            SetLocalConsolePasswordError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for SetLocalConsolePasswordError {}
/// Errors returned by SetSMBGuestPassword
#[derive(Debug, PartialEq)]
pub enum SetSMBGuestPasswordError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl SetSMBGuestPasswordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetSMBGuestPasswordError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(SetSMBGuestPasswordError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(SetSMBGuestPasswordError::InvalidGatewayRequest(
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
impl fmt::Display for SetSMBGuestPasswordError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetSMBGuestPasswordError::InternalServerError(ref cause) => write!(f, "{}", cause),
            SetSMBGuestPasswordError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetSMBGuestPasswordError {}
/// Errors returned by ShutdownGateway
#[derive(Debug, PartialEq)]
pub enum ShutdownGatewayError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl ShutdownGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ShutdownGatewayError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(ShutdownGatewayError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(ShutdownGatewayError::InvalidGatewayRequest(
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
impl fmt::Display for ShutdownGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShutdownGatewayError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ShutdownGatewayError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ShutdownGatewayError {}
/// Errors returned by StartAvailabilityMonitorTest
#[derive(Debug, PartialEq)]
pub enum StartAvailabilityMonitorTestError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl StartAvailabilityMonitorTestError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartAvailabilityMonitorTestError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        StartAvailabilityMonitorTestError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        StartAvailabilityMonitorTestError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartAvailabilityMonitorTestError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartAvailabilityMonitorTestError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            StartAvailabilityMonitorTestError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartAvailabilityMonitorTestError {}
/// Errors returned by StartGateway
#[derive(Debug, PartialEq)]
pub enum StartGatewayError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl StartGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartGatewayError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(StartGatewayError::InternalServerError(err.msg))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(StartGatewayError::InvalidGatewayRequest(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartGatewayError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartGatewayError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StartGatewayError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartGatewayError {}
/// Errors returned by UpdateAutomaticTapeCreationPolicy
#[derive(Debug, PartialEq)]
pub enum UpdateAutomaticTapeCreationPolicyError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl UpdateAutomaticTapeCreationPolicyError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateAutomaticTapeCreationPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        UpdateAutomaticTapeCreationPolicyError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        UpdateAutomaticTapeCreationPolicyError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAutomaticTapeCreationPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAutomaticTapeCreationPolicyError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateAutomaticTapeCreationPolicyError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateAutomaticTapeCreationPolicyError {}
/// Errors returned by UpdateBandwidthRateLimit
#[derive(Debug, PartialEq)]
pub enum UpdateBandwidthRateLimitError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl UpdateBandwidthRateLimitError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateBandwidthRateLimitError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        UpdateBandwidthRateLimitError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        UpdateBandwidthRateLimitError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateBandwidthRateLimitError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateBandwidthRateLimitError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateBandwidthRateLimitError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateBandwidthRateLimitError {}
/// Errors returned by UpdateBandwidthRateLimitSchedule
#[derive(Debug, PartialEq)]
pub enum UpdateBandwidthRateLimitScheduleError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl UpdateBandwidthRateLimitScheduleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateBandwidthRateLimitScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        UpdateBandwidthRateLimitScheduleError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        UpdateBandwidthRateLimitScheduleError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateBandwidthRateLimitScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateBandwidthRateLimitScheduleError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateBandwidthRateLimitScheduleError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateBandwidthRateLimitScheduleError {}
/// Errors returned by UpdateChapCredentials
#[derive(Debug, PartialEq)]
pub enum UpdateChapCredentialsError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl UpdateChapCredentialsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateChapCredentialsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(UpdateChapCredentialsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(UpdateChapCredentialsError::InvalidGatewayRequest(
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
impl fmt::Display for UpdateChapCredentialsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateChapCredentialsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateChapCredentialsError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateChapCredentialsError {}
/// Errors returned by UpdateFileSystemAssociation
#[derive(Debug, PartialEq)]
pub enum UpdateFileSystemAssociationError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl UpdateFileSystemAssociationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateFileSystemAssociationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        UpdateFileSystemAssociationError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        UpdateFileSystemAssociationError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFileSystemAssociationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFileSystemAssociationError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateFileSystemAssociationError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateFileSystemAssociationError {}
/// Errors returned by UpdateGatewayInformation
#[derive(Debug, PartialEq)]
pub enum UpdateGatewayInformationError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl UpdateGatewayInformationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGatewayInformationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        UpdateGatewayInformationError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        UpdateGatewayInformationError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateGatewayInformationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGatewayInformationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateGatewayInformationError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateGatewayInformationError {}
/// Errors returned by UpdateGatewaySoftwareNow
#[derive(Debug, PartialEq)]
pub enum UpdateGatewaySoftwareNowError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl UpdateGatewaySoftwareNowError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGatewaySoftwareNowError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        UpdateGatewaySoftwareNowError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        UpdateGatewaySoftwareNowError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateGatewaySoftwareNowError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateGatewaySoftwareNowError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateGatewaySoftwareNowError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateGatewaySoftwareNowError {}
/// Errors returned by UpdateMaintenanceStartTime
#[derive(Debug, PartialEq)]
pub enum UpdateMaintenanceStartTimeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl UpdateMaintenanceStartTimeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateMaintenanceStartTimeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        UpdateMaintenanceStartTimeError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        UpdateMaintenanceStartTimeError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateMaintenanceStartTimeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateMaintenanceStartTimeError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateMaintenanceStartTimeError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateMaintenanceStartTimeError {}
/// Errors returned by UpdateNFSFileShare
#[derive(Debug, PartialEq)]
pub enum UpdateNFSFileShareError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl UpdateNFSFileShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateNFSFileShareError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(UpdateNFSFileShareError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(UpdateNFSFileShareError::InvalidGatewayRequest(
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
impl fmt::Display for UpdateNFSFileShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateNFSFileShareError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateNFSFileShareError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateNFSFileShareError {}
/// Errors returned by UpdateSMBFileShare
#[derive(Debug, PartialEq)]
pub enum UpdateSMBFileShareError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl UpdateSMBFileShareError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSMBFileShareError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(UpdateSMBFileShareError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(UpdateSMBFileShareError::InvalidGatewayRequest(
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
impl fmt::Display for UpdateSMBFileShareError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSMBFileShareError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateSMBFileShareError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSMBFileShareError {}
/// Errors returned by UpdateSMBFileShareVisibility
#[derive(Debug, PartialEq)]
pub enum UpdateSMBFileShareVisibilityError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl UpdateSMBFileShareVisibilityError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateSMBFileShareVisibilityError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        UpdateSMBFileShareVisibilityError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        UpdateSMBFileShareVisibilityError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateSMBFileShareVisibilityError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSMBFileShareVisibilityError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateSMBFileShareVisibilityError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateSMBFileShareVisibilityError {}
/// Errors returned by UpdateSMBSecurityStrategy
#[derive(Debug, PartialEq)]
pub enum UpdateSMBSecurityStrategyError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl UpdateSMBSecurityStrategyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSMBSecurityStrategyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(
                        UpdateSMBSecurityStrategyError::InternalServerError(err.msg),
                    )
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        UpdateSMBSecurityStrategyError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateSMBSecurityStrategyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSMBSecurityStrategyError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateSMBSecurityStrategyError::InvalidGatewayRequest(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateSMBSecurityStrategyError {}
/// Errors returned by UpdateSnapshotSchedule
#[derive(Debug, PartialEq)]
pub enum UpdateSnapshotScheduleError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl UpdateSnapshotScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateSnapshotScheduleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(UpdateSnapshotScheduleError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(
                        UpdateSnapshotScheduleError::InvalidGatewayRequest(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateSnapshotScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateSnapshotScheduleError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateSnapshotScheduleError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateSnapshotScheduleError {}
/// Errors returned by UpdateVTLDeviceType
#[derive(Debug, PartialEq)]
pub enum UpdateVTLDeviceTypeError {
    /// <p>An internal server error has occurred during the request. For more information, see the error and message fields.</p>
    InternalServerError(String),
    /// <p>An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields.</p>
    InvalidGatewayRequest(String),
}

impl UpdateVTLDeviceTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVTLDeviceTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServerError" => {
                    return RusotoError::Service(UpdateVTLDeviceTypeError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidGatewayRequestException" => {
                    return RusotoError::Service(UpdateVTLDeviceTypeError::InvalidGatewayRequest(
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
impl fmt::Display for UpdateVTLDeviceTypeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateVTLDeviceTypeError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateVTLDeviceTypeError::InvalidGatewayRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateVTLDeviceTypeError {}
/// Trait representing the capabilities of the AWS Storage Gateway API. AWS Storage Gateway clients implement this trait.
#[async_trait]
pub trait StorageGateway {
    /// <p><p>Activates the gateway you previously deployed on your host. In the activation process, you specify information such as the AWS Region that you want to use for storing snapshots or tapes, the time zone for scheduled snapshots the gateway snapshot schedule window, an activation key, and a name for your gateway. The activation process also associates your gateway with your account. For more information, see <a>UpdateGatewayInformation</a>.</p> <note> <p>You must turn on the gateway VM before you can activate your gateway.</p> </note></p>
    async fn activate_gateway(
        &self,
        input: ActivateGatewayInput,
    ) -> Result<ActivateGatewayOutput, RusotoError<ActivateGatewayError>>;

    /// <p>Configures one or more gateway local disks as cache for a gateway. This operation is only supported in the cached volume, tape, and file gateway type (see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/StorageGatewayConcepts.html">How AWS Storage Gateway works (architecture)</a>.</p> <p>In the request, you specify the gateway Amazon Resource Name (ARN) to which you want to add cache, and one or more disk IDs that you want to configure as cache.</p>
    async fn add_cache(
        &self,
        input: AddCacheInput,
    ) -> Result<AddCacheOutput, RusotoError<AddCacheError>>;

    /// <p>Adds one or more tags to the specified resource. You use tags to add metadata to resources, which you can use to categorize these resources. For example, you can categorize resources by purpose, owner, environment, or team. Each tag consists of a key and a value, which you define. You can add tags to the following AWS Storage Gateway resources:</p> <ul> <li> <p>Storage gateways of all types</p> </li> <li> <p>Storage volumes</p> </li> <li> <p>Virtual tapes</p> </li> <li> <p>NFS and SMB file shares</p> </li> </ul> <p>You can create a maximum of 50 tags for each resource. Virtual tapes and storage volumes that are recovered to a new gateway maintain their tags.</p>
    async fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceInput,
    ) -> Result<AddTagsToResourceOutput, RusotoError<AddTagsToResourceError>>;

    /// <p>Configures one or more gateway local disks as upload buffer for a specified gateway. This operation is supported for the stored volume, cached volume, and tape gateway types.</p> <p>In the request, you specify the gateway Amazon Resource Name (ARN) to which you want to add upload buffer, and one or more disk IDs that you want to configure as upload buffer.</p>
    async fn add_upload_buffer(
        &self,
        input: AddUploadBufferInput,
    ) -> Result<AddUploadBufferOutput, RusotoError<AddUploadBufferError>>;

    /// <p>Configures one or more gateway local disks as working storage for a gateway. This operation is only supported in the stored volume gateway type. This operation is deprecated in cached volume API version 20120630. Use <a>AddUploadBuffer</a> instead.</p> <note> <p>Working storage is also referred to as upload buffer. You can also use the <a>AddUploadBuffer</a> operation to add upload buffer to a stored volume gateway.</p> </note> <p>In the request, you specify the gateway Amazon Resource Name (ARN) to which you want to add working storage, and one or more disk IDs that you want to configure as working storage.</p>
    async fn add_working_storage(
        &self,
        input: AddWorkingStorageInput,
    ) -> Result<AddWorkingStorageOutput, RusotoError<AddWorkingStorageError>>;

    /// <p>Assigns a tape to a tape pool for archiving. The tape assigned to a pool is archived in the S3 storage class that is associated with the pool. When you use your backup application to eject the tape, the tape is archived directly into the S3 storage class (S3 Glacier or S3 Glacier Deep Archive) that corresponds to the pool.</p> <p>Valid Values: <code>GLACIER</code> | <code>DEEP_ARCHIVE</code> </p>
    async fn assign_tape_pool(
        &self,
        input: AssignTapePoolInput,
    ) -> Result<AssignTapePoolOutput, RusotoError<AssignTapePoolError>>;

    /// <p>Associate an Amazon FSx file system with the Amazon FSx file gateway. After the association process is complete, the file shares on the Amazon FSx file system are available for access through the gateway. This operation only supports the Amazon FSx file gateway type.</p>
    async fn associate_file_system(
        &self,
        input: AssociateFileSystemInput,
    ) -> Result<AssociateFileSystemOutput, RusotoError<AssociateFileSystemError>>;

    /// <p>Connects a volume to an iSCSI connection and then attaches the volume to the specified gateway. Detaching and attaching a volume enables you to recover your data from one gateway to a different gateway without creating a snapshot. It also makes it easier to move your volumes from an on-premises gateway to a gateway hosted on an Amazon EC2 instance.</p>
    async fn attach_volume(
        &self,
        input: AttachVolumeInput,
    ) -> Result<AttachVolumeOutput, RusotoError<AttachVolumeError>>;

    /// <p>Cancels archiving of a virtual tape to the virtual tape shelf (VTS) after the archiving process is initiated. This operation is only supported in the tape gateway type.</p>
    async fn cancel_archival(
        &self,
        input: CancelArchivalInput,
    ) -> Result<CancelArchivalOutput, RusotoError<CancelArchivalError>>;

    /// <p>Cancels retrieval of a virtual tape from the virtual tape shelf (VTS) to a gateway after the retrieval process is initiated. The virtual tape is returned to the VTS. This operation is only supported in the tape gateway type.</p>
    async fn cancel_retrieval(
        &self,
        input: CancelRetrievalInput,
    ) -> Result<CancelRetrievalOutput, RusotoError<CancelRetrievalError>>;

    /// <p>Creates a cached volume on a specified cached volume gateway. This operation is only supported in the cached volume gateway type.</p> <note> <p>Cache storage must be allocated to the gateway before you can create a cached volume. Use the <a>AddCache</a> operation to add cache storage to a gateway.</p> </note> <p>In the request, you must specify the gateway, size of the volume in bytes, the iSCSI target name, an IP address on which to expose the target, and a unique client token. In response, the gateway creates the volume and returns information about it. This information includes the volume Amazon Resource Name (ARN), its size, and the iSCSI target ARN that initiators can use to connect to the volume target.</p> <p>Optionally, you can provide the ARN for an existing volume as the <code>SourceVolumeARN</code> for this cached volume, which creates an exact copy of the existing volume’s latest recovery point. The <code>VolumeSizeInBytes</code> value must be equal to or larger than the size of the copied volume, in bytes.</p>
    async fn create_cachedi_scsi_volume(
        &self,
        input: CreateCachediSCSIVolumeInput,
    ) -> Result<CreateCachediSCSIVolumeOutput, RusotoError<CreateCachediSCSIVolumeError>>;

    /// <p><p>Creates a Network File System (NFS) file share on an existing file gateway. In Storage Gateway, a file share is a file system mount point backed by Amazon S3 cloud storage. Storage Gateway exposes file shares using an NFS interface. This operation is only supported for file gateways.</p> <important> <p>File gateway requires AWS Security Token Service (AWS STS) to be activated to enable you to create a file share. Make sure AWS STS is activated in the AWS Region you are creating your file gateway in. If AWS STS is not activated in the AWS Region, activate it. For information about how to activate AWS STS, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and deactivating AWS STS in an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> <p>File gateway does not support creating hard or symbolic links on a file share.</p> </important></p>
    async fn create_nfs_file_share(
        &self,
        input: CreateNFSFileShareInput,
    ) -> Result<CreateNFSFileShareOutput, RusotoError<CreateNFSFileShareError>>;

    /// <p><p>Creates a Server Message Block (SMB) file share on an existing file gateway. In Storage Gateway, a file share is a file system mount point backed by Amazon S3 cloud storage. Storage Gateway exposes file shares using an SMB interface. This operation is only supported for file gateways.</p> <important> <p>File gateways require AWS Security Token Service (AWS STS) to be activated to enable you to create a file share. Make sure that AWS STS is activated in the AWS Region you are creating your file gateway in. If AWS STS is not activated in this AWS Region, activate it. For information about how to activate AWS STS, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and deactivating AWS STS in an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> <p>File gateways don&#39;t support creating hard or symbolic links on a file share.</p> </important></p>
    async fn create_smb_file_share(
        &self,
        input: CreateSMBFileShareInput,
    ) -> Result<CreateSMBFileShareOutput, RusotoError<CreateSMBFileShareError>>;

    /// <p><p>Initiates a snapshot of a volume.</p> <p>AWS Storage Gateway provides the ability to back up point-in-time snapshots of your data to Amazon Simple Storage (Amazon S3) for durable off-site recovery, and also import the data to an Amazon Elastic Block Store (EBS) volume in Amazon Elastic Compute Cloud (EC2). You can take snapshots of your gateway volume on a scheduled or ad hoc basis. This API enables you to take an ad hoc snapshot. For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/managing-volumes.html#SchedulingSnapshot">Editing a snapshot schedule</a>.</p> <p>In the <code>CreateSnapshot</code> request, you identify the volume by providing its Amazon Resource Name (ARN). You must also provide description for the snapshot. When AWS Storage Gateway takes the snapshot of specified volume, the snapshot and description appears in the AWS Storage Gateway console. In response, AWS Storage Gateway returns you a snapshot ID. You can use this snapshot ID to check the snapshot progress or later use it when you want to create a volume from a snapshot. This operation is only supported in stored and cached volume gateway type.</p> <note> <p>To list or delete a snapshot, you must use the Amazon EC2 API. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSnapshots.html">DescribeSnapshots</a> or <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DeleteSnapshot.html">DeleteSnapshot</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </note> <important> <p>Volume and snapshot IDs are changing to a longer length ID format. For more information, see the important note on the <a href="https://docs.aws.amazon.com/storagegateway/latest/APIReference/Welcome.html">Welcome</a> page.</p> </important></p>
    async fn create_snapshot(
        &self,
        input: CreateSnapshotInput,
    ) -> Result<CreateSnapshotOutput, RusotoError<CreateSnapshotError>>;

    /// <p><p>Initiates a snapshot of a gateway from a volume recovery point. This operation is only supported in the cached volume gateway type.</p> <p>A volume recovery point is a point in time at which all data of the volume is consistent and from which you can create a snapshot. To get a list of volume recovery point for cached volume gateway, use <a>ListVolumeRecoveryPoints</a>.</p> <p>In the <code>CreateSnapshotFromVolumeRecoveryPoint</code> request, you identify the volume by providing its Amazon Resource Name (ARN). You must also provide a description for the snapshot. When the gateway takes a snapshot of the specified volume, the snapshot and its description appear in the AWS Storage Gateway console. In response, the gateway returns you a snapshot ID. You can use this snapshot ID to check the snapshot progress or later use it when you want to create a volume from a snapshot.</p> <note> <p>To list or delete a snapshot, you must use the Amazon EC2 API. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSnapshots.html">DescribeSnapshots</a> or <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DeleteSnapshot.html">DeleteSnapshot</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </note></p>
    async fn create_snapshot_from_volume_recovery_point(
        &self,
        input: CreateSnapshotFromVolumeRecoveryPointInput,
    ) -> Result<
        CreateSnapshotFromVolumeRecoveryPointOutput,
        RusotoError<CreateSnapshotFromVolumeRecoveryPointError>,
    >;

    /// <p>Creates a volume on a specified gateway. This operation is only supported in the stored volume gateway type.</p> <p>The size of the volume to create is inferred from the disk size. You can choose to preserve existing data on the disk, create volume from an existing snapshot, or create an empty volume. If you choose to create an empty gateway volume, then any existing data on the disk is erased.</p> <p>In the request, you must specify the gateway and the disk information on which you are creating the volume. In response, the gateway creates the volume and returns volume information such as the volume Amazon Resource Name (ARN), its size, and the iSCSI target ARN that initiators can use to connect to the volume target.</p>
    async fn create_storedi_scsi_volume(
        &self,
        input: CreateStorediSCSIVolumeInput,
    ) -> Result<CreateStorediSCSIVolumeOutput, RusotoError<CreateStorediSCSIVolumeError>>;

    /// <p>Creates a new custom tape pool. You can use custom tape pool to enable tape retention lock on tapes that are archived in the custom pool.</p>
    async fn create_tape_pool(
        &self,
        input: CreateTapePoolInput,
    ) -> Result<CreateTapePoolOutput, RusotoError<CreateTapePoolError>>;

    /// <p><p>Creates a virtual tape by using your own barcode. You write data to the virtual tape and then archive the tape. A barcode is unique and cannot be reused if it has already been used on a tape. This applies to barcodes used on deleted tapes. This operation is only supported in the tape gateway type.</p> <note> <p>Cache storage must be allocated to the gateway before you can create a virtual tape. Use the <a>AddCache</a> operation to add cache storage to a gateway.</p> </note></p>
    async fn create_tape_with_barcode(
        &self,
        input: CreateTapeWithBarcodeInput,
    ) -> Result<CreateTapeWithBarcodeOutput, RusotoError<CreateTapeWithBarcodeError>>;

    /// <p><p>Creates one or more virtual tapes. You write data to the virtual tapes and then archive the tapes. This operation is only supported in the tape gateway type.</p> <note> <p>Cache storage must be allocated to the gateway before you can create virtual tapes. Use the <a>AddCache</a> operation to add cache storage to a gateway.</p> </note></p>
    async fn create_tapes(
        &self,
        input: CreateTapesInput,
    ) -> Result<CreateTapesOutput, RusotoError<CreateTapesError>>;

    /// <p>Deletes the automatic tape creation policy of a gateway. If you delete this policy, new virtual tapes must be created manually. Use the Amazon Resource Name (ARN) of the gateway in your request to remove the policy.</p>
    async fn delete_automatic_tape_creation_policy(
        &self,
        input: DeleteAutomaticTapeCreationPolicyInput,
    ) -> Result<
        DeleteAutomaticTapeCreationPolicyOutput,
        RusotoError<DeleteAutomaticTapeCreationPolicyError>,
    >;

    /// <p>Deletes the bandwidth rate limits of a gateway. You can delete either the upload and download bandwidth rate limit, or you can delete both. If you delete only one of the limits, the other limit remains unchanged. To specify which gateway to work with, use the Amazon Resource Name (ARN) of the gateway in your request. This operation is supported for the stored volume, cached volume and tape gateway types.</p>
    async fn delete_bandwidth_rate_limit(
        &self,
        input: DeleteBandwidthRateLimitInput,
    ) -> Result<DeleteBandwidthRateLimitOutput, RusotoError<DeleteBandwidthRateLimitError>>;

    /// <p>Deletes Challenge-Handshake Authentication Protocol (CHAP) credentials for a specified iSCSI target and initiator pair. This operation is supported in volume and tape gateway types.</p>
    async fn delete_chap_credentials(
        &self,
        input: DeleteChapCredentialsInput,
    ) -> Result<DeleteChapCredentialsOutput, RusotoError<DeleteChapCredentialsError>>;

    /// <p>Deletes a file share from a file gateway. This operation is only supported for file gateways.</p>
    async fn delete_file_share(
        &self,
        input: DeleteFileShareInput,
    ) -> Result<DeleteFileShareOutput, RusotoError<DeleteFileShareError>>;

    /// <p><p>Deletes a gateway. To specify which gateway to delete, use the Amazon Resource Name (ARN) of the gateway in your request. The operation deletes the gateway; however, it does not delete the gateway virtual machine (VM) from your host computer.</p> <p>After you delete a gateway, you cannot reactivate it. Completed snapshots of the gateway volumes are not deleted upon deleting the gateway, however, pending snapshots will not complete. After you delete a gateway, your next step is to remove it from your environment.</p> <important> <p>You no longer pay software charges after the gateway is deleted; however, your existing Amazon EBS snapshots persist and you will continue to be billed for these snapshots. You can choose to remove all remaining Amazon EBS snapshots by canceling your Amazon EC2 subscription.  If you prefer not to cancel your Amazon EC2 subscription, you can delete your snapshots using the Amazon EC2 console. For more information, see the <a href="http://aws.amazon.com/storagegateway">AWS Storage Gateway detail page</a>.</p> </important></p>
    async fn delete_gateway(
        &self,
        input: DeleteGatewayInput,
    ) -> Result<DeleteGatewayOutput, RusotoError<DeleteGatewayError>>;

    /// <p><p>Deletes a snapshot of a volume.</p> <p>You can take snapshots of your gateway volumes on a scheduled or ad hoc basis. This API action enables you to delete a snapshot schedule for a volume. For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/backing-up-volumes.html">Backing up your volumes</a>. In the <code>DeleteSnapshotSchedule</code> request, you identify the volume by providing its Amazon Resource Name (ARN). This operation is only supported in stored and cached volume gateway types.</p> <note> <p>To list or delete a snapshot, you must use the Amazon EC2 API. For more information, go to <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSnapshots.html">DescribeSnapshots</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </note></p>
    async fn delete_snapshot_schedule(
        &self,
        input: DeleteSnapshotScheduleInput,
    ) -> Result<DeleteSnapshotScheduleOutput, RusotoError<DeleteSnapshotScheduleError>>;

    /// <p>Deletes the specified virtual tape. This operation is only supported in the tape gateway type.</p>
    async fn delete_tape(
        &self,
        input: DeleteTapeInput,
    ) -> Result<DeleteTapeOutput, RusotoError<DeleteTapeError>>;

    /// <p>Deletes the specified virtual tape from the virtual tape shelf (VTS). This operation is only supported in the tape gateway type.</p>
    async fn delete_tape_archive(
        &self,
        input: DeleteTapeArchiveInput,
    ) -> Result<DeleteTapeArchiveOutput, RusotoError<DeleteTapeArchiveError>>;

    /// <p>Delete a custom tape pool. A custom tape pool can only be deleted if there are no tapes in the pool and if there are no automatic tape creation policies that reference the custom tape pool.</p>
    async fn delete_tape_pool(
        &self,
        input: DeleteTapePoolInput,
    ) -> Result<DeleteTapePoolOutput, RusotoError<DeleteTapePoolError>>;

    /// <p>Deletes the specified storage volume that you previously created using the <a>CreateCachediSCSIVolume</a> or <a>CreateStorediSCSIVolume</a> API. This operation is only supported in the cached volume and stored volume types. For stored volume gateways, the local disk that was configured as the storage volume is not deleted. You can reuse the local disk to create another storage volume.</p> <p>Before you delete a volume, make sure there are no iSCSI connections to the volume you are deleting. You should also make sure there is no snapshot in progress. You can use the Amazon Elastic Compute Cloud (Amazon EC2) API to query snapshots on the volume you are deleting and check the snapshot status. For more information, go to <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-DescribeSnapshots.html">DescribeSnapshots</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> <p>In the request, you must provide the Amazon Resource Name (ARN) of the storage volume you want to delete.</p>
    async fn delete_volume(
        &self,
        input: DeleteVolumeInput,
    ) -> Result<DeleteVolumeOutput, RusotoError<DeleteVolumeError>>;

    /// <p>Returns information about the most recent high availability monitoring test that was performed on the host in a cluster. If a test isn't performed, the status and start time in the response would be null.</p>
    async fn describe_availability_monitor_test(
        &self,
        input: DescribeAvailabilityMonitorTestInput,
    ) -> Result<
        DescribeAvailabilityMonitorTestOutput,
        RusotoError<DescribeAvailabilityMonitorTestError>,
    >;

    /// <p>Returns the bandwidth rate limits of a gateway. By default, these limits are not set, which means no bandwidth rate limiting is in effect. This operation is supported for the stored volume, cached volume, and tape gateway types.</p> <p>This operation only returns a value for a bandwidth rate limit only if the limit is set. If no limits are set for the gateway, then this operation returns only the gateway ARN in the response body. To specify which gateway to describe, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    async fn describe_bandwidth_rate_limit(
        &self,
        input: DescribeBandwidthRateLimitInput,
    ) -> Result<DescribeBandwidthRateLimitOutput, RusotoError<DescribeBandwidthRateLimitError>>;

    /// <p> Returns information about the bandwidth rate limit schedule of a gateway. By default, gateways do not have bandwidth rate limit schedules, which means no bandwidth rate limiting is in effect. This operation is supported only in the volume and tape gateway types. </p> <p>This operation returns information about a gateway's bandwidth rate limit schedule. A bandwidth rate limit schedule consists of one or more bandwidth rate limit intervals. A bandwidth rate limit interval defines a period of time on one or more days of the week, during which bandwidth rate limits are specified for uploading, downloading, or both. </p> <p> A bandwidth rate limit interval consists of one or more days of the week, a start hour and minute, an ending hour and minute, and bandwidth rate limits for uploading and downloading </p> <p> If no bandwidth rate limit schedule intervals are set for the gateway, this operation returns an empty response. To specify which gateway to describe, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    async fn describe_bandwidth_rate_limit_schedule(
        &self,
        input: DescribeBandwidthRateLimitScheduleInput,
    ) -> Result<
        DescribeBandwidthRateLimitScheduleOutput,
        RusotoError<DescribeBandwidthRateLimitScheduleError>,
    >;

    /// <p>Returns information about the cache of a gateway. This operation is only supported in the cached volume, tape, and file gateway types.</p> <p>The response includes disk IDs that are configured as cache, and it includes the amount of cache allocated and used.</p>
    async fn describe_cache(
        &self,
        input: DescribeCacheInput,
    ) -> Result<DescribeCacheOutput, RusotoError<DescribeCacheError>>;

    /// <p>Returns a description of the gateway volumes specified in the request. This operation is only supported in the cached volume gateway types.</p> <p>The list of gateway volumes in the request must be from one gateway. In the response, AWS Storage Gateway returns volume information sorted by volume Amazon Resource Name (ARN).</p>
    async fn describe_cachedi_scsi_volumes(
        &self,
        input: DescribeCachediSCSIVolumesInput,
    ) -> Result<DescribeCachediSCSIVolumesOutput, RusotoError<DescribeCachediSCSIVolumesError>>;

    /// <p>Returns an array of Challenge-Handshake Authentication Protocol (CHAP) credentials information for a specified iSCSI target, one for each target-initiator pair. This operation is supported in the volume and tape gateway types.</p>
    async fn describe_chap_credentials(
        &self,
        input: DescribeChapCredentialsInput,
    ) -> Result<DescribeChapCredentialsOutput, RusotoError<DescribeChapCredentialsError>>;

    /// <p>Gets the file system association information. This operation is only supported for Amazon FSx file gateways.</p>
    async fn describe_file_system_associations(
        &self,
        input: DescribeFileSystemAssociationsInput,
    ) -> Result<
        DescribeFileSystemAssociationsOutput,
        RusotoError<DescribeFileSystemAssociationsError>,
    >;

    /// <p>Returns metadata about a gateway such as its name, network interfaces, configured time zone, and the state (whether the gateway is running or not). To specify which gateway to describe, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    async fn describe_gateway_information(
        &self,
        input: DescribeGatewayInformationInput,
    ) -> Result<DescribeGatewayInformationOutput, RusotoError<DescribeGatewayInformationError>>;

    /// <p>Returns your gateway's weekly maintenance start time including the day and time of the week. Note that values are in terms of the gateway's time zone.</p>
    async fn describe_maintenance_start_time(
        &self,
        input: DescribeMaintenanceStartTimeInput,
    ) -> Result<DescribeMaintenanceStartTimeOutput, RusotoError<DescribeMaintenanceStartTimeError>>;

    /// <p>Gets a description for one or more Network File System (NFS) file shares from a file gateway. This operation is only supported for file gateways.</p>
    async fn describe_nfs_file_shares(
        &self,
        input: DescribeNFSFileSharesInput,
    ) -> Result<DescribeNFSFileSharesOutput, RusotoError<DescribeNFSFileSharesError>>;

    /// <p>Gets a description for one or more Server Message Block (SMB) file shares from a file gateway. This operation is only supported for file gateways.</p>
    async fn describe_smb_file_shares(
        &self,
        input: DescribeSMBFileSharesInput,
    ) -> Result<DescribeSMBFileSharesOutput, RusotoError<DescribeSMBFileSharesError>>;

    /// <p>Gets a description of a Server Message Block (SMB) file share settings from a file gateway. This operation is only supported for file gateways.</p>
    async fn describe_smb_settings(
        &self,
        input: DescribeSMBSettingsInput,
    ) -> Result<DescribeSMBSettingsOutput, RusotoError<DescribeSMBSettingsError>>;

    /// <p>Describes the snapshot schedule for the specified gateway volume. The snapshot schedule information includes intervals at which snapshots are automatically initiated on the volume. This operation is only supported in the cached volume and stored volume types.</p>
    async fn describe_snapshot_schedule(
        &self,
        input: DescribeSnapshotScheduleInput,
    ) -> Result<DescribeSnapshotScheduleOutput, RusotoError<DescribeSnapshotScheduleError>>;

    /// <p>Returns the description of the gateway volumes specified in the request. The list of gateway volumes in the request must be from one gateway. In the response, AWS Storage Gateway returns volume information sorted by volume ARNs. This operation is only supported in stored volume gateway type.</p>
    async fn describe_storedi_scsi_volumes(
        &self,
        input: DescribeStorediSCSIVolumesInput,
    ) -> Result<DescribeStorediSCSIVolumesOutput, RusotoError<DescribeStorediSCSIVolumesError>>;

    /// <p>Returns a description of specified virtual tapes in the virtual tape shelf (VTS). This operation is only supported in the tape gateway type.</p> <p>If a specific <code>TapeARN</code> is not specified, AWS Storage Gateway returns a description of all virtual tapes found in the VTS associated with your account.</p>
    async fn describe_tape_archives(
        &self,
        input: DescribeTapeArchivesInput,
    ) -> Result<DescribeTapeArchivesOutput, RusotoError<DescribeTapeArchivesError>>;

    /// <p>Returns a list of virtual tape recovery points that are available for the specified tape gateway.</p> <p>A recovery point is a point-in-time view of a virtual tape at which all the data on the virtual tape is consistent. If your gateway crashes, virtual tapes that have recovery points can be recovered to a new gateway. This operation is only supported in the tape gateway type.</p>
    async fn describe_tape_recovery_points(
        &self,
        input: DescribeTapeRecoveryPointsInput,
    ) -> Result<DescribeTapeRecoveryPointsOutput, RusotoError<DescribeTapeRecoveryPointsError>>;

    /// <p>Returns a description of the specified Amazon Resource Name (ARN) of virtual tapes. If a <code>TapeARN</code> is not specified, returns a description of all virtual tapes associated with the specified gateway. This operation is only supported in the tape gateway type.</p>
    async fn describe_tapes(
        &self,
        input: DescribeTapesInput,
    ) -> Result<DescribeTapesOutput, RusotoError<DescribeTapesError>>;

    /// <p>Returns information about the upload buffer of a gateway. This operation is supported for the stored volume, cached volume, and tape gateway types.</p> <p>The response includes disk IDs that are configured as upload buffer space, and it includes the amount of upload buffer space allocated and used.</p>
    async fn describe_upload_buffer(
        &self,
        input: DescribeUploadBufferInput,
    ) -> Result<DescribeUploadBufferOutput, RusotoError<DescribeUploadBufferError>>;

    /// <p>Returns a description of virtual tape library (VTL) devices for the specified tape gateway. In the response, AWS Storage Gateway returns VTL device information.</p> <p>This operation is only supported in the tape gateway type.</p>
    async fn describe_vtl_devices(
        &self,
        input: DescribeVTLDevicesInput,
    ) -> Result<DescribeVTLDevicesOutput, RusotoError<DescribeVTLDevicesError>>;

    /// <p>Returns information about the working storage of a gateway. This operation is only supported in the stored volumes gateway type. This operation is deprecated in cached volumes API version (20120630). Use DescribeUploadBuffer instead.</p> <note> <p>Working storage is also referred to as upload buffer. You can also use the DescribeUploadBuffer operation to add upload buffer to a stored volume gateway.</p> </note> <p>The response includes disk IDs that are configured as working storage, and it includes the amount of working storage allocated and used.</p>
    async fn describe_working_storage(
        &self,
        input: DescribeWorkingStorageInput,
    ) -> Result<DescribeWorkingStorageOutput, RusotoError<DescribeWorkingStorageError>>;

    /// <p>Disconnects a volume from an iSCSI connection and then detaches the volume from the specified gateway. Detaching and attaching a volume enables you to recover your data from one gateway to a different gateway without creating a snapshot. It also makes it easier to move your volumes from an on-premises gateway to a gateway hosted on an Amazon EC2 instance. This operation is only supported in the volume gateway type.</p>
    async fn detach_volume(
        &self,
        input: DetachVolumeInput,
    ) -> Result<DetachVolumeOutput, RusotoError<DetachVolumeError>>;

    /// <p><p>Disables a tape gateway when the gateway is no longer functioning. For example, if your gateway VM is damaged, you can disable the gateway so you can recover virtual tapes.</p> <p>Use this operation for a tape gateway that is not reachable or not functioning. This operation is only supported in the tape gateway type.</p> <important> <p>After a gateway is disabled, it cannot be enabled.</p> </important></p>
    async fn disable_gateway(
        &self,
        input: DisableGatewayInput,
    ) -> Result<DisableGatewayOutput, RusotoError<DisableGatewayError>>;

    /// <p>Disassociates an Amazon FSx file system from the specified gateway. After the disassociation process finishes, the gateway can no longer access the Amazon FSx file system. This operation is only supported in the Amazon FSx file gateway type.</p>
    async fn disassociate_file_system(
        &self,
        input: DisassociateFileSystemInput,
    ) -> Result<DisassociateFileSystemOutput, RusotoError<DisassociateFileSystemError>>;

    /// <p>Adds a file gateway to an Active Directory domain. This operation is only supported for file gateways that support the SMB file protocol.</p>
    async fn join_domain(
        &self,
        input: JoinDomainInput,
    ) -> Result<JoinDomainOutput, RusotoError<JoinDomainError>>;

    /// <p>Lists the automatic tape creation policies for a gateway. If there are no automatic tape creation policies for the gateway, it returns an empty list.</p> <p>This operation is only supported for tape gateways.</p>
    async fn list_automatic_tape_creation_policies(
        &self,
        input: ListAutomaticTapeCreationPoliciesInput,
    ) -> Result<
        ListAutomaticTapeCreationPoliciesOutput,
        RusotoError<ListAutomaticTapeCreationPoliciesError>,
    >;

    /// <p>Gets a list of the file shares for a specific file gateway, or the list of file shares that belong to the calling user account. This operation is only supported for file gateways.</p>
    async fn list_file_shares(
        &self,
        input: ListFileSharesInput,
    ) -> Result<ListFileSharesOutput, RusotoError<ListFileSharesError>>;

    /// <p>Gets a list of <code>FileSystemAssociationSummary</code> objects. Each object contains a summary of a file system association. This operation is only supported for Amazon FSx file gateways.</p>
    async fn list_file_system_associations(
        &self,
        input: ListFileSystemAssociationsInput,
    ) -> Result<ListFileSystemAssociationsOutput, RusotoError<ListFileSystemAssociationsError>>;

    /// <p>Lists gateways owned by an AWS account in an AWS Region specified in the request. The returned list is ordered by gateway Amazon Resource Name (ARN).</p> <p>By default, the operation returns a maximum of 100 gateways. This operation supports pagination that allows you to optionally reduce the number of gateways returned in a response.</p> <p>If you have more gateways than are returned in a response (that is, the response returns only a truncated list of your gateways), the response contains a marker that you can specify in your next request to fetch the next page of gateways.</p>
    async fn list_gateways(
        &self,
        input: ListGatewaysInput,
    ) -> Result<ListGatewaysOutput, RusotoError<ListGatewaysError>>;

    /// <p>Returns a list of the gateway's local disks. To specify which gateway to describe, you use the Amazon Resource Name (ARN) of the gateway in the body of the request.</p> <p>The request returns a list of all disks, specifying which are configured as working storage, cache storage, or stored volume or not configured at all. The response includes a <code>DiskStatus</code> field. This field can have a value of present (the disk is available to use), missing (the disk is no longer connected to the gateway), or mismatch (the disk node is occupied by a disk that has incorrect metadata or the disk content is corrupted).</p>
    async fn list_local_disks(
        &self,
        input: ListLocalDisksInput,
    ) -> Result<ListLocalDisksOutput, RusotoError<ListLocalDisksError>>;

    /// <p>Lists the tags that have been added to the specified resource. This operation is supported in storage gateways of all types.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists custom tape pools. You specify custom tape pools to list by specifying one or more custom tape pool Amazon Resource Names (ARNs). If you don't specify a custom tape pool ARN, the operation lists all custom tape pools.</p> <p>This operation supports pagination. You can optionally specify the <code>Limit</code> parameter in the body to limit the number of tape pools in the response. If the number of tape pools returned in the response is truncated, the response includes a <code>Marker</code> element that you can use in your subsequent request to retrieve the next set of tape pools.</p>
    async fn list_tape_pools(
        &self,
        input: ListTapePoolsInput,
    ) -> Result<ListTapePoolsOutput, RusotoError<ListTapePoolsError>>;

    /// <p>Lists virtual tapes in your virtual tape library (VTL) and your virtual tape shelf (VTS). You specify the tapes to list by specifying one or more tape Amazon Resource Names (ARNs). If you don't specify a tape ARN, the operation lists all virtual tapes in both your VTL and VTS.</p> <p>This operation supports pagination. By default, the operation returns a maximum of up to 100 tapes. You can optionally specify the <code>Limit</code> parameter in the body to limit the number of tapes in the response. If the number of tapes returned in the response is truncated, the response includes a <code>Marker</code> element that you can use in your subsequent request to retrieve the next set of tapes. This operation is only supported in the tape gateway type.</p>
    async fn list_tapes(
        &self,
        input: ListTapesInput,
    ) -> Result<ListTapesOutput, RusotoError<ListTapesError>>;

    /// <p>Lists iSCSI initiators that are connected to a volume. You can use this operation to determine whether a volume is being used or not. This operation is only supported in the cached volume and stored volume gateway types.</p>
    async fn list_volume_initiators(
        &self,
        input: ListVolumeInitiatorsInput,
    ) -> Result<ListVolumeInitiatorsOutput, RusotoError<ListVolumeInitiatorsError>>;

    /// <p>Lists the recovery points for a specified gateway. This operation is only supported in the cached volume gateway type.</p> <p>Each cache volume has one recovery point. A volume recovery point is a point in time at which all data of the volume is consistent and from which you can create a snapshot or clone a new cached volume from a source volume. To create a snapshot from a volume recovery point use the <a>CreateSnapshotFromVolumeRecoveryPoint</a> operation.</p>
    async fn list_volume_recovery_points(
        &self,
        input: ListVolumeRecoveryPointsInput,
    ) -> Result<ListVolumeRecoveryPointsOutput, RusotoError<ListVolumeRecoveryPointsError>>;

    /// <p>Lists the iSCSI stored volumes of a gateway. Results are sorted by volume ARN. The response includes only the volume ARNs. If you want additional volume information, use the <a>DescribeStorediSCSIVolumes</a> or the <a>DescribeCachediSCSIVolumes</a> API.</p> <p>The operation supports pagination. By default, the operation returns a maximum of up to 100 volumes. You can optionally specify the <code>Limit</code> field in the body to limit the number of volumes in the response. If the number of volumes returned in the response is truncated, the response includes a Marker field. You can use this Marker value in your subsequent request to retrieve the next set of volumes. This operation is only supported in the cached volume and stored volume gateway types.</p>
    async fn list_volumes(
        &self,
        input: ListVolumesInput,
    ) -> Result<ListVolumesOutput, RusotoError<ListVolumesError>>;

    /// <p>Sends you notification through CloudWatch Events when all files written to your file share have been uploaded to Amazon S3.</p> <p>AWS Storage Gateway can send a notification through Amazon CloudWatch Events when all files written to your file share up to that point in time have been uploaded to Amazon S3. These files include files written to the file share up to the time that you make a request for notification. When the upload is done, Storage Gateway sends you notification through an Amazon CloudWatch Event. You can configure CloudWatch Events to send the notification through event targets such as Amazon SNS or AWS Lambda function. This operation is only supported for file gateways.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/monitoring-file-gateway.html#get-upload-notification">Getting file upload notification</a> in the <i>AWS Storage Gateway User Guide</i>.</p>
    async fn notify_when_uploaded(
        &self,
        input: NotifyWhenUploadedInput,
    ) -> Result<NotifyWhenUploadedOutput, RusotoError<NotifyWhenUploadedError>>;

    /// <p>Refreshes the cached inventory of objects for the specified file share. This operation finds objects in the Amazon S3 bucket that were added, removed, or replaced since the gateway last listed the bucket's contents and cached the results. This operation does not import files into the file gateway cache storage. It only updates the cached inventory to reflect changes in the inventory of the objects in the S3 bucket. This operation is only supported in the file gateway type. You can subscribe to be notified through an Amazon CloudWatch event when your <code>RefreshCache</code> operation completes. For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/monitoring-file-gateway.html#get-notification">Getting notified about file operations</a> in the <i>AWS Storage Gateway User Guide</i>.</p> <p>When this API is called, it only initiates the refresh operation. When the API call completes and returns a success code, it doesn't necessarily mean that the file refresh has completed. You should use the refresh-complete notification to determine that the operation has completed before you check for new files on the gateway file share. You can subscribe to be notified through a CloudWatch event when your <code>RefreshCache</code> operation completes.</p> <p>Throttle limit: This API is asynchronous, so the gateway will accept no more than two refreshes at any time. We recommend using the refresh-complete CloudWatch event notification before issuing additional requests. For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/monitoring-file-gateway.html#get-notification">Getting notified about file operations</a> in the <i>AWS Storage Gateway User Guide</i>.</p> <p>If you invoke the RefreshCache API when two requests are already being processed, any new request will cause an <code>InvalidGatewayRequestException</code> error because too many requests were sent to the server.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/monitoring-file-gateway.html#get-notification">Getting notified about file operations</a> in the <i>AWS Storage Gateway User Guide</i>.</p>
    async fn refresh_cache(
        &self,
        input: RefreshCacheInput,
    ) -> Result<RefreshCacheOutput, RusotoError<RefreshCacheError>>;

    /// <p>Removes one or more tags from the specified resource. This operation is supported in storage gateways of all types.</p>
    async fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceInput,
    ) -> Result<RemoveTagsFromResourceOutput, RusotoError<RemoveTagsFromResourceError>>;

    /// <p><p>Resets all cache disks that have encountered an error and makes the disks available for reconfiguration as cache storage. If your cache disk encounters an error, the gateway prevents read and write operations on virtual tapes in the gateway. For example, an error can occur when a disk is corrupted or removed from the gateway. When a cache is reset, the gateway loses its cache storage. At this point, you can reconfigure the disks as cache disks. This operation is only supported in the cached volume and tape types.</p> <important> <p>If the cache disk you are resetting contains data that has not been uploaded to Amazon S3 yet, that data can be lost. After you reset cache disks, there will be no configured cache disks left in the gateway, so you must configure at least one new cache disk for your gateway to function properly.</p> </important></p>
    async fn reset_cache(
        &self,
        input: ResetCacheInput,
    ) -> Result<ResetCacheOutput, RusotoError<ResetCacheError>>;

    /// <p>Retrieves an archived virtual tape from the virtual tape shelf (VTS) to a tape gateway. Virtual tapes archived in the VTS are not associated with any gateway. However after a tape is retrieved, it is associated with a gateway, even though it is also listed in the VTS, that is, archive. This operation is only supported in the tape gateway type.</p> <p>Once a tape is successfully retrieved to a gateway, it cannot be retrieved again to another gateway. You must archive the tape again before you can retrieve it to another gateway. This operation is only supported in the tape gateway type.</p>
    async fn retrieve_tape_archive(
        &self,
        input: RetrieveTapeArchiveInput,
    ) -> Result<RetrieveTapeArchiveOutput, RusotoError<RetrieveTapeArchiveError>>;

    /// <p><p>Retrieves the recovery point for the specified virtual tape. This operation is only supported in the tape gateway type.</p> <p>A recovery point is a point in time view of a virtual tape at which all the data on the tape is consistent. If your gateway crashes, virtual tapes that have recovery points can be recovered to a new gateway.</p> <note> <p>The virtual tape can be retrieved to only one gateway. The retrieved tape is read-only. The virtual tape can be retrieved to only a tape gateway. There is no charge for retrieving recovery points.</p> </note></p>
    async fn retrieve_tape_recovery_point(
        &self,
        input: RetrieveTapeRecoveryPointInput,
    ) -> Result<RetrieveTapeRecoveryPointOutput, RusotoError<RetrieveTapeRecoveryPointError>>;

    /// <p>Sets the password for your VM local console. When you log in to the local console for the first time, you log in to the VM with the default credentials. We recommend that you set a new password. You don't need to know the default password to set a new password.</p>
    async fn set_local_console_password(
        &self,
        input: SetLocalConsolePasswordInput,
    ) -> Result<SetLocalConsolePasswordOutput, RusotoError<SetLocalConsolePasswordError>>;

    /// <p>Sets the password for the guest user <code>smbguest</code>. The <code>smbguest</code> user is the user when the authentication method for the file share is set to <code>GuestAccess</code>.</p>
    async fn set_smb_guest_password(
        &self,
        input: SetSMBGuestPasswordInput,
    ) -> Result<SetSMBGuestPasswordOutput, RusotoError<SetSMBGuestPasswordError>>;

    /// <p>Shuts down a gateway. To specify which gateway to shut down, use the Amazon Resource Name (ARN) of the gateway in the body of your request.</p> <p>The operation shuts down the gateway service component running in the gateway's virtual machine (VM) and not the host VM.</p> <note> <p>If you want to shut down the VM, it is recommended that you first shut down the gateway component in the VM to avoid unpredictable conditions.</p> </note> <p>After the gateway is shutdown, you cannot call any other API except <a>StartGateway</a>, <a>DescribeGatewayInformation</a>, and <a>ListGateways</a>. For more information, see <a>ActivateGateway</a>. Your applications cannot read from or write to the gateway's storage volumes, and there are no snapshots taken.</p> <note> <p>When you make a shutdown request, you will get a <code>200 OK</code> success response immediately. However, it might take some time for the gateway to shut down. You can call the <a>DescribeGatewayInformation</a> API to check the status. For more information, see <a>ActivateGateway</a>.</p> </note> <p>If do not intend to use the gateway again, you must delete the gateway (using <a>DeleteGateway</a>) to no longer pay software charges associated with the gateway.</p>
    async fn shutdown_gateway(
        &self,
        input: ShutdownGatewayInput,
    ) -> Result<ShutdownGatewayOutput, RusotoError<ShutdownGatewayError>>;

    /// <p><p>Start a test that verifies that the specified gateway is configured for High Availability monitoring in your host environment. This request only initiates the test and that a successful response only indicates that the test was started. It doesn&#39;t indicate that the test passed. For the status of the test, invoke the <code>DescribeAvailabilityMonitorTest</code> API.</p> <note> <p>Starting this test will cause your gateway to go offline for a brief period.</p> </note></p>
    async fn start_availability_monitor_test(
        &self,
        input: StartAvailabilityMonitorTestInput,
    ) -> Result<StartAvailabilityMonitorTestOutput, RusotoError<StartAvailabilityMonitorTestError>>;

    /// <p>Starts a gateway that you previously shut down (see <a>ShutdownGateway</a>). After the gateway starts, you can then make other API calls, your applications can read from or write to the gateway's storage volumes and you will be able to take snapshot backups.</p> <note> <p>When you make a request, you will get a 200 OK success response immediately. However, it might take some time for the gateway to be ready. You should call <a>DescribeGatewayInformation</a> and check the status before making any additional API calls. For more information, see <a>ActivateGateway</a>.</p> </note> <p>To specify which gateway to start, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    async fn start_gateway(
        &self,
        input: StartGatewayInput,
    ) -> Result<StartGatewayOutput, RusotoError<StartGatewayError>>;

    /// <p><p>Updates the automatic tape creation policy of a gateway. Use this to update the policy with a new set of automatic tape creation rules. This is only supported for tape gateways.</p> <p>By default, there is no automatic tape creation policy.</p> <note> <p>A gateway can have only one automatic tape creation policy.</p> </note></p>
    async fn update_automatic_tape_creation_policy(
        &self,
        input: UpdateAutomaticTapeCreationPolicyInput,
    ) -> Result<
        UpdateAutomaticTapeCreationPolicyOutput,
        RusotoError<UpdateAutomaticTapeCreationPolicyError>,
    >;

    /// <p>Updates the bandwidth rate limits of a gateway. You can update both the upload and download bandwidth rate limit or specify only one of the two. If you don't set a bandwidth rate limit, the existing rate limit remains. This operation is supported for the stored volume, cached volume, and tape gateway types.</p> <p>By default, a gateway's bandwidth rate limits are not set. If you don't set any limit, the gateway does not have any limitations on its bandwidth usage and could potentially use the maximum available bandwidth.</p> <p>To specify which gateway to update, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    async fn update_bandwidth_rate_limit(
        &self,
        input: UpdateBandwidthRateLimitInput,
    ) -> Result<UpdateBandwidthRateLimitOutput, RusotoError<UpdateBandwidthRateLimitError>>;

    /// <p> Updates the bandwidth rate limit schedule for a specified gateway. By default, gateways do not have bandwidth rate limit schedules, which means no bandwidth rate limiting is in effect. Use this to initiate or update a gateway's bandwidth rate limit schedule. This operation is supported in the volume and tape gateway types. </p>
    async fn update_bandwidth_rate_limit_schedule(
        &self,
        input: UpdateBandwidthRateLimitScheduleInput,
    ) -> Result<
        UpdateBandwidthRateLimitScheduleOutput,
        RusotoError<UpdateBandwidthRateLimitScheduleError>,
    >;

    /// <p><p>Updates the Challenge-Handshake Authentication Protocol (CHAP) credentials for a specified iSCSI target. By default, a gateway does not have CHAP enabled; however, for added security, you might use it. This operation is supported in the volume and tape gateway types.</p> <important> <p>When you update CHAP credentials, all existing connections on the target are closed and initiators must reconnect with the new credentials.</p> </important></p>
    async fn update_chap_credentials(
        &self,
        input: UpdateChapCredentialsInput,
    ) -> Result<UpdateChapCredentialsOutput, RusotoError<UpdateChapCredentialsError>>;

    /// <p>Updates a file system association. This operation is only supported in the Amazon FSx file gateway type.</p>
    async fn update_file_system_association(
        &self,
        input: UpdateFileSystemAssociationInput,
    ) -> Result<UpdateFileSystemAssociationOutput, RusotoError<UpdateFileSystemAssociationError>>;

    /// <p><p>Updates a gateway&#39;s metadata, which includes the gateway&#39;s name and time zone. To specify which gateway to update, use the Amazon Resource Name (ARN) of the gateway in your request.</p> <note> <p>For gateways activated after September 2, 2015, the gateway&#39;s ARN contains the gateway ID rather than the gateway name. However, changing the name of the gateway has no effect on the gateway&#39;s ARN.</p> </note></p>
    async fn update_gateway_information(
        &self,
        input: UpdateGatewayInformationInput,
    ) -> Result<UpdateGatewayInformationOutput, RusotoError<UpdateGatewayInformationError>>;

    /// <p><p>Updates the gateway virtual machine (VM) software. The request immediately triggers the software update.</p> <note> <p>When you make this request, you get a <code>200 OK</code> success response immediately. However, it might take some time for the update to complete. You can call <a>DescribeGatewayInformation</a> to verify the gateway is in the <code>STATE_RUNNING</code> state.</p> </note> <important> <p>A software update forces a system restart of your gateway. You can minimize the chance of any disruption to your applications by increasing your iSCSI Initiators&#39; timeouts. For more information about increasing iSCSI Initiator timeouts for Windows and Linux, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/ConfiguringiSCSIClientInitiatorWindowsClient.html#CustomizeWindowsiSCSISettings">Customizing your Windows iSCSI settings</a> and <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/ConfiguringiSCSIClientInitiatorRedHatClient.html#CustomizeLinuxiSCSISettings">Customizing your Linux iSCSI settings</a>, respectively.</p> </important></p>
    async fn update_gateway_software_now(
        &self,
        input: UpdateGatewaySoftwareNowInput,
    ) -> Result<UpdateGatewaySoftwareNowOutput, RusotoError<UpdateGatewaySoftwareNowError>>;

    /// <p>Updates a gateway's weekly maintenance start time information, including day and time of the week. The maintenance time is the time in your gateway's time zone.</p>
    async fn update_maintenance_start_time(
        &self,
        input: UpdateMaintenanceStartTimeInput,
    ) -> Result<UpdateMaintenanceStartTimeOutput, RusotoError<UpdateMaintenanceStartTimeError>>;

    /// <p><p>Updates a Network File System (NFS) file share. This operation is only supported in the file gateway type.</p> <note> <p>To leave a file share field unchanged, set the corresponding input field to null.</p> </note> <p>Updates the following file share settings:</p> <ul> <li> <p>Default storage class for your S3 bucket</p> </li> <li> <p>Metadata defaults for your S3 bucket</p> </li> <li> <p>Allowed NFS clients for your file share</p> </li> <li> <p>Squash settings</p> </li> <li> <p>Write status of your file share</p> </li> </ul></p>
    async fn update_nfs_file_share(
        &self,
        input: UpdateNFSFileShareInput,
    ) -> Result<UpdateNFSFileShareOutput, RusotoError<UpdateNFSFileShareError>>;

    /// <p><p>Updates a Server Message Block (SMB) file share. This operation is only supported for file gateways.</p> <note> <p>To leave a file share field unchanged, set the corresponding input field to null.</p> </note> <important> <p>File gateways require AWS Security Token Service (AWS STS) to be activated to enable you to create a file share. Make sure that AWS STS is activated in the AWS Region you are creating your file gateway in. If AWS STS is not activated in this AWS Region, activate it. For information about how to activate AWS STS, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and deactivating AWS STS in an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> <p>File gateways don&#39;t support creating hard or symbolic links on a file share.</p> </important></p>
    async fn update_smb_file_share(
        &self,
        input: UpdateSMBFileShareInput,
    ) -> Result<UpdateSMBFileShareOutput, RusotoError<UpdateSMBFileShareError>>;

    /// <p>Controls whether the shares on a gateway are visible in a net view or browse list.</p>
    async fn update_smb_file_share_visibility(
        &self,
        input: UpdateSMBFileShareVisibilityInput,
    ) -> Result<UpdateSMBFileShareVisibilityOutput, RusotoError<UpdateSMBFileShareVisibilityError>>;

    /// <p><p>Updates the SMB security strategy on a file gateway. This action is only supported in file gateways.</p> <note> <p>This API is called Security level in the User Guide.</p> <p>A higher security level can affect performance of the gateway.</p> </note></p>
    async fn update_smb_security_strategy(
        &self,
        input: UpdateSMBSecurityStrategyInput,
    ) -> Result<UpdateSMBSecurityStrategyOutput, RusotoError<UpdateSMBSecurityStrategyError>>;

    /// <p>Updates a snapshot schedule configured for a gateway volume. This operation is only supported in the cached volume and stored volume gateway types.</p> <p>The default snapshot schedule for volume is once every 24 hours, starting at the creation time of the volume. You can use this API to change the snapshot schedule configured for the volume.</p> <p>In the request you must identify the gateway volume whose snapshot schedule you want to update, and the schedule information, including when you want the snapshot to begin on a day and the frequency (in hours) of snapshots.</p>
    async fn update_snapshot_schedule(
        &self,
        input: UpdateSnapshotScheduleInput,
    ) -> Result<UpdateSnapshotScheduleOutput, RusotoError<UpdateSnapshotScheduleError>>;

    /// <p>Updates the type of medium changer in a tape gateway. When you activate a tape gateway, you select a medium changer type for the tape gateway. This operation enables you to select a different type of medium changer after a tape gateway is activated. This operation is only supported in the tape gateway type.</p>
    async fn update_vtl_device_type(
        &self,
        input: UpdateVTLDeviceTypeInput,
    ) -> Result<UpdateVTLDeviceTypeOutput, RusotoError<UpdateVTLDeviceTypeError>>;
}
/// A client for the AWS Storage Gateway API.
#[derive(Clone)]
pub struct StorageGatewayClient {
    client: Client,
    region: region::Region,
}

impl StorageGatewayClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> StorageGatewayClient {
        StorageGatewayClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> StorageGatewayClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        StorageGatewayClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> StorageGatewayClient {
        StorageGatewayClient { client, region }
    }
}

#[async_trait]
impl StorageGateway for StorageGatewayClient {
    /// <p><p>Activates the gateway you previously deployed on your host. In the activation process, you specify information such as the AWS Region that you want to use for storing snapshots or tapes, the time zone for scheduled snapshots the gateway snapshot schedule window, an activation key, and a name for your gateway. The activation process also associates your gateway with your account. For more information, see <a>UpdateGatewayInformation</a>.</p> <note> <p>You must turn on the gateway VM before you can activate your gateway.</p> </note></p>
    async fn activate_gateway(
        &self,
        input: ActivateGatewayInput,
    ) -> Result<ActivateGatewayOutput, RusotoError<ActivateGatewayError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.ActivateGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ActivateGatewayError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ActivateGatewayOutput, _>()
    }

    /// <p>Configures one or more gateway local disks as cache for a gateway. This operation is only supported in the cached volume, tape, and file gateway type (see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/StorageGatewayConcepts.html">How AWS Storage Gateway works (architecture)</a>.</p> <p>In the request, you specify the gateway Amazon Resource Name (ARN) to which you want to add cache, and one or more disk IDs that you want to configure as cache.</p>
    async fn add_cache(
        &self,
        input: AddCacheInput,
    ) -> Result<AddCacheOutput, RusotoError<AddCacheError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.AddCache");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AddCacheError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AddCacheOutput, _>()
    }

    /// <p>Adds one or more tags to the specified resource. You use tags to add metadata to resources, which you can use to categorize these resources. For example, you can categorize resources by purpose, owner, environment, or team. Each tag consists of a key and a value, which you define. You can add tags to the following AWS Storage Gateway resources:</p> <ul> <li> <p>Storage gateways of all types</p> </li> <li> <p>Storage volumes</p> </li> <li> <p>Virtual tapes</p> </li> <li> <p>NFS and SMB file shares</p> </li> </ul> <p>You can create a maximum of 50 tags for each resource. Virtual tapes and storage volumes that are recovered to a new gateway maintain their tags.</p>
    async fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceInput,
    ) -> Result<AddTagsToResourceOutput, RusotoError<AddTagsToResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.AddTagsToResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AddTagsToResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AddTagsToResourceOutput, _>()
    }

    /// <p>Configures one or more gateway local disks as upload buffer for a specified gateway. This operation is supported for the stored volume, cached volume, and tape gateway types.</p> <p>In the request, you specify the gateway Amazon Resource Name (ARN) to which you want to add upload buffer, and one or more disk IDs that you want to configure as upload buffer.</p>
    async fn add_upload_buffer(
        &self,
        input: AddUploadBufferInput,
    ) -> Result<AddUploadBufferOutput, RusotoError<AddUploadBufferError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.AddUploadBuffer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AddUploadBufferError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AddUploadBufferOutput, _>()
    }

    /// <p>Configures one or more gateway local disks as working storage for a gateway. This operation is only supported in the stored volume gateway type. This operation is deprecated in cached volume API version 20120630. Use <a>AddUploadBuffer</a> instead.</p> <note> <p>Working storage is also referred to as upload buffer. You can also use the <a>AddUploadBuffer</a> operation to add upload buffer to a stored volume gateway.</p> </note> <p>In the request, you specify the gateway Amazon Resource Name (ARN) to which you want to add working storage, and one or more disk IDs that you want to configure as working storage.</p>
    async fn add_working_storage(
        &self,
        input: AddWorkingStorageInput,
    ) -> Result<AddWorkingStorageOutput, RusotoError<AddWorkingStorageError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.AddWorkingStorage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AddWorkingStorageError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AddWorkingStorageOutput, _>()
    }

    /// <p>Assigns a tape to a tape pool for archiving. The tape assigned to a pool is archived in the S3 storage class that is associated with the pool. When you use your backup application to eject the tape, the tape is archived directly into the S3 storage class (S3 Glacier or S3 Glacier Deep Archive) that corresponds to the pool.</p> <p>Valid Values: <code>GLACIER</code> | <code>DEEP_ARCHIVE</code> </p>
    async fn assign_tape_pool(
        &self,
        input: AssignTapePoolInput,
    ) -> Result<AssignTapePoolOutput, RusotoError<AssignTapePoolError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.AssignTapePool");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssignTapePoolError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AssignTapePoolOutput, _>()
    }

    /// <p>Associate an Amazon FSx file system with the Amazon FSx file gateway. After the association process is complete, the file shares on the Amazon FSx file system are available for access through the gateway. This operation only supports the Amazon FSx file gateway type.</p>
    async fn associate_file_system(
        &self,
        input: AssociateFileSystemInput,
    ) -> Result<AssociateFileSystemOutput, RusotoError<AssociateFileSystemError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.AssociateFileSystem",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AssociateFileSystemError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AssociateFileSystemOutput, _>()
    }

    /// <p>Connects a volume to an iSCSI connection and then attaches the volume to the specified gateway. Detaching and attaching a volume enables you to recover your data from one gateway to a different gateway without creating a snapshot. It also makes it easier to move your volumes from an on-premises gateway to a gateway hosted on an Amazon EC2 instance.</p>
    async fn attach_volume(
        &self,
        input: AttachVolumeInput,
    ) -> Result<AttachVolumeOutput, RusotoError<AttachVolumeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.AttachVolume");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AttachVolumeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AttachVolumeOutput, _>()
    }

    /// <p>Cancels archiving of a virtual tape to the virtual tape shelf (VTS) after the archiving process is initiated. This operation is only supported in the tape gateway type.</p>
    async fn cancel_archival(
        &self,
        input: CancelArchivalInput,
    ) -> Result<CancelArchivalOutput, RusotoError<CancelArchivalError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.CancelArchival");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CancelArchivalError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CancelArchivalOutput, _>()
    }

    /// <p>Cancels retrieval of a virtual tape from the virtual tape shelf (VTS) to a gateway after the retrieval process is initiated. The virtual tape is returned to the VTS. This operation is only supported in the tape gateway type.</p>
    async fn cancel_retrieval(
        &self,
        input: CancelRetrievalInput,
    ) -> Result<CancelRetrievalOutput, RusotoError<CancelRetrievalError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.CancelRetrieval");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CancelRetrievalError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CancelRetrievalOutput, _>()
    }

    /// <p>Creates a cached volume on a specified cached volume gateway. This operation is only supported in the cached volume gateway type.</p> <note> <p>Cache storage must be allocated to the gateway before you can create a cached volume. Use the <a>AddCache</a> operation to add cache storage to a gateway.</p> </note> <p>In the request, you must specify the gateway, size of the volume in bytes, the iSCSI target name, an IP address on which to expose the target, and a unique client token. In response, the gateway creates the volume and returns information about it. This information includes the volume Amazon Resource Name (ARN), its size, and the iSCSI target ARN that initiators can use to connect to the volume target.</p> <p>Optionally, you can provide the ARN for an existing volume as the <code>SourceVolumeARN</code> for this cached volume, which creates an exact copy of the existing volume’s latest recovery point. The <code>VolumeSizeInBytes</code> value must be equal to or larger than the size of the copied volume, in bytes.</p>
    async fn create_cachedi_scsi_volume(
        &self,
        input: CreateCachediSCSIVolumeInput,
    ) -> Result<CreateCachediSCSIVolumeOutput, RusotoError<CreateCachediSCSIVolumeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.CreateCachediSCSIVolume",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateCachediSCSIVolumeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateCachediSCSIVolumeOutput, _>()
    }

    /// <p><p>Creates a Network File System (NFS) file share on an existing file gateway. In Storage Gateway, a file share is a file system mount point backed by Amazon S3 cloud storage. Storage Gateway exposes file shares using an NFS interface. This operation is only supported for file gateways.</p> <important> <p>File gateway requires AWS Security Token Service (AWS STS) to be activated to enable you to create a file share. Make sure AWS STS is activated in the AWS Region you are creating your file gateway in. If AWS STS is not activated in the AWS Region, activate it. For information about how to activate AWS STS, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and deactivating AWS STS in an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> <p>File gateway does not support creating hard or symbolic links on a file share.</p> </important></p>
    async fn create_nfs_file_share(
        &self,
        input: CreateNFSFileShareInput,
    ) -> Result<CreateNFSFileShareOutput, RusotoError<CreateNFSFileShareError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.CreateNFSFileShare");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateNFSFileShareError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateNFSFileShareOutput, _>()
    }

    /// <p><p>Creates a Server Message Block (SMB) file share on an existing file gateway. In Storage Gateway, a file share is a file system mount point backed by Amazon S3 cloud storage. Storage Gateway exposes file shares using an SMB interface. This operation is only supported for file gateways.</p> <important> <p>File gateways require AWS Security Token Service (AWS STS) to be activated to enable you to create a file share. Make sure that AWS STS is activated in the AWS Region you are creating your file gateway in. If AWS STS is not activated in this AWS Region, activate it. For information about how to activate AWS STS, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and deactivating AWS STS in an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> <p>File gateways don&#39;t support creating hard or symbolic links on a file share.</p> </important></p>
    async fn create_smb_file_share(
        &self,
        input: CreateSMBFileShareInput,
    ) -> Result<CreateSMBFileShareOutput, RusotoError<CreateSMBFileShareError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.CreateSMBFileShare");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateSMBFileShareError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateSMBFileShareOutput, _>()
    }

    /// <p><p>Initiates a snapshot of a volume.</p> <p>AWS Storage Gateway provides the ability to back up point-in-time snapshots of your data to Amazon Simple Storage (Amazon S3) for durable off-site recovery, and also import the data to an Amazon Elastic Block Store (EBS) volume in Amazon Elastic Compute Cloud (EC2). You can take snapshots of your gateway volume on a scheduled or ad hoc basis. This API enables you to take an ad hoc snapshot. For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/managing-volumes.html#SchedulingSnapshot">Editing a snapshot schedule</a>.</p> <p>In the <code>CreateSnapshot</code> request, you identify the volume by providing its Amazon Resource Name (ARN). You must also provide description for the snapshot. When AWS Storage Gateway takes the snapshot of specified volume, the snapshot and description appears in the AWS Storage Gateway console. In response, AWS Storage Gateway returns you a snapshot ID. You can use this snapshot ID to check the snapshot progress or later use it when you want to create a volume from a snapshot. This operation is only supported in stored and cached volume gateway type.</p> <note> <p>To list or delete a snapshot, you must use the Amazon EC2 API. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSnapshots.html">DescribeSnapshots</a> or <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DeleteSnapshot.html">DeleteSnapshot</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </note> <important> <p>Volume and snapshot IDs are changing to a longer length ID format. For more information, see the important note on the <a href="https://docs.aws.amazon.com/storagegateway/latest/APIReference/Welcome.html">Welcome</a> page.</p> </important></p>
    async fn create_snapshot(
        &self,
        input: CreateSnapshotInput,
    ) -> Result<CreateSnapshotOutput, RusotoError<CreateSnapshotError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.CreateSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateSnapshotError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateSnapshotOutput, _>()
    }

    /// <p><p>Initiates a snapshot of a gateway from a volume recovery point. This operation is only supported in the cached volume gateway type.</p> <p>A volume recovery point is a point in time at which all data of the volume is consistent and from which you can create a snapshot. To get a list of volume recovery point for cached volume gateway, use <a>ListVolumeRecoveryPoints</a>.</p> <p>In the <code>CreateSnapshotFromVolumeRecoveryPoint</code> request, you identify the volume by providing its Amazon Resource Name (ARN). You must also provide a description for the snapshot. When the gateway takes a snapshot of the specified volume, the snapshot and its description appear in the AWS Storage Gateway console. In response, the gateway returns you a snapshot ID. You can use this snapshot ID to check the snapshot progress or later use it when you want to create a volume from a snapshot.</p> <note> <p>To list or delete a snapshot, you must use the Amazon EC2 API. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSnapshots.html">DescribeSnapshots</a> or <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DeleteSnapshot.html">DeleteSnapshot</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </note></p>
    async fn create_snapshot_from_volume_recovery_point(
        &self,
        input: CreateSnapshotFromVolumeRecoveryPointInput,
    ) -> Result<
        CreateSnapshotFromVolumeRecoveryPointOutput,
        RusotoError<CreateSnapshotFromVolumeRecoveryPointError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.CreateSnapshotFromVolumeRecoveryPoint",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                CreateSnapshotFromVolumeRecoveryPointError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateSnapshotFromVolumeRecoveryPointOutput, _>()
    }

    /// <p>Creates a volume on a specified gateway. This operation is only supported in the stored volume gateway type.</p> <p>The size of the volume to create is inferred from the disk size. You can choose to preserve existing data on the disk, create volume from an existing snapshot, or create an empty volume. If you choose to create an empty gateway volume, then any existing data on the disk is erased.</p> <p>In the request, you must specify the gateway and the disk information on which you are creating the volume. In response, the gateway creates the volume and returns volume information such as the volume Amazon Resource Name (ARN), its size, and the iSCSI target ARN that initiators can use to connect to the volume target.</p>
    async fn create_storedi_scsi_volume(
        &self,
        input: CreateStorediSCSIVolumeInput,
    ) -> Result<CreateStorediSCSIVolumeOutput, RusotoError<CreateStorediSCSIVolumeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.CreateStorediSCSIVolume",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateStorediSCSIVolumeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateStorediSCSIVolumeOutput, _>()
    }

    /// <p>Creates a new custom tape pool. You can use custom tape pool to enable tape retention lock on tapes that are archived in the custom pool.</p>
    async fn create_tape_pool(
        &self,
        input: CreateTapePoolInput,
    ) -> Result<CreateTapePoolOutput, RusotoError<CreateTapePoolError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.CreateTapePool");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateTapePoolError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateTapePoolOutput, _>()
    }

    /// <p><p>Creates a virtual tape by using your own barcode. You write data to the virtual tape and then archive the tape. A barcode is unique and cannot be reused if it has already been used on a tape. This applies to barcodes used on deleted tapes. This operation is only supported in the tape gateway type.</p> <note> <p>Cache storage must be allocated to the gateway before you can create a virtual tape. Use the <a>AddCache</a> operation to add cache storage to a gateway.</p> </note></p>
    async fn create_tape_with_barcode(
        &self,
        input: CreateTapeWithBarcodeInput,
    ) -> Result<CreateTapeWithBarcodeOutput, RusotoError<CreateTapeWithBarcodeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.CreateTapeWithBarcode",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateTapeWithBarcodeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateTapeWithBarcodeOutput, _>()
    }

    /// <p><p>Creates one or more virtual tapes. You write data to the virtual tapes and then archive the tapes. This operation is only supported in the tape gateway type.</p> <note> <p>Cache storage must be allocated to the gateway before you can create virtual tapes. Use the <a>AddCache</a> operation to add cache storage to a gateway.</p> </note></p>
    async fn create_tapes(
        &self,
        input: CreateTapesInput,
    ) -> Result<CreateTapesOutput, RusotoError<CreateTapesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.CreateTapes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateTapesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateTapesOutput, _>()
    }

    /// <p>Deletes the automatic tape creation policy of a gateway. If you delete this policy, new virtual tapes must be created manually. Use the Amazon Resource Name (ARN) of the gateway in your request to remove the policy.</p>
    async fn delete_automatic_tape_creation_policy(
        &self,
        input: DeleteAutomaticTapeCreationPolicyInput,
    ) -> Result<
        DeleteAutomaticTapeCreationPolicyOutput,
        RusotoError<DeleteAutomaticTapeCreationPolicyError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DeleteAutomaticTapeCreationPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DeleteAutomaticTapeCreationPolicyError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteAutomaticTapeCreationPolicyOutput, _>()
    }

    /// <p>Deletes the bandwidth rate limits of a gateway. You can delete either the upload and download bandwidth rate limit, or you can delete both. If you delete only one of the limits, the other limit remains unchanged. To specify which gateway to work with, use the Amazon Resource Name (ARN) of the gateway in your request. This operation is supported for the stored volume, cached volume and tape gateway types.</p>
    async fn delete_bandwidth_rate_limit(
        &self,
        input: DeleteBandwidthRateLimitInput,
    ) -> Result<DeleteBandwidthRateLimitOutput, RusotoError<DeleteBandwidthRateLimitError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DeleteBandwidthRateLimit",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteBandwidthRateLimitError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteBandwidthRateLimitOutput, _>()
    }

    /// <p>Deletes Challenge-Handshake Authentication Protocol (CHAP) credentials for a specified iSCSI target and initiator pair. This operation is supported in volume and tape gateway types.</p>
    async fn delete_chap_credentials(
        &self,
        input: DeleteChapCredentialsInput,
    ) -> Result<DeleteChapCredentialsOutput, RusotoError<DeleteChapCredentialsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DeleteChapCredentials",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteChapCredentialsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteChapCredentialsOutput, _>()
    }

    /// <p>Deletes a file share from a file gateway. This operation is only supported for file gateways.</p>
    async fn delete_file_share(
        &self,
        input: DeleteFileShareInput,
    ) -> Result<DeleteFileShareOutput, RusotoError<DeleteFileShareError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.DeleteFileShare");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteFileShareError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteFileShareOutput, _>()
    }

    /// <p><p>Deletes a gateway. To specify which gateway to delete, use the Amazon Resource Name (ARN) of the gateway in your request. The operation deletes the gateway; however, it does not delete the gateway virtual machine (VM) from your host computer.</p> <p>After you delete a gateway, you cannot reactivate it. Completed snapshots of the gateway volumes are not deleted upon deleting the gateway, however, pending snapshots will not complete. After you delete a gateway, your next step is to remove it from your environment.</p> <important> <p>You no longer pay software charges after the gateway is deleted; however, your existing Amazon EBS snapshots persist and you will continue to be billed for these snapshots. You can choose to remove all remaining Amazon EBS snapshots by canceling your Amazon EC2 subscription.  If you prefer not to cancel your Amazon EC2 subscription, you can delete your snapshots using the Amazon EC2 console. For more information, see the <a href="http://aws.amazon.com/storagegateway">AWS Storage Gateway detail page</a>.</p> </important></p>
    async fn delete_gateway(
        &self,
        input: DeleteGatewayInput,
    ) -> Result<DeleteGatewayOutput, RusotoError<DeleteGatewayError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.DeleteGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteGatewayError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteGatewayOutput, _>()
    }

    /// <p><p>Deletes a snapshot of a volume.</p> <p>You can take snapshots of your gateway volumes on a scheduled or ad hoc basis. This API action enables you to delete a snapshot schedule for a volume. For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/backing-up-volumes.html">Backing up your volumes</a>. In the <code>DeleteSnapshotSchedule</code> request, you identify the volume by providing its Amazon Resource Name (ARN). This operation is only supported in stored and cached volume gateway types.</p> <note> <p>To list or delete a snapshot, you must use the Amazon EC2 API. For more information, go to <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSnapshots.html">DescribeSnapshots</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </note></p>
    async fn delete_snapshot_schedule(
        &self,
        input: DeleteSnapshotScheduleInput,
    ) -> Result<DeleteSnapshotScheduleOutput, RusotoError<DeleteSnapshotScheduleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DeleteSnapshotSchedule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteSnapshotScheduleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteSnapshotScheduleOutput, _>()
    }

    /// <p>Deletes the specified virtual tape. This operation is only supported in the tape gateway type.</p>
    async fn delete_tape(
        &self,
        input: DeleteTapeInput,
    ) -> Result<DeleteTapeOutput, RusotoError<DeleteTapeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.DeleteTape");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteTapeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteTapeOutput, _>()
    }

    /// <p>Deletes the specified virtual tape from the virtual tape shelf (VTS). This operation is only supported in the tape gateway type.</p>
    async fn delete_tape_archive(
        &self,
        input: DeleteTapeArchiveInput,
    ) -> Result<DeleteTapeArchiveOutput, RusotoError<DeleteTapeArchiveError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.DeleteTapeArchive");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteTapeArchiveError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteTapeArchiveOutput, _>()
    }

    /// <p>Delete a custom tape pool. A custom tape pool can only be deleted if there are no tapes in the pool and if there are no automatic tape creation policies that reference the custom tape pool.</p>
    async fn delete_tape_pool(
        &self,
        input: DeleteTapePoolInput,
    ) -> Result<DeleteTapePoolOutput, RusotoError<DeleteTapePoolError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.DeleteTapePool");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteTapePoolError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteTapePoolOutput, _>()
    }

    /// <p>Deletes the specified storage volume that you previously created using the <a>CreateCachediSCSIVolume</a> or <a>CreateStorediSCSIVolume</a> API. This operation is only supported in the cached volume and stored volume types. For stored volume gateways, the local disk that was configured as the storage volume is not deleted. You can reuse the local disk to create another storage volume.</p> <p>Before you delete a volume, make sure there are no iSCSI connections to the volume you are deleting. You should also make sure there is no snapshot in progress. You can use the Amazon Elastic Compute Cloud (Amazon EC2) API to query snapshots on the volume you are deleting and check the snapshot status. For more information, go to <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-DescribeSnapshots.html">DescribeSnapshots</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> <p>In the request, you must provide the Amazon Resource Name (ARN) of the storage volume you want to delete.</p>
    async fn delete_volume(
        &self,
        input: DeleteVolumeInput,
    ) -> Result<DeleteVolumeOutput, RusotoError<DeleteVolumeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.DeleteVolume");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteVolumeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteVolumeOutput, _>()
    }

    /// <p>Returns information about the most recent high availability monitoring test that was performed on the host in a cluster. If a test isn't performed, the status and start time in the response would be null.</p>
    async fn describe_availability_monitor_test(
        &self,
        input: DescribeAvailabilityMonitorTestInput,
    ) -> Result<
        DescribeAvailabilityMonitorTestOutput,
        RusotoError<DescribeAvailabilityMonitorTestError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeAvailabilityMonitorTest",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeAvailabilityMonitorTestError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeAvailabilityMonitorTestOutput, _>()
    }

    /// <p>Returns the bandwidth rate limits of a gateway. By default, these limits are not set, which means no bandwidth rate limiting is in effect. This operation is supported for the stored volume, cached volume, and tape gateway types.</p> <p>This operation only returns a value for a bandwidth rate limit only if the limit is set. If no limits are set for the gateway, then this operation returns only the gateway ARN in the response body. To specify which gateway to describe, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    async fn describe_bandwidth_rate_limit(
        &self,
        input: DescribeBandwidthRateLimitInput,
    ) -> Result<DescribeBandwidthRateLimitOutput, RusotoError<DescribeBandwidthRateLimitError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeBandwidthRateLimit",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeBandwidthRateLimitError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeBandwidthRateLimitOutput, _>()
    }

    /// <p> Returns information about the bandwidth rate limit schedule of a gateway. By default, gateways do not have bandwidth rate limit schedules, which means no bandwidth rate limiting is in effect. This operation is supported only in the volume and tape gateway types. </p> <p>This operation returns information about a gateway's bandwidth rate limit schedule. A bandwidth rate limit schedule consists of one or more bandwidth rate limit intervals. A bandwidth rate limit interval defines a period of time on one or more days of the week, during which bandwidth rate limits are specified for uploading, downloading, or both. </p> <p> A bandwidth rate limit interval consists of one or more days of the week, a start hour and minute, an ending hour and minute, and bandwidth rate limits for uploading and downloading </p> <p> If no bandwidth rate limit schedule intervals are set for the gateway, this operation returns an empty response. To specify which gateway to describe, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    async fn describe_bandwidth_rate_limit_schedule(
        &self,
        input: DescribeBandwidthRateLimitScheduleInput,
    ) -> Result<
        DescribeBandwidthRateLimitScheduleOutput,
        RusotoError<DescribeBandwidthRateLimitScheduleError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeBandwidthRateLimitSchedule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DescribeBandwidthRateLimitScheduleError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeBandwidthRateLimitScheduleOutput, _>()
    }

    /// <p>Returns information about the cache of a gateway. This operation is only supported in the cached volume, tape, and file gateway types.</p> <p>The response includes disk IDs that are configured as cache, and it includes the amount of cache allocated and used.</p>
    async fn describe_cache(
        &self,
        input: DescribeCacheInput,
    ) -> Result<DescribeCacheOutput, RusotoError<DescribeCacheError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.DescribeCache");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeCacheError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeCacheOutput, _>()
    }

    /// <p>Returns a description of the gateway volumes specified in the request. This operation is only supported in the cached volume gateway types.</p> <p>The list of gateway volumes in the request must be from one gateway. In the response, AWS Storage Gateway returns volume information sorted by volume Amazon Resource Name (ARN).</p>
    async fn describe_cachedi_scsi_volumes(
        &self,
        input: DescribeCachediSCSIVolumesInput,
    ) -> Result<DescribeCachediSCSIVolumesOutput, RusotoError<DescribeCachediSCSIVolumesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeCachediSCSIVolumes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeCachediSCSIVolumesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeCachediSCSIVolumesOutput, _>()
    }

    /// <p>Returns an array of Challenge-Handshake Authentication Protocol (CHAP) credentials information for a specified iSCSI target, one for each target-initiator pair. This operation is supported in the volume and tape gateway types.</p>
    async fn describe_chap_credentials(
        &self,
        input: DescribeChapCredentialsInput,
    ) -> Result<DescribeChapCredentialsOutput, RusotoError<DescribeChapCredentialsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeChapCredentials",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeChapCredentialsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeChapCredentialsOutput, _>()
    }

    /// <p>Gets the file system association information. This operation is only supported for Amazon FSx file gateways.</p>
    async fn describe_file_system_associations(
        &self,
        input: DescribeFileSystemAssociationsInput,
    ) -> Result<
        DescribeFileSystemAssociationsOutput,
        RusotoError<DescribeFileSystemAssociationsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeFileSystemAssociations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeFileSystemAssociationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeFileSystemAssociationsOutput, _>()
    }

    /// <p>Returns metadata about a gateway such as its name, network interfaces, configured time zone, and the state (whether the gateway is running or not). To specify which gateway to describe, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    async fn describe_gateway_information(
        &self,
        input: DescribeGatewayInformationInput,
    ) -> Result<DescribeGatewayInformationOutput, RusotoError<DescribeGatewayInformationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeGatewayInformation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeGatewayInformationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeGatewayInformationOutput, _>()
    }

    /// <p>Returns your gateway's weekly maintenance start time including the day and time of the week. Note that values are in terms of the gateway's time zone.</p>
    async fn describe_maintenance_start_time(
        &self,
        input: DescribeMaintenanceStartTimeInput,
    ) -> Result<DescribeMaintenanceStartTimeOutput, RusotoError<DescribeMaintenanceStartTimeError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeMaintenanceStartTime",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeMaintenanceStartTimeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeMaintenanceStartTimeOutput, _>()
    }

    /// <p>Gets a description for one or more Network File System (NFS) file shares from a file gateway. This operation is only supported for file gateways.</p>
    async fn describe_nfs_file_shares(
        &self,
        input: DescribeNFSFileSharesInput,
    ) -> Result<DescribeNFSFileSharesOutput, RusotoError<DescribeNFSFileSharesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeNFSFileShares",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeNFSFileSharesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeNFSFileSharesOutput, _>()
    }

    /// <p>Gets a description for one or more Server Message Block (SMB) file shares from a file gateway. This operation is only supported for file gateways.</p>
    async fn describe_smb_file_shares(
        &self,
        input: DescribeSMBFileSharesInput,
    ) -> Result<DescribeSMBFileSharesOutput, RusotoError<DescribeSMBFileSharesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeSMBFileShares",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeSMBFileSharesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeSMBFileSharesOutput, _>()
    }

    /// <p>Gets a description of a Server Message Block (SMB) file share settings from a file gateway. This operation is only supported for file gateways.</p>
    async fn describe_smb_settings(
        &self,
        input: DescribeSMBSettingsInput,
    ) -> Result<DescribeSMBSettingsOutput, RusotoError<DescribeSMBSettingsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeSMBSettings",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeSMBSettingsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeSMBSettingsOutput, _>()
    }

    /// <p>Describes the snapshot schedule for the specified gateway volume. The snapshot schedule information includes intervals at which snapshots are automatically initiated on the volume. This operation is only supported in the cached volume and stored volume types.</p>
    async fn describe_snapshot_schedule(
        &self,
        input: DescribeSnapshotScheduleInput,
    ) -> Result<DescribeSnapshotScheduleOutput, RusotoError<DescribeSnapshotScheduleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeSnapshotSchedule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeSnapshotScheduleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeSnapshotScheduleOutput, _>()
    }

    /// <p>Returns the description of the gateway volumes specified in the request. The list of gateway volumes in the request must be from one gateway. In the response, AWS Storage Gateway returns volume information sorted by volume ARNs. This operation is only supported in stored volume gateway type.</p>
    async fn describe_storedi_scsi_volumes(
        &self,
        input: DescribeStorediSCSIVolumesInput,
    ) -> Result<DescribeStorediSCSIVolumesOutput, RusotoError<DescribeStorediSCSIVolumesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeStorediSCSIVolumes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeStorediSCSIVolumesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeStorediSCSIVolumesOutput, _>()
    }

    /// <p>Returns a description of specified virtual tapes in the virtual tape shelf (VTS). This operation is only supported in the tape gateway type.</p> <p>If a specific <code>TapeARN</code> is not specified, AWS Storage Gateway returns a description of all virtual tapes found in the VTS associated with your account.</p>
    async fn describe_tape_archives(
        &self,
        input: DescribeTapeArchivesInput,
    ) -> Result<DescribeTapeArchivesOutput, RusotoError<DescribeTapeArchivesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeTapeArchives",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeTapeArchivesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeTapeArchivesOutput, _>()
    }

    /// <p>Returns a list of virtual tape recovery points that are available for the specified tape gateway.</p> <p>A recovery point is a point-in-time view of a virtual tape at which all the data on the virtual tape is consistent. If your gateway crashes, virtual tapes that have recovery points can be recovered to a new gateway. This operation is only supported in the tape gateway type.</p>
    async fn describe_tape_recovery_points(
        &self,
        input: DescribeTapeRecoveryPointsInput,
    ) -> Result<DescribeTapeRecoveryPointsOutput, RusotoError<DescribeTapeRecoveryPointsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeTapeRecoveryPoints",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeTapeRecoveryPointsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeTapeRecoveryPointsOutput, _>()
    }

    /// <p>Returns a description of the specified Amazon Resource Name (ARN) of virtual tapes. If a <code>TapeARN</code> is not specified, returns a description of all virtual tapes associated with the specified gateway. This operation is only supported in the tape gateway type.</p>
    async fn describe_tapes(
        &self,
        input: DescribeTapesInput,
    ) -> Result<DescribeTapesOutput, RusotoError<DescribeTapesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.DescribeTapes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeTapesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeTapesOutput, _>()
    }

    /// <p>Returns information about the upload buffer of a gateway. This operation is supported for the stored volume, cached volume, and tape gateway types.</p> <p>The response includes disk IDs that are configured as upload buffer space, and it includes the amount of upload buffer space allocated and used.</p>
    async fn describe_upload_buffer(
        &self,
        input: DescribeUploadBufferInput,
    ) -> Result<DescribeUploadBufferOutput, RusotoError<DescribeUploadBufferError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeUploadBuffer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeUploadBufferError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeUploadBufferOutput, _>()
    }

    /// <p>Returns a description of virtual tape library (VTL) devices for the specified tape gateway. In the response, AWS Storage Gateway returns VTL device information.</p> <p>This operation is only supported in the tape gateway type.</p>
    async fn describe_vtl_devices(
        &self,
        input: DescribeVTLDevicesInput,
    ) -> Result<DescribeVTLDevicesOutput, RusotoError<DescribeVTLDevicesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.DescribeVTLDevices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeVTLDevicesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeVTLDevicesOutput, _>()
    }

    /// <p>Returns information about the working storage of a gateway. This operation is only supported in the stored volumes gateway type. This operation is deprecated in cached volumes API version (20120630). Use DescribeUploadBuffer instead.</p> <note> <p>Working storage is also referred to as upload buffer. You can also use the DescribeUploadBuffer operation to add upload buffer to a stored volume gateway.</p> </note> <p>The response includes disk IDs that are configured as working storage, and it includes the amount of working storage allocated and used.</p>
    async fn describe_working_storage(
        &self,
        input: DescribeWorkingStorageInput,
    ) -> Result<DescribeWorkingStorageOutput, RusotoError<DescribeWorkingStorageError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DescribeWorkingStorage",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeWorkingStorageError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeWorkingStorageOutput, _>()
    }

    /// <p>Disconnects a volume from an iSCSI connection and then detaches the volume from the specified gateway. Detaching and attaching a volume enables you to recover your data from one gateway to a different gateway without creating a snapshot. It also makes it easier to move your volumes from an on-premises gateway to a gateway hosted on an Amazon EC2 instance. This operation is only supported in the volume gateway type.</p>
    async fn detach_volume(
        &self,
        input: DetachVolumeInput,
    ) -> Result<DetachVolumeOutput, RusotoError<DetachVolumeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.DetachVolume");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DetachVolumeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DetachVolumeOutput, _>()
    }

    /// <p><p>Disables a tape gateway when the gateway is no longer functioning. For example, if your gateway VM is damaged, you can disable the gateway so you can recover virtual tapes.</p> <p>Use this operation for a tape gateway that is not reachable or not functioning. This operation is only supported in the tape gateway type.</p> <important> <p>After a gateway is disabled, it cannot be enabled.</p> </important></p>
    async fn disable_gateway(
        &self,
        input: DisableGatewayInput,
    ) -> Result<DisableGatewayOutput, RusotoError<DisableGatewayError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.DisableGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisableGatewayError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DisableGatewayOutput, _>()
    }

    /// <p>Disassociates an Amazon FSx file system from the specified gateway. After the disassociation process finishes, the gateway can no longer access the Amazon FSx file system. This operation is only supported in the Amazon FSx file gateway type.</p>
    async fn disassociate_file_system(
        &self,
        input: DisassociateFileSystemInput,
    ) -> Result<DisassociateFileSystemOutput, RusotoError<DisassociateFileSystemError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.DisassociateFileSystem",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DisassociateFileSystemError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DisassociateFileSystemOutput, _>()
    }

    /// <p>Adds a file gateway to an Active Directory domain. This operation is only supported for file gateways that support the SMB file protocol.</p>
    async fn join_domain(
        &self,
        input: JoinDomainInput,
    ) -> Result<JoinDomainOutput, RusotoError<JoinDomainError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.JoinDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, JoinDomainError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<JoinDomainOutput, _>()
    }

    /// <p>Lists the automatic tape creation policies for a gateway. If there are no automatic tape creation policies for the gateway, it returns an empty list.</p> <p>This operation is only supported for tape gateways.</p>
    async fn list_automatic_tape_creation_policies(
        &self,
        input: ListAutomaticTapeCreationPoliciesInput,
    ) -> Result<
        ListAutomaticTapeCreationPoliciesOutput,
        RusotoError<ListAutomaticTapeCreationPoliciesError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.ListAutomaticTapeCreationPolicies",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                ListAutomaticTapeCreationPoliciesError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListAutomaticTapeCreationPoliciesOutput, _>()
    }

    /// <p>Gets a list of the file shares for a specific file gateway, or the list of file shares that belong to the calling user account. This operation is only supported for file gateways.</p>
    async fn list_file_shares(
        &self,
        input: ListFileSharesInput,
    ) -> Result<ListFileSharesOutput, RusotoError<ListFileSharesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.ListFileShares");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListFileSharesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListFileSharesOutput, _>()
    }

    /// <p>Gets a list of <code>FileSystemAssociationSummary</code> objects. Each object contains a summary of a file system association. This operation is only supported for Amazon FSx file gateways.</p>
    async fn list_file_system_associations(
        &self,
        input: ListFileSystemAssociationsInput,
    ) -> Result<ListFileSystemAssociationsOutput, RusotoError<ListFileSystemAssociationsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.ListFileSystemAssociations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListFileSystemAssociationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListFileSystemAssociationsOutput, _>()
    }

    /// <p>Lists gateways owned by an AWS account in an AWS Region specified in the request. The returned list is ordered by gateway Amazon Resource Name (ARN).</p> <p>By default, the operation returns a maximum of 100 gateways. This operation supports pagination that allows you to optionally reduce the number of gateways returned in a response.</p> <p>If you have more gateways than are returned in a response (that is, the response returns only a truncated list of your gateways), the response contains a marker that you can specify in your next request to fetch the next page of gateways.</p>
    async fn list_gateways(
        &self,
        input: ListGatewaysInput,
    ) -> Result<ListGatewaysOutput, RusotoError<ListGatewaysError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.ListGateways");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListGatewaysError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListGatewaysOutput, _>()
    }

    /// <p>Returns a list of the gateway's local disks. To specify which gateway to describe, you use the Amazon Resource Name (ARN) of the gateway in the body of the request.</p> <p>The request returns a list of all disks, specifying which are configured as working storage, cache storage, or stored volume or not configured at all. The response includes a <code>DiskStatus</code> field. This field can have a value of present (the disk is available to use), missing (the disk is no longer connected to the gateway), or mismatch (the disk node is occupied by a disk that has incorrect metadata or the disk content is corrupted).</p>
    async fn list_local_disks(
        &self,
        input: ListLocalDisksInput,
    ) -> Result<ListLocalDisksOutput, RusotoError<ListLocalDisksError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.ListLocalDisks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListLocalDisksError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListLocalDisksOutput, _>()
    }

    /// <p>Lists the tags that have been added to the specified resource. This operation is supported in storage gateways of all types.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceInput,
    ) -> Result<ListTagsForResourceOutput, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceOutput, _>()
    }

    /// <p>Lists custom tape pools. You specify custom tape pools to list by specifying one or more custom tape pool Amazon Resource Names (ARNs). If you don't specify a custom tape pool ARN, the operation lists all custom tape pools.</p> <p>This operation supports pagination. You can optionally specify the <code>Limit</code> parameter in the body to limit the number of tape pools in the response. If the number of tape pools returned in the response is truncated, the response includes a <code>Marker</code> element that you can use in your subsequent request to retrieve the next set of tape pools.</p>
    async fn list_tape_pools(
        &self,
        input: ListTapePoolsInput,
    ) -> Result<ListTapePoolsOutput, RusotoError<ListTapePoolsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.ListTapePools");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTapePoolsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTapePoolsOutput, _>()
    }

    /// <p>Lists virtual tapes in your virtual tape library (VTL) and your virtual tape shelf (VTS). You specify the tapes to list by specifying one or more tape Amazon Resource Names (ARNs). If you don't specify a tape ARN, the operation lists all virtual tapes in both your VTL and VTS.</p> <p>This operation supports pagination. By default, the operation returns a maximum of up to 100 tapes. You can optionally specify the <code>Limit</code> parameter in the body to limit the number of tapes in the response. If the number of tapes returned in the response is truncated, the response includes a <code>Marker</code> element that you can use in your subsequent request to retrieve the next set of tapes. This operation is only supported in the tape gateway type.</p>
    async fn list_tapes(
        &self,
        input: ListTapesInput,
    ) -> Result<ListTapesOutput, RusotoError<ListTapesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.ListTapes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTapesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTapesOutput, _>()
    }

    /// <p>Lists iSCSI initiators that are connected to a volume. You can use this operation to determine whether a volume is being used or not. This operation is only supported in the cached volume and stored volume gateway types.</p>
    async fn list_volume_initiators(
        &self,
        input: ListVolumeInitiatorsInput,
    ) -> Result<ListVolumeInitiatorsOutput, RusotoError<ListVolumeInitiatorsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.ListVolumeInitiators",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListVolumeInitiatorsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListVolumeInitiatorsOutput, _>()
    }

    /// <p>Lists the recovery points for a specified gateway. This operation is only supported in the cached volume gateway type.</p> <p>Each cache volume has one recovery point. A volume recovery point is a point in time at which all data of the volume is consistent and from which you can create a snapshot or clone a new cached volume from a source volume. To create a snapshot from a volume recovery point use the <a>CreateSnapshotFromVolumeRecoveryPoint</a> operation.</p>
    async fn list_volume_recovery_points(
        &self,
        input: ListVolumeRecoveryPointsInput,
    ) -> Result<ListVolumeRecoveryPointsOutput, RusotoError<ListVolumeRecoveryPointsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.ListVolumeRecoveryPoints",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListVolumeRecoveryPointsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListVolumeRecoveryPointsOutput, _>()
    }

    /// <p>Lists the iSCSI stored volumes of a gateway. Results are sorted by volume ARN. The response includes only the volume ARNs. If you want additional volume information, use the <a>DescribeStorediSCSIVolumes</a> or the <a>DescribeCachediSCSIVolumes</a> API.</p> <p>The operation supports pagination. By default, the operation returns a maximum of up to 100 volumes. You can optionally specify the <code>Limit</code> field in the body to limit the number of volumes in the response. If the number of volumes returned in the response is truncated, the response includes a Marker field. You can use this Marker value in your subsequent request to retrieve the next set of volumes. This operation is only supported in the cached volume and stored volume gateway types.</p>
    async fn list_volumes(
        &self,
        input: ListVolumesInput,
    ) -> Result<ListVolumesOutput, RusotoError<ListVolumesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.ListVolumes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListVolumesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListVolumesOutput, _>()
    }

    /// <p>Sends you notification through CloudWatch Events when all files written to your file share have been uploaded to Amazon S3.</p> <p>AWS Storage Gateway can send a notification through Amazon CloudWatch Events when all files written to your file share up to that point in time have been uploaded to Amazon S3. These files include files written to the file share up to the time that you make a request for notification. When the upload is done, Storage Gateway sends you notification through an Amazon CloudWatch Event. You can configure CloudWatch Events to send the notification through event targets such as Amazon SNS or AWS Lambda function. This operation is only supported for file gateways.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/monitoring-file-gateway.html#get-upload-notification">Getting file upload notification</a> in the <i>AWS Storage Gateway User Guide</i>.</p>
    async fn notify_when_uploaded(
        &self,
        input: NotifyWhenUploadedInput,
    ) -> Result<NotifyWhenUploadedOutput, RusotoError<NotifyWhenUploadedError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.NotifyWhenUploaded");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, NotifyWhenUploadedError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<NotifyWhenUploadedOutput, _>()
    }

    /// <p>Refreshes the cached inventory of objects for the specified file share. This operation finds objects in the Amazon S3 bucket that were added, removed, or replaced since the gateway last listed the bucket's contents and cached the results. This operation does not import files into the file gateway cache storage. It only updates the cached inventory to reflect changes in the inventory of the objects in the S3 bucket. This operation is only supported in the file gateway type. You can subscribe to be notified through an Amazon CloudWatch event when your <code>RefreshCache</code> operation completes. For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/monitoring-file-gateway.html#get-notification">Getting notified about file operations</a> in the <i>AWS Storage Gateway User Guide</i>.</p> <p>When this API is called, it only initiates the refresh operation. When the API call completes and returns a success code, it doesn't necessarily mean that the file refresh has completed. You should use the refresh-complete notification to determine that the operation has completed before you check for new files on the gateway file share. You can subscribe to be notified through a CloudWatch event when your <code>RefreshCache</code> operation completes.</p> <p>Throttle limit: This API is asynchronous, so the gateway will accept no more than two refreshes at any time. We recommend using the refresh-complete CloudWatch event notification before issuing additional requests. For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/monitoring-file-gateway.html#get-notification">Getting notified about file operations</a> in the <i>AWS Storage Gateway User Guide</i>.</p> <p>If you invoke the RefreshCache API when two requests are already being processed, any new request will cause an <code>InvalidGatewayRequestException</code> error because too many requests were sent to the server.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/monitoring-file-gateway.html#get-notification">Getting notified about file operations</a> in the <i>AWS Storage Gateway User Guide</i>.</p>
    async fn refresh_cache(
        &self,
        input: RefreshCacheInput,
    ) -> Result<RefreshCacheOutput, RusotoError<RefreshCacheError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.RefreshCache");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RefreshCacheError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RefreshCacheOutput, _>()
    }

    /// <p>Removes one or more tags from the specified resource. This operation is supported in storage gateways of all types.</p>
    async fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceInput,
    ) -> Result<RemoveTagsFromResourceOutput, RusotoError<RemoveTagsFromResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.RemoveTagsFromResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RemoveTagsFromResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<RemoveTagsFromResourceOutput, _>()
    }

    /// <p><p>Resets all cache disks that have encountered an error and makes the disks available for reconfiguration as cache storage. If your cache disk encounters an error, the gateway prevents read and write operations on virtual tapes in the gateway. For example, an error can occur when a disk is corrupted or removed from the gateway. When a cache is reset, the gateway loses its cache storage. At this point, you can reconfigure the disks as cache disks. This operation is only supported in the cached volume and tape types.</p> <important> <p>If the cache disk you are resetting contains data that has not been uploaded to Amazon S3 yet, that data can be lost. After you reset cache disks, there will be no configured cache disks left in the gateway, so you must configure at least one new cache disk for your gateway to function properly.</p> </important></p>
    async fn reset_cache(
        &self,
        input: ResetCacheInput,
    ) -> Result<ResetCacheOutput, RusotoError<ResetCacheError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.ResetCache");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ResetCacheError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ResetCacheOutput, _>()
    }

    /// <p>Retrieves an archived virtual tape from the virtual tape shelf (VTS) to a tape gateway. Virtual tapes archived in the VTS are not associated with any gateway. However after a tape is retrieved, it is associated with a gateway, even though it is also listed in the VTS, that is, archive. This operation is only supported in the tape gateway type.</p> <p>Once a tape is successfully retrieved to a gateway, it cannot be retrieved again to another gateway. You must archive the tape again before you can retrieve it to another gateway. This operation is only supported in the tape gateway type.</p>
    async fn retrieve_tape_archive(
        &self,
        input: RetrieveTapeArchiveInput,
    ) -> Result<RetrieveTapeArchiveOutput, RusotoError<RetrieveTapeArchiveError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.RetrieveTapeArchive",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RetrieveTapeArchiveError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RetrieveTapeArchiveOutput, _>()
    }

    /// <p><p>Retrieves the recovery point for the specified virtual tape. This operation is only supported in the tape gateway type.</p> <p>A recovery point is a point in time view of a virtual tape at which all the data on the tape is consistent. If your gateway crashes, virtual tapes that have recovery points can be recovered to a new gateway.</p> <note> <p>The virtual tape can be retrieved to only one gateway. The retrieved tape is read-only. The virtual tape can be retrieved to only a tape gateway. There is no charge for retrieving recovery points.</p> </note></p>
    async fn retrieve_tape_recovery_point(
        &self,
        input: RetrieveTapeRecoveryPointInput,
    ) -> Result<RetrieveTapeRecoveryPointOutput, RusotoError<RetrieveTapeRecoveryPointError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.RetrieveTapeRecoveryPoint",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RetrieveTapeRecoveryPointError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<RetrieveTapeRecoveryPointOutput, _>()
    }

    /// <p>Sets the password for your VM local console. When you log in to the local console for the first time, you log in to the VM with the default credentials. We recommend that you set a new password. You don't need to know the default password to set a new password.</p>
    async fn set_local_console_password(
        &self,
        input: SetLocalConsolePasswordInput,
    ) -> Result<SetLocalConsolePasswordOutput, RusotoError<SetLocalConsolePasswordError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.SetLocalConsolePassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SetLocalConsolePasswordError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<SetLocalConsolePasswordOutput, _>()
    }

    /// <p>Sets the password for the guest user <code>smbguest</code>. The <code>smbguest</code> user is the user when the authentication method for the file share is set to <code>GuestAccess</code>.</p>
    async fn set_smb_guest_password(
        &self,
        input: SetSMBGuestPasswordInput,
    ) -> Result<SetSMBGuestPasswordOutput, RusotoError<SetSMBGuestPasswordError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.SetSMBGuestPassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, SetSMBGuestPasswordError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<SetSMBGuestPasswordOutput, _>()
    }

    /// <p>Shuts down a gateway. To specify which gateway to shut down, use the Amazon Resource Name (ARN) of the gateway in the body of your request.</p> <p>The operation shuts down the gateway service component running in the gateway's virtual machine (VM) and not the host VM.</p> <note> <p>If you want to shut down the VM, it is recommended that you first shut down the gateway component in the VM to avoid unpredictable conditions.</p> </note> <p>After the gateway is shutdown, you cannot call any other API except <a>StartGateway</a>, <a>DescribeGatewayInformation</a>, and <a>ListGateways</a>. For more information, see <a>ActivateGateway</a>. Your applications cannot read from or write to the gateway's storage volumes, and there are no snapshots taken.</p> <note> <p>When you make a shutdown request, you will get a <code>200 OK</code> success response immediately. However, it might take some time for the gateway to shut down. You can call the <a>DescribeGatewayInformation</a> API to check the status. For more information, see <a>ActivateGateway</a>.</p> </note> <p>If do not intend to use the gateway again, you must delete the gateway (using <a>DeleteGateway</a>) to no longer pay software charges associated with the gateway.</p>
    async fn shutdown_gateway(
        &self,
        input: ShutdownGatewayInput,
    ) -> Result<ShutdownGatewayOutput, RusotoError<ShutdownGatewayError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.ShutdownGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ShutdownGatewayError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ShutdownGatewayOutput, _>()
    }

    /// <p><p>Start a test that verifies that the specified gateway is configured for High Availability monitoring in your host environment. This request only initiates the test and that a successful response only indicates that the test was started. It doesn&#39;t indicate that the test passed. For the status of the test, invoke the <code>DescribeAvailabilityMonitorTest</code> API.</p> <note> <p>Starting this test will cause your gateway to go offline for a brief period.</p> </note></p>
    async fn start_availability_monitor_test(
        &self,
        input: StartAvailabilityMonitorTestInput,
    ) -> Result<StartAvailabilityMonitorTestOutput, RusotoError<StartAvailabilityMonitorTestError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.StartAvailabilityMonitorTest",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartAvailabilityMonitorTestError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartAvailabilityMonitorTestOutput, _>()
    }

    /// <p>Starts a gateway that you previously shut down (see <a>ShutdownGateway</a>). After the gateway starts, you can then make other API calls, your applications can read from or write to the gateway's storage volumes and you will be able to take snapshot backups.</p> <note> <p>When you make a request, you will get a 200 OK success response immediately. However, it might take some time for the gateway to be ready. You should call <a>DescribeGatewayInformation</a> and check the status before making any additional API calls. For more information, see <a>ActivateGateway</a>.</p> </note> <p>To specify which gateway to start, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    async fn start_gateway(
        &self,
        input: StartGatewayInput,
    ) -> Result<StartGatewayOutput, RusotoError<StartGatewayError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.StartGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartGatewayError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StartGatewayOutput, _>()
    }

    /// <p><p>Updates the automatic tape creation policy of a gateway. Use this to update the policy with a new set of automatic tape creation rules. This is only supported for tape gateways.</p> <p>By default, there is no automatic tape creation policy.</p> <note> <p>A gateway can have only one automatic tape creation policy.</p> </note></p>
    async fn update_automatic_tape_creation_policy(
        &self,
        input: UpdateAutomaticTapeCreationPolicyInput,
    ) -> Result<
        UpdateAutomaticTapeCreationPolicyOutput,
        RusotoError<UpdateAutomaticTapeCreationPolicyError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateAutomaticTapeCreationPolicy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                UpdateAutomaticTapeCreationPolicyError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateAutomaticTapeCreationPolicyOutput, _>()
    }

    /// <p>Updates the bandwidth rate limits of a gateway. You can update both the upload and download bandwidth rate limit or specify only one of the two. If you don't set a bandwidth rate limit, the existing rate limit remains. This operation is supported for the stored volume, cached volume, and tape gateway types.</p> <p>By default, a gateway's bandwidth rate limits are not set. If you don't set any limit, the gateway does not have any limitations on its bandwidth usage and could potentially use the maximum available bandwidth.</p> <p>To specify which gateway to update, use the Amazon Resource Name (ARN) of the gateway in your request.</p>
    async fn update_bandwidth_rate_limit(
        &self,
        input: UpdateBandwidthRateLimitInput,
    ) -> Result<UpdateBandwidthRateLimitOutput, RusotoError<UpdateBandwidthRateLimitError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateBandwidthRateLimit",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateBandwidthRateLimitError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateBandwidthRateLimitOutput, _>()
    }

    /// <p> Updates the bandwidth rate limit schedule for a specified gateway. By default, gateways do not have bandwidth rate limit schedules, which means no bandwidth rate limiting is in effect. Use this to initiate or update a gateway's bandwidth rate limit schedule. This operation is supported in the volume and tape gateway types. </p>
    async fn update_bandwidth_rate_limit_schedule(
        &self,
        input: UpdateBandwidthRateLimitScheduleInput,
    ) -> Result<
        UpdateBandwidthRateLimitScheduleOutput,
        RusotoError<UpdateBandwidthRateLimitScheduleError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateBandwidthRateLimitSchedule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                UpdateBandwidthRateLimitScheduleError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateBandwidthRateLimitScheduleOutput, _>()
    }

    /// <p><p>Updates the Challenge-Handshake Authentication Protocol (CHAP) credentials for a specified iSCSI target. By default, a gateway does not have CHAP enabled; however, for added security, you might use it. This operation is supported in the volume and tape gateway types.</p> <important> <p>When you update CHAP credentials, all existing connections on the target are closed and initiators must reconnect with the new credentials.</p> </important></p>
    async fn update_chap_credentials(
        &self,
        input: UpdateChapCredentialsInput,
    ) -> Result<UpdateChapCredentialsOutput, RusotoError<UpdateChapCredentialsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateChapCredentials",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateChapCredentialsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateChapCredentialsOutput, _>()
    }

    /// <p>Updates a file system association. This operation is only supported in the Amazon FSx file gateway type.</p>
    async fn update_file_system_association(
        &self,
        input: UpdateFileSystemAssociationInput,
    ) -> Result<UpdateFileSystemAssociationOutput, RusotoError<UpdateFileSystemAssociationError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateFileSystemAssociation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateFileSystemAssociationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateFileSystemAssociationOutput, _>()
    }

    /// <p><p>Updates a gateway&#39;s metadata, which includes the gateway&#39;s name and time zone. To specify which gateway to update, use the Amazon Resource Name (ARN) of the gateway in your request.</p> <note> <p>For gateways activated after September 2, 2015, the gateway&#39;s ARN contains the gateway ID rather than the gateway name. However, changing the name of the gateway has no effect on the gateway&#39;s ARN.</p> </note></p>
    async fn update_gateway_information(
        &self,
        input: UpdateGatewayInformationInput,
    ) -> Result<UpdateGatewayInformationOutput, RusotoError<UpdateGatewayInformationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateGatewayInformation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateGatewayInformationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateGatewayInformationOutput, _>()
    }

    /// <p><p>Updates the gateway virtual machine (VM) software. The request immediately triggers the software update.</p> <note> <p>When you make this request, you get a <code>200 OK</code> success response immediately. However, it might take some time for the update to complete. You can call <a>DescribeGatewayInformation</a> to verify the gateway is in the <code>STATE_RUNNING</code> state.</p> </note> <important> <p>A software update forces a system restart of your gateway. You can minimize the chance of any disruption to your applications by increasing your iSCSI Initiators&#39; timeouts. For more information about increasing iSCSI Initiator timeouts for Windows and Linux, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/ConfiguringiSCSIClientInitiatorWindowsClient.html#CustomizeWindowsiSCSISettings">Customizing your Windows iSCSI settings</a> and <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/ConfiguringiSCSIClientInitiatorRedHatClient.html#CustomizeLinuxiSCSISettings">Customizing your Linux iSCSI settings</a>, respectively.</p> </important></p>
    async fn update_gateway_software_now(
        &self,
        input: UpdateGatewaySoftwareNowInput,
    ) -> Result<UpdateGatewaySoftwareNowOutput, RusotoError<UpdateGatewaySoftwareNowError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateGatewaySoftwareNow",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateGatewaySoftwareNowError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateGatewaySoftwareNowOutput, _>()
    }

    /// <p>Updates a gateway's weekly maintenance start time information, including day and time of the week. The maintenance time is the time in your gateway's time zone.</p>
    async fn update_maintenance_start_time(
        &self,
        input: UpdateMaintenanceStartTimeInput,
    ) -> Result<UpdateMaintenanceStartTimeOutput, RusotoError<UpdateMaintenanceStartTimeError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateMaintenanceStartTime",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateMaintenanceStartTimeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateMaintenanceStartTimeOutput, _>()
    }

    /// <p><p>Updates a Network File System (NFS) file share. This operation is only supported in the file gateway type.</p> <note> <p>To leave a file share field unchanged, set the corresponding input field to null.</p> </note> <p>Updates the following file share settings:</p> <ul> <li> <p>Default storage class for your S3 bucket</p> </li> <li> <p>Metadata defaults for your S3 bucket</p> </li> <li> <p>Allowed NFS clients for your file share</p> </li> <li> <p>Squash settings</p> </li> <li> <p>Write status of your file share</p> </li> </ul></p>
    async fn update_nfs_file_share(
        &self,
        input: UpdateNFSFileShareInput,
    ) -> Result<UpdateNFSFileShareOutput, RusotoError<UpdateNFSFileShareError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.UpdateNFSFileShare");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateNFSFileShareError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateNFSFileShareOutput, _>()
    }

    /// <p><p>Updates a Server Message Block (SMB) file share. This operation is only supported for file gateways.</p> <note> <p>To leave a file share field unchanged, set the corresponding input field to null.</p> </note> <important> <p>File gateways require AWS Security Token Service (AWS STS) to be activated to enable you to create a file share. Make sure that AWS STS is activated in the AWS Region you are creating your file gateway in. If AWS STS is not activated in this AWS Region, activate it. For information about how to activate AWS STS, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and deactivating AWS STS in an AWS Region</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> <p>File gateways don&#39;t support creating hard or symbolic links on a file share.</p> </important></p>
    async fn update_smb_file_share(
        &self,
        input: UpdateSMBFileShareInput,
    ) -> Result<UpdateSMBFileShareOutput, RusotoError<UpdateSMBFileShareError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "StorageGateway_20130630.UpdateSMBFileShare");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateSMBFileShareError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateSMBFileShareOutput, _>()
    }

    /// <p>Controls whether the shares on a gateway are visible in a net view or browse list.</p>
    async fn update_smb_file_share_visibility(
        &self,
        input: UpdateSMBFileShareVisibilityInput,
    ) -> Result<UpdateSMBFileShareVisibilityOutput, RusotoError<UpdateSMBFileShareVisibilityError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateSMBFileShareVisibility",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateSMBFileShareVisibilityError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateSMBFileShareVisibilityOutput, _>()
    }

    /// <p><p>Updates the SMB security strategy on a file gateway. This action is only supported in file gateways.</p> <note> <p>This API is called Security level in the User Guide.</p> <p>A higher security level can affect performance of the gateway.</p> </note></p>
    async fn update_smb_security_strategy(
        &self,
        input: UpdateSMBSecurityStrategyInput,
    ) -> Result<UpdateSMBSecurityStrategyOutput, RusotoError<UpdateSMBSecurityStrategyError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateSMBSecurityStrategy",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateSMBSecurityStrategyError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateSMBSecurityStrategyOutput, _>()
    }

    /// <p>Updates a snapshot schedule configured for a gateway volume. This operation is only supported in the cached volume and stored volume gateway types.</p> <p>The default snapshot schedule for volume is once every 24 hours, starting at the creation time of the volume. You can use this API to change the snapshot schedule configured for the volume.</p> <p>In the request you must identify the gateway volume whose snapshot schedule you want to update, and the schedule information, including when you want the snapshot to begin on a day and the frequency (in hours) of snapshots.</p>
    async fn update_snapshot_schedule(
        &self,
        input: UpdateSnapshotScheduleInput,
    ) -> Result<UpdateSnapshotScheduleOutput, RusotoError<UpdateSnapshotScheduleError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateSnapshotSchedule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateSnapshotScheduleError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateSnapshotScheduleOutput, _>()
    }

    /// <p>Updates the type of medium changer in a tape gateway. When you activate a tape gateway, you select a medium changer type for the tape gateway. This operation enables you to select a different type of medium changer after a tape gateway is activated. This operation is only supported in the tape gateway type.</p>
    async fn update_vtl_device_type(
        &self,
        input: UpdateVTLDeviceTypeInput,
    ) -> Result<UpdateVTLDeviceTypeOutput, RusotoError<UpdateVTLDeviceTypeError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "StorageGateway_20130630.UpdateVTLDeviceType",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateVTLDeviceTypeError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateVTLDeviceTypeOutput, _>()
    }
}
