use tihu::api::Response;
use tihu::LightString;

#[derive(thiserror::Error, Debug)]
pub enum ErrNo {
    #[error("用户未登录")]
    LoginRequired,
    #[error("{0}")]
    CommonError(LightString),
    #[error("{0}")]
    Other(#[from] anyhow::Error),
    #[error("配置错误，{0}")]
    ConfigError(LightString),
    #[error("没有可用的{0}服务")]
    NoService(LightString),
    #[error("{0}服务忙")]
    ServiceBusy(LightString),
    #[error("暂停服务")]
    ServicePaused,
    #[error("接口不存在")]
    NoSuchApi,
    #[error("序列化数据失败,{0}")]
    SerializeError(serde_json::Error),
    #[error("反序列化数据失败,{0}")]
    DeserializeError(serde_json::Error),
    #[error("调用远程接口失败,{0}")]
    ApiError(anyhow::Error),
    #[error("只支持UTF-8格式的数据")]
    Utf8Only,
    #[error("参数格式不正确")]
    ParamFormatError,
    #[error("参数无效,{0}")]
    ParamInvalid(LightString),
    #[error("令牌无效")]
    TokenInvalid,
    #[error("没有权限")]
    NotAllowed,
    #[error("访问太频繁")]
    TooFrequent,
    #[error("文件上传请求格式不正确")]
    MultipartRequired,
    #[error("未定义的枚举值,{0}")]
    UndefinedEnumValue(LightString),
    #[error("没有数据库连接")]
    NoDbClient,
    #[error("预编译sql失败,{0}")]
    PrepareStatementError(anyhow::Error),
    #[error("查询数据失败,{0}")]
    QueryError(anyhow::Error),
    #[error("数据操作失败,{0}")]
    ExecuteError(anyhow::Error),
    #[error("开启数据库事务失败,{0}")]
    OpenTransactionError(anyhow::Error),
    #[error("获取数据字段失败,{0}")]
    ExtractDataError(anyhow::Error),
    #[error("提交数据库事务失败,{0}")]
    CommitTransactionError(anyhow::Error),
    #[error("没有缓存连接")]
    NoCacheClient,
    #[error("缓存操作失败,{0}")]
    CacheOperationError(anyhow::Error),
}

impl ErrNo {
    pub fn code(&self) -> i32 {
        return match *self {
            ErrNo::LoginRequired => -1,
            ErrNo::CommonError(_) => -2,
            ErrNo::Other(_) => -3,
            ErrNo::ConfigError(_) => -4,
            ErrNo::NoService(_) => -5,
            ErrNo::ServiceBusy(_) => -6,
            ErrNo::ServicePaused => -7,
            ErrNo::NoSuchApi => -8,
            ErrNo::SerializeError(_) => -9,
            ErrNo::DeserializeError(_) => -10,
            ErrNo::ApiError(_) => -11,
            ErrNo::Utf8Only => -12,
            ErrNo::ParamFormatError => -13,
            ErrNo::ParamInvalid(_) => -14,
            ErrNo::TokenInvalid => -15,
            ErrNo::NotAllowed => -16,
            ErrNo::TooFrequent => -17,
            ErrNo::MultipartRequired => -18,
            ErrNo::UndefinedEnumValue(_) => -19,
            ErrNo::NoDbClient => -20,
            ErrNo::PrepareStatementError(_) => -21,
            ErrNo::QueryError(_) => -22,
            ErrNo::ExecuteError(_) => -23,
            ErrNo::OpenTransactionError(_) => -24,
            ErrNo::ExtractDataError(_) => -25,
            ErrNo::CommitTransactionError(_) => -26,
            ErrNo::NoCacheClient => -27,
            ErrNo::CacheOperationError(_) => -28,
        };
    }
    pub fn message(&self) -> LightString {
        return self.to_string().into();
    }
}

impl<T> From<ErrNo> for Response<T> {
    fn from(err_no: ErrNo) -> Response<T> {
        return Response::failure(err_no.code(), err_no.message(), None);
    }
}

pub fn open_transaction_error<E>(error: E) -> ErrNo
where
    E: std::error::Error + Send + Sync + 'static,
{
    ErrNo::OpenTransactionError(error.into())
}

pub fn prepare_statement_error<E>(error: E) -> ErrNo
where
    E: std::error::Error + Send + Sync + 'static,
{
    ErrNo::PrepareStatementError(error.into())
}

pub fn query_error<E>(error: E) -> ErrNo
where
    E: std::error::Error + Send + Sync + 'static,
{
    ErrNo::QueryError(error.into())
}

pub fn undefined_enum_value(err_msg: LightString) -> ErrNo {
    ErrNo::UndefinedEnumValue(err_msg)
}

pub fn extract_data_error<E>(error: E) -> ErrNo
where
    E: std::error::Error + Send + Sync + 'static,
{
    ErrNo::ExtractDataError(error.into())
}

pub fn execute_error<E>(error: E) -> ErrNo
where
    E: std::error::Error + Send + Sync + 'static,
{
    ErrNo::ExecuteError(error.into())
}

pub fn commit_transaction_error<E>(error: E) -> ErrNo
where
    E: std::error::Error + Send + Sync + 'static,
{
    ErrNo::CommitTransactionError(error.into())
}
