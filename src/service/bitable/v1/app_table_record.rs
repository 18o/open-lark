use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::bitable::v1::Person,
};

pub struct AppTableRecordService {
    config: Config,
}

impl AppTableRecordService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn search(
        &self,
        request: SearchAppTableRecordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SearchAppTableRecordResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/search",
            app_token = request.app_token,
            table_id = request.table_id
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

#[derive(Debug, Serialize, Default)]
pub struct SearchAppTableRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 表ID
    table_id: String,
    /// 视图的唯一标识符，获取指定视图下的记录view_id 参数说明
    ///
    /// 注意：当 filter 参数 或 sort
    /// 参数不为空时，请求视为对数据表中的全部数据做条件过滤，指定的view_id 会被忽略。
    ///
    /// 示例值："vewqhz51lk"
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 字符 ～ 50 字符
    view_id: Option<String>,
    /// 字段名称，用于指定本次查询返回记录中包含的字段
    ///
    /// 示例值：["字段1","字段2"]
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 ～ 200
    field_names: Option<Vec<String>>,
    /// 排序条件
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 ～ 100
    sort: Option<SearchSort>,
    /// 筛选条件
    filter: Option<SearchFilterInfo>,
    /// 控制是否返回自动计算的字段, true 表示返回
    ///
    /// 示例值：false
    automatic: Option<bool>,
}

#[derive(Debug, Serialize, Default)]
pub struct SearchSort {
    /// 字段名称
    ///
    /// 示例值："多行文本"
    ///
    /// 数据校验规则：
    /// - 长度范围：0 字符 ～ 1000 字符
    pub field_name: Option<String>,
    /// 是否倒序排序
    ///
    /// 默认值：false
    pub desc: Option<bool>,
}
#[derive(Debug, Serialize, Default)]
pub struct SearchFilterInfo {
    /// 条件逻辑连接词
    ///
    /// 示例值："and"
    ///
    /// 可选值有：
    ///
    /// and：满足全部条件
    /// or：满足任一条件
    /// 数据校验规则：
    ///
    /// 长度范围：0 字符 ～ 10 字符
    pub conjunction: Option<String>,
    /// 筛选条件集合
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 ～ 50
    pub conditions: Option<Vec<SearchCondition>>,
}

/// 筛选条件
#[derive(Debug, Serialize, Default)]
pub struct SearchCondition {
    /// 筛选条件的左值，值为字段的名称
    ///
    /// 示例值："字段1"
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 字符 ～ 1000 字符
    pub field_name: String,
    /// 条件运算符
    ///
    /// 示例值："is"
    ///
    /// 可选值有：
    ///
    /// is：等于
    /// isNot：不等于
    /// contains：包含
    /// doesNotContain：不包含
    /// isEmpty：为空
    /// isNotEmpty：不为空
    /// isGreater：大于
    /// isGreaterEqual：大于等于
    /// isLess：小于
    /// isLessEqual：小于等于
    /// like：LIKE 运算符。暂未支持
    /// in：IN 运算符。暂未支持
    pub operator: String,
    /// 目标值
    ///
    /// 目标值填写指南
    ///
    /// 示例值：["文本内容"]
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 ～ 10
    pub value: Option<Vec<String>>,
}

impl SearchAppTableRecordRequest {
    pub fn builder() -> AppTableRecordSearchRequestBuilder {
        AppTableRecordSearchRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct AppTableRecordSearchRequestBuilder {
    request: SearchAppTableRecordRequest,
}

impl AppTableRecordSearchRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 表ID
    pub fn table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_id = table_id.to_string();
        self
    }

    /// 用户 ID 类型
    /// 可选值有：
    ///
    /// - open_id：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID
    ///   不同。了解更多：如何获取 Open ID
    /// - union_id：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID
    ///   是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union
    ///   ID，应用开发商可以把同个用户在多个应用中的身份关联起来。了解更多：如何获取 Union ID？
    /// - user_id：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID
    ///   是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User
    ///   ID 主要用于在不同的应用间打通用户数据。
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request
            .api_request
            .query_params
            .insert("user_id_type".to_string(), user_id_type.to_string());
        self
    }

    /// 分页标记，第一次请求不填，表示从头开始遍历；分页查询结果还有更多项时会同时返回新的
    /// page_token，下次遍历可采用该 page_token 获取查询结果
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request
            .api_request
            .query_params
            .insert("page_token".to_string(), page_token.to_string());
        self
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request
            .api_request
            .query_params
            .insert("page_size".to_string(), page_size.to_string());
        self
    }

    /// 视图的唯一标识符，获取指定视图下的记录view_id 参数说明
    ///
    /// 注意：当 filter 参数 或 sort
    /// 参数不为空时，请求视为对数据表中的全部数据做条件过滤，指定的view_id 会被忽略。
    ///
    /// 示例值："vewqhz51lk"
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 字符 ～ 50 字符
    pub fn view_id(mut self, view_id: impl ToString) -> Self {
        self.request.view_id = Some(view_id.to_string());
        self
    }

    /// 字段名称，用于指定本次查询返回记录中包含的字段
    ///
    /// 示例值：["字段1","字段2"]
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 ～ 200
    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.request.field_names = Some(field_names);
        self
    }

    /// 排序条件
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 ～ 100
    pub fn sort(mut self, sort: SearchSort) -> Self {
        self.request.sort = Some(sort);
        self
    }

    /// 筛选条件
    ///
    /// 数据校验规则：
    ///
    /// 长度范围：0 ～ 50
    pub fn filter(mut self, filter: SearchFilterInfo) -> Self {
        self.request.filter = Some(filter);
        self
    }

    /// 控制是否返回自动计算的字段, true 表示返回
    pub fn automatic(mut self, automatic: bool) -> Self {
        self.request.automatic = Some(automatic);
        self
    }

    pub fn build(mut self) -> SearchAppTableRecordRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchAppTableRecordResponse {
    pub items: Vec<Record>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
}

impl ApiResponseTrait for SearchAppTableRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub fields: HashMap<String, Value>,
    /// 记录Id
    pub record_id: String,
    /// 创建人
    pub created_by: Option<Person>,
    /// 创建时间
    pub created_time: Option<u128>,
    /// 修改人
    pub last_modified_by: Option<Person>,
    /// 最近更新时间
    pub last_modified_time: Option<u128>,
}
