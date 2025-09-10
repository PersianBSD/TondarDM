//! انواع دادهٔ مشترک (Meta، Range، JobId…)

#[derive(Debug, Clone)]
pub struct Meta {
    pub filename: String,
    pub size: Option<u64>,
    pub accept_ranges: bool,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct RangeReq {
    pub start: u64,
    pub end:   u64, // شامل end
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JobId(pub u64);

#[derive(Debug, Clone)]
pub struct PathPlan {
    pub dir: String,
    pub final_name: String,
    pub temp_name: String,
}
