use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq)]
pub enum CustomerSource {
    Internal,                  // ระบบภายใน
    External(String),          // ระบบภายนอก เช่น "CRM", "KYC"
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomerRef {
    pub customer_id: Uuid,          // ID กลางที่ใช้ mapping
    pub source: CustomerSource,     // แหล่งข้อมูล
    pub linked_at: NaiveDateTime,   // เวลาเชื่อมโยง
}