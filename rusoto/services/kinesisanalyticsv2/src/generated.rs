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

impl KinesisAnalyticsV2Client {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request =
            SignedRequest::new(http_method, "kinesisanalytics", &self.region, request_uri);

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
pub struct AddApplicationCloudWatchLoggingOptionRequest {
    /// <p>The Kinesis Data Analytics application name.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>Provides the Amazon CloudWatch log stream Amazon Resource Name (ARN). </p>
    #[serde(rename = "cloudWatchLoggingOption")]
    pub cloud_watch_logging_option: CloudWatchLoggingOption,
    /// <p>A value you use to implement strong concurrency for application updates. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You get the application's current <code>ConditionalToken</code> using <a>DescribeApplication</a>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    #[serde(rename = "conditionalToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_token: Option<String>,
    /// <p>The version ID of the Kinesis Data Analytics application. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>.You can retrieve the application version ID using <a>DescribeApplication</a>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    #[serde(rename = "currentApplicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_application_version_id: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddApplicationCloudWatchLoggingOptionResponse {
    /// <p>The application's ARN.</p>
    #[serde(rename = "applicationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    /// <p>The new version ID of the Kinesis Data Analytics application. Kinesis Data Analytics updates the <code>ApplicationVersionId</code> each time you change the CloudWatch logging options. </p>
    #[serde(rename = "applicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    /// <p>The descriptions of the current CloudWatch logging options for the Kinesis Data Analytics application.</p>
    #[serde(rename = "cloudWatchLoggingOptionDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_option_descriptions: Option<Vec<CloudWatchLoggingOptionDescription>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddApplicationInputProcessingConfigurationRequest {
    /// <p>The name of the application to which you want to add the input processing configuration.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The version of the application to which you want to add the input processing configuration. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned.</p>
    #[serde(rename = "currentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The ID of the input configuration to add the input processing configuration to. You can get a list of the input IDs for an application using the <a>DescribeApplication</a> operation.</p>
    #[serde(rename = "inputId")]
    pub input_id: String,
    /// <p>The <a>InputProcessingConfiguration</a> to add to the application.</p>
    #[serde(rename = "inputProcessingConfiguration")]
    pub input_processing_configuration: InputProcessingConfiguration,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddApplicationInputProcessingConfigurationResponse {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "applicationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    /// <p>Provides the current application version. </p>
    #[serde(rename = "applicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    /// <p>The input ID that is associated with the application input. This is the ID that Kinesis Data Analytics assigns to each input configuration that you add to your application.</p>
    #[serde(rename = "inputId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_id: Option<String>,
    /// <p>The description of the preprocessor that executes on records in this input before the application's code is run.</p>
    #[serde(rename = "inputProcessingConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_processing_configuration_description: Option<InputProcessingConfigurationDescription>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddApplicationInputRequest {
    /// <p>The name of your existing application to which you want to add the streaming source.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The current version of your application. You must provide the <code>ApplicationVersionID</code> or the <code>ConditionalToken</code>.You can use the <a>DescribeApplication</a> operation to find the current application version.</p>
    #[serde(rename = "currentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The <a>Input</a> to add.</p>
    #[serde(rename = "input")]
    pub input: Input,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddApplicationInputResponse {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "applicationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    /// <p>Provides the current application version.</p>
    #[serde(rename = "applicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    /// <p>Describes the application input configuration. </p>
    #[serde(rename = "inputDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_descriptions: Option<Vec<InputDescription>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddApplicationOutputRequest {
    /// <p>The name of the application to which you want to add the output configuration.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The version of the application to which you want to add the output configuration. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned. </p>
    #[serde(rename = "currentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>An array of objects, each describing one output configuration. In the output configuration, you specify the name of an in-application stream, a destination (that is, a Kinesis data stream, a Kinesis Data Firehose delivery stream, or an AWS Lambda function), and record the formation to use when writing to the destination.</p>
    #[serde(rename = "output")]
    pub output: Output,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddApplicationOutputResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "applicationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    /// <p>The updated application version ID. Kinesis Data Analytics increments this ID when the application is updated.</p>
    #[serde(rename = "applicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    /// <p>Describes the application output configuration. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html">Configuring Application Output</a>. </p>
    #[serde(rename = "outputDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_descriptions: Option<Vec<OutputDescription>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddApplicationReferenceDataSourceRequest {
    /// <p>The name of an existing application.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The version of the application for which you are adding the reference data source. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned.</p>
    #[serde(rename = "currentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The reference data source can be an object in your Amazon S3 bucket. Kinesis Data Analytics reads the object and copies the data into the in-application table that is created. You provide an S3 bucket, object key name, and the resulting in-application table that is created. </p>
    #[serde(rename = "referenceDataSource")]
    pub reference_data_source: ReferenceDataSource,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddApplicationReferenceDataSourceResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "applicationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    /// <p>The updated application version ID. Kinesis Data Analytics increments this ID when the application is updated.</p>
    #[serde(rename = "applicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    /// <p>Describes reference data sources configured for the application. </p>
    #[serde(rename = "referenceDataSourceDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_data_source_descriptions: Option<Vec<ReferenceDataSourceDescription>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddApplicationVpcConfigurationRequest {
    /// <p>The name of an existing application.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>A value you use to implement strong concurrency for application updates. You must provide the <code>ApplicationVersionID</code> or the <code>ConditionalToken</code>. You get the application's current <code>ConditionalToken</code> using <a>DescribeApplication</a>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    #[serde(rename = "conditionalToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_token: Option<String>,
    /// <p>The version of the application to which you want to add the VPC configuration. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    #[serde(rename = "currentApplicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_application_version_id: Option<i64>,
    /// <p>Description of the VPC to add to the application.</p>
    #[serde(rename = "vpcConfiguration")]
    pub vpc_configuration: VpcConfiguration,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddApplicationVpcConfigurationResponse {
    /// <p>The ARN of the application.</p>
    #[serde(rename = "applicationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    /// <p>Provides the current application version. Kinesis Data Analytics updates the ApplicationVersionId each time you update the application.</p>
    #[serde(rename = "applicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    /// <p>The parameters of the new VPC configuration.</p>
    #[serde(rename = "vpcConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration_description: Option<VpcConfigurationDescription>,
}

/// <p>Describes code configuration for an application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ApplicationCodeConfiguration {
    /// <p>The location and type of the application code.</p>
    #[serde(rename = "codeContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_content: Option<CodeContent>,
    /// <p>Specifies whether the code content is in text or zip format.</p>
    #[serde(rename = "codeContentType")]
    pub code_content_type: String,
}

/// <p>Describes code configuration for an application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationCodeConfigurationDescription {
    /// <p>Describes details about the location and format of the application code.</p>
    #[serde(rename = "codeContentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_content_description: Option<CodeContentDescription>,
    /// <p>Specifies whether the code content is in text or zip format.</p>
    #[serde(rename = "codeContentType")]
    pub code_content_type: String,
}

/// <p>Describes code configuration updates for an application. This is supported for a Flink-based Kinesis Data Analytics application or a SQL-based Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ApplicationCodeConfigurationUpdate {
    /// <p>Describes updates to the code content type.</p>
    #[serde(rename = "codeContentTypeUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_content_type_update: Option<String>,
    /// <p>Describes updates to the code content of an application.</p>
    #[serde(rename = "codeContentUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_content_update: Option<CodeContentUpdate>,
}

/// <p>Specifies the creation parameters for a Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ApplicationConfiguration {
    /// <p>The code location and type parameters for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "applicationCodeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_code_configuration: Option<ApplicationCodeConfiguration>,
    /// <p>Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "applicationSnapshotConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_snapshot_configuration: Option<ApplicationSnapshotConfiguration>,
    /// <p>Describes execution properties for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "environmentProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_properties: Option<EnvironmentProperties>,
    /// <p>The creation and update parameters for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "flinkApplicationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flink_application_configuration: Option<FlinkApplicationConfiguration>,
    /// <p>The creation and update parameters for a SQL-based Kinesis Data Analytics application.</p>
    #[serde(rename = "sqlApplicationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_application_configuration: Option<SqlApplicationConfiguration>,
    /// <p>The array of descriptions of VPC configurations available to the application.</p>
    #[serde(rename = "vpcConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configurations: Option<Vec<VpcConfiguration>>,
    /// <p>The configuration parameters for a Kinesis Data Analytics Studio notebook.</p>
    #[serde(rename = "zeppelinApplicationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeppelin_application_configuration: Option<ZeppelinApplicationConfiguration>,
}

/// <p>Describes details about the application code and starting parameters for a Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationConfigurationDescription {
    /// <p>The details about the application code for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "applicationCodeConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_code_configuration_description: Option<ApplicationCodeConfigurationDescription>,
    /// <p>Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "applicationSnapshotConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_snapshot_configuration_description:
        Option<ApplicationSnapshotConfigurationDescription>,
    /// <p>Describes execution properties for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "environmentPropertyDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_property_descriptions: Option<EnvironmentPropertyDescriptions>,
    /// <p>The details about a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "flinkApplicationConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flink_application_configuration_description:
        Option<FlinkApplicationConfigurationDescription>,
    /// <p>The details about the starting properties for a Kinesis Data Analytics application.</p>
    #[serde(rename = "runConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_configuration_description: Option<RunConfigurationDescription>,
    /// <p>The details about inputs, outputs, and reference data sources for a SQL-based Kinesis Data Analytics application.</p>
    #[serde(rename = "sqlApplicationConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_application_configuration_description: Option<SqlApplicationConfigurationDescription>,
    /// <p>The array of descriptions of VPC configurations available to the application.</p>
    #[serde(rename = "vpcConfigurationDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration_descriptions: Option<Vec<VpcConfigurationDescription>>,
    /// <p>The configuration parameters for a Kinesis Data Analytics Studio notebook.</p>
    #[serde(rename = "zeppelinApplicationConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeppelin_application_configuration_description:
        Option<ZeppelinApplicationConfigurationDescription>,
}

/// <p>Describes updates to an application's configuration.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ApplicationConfigurationUpdate {
    /// <p>Describes updates to an application's code configuration.</p>
    #[serde(rename = "applicationCodeConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_code_configuration_update: Option<ApplicationCodeConfigurationUpdate>,
    /// <p>Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "applicationSnapshotConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_snapshot_configuration_update: Option<ApplicationSnapshotConfigurationUpdate>,
    /// <p>Describes updates to the environment properties for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "environmentPropertyUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_property_updates: Option<EnvironmentPropertyUpdates>,
    /// <p>Describes updates to a Flink-based Kinesis Data Analytics application's configuration.</p>
    #[serde(rename = "flinkApplicationConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flink_application_configuration_update: Option<FlinkApplicationConfigurationUpdate>,
    /// <p>Describes updates to a SQL-based Kinesis Data Analytics application's configuration.</p>
    #[serde(rename = "sqlApplicationConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_application_configuration_update: Option<SqlApplicationConfigurationUpdate>,
    /// <p>Updates to the array of descriptions of VPC configurations available to the application.</p>
    #[serde(rename = "vpcConfigurationUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration_updates: Option<Vec<VpcConfigurationUpdate>>,
    /// <p>Updates to the configuration of a Kinesis Data Analytics Studio notebook.</p>
    #[serde(rename = "zeppelinApplicationConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeppelin_application_configuration_update: Option<ZeppelinApplicationConfigurationUpdate>,
}

/// <p>Describes the application, including the application Amazon Resource Name (ARN), status, latest version, and input and output configurations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationDetail {
    /// <p>The ARN of the application.</p>
    #[serde(rename = "applicationARN")]
    pub application_arn: String,
    /// <p>Describes details about the application code and starting parameters for a Kinesis Data Analytics application.</p>
    #[serde(rename = "applicationConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_configuration_description: Option<ApplicationConfigurationDescription>,
    /// <p>The description of the application.</p>
    #[serde(rename = "applicationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_description: Option<String>,
    /// <p>The details of the maintenance configuration for the application.</p>
    #[serde(rename = "applicationMaintenanceConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_maintenance_configuration_description:
        Option<ApplicationMaintenanceConfigurationDescription>,
    /// <p>To create a Kinesis Data Analytics Studio notebook, you must set the mode to <code>INTERACTIVE</code>. However, for a Kinesis Data Analytics for Apache Flink application, the mode is optional.</p>
    #[serde(rename = "applicationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_mode: Option<String>,
    /// <p>The name of the application.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The status of the application.</p>
    #[serde(rename = "applicationStatus")]
    pub application_status: String,
    /// <p>Provides the current application version. Kinesis Data Analytics updates the <code>ApplicationVersionId</code> each time you update the application.</p>
    #[serde(rename = "applicationVersionId")]
    pub application_version_id: i64,
    /// <p>If you reverted the application using <a>RollbackApplication</a>, the application version when <code>RollbackApplication</code> was called.</p>
    #[serde(rename = "applicationVersionRolledBackFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_rolled_back_from: Option<i64>,
    /// <p>The version to which you want to roll back the application.</p>
    #[serde(rename = "applicationVersionRolledBackTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_rolled_back_to: Option<i64>,
    /// <p>The previous application version before the latest application update. <a>RollbackApplication</a> reverts the application to this version.</p>
    #[serde(rename = "applicationVersionUpdatedFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_updated_from: Option<i64>,
    /// <p>Describes the application Amazon CloudWatch logging options.</p>
    #[serde(rename = "cloudWatchLoggingOptionDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_option_descriptions: Option<Vec<CloudWatchLoggingOptionDescription>>,
    /// <p>A value you use to implement strong concurrency for application updates.</p>
    #[serde(rename = "conditionalToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_token: Option<String>,
    /// <p>The current timestamp when the application was created.</p>
    #[serde(rename = "createTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    /// <p>The current timestamp when the application was last updated.</p>
    #[serde(rename = "lastUpdateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    /// <p>The runtime environment for the application (<code>SQL-1_0</code>, <code>FLINK-1_6</code>, <code>FLINK-1_8</code>, or <code>FLINK-1_11</code>).</p>
    #[serde(rename = "runtimeEnvironment")]
    pub runtime_environment: String,
    /// <p>Specifies the IAM role that the application uses to access external resources.</p>
    #[serde(rename = "serviceExecutionRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_execution_role: Option<String>,
}

/// <p>The details of the maintenance configuration for the application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationMaintenanceConfigurationDescription {
    /// <p>The end time for the maintenance window.</p>
    #[serde(rename = "applicationMaintenanceWindowEndTime")]
    pub application_maintenance_window_end_time: String,
    /// <p>The start time for the maintenance window.</p>
    #[serde(rename = "applicationMaintenanceWindowStartTime")]
    pub application_maintenance_window_start_time: String,
}

/// <p>Describes the updated maintenance configuration for the application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ApplicationMaintenanceConfigurationUpdate {
    /// <p>The updated start time for the maintenance window.</p>
    #[serde(rename = "applicationMaintenanceWindowStartTimeUpdate")]
    pub application_maintenance_window_start_time_update: String,
}

/// <p>Specifies the method and snapshot to use when restarting an application using previously saved application state.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ApplicationRestoreConfiguration {
    /// <p>Specifies how the application should be restored.</p>
    #[serde(rename = "applicationRestoreType")]
    pub application_restore_type: String,
    /// <p>The identifier of an existing snapshot of application state to use to restart an application. The application uses this value if <code>RESTORE_FROM_CUSTOM_SNAPSHOT</code> is specified for the <code>ApplicationRestoreType</code>.</p>
    #[serde(rename = "snapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
}

/// <p>Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ApplicationSnapshotConfiguration {
    /// <p>Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "snapshotsEnabled")]
    pub snapshots_enabled: bool,
}

/// <p>Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationSnapshotConfigurationDescription {
    /// <p>Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "snapshotsEnabled")]
    pub snapshots_enabled: bool,
}

/// <p>Describes updates to whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ApplicationSnapshotConfigurationUpdate {
    /// <p>Describes updates to whether snapshots are enabled for an application.</p>
    #[serde(rename = "snapshotsEnabledUpdate")]
    pub snapshots_enabled_update: bool,
}

/// <p>Provides application summary information, including the application Amazon Resource Name (ARN), name, and status.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationSummary {
    /// <p>The ARN of the application.</p>
    #[serde(rename = "applicationARN")]
    pub application_arn: String,
    /// <p>For a Kinesis Data Analytics for Apache Flink application, the mode is <code>STREAMING</code>. For a Kinesis Data Analytics Studio notebook, it is <code>INTERACTIVE</code>.</p>
    #[serde(rename = "applicationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_mode: Option<String>,
    /// <p>The name of the application.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The status of the application.</p>
    #[serde(rename = "applicationStatus")]
    pub application_status: String,
    /// <p>Provides the current application version.</p>
    #[serde(rename = "applicationVersionId")]
    pub application_version_id: i64,
    /// <p>The runtime environment for the application.</p>
    #[serde(rename = "runtimeEnvironment")]
    pub runtime_environment: String,
}

/// <p>The summary of the application version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplicationVersionSummary {
    /// <p>The status of the application.</p>
    #[serde(rename = "applicationStatus")]
    pub application_status: String,
    /// <p>The ID of the application version. Kinesis Data Analytics updates the <code>ApplicationVersionId</code> each time you update the application.</p>
    #[serde(rename = "applicationVersionId")]
    pub application_version_id: i64,
}

/// <p>For a SQL-based Kinesis Data Analytics application, provides additional mapping information when the record format uses delimiters, such as CSV. For example, the following sample records use CSV format, where the records use the <i>'\n'</i> as the row delimiter and a comma (",") as the column delimiter: </p> <p> <code>"name1", "address1"</code> </p> <p> <code>"name2", "address2"</code> </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CSVMappingParameters {
    /// <p>The column delimiter. For example, in a CSV format, a comma (",") is the typical column delimiter.</p>
    #[serde(rename = "recordColumnDelimiter")]
    pub record_column_delimiter: String,
    /// <p>The row delimiter. For example, in a CSV format, <i>'\n'</i> is the typical row delimiter.</p>
    #[serde(rename = "recordRowDelimiter")]
    pub record_row_delimiter: String,
}

/// <p>The configuration parameters for the default AWS Glue database. You use this database for SQL queries that you write in a Kinesis Data Analytics Studio notebook.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CatalogConfiguration {
    /// <p>The configuration parameters for the default AWS Glue database. You use this database for Apache Flink SQL queries and table API transforms that you write in a Kinesis Data Analytics Studio notebook.</p>
    #[serde(rename = "glueDataCatalogConfiguration")]
    pub glue_data_catalog_configuration: GlueDataCatalogConfiguration,
}

/// <p>The configuration parameters for the default AWS Glue database. You use this database for Apache Flink SQL queries and table API transforms that you write in a Kinesis Data Analytics Studio notebook.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CatalogConfigurationDescription {
    /// <p>The configuration parameters for the default AWS Glue database. You use this database for SQL queries that you write in a Kinesis Data Analytics Studio notebook.</p>
    #[serde(rename = "glueDataCatalogConfigurationDescription")]
    pub glue_data_catalog_configuration_description: GlueDataCatalogConfigurationDescription,
}

/// <p>Updates to </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CatalogConfigurationUpdate {
    /// <p>Updates to the configuration parameters for the default AWS Glue database. You use this database for SQL queries that you write in a Kinesis Data Analytics Studio notebook.</p>
    #[serde(rename = "glueDataCatalogConfigurationUpdate")]
    pub glue_data_catalog_configuration_update: GlueDataCatalogConfigurationUpdate,
}

/// <p>Describes an application's checkpointing configuration. Checkpointing is the process of persisting application state for fault tolerance. For more information, see <a href="https://ci.apache.org/projects/flink/flink-docs-release-1.8/concepts/programming-model.html#checkpoints-for-fault-tolerance"> Checkpoints for Fault Tolerance</a> in the <a href="https://ci.apache.org/projects/flink/flink-docs-release-1.8/">Apache Flink Documentation</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CheckpointConfiguration {
    /// <p><p>Describes the interval in milliseconds between checkpoint operations. </p> <note> <p>If <code>CheckpointConfiguration.ConfigurationType</code> is <code>DEFAULT</code>, the application will use a <code>CheckpointInterval</code> value of 60000, even if this value is set to another value using this API or in application code.</p> </note></p>
    #[serde(rename = "checkpointInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_interval: Option<i64>,
    /// <p><p>Describes whether checkpointing is enabled for a Flink-based Kinesis Data Analytics application.</p> <note> <p>If <code>CheckpointConfiguration.ConfigurationType</code> is <code>DEFAULT</code>, the application will use a <code>CheckpointingEnabled</code> value of <code>true</code>, even if this value is set to another value using this API or in application code.</p> </note></p>
    #[serde(rename = "checkpointingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpointing_enabled: Option<bool>,
    /// <p><p>Describes whether the application uses Kinesis Data Analytics&#39; default checkpointing behavior. You must set this property to <code>CUSTOM</code> in order to set the <code>CheckpointingEnabled</code>, <code>CheckpointInterval</code>, or <code>MinPauseBetweenCheckpoints</code> parameters.</p> <note> <p>If this value is set to <code>DEFAULT</code>, the application will use the following values, even if they are set to other values using APIs or application code:</p> <ul> <li> <p> <b>CheckpointingEnabled:</b> true</p> </li> <li> <p> <b>CheckpointInterval:</b> 60000</p> </li> <li> <p> <b>MinPauseBetweenCheckpoints:</b> 5000</p> </li> </ul> </note></p>
    #[serde(rename = "configurationType")]
    pub configuration_type: String,
    /// <p><p>Describes the minimum time in milliseconds after a checkpoint operation completes that a new checkpoint operation can start. If a checkpoint operation takes longer than the <code>CheckpointInterval</code>, the application otherwise performs continual checkpoint operations. For more information, see <a href="https://ci.apache.org/projects/flink/flink-docs-release-1.8/ops/state/large_state_tuning.html#tuning-checkpointing"> Tuning Checkpointing</a> in the <a href="https://ci.apache.org/projects/flink/flink-docs-release-1.8/">Apache Flink Documentation</a>.</p> <note> <p>If <code>CheckpointConfiguration.ConfigurationType</code> is <code>DEFAULT</code>, the application will use a <code>MinPauseBetweenCheckpoints</code> value of 5000, even if this value is set using this API or in application code.</p> </note></p>
    #[serde(rename = "minPauseBetweenCheckpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pause_between_checkpoints: Option<i64>,
}

/// <p>Describes checkpointing parameters for a Flink-based Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CheckpointConfigurationDescription {
    /// <p><p>Describes the interval in milliseconds between checkpoint operations. </p> <note> <p>If <code>CheckpointConfiguration.ConfigurationType</code> is <code>DEFAULT</code>, the application will use a <code>CheckpointInterval</code> value of 60000, even if this value is set to another value using this API or in application code.</p> </note></p>
    #[serde(rename = "checkpointInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_interval: Option<i64>,
    /// <p><p>Describes whether checkpointing is enabled for a Flink-based Kinesis Data Analytics application.</p> <note> <p>If <code>CheckpointConfiguration.ConfigurationType</code> is <code>DEFAULT</code>, the application will use a <code>CheckpointingEnabled</code> value of <code>true</code>, even if this value is set to another value using this API or in application code.</p> </note></p>
    #[serde(rename = "checkpointingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpointing_enabled: Option<bool>,
    /// <p><p>Describes whether the application uses the default checkpointing behavior in Kinesis Data Analytics. </p> <note> <p>If this value is set to <code>DEFAULT</code>, the application will use the following values, even if they are set to other values using APIs or application code:</p> <ul> <li> <p> <b>CheckpointingEnabled:</b> true</p> </li> <li> <p> <b>CheckpointInterval:</b> 60000</p> </li> <li> <p> <b>MinPauseBetweenCheckpoints:</b> 5000</p> </li> </ul> </note></p>
    #[serde(rename = "configurationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type: Option<String>,
    /// <p><p>Describes the minimum time in milliseconds after a checkpoint operation completes that a new checkpoint operation can start. </p> <note> <p>If <code>CheckpointConfiguration.ConfigurationType</code> is <code>DEFAULT</code>, the application will use a <code>MinPauseBetweenCheckpoints</code> value of 5000, even if this value is set using this API or in application code.</p> </note></p>
    #[serde(rename = "minPauseBetweenCheckpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pause_between_checkpoints: Option<i64>,
}

/// <p>Describes updates to the checkpointing parameters for a Flink-based Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CheckpointConfigurationUpdate {
    /// <p><p>Describes updates to the interval in milliseconds between checkpoint operations.</p> <note> <p>If <code>CheckpointConfiguration.ConfigurationType</code> is <code>DEFAULT</code>, the application will use a <code>CheckpointInterval</code> value of 60000, even if this value is set to another value using this API or in application code.</p> </note></p>
    #[serde(rename = "checkpointIntervalUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_interval_update: Option<i64>,
    /// <p><p>Describes updates to whether checkpointing is enabled for an application.</p> <note> <p>If <code>CheckpointConfiguration.ConfigurationType</code> is <code>DEFAULT</code>, the application will use a <code>CheckpointingEnabled</code> value of <code>true</code>, even if this value is set to another value using this API or in application code.</p> </note></p>
    #[serde(rename = "checkpointingEnabledUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpointing_enabled_update: Option<bool>,
    /// <p><p>Describes updates to whether the application uses the default checkpointing behavior of Kinesis Data Analytics. You must set this property to <code>CUSTOM</code> in order to set the <code>CheckpointingEnabled</code>, <code>CheckpointInterval</code>, or <code>MinPauseBetweenCheckpoints</code> parameters. </p> <note> <p>If this value is set to <code>DEFAULT</code>, the application will use the following values, even if they are set to other values using APIs or application code:</p> <ul> <li> <p> <b>CheckpointingEnabled:</b> true</p> </li> <li> <p> <b>CheckpointInterval:</b> 60000</p> </li> <li> <p> <b>MinPauseBetweenCheckpoints:</b> 5000</p> </li> </ul> </note></p>
    #[serde(rename = "configurationTypeUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type_update: Option<String>,
    /// <p><p>Describes updates to the minimum time in milliseconds after a checkpoint operation completes that a new checkpoint operation can start.</p> <note> <p>If <code>CheckpointConfiguration.ConfigurationType</code> is <code>DEFAULT</code>, the application will use a <code>MinPauseBetweenCheckpoints</code> value of 5000, even if this value is set using this API or in application code.</p> </note></p>
    #[serde(rename = "minPauseBetweenCheckpointsUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pause_between_checkpoints_update: Option<i64>,
}

/// <p>Provides a description of Amazon CloudWatch logging options, including the log stream Amazon Resource Name (ARN). </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CloudWatchLoggingOption {
    /// <p>The ARN of the CloudWatch log to receive application messages.</p>
    #[serde(rename = "logStreamARN")]
    pub log_stream_arn: String,
}

/// <p>Describes the Amazon CloudWatch logging option.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CloudWatchLoggingOptionDescription {
    /// <p>The ID of the CloudWatch logging option description.</p>
    #[serde(rename = "cloudWatchLoggingOptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_option_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the CloudWatch log to receive application messages.</p>
    #[serde(rename = "logStreamARN")]
    pub log_stream_arn: String,
    /// <p><p>The IAM ARN of the role to use to send application messages. </p> <note> <p>Provided for backward compatibility. Applications created with the current API version have an application-level service execution role rather than a resource-level role.</p> </note></p>
    #[serde(rename = "roleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Describes the Amazon CloudWatch logging option updates.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CloudWatchLoggingOptionUpdate {
    /// <p>The ID of the CloudWatch logging option to update</p>
    #[serde(rename = "cloudWatchLoggingOptionId")]
    pub cloud_watch_logging_option_id: String,
    /// <p>The Amazon Resource Name (ARN) of the CloudWatch log to receive application messages.</p>
    #[serde(rename = "logStreamARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_arn_update: Option<String>,
}

/// <p>Specifies either the application code, or the location of the application code, for a Flink-based Kinesis Data Analytics application. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CodeContent {
    /// <p>Information about the Amazon S3 bucket that contains the application code.</p>
    #[serde(rename = "s3ContentLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_content_location: Option<S3ContentLocation>,
    /// <p>The text-format code for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "textContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_content: Option<String>,
    /// <p>The zip-format code for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "zipFileContent")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file_content: Option<bytes::Bytes>,
}

/// <p>Describes details about the code of a Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CodeContentDescription {
    /// <p>The checksum that can be used to validate zip-format code.</p>
    #[serde(rename = "codeMD5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_md5: Option<String>,
    /// <p>The size in bytes of the application code. Can be used to validate zip-format code.</p>
    #[serde(rename = "codeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,
    /// <p>The S3 bucket Amazon Resource Name (ARN), file key, and object version of the application code stored in Amazon S3.</p>
    #[serde(rename = "s3ApplicationCodeLocationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_application_code_location_description: Option<S3ApplicationCodeLocationDescription>,
    /// <p>The text-format code</p>
    #[serde(rename = "textContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_content: Option<String>,
}

/// <p>Describes an update to the code of an application. Not supported for Apache Zeppelin.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CodeContentUpdate {
    /// <p>Describes an update to the location of code for an application.</p>
    #[serde(rename = "s3ContentLocationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_content_location_update: Option<S3ContentLocationUpdate>,
    /// <p>Describes an update to the text code for an application.</p>
    #[serde(rename = "textContentUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_content_update: Option<String>,
    /// <p>Describes an update to the zipped code for an application.</p>
    #[serde(rename = "zipFileContentUpdate")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file_content_update: Option<bytes::Bytes>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApplicationPresignedUrlRequest {
    /// <p>The name of the application.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The duration in seconds for which the returned URL will be valid.</p>
    #[serde(rename = "sessionExpirationDurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_expiration_duration_in_seconds: Option<i64>,
    /// <p>The type of the extension for which to create and return a URL. Currently, the only valid extension URL type is <code>FLINK_DASHBOARD_URL</code>. </p>
    #[serde(rename = "urlType")]
    pub url_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateApplicationPresignedUrlResponse {
    /// <p>The URL of the extension.</p>
    #[serde(rename = "authorizedUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_url: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApplicationRequest {
    /// <p>Use this parameter to configure the application.</p>
    #[serde(rename = "applicationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_configuration: Option<ApplicationConfiguration>,
    /// <p>A summary description of the application.</p>
    #[serde(rename = "applicationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_description: Option<String>,
    /// <p>Use the <code>STREAMING</code> mode to create a Kinesis Data Analytics Studio notebook. To create a Kinesis Data Analytics Studio notebook, use the <code>INTERACTIVE</code> mode.</p>
    #[serde(rename = "applicationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_mode: Option<String>,
    /// <p>The name of your application (for example, <code>sample-app</code>).</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>Use this parameter to configure an Amazon CloudWatch log stream to monitor application configuration errors. </p>
    #[serde(rename = "cloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<Vec<CloudWatchLoggingOption>>,
    /// <p>The runtime environment for the application (<code>SQL-1_0</code>, <code>FLINK-1_6</code>, <code>FLINK-1_8</code>, or <code>FLINK-1_11</code>).</p>
    #[serde(rename = "runtimeEnvironment")]
    pub runtime_environment: String,
    /// <p>The IAM role used by the application to access Kinesis data streams, Kinesis Data Firehose delivery streams, Amazon S3 objects, and other external resources.</p>
    #[serde(rename = "serviceExecutionRole")]
    pub service_execution_role: String,
    /// <p>A list of one or more tags to assign to the application. A tag is a key-value pair that identifies an application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/how-tagging.html">Using Tagging</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateApplicationResponse {
    /// <p>In response to your <code>CreateApplication</code> request, Kinesis Data Analytics returns a response with details of the application it created.</p>
    #[serde(rename = "applicationDetail")]
    pub application_detail: ApplicationDetail,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApplicationSnapshotRequest {
    /// <p>The name of an existing application</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>An identifier for the application snapshot.</p>
    #[serde(rename = "snapshotName")]
    pub snapshot_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateApplicationSnapshotResponse {}

/// <p>Specifies dependency JARs, as well as JAR files that contain user-defined functions (UDF).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CustomArtifactConfiguration {
    /// <p> <code>UDF</code> stands for user-defined functions. This type of artifact must be in an S3 bucket. A <code>DEPENDENCY_JAR</code> can be in either Maven or an S3 bucket.</p>
    #[serde(rename = "artifactType")]
    pub artifact_type: String,
    /// <p>The parameters required to fully specify a Maven reference.</p>
    #[serde(rename = "mavenReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maven_reference: Option<MavenReference>,
    #[serde(rename = "s3ContentLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_content_location: Option<S3ContentLocation>,
}

/// <p>Specifies a dependency JAR or a JAR of user-defined functions.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CustomArtifactConfigurationDescription {
    /// <p> <code>UDF</code> stands for user-defined functions. This type of artifact must be in an S3 bucket. A <code>DEPENDENCY_JAR</code> can be in either Maven or an S3 bucket.</p>
    #[serde(rename = "artifactType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<String>,
    /// <p>The parameters that are required to specify a Maven dependency.</p>
    #[serde(rename = "mavenReferenceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maven_reference_description: Option<MavenReference>,
    #[serde(rename = "s3ContentLocationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_content_location_description: Option<S3ContentLocation>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApplicationCloudWatchLoggingOptionRequest {
    /// <p>The application name.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The <code>CloudWatchLoggingOptionId</code> of the Amazon CloudWatch logging option to delete. You can get the <code>CloudWatchLoggingOptionId</code> by using the <a>DescribeApplication</a> operation. </p>
    #[serde(rename = "cloudWatchLoggingOptionId")]
    pub cloud_watch_logging_option_id: String,
    /// <p>A value you use to implement strong concurrency for application updates. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You get the application's current <code>ConditionalToken</code> using <a>DescribeApplication</a>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    #[serde(rename = "conditionalToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_token: Option<String>,
    /// <p>The version ID of the application. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You can retrieve the application version ID using <a>DescribeApplication</a>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    #[serde(rename = "currentApplicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_application_version_id: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApplicationCloudWatchLoggingOptionResponse {
    /// <p>The application's Amazon Resource Name (ARN).</p>
    #[serde(rename = "applicationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    /// <p>The version ID of the application. Kinesis Data Analytics updates the <code>ApplicationVersionId</code> each time you change the CloudWatch logging options.</p>
    #[serde(rename = "applicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    /// <p>The descriptions of the remaining CloudWatch logging options for the application.</p>
    #[serde(rename = "cloudWatchLoggingOptionDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_option_descriptions: Option<Vec<CloudWatchLoggingOptionDescription>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApplicationInputProcessingConfigurationRequest {
    /// <p>The name of the application.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The application version. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned. </p>
    #[serde(rename = "currentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The ID of the input configuration from which to delete the input processing configuration. You can get a list of the input IDs for an application by using the <a>DescribeApplication</a> operation.</p>
    #[serde(rename = "inputId")]
    pub input_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApplicationInputProcessingConfigurationResponse {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "applicationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    /// <p>The current application version ID.</p>
    #[serde(rename = "applicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApplicationOutputRequest {
    /// <p>The application name.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The application version. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned. </p>
    #[serde(rename = "currentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The ID of the configuration to delete. Each output configuration that is added to the application (either when the application is created or later) using the <a>AddApplicationOutput</a> operation has a unique ID. You need to provide the ID to uniquely identify the output configuration that you want to delete from the application configuration. You can use the <a>DescribeApplication</a> operation to get the specific <code>OutputId</code>. </p>
    #[serde(rename = "outputId")]
    pub output_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApplicationOutputResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "applicationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    /// <p>The current application version ID.</p>
    #[serde(rename = "applicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApplicationReferenceDataSourceRequest {
    /// <p>The name of an existing application.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The current application version. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned.</p>
    #[serde(rename = "currentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The ID of the reference data source. When you add a reference data source to your application using the <a>AddApplicationReferenceDataSource</a>, Kinesis Data Analytics assigns an ID. You can use the <a>DescribeApplication</a> operation to get the reference ID. </p>
    #[serde(rename = "referenceId")]
    pub reference_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApplicationReferenceDataSourceResponse {
    /// <p>The application Amazon Resource Name (ARN).</p>
    #[serde(rename = "applicationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    /// <p>The updated version ID of the application.</p>
    #[serde(rename = "applicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApplicationRequest {
    /// <p>The name of the application to delete.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>Use the <code>DescribeApplication</code> operation to get this value.</p>
    #[serde(rename = "createTimestamp")]
    pub create_timestamp: f64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApplicationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApplicationSnapshotRequest {
    /// <p>The name of an existing application.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The creation timestamp of the application snapshot to delete. You can retrieve this value using or .</p>
    #[serde(rename = "snapshotCreationTimestamp")]
    pub snapshot_creation_timestamp: f64,
    /// <p>The identifier for the snapshot delete.</p>
    #[serde(rename = "snapshotName")]
    pub snapshot_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApplicationSnapshotResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApplicationVpcConfigurationRequest {
    /// <p>The name of an existing application.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>A value you use to implement strong concurrency for application updates. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You get the application's current <code>ConditionalToken</code> using <a>DescribeApplication</a>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    #[serde(rename = "conditionalToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_token: Option<String>,
    /// <p>The current application version ID. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You can retrieve the application version ID using <a>DescribeApplication</a>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    #[serde(rename = "currentApplicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_application_version_id: Option<i64>,
    /// <p>The ID of the VPC configuration to delete.</p>
    #[serde(rename = "vpcConfigurationId")]
    pub vpc_configuration_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApplicationVpcConfigurationResponse {
    /// <p>The ARN of the Kinesis Data Analytics application.</p>
    #[serde(rename = "applicationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    /// <p>The updated version ID of the application.</p>
    #[serde(rename = "applicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
}

/// <p>The information required to deploy a Kinesis Data Analytics Studio notebook as an application with durable state..</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeployAsApplicationConfiguration {
    /// <p>The description of an Amazon S3 object that contains the Amazon Data Analytics application, including the Amazon Resource Name (ARN) of the S3 bucket, the name of the Amazon S3 object that contains the data, and the version number of the Amazon S3 object that contains the data. </p>
    #[serde(rename = "s3ContentLocation")]
    pub s3_content_location: S3ContentBaseLocation,
}

/// <p>The configuration information required to deploy an Amazon Data Analytics Studio notebook as an application with durable state.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeployAsApplicationConfigurationDescription {
    /// <p>The location that holds the data required to specify an Amazon Data Analytics application.</p>
    #[serde(rename = "s3ContentLocationDescription")]
    pub s3_content_location_description: S3ContentBaseLocationDescription,
}

/// <p>Updates to the configuration information required to deploy an Amazon Data Analytics Studio notebook as an application with durable state..</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeployAsApplicationConfigurationUpdate {
    /// <p>Updates to the location that holds the data required to specify an Amazon Data Analytics application.</p>
    #[serde(rename = "s3ContentLocationUpdate")]
    pub s3_content_location_update: S3ContentBaseLocationUpdate,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeApplicationRequest {
    /// <p>The name of the application.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>Displays verbose information about a Kinesis Data Analytics application, including the application's job plan.</p>
    #[serde(rename = "includeAdditionalDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_additional_details: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeApplicationResponse {
    /// <p>Provides a description of the application, such as the application's Amazon Resource Name (ARN), status, and latest version.</p>
    #[serde(rename = "applicationDetail")]
    pub application_detail: ApplicationDetail,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeApplicationSnapshotRequest {
    /// <p>The name of an existing application.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The identifier of an application snapshot. You can retrieve this value using .</p>
    #[serde(rename = "snapshotName")]
    pub snapshot_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeApplicationSnapshotResponse {
    /// <p>An object containing information about the application snapshot.</p>
    #[serde(rename = "snapshotDetails")]
    pub snapshot_details: SnapshotDetails,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeApplicationVersionRequest {
    /// <p>The name of the application for which you want to get the version description.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The ID of the application version for which you want to get the description.</p>
    #[serde(rename = "applicationVersionId")]
    pub application_version_id: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeApplicationVersionResponse {
    #[serde(rename = "applicationVersionDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_detail: Option<ApplicationDetail>,
}

/// <p>Describes the data format when records are written to the destination in a SQL-based Kinesis Data Analytics application. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DestinationSchema {
    /// <p>Specifies the format of the records on the output stream.</p>
    #[serde(rename = "recordFormatType")]
    pub record_format_type: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DiscoverInputSchemaRequest {
    /// <p>The <a>InputProcessingConfiguration</a> to use to preprocess the records before discovering the schema of the records.</p>
    #[serde(rename = "inputProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_processing_configuration: Option<InputProcessingConfiguration>,
    /// <p>The point at which you want Kinesis Data Analytics to start reading records from the specified streaming source discovery purposes.</p>
    #[serde(rename = "inputStartingPositionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_starting_position_configuration: Option<InputStartingPositionConfiguration>,
    /// <p>The Amazon Resource Name (ARN) of the streaming source.</p>
    #[serde(rename = "resourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>Specify this parameter to discover a schema from data in an Amazon S3 object.</p>
    #[serde(rename = "s3Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<S3Configuration>,
    /// <p>The ARN of the role that is used to access the streaming source.</p>
    #[serde(rename = "serviceExecutionRole")]
    pub service_execution_role: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DiscoverInputSchemaResponse {
    /// <p>The schema inferred from the streaming source. It identifies the format of the data in the streaming source and how each data element maps to corresponding columns in the in-application stream that you can create.</p>
    #[serde(rename = "inputSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<SourceSchema>,
    /// <p>An array of elements, where each element corresponds to a row in a stream record (a stream record can have more than one row).</p>
    #[serde(rename = "parsedInputRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed_input_records: Option<Vec<Vec<String>>>,
    /// <p>The stream data that was modified by the processor specified in the <code>InputProcessingConfiguration</code> parameter.</p>
    #[serde(rename = "processedInputRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_input_records: Option<Vec<String>>,
    /// <p>The raw stream data that was sampled to infer the schema.</p>
    #[serde(rename = "rawInputRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_input_records: Option<Vec<String>>,
}

/// <p>Describes execution properties for a Flink-based Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnvironmentProperties {
    /// <p>Describes the execution property groups.</p>
    #[serde(rename = "propertyGroups")]
    pub property_groups: Vec<PropertyGroup>,
}

/// <p>Describes the execution properties for an Apache Flink runtime.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EnvironmentPropertyDescriptions {
    /// <p>Describes the execution property groups.</p>
    #[serde(rename = "propertyGroupDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_group_descriptions: Option<Vec<PropertyGroup>>,
}

/// <p>Describes updates to the execution property groups for a Flink-based Kinesis Data Analytics application or a Studio notebook.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct EnvironmentPropertyUpdates {
    /// <p>Describes updates to the execution property groups.</p>
    #[serde(rename = "propertyGroups")]
    pub property_groups: Vec<PropertyGroup>,
}

/// <p>Describes configuration parameters for a Flink-based Kinesis Data Analytics application or a Studio notebook.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FlinkApplicationConfiguration {
    /// <p>Describes an application's checkpointing configuration. Checkpointing is the process of persisting application state for fault tolerance. For more information, see <a href="https://ci.apache.org/projects/flink/flink-docs-release-1.8/concepts/programming-model.html#checkpoints-for-fault-tolerance"> Checkpoints for Fault Tolerance</a> in the <a href="https://ci.apache.org/projects/flink/flink-docs-release-1.8/">Apache Flink Documentation</a>. </p>
    #[serde(rename = "checkpointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_configuration: Option<CheckpointConfiguration>,
    /// <p>Describes configuration parameters for Amazon CloudWatch logging for an application.</p>
    #[serde(rename = "monitoringConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
    /// <p>Describes parameters for how an application executes multiple tasks simultaneously.</p>
    #[serde(rename = "parallelismConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_configuration: Option<ParallelismConfiguration>,
}

/// <p>Describes configuration parameters for a Flink-based Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FlinkApplicationConfigurationDescription {
    /// <p>Describes an application's checkpointing configuration. Checkpointing is the process of persisting application state for fault tolerance.</p>
    #[serde(rename = "checkpointConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_configuration_description: Option<CheckpointConfigurationDescription>,
    /// <p>The job plan for an application. For more information about the job plan, see <a href="https://ci.apache.org/projects/flink/flink-docs-release-1.8/internals/job_scheduling.html">Jobs and Scheduling</a> in the <a href="https://ci.apache.org/projects/flink/flink-docs-release-1.8/">Apache Flink Documentation</a>. To retrieve the job plan for the application, use the <a>DescribeApplicationRequest$IncludeAdditionalDetails</a> parameter of the <a>DescribeApplication</a> operation.</p>
    #[serde(rename = "jobPlanDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_plan_description: Option<String>,
    /// <p>Describes configuration parameters for Amazon CloudWatch logging for an application.</p>
    #[serde(rename = "monitoringConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration_description: Option<MonitoringConfigurationDescription>,
    /// <p>Describes parameters for how an application executes multiple tasks simultaneously.</p>
    #[serde(rename = "parallelismConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_configuration_description: Option<ParallelismConfigurationDescription>,
}

/// <p>Describes updates to the configuration parameters for a Flink-based Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FlinkApplicationConfigurationUpdate {
    /// <p>Describes updates to an application's checkpointing configuration. Checkpointing is the process of persisting application state for fault tolerance.</p>
    #[serde(rename = "checkpointConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_configuration_update: Option<CheckpointConfigurationUpdate>,
    /// <p>Describes updates to the configuration parameters for Amazon CloudWatch logging for an application.</p>
    #[serde(rename = "monitoringConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration_update: Option<MonitoringConfigurationUpdate>,
    /// <p>Describes updates to the parameters for how an application executes multiple tasks simultaneously.</p>
    #[serde(rename = "parallelismConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_configuration_update: Option<ParallelismConfigurationUpdate>,
}

/// <p>Describes the starting parameters for a Flink-based Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FlinkRunConfiguration {
    /// <p><p>When restoring from a snapshot, specifies whether the runtime is allowed to skip a state that cannot be mapped to the new program. This will happen if the program is updated between snapshots to remove stateful parameters, and state data in the snapshot no longer corresponds to valid application data. For more information, see <a href="https://ci.apache.org/projects/flink/flink-docs-release-1.8/ops/state/savepoints.html#allowing-non-restored-state"> Allowing Non-Restored State</a> in the <a href="https://ci.apache.org/projects/flink/flink-docs-release-1.8/">Apache Flink documentation</a>.</p> <note> <p>This value defaults to <code>false</code>. If you update your application without specifying this parameter, <code>AllowNonRestoredState</code> will be set to <code>false</code>, even if it was previously set to <code>true</code>.</p> </note></p>
    #[serde(rename = "allowNonRestoredState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_non_restored_state: Option<bool>,
}

/// <p>The configuration of the Glue Data Catalog that you use for Apache Flink SQL queries and table API transforms that you write in an application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GlueDataCatalogConfiguration {
    /// <p>The Amazon Resource Name (ARN) of the database.</p>
    #[serde(rename = "databaseARN")]
    pub database_arn: String,
}

/// <p>The configuration of the Glue Data Catalog that you use for Apache Flink SQL queries and table API transforms that you write in an application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GlueDataCatalogConfigurationDescription {
    /// <p>The Amazon Resource Name (ARN) of the database.</p>
    #[serde(rename = "databaseARN")]
    pub database_arn: String,
}

/// <p>Updates to the configuration of the Glue Data Catalog that you use for SQL queries that you write in a Kinesis Data Analytics Studio notebook.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GlueDataCatalogConfigurationUpdate {
    /// <p>The updated Amazon Resource Name (ARN) of the database.</p>
    #[serde(rename = "databaseARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_arn_update: Option<String>,
}

/// <p>When you configure the application input for a SQL-based Kinesis Data Analytics application, you specify the streaming source, the in-application stream name that is created, and the mapping between the two. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Input {
    /// <p>Describes the number of in-application streams to create. </p>
    #[serde(rename = "inputParallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parallelism: Option<InputParallelism>,
    /// <p>The <a>InputProcessingConfiguration</a> for the input. An input processor transforms records as they are received from the stream, before the application's SQL code executes. Currently, the only input processing configuration available is <a>InputLambdaProcessor</a>. </p>
    #[serde(rename = "inputProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_processing_configuration: Option<InputProcessingConfiguration>,
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns in the in-application stream that is being created.</p> <p>Also used to describe the format of the reference data source.</p>
    #[serde(rename = "inputSchema")]
    pub input_schema: SourceSchema,
    /// <p>If the streaming source is an Amazon Kinesis Data Firehose delivery stream, identifies the delivery stream's ARN.</p>
    #[serde(rename = "kinesisFirehoseInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_input: Option<KinesisFirehoseInput>,
    /// <p>If the streaming source is an Amazon Kinesis data stream, identifies the stream's Amazon Resource Name (ARN). </p>
    #[serde(rename = "kinesisStreamsInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_input: Option<KinesisStreamsInput>,
    /// <p>The name prefix to use when creating an in-application stream. Suppose that you specify a prefix "<code>MyInApplicationStream</code>." Kinesis Data Analytics then creates one or more (as per the <code>InputParallelism</code> count you specified) in-application streams with the names "<code>MyInApplicationStream_001</code>," "<code>MyInApplicationStream_002</code>," and so on. </p>
    #[serde(rename = "namePrefix")]
    pub name_prefix: String,
}

/// <p>Describes the application input configuration for a SQL-based Kinesis Data Analytics application. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputDescription {
    /// <p>Returns the in-application stream names that are mapped to the stream source. </p>
    #[serde(rename = "inAppStreamNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_stream_names: Option<Vec<String>>,
    /// <p>The input ID that is associated with the application input. This is the ID that Kinesis Data Analytics assigns to each input configuration that you add to your application. </p>
    #[serde(rename = "inputId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_id: Option<String>,
    /// <p>Describes the configured parallelism (number of in-application streams mapped to the streaming source). </p>
    #[serde(rename = "inputParallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parallelism: Option<InputParallelism>,
    /// <p>The description of the preprocessor that executes on records in this input before the application's code is run. </p>
    #[serde(rename = "inputProcessingConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_processing_configuration_description: Option<InputProcessingConfigurationDescription>,
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns in the in-application stream that is being created. </p>
    #[serde(rename = "inputSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<SourceSchema>,
    /// <p>The point at which the application is configured to read from the input stream.</p>
    #[serde(rename = "inputStartingPositionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_starting_position_configuration: Option<InputStartingPositionConfiguration>,
    /// <p>If a Kinesis Data Firehose delivery stream is configured as a streaming source, provides the delivery stream's ARN. </p>
    #[serde(rename = "kinesisFirehoseInputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_input_description: Option<KinesisFirehoseInputDescription>,
    /// <p>If a Kinesis data stream is configured as a streaming source, provides the Kinesis data stream's Amazon Resource Name (ARN). </p>
    #[serde(rename = "kinesisStreamsInputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_input_description: Option<KinesisStreamsInputDescription>,
    /// <p>The in-application name prefix.</p>
    #[serde(rename = "namePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
}

/// <p>An object that contains the Amazon Resource Name (ARN) of the AWS Lambda function that is used to preprocess records in the stream in a SQL-based Kinesis Data Analytics application. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputLambdaProcessor {
    /// <p><p>The ARN of the AWS Lambda function that operates on records in the stream.</p> <note> <p>To specify an earlier version of the Lambda function than the latest, include the Lambda function version in the Lambda function ARN. For more information about Lambda ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-lambda">Example ARNs: AWS Lambda</a> </p> </note></p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

/// <p>For a SQL-based Kinesis Data Analytics application, an object that contains the Amazon Resource Name (ARN) of the AWS Lambda function that is used to preprocess records in the stream.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputLambdaProcessorDescription {
    /// <p><p>The ARN of the AWS Lambda function that is used to preprocess the records in the stream.</p> <note> <p>To specify an earlier version of the Lambda function than the latest, include the Lambda function version in the Lambda function ARN. For more information about Lambda ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-lambda">Example ARNs: AWS Lambda</a> </p> </note></p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p><p>The ARN of the IAM role that is used to access the AWS Lambda function.</p> <note> <p>Provided for backward compatibility. Applications that are created with the current API version have an application-level service execution role rather than a resource-level role.</p> </note></p>
    #[serde(rename = "roleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>For a SQL-based Kinesis Data Analytics application, represents an update to the <a>InputLambdaProcessor</a> that is used to preprocess the records in the stream.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputLambdaProcessorUpdate {
    /// <p><p>The Amazon Resource Name (ARN) of the new AWS Lambda function that is used to preprocess the records in the stream.</p> <note> <p>To specify an earlier version of the Lambda function than the latest, include the Lambda function version in the Lambda function ARN. For more information about Lambda ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-lambda">Example ARNs: AWS Lambda</a> </p> </note></p>
    #[serde(rename = "resourceARNUpdate")]
    pub resource_arn_update: String,
}

/// <p>For a SQL-based Kinesis Data Analytics application, describes the number of in-application streams to create for a given streaming source. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputParallelism {
    /// <p>The number of in-application streams to create.</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

/// <p>For a SQL-based Kinesis Data Analytics application, provides updates to the parallelism count.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputParallelismUpdate {
    /// <p>The number of in-application streams to create for the specified streaming source.</p>
    #[serde(rename = "countUpdate")]
    pub count_update: i64,
}

/// <p>For a SQL-based Kinesis Data Analytics application, describes a processor that is used to preprocess the records in the stream before being processed by your application code. Currently, the only input processor available is <a href="https://docs.aws.amazon.com/lambda/">AWS Lambda</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputProcessingConfiguration {
    /// <p>The <a>InputLambdaProcessor</a> that is used to preprocess the records in the stream before being processed by your application code.</p>
    #[serde(rename = "inputLambdaProcessor")]
    pub input_lambda_processor: InputLambdaProcessor,
}

/// <p>For a SQL-based Kinesis Data Analytics application, provides the configuration information about an input processor. Currently, the only input processor available is <a href="https://docs.aws.amazon.com/lambda/">AWS Lambda</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputProcessingConfigurationDescription {
    /// <p>Provides configuration information about the associated <a>InputLambdaProcessorDescription</a> </p>
    #[serde(rename = "inputLambdaProcessorDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_lambda_processor_description: Option<InputLambdaProcessorDescription>,
}

/// <p>For a SQL-based Kinesis Data Analytics application, describes updates to an <a>InputProcessingConfiguration</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputProcessingConfigurationUpdate {
    /// <p>Provides update information for an <a>InputLambdaProcessor</a>.</p>
    #[serde(rename = "inputLambdaProcessorUpdate")]
    pub input_lambda_processor_update: InputLambdaProcessorUpdate,
}

/// <p>Describes updates for an SQL-based Kinesis Data Analytics application's input schema.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputSchemaUpdate {
    /// <p>A list of <code>RecordColumn</code> objects. Each object describes the mapping of the streaming source element to the corresponding column in the in-application stream.</p>
    #[serde(rename = "recordColumnUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_column_updates: Option<Vec<RecordColumn>>,
    /// <p>Specifies the encoding of the records in the streaming source; for example, UTF-8.</p>
    #[serde(rename = "recordEncodingUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_encoding_update: Option<String>,
    /// <p>Specifies the format of the records on the streaming source.</p>
    #[serde(rename = "recordFormatUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_format_update: Option<RecordFormat>,
}

/// <p>Describes the point at which the application reads from the streaming source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputStartingPositionConfiguration {
    /// <p><p>The starting position on the stream.</p> <ul> <li> <p> <code>NOW</code> - Start reading just after the most recent record in the stream, and start at the request timestamp that the customer issued.</p> </li> <li> <p> <code>TRIM<em>HORIZON</code> - Start reading at the last untrimmed record in the stream, which is the oldest record available in the stream. This option is not available for an Amazon Kinesis Data Firehose delivery stream.</p> </li> <li> <p> <code>LAST</em>STOPPED_POINT</code> - Resume reading from where the application last stopped reading.</p> </li> </ul></p>
    #[serde(rename = "inputStartingPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_starting_position: Option<String>,
}

/// <p>For a SQL-based Kinesis Data Analytics application, describes updates to a specific input configuration (identified by the <code>InputId</code> of an application). </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputUpdate {
    /// <p>The input ID of the application input to be updated.</p>
    #[serde(rename = "inputId")]
    pub input_id: String,
    /// <p>Describes the parallelism updates (the number of in-application streams Kinesis Data Analytics creates for the specific streaming source).</p>
    #[serde(rename = "inputParallelismUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parallelism_update: Option<InputParallelismUpdate>,
    /// <p>Describes updates to an <a>InputProcessingConfiguration</a>.</p>
    #[serde(rename = "inputProcessingConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_processing_configuration_update: Option<InputProcessingConfigurationUpdate>,
    /// <p>Describes the data format on the streaming source, and how record elements on the streaming source map to columns of the in-application stream that is created.</p>
    #[serde(rename = "inputSchemaUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema_update: Option<InputSchemaUpdate>,
    /// <p>If a Kinesis Data Firehose delivery stream is the streaming source to be updated, provides an updated stream ARN.</p>
    #[serde(rename = "kinesisFirehoseInputUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_input_update: Option<KinesisFirehoseInputUpdate>,
    /// <p>If a Kinesis data stream is the streaming source to be updated, provides an updated stream Amazon Resource Name (ARN).</p>
    #[serde(rename = "kinesisStreamsInputUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_input_update: Option<KinesisStreamsInputUpdate>,
    /// <p>The name prefix for in-application streams that Kinesis Data Analytics creates for the specific streaming source.</p>
    #[serde(rename = "namePrefixUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix_update: Option<String>,
}

/// <p>For a SQL-based Kinesis Data Analytics application, provides additional mapping information when JSON is the record format on the streaming source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JSONMappingParameters {
    /// <p>The path to the top-level parent that contains the records.</p>
    #[serde(rename = "recordRowPath")]
    pub record_row_path: String,
}

/// <p>For a SQL-based Kinesis Data Analytics application, identifies a Kinesis Data Firehose delivery stream as the streaming source. You provide the delivery stream's Amazon Resource Name (ARN).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct KinesisFirehoseInput {
    /// <p>The Amazon Resource Name (ARN) of the delivery stream.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

/// <p>Describes the Amazon Kinesis Data Firehose delivery stream that is configured as the streaming source in the application input configuration. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KinesisFirehoseInputDescription {
    /// <p>The Amazon Resource Name (ARN) of the delivery stream.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p><p>The ARN of the IAM role that Kinesis Data Analytics assumes to access the stream.</p> <note> <p>Provided for backward compatibility. Applications that are created with the current API version have an application-level service execution role rather than a resource-level role.</p> </note></p>
    #[serde(rename = "roleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>For a SQL-based Kinesis Data Analytics application, when updating application input configuration, provides information about a Kinesis Data Firehose delivery stream as the streaming source.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct KinesisFirehoseInputUpdate {
    /// <p>The Amazon Resource Name (ARN) of the input delivery stream to read.</p>
    #[serde(rename = "resourceARNUpdate")]
    pub resource_arn_update: String,
}

/// <p>For a SQL-based Kinesis Data Analytics application, when configuring application output, identifies a Kinesis Data Firehose delivery stream as the destination. You provide the stream Amazon Resource Name (ARN) of the delivery stream. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct KinesisFirehoseOutput {
    /// <p>The ARN of the destination delivery stream to write to.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

/// <p>For a SQL-based Kinesis Data Analytics application's output, describes the Kinesis Data Firehose delivery stream that is configured as its destination.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KinesisFirehoseOutputDescription {
    /// <p>The Amazon Resource Name (ARN) of the delivery stream.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p><p>The ARN of the IAM role that Kinesis Data Analytics can assume to access the stream.</p> <note> <p>Provided for backward compatibility. Applications that are created with the current API version have an application-level service execution role rather than a resource-level role.</p> </note></p>
    #[serde(rename = "roleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>For a SQL-based Kinesis Data Analytics application, when updating an output configuration using the <a>UpdateApplication</a> operation, provides information about a Kinesis Data Firehose delivery stream that is configured as the destination.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct KinesisFirehoseOutputUpdate {
    /// <p>The Amazon Resource Name (ARN) of the delivery stream to write to. </p>
    #[serde(rename = "resourceARNUpdate")]
    pub resource_arn_update: String,
}

/// <p> Identifies a Kinesis data stream as the streaming source. You provide the stream's Amazon Resource Name (ARN).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct KinesisStreamsInput {
    /// <p>The ARN of the input Kinesis data stream to read.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

/// <p>For a SQL-based Kinesis Data Analytics application, describes the Kinesis data stream that is configured as the streaming source in the application input configuration. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KinesisStreamsInputDescription {
    /// <p>The Amazon Resource Name (ARN) of the Kinesis data stream.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p><p>The ARN of the IAM role that Kinesis Data Analytics can assume to access the stream.</p> <note> <p>Provided for backward compatibility. Applications that are created with the current API version have an application-level service execution role rather than a resource-level role.</p> </note></p>
    #[serde(rename = "roleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>When you update the input configuration for a SQL-based Kinesis Data Analytics application, provides information about a Kinesis stream as the streaming source.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct KinesisStreamsInputUpdate {
    /// <p>The Amazon Resource Name (ARN) of the input Kinesis data stream to read.</p>
    #[serde(rename = "resourceARNUpdate")]
    pub resource_arn_update: String,
}

/// <p>When you configure a SQL-based Kinesis Data Analytics application's output, identifies a Kinesis data stream as the destination. You provide the stream Amazon Resource Name (ARN). </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct KinesisStreamsOutput {
    /// <p>The ARN of the destination Kinesis data stream to write to.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

/// <p>For an SQL-based Kinesis Data Analytics application's output, describes the Kinesis data stream that is configured as its destination. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KinesisStreamsOutputDescription {
    /// <p>The Amazon Resource Name (ARN) of the Kinesis data stream.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p><p>The ARN of the IAM role that Kinesis Data Analytics can assume to access the stream.</p> <note> <p>Provided for backward compatibility. Applications that are created with the current API version have an application-level service execution role rather than a resource-level role.</p> </note></p>
    #[serde(rename = "roleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>When you update a SQL-based Kinesis Data Analytics application's output configuration using the <a>UpdateApplication</a> operation, provides information about a Kinesis data stream that is configured as the destination.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct KinesisStreamsOutputUpdate {
    /// <p>The Amazon Resource Name (ARN) of the Kinesis data stream where you want to write the output.</p>
    #[serde(rename = "resourceARNUpdate")]
    pub resource_arn_update: String,
}

/// <p>When you configure a SQL-based Kinesis Data Analytics application's output, identifies an AWS Lambda function as the destination. You provide the function Amazon Resource Name (ARN) of the Lambda function. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LambdaOutput {
    /// <p><p>The Amazon Resource Name (ARN) of the destination Lambda function to write to.</p> <note> <p>To specify an earlier version of the Lambda function than the latest, include the Lambda function version in the Lambda function ARN. For more information about Lambda ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-lambda">Example ARNs: AWS Lambda</a> </p> </note></p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

/// <p>For a SQL-based Kinesis Data Analytics application's output, describes the AWS Lambda function that is configured as its destination. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LambdaOutputDescription {
    /// <p>The Amazon Resource Name (ARN) of the destination Lambda function.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p><p>The ARN of the IAM role that Kinesis Data Analytics can assume to write to the destination function.</p> <note> <p>Provided for backward compatibility. Applications that are created with the current API version have an application-level service execution role rather than a resource-level role.</p> </note></p>
    #[serde(rename = "roleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>When you update an SQL-based Kinesis Data Analytics application's output configuration using the <a>UpdateApplication</a> operation, provides information about an AWS Lambda function that is configured as the destination.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct LambdaOutputUpdate {
    /// <p><p>The Amazon Resource Name (ARN) of the destination AWS Lambda function.</p> <note> <p>To specify an earlier version of the Lambda function than the latest, include the Lambda function version in the Lambda function ARN. For more information about Lambda ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-lambda">Example ARNs: AWS Lambda</a> </p> </note></p>
    #[serde(rename = "resourceARNUpdate")]
    pub resource_arn_update: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListApplicationSnapshotsRequest {
    /// <p>The name of an existing application.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The maximum number of application snapshots to list.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter if you receive a <code>NextToken</code> response in a previous request that indicates that there is more output available. Set it to the value of the previous call's <code>NextToken</code> response to indicate where the output should continue from. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListApplicationSnapshotsResponse {
    /// <p>The token for the next set of results, or <code>null</code> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A collection of objects containing information about the application snapshots.</p>
    #[serde(rename = "snapshotSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_summaries: Option<Vec<SnapshotDetails>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListApplicationVersionsRequest {
    /// <p>The name of the application for which you want to list all versions.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The maximum number of versions to list in this invocation of the operation.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If a previous invocation of this operation returned a pagination token, pass it into this value to retrieve the next set of results. For more information about pagination, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/pagination.html">Using the AWS Command Line Interface's Pagination Options</a>.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListApplicationVersionsResponse {
    /// <p>A list of the application versions and the associated configuration summaries. The list includes application versions that were rolled back.</p> <p>To get the complete description of a specific application version, invoke the <a>DescribeApplicationVersion</a> operation.</p>
    #[serde(rename = "applicationVersionSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_summaries: Option<Vec<ApplicationVersionSummary>>,
    /// <p>The pagination token for the next set of results, or <code>null</code> if there are no additional results. To retrieve the next set of items, pass this token into a subsequent invocation of this operation. For more information about pagination, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/pagination.html">Using the AWS Command Line Interface's Pagination Options</a>.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListApplicationsRequest {
    /// <p>The maximum number of applications to list.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>If a previous command returned a pagination token, pass it into this value to retrieve the next set of results. For more information about pagination, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/pagination.html">Using the AWS Command Line Interface's Pagination Options</a>.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListApplicationsResponse {
    /// <p>A list of <code>ApplicationSummary</code> objects.</p>
    #[serde(rename = "applicationSummaries")]
    pub application_summaries: Vec<ApplicationSummary>,
    /// <p>The pagination token for the next set of results, or <code>null</code> if there are no additional results. Pass this token into a subsequent command to retrieve the next set of items For more information about pagination, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/pagination.html">Using the AWS Command Line Interface's Pagination Options</a>.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the application for which to retrieve tags.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The key-value tags assigned to the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>When you configure a SQL-based Kinesis Data Analytics application's input at the time of creating or updating an application, provides additional mapping information specific to the record format (such as JSON, CSV, or record fields delimited by some delimiter) on the streaming source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MappingParameters {
    /// <p>Provides additional mapping information when the record format uses delimiters (for example, CSV).</p>
    #[serde(rename = "cSVMappingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_mapping_parameters: Option<CSVMappingParameters>,
    /// <p>Provides additional mapping information when JSON is the record format on the streaming source.</p>
    #[serde(rename = "jSONMappingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_mapping_parameters: Option<JSONMappingParameters>,
}

/// <p>The information required to specify a Maven reference. You can use Maven references to specify dependency JAR files.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MavenReference {
    /// <p>The artifact ID of the Maven reference.</p>
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    /// <p>The group ID of the Maven reference.</p>
    #[serde(rename = "groupId")]
    pub group_id: String,
    /// <p>The version of the Maven reference.</p>
    #[serde(rename = "version")]
    pub version: String,
}

/// <p>Describes configuration parameters for Amazon CloudWatch logging for an application. For more information about CloudWatch logging, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/monitoring-overview.html">Monitoring</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MonitoringConfiguration {
    /// <p>Describes whether to use the default CloudWatch logging configuration for an application. You must set this property to <code>CUSTOM</code> in order to set the <code>LogLevel</code> or <code>MetricsLevel</code> parameters.</p>
    #[serde(rename = "configurationType")]
    pub configuration_type: String,
    /// <p>Describes the verbosity of the CloudWatch Logs for an application.</p>
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    /// <p>Describes the granularity of the CloudWatch Logs for an application. The <code>Parallelism</code> level is not recommended for applications with a Parallelism over 64 due to excessive costs.</p>
    #[serde(rename = "metricsLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_level: Option<String>,
}

/// <p>Describes configuration parameters for CloudWatch logging for an application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MonitoringConfigurationDescription {
    /// <p>Describes whether to use the default CloudWatch logging configuration for an application.</p>
    #[serde(rename = "configurationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type: Option<String>,
    /// <p>Describes the verbosity of the CloudWatch Logs for an application.</p>
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    /// <p>Describes the granularity of the CloudWatch Logs for an application.</p>
    #[serde(rename = "metricsLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_level: Option<String>,
}

/// <p>Describes updates to configuration parameters for Amazon CloudWatch logging for an application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MonitoringConfigurationUpdate {
    /// <p>Describes updates to whether to use the default CloudWatch logging configuration for an application. You must set this property to <code>CUSTOM</code> in order to set the <code>LogLevel</code> or <code>MetricsLevel</code> parameters.</p>
    #[serde(rename = "configurationTypeUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type_update: Option<String>,
    /// <p>Describes updates to the verbosity of the CloudWatch Logs for an application.</p>
    #[serde(rename = "logLevelUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level_update: Option<String>,
    /// <p>Describes updates to the granularity of the CloudWatch Logs for an application. The <code>Parallelism</code> level is not recommended for applications with a Parallelism over 64 due to excessive costs.</p>
    #[serde(rename = "metricsLevelUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_level_update: Option<String>,
}

/// <p><p> Describes a SQL-based Kinesis Data Analytics application&#39;s output configuration, in which you identify an in-application stream and a destination where you want the in-application stream data to be written. The destination can be a Kinesis data stream or a Kinesis Data Firehose delivery stream. </p> <p/></p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Output {
    /// <p>Describes the data format when records are written to the destination. </p>
    #[serde(rename = "destinationSchema")]
    pub destination_schema: DestinationSchema,
    /// <p>Identifies a Kinesis Data Firehose delivery stream as the destination.</p>
    #[serde(rename = "kinesisFirehoseOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_output: Option<KinesisFirehoseOutput>,
    /// <p>Identifies a Kinesis data stream as the destination.</p>
    #[serde(rename = "kinesisStreamsOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_output: Option<KinesisStreamsOutput>,
    /// <p>Identifies an AWS Lambda function as the destination.</p>
    #[serde(rename = "lambdaOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_output: Option<LambdaOutput>,
    /// <p>The name of the in-application stream.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>For a SQL-based Kinesis Data Analytics application, describes the application output configuration, which includes the in-application stream name and the destination where the stream data is written. The destination can be a Kinesis data stream or a Kinesis Data Firehose delivery stream. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OutputDescription {
    /// <p>The data format used for writing data to the destination.</p>
    #[serde(rename = "destinationSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_schema: Option<DestinationSchema>,
    /// <p>Describes the Kinesis Data Firehose delivery stream that is configured as the destination where output is written.</p>
    #[serde(rename = "kinesisFirehoseOutputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_output_description: Option<KinesisFirehoseOutputDescription>,
    /// <p>Describes the Kinesis data stream that is configured as the destination where output is written.</p>
    #[serde(rename = "kinesisStreamsOutputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_output_description: Option<KinesisStreamsOutputDescription>,
    /// <p>Describes the Lambda function that is configured as the destination where output is written.</p>
    #[serde(rename = "lambdaOutputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_output_description: Option<LambdaOutputDescription>,
    /// <p>The name of the in-application stream that is configured as output.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier for the output configuration.</p>
    #[serde(rename = "outputId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_id: Option<String>,
}

/// <p> For a SQL-based Kinesis Data Analytics application, describes updates to the output configuration identified by the <code>OutputId</code>. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OutputUpdate {
    /// <p>Describes the data format when records are written to the destination. </p>
    #[serde(rename = "destinationSchemaUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_schema_update: Option<DestinationSchema>,
    /// <p>Describes a Kinesis Data Firehose delivery stream as the destination for the output.</p>
    #[serde(rename = "kinesisFirehoseOutputUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_output_update: Option<KinesisFirehoseOutputUpdate>,
    /// <p>Describes a Kinesis data stream as the destination for the output.</p>
    #[serde(rename = "kinesisStreamsOutputUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_output_update: Option<KinesisStreamsOutputUpdate>,
    /// <p>Describes an AWS Lambda function as the destination for the output.</p>
    #[serde(rename = "lambdaOutputUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_output_update: Option<LambdaOutputUpdate>,
    /// <p>If you want to specify a different in-application stream for this output configuration, use this field to specify the new in-application stream name.</p>
    #[serde(rename = "nameUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_update: Option<String>,
    /// <p>Identifies the specific output configuration that you want to update.</p>
    #[serde(rename = "outputId")]
    pub output_id: String,
}

/// <p>Describes parameters for how a Flink-based Kinesis Data Analytics application executes multiple tasks simultaneously. For more information about parallelism, see <a href="https://ci.apache.org/projects/flink/flink-docs-release-1.8/dev/parallel.html">Parallel Execution</a> in the <a href="https://ci.apache.org/projects/flink/flink-docs-release-1.8/">Apache Flink Documentation</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ParallelismConfiguration {
    /// <p>Describes whether the Kinesis Data Analytics service can increase the parallelism of the application in response to increased throughput.</p>
    #[serde(rename = "autoScalingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_enabled: Option<bool>,
    /// <p>Describes whether the application uses the default parallelism for the Kinesis Data Analytics service. You must set this property to <code>CUSTOM</code> in order to change your application's <code>AutoScalingEnabled</code>, <code>Parallelism</code>, or <code>ParallelismPerKPU</code> properties.</p>
    #[serde(rename = "configurationType")]
    pub configuration_type: String,
    /// <p>Describes the initial number of parallel tasks that a Flink-based Kinesis Data Analytics application can perform. If <code>AutoScalingEnabled</code> is set to True, Kinesis Data Analytics increases the <code>CurrentParallelism</code> value in response to application load. The service can increase the <code>CurrentParallelism</code> value up to the maximum parallelism, which is <code>ParalellismPerKPU</code> times the maximum KPUs for the application. The maximum KPUs for an application is 32 by default, and can be increased by requesting a limit increase. If application load is reduced, the service can reduce the <code>CurrentParallelism</code> value down to the <code>Parallelism</code> setting.</p>
    #[serde(rename = "parallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i64>,
    /// <p>Describes the number of parallel tasks that a Flink-based Kinesis Data Analytics application can perform per Kinesis Processing Unit (KPU) used by the application. For more information about KPUs, see <a href="http://aws.amazon.com/kinesis/data-analytics/pricing/">Amazon Kinesis Data Analytics Pricing</a>.</p>
    #[serde(rename = "parallelismPerKPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_per_kpu: Option<i64>,
}

/// <p>Describes parameters for how a Flink-based Kinesis Data Analytics application executes multiple tasks simultaneously.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ParallelismConfigurationDescription {
    /// <p>Describes whether the Kinesis Data Analytics service can increase the parallelism of the application in response to increased throughput.</p>
    #[serde(rename = "autoScalingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_enabled: Option<bool>,
    /// <p>Describes whether the application uses the default parallelism for the Kinesis Data Analytics service. </p>
    #[serde(rename = "configurationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type: Option<String>,
    /// <p>Describes the current number of parallel tasks that a Flink-based Kinesis Data Analytics application can perform. If <code>AutoScalingEnabled</code> is set to True, Kinesis Data Analytics can increase this value in response to application load. The service can increase this value up to the maximum parallelism, which is <code>ParalellismPerKPU</code> times the maximum KPUs for the application. The maximum KPUs for an application is 32 by default, and can be increased by requesting a limit increase. If application load is reduced, the service can reduce the <code>CurrentParallelism</code> value down to the <code>Parallelism</code> setting.</p>
    #[serde(rename = "currentParallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_parallelism: Option<i64>,
    /// <p>Describes the initial number of parallel tasks that a Flink-based Kinesis Data Analytics application can perform. If <code>AutoScalingEnabled</code> is set to True, then Kinesis Data Analytics can increase the <code>CurrentParallelism</code> value in response to application load. The service can increase <code>CurrentParallelism</code> up to the maximum parallelism, which is <code>ParalellismPerKPU</code> times the maximum KPUs for the application. The maximum KPUs for an application is 32 by default, and can be increased by requesting a limit increase. If application load is reduced, the service can reduce the <code>CurrentParallelism</code> value down to the <code>Parallelism</code> setting.</p>
    #[serde(rename = "parallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i64>,
    /// <p>Describes the number of parallel tasks that a Flink-based Kinesis Data Analytics application can perform per Kinesis Processing Unit (KPU) used by the application.</p>
    #[serde(rename = "parallelismPerKPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_per_kpu: Option<i64>,
}

/// <p>Describes updates to parameters for how an application executes multiple tasks simultaneously.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ParallelismConfigurationUpdate {
    /// <p>Describes updates to whether the Kinesis Data Analytics service can increase the parallelism of a Flink-based Kinesis Data Analytics application in response to increased throughput.</p>
    #[serde(rename = "autoScalingEnabledUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_enabled_update: Option<bool>,
    /// <p>Describes updates to whether the application uses the default parallelism for the Kinesis Data Analytics service, or if a custom parallelism is used. You must set this property to <code>CUSTOM</code> in order to change your application's <code>AutoScalingEnabled</code>, <code>Parallelism</code>, or <code>ParallelismPerKPU</code> properties.</p>
    #[serde(rename = "configurationTypeUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type_update: Option<String>,
    /// <p>Describes updates to the number of parallel tasks an application can perform per Kinesis Processing Unit (KPU) used by the application.</p>
    #[serde(rename = "parallelismPerKPUUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_per_kpu_update: Option<i64>,
    /// <p>Describes updates to the initial number of parallel tasks an application can perform. If <code>AutoScalingEnabled</code> is set to True, then Kinesis Data Analytics can increase the <code>CurrentParallelism</code> value in response to application load. The service can increase <code>CurrentParallelism</code> up to the maximum parallelism, which is <code>ParalellismPerKPU</code> times the maximum KPUs for the application. The maximum KPUs for an application is 32 by default, and can be increased by requesting a limit increase. If application load is reduced, the service will reduce <code>CurrentParallelism</code> down to the <code>Parallelism</code> setting.</p>
    #[serde(rename = "parallelismUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_update: Option<i64>,
}

/// <p>Property key-value pairs passed into an application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PropertyGroup {
    /// <p>Describes the key of an application execution property key-value pair.</p>
    #[serde(rename = "propertyGroupId")]
    pub property_group_id: String,
    /// <p>Describes the value of an application execution property key-value pair.</p>
    #[serde(rename = "propertyMap")]
    pub property_map: ::std::collections::HashMap<String, String>,
}

/// <p>For a SQL-based Kinesis Data Analytics application, describes the mapping of each data element in the streaming source to the corresponding column in the in-application stream.</p> <p>Also used to describe the format of the reference data source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RecordColumn {
    /// <p>A reference to the data element in the streaming input or the reference data source.</p>
    #[serde(rename = "mapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<String>,
    /// <p>The name of the column that is created in the in-application input stream or reference table.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The type of column created in the in-application input stream or reference table.</p>
    #[serde(rename = "sqlType")]
    pub sql_type: String,
}

/// <p> For a SQL-based Kinesis Data Analytics application, describes the record format and relevant mapping information that should be applied to schematize the records on the stream. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RecordFormat {
    /// <p>When you configure application input at the time of creating or updating an application, provides additional mapping information specific to the record format (such as JSON, CSV, or record fields delimited by some delimiter) on the streaming source.</p>
    #[serde(rename = "mappingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping_parameters: Option<MappingParameters>,
    /// <p>The type of record format.</p>
    #[serde(rename = "recordFormatType")]
    pub record_format_type: String,
}

/// <p>For a SQL-based Kinesis Data Analytics application, describes the reference data source by providing the source information (Amazon S3 bucket name and object key name), the resulting in-application table name that is created, and the necessary schema to map the data elements in the Amazon S3 object to the in-application table.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReferenceDataSource {
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.</p>
    #[serde(rename = "referenceSchema")]
    pub reference_schema: SourceSchema,
    /// <p>Identifies the S3 bucket and object that contains the reference data. A Kinesis Data Analytics application loads reference data only once. If the data changes, you call the <a>UpdateApplication</a> operation to trigger reloading of data into your application. </p>
    #[serde(rename = "s3ReferenceDataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_reference_data_source: Option<S3ReferenceDataSource>,
    /// <p>The name of the in-application table to create.</p>
    #[serde(rename = "tableName")]
    pub table_name: String,
}

/// <p>For a SQL-based Kinesis Data Analytics application, describes the reference data source configured for an application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReferenceDataSourceDescription {
    /// <p>The ID of the reference data source. This is the ID that Kinesis Data Analytics assigns when you add the reference data source to your application using the <a>CreateApplication</a> or <a>UpdateApplication</a> operation.</p>
    #[serde(rename = "referenceId")]
    pub reference_id: String,
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.</p>
    #[serde(rename = "referenceSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_schema: Option<SourceSchema>,
    /// <p>Provides the Amazon S3 bucket name, the object key name that contains the reference data. </p>
    #[serde(rename = "s3ReferenceDataSourceDescription")]
    pub s3_reference_data_source_description: S3ReferenceDataSourceDescription,
    /// <p>The in-application table name created by the specific reference data source configuration.</p>
    #[serde(rename = "tableName")]
    pub table_name: String,
}

/// <p>When you update a reference data source configuration for a SQL-based Kinesis Data Analytics application, this object provides all the updated values (such as the source bucket name and object key name), the in-application table name that is created, and updated mapping information that maps the data in the Amazon S3 object to the in-application reference table that is created.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReferenceDataSourceUpdate {
    /// <p>The ID of the reference data source that is being updated. You can use the <a>DescribeApplication</a> operation to get this value.</p>
    #[serde(rename = "referenceId")]
    pub reference_id: String,
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream. </p>
    #[serde(rename = "referenceSchemaUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_schema_update: Option<SourceSchema>,
    /// <p>Describes the S3 bucket name, object key name, and IAM role that Kinesis Data Analytics can assume to read the Amazon S3 object on your behalf and populate the in-application reference table.</p>
    #[serde(rename = "s3ReferenceDataSourceUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_reference_data_source_update: Option<S3ReferenceDataSourceUpdate>,
    /// <p>The in-application table name that is created by this update.</p>
    #[serde(rename = "tableNameUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name_update: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RollbackApplicationRequest {
    /// <p>The name of the application.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>The current application version ID. You can retrieve the application version ID using <a>DescribeApplication</a>.</p>
    #[serde(rename = "currentApplicationVersionId")]
    pub current_application_version_id: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RollbackApplicationResponse {
    #[serde(rename = "applicationDetail")]
    pub application_detail: ApplicationDetail,
}

/// <p>Describes the starting parameters for an Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RunConfiguration {
    /// <p>Describes the restore behavior of a restarting application.</p>
    #[serde(rename = "applicationRestoreConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_restore_configuration: Option<ApplicationRestoreConfiguration>,
    /// <p>Describes the starting parameters for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "flinkRunConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flink_run_configuration: Option<FlinkRunConfiguration>,
    /// <p>Describes the starting parameters for a SQL-based Kinesis Data Analytics application application.</p>
    #[serde(rename = "sqlRunConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_run_configurations: Option<Vec<SqlRunConfiguration>>,
}

/// <p>Describes the starting properties for a Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RunConfigurationDescription {
    /// <p>Describes the restore behavior of a restarting application.</p>
    #[serde(rename = "applicationRestoreConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_restore_configuration_description: Option<ApplicationRestoreConfiguration>,
    #[serde(rename = "flinkRunConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flink_run_configuration_description: Option<FlinkRunConfiguration>,
}

/// <p>Describes the updates to the starting parameters for a Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RunConfigurationUpdate {
    /// <p>Describes updates to the restore behavior of a restarting application.</p>
    #[serde(rename = "applicationRestoreConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_restore_configuration: Option<ApplicationRestoreConfiguration>,
    /// <p>Describes the starting parameters for a Flink-based Kinesis Data Analytics application.</p>
    #[serde(rename = "flinkRunConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flink_run_configuration: Option<FlinkRunConfiguration>,
}

/// <p>Describes the location of an application's code stored in an S3 bucket.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct S3ApplicationCodeLocationDescription {
    /// <p>The Amazon Resource Name (ARN) for the S3 bucket containing the application code.</p>
    #[serde(rename = "bucketARN")]
    pub bucket_arn: String,
    /// <p>The file key for the object containing the application code.</p>
    #[serde(rename = "fileKey")]
    pub file_key: String,
    /// <p>The version of the object containing the application code.</p>
    #[serde(rename = "objectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_version: Option<String>,
}

/// <p>For a SQL-based Kinesis Data Analytics application, provides a description of an Amazon S3 data source, including the Amazon Resource Name (ARN) of the S3 bucket and the name of the Amazon S3 object that contains the data.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3Configuration {
    /// <p>The ARN of the S3 bucket that contains the data.</p>
    #[serde(rename = "bucketARN")]
    pub bucket_arn: String,
    /// <p>The name of the object that contains the data.</p>
    #[serde(rename = "fileKey")]
    pub file_key: String,
}

/// <p>The S3 bucket that holds the application information.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3ContentBaseLocation {
    /// <p>The base path for the S3 bucket.</p>
    #[serde(rename = "basePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the S3 bucket.</p>
    #[serde(rename = "bucketARN")]
    pub bucket_arn: String,
}

/// <p>The description of the S3 base location that holds the application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct S3ContentBaseLocationDescription {
    /// <p>The base path for the S3 bucket.</p>
    #[serde(rename = "basePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the S3 bucket.</p>
    #[serde(rename = "bucketARN")]
    pub bucket_arn: String,
}

/// <p>The information required to update the S3 base location that holds the application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3ContentBaseLocationUpdate {
    /// <p>The updated S3 bucket path.</p>
    #[serde(rename = "basePathUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path_update: Option<String>,
    /// <p>The updated Amazon Resource Name (ARN) of the S3 bucket.</p>
    #[serde(rename = "bucketARNUpdate")]
    pub bucket_arn_update: String,
}

/// <p>For a Kinesis Data Analytics application provides a description of an Amazon S3 object, including the Amazon Resource Name (ARN) of the S3 bucket, the name of the Amazon S3 object that contains the data, and the version number of the Amazon S3 object that contains the data. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3ContentLocation {
    /// <p>The Amazon Resource Name (ARN) for the S3 bucket containing the application code.</p>
    #[serde(rename = "bucketARN")]
    pub bucket_arn: String,
    /// <p>The file key for the object containing the application code.</p>
    #[serde(rename = "fileKey")]
    pub file_key: String,
    /// <p>The version of the object containing the application code.</p>
    #[serde(rename = "objectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_version: Option<String>,
}

/// <p>Describes an update for the Amazon S3 code content location for an application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3ContentLocationUpdate {
    /// <p>The new Amazon Resource Name (ARN) for the S3 bucket containing the application code.</p>
    #[serde(rename = "bucketARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_arn_update: Option<String>,
    /// <p>The new file key for the object containing the application code.</p>
    #[serde(rename = "fileKeyUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_key_update: Option<String>,
    /// <p>The new version of the object containing the application code.</p>
    #[serde(rename = "objectVersionUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_version_update: Option<String>,
}

/// <p>For a SQL-based Kinesis Data Analytics application, identifies the Amazon S3 bucket and object that contains the reference data.</p> <p>A Kinesis Data Analytics application loads reference data only once. If the data changes, you call the <a>UpdateApplication</a> operation to trigger reloading of data into your application. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3ReferenceDataSource {
    /// <p>The Amazon Resource Name (ARN) of the S3 bucket.</p>
    #[serde(rename = "bucketARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_arn: Option<String>,
    /// <p>The object key name containing the reference data.</p>
    #[serde(rename = "fileKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_key: Option<String>,
}

/// <p>For a SQL-based Kinesis Data Analytics application, provides the bucket name and object key name that stores the reference data.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct S3ReferenceDataSourceDescription {
    /// <p>The Amazon Resource Name (ARN) of the S3 bucket.</p>
    #[serde(rename = "bucketARN")]
    pub bucket_arn: String,
    /// <p>Amazon S3 object key name.</p>
    #[serde(rename = "fileKey")]
    pub file_key: String,
    /// <p><p>The ARN of the IAM role that Kinesis Data Analytics can assume to read the Amazon S3 object on your behalf to populate the in-application reference table. </p> <note> <p>Provided for backward compatibility. Applications that are created with the current API version have an application-level service execution role rather than a resource-level role.</p> </note></p>
    #[serde(rename = "referenceRoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_role_arn: Option<String>,
}

/// <p>For a SQL-based Kinesis Data Analytics application, describes the Amazon S3 bucket name and object key name for an in-application reference table. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct S3ReferenceDataSourceUpdate {
    /// <p>The Amazon Resource Name (ARN) of the S3 bucket.</p>
    #[serde(rename = "bucketARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_arn_update: Option<String>,
    /// <p>The object key name.</p>
    #[serde(rename = "fileKeyUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_key_update: Option<String>,
}

/// <p>Provides details about a snapshot of application state.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SnapshotDetails {
    /// <p>The current application version ID when the snapshot was created.</p>
    #[serde(rename = "applicationVersionId")]
    pub application_version_id: i64,
    /// <p>The timestamp of the application snapshot.</p>
    #[serde(rename = "snapshotCreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_creation_timestamp: Option<f64>,
    /// <p>The identifier for the application snapshot.</p>
    #[serde(rename = "snapshotName")]
    pub snapshot_name: String,
    /// <p>The status of the application snapshot.</p>
    #[serde(rename = "snapshotStatus")]
    pub snapshot_status: String,
}

/// <p>For a SQL-based Kinesis Data Analytics application, describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SourceSchema {
    /// <p>A list of <code>RecordColumn</code> objects. </p>
    #[serde(rename = "recordColumns")]
    pub record_columns: Vec<RecordColumn>,
    /// <p>Specifies the encoding of the records in the streaming source. For example, UTF-8.</p>
    #[serde(rename = "recordEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_encoding: Option<String>,
    /// <p>Specifies the format of the records on the streaming source.</p>
    #[serde(rename = "recordFormat")]
    pub record_format: RecordFormat,
}

/// <p>Describes the inputs, outputs, and reference data sources for a SQL-based Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SqlApplicationConfiguration {
    /// <p>The array of <a>Input</a> objects describing the input streams used by the application.</p>
    #[serde(rename = "inputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<Input>>,
    /// <p>The array of <a>Output</a> objects describing the destination streams used by the application.</p>
    #[serde(rename = "outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
    /// <p>The array of <a>ReferenceDataSource</a> objects describing the reference data sources used by the application.</p>
    #[serde(rename = "referenceDataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_data_sources: Option<Vec<ReferenceDataSource>>,
}

/// <p>Describes the inputs, outputs, and reference data sources for a SQL-based Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SqlApplicationConfigurationDescription {
    /// <p>The array of <a>InputDescription</a> objects describing the input streams used by the application.</p>
    #[serde(rename = "inputDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_descriptions: Option<Vec<InputDescription>>,
    /// <p>The array of <a>OutputDescription</a> objects describing the destination streams used by the application.</p>
    #[serde(rename = "outputDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_descriptions: Option<Vec<OutputDescription>>,
    /// <p>The array of <a>ReferenceDataSourceDescription</a> objects describing the reference data sources used by the application.</p>
    #[serde(rename = "referenceDataSourceDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_data_source_descriptions: Option<Vec<ReferenceDataSourceDescription>>,
}

/// <p>Describes updates to the input streams, destination streams, and reference data sources for a SQL-based Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SqlApplicationConfigurationUpdate {
    /// <p>The array of <a>InputUpdate</a> objects describing the new input streams used by the application.</p>
    #[serde(rename = "inputUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_updates: Option<Vec<InputUpdate>>,
    /// <p>The array of <a>OutputUpdate</a> objects describing the new destination streams used by the application.</p>
    #[serde(rename = "outputUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_updates: Option<Vec<OutputUpdate>>,
    /// <p>The array of <a>ReferenceDataSourceUpdate</a> objects describing the new reference data sources used by the application.</p>
    #[serde(rename = "referenceDataSourceUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_data_source_updates: Option<Vec<ReferenceDataSourceUpdate>>,
}

/// <p>Describes the starting parameters for a SQL-based Kinesis Data Analytics application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct SqlRunConfiguration {
    /// <p>The input source ID. You can get this ID by calling the <a>DescribeApplication</a> operation. </p>
    #[serde(rename = "inputId")]
    pub input_id: String,
    /// <p>The point at which you want the application to start processing records from the streaming source. </p>
    #[serde(rename = "inputStartingPositionConfiguration")]
    pub input_starting_position_configuration: InputStartingPositionConfiguration,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartApplicationRequest {
    /// <p>The name of the application.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>Identifies the run configuration (start parameters) of a Kinesis Data Analytics application.</p>
    #[serde(rename = "runConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_configuration: Option<RunConfiguration>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartApplicationResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopApplicationRequest {
    /// <p>The name of the running application to stop.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>Set to <code>true</code> to force the application to stop. If you set <code>Force</code> to <code>true</code>, Kinesis Data Analytics stops the application without taking a snapshot. </p> <note> <p>Force-stopping your application may lead to data loss or duplication. To prevent data loss or duplicate processing of data during application restarts, we recommend you to take frequent snapshots of your application.</p> </note> <p>You can only force stop a Flink-based Kinesis Data Analytics application. You can't force stop a SQL-based Kinesis Data Analytics application.</p> <p>The application must be in the <code>STARTING</code>, <code>UPDATING</code>, <code>STOPPING</code>, <code>AUTOSCALING</code>, or <code>RUNNING</code> status. </p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopApplicationResponse {}

/// <p>A key-value pair (the value is optional) that you can define and assign to AWS resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/how-tagging.html">Using Tagging</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The key of the key-value tag.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value of the key-value tag. The value is optional.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of the application to assign the tags.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>The key-value tags to assign to the application.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of the Kinesis Data Analytics application from which to remove the tags.</p>
    #[serde(rename = "resourceARN")]
    pub resource_arn: String,
    /// <p>A list of keys of tags to remove from the specified application.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApplicationMaintenanceConfigurationRequest {
    /// <p>Describes the application maintenance configuration update.</p>
    #[serde(rename = "applicationMaintenanceConfigurationUpdate")]
    pub application_maintenance_configuration_update: ApplicationMaintenanceConfigurationUpdate,
    /// <p>The name of the application for which you want to update the maintenance configuration.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApplicationMaintenanceConfigurationResponse {
    /// <p>The Amazon Resource Name (ARN) of the application.</p>
    #[serde(rename = "applicationARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    /// <p>The application maintenance configuration description after the update.</p>
    #[serde(rename = "applicationMaintenanceConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_maintenance_configuration_description:
        Option<ApplicationMaintenanceConfigurationDescription>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApplicationRequest {
    /// <p>Describes application configuration updates.</p>
    #[serde(rename = "applicationConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_configuration_update: Option<ApplicationConfigurationUpdate>,
    /// <p>The name of the application to update.</p>
    #[serde(rename = "applicationName")]
    pub application_name: String,
    /// <p>Describes application Amazon CloudWatch logging option updates. You can only update existing CloudWatch logging options with this action. To add a new CloudWatch logging option, use <a>AddApplicationCloudWatchLoggingOption</a>.</p>
    #[serde(rename = "cloudWatchLoggingOptionUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_option_updates: Option<Vec<CloudWatchLoggingOptionUpdate>>,
    /// <p>A value you use to implement strong concurrency for application updates. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>. You get the application's current <code>ConditionalToken</code> using <a>DescribeApplication</a>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    #[serde(rename = "conditionalToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_token: Option<String>,
    /// <p>The current application version ID. You must provide the <code>CurrentApplicationVersionId</code> or the <code>ConditionalToken</code>.You can retrieve the application version ID using <a>DescribeApplication</a>. For better concurrency support, use the <code>ConditionalToken</code> parameter instead of <code>CurrentApplicationVersionId</code>.</p>
    #[serde(rename = "currentApplicationVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_application_version_id: Option<i64>,
    /// <p>Describes updates to the application's starting parameters.</p>
    #[serde(rename = "runConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_configuration_update: Option<RunConfigurationUpdate>,
    /// <p>Describes updates to the service execution role.</p>
    #[serde(rename = "serviceExecutionRoleUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_execution_role_update: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApplicationResponse {
    /// <p>Describes application updates.</p>
    #[serde(rename = "applicationDetail")]
    pub application_detail: ApplicationDetail,
}

/// <p>Describes the parameters of a VPC used by the application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VpcConfiguration {
    /// <p>The array of <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_SecurityGroup.html">SecurityGroup</a> IDs used by the VPC configuration.</p>
    #[serde(rename = "securityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>The array of <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_Subnet.html">Subnet</a> IDs used by the VPC configuration.</p>
    #[serde(rename = "subnetIds")]
    pub subnet_ids: Vec<String>,
}

/// <p>Describes the parameters of a VPC used by the application.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VpcConfigurationDescription {
    /// <p>The array of <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_SecurityGroup.html">SecurityGroup</a> IDs used by the VPC configuration.</p>
    #[serde(rename = "securityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>The array of <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_Subnet.html">Subnet</a> IDs used by the VPC configuration.</p>
    #[serde(rename = "subnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>The ID of the VPC configuration.</p>
    #[serde(rename = "vpcConfigurationId")]
    pub vpc_configuration_id: String,
    /// <p>The ID of the associated VPC.</p>
    #[serde(rename = "vpcId")]
    pub vpc_id: String,
}

/// <p>Describes updates to the VPC configuration used by the application.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VpcConfigurationUpdate {
    /// <p>Describes updates to the array of <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_SecurityGroup.html">SecurityGroup</a> IDs used by the VPC configuration.</p>
    #[serde(rename = "securityGroupIdUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id_updates: Option<Vec<String>>,
    /// <p>Describes updates to the array of <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_Subnet.html">Subnet</a> IDs used by the VPC configuration.</p>
    #[serde(rename = "subnetIdUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id_updates: Option<Vec<String>>,
    /// <p>Describes an update to the ID of the VPC configuration.</p>
    #[serde(rename = "vpcConfigurationId")]
    pub vpc_configuration_id: String,
}

/// <p>The configuration of a Kinesis Data Analytics Studio notebook.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ZeppelinApplicationConfiguration {
    /// <p>The AWS Glue Data Catalog that you use in queries in a Kinesis Data Analytics Studio notebook.</p>
    #[serde(rename = "catalogConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_configuration: Option<CatalogConfiguration>,
    /// <p>Custom artifacts are dependency JARs and user-defined functions (UDF).</p>
    #[serde(rename = "customArtifactsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_artifacts_configuration: Option<Vec<CustomArtifactConfiguration>>,
    /// <p>The information required to deploy a Kinesis Data Analytics Studio notebook as an application with durable state..</p>
    #[serde(rename = "deployAsApplicationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy_as_application_configuration: Option<DeployAsApplicationConfiguration>,
    /// <p>The monitoring configuration of a Kinesis Data Analytics Studio notebook.</p>
    #[serde(rename = "monitoringConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<ZeppelinMonitoringConfiguration>,
}

/// <p>The configuration of a Kinesis Data Analytics Studio notebook.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ZeppelinApplicationConfigurationDescription {
    /// <p>The AWS Glue Data Catalog that is associated with the Kinesis Data Analytics Studio notebook.</p>
    #[serde(rename = "catalogConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_configuration_description: Option<CatalogConfigurationDescription>,
    /// <p>Custom artifacts are dependency JARs and user-defined functions (UDF).</p>
    #[serde(rename = "customArtifactsConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_artifacts_configuration_description:
        Option<Vec<CustomArtifactConfigurationDescription>>,
    /// <p>The parameters required to deploy a Kinesis Data Analytics Studio notebook as an application with durable state..</p>
    #[serde(rename = "deployAsApplicationConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy_as_application_configuration_description:
        Option<DeployAsApplicationConfigurationDescription>,
    /// <p>The monitoring configuration of a Kinesis Data Analytics Studio notebook.</p>
    #[serde(rename = "monitoringConfigurationDescription")]
    pub monitoring_configuration_description: ZeppelinMonitoringConfigurationDescription,
}

/// <p>Updates to the configuration of Kinesis Data Analytics Studio notebook.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ZeppelinApplicationConfigurationUpdate {
    /// <p>Updates to the configuration of the AWS Glue Data Catalog that is associated with the Kinesis Data Analytics Studio notebook.</p>
    #[serde(rename = "catalogConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_configuration_update: Option<CatalogConfigurationUpdate>,
    /// <p>Updates to the customer artifacts. Custom artifacts are dependency JAR files and user-defined functions (UDF).</p>
    #[serde(rename = "customArtifactsConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_artifacts_configuration_update: Option<Vec<CustomArtifactConfiguration>>,
    #[serde(rename = "deployAsApplicationConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy_as_application_configuration_update: Option<DeployAsApplicationConfigurationUpdate>,
    /// <p>Updates to the monitoring configuration of a Kinesis Data Analytics Studio notebook.</p>
    #[serde(rename = "monitoringConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration_update: Option<ZeppelinMonitoringConfigurationUpdate>,
}

/// <p>Describes configuration parameters for Amazon CloudWatch logging for a Kinesis Data Analytics Studio notebook. For more information about CloudWatch logging, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/monitoring-overview.html">Monitoring</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ZeppelinMonitoringConfiguration {
    /// <p>The verbosity of the CloudWatch Logs for an application.</p>
    #[serde(rename = "logLevel")]
    pub log_level: String,
}

/// <p>The monitoring configuration for Apache Zeppelin within a Kinesis Data Analytics Studio notebook.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ZeppelinMonitoringConfigurationDescription {
    /// <p>Describes the verbosity of the CloudWatch Logs for an application.</p>
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
}

/// <p>Updates to the monitoring configuration for Apache Zeppelin within a Kinesis Data Analytics Studio notebook.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ZeppelinMonitoringConfigurationUpdate {
    /// <p>Updates to the logging level for Apache Zeppelin within a Kinesis Data Analytics Studio notebook.</p>
    #[serde(rename = "logLevelUpdate")]
    pub log_level_update: String,
}

/// Errors returned by AddApplicationCloudWatchLoggingOption
#[derive(Debug, PartialEq)]
pub enum AddApplicationCloudWatchLoggingOptionError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The user-provided application configuration is not valid.</p>
    InvalidApplicationConfiguration(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl AddApplicationCloudWatchLoggingOptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AddApplicationCloudWatchLoggingOptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        AddApplicationCloudWatchLoggingOptionError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidApplicationConfigurationException" => {
                    return RusotoError::Service(
                        AddApplicationCloudWatchLoggingOptionError::InvalidApplicationConfiguration(
                            err.msg,
                        ),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        AddApplicationCloudWatchLoggingOptionError::InvalidArgument(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        AddApplicationCloudWatchLoggingOptionError::InvalidRequest(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        AddApplicationCloudWatchLoggingOptionError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AddApplicationCloudWatchLoggingOptionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddApplicationCloudWatchLoggingOptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddApplicationCloudWatchLoggingOptionError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationCloudWatchLoggingOptionError::InvalidApplicationConfiguration(
                ref cause,
            ) => write!(f, "{}", cause),
            AddApplicationCloudWatchLoggingOptionError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationCloudWatchLoggingOptionError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationCloudWatchLoggingOptionError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationCloudWatchLoggingOptionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AddApplicationCloudWatchLoggingOptionError {}
/// Errors returned by AddApplicationInput
#[derive(Debug, PartialEq)]
pub enum AddApplicationInputError {
    /// <p>The user-provided application code (query) is not valid. This can be a simple syntax error.</p>
    CodeValidation(String),
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl AddApplicationInputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddApplicationInputError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeValidationException" => {
                    return RusotoError::Service(AddApplicationInputError::CodeValidation(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(AddApplicationInputError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(AddApplicationInputError::InvalidArgument(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AddApplicationInputError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(AddApplicationInputError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddApplicationInputError::ResourceNotFound(
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
impl fmt::Display for AddApplicationInputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddApplicationInputError::CodeValidation(ref cause) => write!(f, "{}", cause),
            AddApplicationInputError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            AddApplicationInputError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            AddApplicationInputError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            AddApplicationInputError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            AddApplicationInputError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddApplicationInputError {}
/// Errors returned by AddApplicationInputProcessingConfiguration
#[derive(Debug, PartialEq)]
pub enum AddApplicationInputProcessingConfigurationError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl AddApplicationInputProcessingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AddApplicationInputProcessingConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        AddApplicationInputProcessingConfigurationError::ConcurrentModification(
                            err.msg,
                        ),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        AddApplicationInputProcessingConfigurationError::InvalidArgument(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        AddApplicationInputProcessingConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        AddApplicationInputProcessingConfigurationError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AddApplicationInputProcessingConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddApplicationInputProcessingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddApplicationInputProcessingConfigurationError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationInputProcessingConfigurationError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationInputProcessingConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationInputProcessingConfigurationError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationInputProcessingConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AddApplicationInputProcessingConfigurationError {}
/// Errors returned by AddApplicationOutput
#[derive(Debug, PartialEq)]
pub enum AddApplicationOutputError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl AddApplicationOutputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddApplicationOutputError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(AddApplicationOutputError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(AddApplicationOutputError::InvalidArgument(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(AddApplicationOutputError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(AddApplicationOutputError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddApplicationOutputError::ResourceNotFound(
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
impl fmt::Display for AddApplicationOutputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddApplicationOutputError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            AddApplicationOutputError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            AddApplicationOutputError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            AddApplicationOutputError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            AddApplicationOutputError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddApplicationOutputError {}
/// Errors returned by AddApplicationReferenceDataSource
#[derive(Debug, PartialEq)]
pub enum AddApplicationReferenceDataSourceError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl AddApplicationReferenceDataSourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AddApplicationReferenceDataSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        AddApplicationReferenceDataSourceError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        AddApplicationReferenceDataSourceError::InvalidArgument(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        AddApplicationReferenceDataSourceError::InvalidRequest(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        AddApplicationReferenceDataSourceError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AddApplicationReferenceDataSourceError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddApplicationReferenceDataSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddApplicationReferenceDataSourceError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationReferenceDataSourceError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationReferenceDataSourceError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationReferenceDataSourceError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationReferenceDataSourceError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AddApplicationReferenceDataSourceError {}
/// Errors returned by AddApplicationVpcConfiguration
#[derive(Debug, PartialEq)]
pub enum AddApplicationVpcConfigurationError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The user-provided application configuration is not valid.</p>
    InvalidApplicationConfiguration(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl AddApplicationVpcConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AddApplicationVpcConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        AddApplicationVpcConfigurationError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidApplicationConfigurationException" => {
                    return RusotoError::Service(
                        AddApplicationVpcConfigurationError::InvalidApplicationConfiguration(
                            err.msg,
                        ),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        AddApplicationVpcConfigurationError::InvalidArgument(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        AddApplicationVpcConfigurationError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AddApplicationVpcConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddApplicationVpcConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddApplicationVpcConfigurationError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationVpcConfigurationError::InvalidApplicationConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationVpcConfigurationError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            AddApplicationVpcConfigurationError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            AddApplicationVpcConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AddApplicationVpcConfigurationError {}
/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    /// <p>The user-provided application code (query) is not valid. This can be a simple syntax error.</p>
    CodeValidation(String),
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The number of allowed resources has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Application created with too many tags, or too many tags added to an application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50.</p>
    TooManyTags(String),
}

impl CreateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeValidationException" => {
                    return RusotoError::Service(CreateApplicationError::CodeValidation(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateApplicationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(CreateApplicationError::InvalidArgument(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateApplicationError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateApplicationError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateApplicationError::ResourceInUse(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreateApplicationError::TooManyTags(err.msg))
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
            CreateApplicationError::CodeValidation(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateApplicationError {}
/// Errors returned by CreateApplicationPresignedUrl
#[derive(Debug, PartialEq)]
pub enum CreateApplicationPresignedUrlError {
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl CreateApplicationPresignedUrlError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateApplicationPresignedUrlError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        CreateApplicationPresignedUrlError::InvalidArgument(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateApplicationPresignedUrlError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        CreateApplicationPresignedUrlError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateApplicationPresignedUrlError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateApplicationPresignedUrlError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateApplicationPresignedUrlError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateApplicationPresignedUrlError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateApplicationPresignedUrlError {}
/// Errors returned by CreateApplicationSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateApplicationSnapshotError {
    /// <p>The user-provided application configuration is not valid.</p>
    InvalidApplicationConfiguration(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The number of allowed resources has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl CreateApplicationSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApplicationSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidApplicationConfigurationException" => {
                    return RusotoError::Service(
                        CreateApplicationSnapshotError::InvalidApplicationConfiguration(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(CreateApplicationSnapshotError::InvalidArgument(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateApplicationSnapshotError::InvalidRequest(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateApplicationSnapshotError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateApplicationSnapshotError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateApplicationSnapshotError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        CreateApplicationSnapshotError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateApplicationSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateApplicationSnapshotError::InvalidApplicationConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateApplicationSnapshotError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateApplicationSnapshotError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            CreateApplicationSnapshotError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateApplicationSnapshotError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            CreateApplicationSnapshotError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateApplicationSnapshotError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateApplicationSnapshotError {}
/// Errors returned by DeleteApplication
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The user-provided application configuration is not valid.</p>
    InvalidApplicationConfiguration(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl DeleteApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteApplicationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidApplicationConfigurationException" => {
                    return RusotoError::Service(
                        DeleteApplicationError::InvalidApplicationConfiguration(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DeleteApplicationError::InvalidArgument(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteApplicationError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteApplicationError::ResourceInUse(err.msg))
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
            DeleteApplicationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::InvalidApplicationConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApplicationError {}
/// Errors returned by DeleteApplicationCloudWatchLoggingOption
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationCloudWatchLoggingOptionError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The user-provided application configuration is not valid.</p>
    InvalidApplicationConfiguration(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl DeleteApplicationCloudWatchLoggingOptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteApplicationCloudWatchLoggingOptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteApplicationCloudWatchLoggingOptionError::ConcurrentModification(
                            err.msg,
                        ),
                    )
                }
                "InvalidApplicationConfigurationException" => return RusotoError::Service(
                    DeleteApplicationCloudWatchLoggingOptionError::InvalidApplicationConfiguration(
                        err.msg,
                    ),
                ),
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        DeleteApplicationCloudWatchLoggingOptionError::InvalidArgument(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DeleteApplicationCloudWatchLoggingOptionError::InvalidRequest(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        DeleteApplicationCloudWatchLoggingOptionError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteApplicationCloudWatchLoggingOptionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApplicationCloudWatchLoggingOptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApplicationCloudWatchLoggingOptionError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationCloudWatchLoggingOptionError::InvalidApplicationConfiguration(
                ref cause,
            ) => write!(f, "{}", cause),
            DeleteApplicationCloudWatchLoggingOptionError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationCloudWatchLoggingOptionError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationCloudWatchLoggingOptionError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationCloudWatchLoggingOptionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteApplicationCloudWatchLoggingOptionError {}
/// Errors returned by DeleteApplicationInputProcessingConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationInputProcessingConfigurationError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl DeleteApplicationInputProcessingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteApplicationInputProcessingConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteApplicationInputProcessingConfigurationError::ConcurrentModification(
                            err.msg,
                        ),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        DeleteApplicationInputProcessingConfigurationError::InvalidArgument(
                            err.msg,
                        ),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DeleteApplicationInputProcessingConfigurationError::InvalidRequest(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        DeleteApplicationInputProcessingConfigurationError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteApplicationInputProcessingConfigurationError::ResourceNotFound(
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
impl fmt::Display for DeleteApplicationInputProcessingConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApplicationInputProcessingConfigurationError::ConcurrentModification(
                ref cause,
            ) => write!(f, "{}", cause),
            DeleteApplicationInputProcessingConfigurationError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationInputProcessingConfigurationError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationInputProcessingConfigurationError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationInputProcessingConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteApplicationInputProcessingConfigurationError {}
/// Errors returned by DeleteApplicationOutput
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationOutputError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl DeleteApplicationOutputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApplicationOutputError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteApplicationOutputError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DeleteApplicationOutputError::InvalidArgument(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteApplicationOutputError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteApplicationOutputError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteApplicationOutputError::ResourceNotFound(
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
impl fmt::Display for DeleteApplicationOutputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApplicationOutputError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationOutputError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DeleteApplicationOutputError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteApplicationOutputError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteApplicationOutputError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApplicationOutputError {}
/// Errors returned by DeleteApplicationReferenceDataSource
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationReferenceDataSourceError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl DeleteApplicationReferenceDataSourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteApplicationReferenceDataSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteApplicationReferenceDataSourceError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        DeleteApplicationReferenceDataSourceError::InvalidArgument(err.msg),
                    )
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(
                        DeleteApplicationReferenceDataSourceError::InvalidRequest(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        DeleteApplicationReferenceDataSourceError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteApplicationReferenceDataSourceError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApplicationReferenceDataSourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApplicationReferenceDataSourceError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationReferenceDataSourceError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationReferenceDataSourceError::InvalidRequest(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationReferenceDataSourceError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationReferenceDataSourceError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteApplicationReferenceDataSourceError {}
/// Errors returned by DeleteApplicationSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationSnapshotError {
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl DeleteApplicationSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApplicationSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(DeleteApplicationSnapshotError::InvalidArgument(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteApplicationSnapshotError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteApplicationSnapshotError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteApplicationSnapshotError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DeleteApplicationSnapshotError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApplicationSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApplicationSnapshotError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DeleteApplicationSnapshotError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DeleteApplicationSnapshotError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteApplicationSnapshotError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteApplicationSnapshotError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteApplicationSnapshotError {}
/// Errors returned by DeleteApplicationVpcConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationVpcConfigurationError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The user-provided application configuration is not valid.</p>
    InvalidApplicationConfiguration(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl DeleteApplicationVpcConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteApplicationVpcConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteApplicationVpcConfigurationError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidApplicationConfigurationException" => {
                    return RusotoError::Service(
                        DeleteApplicationVpcConfigurationError::InvalidApplicationConfiguration(
                            err.msg,
                        ),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        DeleteApplicationVpcConfigurationError::InvalidArgument(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        DeleteApplicationVpcConfigurationError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteApplicationVpcConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteApplicationVpcConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApplicationVpcConfigurationError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationVpcConfigurationError::InvalidApplicationConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationVpcConfigurationError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationVpcConfigurationError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteApplicationVpcConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteApplicationVpcConfigurationError {}
/// Errors returned by DescribeApplication
#[derive(Debug, PartialEq)]
pub enum DescribeApplicationError {
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl DescribeApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeApplicationError::InvalidArgument(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeApplicationError::InvalidRequest(err.msg))
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
            DescribeApplicationError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DescribeApplicationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DescribeApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeApplicationError {}
/// Errors returned by DescribeApplicationSnapshot
#[derive(Debug, PartialEq)]
pub enum DescribeApplicationSnapshotError {
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl DescribeApplicationSnapshotError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeApplicationSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeApplicationSnapshotError::InvalidArgument(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeApplicationSnapshotError::ResourceNotFound(err.msg),
                    )
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DescribeApplicationSnapshotError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeApplicationSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeApplicationSnapshotError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DescribeApplicationSnapshotError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeApplicationSnapshotError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeApplicationSnapshotError {}
/// Errors returned by DescribeApplicationVersion
#[derive(Debug, PartialEq)]
pub enum DescribeApplicationVersionError {
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl DescribeApplicationVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeApplicationVersionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(DescribeApplicationVersionError::InvalidArgument(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeApplicationVersionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DescribeApplicationVersionError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeApplicationVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeApplicationVersionError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DescribeApplicationVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeApplicationVersionError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeApplicationVersionError {}
/// Errors returned by DiscoverInputSchema
#[derive(Debug, PartialEq)]
pub enum DiscoverInputSchemaError {
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>Discovery failed to get a record from the streaming source because of the Kinesis Streams <code>ProvisionedThroughputExceededException</code>. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/APIReference/API_GetRecords.html">GetRecords</a> in the Amazon Kinesis Streams API Reference.</p>
    ResourceProvisionedThroughputExceeded(String),
    /// <p>The service cannot complete the request.</p>
    ServiceUnavailable(String),
    /// <p>The data format is not valid. Kinesis Data Analytics cannot detect the schema for the given streaming source.</p>
    UnableToDetectSchema(String),
}

impl DiscoverInputSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DiscoverInputSchemaError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(DiscoverInputSchemaError::InvalidArgument(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DiscoverInputSchemaError::InvalidRequest(err.msg))
                }
                "ResourceProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DiscoverInputSchemaError::ResourceProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DiscoverInputSchemaError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnableToDetectSchemaException" => {
                    return RusotoError::Service(DiscoverInputSchemaError::UnableToDetectSchema(
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
impl fmt::Display for DiscoverInputSchemaError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DiscoverInputSchemaError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            DiscoverInputSchemaError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            DiscoverInputSchemaError::ResourceProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            DiscoverInputSchemaError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
            DiscoverInputSchemaError::UnableToDetectSchema(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DiscoverInputSchemaError {}
/// Errors returned by ListApplicationSnapshots
#[derive(Debug, PartialEq)]
pub enum ListApplicationSnapshotsError {
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl ListApplicationSnapshotsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApplicationSnapshotsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListApplicationSnapshotsError::InvalidArgument(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        ListApplicationSnapshotsError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListApplicationSnapshotsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListApplicationSnapshotsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListApplicationSnapshotsError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListApplicationSnapshotsError {}
/// Errors returned by ListApplicationVersions
#[derive(Debug, PartialEq)]
pub enum ListApplicationVersionsError {
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl ListApplicationVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApplicationVersionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListApplicationVersionsError::InvalidArgument(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListApplicationVersionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        ListApplicationVersionsError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListApplicationVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListApplicationVersionsError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListApplicationVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListApplicationVersionsError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListApplicationVersionsError {}
/// Errors returned by ListApplications
#[derive(Debug, PartialEq)]
pub enum ListApplicationsError {
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
}

impl ListApplicationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApplicationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidRequestException" => {
                    return RusotoError::Service(ListApplicationsError::InvalidRequest(err.msg))
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
            ListApplicationsError::InvalidRequest(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListApplicationsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(ListTagsForResourceError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidArgument(err.msg))
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
            ListTagsForResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by RollbackApplication
#[derive(Debug, PartialEq)]
pub enum RollbackApplicationError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl RollbackApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RollbackApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(RollbackApplicationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(RollbackApplicationError::InvalidArgument(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(RollbackApplicationError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(RollbackApplicationError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RollbackApplicationError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(RollbackApplicationError::UnsupportedOperation(
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
impl fmt::Display for RollbackApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RollbackApplicationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            RollbackApplicationError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            RollbackApplicationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            RollbackApplicationError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            RollbackApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RollbackApplicationError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RollbackApplicationError {}
/// Errors returned by StartApplication
#[derive(Debug, PartialEq)]
pub enum StartApplicationError {
    /// <p>The user-provided application configuration is not valid.</p>
    InvalidApplicationConfiguration(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl StartApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidApplicationConfigurationException" => {
                    return RusotoError::Service(
                        StartApplicationError::InvalidApplicationConfiguration(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(StartApplicationError::InvalidArgument(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartApplicationError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StartApplicationError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartApplicationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartApplicationError::InvalidApplicationConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            StartApplicationError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            StartApplicationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StartApplicationError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StartApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartApplicationError {}
/// Errors returned by StopApplication
#[derive(Debug, PartialEq)]
pub enum StopApplicationError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The user-provided application configuration is not valid.</p>
    InvalidApplicationConfiguration(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl StopApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(StopApplicationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidApplicationConfigurationException" => {
                    return RusotoError::Service(
                        StopApplicationError::InvalidApplicationConfiguration(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(StopApplicationError::InvalidArgument(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StopApplicationError::InvalidRequest(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StopApplicationError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopApplicationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopApplicationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            StopApplicationError::InvalidApplicationConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            StopApplicationError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            StopApplicationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            StopApplicationError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StopApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopApplicationError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
    /// <p>Application created with too many tags, or too many tags added to an application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(TagResourceError::ConcurrentModification(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(TagResourceError::InvalidArgument(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(TagResourceError::ResourceInUse(err.msg))
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
            TagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            TagResourceError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
    /// <p>Application created with too many tags, or too many tags added to an application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50.</p>
    TooManyTags(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UntagResourceError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UntagResourceError::InvalidArgument(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UntagResourceError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(UntagResourceError::TooManyTags(err.msg))
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
            UntagResourceError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TooManyTags(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateApplication
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationError {
    /// <p>The user-provided application code (query) is not valid. This can be a simple syntax error.</p>
    CodeValidation(String),
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The user-provided application configuration is not valid.</p>
    InvalidApplicationConfiguration(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The request JSON is not valid for the operation.</p>
    InvalidRequest(String),
    /// <p>The number of allowed resources has been exceeded.</p>
    LimitExceeded(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl UpdateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeValidationException" => {
                    return RusotoError::Service(UpdateApplicationError::CodeValidation(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateApplicationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidApplicationConfigurationException" => {
                    return RusotoError::Service(
                        UpdateApplicationError::InvalidApplicationConfiguration(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UpdateApplicationError::InvalidArgument(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateApplicationError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateApplicationError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateApplicationError::ResourceInUse(err.msg))
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
            UpdateApplicationError::CodeValidation(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::InvalidApplicationConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateApplicationError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::InvalidRequest(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApplicationError {}
/// Errors returned by UpdateApplicationMaintenanceConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationMaintenanceConfigurationError {
    /// <p>Exception thrown as a result of concurrent modifications to an application. This error can be the result of attempting to modify an application without using the current application ID.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter value is not valid.</p>
    InvalidArgument(String),
    /// <p>The application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl UpdateApplicationMaintenanceConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateApplicationMaintenanceConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        UpdateApplicationMaintenanceConfigurationError::ConcurrentModification(
                            err.msg,
                        ),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        UpdateApplicationMaintenanceConfigurationError::InvalidArgument(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        UpdateApplicationMaintenanceConfigurationError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateApplicationMaintenanceConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        UpdateApplicationMaintenanceConfigurationError::UnsupportedOperation(
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
impl fmt::Display for UpdateApplicationMaintenanceConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApplicationMaintenanceConfigurationError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateApplicationMaintenanceConfigurationError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateApplicationMaintenanceConfigurationError::ResourceInUse(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateApplicationMaintenanceConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateApplicationMaintenanceConfigurationError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateApplicationMaintenanceConfigurationError {}
/// Trait representing the capabilities of the Kinesis Analytics V2 API. Kinesis Analytics V2 clients implement this trait.
#[async_trait]
pub trait KinesisAnalyticsV2 {
    /// <p>Adds an Amazon CloudWatch log stream to monitor application configuration errors.</p>
    async fn add_application_cloud_watch_logging_option(
        &self,
        input: AddApplicationCloudWatchLoggingOptionRequest,
    ) -> Result<
        AddApplicationCloudWatchLoggingOptionResponse,
        RusotoError<AddApplicationCloudWatchLoggingOptionError>,
    >;

    /// <p> Adds a streaming source to your SQL-based Kinesis Data Analytics application. </p> <p>You can add a streaming source when you create an application, or you can use this operation to add a streaming source after you create an application. For more information, see <a>CreateApplication</a>.</p> <p>Any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a>DescribeApplication</a> operation to find the current application version. </p>
    async fn add_application_input(
        &self,
        input: AddApplicationInputRequest,
    ) -> Result<AddApplicationInputResponse, RusotoError<AddApplicationInputError>>;

    /// <p>Adds an <a>InputProcessingConfiguration</a> to a SQL-based Kinesis Data Analytics application. An input processor pre-processes records on the input stream before the application's SQL code executes. Currently, the only input processor available is <a href="https://docs.aws.amazon.com/lambda/">AWS Lambda</a>.</p>
    async fn add_application_input_processing_configuration(
        &self,
        input: AddApplicationInputProcessingConfigurationRequest,
    ) -> Result<
        AddApplicationInputProcessingConfigurationResponse,
        RusotoError<AddApplicationInputProcessingConfigurationError>,
    >;

    /// <p>Adds an external destination to your SQL-based Kinesis Data Analytics application.</p> <p>If you want Kinesis Data Analytics to deliver data from an in-application stream within your application to an external destination (such as an Kinesis data stream, a Kinesis Data Firehose delivery stream, or an AWS Lambda function), you add the relevant configuration to your application using this operation. You can configure one or more outputs for your application. Each output configuration maps an in-application stream and an external destination.</p> <p> You can use one of the output configurations to deliver data from your in-application error stream to an external destination so that you can analyze the errors. </p> <p> Any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a>DescribeApplication</a> operation to find the current application version.</p>
    async fn add_application_output(
        &self,
        input: AddApplicationOutputRequest,
    ) -> Result<AddApplicationOutputResponse, RusotoError<AddApplicationOutputError>>;

    /// <p>Adds a reference data source to an existing SQL-based Kinesis Data Analytics application.</p> <p>Kinesis Data Analytics reads reference data (that is, an Amazon S3 object) and creates an in-application table within your application. In the request, you provide the source (S3 bucket name and object key name), name of the in-application table to create, and the necessary mapping information that describes how data in an Amazon S3 object maps to columns in the resulting in-application table.</p>
    async fn add_application_reference_data_source(
        &self,
        input: AddApplicationReferenceDataSourceRequest,
    ) -> Result<
        AddApplicationReferenceDataSourceResponse,
        RusotoError<AddApplicationReferenceDataSourceError>,
    >;

    /// <p><p>Adds a Virtual Private Cloud (VPC) configuration to the application. Applications can use VPCs to store and access resources securely.</p> <p>Note the following about VPC configurations for Kinesis Data Analytics applications:</p> <ul> <li> <p>VPC configurations are not supported for SQL applications.</p> </li> <li> <p>When a VPC is added to a Kinesis Data Analytics application, the application can no longer be accessed from the Internet directly. To enable Internet access to the application, add an Internet gateway to your VPC.</p> </li> </ul></p>
    async fn add_application_vpc_configuration(
        &self,
        input: AddApplicationVpcConfigurationRequest,
    ) -> Result<
        AddApplicationVpcConfigurationResponse,
        RusotoError<AddApplicationVpcConfigurationError>,
    >;

    /// <p>Creates a Kinesis Data Analytics application. For information about creating a Kinesis Data Analytics application, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/getting-started.html">Creating an Application</a>.</p>
    async fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> Result<CreateApplicationResponse, RusotoError<CreateApplicationError>>;

    /// <p><p>Creates and returns a URL that you can use to connect to an application&#39;s extension. Currently, the only available extension is the Apache Flink dashboard.</p> <p>The IAM role or user used to call this API defines the permissions to access the extension. After the presigned URL is created, no additional permission is required to access this URL. IAM authorization policies for this API are also enforced for every HTTP request that attempts to connect to the extension. </p> <p>You control the amount of time that the URL will be valid using the <code>SessionExpirationDurationInSeconds</code> parameter. If you do not provide this parameter, the returned URL is valid for twelve hours.</p> <note> <p>The URL that you get from a call to CreateApplicationPresignedUrl must be used within 3 minutes to be valid. If you first try to use the URL after the 3-minute limit expires, the service returns an HTTP 403 Forbidden error.</p> </note></p>
    async fn create_application_presigned_url(
        &self,
        input: CreateApplicationPresignedUrlRequest,
    ) -> Result<
        CreateApplicationPresignedUrlResponse,
        RusotoError<CreateApplicationPresignedUrlError>,
    >;

    /// <p>Creates a snapshot of the application's state data.</p>
    async fn create_application_snapshot(
        &self,
        input: CreateApplicationSnapshotRequest,
    ) -> Result<CreateApplicationSnapshotResponse, RusotoError<CreateApplicationSnapshotError>>;

    /// <p>Deletes the specified application. Kinesis Data Analytics halts application execution and deletes the application.</p>
    async fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> Result<DeleteApplicationResponse, RusotoError<DeleteApplicationError>>;

    /// <p>Deletes an Amazon CloudWatch log stream from an Kinesis Data Analytics application. </p>
    async fn delete_application_cloud_watch_logging_option(
        &self,
        input: DeleteApplicationCloudWatchLoggingOptionRequest,
    ) -> Result<
        DeleteApplicationCloudWatchLoggingOptionResponse,
        RusotoError<DeleteApplicationCloudWatchLoggingOptionError>,
    >;

    /// <p>Deletes an <a>InputProcessingConfiguration</a> from an input.</p>
    async fn delete_application_input_processing_configuration(
        &self,
        input: DeleteApplicationInputProcessingConfigurationRequest,
    ) -> Result<
        DeleteApplicationInputProcessingConfigurationResponse,
        RusotoError<DeleteApplicationInputProcessingConfigurationError>,
    >;

    /// <p>Deletes the output destination configuration from your SQL-based Kinesis Data Analytics application's configuration. Kinesis Data Analytics will no longer write data from the corresponding in-application stream to the external output destination.</p>
    async fn delete_application_output(
        &self,
        input: DeleteApplicationOutputRequest,
    ) -> Result<DeleteApplicationOutputResponse, RusotoError<DeleteApplicationOutputError>>;

    /// <p>Deletes a reference data source configuration from the specified SQL-based Kinesis Data Analytics application's configuration.</p> <p>If the application is running, Kinesis Data Analytics immediately removes the in-application table that you created using the <a>AddApplicationReferenceDataSource</a> operation. </p>
    async fn delete_application_reference_data_source(
        &self,
        input: DeleteApplicationReferenceDataSourceRequest,
    ) -> Result<
        DeleteApplicationReferenceDataSourceResponse,
        RusotoError<DeleteApplicationReferenceDataSourceError>,
    >;

    /// <p>Deletes a snapshot of application state.</p>
    async fn delete_application_snapshot(
        &self,
        input: DeleteApplicationSnapshotRequest,
    ) -> Result<DeleteApplicationSnapshotResponse, RusotoError<DeleteApplicationSnapshotError>>;

    /// <p>Removes a VPC configuration from a Kinesis Data Analytics application.</p>
    async fn delete_application_vpc_configuration(
        &self,
        input: DeleteApplicationVpcConfigurationRequest,
    ) -> Result<
        DeleteApplicationVpcConfigurationResponse,
        RusotoError<DeleteApplicationVpcConfigurationError>,
    >;

    /// <p>Returns information about a specific Kinesis Data Analytics application.</p> <p>If you want to retrieve a list of all applications in your account, use the <a>ListApplications</a> operation.</p>
    async fn describe_application(
        &self,
        input: DescribeApplicationRequest,
    ) -> Result<DescribeApplicationResponse, RusotoError<DescribeApplicationError>>;

    /// <p>Returns information about a snapshot of application state data.</p>
    async fn describe_application_snapshot(
        &self,
        input: DescribeApplicationSnapshotRequest,
    ) -> Result<DescribeApplicationSnapshotResponse, RusotoError<DescribeApplicationSnapshotError>>;

    /// <p><p>Provides a detailed description of a specified version of the application. To see a list of all the versions of an application, invoke the <a>ListApplicationVersions</a> operation.</p> <note> <p>This operation is supported only for Amazon Kinesis Data Analytics for Apache Flink.</p> </note></p>
    async fn describe_application_version(
        &self,
        input: DescribeApplicationVersionRequest,
    ) -> Result<DescribeApplicationVersionResponse, RusotoError<DescribeApplicationVersionError>>;

    /// <p>Infers a schema for a SQL-based Kinesis Data Analytics application by evaluating sample records on the specified streaming source (Kinesis data stream or Kinesis Data Firehose delivery stream) or Amazon S3 object. In the response, the operation returns the inferred schema and also the sample records that the operation used to infer the schema.</p> <p> You can use the inferred schema when configuring a streaming source for your application. When you create an application using the Kinesis Data Analytics console, the console uses this operation to infer a schema and show it in the console user interface. </p>
    async fn discover_input_schema(
        &self,
        input: DiscoverInputSchemaRequest,
    ) -> Result<DiscoverInputSchemaResponse, RusotoError<DiscoverInputSchemaError>>;

    /// <p>Lists information about the current application snapshots.</p>
    async fn list_application_snapshots(
        &self,
        input: ListApplicationSnapshotsRequest,
    ) -> Result<ListApplicationSnapshotsResponse, RusotoError<ListApplicationSnapshotsError>>;

    /// <p><p>Lists all the versions for the specified application, including versions that were rolled back. The response also includes a summary of the configuration associated with each version.</p> <p>To get the complete description of a specific application version, invoke the <a>DescribeApplicationVersion</a> operation.</p> <note> <p>This operation is supported only for Amazon Kinesis Data Analytics for Apache Flink.</p> </note></p>
    async fn list_application_versions(
        &self,
        input: ListApplicationVersionsRequest,
    ) -> Result<ListApplicationVersionsResponse, RusotoError<ListApplicationVersionsError>>;

    /// <p>Returns a list of Kinesis Data Analytics applications in your account. For each application, the response includes the application name, Amazon Resource Name (ARN), and status. </p> <p>If you want detailed information about a specific application, use <a>DescribeApplication</a>.</p>
    async fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> Result<ListApplicationsResponse, RusotoError<ListApplicationsError>>;

    /// <p>Retrieves the list of key-value tags assigned to the application. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/how-tagging.html">Using Tagging</a>.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Reverts the application to the previous running version. You can roll back an application if you suspect it is stuck in a transient status. </p> <p>You can roll back an application only if it is in the <code>UPDATING</code> or <code>AUTOSCALING</code> status.</p> <p>When you rollback an application, it loads state data from the last successful snapshot. If the application has no snapshots, Kinesis Data Analytics rejects the rollback request.</p> <p>This action is not supported for Kinesis Data Analytics for SQL applications.</p>
    async fn rollback_application(
        &self,
        input: RollbackApplicationRequest,
    ) -> Result<RollbackApplicationResponse, RusotoError<RollbackApplicationError>>;

    /// <p>Starts the specified Kinesis Data Analytics application. After creating an application, you must exclusively call this operation to start your application.</p>
    async fn start_application(
        &self,
        input: StartApplicationRequest,
    ) -> Result<StartApplicationResponse, RusotoError<StartApplicationError>>;

    /// <p>Stops the application from processing data. You can stop an application only if it is in the running status, unless you set the <code>Force</code> parameter to <code>true</code>.</p> <p>You can use the <a>DescribeApplication</a> operation to find the application status. </p> <p>Kinesis Data Analytics takes a snapshot when the application is stopped, unless <code>Force</code> is set to <code>true</code>.</p>
    async fn stop_application(
        &self,
        input: StopApplicationRequest,
    ) -> Result<StopApplicationResponse, RusotoError<StopApplicationError>>;

    /// <p>Adds one or more key-value tags to a Kinesis Data Analytics application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/how-tagging.html">Using Tagging</a>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes one or more tags from a Kinesis Data Analytics application. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/how-tagging.html">Using Tagging</a>.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p><p>Updates an existing Kinesis Data Analytics application. Using this operation, you can update application code, input configuration, and output configuration. </p> <p>Kinesis Data Analytics updates the <code>ApplicationVersionId</code> each time you update your application. </p> <note> <p>You cannot update the <code>RuntimeEnvironment</code> of an existing application. If you need to update an application&#39;s <code>RuntimeEnvironment</code>, you must delete the application and create it again.</p> </note></p>
    async fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> Result<UpdateApplicationResponse, RusotoError<UpdateApplicationError>>;

    /// <p><p>Updates the maintenance configuration of the Kinesis Data Analytics application. </p> <p>You can invoke this operation on an application that is in one of the two following states: <code>READY</code> or <code>RUNNING</code>. If you invoke it when the application is in a state other than these two states, it throws a <code>ResourceInUseException</code>. The service makes use of the updated configuration the next time it schedules maintenance for the application. If you invoke this operation after the service schedules maintenance, the service will apply the configuration update the next time it schedules maintenance for the application. This means that you might not see the maintenance configuration update applied to the maintenance process that follows a successful invocation of this operation, but to the following maintenance process instead.</p> <p>To see the current maintenance configuration of your application, invoke the <a>DescribeApplication</a> operation.</p> <p>For information about application maintenance, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/maintenance.html">Kinesis Data Analytics for Apache Flink Maintenance</a>.</p> <note> <p>This operation is supported only for Amazon Kinesis Data Analytics for Apache Flink.</p> </note></p>
    async fn update_application_maintenance_configuration(
        &self,
        input: UpdateApplicationMaintenanceConfigurationRequest,
    ) -> Result<
        UpdateApplicationMaintenanceConfigurationResponse,
        RusotoError<UpdateApplicationMaintenanceConfigurationError>,
    >;
}
/// A client for the Kinesis Analytics V2 API.
#[derive(Clone)]
pub struct KinesisAnalyticsV2Client {
    client: Client,
    region: region::Region,
}

impl KinesisAnalyticsV2Client {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> KinesisAnalyticsV2Client {
        KinesisAnalyticsV2Client {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KinesisAnalyticsV2Client
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        KinesisAnalyticsV2Client {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> KinesisAnalyticsV2Client {
        KinesisAnalyticsV2Client { client, region }
    }
}

#[async_trait]
impl KinesisAnalyticsV2 for KinesisAnalyticsV2Client {
    /// <p>Adds an Amazon CloudWatch log stream to monitor application configuration errors.</p>
    async fn add_application_cloud_watch_logging_option(
        &self,
        input: AddApplicationCloudWatchLoggingOptionRequest,
    ) -> Result<
        AddApplicationCloudWatchLoggingOptionResponse,
        RusotoError<AddApplicationCloudWatchLoggingOptionError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.AddApplicationCloudWatchLoggingOption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                AddApplicationCloudWatchLoggingOptionError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AddApplicationCloudWatchLoggingOptionResponse, _>()
    }

    /// <p> Adds a streaming source to your SQL-based Kinesis Data Analytics application. </p> <p>You can add a streaming source when you create an application, or you can use this operation to add a streaming source after you create an application. For more information, see <a>CreateApplication</a>.</p> <p>Any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a>DescribeApplication</a> operation to find the current application version. </p>
    async fn add_application_input(
        &self,
        input: AddApplicationInputRequest,
    ) -> Result<AddApplicationInputResponse, RusotoError<AddApplicationInputError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.AddApplicationInput",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AddApplicationInputError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AddApplicationInputResponse, _>()
    }

    /// <p>Adds an <a>InputProcessingConfiguration</a> to a SQL-based Kinesis Data Analytics application. An input processor pre-processes records on the input stream before the application's SQL code executes. Currently, the only input processor available is <a href="https://docs.aws.amazon.com/lambda/">AWS Lambda</a>.</p>
    async fn add_application_input_processing_configuration(
        &self,
        input: AddApplicationInputProcessingConfigurationRequest,
    ) -> Result<
        AddApplicationInputProcessingConfigurationResponse,
        RusotoError<AddApplicationInputProcessingConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.AddApplicationInputProcessingConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                AddApplicationInputProcessingConfigurationError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AddApplicationInputProcessingConfigurationResponse, _>()
    }

    /// <p>Adds an external destination to your SQL-based Kinesis Data Analytics application.</p> <p>If you want Kinesis Data Analytics to deliver data from an in-application stream within your application to an external destination (such as an Kinesis data stream, a Kinesis Data Firehose delivery stream, or an AWS Lambda function), you add the relevant configuration to your application using this operation. You can configure one or more outputs for your application. Each output configuration maps an in-application stream and an external destination.</p> <p> You can use one of the output configurations to deliver data from your in-application error stream to an external destination so that you can analyze the errors. </p> <p> Any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a>DescribeApplication</a> operation to find the current application version.</p>
    async fn add_application_output(
        &self,
        input: AddApplicationOutputRequest,
    ) -> Result<AddApplicationOutputResponse, RusotoError<AddApplicationOutputError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.AddApplicationOutput",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AddApplicationOutputError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AddApplicationOutputResponse, _>()
    }

    /// <p>Adds a reference data source to an existing SQL-based Kinesis Data Analytics application.</p> <p>Kinesis Data Analytics reads reference data (that is, an Amazon S3 object) and creates an in-application table within your application. In the request, you provide the source (S3 bucket name and object key name), name of the in-application table to create, and the necessary mapping information that describes how data in an Amazon S3 object maps to columns in the resulting in-application table.</p>
    async fn add_application_reference_data_source(
        &self,
        input: AddApplicationReferenceDataSourceRequest,
    ) -> Result<
        AddApplicationReferenceDataSourceResponse,
        RusotoError<AddApplicationReferenceDataSourceError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.AddApplicationReferenceDataSource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                AddApplicationReferenceDataSourceError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AddApplicationReferenceDataSourceResponse, _>()
    }

    /// <p><p>Adds a Virtual Private Cloud (VPC) configuration to the application. Applications can use VPCs to store and access resources securely.</p> <p>Note the following about VPC configurations for Kinesis Data Analytics applications:</p> <ul> <li> <p>VPC configurations are not supported for SQL applications.</p> </li> <li> <p>When a VPC is added to a Kinesis Data Analytics application, the application can no longer be accessed from the Internet directly. To enable Internet access to the application, add an Internet gateway to your VPC.</p> </li> </ul></p>
    async fn add_application_vpc_configuration(
        &self,
        input: AddApplicationVpcConfigurationRequest,
    ) -> Result<
        AddApplicationVpcConfigurationResponse,
        RusotoError<AddApplicationVpcConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.AddApplicationVpcConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AddApplicationVpcConfigurationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<AddApplicationVpcConfigurationResponse, _>()
    }

    /// <p>Creates a Kinesis Data Analytics application. For information about creating a Kinesis Data Analytics application, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/getting-started.html">Creating an Application</a>.</p>
    async fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> Result<CreateApplicationResponse, RusotoError<CreateApplicationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.CreateApplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateApplicationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateApplicationResponse, _>()
    }

    /// <p><p>Creates and returns a URL that you can use to connect to an application&#39;s extension. Currently, the only available extension is the Apache Flink dashboard.</p> <p>The IAM role or user used to call this API defines the permissions to access the extension. After the presigned URL is created, no additional permission is required to access this URL. IAM authorization policies for this API are also enforced for every HTTP request that attempts to connect to the extension. </p> <p>You control the amount of time that the URL will be valid using the <code>SessionExpirationDurationInSeconds</code> parameter. If you do not provide this parameter, the returned URL is valid for twelve hours.</p> <note> <p>The URL that you get from a call to CreateApplicationPresignedUrl must be used within 3 minutes to be valid. If you first try to use the URL after the 3-minute limit expires, the service returns an HTTP 403 Forbidden error.</p> </note></p>
    async fn create_application_presigned_url(
        &self,
        input: CreateApplicationPresignedUrlRequest,
    ) -> Result<
        CreateApplicationPresignedUrlResponse,
        RusotoError<CreateApplicationPresignedUrlError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.CreateApplicationPresignedUrl",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateApplicationPresignedUrlError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateApplicationPresignedUrlResponse, _>()
    }

    /// <p>Creates a snapshot of the application's state data.</p>
    async fn create_application_snapshot(
        &self,
        input: CreateApplicationSnapshotRequest,
    ) -> Result<CreateApplicationSnapshotResponse, RusotoError<CreateApplicationSnapshotError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.CreateApplicationSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateApplicationSnapshotError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateApplicationSnapshotResponse, _>()
    }

    /// <p>Deletes the specified application. Kinesis Data Analytics halts application execution and deletes the application.</p>
    async fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> Result<DeleteApplicationResponse, RusotoError<DeleteApplicationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.DeleteApplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteApplicationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteApplicationResponse, _>()
    }

    /// <p>Deletes an Amazon CloudWatch log stream from an Kinesis Data Analytics application. </p>
    async fn delete_application_cloud_watch_logging_option(
        &self,
        input: DeleteApplicationCloudWatchLoggingOptionRequest,
    ) -> Result<
        DeleteApplicationCloudWatchLoggingOptionResponse,
        RusotoError<DeleteApplicationCloudWatchLoggingOptionError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.DeleteApplicationCloudWatchLoggingOption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DeleteApplicationCloudWatchLoggingOptionError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteApplicationCloudWatchLoggingOptionResponse, _>()
    }

    /// <p>Deletes an <a>InputProcessingConfiguration</a> from an input.</p>
    async fn delete_application_input_processing_configuration(
        &self,
        input: DeleteApplicationInputProcessingConfigurationRequest,
    ) -> Result<
        DeleteApplicationInputProcessingConfigurationResponse,
        RusotoError<DeleteApplicationInputProcessingConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.DeleteApplicationInputProcessingConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DeleteApplicationInputProcessingConfigurationError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteApplicationInputProcessingConfigurationResponse, _>()
    }

    /// <p>Deletes the output destination configuration from your SQL-based Kinesis Data Analytics application's configuration. Kinesis Data Analytics will no longer write data from the corresponding in-application stream to the external output destination.</p>
    async fn delete_application_output(
        &self,
        input: DeleteApplicationOutputRequest,
    ) -> Result<DeleteApplicationOutputResponse, RusotoError<DeleteApplicationOutputError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.DeleteApplicationOutput",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteApplicationOutputError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteApplicationOutputResponse, _>()
    }

    /// <p>Deletes a reference data source configuration from the specified SQL-based Kinesis Data Analytics application's configuration.</p> <p>If the application is running, Kinesis Data Analytics immediately removes the in-application table that you created using the <a>AddApplicationReferenceDataSource</a> operation. </p>
    async fn delete_application_reference_data_source(
        &self,
        input: DeleteApplicationReferenceDataSourceRequest,
    ) -> Result<
        DeleteApplicationReferenceDataSourceResponse,
        RusotoError<DeleteApplicationReferenceDataSourceError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.DeleteApplicationReferenceDataSource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DeleteApplicationReferenceDataSourceError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteApplicationReferenceDataSourceResponse, _>()
    }

    /// <p>Deletes a snapshot of application state.</p>
    async fn delete_application_snapshot(
        &self,
        input: DeleteApplicationSnapshotRequest,
    ) -> Result<DeleteApplicationSnapshotResponse, RusotoError<DeleteApplicationSnapshotError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.DeleteApplicationSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteApplicationSnapshotError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteApplicationSnapshotResponse, _>()
    }

    /// <p>Removes a VPC configuration from a Kinesis Data Analytics application.</p>
    async fn delete_application_vpc_configuration(
        &self,
        input: DeleteApplicationVpcConfigurationRequest,
    ) -> Result<
        DeleteApplicationVpcConfigurationResponse,
        RusotoError<DeleteApplicationVpcConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.DeleteApplicationVpcConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DeleteApplicationVpcConfigurationError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteApplicationVpcConfigurationResponse, _>()
    }

    /// <p>Returns information about a specific Kinesis Data Analytics application.</p> <p>If you want to retrieve a list of all applications in your account, use the <a>ListApplications</a> operation.</p>
    async fn describe_application(
        &self,
        input: DescribeApplicationRequest,
    ) -> Result<DescribeApplicationResponse, RusotoError<DescribeApplicationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.DescribeApplication",
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

    /// <p>Returns information about a snapshot of application state data.</p>
    async fn describe_application_snapshot(
        &self,
        input: DescribeApplicationSnapshotRequest,
    ) -> Result<DescribeApplicationSnapshotResponse, RusotoError<DescribeApplicationSnapshotError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.DescribeApplicationSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeApplicationSnapshotError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeApplicationSnapshotResponse, _>()
    }

    /// <p><p>Provides a detailed description of a specified version of the application. To see a list of all the versions of an application, invoke the <a>ListApplicationVersions</a> operation.</p> <note> <p>This operation is supported only for Amazon Kinesis Data Analytics for Apache Flink.</p> </note></p>
    async fn describe_application_version(
        &self,
        input: DescribeApplicationVersionRequest,
    ) -> Result<DescribeApplicationVersionResponse, RusotoError<DescribeApplicationVersionError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.DescribeApplicationVersion",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeApplicationVersionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeApplicationVersionResponse, _>()
    }

    /// <p>Infers a schema for a SQL-based Kinesis Data Analytics application by evaluating sample records on the specified streaming source (Kinesis data stream or Kinesis Data Firehose delivery stream) or Amazon S3 object. In the response, the operation returns the inferred schema and also the sample records that the operation used to infer the schema.</p> <p> You can use the inferred schema when configuring a streaming source for your application. When you create an application using the Kinesis Data Analytics console, the console uses this operation to infer a schema and show it in the console user interface. </p>
    async fn discover_input_schema(
        &self,
        input: DiscoverInputSchemaRequest,
    ) -> Result<DiscoverInputSchemaResponse, RusotoError<DiscoverInputSchemaError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.DiscoverInputSchema",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DiscoverInputSchemaError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DiscoverInputSchemaResponse, _>()
    }

    /// <p>Lists information about the current application snapshots.</p>
    async fn list_application_snapshots(
        &self,
        input: ListApplicationSnapshotsRequest,
    ) -> Result<ListApplicationSnapshotsResponse, RusotoError<ListApplicationSnapshotsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.ListApplicationSnapshots",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListApplicationSnapshotsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListApplicationSnapshotsResponse, _>()
    }

    /// <p><p>Lists all the versions for the specified application, including versions that were rolled back. The response also includes a summary of the configuration associated with each version.</p> <p>To get the complete description of a specific application version, invoke the <a>DescribeApplicationVersion</a> operation.</p> <note> <p>This operation is supported only for Amazon Kinesis Data Analytics for Apache Flink.</p> </note></p>
    async fn list_application_versions(
        &self,
        input: ListApplicationVersionsRequest,
    ) -> Result<ListApplicationVersionsResponse, RusotoError<ListApplicationVersionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.ListApplicationVersions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListApplicationVersionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListApplicationVersionsResponse, _>()
    }

    /// <p>Returns a list of Kinesis Data Analytics applications in your account. For each application, the response includes the application name, Amazon Resource Name (ARN), and status. </p> <p>If you want detailed information about a specific application, use <a>DescribeApplication</a>.</p>
    async fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> Result<ListApplicationsResponse, RusotoError<ListApplicationsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "KinesisAnalytics_20180523.ListApplications");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListApplicationsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListApplicationsResponse, _>()
    }

    /// <p>Retrieves the list of key-value tags assigned to the application. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/how-tagging.html">Using Tagging</a>.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.ListTagsForResource",
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

    /// <p>Reverts the application to the previous running version. You can roll back an application if you suspect it is stuck in a transient status. </p> <p>You can roll back an application only if it is in the <code>UPDATING</code> or <code>AUTOSCALING</code> status.</p> <p>When you rollback an application, it loads state data from the last successful snapshot. If the application has no snapshots, Kinesis Data Analytics rejects the rollback request.</p> <p>This action is not supported for Kinesis Data Analytics for SQL applications.</p>
    async fn rollback_application(
        &self,
        input: RollbackApplicationRequest,
    ) -> Result<RollbackApplicationResponse, RusotoError<RollbackApplicationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.RollbackApplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RollbackApplicationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RollbackApplicationResponse, _>()
    }

    /// <p>Starts the specified Kinesis Data Analytics application. After creating an application, you must exclusively call this operation to start your application.</p>
    async fn start_application(
        &self,
        input: StartApplicationRequest,
    ) -> Result<StartApplicationResponse, RusotoError<StartApplicationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "KinesisAnalytics_20180523.StartApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartApplicationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StartApplicationResponse, _>()
    }

    /// <p>Stops the application from processing data. You can stop an application only if it is in the running status, unless you set the <code>Force</code> parameter to <code>true</code>.</p> <p>You can use the <a>DescribeApplication</a> operation to find the application status. </p> <p>Kinesis Data Analytics takes a snapshot when the application is stopped, unless <code>Force</code> is set to <code>true</code>.</p>
    async fn stop_application(
        &self,
        input: StopApplicationRequest,
    ) -> Result<StopApplicationResponse, RusotoError<StopApplicationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "KinesisAnalytics_20180523.StopApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopApplicationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StopApplicationResponse, _>()
    }

    /// <p>Adds one or more key-value tags to a Kinesis Data Analytics application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/how-tagging.html">Using Tagging</a>.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "KinesisAnalytics_20180523.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagResourceResponse, _>()
    }

    /// <p>Removes one or more tags from a Kinesis Data Analytics application. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/how-tagging.html">Using Tagging</a>.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "KinesisAnalytics_20180523.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagResourceResponse, _>()
    }

    /// <p><p>Updates an existing Kinesis Data Analytics application. Using this operation, you can update application code, input configuration, and output configuration. </p> <p>Kinesis Data Analytics updates the <code>ApplicationVersionId</code> each time you update your application. </p> <note> <p>You cannot update the <code>RuntimeEnvironment</code> of an existing application. If you need to update an application&#39;s <code>RuntimeEnvironment</code>, you must delete the application and create it again.</p> </note></p>
    async fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> Result<UpdateApplicationResponse, RusotoError<UpdateApplicationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.UpdateApplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateApplicationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateApplicationResponse, _>()
    }

    /// <p><p>Updates the maintenance configuration of the Kinesis Data Analytics application. </p> <p>You can invoke this operation on an application that is in one of the two following states: <code>READY</code> or <code>RUNNING</code>. If you invoke it when the application is in a state other than these two states, it throws a <code>ResourceInUseException</code>. The service makes use of the updated configuration the next time it schedules maintenance for the application. If you invoke this operation after the service schedules maintenance, the service will apply the configuration update the next time it schedules maintenance for the application. This means that you might not see the maintenance configuration update applied to the maintenance process that follows a successful invocation of this operation, but to the following maintenance process instead.</p> <p>To see the current maintenance configuration of your application, invoke the <a>DescribeApplication</a> operation.</p> <p>For information about application maintenance, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/java/maintenance.html">Kinesis Data Analytics for Apache Flink Maintenance</a>.</p> <note> <p>This operation is supported only for Amazon Kinesis Data Analytics for Apache Flink.</p> </note></p>
    async fn update_application_maintenance_configuration(
        &self,
        input: UpdateApplicationMaintenanceConfigurationRequest,
    ) -> Result<
        UpdateApplicationMaintenanceConfigurationResponse,
        RusotoError<UpdateApplicationMaintenanceConfigurationError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20180523.UpdateApplicationMaintenanceConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                UpdateApplicationMaintenanceConfigurationError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<UpdateApplicationMaintenanceConfigurationResponse, _>()
    }
}
