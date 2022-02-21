// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::str::FromStr;

use common_exception::ErrorCode;
use common_exception::Result;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum StageType {
    Internal,
    External,
}

impl Default for StageType {
    fn default() -> Self {
        Self::External
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum StageFileCompression {
    Auto,
    Gzip,
    Bz2,
    Brotli,
    Zstd,
    Deflate,
    RawDeflate,
    Lzo,
    Snappy,
    None,
}

impl Default for StageFileCompression {
    fn default() -> Self {
        Self::None
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum StageFileFormatType {
    Csv,
    Json,
    Avro,
    Orc,
    Parquet,
    Xml,
}

impl Default for StageFileFormatType {
    fn default() -> Self {
        Self::Csv
    }
}

impl FromStr for StageFileFormatType {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, String> {
        match s.to_uppercase().as_str() {
            "CSV" => Ok(StageFileFormatType::Csv),
            "JSON" => Ok(StageFileFormatType::Json),
            "AVRO" => Ok(StageFileFormatType::Avro),
            "ORC" => Ok(StageFileFormatType::Orc),
            "PARQUET" => Ok(StageFileFormatType::Parquet),
            "XML" => Ok(StageFileFormatType::Xml),
            _ => Err(
                "Unknown file format type, must one of { CSV | JSON | AVRO | ORC | PARQUET | XML }"
                    .to_string(),
            ),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(default)]
pub struct FileFormatOptions {
    pub format: StageFileFormatType,
    // Number of lines at the start of the file to skip.
    pub skip_header: i32,
    pub field_delimiter: String,
    pub record_delimiter: String,
    pub compression: StageFileCompression,
}

impl Default for FileFormatOptions {
    fn default() -> Self {
        Self {
            format: StageFileFormatType::default(),
            record_delimiter: "\n".to_string(),
            field_delimiter: ",".to_string(),
            skip_header: 0,
            compression: StageFileCompression::default(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone, Debug, Eq, PartialEq)]
#[serde(default)]
pub struct StageS3Storage {
    // `example-bucket` in `s3://example-bucket/path/to/object`
    pub bucket: String,
    // `path/to/object` in `s3://example-bucket/path/to/object`
    pub path: String,
    pub credentials_aws_key_id: String,
    pub credentials_aws_secret_key: String,
    pub encryption_master_key: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum StageStorage {
    // Location is aws s3.
    S3(StageS3Storage),
}

impl Default for StageStorage {
    fn default() -> Self {
        Self::S3(StageS3Storage::default())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone, Debug, Eq, PartialEq)]
#[serde(default)]
pub struct StageParams {
    pub storage: StageStorage,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone, Debug, Eq, PartialEq)]
#[serde(default)]
pub struct CopyOptions {}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone, Debug, Eq, PartialEq)]
#[serde(default)]
pub struct UserStageInfo {
    pub stage_name: String,
    pub stage_type: StageType,
    pub stage_params: StageParams,
    pub file_format_options: FileFormatOptions,
    pub copy_options: CopyOptions,
    pub comment: String,
}

impl TryFrom<Vec<u8>> for UserStageInfo {
    type Error = ErrorCode;

    fn try_from(value: Vec<u8>) -> Result<Self> {
        match serde_json::from_slice(&value) {
            Ok(info) => Ok(info),
            Err(serialize_error) => Err(ErrorCode::IllegalUserStageFormat(format!(
                "Cannot deserialize stage from bytes. cause {}",
                serialize_error
            ))),
        }
    }
}
